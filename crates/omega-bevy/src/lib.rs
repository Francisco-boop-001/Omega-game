use anyhow::{Context, Result};
use bevy::prelude::Color;
use bevy::prelude::Time;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::{Event, EventReader, EventWriter};
use bevy_ecs::prelude::{Commands, Query, Res, ResMut, Resource, With};
use bevy_ecs::schedule::IntoSystemConfigs;
use omega_content::bootstrap_game_state_with_mode;
use omega_core::{
    Command, DeterministicRng, Direction, GameMode, GameState, ModalInputProfile,
    ObjectiveSnapshot, Outcome, Position, SessionStatus, SiteInteractionKind, TILE_FLAG_BLOCK_MOVE,
    TILE_FLAG_BURNING, TILE_FLAG_BURNT, active_activation_interaction_help_hint,
    active_activation_interaction_prompt, active_inventory_interaction_help_hint,
    active_inventory_interaction_prompt, active_item_prompt, active_item_prompt_help_hint,
    active_objective_snapshot, active_quit_interaction_help_hint, active_quit_interaction_prompt,
    active_site_interaction_help_hint, active_site_interaction_prompt,
    active_spell_interaction_help_hint, active_spell_interaction_prompt,
    active_talk_direction_help_hint, active_talk_direction_prompt,
    active_targeting_interaction_help_hint, active_targeting_interaction_prompt,
    active_wizard_interaction_help_hint, active_wizard_interaction_prompt, modal_input_profile,
    objective_journal, objective_map_hints, renderable_timeline_lines,
    sanitize_legacy_prompt_noise, step,
};
use omega_save::{decode_state_json_for_mode, encode_json};
use std::fs;
use std::path::PathBuf;

pub mod presentation;
pub mod simulation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Boot,
    Menu,
    InGame,
    WizardArena,
    Pause,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BevyKey {
    Char(char),
    Ctrl(char),
    Up,
    Down,
    Left,
    Right,
    F8,
    F12,
    Enter,
    Backspace,
    Esc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputAction {
    StartGame,
    NewGame,
    StartWizardArena,
    SaveSlot,
    SaveAndQuit,
    LoadSlot,
    RestartSession,
    ReturnToMenu,
    QuitApp,
    TogglePause,
    Dispatch(Command),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileKind {
    Floor,
    Wall,
    Grass,
    Water,
    Fire,
    Feature,
    Player,
    Monster,
    GroundItem,
    TargetCursor,
    ObjectiveMarker,
    ProjectileTrail,
    ProjectileImpact,
}

impl TileKind {
    /// Maps a TileKind to its corresponding semantic ColorId.
    ///
    /// This mapping determines which color from the active theme
    /// should be applied to each type of tile entity.
    ///
    /// # Map Overlay Colors
    ///
    /// Map overlays (halos, markers, targeting cursor, projectiles) are themed
    /// using semantic color categories:
    ///
    /// - **TargetCursor**: `UiColorId::Cursor` - Spell/action targeting reticle
    /// - **ObjectiveMarker**: `UiColorId::Highlight` - Quest objective halos and markers
    /// - **ProjectileTrail**: `EffectColorId::MagicArcane` - Magic projectile paths
    /// - **ProjectileImpact**: `EffectColorId::Impact` - Hit/explosion effects
    /// - **Fire**: `EffectColorId::Fire` - Persistent fire hazard
    ///
    /// These colors are resolved via `BevyTheme` in `sync_tile_entities_system`
    /// and applied as `RenderTileColor` components for sprite rendering.
    pub fn to_color_id(self) -> omega_core::color::ColorId {
        use omega_core::color::{
            ColorId, EffectColorId, EntityColorId, ItemRarityColorId, MonsterColorId,
            TerrainColorId, UiColorId,
        };

        match self {
            TileKind::Floor => ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorStone)),
            TileKind::Wall => ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallStone)),
            TileKind::Grass => ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorGrass)),
            TileKind::Water => ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Water)),
            TileKind::Fire => ColorId::Effect(EffectColorId::Fire),
            TileKind::Feature => ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Door)),
            TileKind::Player => ColorId::Entity(EntityColorId::Player),
            TileKind::Monster => {
                ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileHumanoid))
            }
            TileKind::GroundItem => ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Common)),
            // UI Overlays
            TileKind::TargetCursor => ColorId::Ui(UiColorId::Cursor),
            TileKind::ObjectiveMarker => ColorId::Ui(UiColorId::Highlight),
            // Effect Overlays
            TileKind::ProjectileTrail => ColorId::Effect(EffectColorId::MagicArcane),
            TileKind::ProjectileImpact => ColorId::Effect(EffectColorId::Impact),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteRef {
    pub atlas: String,
    pub index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileRender {
    pub position: Position,
    pub kind: TileKind,
    pub sprite: SpriteRef,
    pub glyph: Option<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderFrame {
    pub mode: GameMode,
    pub bounds: (i32, i32),
    pub tiles: Vec<TileRender>,
    pub hud_lines: Vec<String>,
    pub interaction_lines: Vec<String>,
    pub timeline_lines: Vec<String>,
    pub event_lines: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEventSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionFocusState {
    None,
    Prompt,
    TextEntry,
    DirectionEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapFxFrame {
    pub target_cursor: Option<Position>,
    pub objective_markers: Vec<Position>,
    pub projectile_path: Vec<Position>,
    pub projectile_impact: Option<Position>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModernCompassMarker {
    pub position: Position,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModernObjectiveUiState {
    pub active: Option<ObjectiveSnapshot>,
    pub journal: Vec<ObjectiveSnapshot>,
    pub markers: Vec<ModernCompassMarker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteAtlas {
    pub floor: SpriteRef,
    pub wall: SpriteRef,
    pub grass: SpriteRef,
    pub water: SpriteRef,
    pub fire: SpriteRef,
    pub feature: SpriteRef,
    pub player: SpriteRef,
    pub monster: SpriteRef,
    pub ground_item: SpriteRef,
    pub target_cursor: SpriteRef,
    pub objective_marker: SpriteRef,
    pub projectile_trail: SpriteRef,
    pub projectile_impact: SpriteRef,
}

impl Default for SpriteAtlas {
    fn default() -> Self {
        Self {
            floor: SpriteRef { atlas: "omega_base".to_string(), index: 0 },
            wall: SpriteRef { atlas: "omega_base".to_string(), index: 4 },
            grass: SpriteRef { atlas: "omega_base".to_string(), index: 0 },
            water: SpriteRef { atlas: "omega_base".to_string(), index: 5 },
            fire: SpriteRef { atlas: "omega_base".to_string(), index: 7 }, // Re-using projectile trail for now or similar
            feature: SpriteRef { atlas: "omega_base".to_string(), index: 5 },
            player: SpriteRef { atlas: "omega_base".to_string(), index: 1 },
            monster: SpriteRef { atlas: "omega_base".to_string(), index: 2 },
            ground_item: SpriteRef { atlas: "omega_base".to_string(), index: 3 },
            target_cursor: SpriteRef { atlas: "omega_base".to_string(), index: 6 },
            objective_marker: SpriteRef { atlas: "omega_base".to_string(), index: 9 },
            projectile_trail: SpriteRef { atlas: "omega_base".to_string(), index: 7 },
            projectile_impact: SpriteRef { atlas: "omega_base".to_string(), index: 8 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameSession {
    pub state: GameState,
    pub last_outcome: Option<Outcome>,
    rng: DeterministicRng,
}

impl Default for GameSession {
    fn default() -> Self {
        Self::new(0xBEE5_0001)
    }
}

impl GameSession {
    pub fn new(seed: u64) -> Self {
        Self::from_state(seed, GameState::default())
    }

    pub fn from_state(seed: u64, state: GameState) -> Self {
        Self { state, last_outcome: None, rng: DeterministicRng::seeded(seed) }
    }

    pub fn dispatch(&mut self, command: Command) {
        self.last_outcome = Some(step(&mut self.state, command, &mut self.rng));
    }

    pub fn project_frame(&self, atlas: &SpriteAtlas) -> RenderFrame {
        project_to_frame(&self.state, self.last_outcome.as_ref(), atlas)
    }
}

#[derive(Debug, Clone)]
pub struct BevyFrontend {
    pub app_state: AppState,
    pub session: Option<GameSession>,
    pub should_quit: bool,
    pub mode: GameMode,
    pub save_slot: PathBuf,
    pub bootstrap_state: GameState,
    session_seed: u64,
    restart_count: u64,
}

impl Default for BevyFrontend {
    fn default() -> Self {
        Self::new()
    }
}

impl BevyFrontend {
    pub fn new() -> Self {
        Self::with_seed_and_mode(0xBEE5_0001, GameMode::Classic)
    }

    pub fn with_seed(seed: u64) -> Self {
        Self::with_seed_and_mode(seed, GameMode::Classic)
    }

    pub fn with_seed_and_mode(seed: u64, mode: GameMode) -> Self {
        let bootstrap_state = load_bootstrap_or_default(mode);
        Self::with_seed_and_bootstrap(seed, bootstrap_state, default_save_slot_path_for_mode(mode))
    }

    pub fn with_seed_and_bootstrap(
        seed: u64,
        bootstrap_state: GameState,
        save_slot: PathBuf,
    ) -> Self {
        let mode = bootstrap_state.mode;
        Self {
            app_state: AppState::Boot,
            session: None,
            should_quit: false,
            mode,
            save_slot,
            bootstrap_state,
            session_seed: seed,
            restart_count: 0,
        }
    }

    pub fn boot(&mut self) {
        if self.app_state == AppState::Boot {
            self.app_state = AppState::Menu;
        }
    }

    pub fn handle_key(&mut self, key: BevyKey) {
        let action = self.action_for_key(key);
        self.apply_action(action);
    }

    fn action_for_key(&self, key: BevyKey) -> InputAction {
        if (self.app_state == AppState::InGame || self.app_state == AppState::WizardArena)
            && let Some(session) = self.session.as_ref()
        {
            let profile = modal_input_profile(&session.state);
            if profile != ModalInputProfile::None {
                return map_modal_interaction_key(key, profile);
            }
        }
        if (self.app_state == AppState::InGame || self.app_state == AppState::WizardArena)
            && let BevyKey::Char(ch) = key
            && let Some(command) = self.adaptive_directional_command(ch)
        {
            return InputAction::Dispatch(command);
        }
        map_input(self.app_state, key)
    }

    fn adaptive_directional_command(&self, ch: char) -> Option<Command> {
        let direction = match ch {
            'W' => Direction::North,
            'X' => Direction::South,
            'A' => Direction::West,
            'D' => Direction::East,
            _ => return None,
        };

        let session = self.session.as_ref()?;
        let target = session.state.player.position.offset(direction);
        let has_adjacent_monster =
            session.state.monsters.iter().any(|monster| monster.position == target);
        if has_adjacent_monster {
            Some(Command::Attack(direction))
        } else {
            Some(Command::Move(direction))
        }
    }

    pub fn apply_action(&mut self, action: InputAction) {
        match action {
            InputAction::StartGame | InputAction::NewGame => {
                self.restart_count = self.restart_count.wrapping_add(1);
                let seed = self.session_seed.wrapping_add(self.restart_count);
                self.session = Some(GameSession::from_state(seed, self.bootstrap_state.clone()));
                self.app_state = AppState::InGame;
            }
            InputAction::StartWizardArena => {
                let (state, _) =
                    omega_content::bootstrap_wizard_arena().expect("Wizard Arena bootstrap failed");
                self.restart_count = self.restart_count.wrapping_add(1);
                let seed = self.session_seed.wrapping_add(self.restart_count);
                self.session = Some(GameSession::from_state(seed, state));
                self.app_state = AppState::WizardArena;
            }
            InputAction::SaveSlot => {
                if let Err(err) = self.save_to_slot()
                    && let Some(session) = self.session.as_mut()
                {
                    session.state.log.push(format!("Save failed: {err}"));
                }
            }
            InputAction::SaveAndQuit => {
                if let Err(err) = self.save_to_slot()
                    && let Some(session) = self.session.as_mut()
                {
                    session.state.log.push(format!("Save failed: {err}"));
                } else {
                    self.session = None;
                    self.app_state = AppState::Menu;
                }
            }
            InputAction::LoadSlot => {
                if let Err(err) = self.load_from_slot()
                    && let Some(session) = self.session.as_mut()
                {
                    session.state.log.push(format!("Load failed: {err}"));
                }
            }
            InputAction::RestartSession => {
                self.restart_count = self.restart_count.wrapping_add(1);
                let seed = self.session_seed.wrapping_add(self.restart_count);
                self.session = Some(GameSession::from_state(seed, self.bootstrap_state.clone()));
                self.app_state = AppState::InGame;
            }
            InputAction::ReturnToMenu => {
                self.session = None;
                self.app_state = AppState::Menu;
            }
            InputAction::QuitApp => {
                self.should_quit = true;
            }
            InputAction::TogglePause => match self.app_state {
                AppState::InGame => self.app_state = AppState::Pause,
                AppState::Pause => self.app_state = AppState::InGame,
                _ => {}
            },
            InputAction::Dispatch(command) => {
                if self.app_state != AppState::InGame && self.app_state != AppState::WizardArena {
                    return;
                }
                if let Some(session) = self.session.as_mut() {
                    let was_in_progress = session.state.status == SessionStatus::InProgress;
                    session.dispatch(command);
                    if session.state.status != SessionStatus::InProgress {
                        if was_in_progress {
                            let prompt = match session.state.status {
                                SessionStatus::Lost => {
                                    "You died. Press c/q/esc to continue, r to restart, n to start a new run."
                                }
                                SessionStatus::Won => {
                                    "Victory complete. Press c/q/esc to continue, r to restart, n to start a new run."
                                }
                                SessionStatus::InProgress => "",
                            };
                            if !prompt.is_empty() {
                                session.state.log.push(prompt.to_string());
                            }
                        }
                        self.app_state = AppState::GameOver;
                    }
                }
            }
            InputAction::None => {}
        }
    }

    pub fn render_frame(&self, atlas: &SpriteAtlas) -> Option<RenderFrame> {
        self.session.as_ref().map(|s| s.project_frame(atlas))
    }

    pub fn save_to_slot(&mut self) -> Result<()> {
        let Some(session) = self.session.as_ref() else {
            return Ok(());
        };

        let raw = encode_json(&session.state).context("encode bevy save slot")?;
        if let Some(parent) = self.save_slot.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)
                .with_context(|| format!("create save slot directory {}", parent.display()))?;
        }
        fs::write(&self.save_slot, raw)
            .with_context(|| format!("write save slot {}", self.save_slot.display()))?;
        if let Some(session) = self.session.as_mut() {
            session.state.log.push(format!("Saved slot: {}", self.save_slot.display()));
        }
        Ok(())
    }

    pub fn load_from_slot(&mut self) -> Result<()> {
        let raw = fs::read_to_string(&self.save_slot)
            .with_context(|| format!("read save slot {}", self.save_slot.display()))?;
        let mut state =
            decode_state_json_for_mode(&raw, self.mode).context("decode bevy save slot")?;
        state.options.interactive_sites = true;
        sanitize_legacy_prompt_noise(&mut state.log);
        self.restart_count = self.restart_count.wrapping_add(1);
        let seed = self.session_seed.wrapping_add(self.restart_count);
        self.session = Some(GameSession::from_state(seed, state));
        self.app_state = AppState::InGame;
        if let Some(session) = self.session.as_mut() {
            session.state.log.push(format!("Loaded slot: {}", self.save_slot.display()));
        }
        Ok(())
    }
}

pub fn default_save_slot_path() -> PathBuf {
    default_save_slot_path_for_mode(GameMode::Classic)
}

pub fn default_save_slot_path_for_mode(mode: GameMode) -> PathBuf {
    PathBuf::from(format!("target/omega-{}-slot-1.json", mode.as_str()))
}

fn load_bootstrap_or_default(mode: GameMode) -> GameState {
    let (mut state, diagnostics) = bootstrap_game_state_with_mode(mode)
        .unwrap_or_else(|err| panic!("Content bootstrap failed, refusing fallback runtime: {err}"));
    state.options.interactive_sites = true;
    state.log.push(format!(
        "Bootstrap: source={}, spawn={}, monsters={}, items={}",
        diagnostics.map_source,
        diagnostics.player_spawn_source,
        diagnostics.monster_spawns,
        diagnostics.item_spawns
    ));
    state
}

fn map_ctrl_legacy(ch: char) -> Option<String> {
    let lowered = ch.to_ascii_lowercase();
    if matches!(lowered, 'f' | 'g' | 'i' | 'k' | 'l' | 'o' | 'p' | 'r' | 'w' | 'x') {
        Some(format!("^{lowered}"))
    } else {
        None
    }
}

fn map_modal_interaction_key(key: BevyKey, profile: ModalInputProfile) -> InputAction {
    match key {
        BevyKey::Esc => InputAction::Dispatch(Command::Legacy { token: "<esc>".to_string() }),
        BevyKey::Enter => InputAction::Dispatch(Command::Legacy { token: "<enter>".to_string() }),
        BevyKey::Backspace => {
            InputAction::Dispatch(Command::Legacy { token: "<backspace>".to_string() })
        }
        BevyKey::Ctrl(ch) => {
            if let Some(token) = map_ctrl_legacy(ch) {
                InputAction::Dispatch(Command::Legacy { token })
            } else {
                InputAction::None
            }
        }
        BevyKey::Char(ch) => InputAction::Dispatch(Command::Legacy { token: ch.to_string() }),
        BevyKey::Up => {
            if profile == ModalInputProfile::DirectionEntry {
                InputAction::Dispatch(Command::Move(Direction::North))
            } else {
                InputAction::None
            }
        }
        BevyKey::Down => {
            if profile == ModalInputProfile::DirectionEntry {
                InputAction::Dispatch(Command::Move(Direction::South))
            } else {
                InputAction::None
            }
        }
        BevyKey::Left => {
            if profile == ModalInputProfile::DirectionEntry {
                InputAction::Dispatch(Command::Move(Direction::West))
            } else {
                InputAction::None
            }
        }
        BevyKey::Right => {
            if profile == ModalInputProfile::DirectionEntry {
                InputAction::Dispatch(Command::Move(Direction::East))
            } else {
                InputAction::None
            }
        }
        BevyKey::F12 => InputAction::Dispatch(Command::Legacy { token: "^g".to_string() }),
        BevyKey::F8 => InputAction::None,
    }
}

pub fn map_shared_gameplay_key(key: BevyKey) -> InputAction {
    match key {
        BevyKey::Esc => InputAction::None,
        BevyKey::Up => InputAction::Dispatch(Command::Move(Direction::North)),
        BevyKey::Down => InputAction::Dispatch(Command::Move(Direction::South)),
        BevyKey::Left => InputAction::Dispatch(Command::Move(Direction::West)),
        BevyKey::Right => InputAction::Dispatch(Command::Move(Direction::East)),
        BevyKey::F8 => InputAction::None,
        BevyKey::F12 => InputAction::Dispatch(Command::Legacy { token: "^g".to_string() }),
        BevyKey::Enter => InputAction::Dispatch(Command::Legacy { token: "<enter>".to_string() }),
        BevyKey::Backspace => {
            InputAction::Dispatch(Command::Legacy { token: "<backspace>".to_string() })
        }
        BevyKey::Ctrl(ch) => {
            if let Some(token) = map_ctrl_legacy(ch) {
                InputAction::Dispatch(Command::Legacy { token })
            } else {
                InputAction::None
            }
        }
        BevyKey::Char(ch) => match ch {
            ' ' | '.' => InputAction::Dispatch(Command::Wait),
            'w' | 'k' => InputAction::Dispatch(Command::Move(Direction::North)),
            's' | 'j' => InputAction::Dispatch(Command::Move(Direction::South)),
            'h' => InputAction::Dispatch(Command::Move(Direction::West)),
            'd' | 'l' => InputAction::Dispatch(Command::Move(Direction::East)),
            'W' => InputAction::Dispatch(Command::Attack(Direction::North)),
            'X' => InputAction::Dispatch(Command::Attack(Direction::South)),
            'A' => InputAction::Dispatch(Command::Attack(Direction::West)),
            'D' => InputAction::Dispatch(Command::Attack(Direction::East)),
            'g' => InputAction::Dispatch(Command::Pickup),
            ',' | '@' | '<' | '>' | '?' | '/' => {
                InputAction::Dispatch(Command::Legacy { token: ch.to_string() })
            }
            'a' | 'e' | 'f' | 'm' | 'o' | 'p' | 'r' | 't' | 'v' | 'x' | 'z' | 'c' => {
                InputAction::Dispatch(Command::Legacy { token: ch.to_string() })
            }
            'C' | 'E' | 'F' | 'G' | 'H' | 'I' | 'M' | 'O' | 'T' | 'V' | 'Z' => {
                InputAction::Dispatch(Command::Legacy { token: ch.to_string() })
            }
            'u' | 'y' | 'b' | 'n' => {
                InputAction::Dispatch(Command::Legacy { token: ch.to_string() })
            }
            '1'..='9' => InputAction::Dispatch(Command::Drop { slot: (ch as u8 - b'1') as usize }),
            'P' => InputAction::Dispatch(Command::Legacy { token: ch.to_string() }),
            'S' => InputAction::SaveAndQuit,
            'L' => InputAction::LoadSlot,
            'R' => InputAction::RestartSession,
            'N' => InputAction::NewGame,
            'Q' => InputAction::Dispatch(Command::Legacy { token: "Q".to_string() }),
            'q' => InputAction::Dispatch(Command::Legacy { token: "q".to_string() }),
            _ if ch.is_ascii_graphic() => {
                InputAction::Dispatch(Command::Legacy { token: ch.to_string() })
            }
            _ => InputAction::None,
        },
    }
}

pub fn map_input(state: AppState, key: BevyKey) -> InputAction {
    match state {
        AppState::Boot => InputAction::None,
        AppState::Menu => match key {
            BevyKey::Enter => InputAction::StartGame,
            BevyKey::Char('n') | BevyKey::Char('N') => InputAction::NewGame,
            BevyKey::F8 => InputAction::StartWizardArena,
            BevyKey::Char('L') => InputAction::LoadSlot,
            BevyKey::Esc | BevyKey::Char('q') => InputAction::QuitApp,
            _ => InputAction::None,
        },
        AppState::InGame => match key {
            BevyKey::Esc => InputAction::TogglePause,
            BevyKey::F8 => InputAction::StartWizardArena,
            other => map_shared_gameplay_key(other),
        },
        AppState::WizardArena => match key {
            BevyKey::Esc => InputAction::ReturnToMenu,
            other => map_shared_gameplay_key(other),
        },
        AppState::Pause => match key {
            BevyKey::Esc => InputAction::TogglePause,
            BevyKey::Char('Q') => InputAction::ReturnToMenu,
            BevyKey::Char('L') => InputAction::LoadSlot,
            BevyKey::Char('S') => InputAction::SaveAndQuit,
            _ => InputAction::None,
        },
        AppState::GameOver => match key {
            BevyKey::Enter | BevyKey::Char('N') | BevyKey::Char('n') => InputAction::StartGame,
            BevyKey::Char('R') | BevyKey::Char('r') => InputAction::RestartSession,
            BevyKey::Char('Q')
            | BevyKey::Char('q')
            | BevyKey::Char('C')
            | BevyKey::Char('c')
            | BevyKey::Esc => InputAction::ReturnToMenu,
            _ => InputAction::None,
        },
    }
}

pub fn project_to_frame(
    state: &GameState,
    last_outcome: Option<&Outcome>,
    atlas: &SpriteAtlas,
) -> RenderFrame {
    let mut tiles = Vec::new();
    let modern_objectives = if state.mode == GameMode::Modern {
        let journal = objective_journal(state);
        let active = active_objective_snapshot(state);
        let markers = objective_map_hints(state)
            .into_iter()
            .map(|position| ModernCompassMarker { position, label: "Objective marker".to_string() })
            .collect();
        Some(ModernObjectiveUiState { active, journal, markers })
    } else {
        None
    };

    for y in 0..state.bounds.height {
        for x in 0..state.bounds.width {
            let position = Position { x, y };
            let glyph = state.map_glyph_at(position);
            let kind = tile_kind_from_map_glyph(glyph);
            tiles.push(TileRender {
                position,
                kind,
                sprite: sprite_for_tile_kind(atlas, kind),
                glyph: Some(glyph),
            });
            if let Some(cell) = state.tile_site_at(position)
                && (cell.flags & omega_core::TILE_FLAG_BURNING) != 0
            {
                tiles.push(TileRender {
                    position,
                    kind: TileKind::Fire,
                    sprite: atlas.fire.clone(),
                    glyph: Some('*'), // Fire glyph
                });
            }
        }
    }

    for ground in &state.ground_items {
        tiles.push(TileRender {
            position: ground.position,
            kind: TileKind::GroundItem,
            sprite: atlas.ground_item.clone(),
            glyph: Some('!'),
        });
    }

    for trap in &state.traps {
        if trap.armed {
            tiles.push(TileRender {
                position: trap.position,
                kind: TileKind::Feature,
                sprite: atlas.feature.clone(),
                glyph: Some('^'),
            });
        }
    }

    for monster in &state.monsters {
        tiles.push(TileRender {
            position: monster.position,
            kind: TileKind::Monster,
            sprite: atlas.monster.clone(),
            glyph: monster.display_glyph.or(Some('m')),
        });
    }

    tiles.push(TileRender {
        position: state.player.position,
        kind: TileKind::Player,
        sprite: atlas.player.clone(),
        glyph: Some('@'),
    });

    for pos in &state.transient_projectile_path {
        tiles.push(TileRender {
            position: *pos,
            kind: TileKind::ProjectileTrail,
            sprite: atlas.projectile_trail.clone(),
            glyph: Some('*'),
        });
    }
    if let Some(impact) = state.transient_projectile_impact {
        tiles.push(TileRender {
            position: impact,
            kind: TileKind::ProjectileImpact,
            sprite: atlas.projectile_impact.clone(),
            glyph: Some('x'),
        });
    }
    if let Some(targeting) = state.pending_targeting_interaction.as_ref() {
        tiles.push(TileRender {
            position: targeting.cursor,
            kind: TileKind::TargetCursor,
            sprite: atlas.target_cursor.clone(),
            glyph: Some('X'),
        });
    }
    if let Some(objectives) = modern_objectives.as_ref() {
        for marker in &objectives.markers {
            tiles.push(TileRender {
                position: marker.position,
                kind: TileKind::ObjectiveMarker,
                sprite: atlas.objective_marker.clone(),
                glyph: Some('O'),
            });
        }
    }

    let mut hud_lines = vec![
        format!("Name {}", state.player_name),
        format!("Mode {}", state.mode.as_str()),
        format!("Turn {}", state.clock.turn),
        format!("Time {}m", state.clock.minutes),
        format!("HP {}/{}", state.player.stats.hp, state.player.stats.max_hp),
        format!("Mana {}/{}", state.spellbook.mana, state.spellbook.max_mana),
        format!("Inventory {}/{}", state.player.inventory.len(), state.player.inventory_capacity),
        format!(
            "Equip W:{} S:{} A:{} C:{} B:{} R1:{} R2:{}",
            if state.player.equipment.weapon_hand.is_some() { "Y" } else { "-" },
            if state.player.equipment.shield.is_some() { "Y" } else { "-" },
            if state.player.equipment.armor.is_some() { "Y" } else { "-" },
            if state.player.equipment.cloak.is_some() { "Y" } else { "-" },
            if state.player.equipment.boots.is_some() { "Y" } else { "-" },
            if state.player.equipment.ring_1.is_some() { "Y" } else { "-" },
            if state.player.equipment.ring_2.is_some() { "Y" } else { "-" },
        ),
        format!("Gold/Bank/Food {}/{}/{}", state.gold, state.bank_gold, state.food),
        format!("World {:?} Quest {:?}", state.world_mode, state.progression.quest_state),
        format!("Status {:?}", state.status),
        format!("Interaction {}", describe_pending_interaction(state)),
        "Keys a=activate, z=zap, Q=retire/quit flow, move west with h/Left".to_string(),
        "Combat move into adjacent monsters to bump-attack; uppercase WASD attacks directly."
            .to_string(),
    ];
    if let Some(objectives) = modern_objectives.as_ref() {
        if let Some(active) = objectives.active.as_ref() {
            hud_lines.push(format!("Objective {}", active.summary));
            if let Some(next_step) = active.steps.iter().find(|step| !step.complete) {
                hud_lines.push(format!("Next {}", next_step.description));
            }
            hud_lines.push(format!("Journal entries {}", objectives.journal.len()));
        } else {
            hud_lines.push("Objective none".to_string());
        }
    }

    let mut interaction_lines = Vec::new();
    let mut timeline_lines = Vec::new();
    let active_prompt = active_wizard_interaction_prompt(state)
        .or_else(|| active_spell_interaction_prompt(state))
        .or_else(|| active_quit_interaction_prompt(state))
        .or_else(|| active_talk_direction_prompt(state))
        .or_else(|| active_activation_interaction_prompt(state))
        .or_else(|| active_targeting_interaction_prompt(state))
        .or_else(|| active_inventory_interaction_prompt(state))
        .or_else(|| active_item_prompt(state))
        .or_else(|| active_site_interaction_prompt(state));
    let active_hint = active_wizard_interaction_help_hint(state)
        .or_else(|| active_spell_interaction_help_hint(state))
        .or_else(|| active_quit_interaction_help_hint(state))
        .or_else(|| active_talk_direction_help_hint(state))
        .or_else(|| active_activation_interaction_help_hint(state))
        .or_else(|| active_targeting_interaction_help_hint(state))
        .or_else(|| active_inventory_interaction_help_hint(state))
        .or_else(|| active_item_prompt_help_hint(state))
        .or_else(|| active_site_interaction_help_hint(state));
    if let Some(prompt) = active_prompt.as_ref() {
        interaction_lines.push(format!("ACTIVE: {prompt}"));
    }
    if let Some(hint) = active_hint.as_ref() {
        interaction_lines.push(hint.clone());
    }
    if modal_input_profile(state) == ModalInputProfile::TextEntry {
        if state.pending_wizard_interaction.is_some() && !state.wizard_input_buffer.is_empty() {
            interaction_lines.push(format!("INPUT: {}", state.wizard_input_buffer));
        } else if state.pending_spell_interaction.is_some() && !state.spell_input_buffer.is_empty()
        {
            interaction_lines.push(format!("INPUT: {}", state.spell_input_buffer));
        } else if state.pending_targeting_interaction.is_some()
            && !state.target_input_buffer.is_empty()
        {
            interaction_lines.push(format!("INPUT: {}", state.target_input_buffer));
        } else if !state.interaction_buffer.is_empty() {
            interaction_lines.push(format!("INPUT: {}", state.interaction_buffer));
        }
    }
    if let Some(objectives) = modern_objectives.as_ref()
        && let Some(active) = objectives.active.as_ref()
    {
        interaction_lines.push(format!("OBJECTIVE: {}", active.title));
        interaction_lines.push(format!("NEXT: {}", active.summary));
    }
    if state.status == SessionStatus::Lost
        && let Some(source) = state.death_source.as_deref()
    {
        timeline_lines.push(format!("Killed by {source}."));
    }
    let resolved_timeline = renderable_timeline_lines(state, 8);
    if !resolved_timeline.is_empty() {
        timeline_lines.extend(resolved_timeline);
    } else if let Some(outcome) = last_outcome {
        for event in outcome.events.iter().rev().take(8).rev() {
            timeline_lines.push(format_event_line(event));
        }
        if outcome.status != omega_core::SessionStatus::InProgress {
            timeline_lines.push(format!("session status: {:?}", outcome.status));
        }
    }
    let mut event_lines = interaction_lines.clone();
    event_lines.extend(timeline_lines.iter().cloned());

    RenderFrame {
        mode: state.mode,
        bounds: (state.bounds.width, state.bounds.height),
        tiles,
        hud_lines,
        interaction_lines,
        timeline_lines,
        event_lines,
    }
}

fn describe_pending_interaction(state: &GameState) -> String {
    if state.pending_wizard_interaction.is_some() {
        return "wizard prompt".to_string();
    }
    if state.pending_spell_interaction.is_some() {
        return "spell prompt".to_string();
    }
    if state.pending_activation_interaction.is_some() {
        return "activation prompt".to_string();
    }
    if state.pending_quit_interaction.is_some() {
        return "quit confirmation".to_string();
    }
    if state.pending_talk_direction.is_some() {
        return "directional talk/tunnel prompt".to_string();
    }
    if state.pending_targeting_interaction.is_some() {
        return "targeting prompt".to_string();
    }
    if state.pending_inventory_interaction.is_some() {
        return "inventory interaction".to_string();
    }
    if state.pending_item_prompt.is_some() {
        return "item selection prompt".to_string();
    }
    let Some(kind) = state.pending_site_interaction.as_ref() else {
        return "none".to_string();
    };
    match kind {
        SiteInteractionKind::Shop => "shop menu".to_string(),
        SiteInteractionKind::Armorer => "armorer menu".to_string(),
        SiteInteractionKind::Club => "club menu".to_string(),
        SiteInteractionKind::Gym => "gym menu".to_string(),
        SiteInteractionKind::Healer => "healer menu".to_string(),
        SiteInteractionKind::Casino => "casino menu".to_string(),
        SiteInteractionKind::Commandant => "commandant menu".to_string(),
        SiteInteractionKind::Diner => "diner menu".to_string(),
        SiteInteractionKind::Craps => "craps menu".to_string(),
        SiteInteractionKind::Tavern => "tavern menu".to_string(),
        SiteInteractionKind::PawnShop => "pawn shop menu".to_string(),
        SiteInteractionKind::Brothel => "brothel menu".to_string(),
        SiteInteractionKind::Condo => "condo menu".to_string(),
        SiteInteractionKind::Bank => "bank menu".to_string(),
        SiteInteractionKind::MercGuild => "merc guild menu".to_string(),
        SiteInteractionKind::ThievesGuild => "thieves guild menu".to_string(),
        SiteInteractionKind::Temple => "temple menu".to_string(),
        SiteInteractionKind::College => "college menu".to_string(),
        SiteInteractionKind::Sorcerors => "sorcerors menu".to_string(),
        SiteInteractionKind::Castle => "castle menu".to_string(),
        SiteInteractionKind::Palace => "palace menu".to_string(),
        SiteInteractionKind::Order => "order menu".to_string(),
        SiteInteractionKind::Charity => "charity menu".to_string(),
        SiteInteractionKind::Monastery => "monastery menu".to_string(),
        SiteInteractionKind::Arena => {
            if state.progression.arena_rank > 0 {
                "Rampart Coliseum (fight/leave)".to_string()
            } else {
                "Rampart Coliseum (enter/register/leave)".to_string()
            }
        }
        SiteInteractionKind::Altar { deity_id } => match deity_id {
            1 => "Odin altar".to_string(),
            2 => "Set altar".to_string(),
            3 => "Athena altar".to_string(),
            4 => "Hecate altar".to_string(),
            5 => "Destiny altar".to_string(),
            _ => "altar".to_string(),
        },
    }
}

pub fn interaction_focus_state(state: &GameState) -> InteractionFocusState {
    if state.pending_wizard_interaction.is_some()
        || state.pending_spell_interaction.is_some()
        || state.pending_quit_interaction.is_some()
        || state.pending_talk_direction.is_some()
        || state.pending_activation_interaction.is_some()
        || state.pending_inventory_interaction.is_some()
        || state.pending_item_prompt.is_some()
        || state.pending_site_interaction.is_some()
    {
        return if modal_input_profile(state) == ModalInputProfile::TextEntry {
            InteractionFocusState::TextEntry
        } else {
            InteractionFocusState::Prompt
        };
    }
    if state.pending_targeting_interaction.is_some() {
        return InteractionFocusState::DirectionEntry;
    }
    InteractionFocusState::None
}

pub fn map_fx_frame(state: &GameState) -> MapFxFrame {
    let objective_markers =
        if state.mode == GameMode::Modern { objective_map_hints(state) } else { Vec::new() };
    MapFxFrame {
        target_cursor: state
            .pending_targeting_interaction
            .as_ref()
            .map(|targeting| targeting.cursor),
        objective_markers,
        projectile_path: state.transient_projectile_path.clone(),
        projectile_impact: state.transient_projectile_impact,
    }
}

pub fn classify_timeline_line(line: &str) -> UiEventSeverity {
    let lowered = line.to_ascii_lowercase();
    if lowered.contains("killed")
        || lowered.contains("death")
        || lowered.contains("dies")
        || lowered.contains("fatal")
    {
        UiEventSeverity::Critical
    } else if lowered.contains("fail")
        || lowered.contains("cannot")
        || lowered.contains("blocked")
        || lowered.contains("reject")
    {
        UiEventSeverity::Warning
    } else {
        UiEventSeverity::Info
    }
}

fn format_event_line(event: &omega_core::Event) -> String {
    match event {
        omega_core::Event::Moved { from, to } => {
            format!("moved: ({}, {}) -> ({}, {})", from.x, from.y, to.x, to.y)
        }
        omega_core::Event::MoveBlocked { target } => {
            format!("blocked: ({}, {})", target.x, target.y)
        }
        omega_core::Event::Attacked { monster_id, damage, remaining_hp } => {
            format!("hit monster#{monster_id} for {damage} (hp {remaining_hp})")
        }
        omega_core::Event::MonsterAttacked { monster_id, damage, remaining_hp } => {
            format!("monster#{monster_id} hit you for {damage} (hp {remaining_hp})")
        }
        omega_core::Event::LegacyHandled { token, note, fully_modeled: _ } => {
            format!("legacy `{token}`: {note}")
        }
        omega_core::Event::EconomyUpdated { source, gold, bank_gold } => {
            format!("economy `{source}` gold={gold} bank={bank_gold}")
        }
        omega_core::Event::DialogueAdvanced { speaker, quest_state } => {
            format!("dialogue `{speaker}` -> quest {quest_state:?}")
        }
        omega_core::Event::QuestAdvanced { state, steps_completed } => {
            format!("quest -> {state:?} (steps {steps_completed})")
        }
        omega_core::Event::ProgressionUpdated { guild_rank, priest_rank, alignment } => {
            format!("progression g{guild_rank}/p{priest_rank} {alignment:?}")
        }
        omega_core::Event::TurnAdvanced { turn, minutes } => {
            format!("turn advanced: {turn} ({minutes}m)")
        }
        other => format!("{other:?}"),
    }
}

fn tile_kind_from_map_glyph(ch: char) -> TileKind {
    match ch {
        '#' | '=' => TileKind::Wall,
        '.' | ' ' => TileKind::Floor,
        '\"' | ',' => TileKind::Grass,
        '~' => TileKind::Water,
        _ => TileKind::Feature,
    }
}

fn sprite_for_tile_kind(atlas: &SpriteAtlas, kind: TileKind) -> SpriteRef {
    match kind {
        TileKind::Floor => atlas.floor.clone(),
        TileKind::Wall => atlas.wall.clone(),
        TileKind::Grass => atlas.grass.clone(),
        TileKind::Water => atlas.water.clone(),
        TileKind::Fire => atlas.fire.clone(),
        TileKind::Feature => atlas.feature.clone(),
        TileKind::Player => atlas.player.clone(),
        TileKind::Monster => atlas.monster.clone(),
        TileKind::GroundItem => atlas.ground_item.clone(),
        TileKind::TargetCursor => atlas.target_cursor.clone(),
        TileKind::ObjectiveMarker => atlas.objective_marker.clone(),
        TileKind::ProjectileTrail => atlas.projectile_trail.clone(),
        TileKind::ProjectileImpact => atlas.projectile_impact.clone(),
    }
}

#[derive(Debug, Clone, Resource)]
pub struct FrontendRuntime(pub BevyFrontend);

#[derive(Debug, Clone, Resource)]
pub struct RuntimeSpriteAtlas(pub SpriteAtlas);

#[derive(Debug, Clone, Resource, Default)]
pub struct PendingInput {
    pub keys: Vec<BevyKey>,
}

#[derive(Debug, Clone, Resource, Default)]
pub struct RuntimeFrame {
    pub frame: Option<RenderFrame>,
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct RuntimeStatus {
    pub app_state: AppState,
    pub should_quit: bool,
}

#[derive(Debug, Clone, Resource)]
struct ArenaHazardBridgeState {
    damage_cooldown_s: f32,
    fire_immunity_s: f32,
    gas_damage_cooldown_s: f32,
    monster_damage_cooldown_s: f32,
    wind_push_cooldown_s: f32,
    was_in_fire: bool,
    was_in_water: bool,
    was_in_gas: bool,
}

impl Default for ArenaHazardBridgeState {
    fn default() -> Self {
        Self {
            damage_cooldown_s: 0.0,
            fire_immunity_s: 0.0,
            gas_damage_cooldown_s: 0.0,
            monster_damage_cooldown_s: 0.0,
            wind_push_cooldown_s: 0.0,
            was_in_fire: false,
            was_in_water: false,
            was_in_gas: false,
        }
    }
}

#[derive(Debug, Clone, Event)]
pub struct InputActionEvent(pub InputAction);

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub struct RenderTile;

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub struct RenderTilePosition(pub Position);

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Hash)]
pub struct RenderTileKind(pub TileKind);

#[derive(Debug, Clone, Component, PartialEq, Eq)]
pub struct RenderTileSprite(pub SpriteRef);

#[derive(Debug, Clone, Copy, Component)]
pub struct RenderTileColor(pub Color);

pub struct OmegaBevyRuntimePlugin {
    pub session_seed: u64,
    pub mode: Option<GameMode>,
    pub bootstrap_state: Option<GameState>,
    pub save_slot: Option<PathBuf>,
}

impl Default for OmegaBevyRuntimePlugin {
    fn default() -> Self {
        Self { session_seed: 0xBEE5_0001, mode: None, bootstrap_state: None, save_slot: None }
    }
}

impl Plugin for OmegaBevyRuntimePlugin {
    fn build(&self, app: &mut App) {
        let mode = self.mode.unwrap_or(GameMode::Classic);
        let bootstrap =
            self.bootstrap_state.clone().unwrap_or_else(|| load_bootstrap_or_default(mode));
        let slot = self.save_slot.clone().unwrap_or_else(|| default_save_slot_path_for_mode(mode));

        let color_theme = presentation::color_adapter::load_builtin_theme("classic")
            .expect("Failed to load classic theme");
        let bevy_theme = presentation::BevyTheme::new(color_theme);

        let (grid_w, grid_h) = (bootstrap.bounds.width as usize, bootstrap.bounds.height as usize);

        app.insert_resource(FrontendRuntime(BevyFrontend::with_seed_and_bootstrap(
            self.session_seed,
            bootstrap,
            slot,
        )))
        .insert_resource(bevy_theme)
        .insert_resource(RuntimeSpriteAtlas(SpriteAtlas::default()))
        .insert_resource(PendingInput::default())
        .insert_resource(RuntimeFrame::default())
        .insert_resource(RuntimeStatus { app_state: AppState::Boot, should_quit: false })
        .insert_resource(ArenaHazardBridgeState::default())
        .add_plugins(simulation::SimulationPlugin::new(grid_w, grid_h, self.session_seed))
        .add_event::<InputActionEvent>()
        .add_systems(
            Update,
            (
                boot_system,
                input_to_action_events_system,
                apply_action_events_system,
                project_frame_system,
                apply_wizard_arena_ca_overlay_system,
                apply_arena_hazard_bridge_system,
                sync_tile_entities_system,
            )
                .chain(),
        );
    }
}

pub fn build_runtime_app(seed: u64) -> App {
    build_runtime_app_with_mode(seed, GameMode::Classic)
}

pub fn build_runtime_app_with_mode(seed: u64, mode: GameMode) -> App {
    let mut app = App::new();
    app.add_plugins(OmegaBevyRuntimePlugin {
        session_seed: seed,
        mode: Some(mode),
        bootstrap_state: Some(load_bootstrap_or_default(mode)),
        save_slot: Some(default_save_slot_path_for_mode(mode)),
    });
    app
}

pub fn build_runtime_app_with_options(
    seed: u64,
    bootstrap_state: GameState,
    save_slot: PathBuf,
) -> App {
    build_runtime_app_with_options_and_mode(seed, bootstrap_state.mode, bootstrap_state, save_slot)
}

pub fn build_runtime_app_with_options_and_mode(
    seed: u64,
    mode: GameMode,
    bootstrap_state: GameState,
    save_slot: PathBuf,
) -> App {
    let mut app = App::new();
    app.add_plugins(OmegaBevyRuntimePlugin {
        session_seed: seed,
        mode: Some(mode),
        bootstrap_state: Some(bootstrap_state),
        save_slot: Some(save_slot),
    });
    app
}

pub fn enqueue_input(app: &mut App, key: BevyKey) {
    app.world_mut().resource_mut::<PendingInput>().keys.push(key);
}

pub fn runtime_status(app: &App) -> RuntimeStatus {
    *app.world().resource::<RuntimeStatus>()
}

pub fn runtime_frame(app: &App) -> Option<RenderFrame> {
    app.world().resource::<RuntimeFrame>().frame.clone()
}

pub fn runtime_tile_count(app: &mut App) -> usize {
    let world = app.world_mut();
    let mut query = world.query_filtered::<Entity, With<RenderTile>>();
    query.iter(world).count()
}

fn boot_system(mut runtime: ResMut<FrontendRuntime>) {
    runtime.0.boot();
}

fn input_to_action_events_system(
    runtime: Res<FrontendRuntime>,
    mut pending: ResMut<PendingInput>,
    mut actions: EventWriter<InputActionEvent>,
) {
    for key in pending.keys.drain(..) {
        actions.send(InputActionEvent(runtime.0.action_for_key(key)));
    }
}

fn apply_action_events_system(
    mut runtime: ResMut<FrontendRuntime>,
    mut status: ResMut<RuntimeStatus>,
    mut actions: EventReader<InputActionEvent>,
) {
    for action in actions.read() {
        runtime.0.apply_action(action.0.clone());
    }
    status.app_state = runtime.0.app_state;
    status.should_quit = runtime.0.should_quit;
}

fn project_frame_system(
    runtime: Res<FrontendRuntime>,
    atlas: Res<RuntimeSpriteAtlas>,
    mut frame: ResMut<RuntimeFrame>,
) {
    frame.frame = runtime.0.render_frame(&atlas.0);
}

fn apply_wizard_arena_ca_overlay_system(
    status: Res<RuntimeStatus>,
    overlay_state: Option<Res<presentation::arena_controls::ArenaOverlayState>>,
    grid: Res<omega_core::simulation::grid::CaGrid>,
    wind: Res<omega_core::simulation::wind::WindGrid>,
    atlas: Res<RuntimeSpriteAtlas>,
    mut frame: ResMut<RuntimeFrame>,
) {
    if status.app_state != AppState::WizardArena {
        return;
    }
    if overlay_state.as_ref().is_some_and(|state| !state.show_ca_overlay) {
        return;
    }
    if let Some(rendered) = frame.frame.as_mut() {
        append_ca_overlay_tiles(rendered, &grid, &wind, &atlas.0);
    }
}

fn site_tile_is_flammable(site: &omega_core::TileSiteCell) -> bool {
    if site.glyph == '"' {
        return true;
    }
    if matches!(site.glyph, '#' | '=' | '~' | '.') {
        return false;
    }
    (site.flags & TILE_FLAG_BLOCK_MOVE) != 0
        || site.glyph.is_ascii_alphabetic()
        || matches!(site.glyph, '+' | '-' | '/' | '\\' | '|' | 'P' | 'D' | 'J')
}

fn site_tile_solid_hint(
    site: &omega_core::TileSiteCell,
) -> Option<omega_core::simulation::state::Solid> {
    use omega_core::simulation::state::Solid;

    if site.glyph == '"' {
        return Some(Solid::Grass);
    }
    if matches!(site.glyph, '#' | '=') {
        return Some(Solid::Stone);
    }
    if site_tile_is_flammable(site) {
        return Some(Solid::Wood);
    }
    None
}

fn sync_site_tile_with_ca(
    site: &mut omega_core::TileSiteCell,
    ca: &mut omega_core::simulation::cell::Cell,
) -> bool {
    use omega_core::simulation::state::{Gas, Liquid, Solid};

    let previous_glyph = site.glyph;
    if ca.solid.is_none() {
        ca.solid = site_tile_solid_hint(site);
    }

    let ca_on_fire = ca.gas == Some(Gas::Fire) || ca.heat >= 185;
    let ca_watered = ca.liquid == Some(Liquid::Water) || ca.wet >= 120;

    if ca_on_fire && site_tile_is_flammable(site) {
        site.flags |= TILE_FLAG_BURNING;
    }
    if ca_watered && (site.flags & TILE_FLAG_BURNING) != 0 {
        site.flags &= !TILE_FLAG_BURNING;
    }
    if ca_on_fire && ca_watered {
        ca.gas = Some(Gas::Steam);
        ca.pressure = ca.pressure.max(40);
        ca.heat = ca.heat.saturating_sub(35);
    }

    if (site.flags & TILE_FLAG_BURNING) != 0 {
        ca.gas = Some(Gas::Fire);
        ca.heat = ca.heat.max(190);
        ca.pressure = ca.pressure.max(30);
    } else if ca.gas == Some(Gas::Fire) && ca.heat < 95 {
        ca.gas = Some(Gas::Smoke);
        ca.pressure = ca.pressure.max(20);
    }

    if site_tile_is_flammable(site)
        && (site.flags & TILE_FLAG_BURNING) != 0
        && ca.gas != Some(Gas::Fire)
        && ca.heat < 95
    {
        site.flags &= !TILE_FLAG_BURNING;
        site.flags |= TILE_FLAG_BURNT;
        if !matches!(site.glyph, '#' | '=') {
            site.glyph = '.';
            site.flags &= !TILE_FLAG_BLOCK_MOVE;
        }
        if ca.solid.is_some_and(|solid| solid != Solid::Stone) {
            ca.solid = Some(Solid::Ash);
        }
    }

    site.glyph != previous_glyph
}

fn can_push_actor_to(state: &GameState, target: Position) -> bool {
    state.bounds.contains(target) && state.tile_is_walkable(target)
}

fn apply_arena_hazard_bridge_system(
    time: Res<Time>,
    mut status: ResMut<RuntimeStatus>,
    mut grid: ResMut<omega_core::simulation::grid::CaGrid>,
    wind: Res<omega_core::simulation::wind::WindGrid>,
    mut runtime: ResMut<FrontendRuntime>,
    mut bridge: ResMut<ArenaHazardBridgeState>,
) {
    use omega_core::simulation::state::{Gas, Liquid};

    if status.app_state != AppState::WizardArena {
        *bridge = ArenaHazardBridgeState::default();
        return;
    }
    if bridge.damage_cooldown_s > 0.0 {
        bridge.damage_cooldown_s = (bridge.damage_cooldown_s - time.delta_secs()).max(0.0);
    }
    if bridge.fire_immunity_s > 0.0 {
        bridge.fire_immunity_s = (bridge.fire_immunity_s - time.delta_secs()).max(0.0);
    }
    if bridge.gas_damage_cooldown_s > 0.0 {
        bridge.gas_damage_cooldown_s = (bridge.gas_damage_cooldown_s - time.delta_secs()).max(0.0);
    }
    if bridge.monster_damage_cooldown_s > 0.0 {
        bridge.monster_damage_cooldown_s =
            (bridge.monster_damage_cooldown_s - time.delta_secs()).max(0.0);
    }
    if bridge.wind_push_cooldown_s > 0.0 {
        bridge.wind_push_cooldown_s = (bridge.wind_push_cooldown_s - time.delta_secs()).max(0.0);
    }

    let mut should_game_over = false;
    {
        let Some(session) = runtime.0.session.as_mut() else {
            *bridge = ArenaHazardBridgeState::default();
            return;
        };

        let map_width = usize::try_from(session.state.bounds.width.max(0)).unwrap_or(0);
        let map_height = usize::try_from(session.state.bounds.height.max(0)).unwrap_or(0);
        let mut glyph_updates: Vec<(Position, char)> = Vec::new();

        for y in 0..map_height {
            for x in 0..map_width {
                if !grid.in_bounds(x as isize, y as isize) {
                    continue;
                }
                let idx = y.saturating_mul(map_width).saturating_add(x);
                if idx >= session.state.site_grid.len() {
                    continue;
                }
                let site = &mut session.state.site_grid[idx];
                let mut ca_cell = *grid.get(x, y);
                let previous_ca = ca_cell;
                let glyph_changed = sync_site_tile_with_ca(site, &mut ca_cell);
                if ca_cell != previous_ca {
                    grid.set_immediate(x, y, ca_cell);
                }
                if glyph_changed {
                    glyph_updates.push((Position { x: x as i32, y: y as i32 }, site.glyph));
                }
            }
        }
        for (pos, glyph) in glyph_updates {
            let _ = session.state.set_map_glyph_at(pos, glyph);
        }

        if bridge.monster_damage_cooldown_s <= 0.0 && !session.state.monsters.is_empty() {
            let mut defeated_indices: Vec<usize> = Vec::new();
            for (idx, monster) in session.state.monsters.iter_mut().enumerate() {
                if !grid.in_bounds(monster.position.x as isize, monster.position.y as isize) {
                    continue;
                }
                let cell = *grid.get(monster.position.x as usize, monster.position.y as usize);
                let damage = if cell.gas == Some(Gas::Fire) || cell.heat >= 180 {
                    2
                } else if matches!(cell.gas, Some(Gas::Smoke) | Some(Gas::Steam)) {
                    1
                } else {
                    0
                };
                if damage > 0 {
                    monster.stats.hp = (monster.stats.hp - damage).max(0);
                    if monster.stats.hp <= 0 {
                        defeated_indices.push(idx);
                    }
                }
            }
            for idx in defeated_indices.into_iter().rev() {
                let name = session.state.monsters[idx].name.clone();
                session.state.monsters.swap_remove(idx);
                session.state.log.push(format!("{name} is consumed by arena hazards."));
            }
            bridge.monster_damage_cooldown_s = 0.75;
        }

        if bridge.wind_push_cooldown_s <= 0.0 {
            let mut player_pushed = false;
            let mut pushed_monsters = 0usize;

            let player_pos = session.state.player.position;
            if wind.in_bounds(player_pos.x as isize, player_pos.y as isize) {
                let vector = wind.get(player_pos.x as usize, player_pos.y as usize);
                if vector.strength >= 100 && (vector.dx != 0 || vector.dy != 0) {
                    let target = Position {
                        x: player_pos.x + i32::from(vector.dx),
                        y: player_pos.y + i32::from(vector.dy),
                    };
                    let blocked_by_monster =
                        session.state.monsters.iter().any(|monster| monster.position == target);
                    if !blocked_by_monster && can_push_actor_to(&session.state, target) {
                        session.state.player.position = target;
                        player_pushed = true;
                    }
                }
            }

            let mut monster_positions: Vec<Position> =
                session.state.monsters.iter().map(|monster| monster.position).collect();
            for idx in 0..monster_positions.len() {
                let pos = monster_positions[idx];
                if !wind.in_bounds(pos.x as isize, pos.y as isize) {
                    continue;
                }
                let vector = wind.get(pos.x as usize, pos.y as usize);
                if vector.strength < 130 || (vector.dx == 0 && vector.dy == 0) {
                    continue;
                }
                let target =
                    Position { x: pos.x + i32::from(vector.dx), y: pos.y + i32::from(vector.dy) };
                if target == session.state.player.position {
                    continue;
                }
                if !can_push_actor_to(&session.state, target) {
                    continue;
                }
                if monster_positions
                    .iter()
                    .enumerate()
                    .any(|(other_idx, other_pos)| other_idx != idx && *other_pos == target)
                {
                    continue;
                }
                monster_positions[idx] = target;
                pushed_monsters = pushed_monsters.saturating_add(1);
            }
            for (monster, new_pos) in
                session.state.monsters.iter_mut().zip(monster_positions.into_iter())
            {
                monster.position = new_pos;
            }

            if player_pushed || pushed_monsters > 0 {
                bridge.wind_push_cooldown_s = 0.2;
                if player_pushed {
                    session
                        .state
                        .log
                        .push("A strong gust shoves you across the arena.".to_string());
                }
                if pushed_monsters > 0 {
                    session.state.log.push(format!("Wind pushes {pushed_monsters} monster(s)."));
                }
            }
        }

        let player_pos = session.state.player.position;
        if !grid.in_bounds(player_pos.x as isize, player_pos.y as isize) {
            bridge.was_in_fire = false;
            bridge.was_in_water = false;
            bridge.was_in_gas = false;
            return;
        }
        let cell = *grid.get(player_pos.x as usize, player_pos.y as usize);
        let in_fire = cell.gas == Some(Gas::Fire) || cell.heat >= 160;
        let in_water = cell.liquid == Some(Liquid::Water) || cell.wet >= 100;
        let in_gas = matches!(cell.gas, Some(Gas::Smoke) | Some(Gas::Steam));

        if in_water {
            bridge.fire_immunity_s = bridge.fire_immunity_s.max(1.25);
            if !bridge.was_in_water {
                session.state.log.push("Arena water cools the flames around you.".to_string());
            }
        }

        if in_fire && bridge.fire_immunity_s <= 0.0 && bridge.damage_cooldown_s <= 0.0 {
            let hp_before = session.state.player.stats.hp.max(0);
            let next_hp = (hp_before - 2).max(0);
            let applied = hp_before - next_hp;
            session.state.player.stats.hp = next_hp;
            bridge.damage_cooldown_s = 0.5;
            if applied > 0 {
                session.state.log.push(format!("Arena fire scorches you for {applied} damage."));
                if session.state.player.stats.hp <= 0 {
                    should_game_over = true;
                }
            }
        }
        if in_gas && bridge.gas_damage_cooldown_s <= 0.0 {
            let hp_before = session.state.player.stats.hp.max(0);
            let next_hp = (hp_before - 1).max(0);
            let applied = hp_before - next_hp;
            session.state.player.stats.hp = next_hp;
            bridge.gas_damage_cooldown_s = 0.8;
            if applied > 0 {
                if cell.gas == Some(Gas::Smoke) {
                    session.state.log.push("Smoke burns your lungs for 1 damage.".to_string());
                } else {
                    session.state.log.push("Steam scalds you for 1 damage.".to_string());
                }
                if session.state.player.stats.hp <= 0 {
                    should_game_over = true;
                }
            }
        }

        if !in_fire && bridge.was_in_fire {
            session.state.log.push("You step clear of the worst heat.".to_string());
        }
        if !in_gas && bridge.was_in_gas {
            session.state.log.push("The air clears enough to breathe again.".to_string());
        }

        bridge.was_in_fire = in_fire;
        bridge.was_in_water = in_water;
        bridge.was_in_gas = in_gas;
    }

    if should_game_over {
        runtime.0.app_state = AppState::GameOver;
        status.app_state = AppState::GameOver;
    }
}

fn append_ca_overlay_tiles(
    rendered: &mut RenderFrame,
    grid: &omega_core::simulation::grid::CaGrid,
    wind: &omega_core::simulation::wind::WindGrid,
    atlas: &SpriteAtlas,
) {
    use omega_core::simulation::cell::Cell;
    use omega_core::simulation::state::{Gas, Liquid, Solid};

    fn wind_glyph(vector: omega_core::simulation::wind::WindVector) -> char {
        match (vector.dx, vector.dy) {
            (1, 0) => '>',
            (-1, 0) => '<',
            (0, 1) => 'v',
            (0, -1) => '^',
            (1, 1) => '\\',
            (1, -1) => '/',
            (-1, 1) => '/',
            (-1, -1) => '\\',
            _ => '~',
        }
    }

    fn overlay_for_cell(
        cell: &Cell,
        wind_vec: omega_core::simulation::wind::WindVector,
        x: usize,
        y: usize,
    ) -> Option<(TileKind, char)> {
        if cell.gas == Some(Gas::Fire) || cell.heat >= 180 {
            return Some((TileKind::Fire, '*'));
        }
        if cell.gas == Some(Gas::Steam) {
            return Some((TileKind::Water, ':'));
        }
        if cell.gas == Some(Gas::Smoke) {
            return Some((TileKind::Feature, ';'));
        }
        if cell.liquid == Some(Liquid::Water) || cell.wet >= 100 {
            return Some((TileKind::Water, '~'));
        }
        if matches!(cell.solid, Some(Solid::Wood)) && x.is_multiple_of(2) && y.is_multiple_of(2) {
            return Some((TileKind::Grass, '"'));
        }
        if wind_vec.strength >= 80 && x.is_multiple_of(3) && y.is_multiple_of(2) {
            return Some((TileKind::Feature, wind_glyph(wind_vec)));
        }
        None
    }

    let width = rendered.bounds.0.max(0) as usize;
    let height = rendered.bounds.1.max(0) as usize;
    let overlay_w = width.min(grid.width()).min(wind.width());
    let overlay_h = height.min(grid.height()).min(wind.height());
    for y in 0..overlay_h {
        for x in 0..overlay_w {
            let cell = grid.get(x, y);
            let wind_vec = wind.get(x, y);
            if let Some((kind, glyph)) = overlay_for_cell(cell, wind_vec, x, y) {
                let position = Position { x: x as i32, y: y as i32 };
                rendered.tiles.push(TileRender {
                    position,
                    kind,
                    sprite: sprite_for_tile_kind(atlas, kind),
                    glyph: Some(glyph),
                });
            }
        }
    }
}

fn sync_tile_entities_system(
    mut commands: Commands,
    frame: Res<RuntimeFrame>,
    existing_tiles: Query<Entity, With<RenderTile>>,
    bevy_theme: Res<presentation::BevyTheme>,
) {
    for entity in existing_tiles.iter() {
        commands.entity(entity).despawn();
    }

    if let Some(rendered) = &frame.frame {
        for tile in &rendered.tiles {
            // Resolve the semantic color for this tile type
            let color_id = tile.kind.to_color_id();
            let tile_color = bevy_theme.resolve(&color_id);

            commands.spawn((
                RenderTile,
                RenderTilePosition(tile.position),
                RenderTileKind(tile.kind),
                RenderTileSprite(tile.sprite.clone()),
                RenderTileColor(tile_color),
            ));
        }
    }
}

pub fn run_headless_bootstrap() -> Result<GameState> {
    let mut app = build_runtime_app(0xBEE5_0001);
    app.update();
    enqueue_input(&mut app, BevyKey::Enter);
    enqueue_input(&mut app, BevyKey::Char(' '));
    app.update();
    let frontend = app.world().resource::<FrontendRuntime>();
    Ok(frontend.0.session.as_ref().expect("session should exist after start").state.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_content::LEGACY_RAMPART_START;
    use omega_core::Stats;

    #[test]
    fn app_state_flow_boot_menu_ingame_pause() {
        let mut app = build_runtime_app(7);
        app.update();
        assert_eq!(runtime_status(&app).app_state, AppState::Menu);

        enqueue_input(&mut app, BevyKey::Enter);
        app.update();
        assert_eq!(runtime_status(&app).app_state, AppState::InGame);

        enqueue_input(&mut app, BevyKey::Esc);
        app.update();
        assert_eq!(runtime_status(&app).app_state, AppState::Pause);

        enqueue_input(&mut app, BevyKey::Esc);
        app.update();
        assert_eq!(runtime_status(&app).app_state, AppState::InGame);
    }

    #[test]
    fn game_over_mapping_supports_continue_and_lowercase_controls() {
        assert_eq!(map_input(AppState::GameOver, BevyKey::Char('c')), InputAction::ReturnToMenu);
        assert_eq!(map_input(AppState::GameOver, BevyKey::Char('q')), InputAction::ReturnToMenu);
        assert_eq!(map_input(AppState::GameOver, BevyKey::Char('r')), InputAction::RestartSession);
        assert_eq!(map_input(AppState::GameOver, BevyKey::Char('n')), InputAction::StartGame);
    }

    #[test]
    fn game_over_transition_is_environment_agnostic() {
        let slot = PathBuf::from("target/test-omega-bevy-gameover-any-env.json");
        let bootstrap = GameState::default();
        let mut runtime = BevyFrontend::with_seed_and_bootstrap(111, bootstrap, slot);
        runtime.boot();
        runtime.apply_action(InputAction::StartGame);
        assert_eq!(runtime.app_state, AppState::InGame);

        {
            let session = runtime.session.as_mut().expect("session started");
            session.state.world_mode = omega_core::WorldMode::Countryside;
            session.state.environment = omega_core::LegacyEnvironment::Countryside;
            session.state.status = SessionStatus::Lost;
        }

        runtime.apply_action(InputAction::Dispatch(Command::Wait));
        assert_eq!(runtime.app_state, AppState::GameOver);
    }

    #[test]
    fn shared_input_mapping_matches_tui_contract() {
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('w')),
            InputAction::Dispatch(Command::Move(Direction::North))
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('h')),
            InputAction::Dispatch(Command::Move(Direction::West))
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('a')),
            InputAction::Dispatch(Command::Legacy { token: "a".to_string() })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('D')),
            InputAction::Dispatch(Command::Attack(Direction::East))
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('g')),
            InputAction::Dispatch(Command::Pickup)
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('3')),
            InputAction::Dispatch(Command::Drop { slot: 2 })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('?')),
            InputAction::Dispatch(Command::Legacy { token: "?".to_string() })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('q')),
            InputAction::Dispatch(Command::Legacy { token: "q".to_string() })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::F12),
            InputAction::Dispatch(Command::Legacy { token: "^g".to_string() })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Ctrl('x')),
            InputAction::Dispatch(Command::Legacy { token: "^x".to_string() })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Enter),
            InputAction::Dispatch(Command::Legacy { token: "<enter>".to_string() })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Backspace),
            InputAction::Dispatch(Command::Legacy { token: "<backspace>".to_string() })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('P')),
            InputAction::Dispatch(Command::Legacy { token: "P".to_string() })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('!')),
            InputAction::Dispatch(Command::Legacy { token: "!".to_string() })
        );
        assert_eq!(
            map_shared_gameplay_key(BevyKey::Char('Q')),
            InputAction::Dispatch(Command::Legacy { token: "Q".to_string() })
        );
        assert_eq!(map_shared_gameplay_key(BevyKey::Char('S')), InputAction::SaveAndQuit);
        assert_eq!(map_shared_gameplay_key(BevyKey::Char('L')), InputAction::LoadSlot);
    }

    #[test]
    fn pending_wizard_esc_is_routed_to_core_cancel() {
        let slot = PathBuf::from("target/test-omega-bevy-wizard-cancel.json");
        let mut runtime = BevyFrontend::with_seed_and_bootstrap(113, GameState::default(), slot);
        runtime.boot();
        runtime.apply_action(InputAction::StartGame);

        {
            let session = runtime.session.as_mut().expect("session started");
            session.state.pending_wizard_interaction =
                Some(omega_core::WizardInteraction::WishTextEntry { blessing: 1 });
        }

        runtime.handle_key(BevyKey::Esc);

        let session = runtime.session.as_ref().expect("session exists");
        assert!(session.state.pending_wizard_interaction.is_none());
        assert!(session.state.log.iter().any(|line| line.contains("Wish canceled")));
    }

    #[test]
    fn site_prompt_locks_directional_input_from_wasd_mapping() {
        let slot = PathBuf::from("target/test-omega-bevy-site-lock.json");
        let mut runtime = BevyFrontend::with_seed_and_bootstrap(114, GameState::default(), slot);
        runtime.boot();
        runtime.apply_action(InputAction::StartGame);

        {
            let session = runtime.session.as_mut().expect("session started");
            session.state.pending_site_interaction = Some(SiteInteractionKind::Temple);
        }

        let before = runtime.session.as_ref().expect("session").state.player.position;
        runtime.handle_key(BevyKey::Char('w'));

        let after = runtime.session.as_ref().expect("session").state.player.position;
        assert_eq!(after, before);
        assert!(
            runtime.session.as_ref().expect("session").state.pending_site_interaction.is_some()
        );
    }

    #[test]
    fn site_prompt_blocks_arrow_movement_unless_direction_modal() {
        let slot = PathBuf::from("target/test-omega-bevy-site-arrow-lock.json");
        let mut runtime = BevyFrontend::with_seed_and_bootstrap(1214, GameState::default(), slot);
        runtime.boot();
        runtime.apply_action(InputAction::StartGame);

        {
            let session = runtime.session.as_mut().expect("session started");
            session.state.pending_site_interaction = Some(SiteInteractionKind::Temple);
        }

        let before = runtime.session.as_ref().expect("session").state.player.position;
        runtime.handle_key(BevyKey::Up);

        let after = runtime.session.as_ref().expect("session").state.player.position;
        assert_eq!(after, before);
        assert!(
            runtime.session.as_ref().expect("session").state.pending_site_interaction.is_some()
        );
    }

    #[test]
    fn item_prompt_locks_directional_input_from_wasd_mapping() {
        let slot = PathBuf::from("target/test-omega-bevy-item-lock.json");
        let mut runtime = BevyFrontend::with_seed_and_bootstrap(115, GameState::default(), slot);
        runtime.boot();
        runtime.apply_action(InputAction::StartGame);

        {
            let session = runtime.session.as_mut().expect("session started");
            session.state.pending_item_prompt = Some(omega_core::ItemPromptInteraction {
                context: omega_core::ItemPromptContext::Drop,
                filter: omega_core::ItemPromptFilter::Any,
                prompt: "Drop which item?".to_string(),
            });
        }

        let before = runtime.session.as_ref().expect("session").state.player.position;
        runtime.handle_key(BevyKey::Char('w'));

        let after = runtime.session.as_ref().expect("session").state.player.position;
        assert_eq!(after, before);
        assert!(runtime.session.as_ref().expect("session").state.pending_item_prompt.is_some());
    }

    #[test]
    fn inventory_modal_show_pack_key_logs_pack_listing() {
        let slot = PathBuf::from("target/test-omega-bevy-inventory-show-pack.json");
        let mut runtime = BevyFrontend::with_seed_and_bootstrap(1203, GameState::default(), slot);
        runtime.boot();
        runtime.apply_action(InputAction::StartGame);

        {
            let session = runtime.session.as_mut().expect("session started");
            session.state.player.inventory.push(omega_core::Item::new(9, "practice blade"));
        }

        runtime.handle_key(BevyKey::Char('i'));
        assert!(
            runtime
                .session
                .as_ref()
                .expect("session")
                .state
                .pending_inventory_interaction
                .is_some()
        );
        let before = runtime.session.as_ref().expect("session").state.player.position;

        runtime.handle_key(BevyKey::Char('s'));

        let session = runtime.session.as_ref().expect("session");
        assert_eq!(session.state.player.position, before);
        assert!(session.state.log.iter().any(|line| line.starts_with("Pack:")));
    }

    #[test]
    fn runtime_projection_populates_tile_entities_and_hud() {
        let mut app = build_runtime_app(9);
        app.update();

        enqueue_input(&mut app, BevyKey::Enter);
        enqueue_input(&mut app, BevyKey::Char(' '));
        app.update();

        let frame = runtime_frame(&app).expect("frame should exist after session start");
        assert!(frame.tiles.iter().any(|tile| tile.kind == TileKind::Player));
        assert!(frame.hud_lines.iter().any(|line| line.contains("Turn")));
        assert!(frame.hud_lines.iter().any(|line| line.contains("HP")));
        assert!(runtime_tile_count(&mut app) > 0);
    }

    #[test]
    fn save_load_and_restart_actions_work() {
        let slot = PathBuf::from("target/test-omega-bevy-slot.json");
        let bootstrap = GameState::default();
        let mut app = build_runtime_app_with_options(33, bootstrap.clone(), slot.clone());

        app.update();
        enqueue_input(&mut app, BevyKey::Enter);
        enqueue_input(&mut app, BevyKey::Char(' '));
        app.update();

        enqueue_input(&mut app, BevyKey::Char('S'));
        app.update();

        enqueue_input(&mut app, BevyKey::Char(' '));
        app.update();

        enqueue_input(&mut app, BevyKey::Char('L'));
        app.update();
        assert_eq!(runtime_status(&app).app_state, AppState::InGame);

        enqueue_input(&mut app, BevyKey::Char('R'));
        app.update();
        assert_eq!(runtime_status(&app).app_state, AppState::InGame);

        let _ = fs::remove_file(slot);
    }

    #[test]
    fn bootstrap_starts_in_rampart_city_context() {
        let state = run_headless_bootstrap().expect("headless bootstrap should run");
        assert_eq!(state.player.position, LEGACY_RAMPART_START);
        assert_eq!(state.topology.city_site_id, 1);
        assert_eq!(state.world_mode, omega_core::WorldMode::DungeonCity);
        assert!(state.log.iter().any(|line| line.contains("Rampart")));
    }

    #[test]
    fn uppercase_directional_keys_move_when_no_adjacent_monster() {
        let slot = PathBuf::from("target/test-omega-bevy-controls.json");
        let mut runtime = BevyFrontend::with_seed_and_bootstrap(55, GameState::default(), slot);
        runtime.boot();
        runtime.apply_action(InputAction::StartGame);

        let start = runtime.session.as_ref().expect("session started").state.player.position;
        runtime.handle_key(BevyKey::Char('D'));
        let pos = runtime.session.as_ref().expect("session exists").state.player.position;
        assert_eq!(pos, Position { x: start.x + 1, y: start.y });
    }

    #[test]
    fn uppercase_directional_keys_attack_when_adjacent_monster_exists() {
        let slot = PathBuf::from("target/test-omega-bevy-controls-attack.json");
        let mut runtime = BevyFrontend::with_seed_and_bootstrap(57, GameState::default(), slot);
        runtime.boot();
        runtime.apply_action(InputAction::StartGame);

        {
            let session = runtime.session.as_mut().expect("session started");
            let target = Position {
                x: session.state.player.position.x + 1,
                y: session.state.player.position.y,
            };
            session.state.spawn_monster(
                "rat",
                target,
                Stats { hp: 6, max_hp: 6, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
            );
        }

        runtime.handle_key(BevyKey::Char('D'));
        let events = runtime
            .session
            .as_ref()
            .expect("session exists")
            .last_outcome
            .as_ref()
            .map(|outcome| outcome.events.as_slice())
            .unwrap_or(&[]);
        assert!(events.iter().any(|event| {
            matches!(
                event,
                omega_core::Event::MonsterAttacked { .. }
                    | omega_core::Event::MonsterDefeated { .. }
            )
        }));
    }

    #[test]
    fn lowercase_directional_keys_bump_attack_when_adjacent_monster_exists() {
        let slot = PathBuf::from("target/test-omega-bevy-controls-bump-attack.json");
        let mut runtime = BevyFrontend::with_seed_and_bootstrap(5901, GameState::default(), slot);
        runtime.boot();
        runtime.apply_action(InputAction::StartGame);

        let start = {
            let session = runtime.session.as_ref().expect("session started");
            session.state.player.position
        };
        {
            let session = runtime.session.as_mut().expect("session started");
            let target = Position {
                x: session.state.player.position.x + 1,
                y: session.state.player.position.y,
            };
            session.state.spawn_monster(
                "rat",
                target,
                Stats { hp: 6, max_hp: 6, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
            );
        }

        runtime.handle_key(BevyKey::Char('d'));
        let session = runtime.session.as_ref().expect("session exists");
        assert_eq!(session.state.player.position, start);
        let events =
            session.last_outcome.as_ref().map(|outcome| outcome.events.as_slice()).unwrap_or(&[]);
        assert!(events.iter().any(|event| {
            matches!(
                event,
                omega_core::Event::MonsterAttacked { .. }
                    | omega_core::Event::MonsterDefeated { .. }
            )
        }));
        assert!(events.iter().all(|event| !matches!(event, omega_core::Event::MoveBlocked { .. })));
    }

    #[test]
    fn projection_prefers_chronological_state_log_lines() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.log = vec!["oldest".to_string(), "middle".to_string(), "newest".to_string()];
        let outcome = Outcome {
            turn: 1,
            minutes: 6,
            status: SessionStatus::InProgress,
            events: vec![omega_core::Event::Waited],
        };
        let frame = project_to_frame(&state, Some(&outcome), &SpriteAtlas::default());
        assert_eq!(
            frame.event_lines,
            vec!["oldest".to_string(), "middle".to_string(), "newest".to_string()]
        );
    }

    #[test]
    fn projection_falls_back_to_human_readable_event_lines_when_log_empty() {
        let state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        let outcome = Outcome {
            turn: 2,
            minutes: 12,
            status: SessionStatus::InProgress,
            events: vec![omega_core::Event::Moved {
                from: Position { x: 2, y: 2 },
                to: Position { x: 3, y: 2 },
            }],
        };
        let frame = project_to_frame(&state, Some(&outcome), &SpriteAtlas::default());
        assert!(frame.event_lines.iter().any(|line| line.contains("moved:")));
    }

    #[test]
    fn projection_hud_includes_mana_totals() {
        let state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        let frame = project_to_frame(&state, None, &SpriteAtlas::default());
        assert!(frame.hud_lines.iter().any(|line| line.contains("Mana ")));
    }

    #[test]
    fn projection_renders_target_cursor_and_projectile_overlays() {
        let mut state = GameState::new(omega_core::MapBounds { width: 8, height: 8 });
        let origin = state.player.position;
        let target = Position { x: origin.x + 1, y: origin.y };
        state.pending_targeting_interaction = Some(omega_core::TargetingInteraction {
            origin,
            cursor: target,
            mode: omega_core::ProjectileKind::MagicMissile,
        });
        state.transient_projectile_path = vec![origin, target];
        state.transient_projectile_impact = Some(target);
        let frame = project_to_frame(&state, None, &SpriteAtlas::default());
        assert!(frame.tiles.iter().any(|tile| tile.kind == TileKind::TargetCursor));
        assert!(frame.tiles.iter().any(|tile| tile.kind == TileKind::ProjectileTrail));
        assert!(frame.tiles.iter().any(|tile| tile.kind == TileKind::ProjectileImpact));
    }

    #[test]
    fn projection_renders_objective_ui_only_in_modern_mode() {
        let mut state = GameState::new(omega_core::MapBounds { width: 8, height: 8 });
        state.mode = GameMode::Modern;
        state.progression.quest_state = omega_core::LegacyQuestState::Active;
        state.progression.main_quest.objective =
            "Report to the Mercenary Guild for your contract.".to_string();
        state.site_grid = vec![omega_core::TileSiteCell::default(); 64];
        state.site_grid[10].aux = omega_core::SITE_AUX_SERVICE_MERC_GUILD;

        let modern_frame = project_to_frame(&state, None, &SpriteAtlas::default());
        assert!(modern_frame.tiles.iter().any(|tile| tile.kind == TileKind::ObjectiveMarker));
        assert!(modern_frame.hud_lines.iter().any(|line| line.contains("Objective ")));

        state.mode = GameMode::Classic;
        let classic_frame = project_to_frame(&state, None, &SpriteAtlas::default());
        assert!(!classic_frame.tiles.iter().any(|tile| tile.kind == TileKind::ObjectiveMarker));
        assert!(!classic_frame.hud_lines.iter().any(|line| line.contains("Objective ")));
    }

    #[test]
    fn projection_pins_active_interaction_prompt_and_filters_prompt_noise() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.pending_site_interaction = Some(SiteInteractionKind::Temple);
        state.log = vec![
            "Temple: [1/t] tithe (15g) [2/p] pray [3/b] blessing (35g) [4/s] sanctuary [5/x] leave | favor=0 gold=200".to_string(),
            "Site prompt active: choose a bracketed option, or press q/x to close.".to_string(),
            "Selected option 1. You make a tithe at the temple.".to_string(),
        ];

        let frame = project_to_frame(&state, None, &SpriteAtlas::default());

        assert!(
            frame.event_lines.iter().any(|line| line.contains("ACTIVE: Temple: [1/t] tithe (15g)"))
        );
        assert!(frame.event_lines.iter().any(|line| {
            line.contains("Temple prompt active: choose 1-5 or letter aliases shown in brackets.")
        }));
        assert!(
            !frame
                .event_lines
                .iter()
                .any(|line| line.contains("Site prompt active: choose a bracketed option"))
        );
    }

    #[test]
    fn projection_surfaces_activation_prompt() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.pending_activation_interaction = Some(omega_core::ActivationInteraction::ChooseKind);

        let frame = project_to_frame(&state, None, &SpriteAtlas::default());

        assert!(frame.hud_lines.iter().any(|line| line.contains("Interaction activation prompt")));
        assert!(
            frame
                .event_lines
                .iter()
                .any(|line| line.contains("Activate -- item [i] or artifact [a]"))
        );
    }

    #[test]
    fn sync_site_tile_with_ca_marks_flammable_structure_as_burning() {
        use omega_core::simulation::cell::Cell;
        use omega_core::simulation::state::Gas;

        let mut site = omega_core::TileSiteCell {
            glyph: 'h',
            flags: TILE_FLAG_BLOCK_MOVE,
            ..Default::default()
        };
        let mut ca = Cell { gas: Some(Gas::Fire), heat: 210, ..Cell::default() };

        let glyph_changed = sync_site_tile_with_ca(&mut site, &mut ca);

        assert!(!glyph_changed);
        assert!((site.flags & TILE_FLAG_BURNING) != 0);
        assert_eq!(ca.gas, Some(Gas::Fire));
    }

    #[test]
    fn sync_site_tile_with_ca_water_creates_steam_and_extinguishes() {
        use omega_core::simulation::cell::Cell;
        use omega_core::simulation::state::{Gas, Liquid};

        let mut site =
            omega_core::TileSiteCell { glyph: '"', flags: TILE_FLAG_BURNING, ..Default::default() };
        let mut ca = Cell {
            gas: Some(Gas::Fire),
            liquid: Some(Liquid::Water),
            heat: 220,
            wet: 220,
            ..Cell::default()
        };

        let _ = sync_site_tile_with_ca(&mut site, &mut ca);

        assert_eq!(ca.gas, Some(Gas::Steam));
        assert_eq!(site.flags & TILE_FLAG_BURNING, 0);
    }

    #[test]
    fn ca_overlay_renders_fire_steam_and_smoke_glyphs() {
        use omega_core::simulation::cell::Cell;
        use omega_core::simulation::grid::CaGrid;
        use omega_core::simulation::state::Gas;
        use omega_core::simulation::wind::WindGrid;

        let mut frame = RenderFrame {
            mode: GameMode::Classic,
            bounds: (3, 1),
            tiles: Vec::new(),
            hud_lines: Vec::new(),
            interaction_lines: Vec::new(),
            timeline_lines: Vec::new(),
            event_lines: Vec::new(),
        };
        let mut grid = CaGrid::new(3, 1);
        let wind = WindGrid::new(3, 1);

        grid.set_immediate(0, 0, Cell { gas: Some(Gas::Steam), ..Cell::default() });
        grid.set_immediate(1, 0, Cell { gas: Some(Gas::Smoke), ..Cell::default() });
        grid.set_immediate(2, 0, Cell { gas: Some(Gas::Fire), ..Cell::default() });

        append_ca_overlay_tiles(&mut frame, &grid, &wind, &SpriteAtlas::default());

        assert!(frame.tiles.iter().any(|tile| tile.glyph == Some(':')));
        assert!(frame.tiles.iter().any(|tile| tile.glyph == Some(';')));
        assert!(frame.tiles.iter().any(|tile| tile.glyph == Some('*')));
    }
}
