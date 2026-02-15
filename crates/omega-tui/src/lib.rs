use anyhow::{Context, Result};
use crossterm::event::{self, Event as CEvent, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{ExecutableCommand, execute};
use omega_content::bootstrap_game_state_with_mode;
use omega_core::color::AnimationKind;
use omega_core::{
    Command, DeterministicRng, Direction, Event, GameMode, GameState, ModalInputProfile, Outcome,
    Position, SessionStatus, SiteInteractionKind, active_activation_interaction_help_hint,
    active_activation_interaction_prompt, active_inventory_interaction_help_hint,
    active_inventory_interaction_prompt, active_item_prompt, active_item_prompt_help_hint,
    active_objective_snapshot, active_quit_interaction_help_hint, active_quit_interaction_prompt,
    active_site_interaction_help_hint, active_site_interaction_prompt,
    active_spell_interaction_help_hint, active_spell_interaction_prompt,
    active_talk_direction_help_hint, active_talk_direction_prompt,
    active_targeting_interaction_help_hint, active_targeting_interaction_prompt,
    active_wizard_interaction_help_hint, active_wizard_interaction_prompt, modal_input_profile,
    objective_map_hints, renderable_timeline_lines, sanitize_legacy_prompt_noise, step,
};
use omega_save::{decode_state_json_for_mode, encode_json};
use ratatui::backend::{CrosstermBackend, TestBackend};
use ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use std::collections::HashSet;
use std::fs;
use std::io::{Stdout, stdout};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub mod arena;
pub mod color_adapter;
pub use color_adapter::{StyleCache, colorspec_to_ratatui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiKey {
    Char(char),
    Ctrl(char),
    Up,
    Down,
    Left,
    Right,
    WizardToggle,
    Enter,
    Backspace,
    Esc,
    ThemeCycle,
    Mouse(crossterm::event::MouseEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    Dispatch(Command),
    SaveSlot,
    SaveAndQuit,
    LoadSlot,
    Restart,
    NewGame,
    Quit,
    None,
}

#[derive(Debug, Clone)]
pub struct App {
    pub state: GameState,
    pub quit: bool,
    pub last_outcome: Option<Outcome>,
    pub save_slot: PathBuf,
    pub bootstrap_state: GameState,
    pub mode: GameMode,
    pub theme: omega_core::color::ColorTheme,
    pub style_cache: StyleCache,
    pub capability: omega_core::color::ColorCapability,
    pub animation_time: f32,
    pub arena_ui: Option<arena::ArenaUi>,
    pub ca_grid: Option<omega_core::simulation::CaGrid>,
    pub wind_grid: Option<omega_core::simulation::WindGrid>,
    pub last_map_area: Rect,
    rng: DeterministicRng,
    seed: u64,
    restart_count: u64,
}

impl Default for App {
    fn default() -> Self {
        Self::new(0x0BAD_5EED)
    }
}

impl App {
    fn map_ctrl_legacy(ch: char) -> Option<String> {
        let lowered = ch.to_ascii_lowercase();
        if matches!(lowered, 'f' | 'g' | 'i' | 'k' | 'l' | 'o' | 'p' | 'r' | 'w' | 'x') {
            Some(format!("^{lowered}"))
        } else {
            None
        }
    }

    pub fn new(seed: u64) -> Self {
        Self::new_with_mode(seed, GameMode::Classic)
    }

    pub fn new_with_mode(seed: u64, mode: GameMode) -> Self {
        let bootstrap = load_bootstrap_or_default(mode);
        let slot = default_save_slot_path_for_mode(mode);
        Self::with_options(seed, bootstrap.clone(), bootstrap, slot)
    }

    pub fn with_options(
        seed: u64,
        initial_state: GameState,
        bootstrap_state: GameState,
        save_slot: PathBuf,
    ) -> Self {
        let mode = initial_state.mode;

        // Load classic theme embedded at compile time
        const CLASSIC_THEME_TOML: &str = include_str!("../../omega-content/themes/classic.toml");
        let theme = omega_core::color::ColorTheme::from_toml(CLASSIC_THEME_TOML)
            .unwrap_or_else(|e| panic!("Failed to load embedded classic theme: {}", e));

        // Detect terminal capability
        let capability = omega_core::color::ColorCapability::detect();

        // Add default animations
        let mut theme = theme;
        let (hp_low_fg, _) = theme
            .resolve(&omega_core::color::ColorId::Ui(omega_core::color::UiColorId::HealthLow))
            .unwrap_or((
                omega_core::color::HexColor::from_hex("#FF0000").unwrap(),
                omega_core::color::HexColor::from_hex("#000000").unwrap(),
            ));

        theme.animations.insert(
            "ui.healthlow".to_string(),
            AnimationKind::Flash {
                colors: (hp_low_fg.into(), omega_core::color::ColorSpec::Rgb { r: 0, g: 0, b: 0 }),
                frequency: 2.0,
            },
        );

        let (highlight_fg, _) = theme
            .resolve(&omega_core::color::ColorId::Ui(omega_core::color::UiColorId::Highlight))
            .unwrap_or((
                omega_core::color::HexColor::from_hex("#FFFF00").unwrap(),
                omega_core::color::HexColor::from_hex("#000000").unwrap(),
            ));

        theme.animations.insert(
            "ui.highlight".to_string(),
            AnimationKind::Pulse {
                base: highlight_fg.into(),
                target: omega_core::color::ColorSpec::Rgb { r: 255, g: 255, b: 255 },
                frequency: 1.0,
            },
        );

        // Create style cache
        let style_cache = StyleCache::new(&theme, capability);

        let (arena_ui, ca_grid, wind_grid) =
            if initial_state.environment == omega_core::LegacyEnvironment::Arena {
                (
                    Some(arena::ArenaUi::default()),
                    Some(omega_core::simulation::CaGrid::new(
                        initial_state.bounds.width as usize,
                        initial_state.bounds.height as usize,
                    )),
                    Some(omega_core::simulation::WindGrid::new(
                        initial_state.bounds.width as usize,
                        initial_state.bounds.height as usize,
                    )),
                )
            } else {
                (None, None, None)
            };

        Self {
            state: initial_state,
            quit: false,
            last_outcome: None,
            save_slot,
            bootstrap_state,
            mode,
            theme,
            style_cache,
            capability,
            animation_time: 0.0,
            arena_ui,
            ca_grid,
            wind_grid,
            last_map_area: Rect::default(),
            rng: DeterministicRng::seeded(seed),
            seed,
            restart_count: 0,
        }
    }

    /// Replaces the theme and rebuilds the style cache.
    ///
    /// Useful for runtime theme switching or testing.
    pub fn with_theme(mut self, theme: omega_core::color::ColorTheme) -> Self {
        self.style_cache = StyleCache::new(&theme, self.capability);
        self.theme = theme;
        self
    }

    /// Switches to a new theme and rebuilds the style cache.
    ///
    /// This is the mutable variant of with_theme for runtime switching.
    pub fn switch_theme(&mut self, theme: omega_core::color::ColorTheme) {
        self.style_cache = StyleCache::new(&theme, self.capability);
        self.theme = theme;
    }

    /// Cycles between built-in themes (classic <-> accessible).
    fn cycle_theme(&mut self) {
        let next_theme_name =
            if self.theme.meta.name.contains("Classic") { "accessible" } else { "classic" };

        if let Ok(theme) = color_adapter::load_builtin_theme(next_theme_name) {
            let theme_name = theme.meta.name.clone();
            self.switch_theme(theme);
            self.state.log.push(format!("Theme switched to: {}", theme_name));
        } else {
            self.state.log.push(format!("Failed to load {} theme", next_theme_name));
        }
    }

    fn has_modal_interaction(&self) -> bool {
        self.state.pending_wizard_interaction.is_some()
            || self.state.pending_spell_interaction.is_some()
            || self.state.pending_activation_interaction.is_some()
            || self.state.pending_quit_interaction.is_some()
            || self.state.pending_talk_direction.is_some()
            || self.state.pending_inventory_interaction.is_some()
            || self.state.pending_item_prompt.is_some()
            || self.state.pending_targeting_interaction.is_some()
            || self.state.pending_site_interaction.is_some()
    }

    pub fn map_input(key: UiKey) -> UiAction {
        match key {
            UiKey::Esc => UiAction::Quit,
            UiKey::ThemeCycle => UiAction::None, // Only handled in handle_key directly
            UiKey::WizardToggle => UiAction::Dispatch(Command::Legacy { token: "^g".to_string() }),
            UiKey::Enter => UiAction::Dispatch(Command::Legacy { token: "<enter>".to_string() }),
            UiKey::Backspace => {
                UiAction::Dispatch(Command::Legacy { token: "<backspace>".to_string() })
            }
            UiKey::Ctrl(ch) => {
                if let Some(token) = Self::map_ctrl_legacy(ch) {
                    UiAction::Dispatch(Command::Legacy { token })
                } else {
                    UiAction::None
                }
            }
            UiKey::Up => UiAction::Dispatch(Command::Move(Direction::North)),
            UiKey::Down => UiAction::Dispatch(Command::Move(Direction::South)),
            UiKey::Left => UiAction::Dispatch(Command::Move(Direction::West)),
            UiKey::Right => UiAction::Dispatch(Command::Move(Direction::East)),
            UiKey::Char(ch) => match ch {
                'Q' => UiAction::Dispatch(Command::Legacy { token: "Q".to_string() }),
                ' ' | '.' => UiAction::Dispatch(Command::Wait),
                'w' | 'k' => UiAction::Dispatch(Command::Move(Direction::North)),
                's' | 'j' => UiAction::Dispatch(Command::Move(Direction::South)),
                'h' => UiAction::Dispatch(Command::Move(Direction::West)),
                'd' | 'l' => UiAction::Dispatch(Command::Move(Direction::East)),
                'W' => UiAction::Dispatch(Command::Attack(Direction::North)),
                'X' => UiAction::Dispatch(Command::Attack(Direction::South)),
                'A' => UiAction::Dispatch(Command::Attack(Direction::West)),
                'D' => UiAction::Dispatch(Command::Attack(Direction::East)),
                'g' => UiAction::Dispatch(Command::Pickup),
                ',' | '@' | '<' | '>' | '?' | '/' => {
                    UiAction::Dispatch(Command::Legacy { token: ch.to_string() })
                }
                'a' | 'e' | 'f' | 'm' | 'o' | 'p' | 'r' | 't' | 'v' | 'x' | 'z' | 'c' => {
                    UiAction::Dispatch(Command::Legacy { token: ch.to_string() })
                }
                'C' | 'E' | 'F' | 'G' | 'H' | 'I' | 'M' | 'O' | 'T' | 'V' | 'Z' => {
                    UiAction::Dispatch(Command::Legacy { token: ch.to_string() })
                }
                'u' | 'y' | 'b' | 'n' => {
                    UiAction::Dispatch(Command::Legacy { token: ch.to_string() })
                }
                '1'..='9' => UiAction::Dispatch(Command::Drop { slot: (ch as u8 - b'1') as usize }),
                'P' => UiAction::Dispatch(Command::Legacy { token: ch.to_string() }),
                'S' => UiAction::SaveAndQuit,
                'L' => UiAction::LoadSlot,
                'R' => UiAction::Restart,
                'N' => UiAction::NewGame,
                'q' => UiAction::Dispatch(Command::Legacy { token: "q".to_string() }),
                _ if ch.is_ascii_graphic() => {
                    UiAction::Dispatch(Command::Legacy { token: ch.to_string() })
                }
                _ => UiAction::None,
            },
            UiKey::Mouse(_) => UiAction::None,
        }
    }

    pub fn handle_key(&mut self, key: UiKey) {
        // Handle theme cycling early - works in any state
        if key == UiKey::ThemeCycle {
            self.cycle_theme();
            return;
        }

        // Arena Controls
        if let Some(arena_ui) = &mut self.arena_ui
            && let (Some(grid), Some(wind)) = (&mut self.ca_grid, &mut self.wind_grid)
        {
            match key {
                UiKey::Mouse(mouse) => {
                    if arena_ui.tooling_enabled {
                        arena_ui.handle_brush_paint(mouse, self.last_map_area, grid);
                        return;
                    }
                }
                _ => {
                    let key_code = match key {
                        UiKey::Char('\t') => KeyCode::Tab,
                        UiKey::Char(c) => KeyCode::Char(c),
                        UiKey::Up => KeyCode::Up,
                        UiKey::Down => KeyCode::Down,
                        UiKey::Enter => KeyCode::Enter,
                        UiKey::Esc => KeyCode::Esc,
                        UiKey::Backspace => KeyCode::Backspace,
                        _ => KeyCode::Null,
                    };
                    let action = arena_ui.handle_arena_input(
                        key_code,
                        grid,
                        wind,
                        self.state.player.position,
                    );
                    match action {
                        arena::ArenaAction::Consumed => return,
                        arena::ArenaAction::SpawnMonster { name, stats } => {
                            let spawn = [
                                self.state.player.position.offset(Direction::East),
                                self.state.player.position.offset(Direction::West),
                                self.state.player.position.offset(Direction::North),
                                self.state.player.position.offset(Direction::South),
                            ]
                            .into_iter()
                            .find(|candidate| {
                                self.state.bounds.contains(*candidate)
                                    && self.state.tile_is_walkable(*candidate)
                            })
                            .unwrap_or(self.state.player.position);
                            self.state.spawn_monster(name.clone(), spawn, stats);
                            arena_ui.log_event(&format!(
                                "Spawned {} at ({}, {}).",
                                name, spawn.x, spawn.y
                            ));
                            return;
                        }
                        arena::ArenaAction::SpawnItem { name } => {
                            let pos = self.state.player.position;
                            if name == "fire" {
                                if let Some(cell) = self.state.tile_site_at_mut(pos) {
                                    cell.flags |= omega_core::TILE_FLAG_BURNING;
                                    arena_ui.log_event(&format!(
                                        "Ignited tile at ({}, {}).",
                                        pos.x, pos.y
                                    ));
                                } else {
                                    arena_ui.log_event(&format!(
                                        "Fire spawn failed at ({}, {}).",
                                        pos.x, pos.y
                                    ));
                                }
                            } else {
                                self.state.place_item(name.clone(), pos);
                                arena_ui.log_event(&format!(
                                    "Spawned item {} at ({}, {}).",
                                    name, pos.x, pos.y
                                ));
                            }
                            return;
                        }
                        arena::ArenaAction::SpawnHazard { effect_id, damage } => {
                            let pos = self.state.player.position;
                            let trap_id = self.state.place_trap(pos, damage, effect_id.clone());
                            arena_ui.log_event(&format!(
                                "Spawned hazard {} (id {}) at ({}, {}).",
                                effect_id, trap_id, pos.x, pos.y
                            ));
                            return;
                        }
                        arena::ArenaAction::ClearMonsters => {
                            let removed = self.state.monsters.len();
                            self.state.monsters.clear();
                            arena_ui.log_event(&format!("Cleared {} monster(s).", removed));
                            return;
                        }
                        arena::ArenaAction::ClearItems => {
                            let removed = self.state.ground_items.len();
                            self.state.ground_items.clear();
                            arena_ui.log_event(&format!("Cleared {} item(s).", removed));
                            return;
                        }
                        arena::ArenaAction::ToggleAiPaused => {
                            self.state.ai_paused = !self.state.ai_paused;
                            arena_ui.log_event(if self.state.ai_paused {
                                "Monster turns paused."
                            } else {
                                "Monster turns resumed."
                            });
                            return;
                        }
                        arena::ArenaAction::ResetArenaFixture => {
                            self.state = self.bootstrap_state.clone();
                            self.last_outcome = None;
                            *arena_ui = arena::ArenaUi::default();
                            self.ca_grid = Some(omega_core::simulation::CaGrid::new(
                                self.state.bounds.width.max(1) as usize,
                                self.state.bounds.height.max(1) as usize,
                            ));
                            self.wind_grid = Some(omega_core::simulation::WindGrid::new(
                                self.state.bounds.width.max(1) as usize,
                                self.state.bounds.height.max(1) as usize,
                            ));
                            arena_ui.log_event("Arena fixture reset.");
                            return;
                        }
                        arena::ArenaAction::None => {}
                    }
                }
            }
        }

        if self.state.is_terminal() && self.handle_terminal_key(key) {
            return;
        }
        if self.has_modal_interaction() {
            let modal_profile = modal_input_profile(&self.state);
            let action = match key {
                UiKey::Esc => UiAction::Dispatch(Command::Legacy { token: "<esc>".to_string() }),
                UiKey::Enter => {
                    UiAction::Dispatch(Command::Legacy { token: "<enter>".to_string() })
                }
                UiKey::Backspace => {
                    UiAction::Dispatch(Command::Legacy { token: "<backspace>".to_string() })
                }
                UiKey::Ctrl(ch) => {
                    if let Some(token) = Self::map_ctrl_legacy(ch) {
                        UiAction::Dispatch(Command::Legacy { token })
                    } else {
                        UiAction::None
                    }
                }
                UiKey::Char(ch) => UiAction::Dispatch(Command::Legacy { token: ch.to_string() }),
                UiKey::Up => {
                    if modal_profile == ModalInputProfile::DirectionEntry {
                        UiAction::Dispatch(Command::Move(Direction::North))
                    } else {
                        UiAction::None
                    }
                }
                UiKey::Down => {
                    if modal_profile == ModalInputProfile::DirectionEntry {
                        UiAction::Dispatch(Command::Move(Direction::South))
                    } else {
                        UiAction::None
                    }
                }
                UiKey::Left => {
                    if modal_profile == ModalInputProfile::DirectionEntry {
                        UiAction::Dispatch(Command::Move(Direction::West))
                    } else {
                        UiAction::None
                    }
                }
                UiKey::Right => {
                    if modal_profile == ModalInputProfile::DirectionEntry {
                        UiAction::Dispatch(Command::Move(Direction::East))
                    } else {
                        UiAction::None
                    }
                }
                UiKey::Mouse(_) => UiAction::None,
                UiKey::ThemeCycle | UiKey::WizardToggle => UiAction::None,
            };
            self.apply_action(action);
            return;
        }
        if let UiKey::Char(ch) = key
            && !self.has_modal_interaction()
            && let Some(command) = self.adaptive_directional_command(ch)
        {
            self.apply_action(UiAction::Dispatch(command));
            return;
        }
        self.apply_action(Self::map_input(key));
    }

    fn handle_terminal_key(&mut self, key: UiKey) -> bool {
        match key {
            UiKey::Esc => {
                self.quit = true;
                self.state.log.push("Game over acknowledged; returning to launcher.".to_string());
                true
            }
            UiKey::Char(ch) => {
                if ch.eq_ignore_ascii_case(&'c') || ch.eq_ignore_ascii_case(&'q') {
                    self.quit = true;
                    self.state
                        .log
                        .push("Game over acknowledged; returning to launcher.".to_string());
                    true
                } else if ch.eq_ignore_ascii_case(&'r') || ch.eq_ignore_ascii_case(&'n') {
                    self.restart_from_bootstrap();
                    true
                } else if ch.eq_ignore_ascii_case(&'l') {
                    if let Err(err) = self.load_from_slot() {
                        self.state.log.push(format!("Load failed: {err}"));
                    }
                    true
                } else {
                    self.state.log.push(
                        "Game over: press c/q/esc to continue, r/n to restart, or l to load."
                            .to_string(),
                    );
                    true
                }
            }
            _ => true,
        }
    }

    fn adaptive_directional_command(&self, ch: char) -> Option<Command> {
        let direction = match ch {
            'W' => Direction::North,
            'X' => Direction::South,
            'A' => Direction::West,
            'D' => Direction::East,
            _ => return None,
        };

        let target = self.state.player.position.offset(direction);
        let has_adjacent_monster =
            self.state.monsters.iter().any(|monster| monster.position == target);
        if has_adjacent_monster {
            Some(Command::Attack(direction))
        } else {
            Some(Command::Move(direction))
        }
    }

    pub fn apply_action(&mut self, action: UiAction) {
        match action {
            UiAction::Quit => {
                self.quit = true;
            }
            UiAction::Dispatch(command) => {
                let was_in_progress = self.state.status == SessionStatus::InProgress;
                let old_env = self.state.environment;
                let outcome = step(&mut self.state, command, &mut self.rng);

                // Check for environment change
                if old_env != self.state.environment {
                    let next_theme =
                        omega_core::color::ColorTheme::name_for_environment(self.state.environment);
                    if self.theme.meta.name.to_lowercase() != next_theme
                        && let Ok(theme) = color_adapter::load_builtin_theme(next_theme)
                    {
                        self.switch_theme(theme);
                    }
                }

                if was_in_progress && self.state.status != SessionStatus::InProgress {
                    let prompt = match self.state.status {
                        SessionStatus::Lost => {
                            "You have died. Press c/q/esc to continue, r/n to restart, or l to load."
                        }
                        SessionStatus::Won => {
                            "Victory complete. Press c/q/esc to continue, r/n to restart, or l to load."
                        }
                        SessionStatus::InProgress => "",
                    };
                    if !prompt.is_empty() {
                        self.state.log.push(prompt.to_string());
                    }
                }
                self.last_outcome = Some(outcome);
            }
            UiAction::SaveSlot => {
                if let Err(err) = self.save_to_slot() {
                    self.state.log.push(format!("Save failed: {err}"));
                }
            }
            UiAction::SaveAndQuit => {
                if let Err(err) = self.save_to_slot() {
                    self.state.log.push(format!("Save failed: {err}"));
                } else {
                    self.quit = true;
                    self.state.log.push("Saved and quit.".to_string());
                }
            }
            UiAction::LoadSlot => {
                if let Err(err) = self.load_from_slot() {
                    self.state.log.push(format!("Load failed: {err}"));
                }
            }
            UiAction::Restart | UiAction::NewGame => {
                self.restart_from_bootstrap();
            }
            UiAction::None => {}
        }
    }

    pub fn save_to_slot(&mut self) -> Result<()> {
        let raw = encode_json(&self.state).context("encode save slot")?;
        if let Some(parent) = self.save_slot.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)
                .with_context(|| format!("create save slot directory {}", parent.display()))?;
        }
        fs::write(&self.save_slot, raw)
            .with_context(|| format!("write save slot {}", self.save_slot.display()))?;
        self.state.log.push(format!("Saved slot: {}", self.save_slot.display()));
        Ok(())
    }

    pub fn load_from_slot(&mut self) -> Result<()> {
        let raw = fs::read_to_string(&self.save_slot)
            .with_context(|| format!("read save slot {}", self.save_slot.display()))?;
        let mut loaded = decode_state_json_for_mode(&raw, self.mode).context("decode save slot")?;
        loaded.options.interactive_sites = true;
        sanitize_legacy_prompt_noise(&mut loaded.log);
        self.state = loaded;
        self.last_outcome = None;
        self.state.log.push(format!("Loaded slot: {}", self.save_slot.display()));
        Ok(())
    }

    pub fn restart_from_bootstrap(&mut self) {
        self.restart_count = self.restart_count.wrapping_add(1);
        self.rng = DeterministicRng::seeded(self.seed.wrapping_add(self.restart_count));
        self.state = self.bootstrap_state.clone();
        self.last_outcome = None;
        self.state.log.push("Session restarted from bootstrap.".to_string());
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

pub fn run_scripted_session(keys: impl IntoIterator<Item = UiKey>) -> App {
    let mut app = App::default();
    for key in keys {
        app.handle_key(key);
        if app.quit {
            break;
        }
    }
    app
}

pub fn run_ratatui_app(seed: u64) -> Result<GameState> {
    run_ratatui_app_with_mode(seed, GameMode::Classic)
}

pub fn run_ratatui_app_with_mode(seed: u64, mode: GameMode) -> Result<GameState> {
    let bootstrap = load_bootstrap_or_default(mode);
    run_ratatui_app_with_options(
        seed,
        bootstrap.clone(),
        bootstrap,
        default_save_slot_path_for_mode(mode),
    )
}

pub fn run_ratatui_app_with_options(
    seed: u64,
    initial_state: GameState,
    bootstrap_state: GameState,
    save_slot: PathBuf,
) -> Result<GameState> {
    let mut app = App::with_options(seed, initial_state, bootstrap_state, save_slot);
    let mut terminal = setup_terminal()?;

    while !app.quit {
        terminal.draw(|frame| render_frame(frame, &app))?;

        let poll_duration = Duration::from_millis(50);
        if event::poll(poll_duration)? {
            let maybe_key = read_ui_key()?;
            if let Some(key) = maybe_key {
                let size = terminal.size()?;
                app.last_map_area = calculate_map_area(
                    Rect::new(0, 0, size.width, size.height),
                    app.arena_ui.is_some(),
                );
                app.handle_key(key);
            }
        }
        app.animation_time += poll_duration.as_secs_f32();
    }

    restore_terminal(&mut terminal)?;
    Ok(app.state)
}

pub fn run_ratatui_app_themed(
    seed: u64,
    initial_state: GameState,
    bootstrap_state: GameState,
    save_slot: PathBuf,
    theme: omega_core::color::ColorTheme,
) -> Result<GameState> {
    let mut app =
        App::with_options(seed, initial_state, bootstrap_state, save_slot).with_theme(theme);
    let mut terminal = setup_terminal()?;

    while !app.quit {
        terminal.draw(|frame| render_frame(frame, &app))?;

        let poll_duration = Duration::from_millis(50);
        if event::poll(poll_duration)? {
            let maybe_key = read_ui_key()?;
            if let Some(key) = maybe_key {
                let size = terminal.size()?;
                app.last_map_area = calculate_map_area(
                    Rect::new(0, 0, size.width, size.height),
                    app.arena_ui.is_some(),
                );
                app.handle_key(key);
            }
        }
        app.animation_time += poll_duration.as_secs_f32();
    }

    restore_terminal(&mut terminal)?;
    Ok(app.state)
}

pub fn run_headless_bootstrap() -> Result<GameState> {
    let app = run_scripted_session([UiKey::Char(' ')]);
    Ok(app.state)
}

pub fn render_screen(app: &App) -> String {
    render_to_string_with_ratatui(app, 100, 36)
}

pub fn render_to_string_with_ratatui(app: &App, width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).expect("test backend terminal should initialize");
    terminal.draw(|frame| render_frame(frame, app)).expect("drawing to test backend should work");

    let backend = terminal.backend();
    let buffer = backend.buffer();
    let area = buffer.area;

    let mut lines = Vec::new();
    for y in area.top()..area.bottom() {
        let mut line = String::new();
        for x in area.left()..area.right() {
            line.push_str(buffer[(x, y)].symbol());
        }
        lines.push(line.trim_end().to_string());
    }
    lines.join("\n")
}

pub fn calculate_map_area(total_area: Rect, arena_active: bool) -> Rect {
    let game_area = if arena_active {
        let chunks = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(total_area);
        chunks[0]
    } else {
        total_area
    };

    let rows = Layout::default()
        .direction(LayoutDirection::Vertical)
        .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
        .split(game_area);

    let top = Layout::default()
        .direction(LayoutDirection::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(rows[0]);

    // Account for Borders::ALL
    top[0].inner(Margin { vertical: 1, horizontal: 1 })
}

pub fn render_frame(frame: &mut Frame, app: &App) {
    if app.state.is_terminal() {
        let title = match app.state.status {
            SessionStatus::Lost => "DEATH",
            SessionStatus::Won => "VICTORY",
            SessionStatus::InProgress => "SESSION",
        };
        let terminal_panel =
            Paragraph::new(render_terminal_panel(&app.state, &app.style_cache, &app.save_slot))
                .block(Block::default().title(title).borders(Borders::ALL))
                .wrap(Wrap { trim: false });
        frame.render_widget(terminal_panel, frame.area());
        return;
    }

    let (game_area, arena_area) = if app.arena_ui.is_some() {
        let chunks = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(frame.area());
        (chunks[0], Some(chunks[1]))
    } else {
        (frame.area(), None)
    };

    if let Some(arena_ui) = &app.arena_ui
        && let Some(area) = arena_area
    {
        arena_ui.render_controls_panel(frame, area);
    }

    let rows = Layout::default()
        .direction(LayoutDirection::Vertical)
        .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
        .split(game_area);

    let top = Layout::default()
        .direction(LayoutDirection::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(rows[0]);

    let bottom = Layout::default()
        .direction(LayoutDirection::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(rows[1]);

    let map_view_width = top[0].width.saturating_sub(2).max(1);
    let map_view_height = top[0].height.saturating_sub(2).max(1);
    let map = Paragraph::new(render_map_panel(
        &app.state,
        &app.style_cache,
        map_view_width,
        map_view_height,
    ))
    .block(Block::default().title("MAP").borders(Borders::ALL))
    .wrap(Wrap { trim: false });

    let status = Paragraph::new(render_status_panel(
        &app.state,
        &app.style_cache,
        &app.save_slot,
        &app.theme,
        app.animation_time,
    ))
    .block(Block::default().title("STATUS").borders(Borders::ALL))
    .wrap(Wrap { trim: false });

    let inventory = Paragraph::new(render_inventory_panel(&app.state, &app.style_cache))
        .block(Block::default().title("INVENTORY").borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    let interaction_rows = Layout::default()
        .direction(LayoutDirection::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(5)])
        .split(bottom[1]);

    let interaction = Paragraph::new(render_interaction_panel(
        &app.state,
        &app.style_cache,
        &app.theme,
        app.animation_time,
    ))
    .block(Block::default().title("INTERACTION").borders(Borders::ALL))
    .wrap(Wrap { trim: false });

    let log_lines = render_log_panel(&app.state, &app.style_cache, app.last_outcome.as_ref());
    let log_line_count = log_lines.len() as u16;
    let log_inner_height = interaction_rows[1].height.saturating_sub(2);
    let log_scroll = log_line_count.saturating_sub(log_inner_height);

    let log = Paragraph::new(log_lines)
        .block(Block::default().title("LOG").borders(Borders::ALL))
        .scroll((log_scroll, 0))
        .wrap(Wrap { trim: false });

    frame.render_widget(map, top[0]);
    frame.render_widget(status, top[1]);
    frame.render_widget(inventory, bottom[0]);
    frame.render_widget(interaction, interaction_rows[0]);
    frame.render_widget(log, interaction_rows[1]);
}

fn render_terminal_panel(
    state: &GameState,
    style_cache: &StyleCache,
    save_slot: &Path,
) -> Vec<Line<'static>> {
    use omega_core::color::{ColorId, UiColorId};

    let text_default = style_cache.get_fg(&ColorId::Ui(UiColorId::TextDefault));
    let highlight = style_cache.get_fg(&ColorId::Ui(UiColorId::Highlight));

    let (headline, headline_color) = match state.status {
        SessionStatus::Lost => ("You died!", ColorId::Ui(UiColorId::MessageDanger)),
        SessionStatus::Won => ("You are victorious!", ColorId::Ui(UiColorId::MessageSuccess)),
        SessionStatus::InProgress => ("Session in progress.", ColorId::Ui(UiColorId::TextDefault)),
    };
    let headline_style = style_cache.get_fg(&headline_color);

    let mut lines = vec![Line::from(Span::styled(headline, headline_style))];

    if state.status == SessionStatus::Lost
        && let Some(source) = state.death_source.as_deref()
    {
        lines.push(Line::from(Span::styled(format!("Killed by {source}."), text_default)));
    }

    lines.push(Line::from(Span::styled(format!("Name: {}", state.player_name), text_default)));
    lines.push(Line::from(Span::styled(format!("Mode: {}", state.mode.as_str()), text_default)));
    lines.push(Line::from(Span::styled(
        format!(
            "Turn {}  Time {}m  HP {}/{}",
            state.clock.turn, state.clock.minutes, state.player.stats.hp, state.player.stats.max_hp
        ),
        text_default,
    )));
    lines.push(Line::from(Span::styled(
        format!("Score: {}", state.progression.score),
        text_default,
    )));
    lines.push(Line::from(Span::styled(
        format!("Save slot: {}", save_slot.display()),
        text_default,
    )));
    lines.push(Line::from(Span::styled(
        "Press c/q/esc to continue, r/n to restart, or l to load.",
        highlight,
    )));
    lines.push(Line::from(Span::styled("", text_default)));
    lines.push(Line::from(Span::styled("Recent log:", text_default)));

    for message in state.log.iter().rev().take(6).rev() {
        lines.push(Line::from(Span::styled(format!("- {message}"), text_default)));
    }

    lines
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut out = stdout();
    out.execute(EnterAlternateScreen)?;
    out.execute(crossterm::event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(out);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::event::DisableMouseCapture, LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn read_ui_key() -> Result<Option<UiKey>> {
    match event::read()? {
        CEvent::Key(key) => {
            if key.kind != KeyEventKind::Press {
                return Ok(None);
            }

            let mapped = match key.code {
                KeyCode::Esc => Some(UiKey::Esc),
                KeyCode::Enter => Some(UiKey::Enter),
                KeyCode::Backspace => Some(UiKey::Backspace),
                KeyCode::Up => Some(UiKey::Up),
                KeyCode::Down => Some(UiKey::Down),
                KeyCode::Left => Some(UiKey::Left),
                KeyCode::Right => Some(UiKey::Right),
                KeyCode::Tab => Some(UiKey::Char('\t')),
                KeyCode::F(10) => Some(UiKey::ThemeCycle),
                KeyCode::F(12) => Some(UiKey::WizardToggle),
                KeyCode::Char(ch)
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && App::map_ctrl_legacy(ch).is_some() =>
                {
                    Some(UiKey::Ctrl(ch.to_ascii_lowercase()))
                }
                KeyCode::Char(ch) => Some(UiKey::Char(ch)),
                _ => None,
            };

            Ok(mapped)
        }
        CEvent::Mouse(mouse) => Ok(Some(UiKey::Mouse(mouse))),
        _ => Ok(None),
    }
}

fn render_map_panel(
    state: &GameState,
    style_cache: &StyleCache,
    view_width: u16,
    view_height: u16,
) -> Vec<Line<'static>> {
    use omega_core::color::{
        ColorId, EffectColorId, EntityColorId, ItemRarityColorId, MonsterColorId, TerrainColorId,
        UiColorId,
    };

    let max_w = i32::from(view_width.max(1)).min(state.bounds.width.max(1));
    let max_h = i32::from(view_height.max(1)).min(state.bounds.height.max(1));
    let center = state.player.position;
    let mut min_x = center.x - (max_w / 2);
    let mut min_y = center.y - (max_h / 2);
    min_x = min_x.clamp(0, state.bounds.width.saturating_sub(max_w).max(0));
    min_y = min_y.clamp(0, state.bounds.height.saturating_sub(max_h).max(0));
    let max_x = (min_x + max_w - 1).clamp(0, state.bounds.width - 1);
    let max_y = (min_y + max_h - 1).clamp(0, state.bounds.height - 1);

    let targeting_cursor =
        state.pending_targeting_interaction.as_ref().map(|interaction| interaction.cursor);
    let projectile_impact = state.transient_projectile_impact;
    let projectile_path = &state.transient_projectile_path;
    let objective_target = if state.mode == GameMode::Modern {
        objective_map_hints(state).into_iter().next()
    } else {
        None
    };
    let objective_route: HashSet<(i32, i32)> = objective_target
        .map(|target| {
            line_path(state.player.position, target)
                .into_iter()
                .enumerate()
                .filter_map(|(idx, pos)| {
                    if idx < 2 || pos == state.player.position || pos == target || idx % 3 != 0 {
                        None
                    } else {
                        Some(pos)
                    }
                })
                .map(|pos| (pos.x, pos.y))
                .collect()
        })
        .unwrap_or_default();

    let mut lines = Vec::new();

    for y in min_y..=max_y {
        let mut spans: Vec<Span> = Vec::new();
        let mut current_text = String::new();
        let mut current_style = Style::default();

        for x in min_x..=max_x {
            let pos = Position { x, y };

            // Determine character and color
            let (ch, color_id) = if targeting_cursor == Some(pos) {
                ('X', Some(ColorId::Ui(UiColorId::Cursor)))
            } else if projectile_impact == Some(pos) {
                ('!', Some(ColorId::Effect(EffectColorId::Impact)))
            } else if projectile_path.contains(&pos) {
                (':', Some(ColorId::Effect(EffectColorId::MagicArcane)))
            } else if state.player.position == pos {
                ('@', Some(ColorId::Entity(EntityColorId::Player)))
            } else if state.monsters.iter().any(|m| m.position == pos) {
                (
                    'm',
                    Some(ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileHumanoid))),
                )
            } else if state.ground_items.iter().any(|g| g.position == pos) {
                ('*', Some(ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Common))))
            } else if objective_target == Some(pos) && state.map_glyph_at(pos) == '.' {
                ('o', Some(ColorId::Ui(UiColorId::Highlight)))
            } else if objective_route.contains(&(pos.x, pos.y)) && state.map_glyph_at(pos) == '.' {
                (':', Some(ColorId::Ui(UiColorId::TextDim)))
            } else {
                let glyph = state.map_glyph_at(pos);
                let terrain_color = match glyph {
                    '#' => Some(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallStone))),
                    '.' => {
                        Some(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorStone)))
                    }
                    '+' => Some(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Door))),
                    '<' => Some(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::StairsUp))),
                    '>' => {
                        Some(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::StairsDown)))
                    }
                    '~' => Some(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Water))),
                    '^' => Some(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Lava))),
                    '"' | ',' => {
                        Some(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorGrass)))
                    }
                    ' ' => None,
                    _ => Some(ColorId::Ui(UiColorId::TextDefault)),
                };
                (glyph, terrain_color)
            };

            // Get style for this character
            let style =
                if let Some(cid) = color_id { style_cache.get_fg(&cid) } else { Style::default() };

            // Batch consecutive characters with same style
            if style == current_style {
                current_text.push(ch);
            } else {
                if !current_text.is_empty() {
                    spans.push(Span::styled(std::mem::take(&mut current_text), current_style));
                }
                current_text.push(ch);
                current_style = style;
            }
        }

        // Flush remaining text in this row
        if !current_text.is_empty() {
            spans.push(Span::styled(current_text, current_style));
        }

        lines.push(Line::from(spans));
    }

    lines
}

fn line_path(mut from: Position, to: Position) -> Vec<Position> {
    let mut path = Vec::new();
    let dx = (to.x - from.x).abs();
    let sx = if from.x < to.x { 1 } else { -1 };
    let dy = -(to.y - from.y).abs();
    let sy = if from.y < to.y { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        path.push(from);
        if from == to {
            break;
        }
        let e2 = err * 2;
        if e2 >= dy {
            err += dy;
            from.x += sx;
        }
        if e2 <= dx {
            err += dx;
            from.y += sy;
        }
    }
    path
}

fn render_status_panel(
    state: &GameState,
    style_cache: &StyleCache,
    save_slot: &Path,
    theme: &omega_core::color::ColorTheme,
    time: f32,
) -> Vec<Line<'static>> {
    use omega_core::color::{ColorId, UiColorId};

    let interaction = if state.pending_wizard_interaction.is_some() {
        "wizard prompt active".to_string()
    } else if state.pending_spell_interaction.is_some() {
        "spell prompt active".to_string()
    } else if state.pending_activation_interaction.is_some() {
        "activation prompt active".to_string()
    } else if state.pending_quit_interaction.is_some() {
        "quit confirmation active".to_string()
    } else if state.pending_talk_direction.is_some() {
        "directional talk/tunnel prompt active".to_string()
    } else if state.pending_inventory_interaction.is_some() {
        "inventory interaction active".to_string()
    } else if state.pending_item_prompt.is_some() {
        "item selection prompt active".to_string()
    } else if state.pending_targeting_interaction.is_some() {
        "targeting prompt active".to_string()
    } else {
        state
            .pending_site_interaction
            .as_ref()
            .map(|kind| describe_pending_interaction(kind, state))
            .unwrap_or_else(|| "none".to_string())
    };

    let text_default = style_cache.get_fg(&ColorId::Ui(UiColorId::TextDefault));
    let text_dim = style_cache.get_fg(&ColorId::Ui(UiColorId::TextDim));
    let highlight = style_cache.get_fg(&ColorId::Ui(UiColorId::Highlight));

    // HP color based on percentage
    let hp_percent =
        (state.player.stats.hp as f32 / state.player.stats.max_hp.max(1) as f32) * 100.0;
    let hp_color = if hp_percent > 66.0 {
        ColorId::Ui(UiColorId::HealthHigh)
    } else if hp_percent > 33.0 {
        ColorId::Ui(UiColorId::HealthMedium)
    } else {
        ColorId::Ui(UiColorId::HealthLow)
    };

    // Use animated color for health
    let hp_spec = theme.resolve_animated(&hp_color, time);
    let hp_style = Style::default().fg(colorspec_to_ratatui(&hp_spec));

    let mana_style = style_cache.get_fg(&ColorId::Ui(UiColorId::Mana));

    // State color (Lost/Won)
    let state_color = match state.status {
        SessionStatus::Lost => ColorId::Ui(UiColorId::MessageDanger),
        SessionStatus::Won => ColorId::Ui(UiColorId::MessageSuccess),
        SessionStatus::InProgress => ColorId::Ui(UiColorId::TextDefault),
    };
    let state_style = style_cache.get_fg(&state_color);

    // Interaction color
    let interaction_style = if interaction == "none" { text_dim } else { highlight };

    let mut lines = vec![
        Line::from(Span::styled(format!("Name: {}", state.player_name), text_default)),
        Line::from(Span::styled(format!("Mode: {}", state.mode.as_str()), text_default)),
        Line::from(Span::styled(format!("Turn: {}", state.clock.turn), text_default)),
        Line::from(Span::styled(format!("Time: {}m", state.clock.minutes), text_default)),
        Line::from(Span::styled(
            format!("Pos: ({}, {})", state.player.position.x, state.player.position.y),
            text_default,
        )),
        Line::from(vec![
            Span::styled("HP: ", text_default),
            Span::styled(
                format!("{}/{}", state.player.stats.hp, state.player.stats.max_hp),
                hp_style,
            ),
        ]),
        Line::from(vec![
            Span::styled("Mana: ", text_default),
            Span::styled(
                format!("{}/{}", state.spellbook.mana, state.spellbook.max_mana),
                mana_style,
            ),
        ]),
        Line::from(Span::styled(
            format!(
                "Inventory: {}/{}",
                state.player.inventory.len(),
                state.player.inventory_capacity
            ),
            text_default,
        )),
        Line::from(Span::styled(
            format!("Gold/Bank/Food: {}/{}/{}", state.gold, state.bank_gold, state.food),
            text_default,
        )),
        Line::from(Span::styled(format!("World: {:?}", state.world_mode), text_default)),
        Line::from(Span::styled(
            format!("Quest: {:?}", state.progression.quest_state),
            text_default,
        )),
        Line::from(vec![
            Span::styled("State: ", text_default),
            Span::styled(format!("{:?}", state.status), state_style),
        ]),
        Line::from(vec![
            Span::styled("Interaction: ", text_default),
            Span::styled(interaction, interaction_style),
        ]),
        Line::from(Span::styled(format!("Slot: {}", save_slot.display()), text_default)),
        Line::from(Span::styled(
            format!("Theme: {} (F10 to switch)", theme.meta.name),
            text_default,
        )),
        Line::from(Span::styled(
            "Keys: S save+quit, L load, R restart, Q retire/quit flow, a activate, z zap, Ctrl+F/G/I/K/L/O/P/R/W/X, F12 wizard",
            text_default,
        )),
        Line::from(Span::styled(
            "Combat: move into adjacent monsters to bump-attack; uppercase WASD attacks directly. Move west with h or Left.",
            text_default,
        )),
    ];

    if let Some(last) = renderable_timeline_lines(state, 1).first() {
        lines.push(Line::from(Span::styled(format!("Latest: {last}"), text_default)));
    }

    if state.mode == GameMode::Modern {
        let objective_summary = active_objective_snapshot(state)
            .map(|snapshot| snapshot.summary)
            .unwrap_or_else(|| "No active objective.".to_string());
        lines.push(Line::from(Span::styled(
            format!("Objective: {objective_summary}"),
            text_default,
        )));
        if let Some(target) = objective_map_hints(state).into_iter().next() {
            let dx = target.x - state.player.position.x;
            let dy = target.y - state.player.position.y;
            lines.push(Line::from(Span::styled(
                format!("Objective marker: ({}, {})  Δx={} Δy={}", target.x, target.y, dx, dy),
                text_default,
            )));
        }
    }

    lines
}

fn describe_pending_interaction(kind: &SiteInteractionKind, state: &GameState) -> String {
    match kind {
        SiteInteractionKind::Shop => "shop menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::Armorer => "armorer menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::Club => "club menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Gym => "gym menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Healer => "healer menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Casino => "casino menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Commandant => "commandant menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Diner => "diner menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Craps => "craps menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Tavern => "tavern menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::PawnShop => "pawn shop menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Brothel => "brothel menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Condo => "condo menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Bank => "bank menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::MercGuild => "merc guild menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::ThievesGuild => "thieves guild menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::Temple => "temple menu (1-5, q/x close)".to_string(),
        SiteInteractionKind::College => "college menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::Sorcerors => "sorcerors menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::Castle => "castle menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::Palace => "palace menu (1-3, q/x close)".to_string(),
        SiteInteractionKind::Order => "order menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::Charity => "charity menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::Monastery => "monastery menu (1-4, q/x close)".to_string(),
        SiteInteractionKind::Arena => {
            if state.progression.arena_rank > 0 {
                "Rampart Coliseum prompt (1/y fight, 2/n leave)".to_string()
            } else {
                "Rampart Coliseum prompt (1/e enter, 2/r register, 3/x leave)".to_string()
            }
        }
        SiteInteractionKind::Altar { deity_id } => {
            let deity = match deity_id {
                1 => "Odin",
                2 => "Set",
                3 => "Athena",
                4 => "Hecate",
                5 => "Destiny",
                _ => "Unknown",
            };
            format!("{deity} altar menu (1-4, q/x close)")
        }
    }
}

fn render_inventory_panel(state: &GameState, style_cache: &StyleCache) -> Vec<Line<'static>> {
    use omega_core::color::{ColorId, UiColorId};

    let text_default = style_cache.get_fg(&ColorId::Ui(UiColorId::TextDefault));
    let text_dim = style_cache.get_fg(&ColorId::Ui(UiColorId::TextDim));
    let success = style_cache.get_fg(&ColorId::Ui(UiColorId::MessageSuccess));

    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        format!(
            "Pack: {}/{}  Burden: {}",
            state.player.inventory.len(),
            state.player.inventory_capacity,
            state.carry_burden
        ),
        text_default,
    )));

    if state.player.inventory.is_empty() {
        lines.push(Line::from(Span::styled("(empty)", text_dim)));
    } else {
        for (idx, item) in state.player.inventory.iter().enumerate() {
            lines.push(Line::from(Span::styled(
                format!(
                    "{}: {} [{} | {}]",
                    idx + 1,
                    item.name,
                    format!("{:?}", item.family).to_ascii_lowercase(),
                    if item.usef.is_empty() { "no-usef" } else { item.usef.as_str() }
                ),
                text_default,
            )));
        }
    }

    // Equipment line with color coding
    let equip_line = vec![
        Span::styled("Equip Wpn:", text_default),
        Span::styled(
            if state.player.equipment.weapon_hand.is_some() { "set" } else { "-" },
            if state.player.equipment.weapon_hand.is_some() { success } else { text_dim },
        ),
        Span::styled(" Shd:", text_default),
        Span::styled(
            if state.player.equipment.shield.is_some() { "set" } else { "-" },
            if state.player.equipment.shield.is_some() { success } else { text_dim },
        ),
        Span::styled(" Arm:", text_default),
        Span::styled(
            if state.player.equipment.armor.is_some() { "set" } else { "-" },
            if state.player.equipment.armor.is_some() { success } else { text_dim },
        ),
        Span::styled(" Clk:", text_default),
        Span::styled(
            if state.player.equipment.cloak.is_some() { "set" } else { "-" },
            if state.player.equipment.cloak.is_some() { success } else { text_dim },
        ),
        Span::styled(" Bts:", text_default),
        Span::styled(
            if state.player.equipment.boots.is_some() { "set" } else { "-" },
            if state.player.equipment.boots.is_some() { success } else { text_dim },
        ),
        Span::styled(" R1:", text_default),
        Span::styled(
            if state.player.equipment.ring_1.is_some() { "set" } else { "-" },
            if state.player.equipment.ring_1.is_some() { success } else { text_dim },
        ),
        Span::styled(" R2:", text_default),
        Span::styled(
            if state.player.equipment.ring_2.is_some() { "set" } else { "-" },
            if state.player.equipment.ring_2.is_some() { success } else { text_dim },
        ),
    ];
    lines.push(Line::from(equip_line));

    let mut on_ground: Vec<&str> = state
        .ground_items
        .iter()
        .filter(|entry| entry.position == state.player.position)
        .map(|entry| entry.item.name.as_str())
        .collect();
    on_ground.sort_unstable();
    if !on_ground.is_empty() {
        lines.push(Line::from(Span::styled("Ground here:", text_default)));
        for name in on_ground {
            lines.push(Line::from(Span::styled(format!("- {name}"), text_default)));
        }
    }
    lines
}

fn render_interaction_panel(
    state: &GameState,
    style_cache: &StyleCache,
    theme: &omega_core::color::ColorTheme,
    time: f32,
) -> Vec<Line<'static>> {
    use omega_core::color::{ColorId, UiColorId};

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

    let text_dim = style_cache.get_fg(&ColorId::Ui(UiColorId::TextDim));
    let text_bold = style_cache.get_fg(&ColorId::Ui(UiColorId::TextBold));

    // Animated highlight for active prompts
    let highlight_spec = theme.resolve_animated(&ColorId::Ui(UiColorId::Highlight), time);
    let highlight_style = Style::default().fg(colorspec_to_ratatui(&highlight_spec));

    let mut lines = Vec::new();
    if let Some(prompt) = active_prompt {
        lines.push(Line::from(Span::styled(prompt, highlight_style)));
    } else {
        lines.push(Line::from(Span::styled("No active interaction.", text_dim)));
    }
    if let Some(hint) = active_hint {
        lines.push(Line::from(Span::styled(hint, text_dim)));
    }
    if modal_input_profile(state) == ModalInputProfile::TextEntry {
        if state.pending_wizard_interaction.is_some() && !state.wizard_input_buffer.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("Input: {}", state.wizard_input_buffer),
                text_bold,
            )));
        } else if state.pending_spell_interaction.is_some() && !state.spell_input_buffer.is_empty()
        {
            lines.push(Line::from(Span::styled(
                format!("Input: {}", state.spell_input_buffer),
                text_bold,
            )));
        } else if state.pending_targeting_interaction.is_some()
            && !state.target_input_buffer.is_empty()
        {
            lines.push(Line::from(Span::styled(
                format!("Input: {}", state.target_input_buffer),
                text_bold,
            )));
        } else if !state.interaction_buffer.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("Input: {}", state.interaction_buffer),
                text_bold,
            )));
        }
    }
    lines
}

fn render_log_panel(
    state: &GameState,
    style_cache: &StyleCache,
    last_outcome: Option<&Outcome>,
) -> Vec<Line<'static>> {
    use omega_core::color::{ColorId, UiColorId};

    let mut text_lines = renderable_timeline_lines(state, 12);
    if text_lines.is_empty()
        && let Some(outcome) = last_outcome
    {
        for event in outcome.events.iter().rev().take(8).rev() {
            text_lines.push(format_event(event));
        }
        if outcome.status != SessionStatus::InProgress {
            text_lines.push(format!("session status: {:?}", outcome.status));
        }
    }

    if text_lines.is_empty() {
        let text_dim = style_cache.get_fg(&ColorId::Ui(UiColorId::TextDim));
        return vec![Line::from(Span::styled("(no messages)", text_dim))];
    }

    // Color messages by content heuristic
    text_lines
        .into_iter()
        .map(|msg| {
            let color = if msg.contains("died")
                || msg.contains("defeated")
                || msg.contains("killed")
                || msg.contains("damage")
                || msg.contains("hit you")
            {
                ColorId::Ui(UiColorId::MessageDanger)
            } else if msg.contains("warning") || msg.contains("caution") || msg.contains("careful")
            {
                ColorId::Ui(UiColorId::MessageWarning)
            } else if msg.contains("victory")
                || msg.contains("gained")
                || msg.contains("found")
                || msg.contains("success")
                || msg.contains("healed")
                || msg.contains("picked")
            {
                ColorId::Ui(UiColorId::MessageSuccess)
            } else {
                ColorId::Ui(UiColorId::MessageInfo)
            };
            let style = style_cache.get_fg(&color);
            Line::from(Span::styled(msg, style))
        })
        .collect()
}

fn format_event(event: &Event) -> String {
    match event {
        Event::Waited => "waited".to_string(),
        Event::Moved { from, to } => {
            format!("moved: ({}, {}) -> ({}, {})", from.x, from.y, to.x, to.y)
        }
        Event::MoveBlocked { target } => format!("blocked: ({}, {})", target.x, target.y),
        Event::AttackMissed { target } => format!("missed: ({}, {})", target.x, target.y),
        Event::Attacked { monster_id, damage, remaining_hp } => {
            format!("hit monster#{monster_id} for {damage} (hp {remaining_hp})")
        }
        Event::MonsterMoved { monster_id, from, to } => {
            format!("monster#{monster_id} moved ({}, {}) -> ({}, {})", from.x, from.y, to.x, to.y)
        }
        Event::MonsterAttacked { monster_id, damage, remaining_hp } => {
            format!("monster#{monster_id} hit you for {damage} (hp {remaining_hp})")
        }
        Event::MonsterDefeated { monster_id } => format!("monster#{monster_id} defeated"),
        Event::PlayerDefeated => "you are defeated".to_string(),
        Event::VictoryAchieved => "victory achieved".to_string(),
        Event::CommandIgnoredTerminal { status } => format!("command ignored ({status:?})"),
        Event::PickedUp { item_id, name } => format!("picked {name}#{item_id}"),
        Event::Dropped { item_id, name } => format!("dropped {name}#{item_id}"),
        Event::InventoryFull { capacity } => format!("inventory full ({capacity})"),
        Event::NoItemToPickUp => "no item to pick up".to_string(),
        Event::InvalidDropSlot { slot } => format!("invalid drop slot: {slot}"),
        Event::LegacyHandled { token, note, fully_modeled: _ } => {
            format!("legacy `{token}`: {note}")
        }
        Event::ConfirmationRequired { token } => {
            format!("confirmation required for `{token}`")
        }
        Event::EconomyUpdated { source, gold, bank_gold } => {
            format!("economy `{source}` gold={gold} bank={bank_gold}")
        }
        Event::DialogueAdvanced { speaker, quest_state } => {
            format!("dialogue `{speaker}` -> quest {quest_state:?}")
        }
        Event::QuestAdvanced { state, steps_completed } => {
            format!("quest -> {state:?} (steps {steps_completed})")
        }
        Event::ProgressionUpdated { guild_rank, priest_rank, alignment } => {
            format!("progression g{guild_rank}/p{priest_rank} {alignment:?}")
        }
        Event::EndingResolved { ending, score, high_score_eligible } => {
            format!("ending {ending:?} score={score} eligible={high_score_eligible}")
        }
        Event::ActionPointsSpent { cost, budget_per_turn, total_spent } => {
            format!("ap +{cost}/{budget_per_turn} total={total_spent}")
        }
        Event::StatusTick { effect_id, magnitude, remaining_turns } => {
            format!("status `{effect_id}` tick {magnitude} (remaining {remaining_turns})")
        }
        Event::StatusExpired { effect_id } => format!("status `{effect_id}` expired"),
        Event::TurnAdvanced { turn, minutes } => format!("turn advanced: {turn} ({minutes}m)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_content::LEGACY_RAMPART_START;
    use omega_core::Stats;

    #[test]
    fn key_mapping_dispatches_expected_commands() {
        assert_eq!(
            App::map_input(UiKey::Char('w')),
            UiAction::Dispatch(Command::Move(Direction::North))
        );
        assert_eq!(
            App::map_input(UiKey::Char('h')),
            UiAction::Dispatch(Command::Move(Direction::West))
        );
        assert_eq!(
            App::map_input(UiKey::Char('a')),
            UiAction::Dispatch(Command::Legacy { token: "a".to_string() })
        );
        assert_eq!(
            App::map_input(UiKey::Char('D')),
            UiAction::Dispatch(Command::Attack(Direction::East))
        );
        assert_eq!(App::map_input(UiKey::Char('g')), UiAction::Dispatch(Command::Pickup));
        assert_eq!(App::map_input(UiKey::Char('2')), UiAction::Dispatch(Command::Drop { slot: 1 }));
        assert_eq!(
            App::map_input(UiKey::Char('?')),
            UiAction::Dispatch(Command::Legacy { token: "?".to_string() })
        );
        assert_eq!(
            App::map_input(UiKey::Char('q')),
            UiAction::Dispatch(Command::Legacy { token: "q".to_string() })
        );
        assert_eq!(
            App::map_input(UiKey::WizardToggle),
            UiAction::Dispatch(Command::Legacy { token: "^g".to_string() })
        );
        assert_eq!(
            App::map_input(UiKey::Ctrl('x')),
            UiAction::Dispatch(Command::Legacy { token: "^x".to_string() })
        );
        assert_eq!(
            App::map_input(UiKey::Enter),
            UiAction::Dispatch(Command::Legacy { token: "<enter>".to_string() })
        );
        assert_eq!(
            App::map_input(UiKey::Backspace),
            UiAction::Dispatch(Command::Legacy { token: "<backspace>".to_string() })
        );
        assert_eq!(App::map_input(UiKey::Esc), UiAction::Quit);
        assert_eq!(
            App::map_input(UiKey::Char('Q')),
            UiAction::Dispatch(Command::Legacy { token: "Q".to_string() })
        );
        assert_eq!(App::map_input(UiKey::Char('S')), UiAction::SaveAndQuit);
        assert_eq!(
            App::map_input(UiKey::Char('P')),
            UiAction::Dispatch(Command::Legacy { token: "P".to_string() })
        );
        assert_eq!(
            App::map_input(UiKey::Char('!')),
            UiAction::Dispatch(Command::Legacy { token: "!".to_string() })
        );
        assert_eq!(App::map_input(UiKey::Char('L')), UiAction::LoadSlot);
        assert_eq!(App::map_input(UiKey::Char('R')), UiAction::Restart);
    }

    #[test]
    fn wizard_pending_esc_routes_to_core_cancel_instead_of_quit() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.pending_wizard_interaction =
            Some(omega_core::WizardInteraction::WishTextEntry { blessing: 1 });
        let slot = PathBuf::from("target/test-omega-tui-wizard-cancel.json");
        let mut app = App::with_options(101, state.clone(), state, slot);

        app.handle_key(UiKey::Esc);

        assert!(!app.quit);
        assert!(app.state.pending_wizard_interaction.is_none());
        assert!(app.state.log.iter().any(|line| line.contains("Wish canceled")));
    }

    #[test]
    fn site_prompt_locks_directional_input_from_wasd_mapping() {
        let mut state = GameState::new(omega_core::MapBounds { width: 7, height: 7 });
        state.player.position = Position { x: 3, y: 3 };
        state.pending_site_interaction = Some(SiteInteractionKind::Temple);
        let slot = PathBuf::from("target/test-omega-tui-site-lock.json");
        let mut app = App::with_options(102, state.clone(), state, slot);

        let before = app.state.player.position;
        app.handle_key(UiKey::Char('w'));

        assert_eq!(app.state.player.position, before);
        assert!(app.state.pending_site_interaction.is_some());
    }

    #[test]
    fn site_prompt_blocks_arrow_movement_unless_direction_modal() {
        let mut state = GameState::new(omega_core::MapBounds { width: 7, height: 7 });
        state.player.position = Position { x: 3, y: 3 };
        state.pending_site_interaction = Some(SiteInteractionKind::Temple);
        let slot = PathBuf::from("target/test-omega-tui-site-arrow-lock.json");
        let mut app = App::with_options(1202, state.clone(), state, slot);

        let before = app.state.player.position;
        app.handle_key(UiKey::Up);

        assert_eq!(app.state.player.position, before);
        assert!(app.state.pending_site_interaction.is_some());
    }

    #[test]
    fn item_prompt_locks_directional_input_from_wasd_mapping() {
        let mut state = GameState::new(omega_core::MapBounds { width: 7, height: 7 });
        state.player.position = Position { x: 3, y: 3 };
        state.pending_item_prompt = Some(omega_core::ItemPromptInteraction {
            context: omega_core::ItemPromptContext::Drop,
            filter: omega_core::ItemPromptFilter::Any,
            prompt: "Drop which item?".to_string(),
        });
        let slot = PathBuf::from("target/test-omega-tui-item-lock.json");
        let mut app = App::with_options(103, state.clone(), state, slot);

        let before = app.state.player.position;
        app.handle_key(UiKey::Char('w'));

        assert_eq!(app.state.player.position, before);
        assert!(app.state.pending_item_prompt.is_some());
    }

    #[test]
    fn inventory_modal_show_pack_key_logs_pack_listing() {
        let mut state = GameState::new(omega_core::MapBounds { width: 7, height: 7 });
        state.player.position = Position { x: 3, y: 3 };
        state.player.inventory.push(omega_core::Item::new(9, "practice blade"));
        let slot = PathBuf::from("target/test-omega-tui-inventory-show-pack.json");
        let mut app = App::with_options(1203, state.clone(), state, slot);

        app.handle_key(UiKey::Char('i'));
        assert!(app.state.pending_inventory_interaction.is_some());
        let before = app.state.player.position;

        app.handle_key(UiKey::Char('s'));

        assert_eq!(app.state.player.position, before);
        assert!(app.state.log.iter().any(|line| line.starts_with("Pack:")));
    }

    #[test]
    fn uppercase_directional_keys_move_when_no_adjacent_monster() {
        let state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        let slot = PathBuf::from("target/test-omega-tui-controls.json");
        let mut app = App::with_options(17, state.clone(), state, slot);
        let start = app.state.player.position;
        app.handle_key(UiKey::Char('D'));
        assert_eq!(app.state.player.position, Position { x: start.x + 1, y: start.y });
    }

    #[test]
    fn uppercase_directional_keys_attack_when_adjacent_monster_exists() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        let target = Position { x: state.player.position.x + 1, y: state.player.position.y };
        state.spawn_monster(
            "rat",
            target,
            Stats { hp: 6, max_hp: 6, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
        );
        let slot = PathBuf::from("target/test-omega-tui-controls-attack.json");
        let mut app = App::with_options(19, state.clone(), state, slot);

        app.handle_key(UiKey::Char('D'));

        let events = app.last_outcome.as_ref().map(|o| o.events.as_slice()).unwrap_or(&[]);
        assert!(events.iter().any(|event| {
            matches!(event, Event::MonsterAttacked { .. } | Event::MonsterDefeated { .. })
        }));
    }

    #[test]
    fn lowercase_directional_keys_bump_attack_when_adjacent_monster_exists() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        let start = state.player.position;
        let target = Position { x: start.x + 1, y: start.y };
        state.spawn_monster(
            "rat",
            target,
            Stats { hp: 6, max_hp: 6, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
        );
        let slot = PathBuf::from("target/test-omega-tui-controls-bump-attack.json");
        let mut app = App::with_options(1901, state.clone(), state, slot);

        app.handle_key(UiKey::Char('d'));

        assert_eq!(app.state.player.position, start);
        let events = app.last_outcome.as_ref().map(|o| o.events.as_slice()).unwrap_or(&[]);
        assert!(events.iter().any(|event| {
            matches!(event, Event::MonsterAttacked { .. } | Event::MonsterDefeated { .. })
        }));
        assert!(events.iter().all(|event| !matches!(event, Event::MoveBlocked { .. })));
    }

    #[test]
    fn scripted_session_dispatches_to_core_and_advances_time() {
        let app = run_scripted_session([UiKey::Char(' ')]);
        assert!(!app.quit);
        assert_eq!(app.state.clock.turn, 1);
        assert_eq!(app.state.clock.minutes, 6);
    }

    #[test]
    fn uppercase_q_routes_to_core_quit_confirmation_flow() {
        let mut app = run_scripted_session([UiKey::Char('Q')]);
        assert!(app.state.pending_quit_interaction.is_some());
        app.handle_key(UiKey::Char('n'));
        assert!(!app.quit);
        assert!(app.state.pending_quit_interaction.is_none());
        assert!(app.state.status == SessionStatus::InProgress);
    }

    #[test]
    fn ratatui_render_contains_main_panels() {
        let app = App::default();
        let screen = render_to_string_with_ratatui(&app, 100, 36);
        assert!(screen.contains("MAP"));
        assert!(screen.contains("STATUS"));
        assert!(screen.contains("INVENTORY"));
        assert!(screen.contains("LOG"));
    }

    #[test]
    fn save_load_and_restart_are_available() {
        let bootstrap = GameState::default();
        let save_path = PathBuf::from("target/test-omega-slot.json");
        let mut app =
            App::with_options(77, bootstrap.clone(), bootstrap.clone(), save_path.clone());

        app.apply_action(UiAction::Dispatch(Command::Wait));
        let saved_turn = app.state.clock.turn;
        app.apply_action(UiAction::SaveSlot);

        app.apply_action(UiAction::Dispatch(Command::Wait));
        assert!(app.state.clock.turn > saved_turn);

        app.apply_action(UiAction::LoadSlot);
        assert_eq!(app.state.clock.turn, saved_turn);

        app.apply_action(UiAction::Restart);
        assert_eq!(app.state.clock.turn, bootstrap.clock.turn);

        let _ = fs::remove_file(save_path);
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
    fn inventory_panel_is_deterministic_and_human_readable() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.player.inventory = vec![
            omega_core::Item::new(1, "healing potion"),
            omega_core::Item::new(2, "identify scroll"),
        ];
        state.ground_items = vec![
            omega_core::GroundItem {
                position: state.player.position,
                item: omega_core::Item::new(3, "zinc ring"),
            },
            omega_core::GroundItem {
                position: state.player.position,
                item: omega_core::Item::new(4, "amber ring"),
            },
        ];

        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        let first_lines = render_inventory_panel(&state, &cache);
        let second_lines = render_inventory_panel(&state, &cache);
        let first = lines_to_string(first_lines.clone());
        let second = lines_to_string(second_lines);
        assert_eq!(first, second);
        assert!(first.contains("Pack: 2/"));
        assert!(first.contains("1: healing potion"));
        assert!(first.contains("2: identify scroll"));
        assert!(
            first.find("amber ring").unwrap_or(usize::MAX) < first.find("zinc ring").unwrap_or(0),
            "ground items should be displayed in a stable lexical order"
        );
    }

    #[test]
    fn log_panel_keeps_chronological_order() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.log = vec!["first".to_string(), "second".to_string(), "third".to_string()];

        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        let rendered_lines = render_log_panel(&state, &cache, None);
        let rendered = lines_to_string(rendered_lines);
        assert!(
            rendered.find("first").unwrap_or(usize::MAX) < rendered.find("second").unwrap_or(0)
        );
        assert!(
            rendered.find("second").unwrap_or(usize::MAX) < rendered.find("third").unwrap_or(0)
        );
    }

    #[test]
    fn interaction_panel_pins_active_prompt_and_log_filters_prompt_spam() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.pending_site_interaction = Some(SiteInteractionKind::Temple);
        state.log = vec![
            "Temple: [1/t] tithe (15g) [2/p] pray [3/b] blessing (35g) [4/s] sanctuary [5/x] leave | favor=0 gold=200".to_string(),
            "Site prompt active: choose a bracketed option, or press q/x to close.".to_string(),
            "Selected option 1. You make a tithe at the temple.".to_string(),
        ];

        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        let interaction_lines = render_interaction_panel(&state, &cache, &theme, 0.0);
        let interaction = lines_to_string(interaction_lines);
        let rendered_log_lines = render_log_panel(&state, &cache, None);
        let rendered_log = lines_to_string(rendered_log_lines);

        assert!(interaction.contains("Temple: [1/t] tithe (15g)"));
        assert!(
            interaction
                .contains("Temple prompt active: choose 1-5 or letter aliases shown in brackets.")
        );
        assert!(!rendered_log.contains("Site prompt active: choose a bracketed option"));
    }

    #[test]
    fn status_panel_surfaces_pending_interaction_hint() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.pending_site_interaction = Some(SiteInteractionKind::MercGuild);
        let slot = PathBuf::from("target/test-omega-tui-status-hint.json");

        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        let rendered_lines = render_status_panel(&state, &cache, &slot, &theme, 0.0);
        let rendered = lines_to_string(rendered_lines);

        assert!(rendered.contains("Interaction: merc guild menu"));
    }

    #[test]
    fn status_panel_includes_mana_totals() {
        let state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        let slot = PathBuf::from("target/test-omega-tui-status-mana.json");

        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        let rendered_lines = render_status_panel(&state, &cache, &slot, &theme, 0.0);
        let rendered = lines_to_string(rendered_lines);
        assert!(rendered.contains("Mana: "));
    }

    #[test]
    fn status_panel_includes_objective_summary_only_for_modern_mode() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.progression.quest_state = omega_core::LegacyQuestState::Active;
        state.progression.main_quest.objective = "Report to the Mercenary Guild.".to_string();
        let slot = PathBuf::from("target/test-omega-tui-status-objective.json");

        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        state.mode = GameMode::Classic;
        let classic_lines = render_status_panel(&state, &cache, &slot, &theme, 0.0);
        let classic = lines_to_string(classic_lines);
        assert!(!classic.contains("Objective:"));

        state.mode = GameMode::Modern;
        let modern_lines = render_status_panel(&state, &cache, &slot, &theme, 0.0);
        let modern = lines_to_string(modern_lines);
        assert!(modern.contains("Objective:"));
        assert!(modern.contains("Mercenary Guild"));
    }

    #[test]
    fn interaction_panel_surfaces_activation_prompt() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.pending_activation_interaction = Some(omega_core::ActivationInteraction::ChooseKind);

        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        let rendered_lines = render_interaction_panel(&state, &cache, &theme, 0.0);
        let rendered = lines_to_string(rendered_lines);
        assert!(rendered.contains("Activate -- item [i] or artifact [a]"));
    }

    // Helper to extract plain text from styled lines for testing
    fn lines_to_string(lines: Vec<Line>) -> String {
        lines
            .iter()
            .map(|line| line.spans.iter().map(|span| span.content.as_ref()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn map_panel_scales_to_available_space() {
        let state = GameState::new(omega_core::MapBounds { width: 80, height: 80 });
        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        let rendered_lines = render_map_panel(&state, &cache, 40, 18);
        let rendered = lines_to_string(rendered_lines.clone());
        let lines: Vec<&str> = rendered.lines().collect();

        assert_eq!(rendered_lines.len(), 18);
        assert!(lines.iter().all(|line| line.chars().count() == 40));
    }

    #[test]
    fn map_panel_shows_target_cursor_and_projectile_trace() {
        let mut state = GameState::new(omega_core::MapBounds { width: 20, height: 20 });
        let origin = state.player.position;
        state.pending_targeting_interaction = Some(omega_core::TargetingInteraction {
            origin,
            cursor: Position { x: origin.x + 3, y: origin.y },
            mode: omega_core::ProjectileKind::MagicMissile,
        });
        state.transient_projectile_path = vec![
            origin,
            Position { x: origin.x + 1, y: origin.y },
            Position { x: origin.x + 2, y: origin.y },
        ];
        state.transient_projectile_impact = Some(Position { x: origin.x + 2, y: origin.y });

        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        let rendered_lines = render_map_panel(&state, &cache, 20, 10);
        let rendered = lines_to_string(rendered_lines);
        assert!(rendered.contains("X"));
        assert!(rendered.contains(":"));
        assert!(rendered.contains("!"));
    }

    #[test]
    fn modern_map_panel_surfaces_objective_marker_and_route() {
        let mut state = GameState::new(omega_core::MapBounds { width: 20, height: 20 });
        state.mode = GameMode::Modern;
        state.progression.quest_state = omega_core::LegacyQuestState::Active;
        state.progression.main_quest.objective = "Report to the mercenary guild.".to_string();
        state.site_grid = vec![omega_core::TileSiteCell::default(); 400];
        state.site_grid[5 * 20 + 14].aux = omega_core::SITE_AUX_SERVICE_MERC_GUILD;

        let theme = omega_core::color::ColorTheme::from_toml(include_str!(
            "../../omega-content/themes/classic.toml"
        ))
        .unwrap();
        let capability = omega_core::color::ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        let rendered_lines = render_map_panel(&state, &cache, 20, 12);
        let rendered = lines_to_string(rendered_lines);
        assert!(rendered.contains('o'));
        assert!(rendered.contains(':'));

        state.mode = GameMode::Classic;
        let classic_lines = render_map_panel(&state, &cache, 20, 12);
        let classic = lines_to_string(classic_lines);
        assert!(!classic.contains('o'));
    }

    #[test]
    fn terminal_continue_key_exits_session() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.status = SessionStatus::Lost;
        let slot = PathBuf::from("target/test-omega-tui-terminal-continue.json");
        let mut app = App::with_options(91, state.clone(), state, slot);

        app.handle_key(UiKey::Char('c'));

        assert!(app.quit);
        assert!(
            app.state
                .log
                .iter()
                .any(|line| line.contains("Game over acknowledged; returning to launcher"))
        );
    }

    #[test]
    fn terminal_restart_key_restarts_from_bootstrap() {
        let mut bootstrap = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        bootstrap.player.position = Position { x: 2, y: 2 };
        let mut initial = bootstrap.clone();
        initial.status = SessionStatus::Lost;
        initial.player.position = Position { x: 4, y: 4 };
        initial.clock.turn = 99;
        let slot = PathBuf::from("target/test-omega-tui-terminal-restart.json");
        let mut app = App::with_options(93, initial, bootstrap.clone(), slot);

        app.handle_key(UiKey::Char('r'));

        assert_eq!(app.state.status, SessionStatus::InProgress);
        assert_eq!(app.state.player.position, bootstrap.player.position);
        assert_eq!(app.state.clock.turn, bootstrap.clock.turn);
    }

    #[test]
    fn terminal_render_shows_dedicated_death_screen() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.status = SessionStatus::Lost;
        state.death_source = Some("fang".to_string());
        state.log.push("You are defeated.".to_string());
        let slot = PathBuf::from("target/test-omega-tui-terminal-render.json");
        let app = App::with_options(95, state.clone(), state, slot);

        let screen = render_to_string_with_ratatui(&app, 100, 36);
        assert!(screen.contains("DEATH"));
        assert!(screen.contains("You died!"));
        assert!(screen.contains("Killed by fang."));
        assert!(screen.contains("Press c/q/esc to continue"));
    }

    #[test]
    fn terminal_render_shows_death_screen_outside_arena() {
        let mut state = GameState::new(omega_core::MapBounds { width: 5, height: 5 });
        state.status = SessionStatus::Lost;
        state.world_mode = omega_core::WorldMode::Countryside;
        state.environment = omega_core::LegacyEnvironment::Countryside;
        state.log.push("You are defeated in the wilderness.".to_string());
        let slot = PathBuf::from("target/test-omega-tui-terminal-render-country.json");
        let app = App::with_options(97, state.clone(), state, slot);

        let screen = render_to_string_with_ratatui(&app, 100, 36);
        assert!(screen.contains("DEATH"));
        assert!(screen.contains("You died!"));
        assert!(screen.contains("Press c/q/esc to continue"));
    }
}
