use crate::presentation::arena_controls::{ArenaActionFeedback, ArenaToolingState};
use crate::presentation::cursor_grid::{CursorGridError, map_panel_cursor_to_grid};
use crate::presentation::theme::UiLayoutTokens;
use crate::presentation::{MapPanelCard, UiReadabilityConfig};
use crate::{AppState, FrontendRuntime, RuntimeFrame, RuntimeStatus};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_egui::{EguiContexts, egui};
use omega_core::TILE_FLAG_BURNING;
use omega_core::simulation::catastrophe::Catastrophe;
use omega_core::simulation::grid::CaGrid;
use omega_core::simulation::state::{Gas, Liquid};
use omega_core::simulation::wind::{WindGrid, WindVector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerCategory {
    Monster,
    Weapon,
    Armor,
    Scroll,
    Potion,
    Utility,
    Element,
    Hazard,
    Disaster,
}

impl SpawnerCategory {
    pub const fn all() -> &'static [Self] {
        &[
            Self::Monster,
            Self::Weapon,
            Self::Armor,
            Self::Scroll,
            Self::Potion,
            Self::Utility,
            Self::Element,
            Self::Hazard,
            Self::Disaster,
        ]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Monster => "Monsters",
            Self::Weapon => "Weapons",
            Self::Armor => "Armor",
            Self::Scroll => "Scrolls",
            Self::Potion => "Potions",
            Self::Utility => "Utility",
            Self::Element => "Elemental",
            Self::Hazard => "Hazards",
            Self::Disaster => "Disasters",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerMonsterSpec {
    Rat,
    Goblin,
    Orc,
    Wolf,
    Bandit,
}

impl SpawnerMonsterSpec {
    pub const fn all() -> &'static [Self] {
        &[Self::Rat, Self::Goblin, Self::Orc, Self::Wolf, Self::Bandit]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Rat => "rat",
            Self::Goblin => "goblin",
            Self::Orc => "orc",
            Self::Wolf => "wolf",
            Self::Bandit => "bandit",
        }
    }

    pub const fn stats(self) -> omega_core::Stats {
        match self {
            Self::Rat => omega_core::Stats {
                hp: 6,
                max_hp: 6,
                attack_min: 1,
                attack_max: 2,
                defense: 0,
                weight: 20,
            },
            Self::Goblin => omega_core::Stats {
                hp: 12,
                max_hp: 12,
                attack_min: 2,
                attack_max: 4,
                defense: 1,
                weight: 50,
            },
            Self::Orc => omega_core::Stats {
                hp: 20,
                max_hp: 20,
                attack_min: 3,
                attack_max: 6,
                defense: 2,
                weight: 80,
            },
            Self::Wolf => omega_core::Stats {
                hp: 10,
                max_hp: 10,
                attack_min: 3,
                attack_max: 5,
                defense: 1,
                weight: 60,
            },
            Self::Bandit => omega_core::Stats {
                hp: 16,
                max_hp: 16,
                attack_min: 3,
                attack_max: 5,
                defense: 1,
                weight: 70,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerWeaponSpec {
    ShortSword,
    Victrix,
}

impl SpawnerWeaponSpec {
    pub const fn all() -> &'static [Self] {
        &[Self::ShortSword, Self::Victrix]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ShortSword => "short sword",
            Self::Victrix => "Victrix",
        }
    }

    pub const fn spawn_name(self) -> &'static str {
        match self {
            Self::ShortSword => "short sword",
            Self::Victrix => "Victrix",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerArmorSpec {
    Buckler,
    HeaterShield,
    TowerShield,
    FullPlateMail,
}

impl SpawnerArmorSpec {
    pub const fn all() -> &'static [Self] {
        &[Self::Buckler, Self::HeaterShield, Self::TowerShield, Self::FullPlateMail]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Buckler => "buckler",
            Self::HeaterShield => "heater shield",
            Self::TowerShield => "tower shield",
            Self::FullPlateMail => "full plate mail",
        }
    }

    pub const fn spawn_name(self) -> &'static str {
        match self {
            Self::Buckler => "buckler",
            Self::HeaterShield => "heater shield",
            Self::TowerShield => "tower shield",
            Self::FullPlateMail => "full plate mail",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerScrollSpec {
    IdentifyScroll,
    Scroll,
}

impl SpawnerScrollSpec {
    pub const fn all() -> &'static [Self] {
        &[Self::IdentifyScroll, Self::Scroll]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::IdentifyScroll => "identify scroll",
            Self::Scroll => "scroll",
        }
    }

    pub const fn spawn_name(self) -> &'static str {
        match self {
            Self::IdentifyScroll => "identify scroll",
            Self::Scroll => "scroll",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerPotionSpec {
    HealingPotion,
    Potion,
}

impl SpawnerPotionSpec {
    pub const fn all() -> &'static [Self] {
        &[Self::HealingPotion, Self::Potion]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::HealingPotion => "healing potion",
            Self::Potion => "potion",
        }
    }

    pub const fn spawn_name(self) -> &'static str {
        match self {
            Self::HealingPotion => "healing potion",
            Self::Potion => "potion",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerItemSpec {
    ShortSword,
    Buckler,
    HealingPotion,
    IdentifyScroll,
    Water,
    Fire,
}

impl SpawnerItemSpec {
    pub const fn all() -> &'static [Self] {
        &[
            Self::ShortSword,
            Self::Buckler,
            Self::HealingPotion,
            Self::IdentifyScroll,
            Self::Water,
            Self::Fire,
        ]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ShortSword => "short sword",
            Self::Buckler => "buckler",
            Self::HealingPotion => "healing potion",
            Self::IdentifyScroll => "identify scroll",
            Self::Water => "water",
            Self::Fire => "fire",
        }
    }

    pub const fn spawn_name(self) -> Option<&'static str> {
        match self {
            Self::ShortSword => Some("short sword"),
            Self::Buckler => Some("buckler"),
            Self::HealingPotion => Some("healing potion"),
            Self::IdentifyScroll => Some("identify scroll"),
            Self::Water => None,
            Self::Fire => None,
        }
    }

    pub const fn expected_family(self) -> Option<omega_core::ItemFamily> {
        match self {
            Self::ShortSword => Some(omega_core::ItemFamily::Weapon),
            Self::Buckler => Some(omega_core::ItemFamily::Shield),
            Self::HealingPotion => Some(omega_core::ItemFamily::Potion),
            Self::IdentifyScroll => Some(omega_core::ItemFamily::Scroll),
            Self::Water => None,
            Self::Fire => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerUtilitySpec {
    FoodRation,
}

impl SpawnerUtilitySpec {
    pub const fn all() -> &'static [Self] {
        &[Self::FoodRation]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::FoodRation => "food ration",
        }
    }

    pub const fn spawn_name(self) -> &'static str {
        match self {
            Self::FoodRation => "food ration",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerElementSpec {
    Fire,
    Water,
}

impl SpawnerElementSpec {
    pub const fn all() -> &'static [Self] {
        &[Self::Fire, Self::Water]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Fire => "fire",
            Self::Water => "water",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerHazardSpec {
    PoisonTrap,
    FireTrap,
}

impl SpawnerHazardSpec {
    pub const fn all() -> &'static [Self] {
        &[Self::PoisonTrap, Self::FireTrap]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::PoisonTrap => "poison trap",
            Self::FireTrap => "fire trap",
        }
    }

    pub const fn effect_id(self) -> &'static str {
        match self {
            Self::PoisonTrap => "poison",
            Self::FireTrap => "fire",
        }
    }

    pub const fn damage(self) -> i32 {
        match self {
            Self::PoisonTrap => 6,
            Self::FireTrap => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerDisasterSpec {
    GreatFlood,
    ForestFire,
    WindBurst,
}

impl SpawnerDisasterSpec {
    pub const fn all() -> &'static [Self] {
        &[Self::GreatFlood, Self::ForestFire, Self::WindBurst]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::GreatFlood => "great flood",
            Self::ForestFire => "forest fire",
            Self::WindBurst => "wind burst",
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct SpawnerState {
    pub visible: bool,
    pub selected_category: SpawnerCategory,
    pub selected_monster: SpawnerMonsterSpec,
    pub selected_weapon: SpawnerWeaponSpec,
    pub selected_armor: SpawnerArmorSpec,
    pub selected_scroll: SpawnerScrollSpec,
    pub selected_potion: SpawnerPotionSpec,
    pub selected_utility: SpawnerUtilitySpec,
    pub selected_element: SpawnerElementSpec,
    pub selected_hazard: SpawnerHazardSpec,
    pub selected_disaster: SpawnerDisasterSpec,
}

impl Default for SpawnerState {
    fn default() -> Self {
        Self {
            visible: true,
            selected_category: SpawnerCategory::Monster,
            selected_monster: SpawnerMonsterSpec::Rat,
            selected_weapon: SpawnerWeaponSpec::ShortSword,
            selected_armor: SpawnerArmorSpec::Buckler,
            selected_scroll: SpawnerScrollSpec::IdentifyScroll,
            selected_potion: SpawnerPotionSpec::HealingPotion,
            selected_utility: SpawnerUtilitySpec::FoodRation,
            selected_element: SpawnerElementSpec::Fire,
            selected_hazard: SpawnerHazardSpec::PoisonTrap,
            selected_disaster: SpawnerDisasterSpec::GreatFlood,
        }
    }
}

pub fn spawner_ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<SpawnerState>,
    mut runtime: ResMut<FrontendRuntime>,
    status: Res<RuntimeStatus>,
    keys: Res<ButtonInput<KeyCode>>,
    mut tooling: ResMut<ArenaToolingState>,
    mut feedback: ResMut<ArenaActionFeedback>,
) {
    if status.app_state == AppState::WizardArena && keys.just_pressed(KeyCode::F9) {
        tooling.enabled = !tooling.enabled;
        feedback.push_line(if tooling.enabled {
            "Test Ground controls enabled.".to_string()
        } else {
            "Test Ground controls disabled.".to_string()
        });
    }
    if keys.just_pressed(KeyCode::F7) {
        state.visible = !state.visible;
    }
    if status.app_state != AppState::WizardArena || !state.visible || !tooling.enabled {
        return;
    }

    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    egui::SidePanel::right("spawner_panel")
        .default_width(340.0)
        .min_width(300.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 8.0;
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.heading("Wizard Test Spawner");
                ui.label("Pick a category, choose a payload, then click the map.");
                ui.label("F7: hide panel | F9: toggle test controls");
                ui.separator();

                egui::ComboBox::from_label("Category")
                    .selected_text(state.selected_category.label())
                    .show_ui(ui, |ui| {
                        for category in SpawnerCategory::all() {
                            ui.selectable_value(
                                &mut state.selected_category,
                                *category,
                                category.label(),
                            );
                        }
                    });

                match state.selected_category {
                    SpawnerCategory::Monster => {
                        egui::ComboBox::from_label("Monster")
                            .selected_text(state.selected_monster.label())
                            .show_ui(ui, |ui| {
                                for spec in SpawnerMonsterSpec::all() {
                                    ui.selectable_value(
                                        &mut state.selected_monster,
                                        *spec,
                                        spec.label(),
                                    );
                                }
                            });
                    }
                    SpawnerCategory::Weapon => {
                        egui::ComboBox::from_label("Weapon")
                            .selected_text(state.selected_weapon.label())
                            .show_ui(ui, |ui| {
                                for spec in SpawnerWeaponSpec::all() {
                                    ui.selectable_value(
                                        &mut state.selected_weapon,
                                        *spec,
                                        spec.label(),
                                    );
                                }
                            });
                    }
                    SpawnerCategory::Armor => {
                        egui::ComboBox::from_label("Armor")
                            .selected_text(state.selected_armor.label())
                            .show_ui(ui, |ui| {
                                for spec in SpawnerArmorSpec::all() {
                                    ui.selectable_value(
                                        &mut state.selected_armor,
                                        *spec,
                                        spec.label(),
                                    );
                                }
                            });
                    }
                    SpawnerCategory::Scroll => {
                        egui::ComboBox::from_label("Scroll")
                            .selected_text(state.selected_scroll.label())
                            .show_ui(ui, |ui| {
                                for spec in SpawnerScrollSpec::all() {
                                    ui.selectable_value(
                                        &mut state.selected_scroll,
                                        *spec,
                                        spec.label(),
                                    );
                                }
                            });
                    }
                    SpawnerCategory::Potion => {
                        egui::ComboBox::from_label("Potion")
                            .selected_text(state.selected_potion.label())
                            .show_ui(ui, |ui| {
                                for spec in SpawnerPotionSpec::all() {
                                    ui.selectable_value(
                                        &mut state.selected_potion,
                                        *spec,
                                        spec.label(),
                                    );
                                }
                            });
                    }
                    SpawnerCategory::Utility => {
                        egui::ComboBox::from_label("Utility")
                            .selected_text(state.selected_utility.label())
                            .show_ui(ui, |ui| {
                                for spec in SpawnerUtilitySpec::all() {
                                    ui.selectable_value(
                                        &mut state.selected_utility,
                                        *spec,
                                        spec.label(),
                                    );
                                }
                            });
                    }
                    SpawnerCategory::Element => {
                        egui::ComboBox::from_label("Element")
                            .selected_text(state.selected_element.label())
                            .show_ui(ui, |ui| {
                                for spec in SpawnerElementSpec::all() {
                                    ui.selectable_value(
                                        &mut state.selected_element,
                                        *spec,
                                        spec.label(),
                                    );
                                }
                            });
                    }
                    SpawnerCategory::Hazard => {
                        egui::ComboBox::from_label("Hazard")
                            .selected_text(state.selected_hazard.label())
                            .show_ui(ui, |ui| {
                                for spec in SpawnerHazardSpec::all() {
                                    ui.selectable_value(
                                        &mut state.selected_hazard,
                                        *spec,
                                        spec.label(),
                                    );
                                }
                            });
                    }
                    SpawnerCategory::Disaster => {
                        egui::ComboBox::from_label("Disaster")
                            .selected_text(state.selected_disaster.label())
                            .show_ui(ui, |ui| {
                                for spec in SpawnerDisasterSpec::all() {
                                    ui.selectable_value(
                                        &mut state.selected_disaster,
                                        *spec,
                                        spec.label(),
                                    );
                                }
                            });
                    }
                }

                ui.label(format!("Active: {}", current_selection_label(&state)));
                ui.separator();

                if let Some(session) = runtime.0.session.as_mut() {
                    let before = session.state.ai_paused;
                    ui.checkbox(&mut session.state.ai_paused, "Pause Monsters");
                    if before != session.state.ai_paused {
                        feedback.push_line(if session.state.ai_paused {
                            "Monster turns paused.".to_string()
                        } else {
                            "Monster turns resumed.".to_string()
                        });
                    }
                } else {
                    ui.label("Pause Monsters unavailable (no session)");
                }

                ui.horizontal(|ui| {
                    if ui.button("Clear Monsters").clicked() {
                        if let Some(session) = runtime.0.session.as_mut() {
                            let removed = session.state.monsters.len();
                            session.state.monsters.clear();
                            feedback.push_line(format!("Cleared {removed} monster(s)."));
                        } else {
                            feedback
                                .push_line("Clear Monsters failed: no active session.".to_string());
                        }
                    }

                    if ui.button("Clear Items").clicked() {
                        if let Some(session) = runtime.0.session.as_mut() {
                            let removed = session.state.ground_items.len();
                            session.state.ground_items.clear();
                            feedback.push_line(format!("Cleared {removed} ground item(s)."));
                        } else {
                            feedback
                                .push_line("Clear Items failed: no active session.".to_string());
                        }
                    }
                });

                ui.separator();
                ui.heading("Action Feedback");
                for line in feedback.recent_lines(6) {
                    ui.label(line);
                }
            });
        });
}

fn current_selection_label(state: &SpawnerState) -> String {
    match state.selected_category {
        SpawnerCategory::Monster => format!("Monster: {}", state.selected_monster.label()),
        SpawnerCategory::Weapon => format!("Weapon: {}", state.selected_weapon.label()),
        SpawnerCategory::Armor => format!("Armor: {}", state.selected_armor.label()),
        SpawnerCategory::Scroll => format!("Scroll: {}", state.selected_scroll.label()),
        SpawnerCategory::Potion => format!("Potion: {}", state.selected_potion.label()),
        SpawnerCategory::Utility => format!("Utility: {}", state.selected_utility.label()),
        SpawnerCategory::Element => format!("Element: {}", state.selected_element.label()),
        SpawnerCategory::Hazard => format!("Hazard: {}", state.selected_hazard.label()),
        SpawnerCategory::Disaster => format!("Disaster: {}", state.selected_disaster.label()),
    }
}

pub fn mouse_spawning_system(params: MouseSpawningParams) {
    let MouseSpawningParams {
        mut contexts,
        buttons,
        q_map_panel,
        tooling,
        state,
        mut grid,
        mut wind_grid,
        mut runtime,
        status,
        layout,
        readability,
        frame,
        mut feedback,
    } = params;

    if status.app_state != AppState::WizardArena || !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    if let Some(ctx) = contexts.try_ctx_mut()
        && ctx.wants_pointer_input()
    {
        return;
    }
    if !tooling.enabled {
        feedback.push_line("Spawn ignored: Test Ground controls are disabled (F9).".to_string());
        return;
    }

    let Ok((rel_cursor, node)) = q_map_panel.get_single() else {
        return;
    };
    let pos = match map_panel_cursor_to_grid(rel_cursor, node, &layout, &readability, &frame) {
        Ok(pos) => pos,
        Err(err @ (CursorGridError::NoFrame | CursorGridError::InvalidGeometry)) => {
            feedback.push_line(format!("Spawn canceled: {}", cursor_error_message(err)));
            return;
        }
        Err(CursorGridError::CursorOutsideNode | CursorGridError::OutOfBounds) => {
            return;
        }
    };

    let Some(session) = runtime.0.session.as_mut() else {
        feedback.push_line("Spawn failed: no active session.".to_string());
        return;
    };

    match state.selected_category {
        SpawnerCategory::Monster => {
            let spec = state.selected_monster;
            session.state.spawn_monster(spec.label(), pos, spec.stats());
            feedback.push_line(format!("Spawned {} at ({}, {}).", spec.label(), pos.x, pos.y));
        }
        SpawnerCategory::Weapon => {
            let spec = state.selected_weapon;
            spawn_named_item(
                &mut session.state,
                spec.spawn_name(),
                spec.label(),
                pos,
                &mut feedback,
            );
        }
        SpawnerCategory::Armor => {
            let spec = state.selected_armor;
            spawn_named_item(
                &mut session.state,
                spec.spawn_name(),
                spec.label(),
                pos,
                &mut feedback,
            );
        }
        SpawnerCategory::Scroll => {
            let spec = state.selected_scroll;
            spawn_named_item(
                &mut session.state,
                spec.spawn_name(),
                spec.label(),
                pos,
                &mut feedback,
            );
        }
        SpawnerCategory::Potion => {
            let spec = state.selected_potion;
            spawn_named_item(
                &mut session.state,
                spec.spawn_name(),
                spec.label(),
                pos,
                &mut feedback,
            );
        }
        SpawnerCategory::Utility => {
            let spec = state.selected_utility;
            spawn_named_item(
                &mut session.state,
                spec.spawn_name(),
                spec.label(),
                pos,
                &mut feedback,
            );
        }
        SpawnerCategory::Element => {
            apply_element_spawn(
                state.selected_element,
                &mut session.state,
                &mut grid,
                pos,
                &mut feedback,
            );
        }
        SpawnerCategory::Hazard => {
            let spec = state.selected_hazard;
            let trap_id = session.state.place_trap(pos, spec.damage(), spec.effect_id());
            feedback.push_line(format!(
                "Spawned {} (id {}) at ({}, {}) damage {}.",
                spec.label(),
                trap_id,
                pos.x,
                pos.y,
                spec.damage()
            ));
        }
        SpawnerCategory::Disaster => {
            apply_disaster_spawn(
                state.selected_disaster,
                &mut grid,
                &mut wind_grid,
                pos,
                &mut feedback,
            );
        }
    }
}

fn spawn_named_item(
    state: &mut omega_core::GameState,
    spawn_name: &str,
    label: &str,
    pos: omega_core::Position,
    feedback: &mut ArenaActionFeedback,
) {
    let item_id = state.place_item(spawn_name, pos);
    if let Some(spawned) = state.ground_items.iter().find(|entry| entry.item.id == item_id) {
        feedback.push_line(format!(
            "Spawned {} -> {} [{:?}, usef={}] at ({}, {}).",
            label, spawned.item.name, spawned.item.family, spawned.item.usef, pos.x, pos.y
        ));
    } else {
        feedback.push_line(format!(
            "Spawned {} at ({}, {}) (item id {}).",
            label, pos.x, pos.y, item_id
        ));
    }
}

fn apply_element_spawn(
    spec: SpawnerElementSpec,
    state: &mut omega_core::GameState,
    grid: &mut CaGrid,
    pos: omega_core::Position,
    feedback: &mut ArenaActionFeedback,
) {
    match spec {
        SpawnerElementSpec::Fire => {
            if let Some(cell) = state.tile_site_at_mut(pos) {
                cell.flags |= TILE_FLAG_BURNING;
                if grid.in_bounds(pos.x as isize, pos.y as isize) {
                    let mut ca_cell = *grid.get(pos.x as usize, pos.y as usize);
                    ca_cell.gas = Some(Gas::Fire);
                    ca_cell.liquid = None;
                    ca_cell.wet = ca_cell.wet.saturating_sub(40);
                    ca_cell.heat = ca_cell.heat.max(220);
                    ca_cell.pressure = ca_cell.pressure.max(55);
                    grid.set_immediate(pos.x as usize, pos.y as usize, ca_cell);
                }
                feedback.push_line(format!("Ignited tile at ({}, {}).", pos.x, pos.y));
            } else {
                feedback.push_line(format!(
                    "Fire spawn failed: tile ({}, {}) is out of bounds.",
                    pos.x, pos.y
                ));
            }
        }
        SpawnerElementSpec::Water => {
            if let Some(cell) = state.tile_site_at_mut(pos) {
                let was_site_burning = (cell.flags & TILE_FLAG_BURNING) != 0;
                cell.flags &= !TILE_FLAG_BURNING;
                if grid.in_bounds(pos.x as isize, pos.y as isize) {
                    let mut ca_cell = *grid.get(pos.x as usize, pos.y as usize);
                    let was_hot_fire = was_site_burning
                        || ca_cell.gas == Some(Gas::Fire)
                        || ca_cell.heat >= 150
                        || ca_cell.pressure >= 35;
                    ca_cell.liquid = Some(Liquid::Water);
                    ca_cell.wet = ca_cell.wet.max(180);
                    if was_hot_fire {
                        ca_cell.gas = Some(Gas::Steam);
                        ca_cell.pressure = ca_cell.pressure.max(95);
                        ca_cell.heat = ca_cell.heat.max(210).saturating_sub(20);
                    } else {
                        ca_cell.gas = None;
                        ca_cell.heat = ca_cell.heat.saturating_sub(40);
                    }
                    grid.set_immediate(pos.x as usize, pos.y as usize, ca_cell);
                }
                feedback.push_line(format!("Flooded tile at ({}, {}).", pos.x, pos.y));
            } else {
                feedback.push_line(format!(
                    "Water spawn failed: tile ({}, {}) is out of bounds.",
                    pos.x, pos.y
                ));
            }
        }
    }
}

fn apply_disaster_spawn(
    spec: SpawnerDisasterSpec,
    grid: &mut CaGrid,
    wind_grid: &mut WindGrid,
    pos: omega_core::Position,
    feedback: &mut ArenaActionFeedback,
) {
    if !grid.in_bounds(pos.x as isize, pos.y as isize) {
        feedback.push_line(format!(
            "Disaster canceled: target tile ({}, {}) is out of bounds.",
            pos.x, pos.y
        ));
        return;
    }

    let center = (pos.x as usize, pos.y as usize);
    match spec {
        SpawnerDisasterSpec::GreatFlood => {
            Catastrophe::great_flood(grid, center);
            feedback
                .push_line(format!("Great Flood triggered at click point ({}, {}).", pos.x, pos.y));
        }
        SpawnerDisasterSpec::ForestFire => {
            Catastrophe::forest_fire_jump(grid, center);
            feedback
                .push_line(format!("Forest Fire triggered at click point ({}, {}).", pos.x, pos.y));
        }
        SpawnerDisasterSpec::WindBurst => {
            apply_local_wind_burst(wind_grid, center, 12);
            feedback
                .push_line(format!("Wind Burst triggered at click point ({}, {}).", pos.x, pos.y));
        }
    }
}

fn apply_local_wind_burst(wind_grid: &mut WindGrid, center: (usize, usize), radius: usize) {
    let (cx, cy) = center;
    let max_x = wind_grid.width().saturating_sub(1);
    let max_y = wind_grid.height().saturating_sub(1);
    let radius_sq = (radius * radius) as isize;

    wind_grid.set_global(WindVector::default());
    for y in cy.saturating_sub(radius)..=(cy + radius).min(max_y) {
        for x in cx.saturating_sub(radius)..=(cx + radius).min(max_x) {
            let dx = x as isize - cx as isize;
            let dy = y as isize - cy as isize;
            let distance_sq = dx * dx + dy * dy;
            if distance_sq > radius_sq {
                continue;
            }

            let mut dir_x = dx.signum() as i8;
            let dir_y = dy.signum() as i8;
            if dir_x == 0 && dir_y == 0 {
                dir_x = 1;
            }

            let falloff = ((distance_sq as f32).sqrt() * 16.0) as i32;
            let strength = (220 - falloff).clamp(90, 220) as u8;
            wind_grid.set(x, y, WindVector { dx: dir_x, dy: dir_y, strength });
        }
    }
}

#[derive(SystemParam)]
pub struct MouseSpawningParams<'w, 's> {
    contexts: EguiContexts<'w, 's>,
    buttons: Res<'w, ButtonInput<MouseButton>>,
    q_map_panel:
        Query<'w, 's, (&'static RelativeCursorPosition, &'static ComputedNode), With<MapPanelCard>>,
    tooling: Res<'w, ArenaToolingState>,
    state: Res<'w, SpawnerState>,
    grid: ResMut<'w, CaGrid>,
    wind_grid: ResMut<'w, WindGrid>,
    runtime: ResMut<'w, FrontendRuntime>,
    status: Res<'w, RuntimeStatus>,
    layout: Res<'w, UiLayoutTokens>,
    readability: Res<'w, UiReadabilityConfig>,
    frame: Res<'w, RuntimeFrame>,
    feedback: ResMut<'w, ArenaActionFeedback>,
}

fn cursor_error_message(err: CursorGridError) -> &'static str {
    match err {
        CursorGridError::NoFrame => "runtime frame unavailable",
        CursorGridError::CursorOutsideNode => "cursor is outside map panel",
        CursorGridError::OutOfBounds => "click is outside rendered map viewport",
        CursorGridError::InvalidGeometry => "map panel geometry is invalid",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::GameState;
    use omega_core::ItemFamily;

    #[test]
    fn monster_specs_have_distinct_hp_profiles() {
        let stats: Vec<_> = SpawnerMonsterSpec::all().iter().map(|spec| spec.stats().hp).collect();
        assert_eq!(stats, vec![6, 12, 20, 10, 16]);
    }

    #[test]
    fn baseline_catalog_item_families_resolve() {
        let mut state = GameState::new(omega_core::MapBounds { width: 8, height: 8 });
        let pos = omega_core::Position { x: 2, y: 2 };

        let short_sword_id = state.place_item(SpawnerWeaponSpec::ShortSword.spawn_name(), pos);
        let buckler_id = state.place_item(SpawnerArmorSpec::Buckler.spawn_name(), pos);
        let potion_id = state.place_item(SpawnerPotionSpec::HealingPotion.spawn_name(), pos);
        let scroll_id = state.place_item(SpawnerScrollSpec::IdentifyScroll.spawn_name(), pos);

        let short_sword = state
            .ground_items
            .iter()
            .find(|entry| entry.item.id == short_sword_id)
            .expect("short sword should be present");
        assert_eq!(short_sword.item.family, ItemFamily::Weapon);

        let buckler = state
            .ground_items
            .iter()
            .find(|entry| entry.item.id == buckler_id)
            .expect("buckler should be present");
        assert_eq!(buckler.item.family, ItemFamily::Shield);

        let potion = state
            .ground_items
            .iter()
            .find(|entry| entry.item.id == potion_id)
            .expect("healing potion should be present");
        assert_eq!(potion.item.family, ItemFamily::Potion);

        let scroll = state
            .ground_items
            .iter()
            .find(|entry| entry.item.id == scroll_id)
            .expect("identify scroll should be present");
        assert_eq!(scroll.item.family, ItemFamily::Scroll);
    }

    #[test]
    fn all_spawnable_item_specs_place_items() {
        let mut state = GameState::new(omega_core::MapBounds { width: 8, height: 8 });
        let pos = omega_core::Position { x: 3, y: 3 };

        for spec in SpawnerWeaponSpec::all() {
            let item_id = state.place_item(spec.spawn_name(), pos);
            assert!(state.ground_items.iter().any(|entry| entry.item.id == item_id));
        }
        for spec in SpawnerArmorSpec::all() {
            let item_id = state.place_item(spec.spawn_name(), pos);
            assert!(state.ground_items.iter().any(|entry| entry.item.id == item_id));
        }
        for spec in SpawnerScrollSpec::all() {
            let item_id = state.place_item(spec.spawn_name(), pos);
            assert!(state.ground_items.iter().any(|entry| entry.item.id == item_id));
        }
        for spec in SpawnerPotionSpec::all() {
            let item_id = state.place_item(spec.spawn_name(), pos);
            assert!(state.ground_items.iter().any(|entry| entry.item.id == item_id));
        }
        for spec in SpawnerUtilitySpec::all() {
            let item_id = state.place_item(spec.spawn_name(), pos);
            assert!(state.ground_items.iter().any(|entry| entry.item.id == item_id));
        }
    }

    #[test]
    fn hazard_specs_spawn_armed_traps() {
        let mut state = GameState::new(omega_core::MapBounds { width: 8, height: 8 });
        let pos = omega_core::Position { x: 3, y: 3 };
        for spec in SpawnerHazardSpec::all() {
            let trap_id = state.place_trap(pos, spec.damage(), spec.effect_id());
            let trap = state
                .traps
                .iter()
                .find(|entry| entry.id == trap_id)
                .expect("placed trap should be present");
            assert!(trap.armed);
            assert_eq!(trap.effect_id, spec.effect_id());
            assert_eq!(trap.damage, spec.damage());
        }
    }

    #[test]
    fn local_wind_burst_is_centered_on_click_point() {
        let mut wind = WindGrid::new(16, 16);
        apply_local_wind_burst(&mut wind, (8, 8), 3);

        let center = wind.get(8, 8);
        let corner = wind.get(0, 0);
        let near = wind.get(9, 8);
        assert!(center.strength > 0);
        assert!(near.strength > 0);
        assert_eq!(corner.strength, 0);
    }

    #[test]
    fn water_spawn_over_fire_creates_steam_with_pressure() {
        let mut state = GameState::new(omega_core::MapBounds { width: 8, height: 8 });
        state.site_grid = vec![omega_core::TileSiteCell::default(); 8 * 8];
        let mut grid = CaGrid::new(8, 8);
        let pos = omega_core::Position { x: 3, y: 3 };
        let mut feedback = ArenaActionFeedback::default();

        if let Some(site) = state.tile_site_at_mut(pos) {
            site.flags |= TILE_FLAG_BURNING;
        }
        grid.set_immediate(
            pos.x as usize,
            pos.y as usize,
            omega_core::simulation::cell::Cell {
                gas: Some(Gas::Fire),
                heat: 220,
                pressure: 30,
                ..omega_core::simulation::cell::Cell::default()
            },
        );

        apply_element_spawn(SpawnerElementSpec::Water, &mut state, &mut grid, pos, &mut feedback);
        let cell = *grid.get(pos.x as usize, pos.y as usize);
        assert_eq!(cell.gas, Some(Gas::Steam));
        assert_eq!(cell.liquid, Some(Liquid::Water));
        assert!(cell.heat >= 180);
        assert!(cell.pressure >= 95);
    }
}
