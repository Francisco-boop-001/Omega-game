use crate::presentation::MapPanelCard;
use crate::presentation::UiReadabilityConfig;
use crate::presentation::theme::UiLayoutTokens;
use crate::{AppState, FrontendRuntime, Position, RuntimeFrame, RuntimeStatus};
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_egui::{EguiContexts, egui};
use omega_core::Stats;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnerCategory {
    Monster,
    Item,
    Hazard,
}

#[derive(Debug, Clone, Resource)]
pub struct SpawnerState {
    pub visible: bool,
    pub selected_category: SpawnerCategory,
    pub selected_id: String,
    pub monster_catalog: Vec<String>,
    pub item_catalog: Vec<String>,
}

impl Default for SpawnerState {
    fn default() -> Self {
        Self {
            visible: true,
            selected_category: SpawnerCategory::Monster,
            selected_id: "rat".to_string(),
            monster_catalog: vec![
                "rat".to_string(),
                "goblin".to_string(),
                "orc".to_string(),
                "wolf".to_string(),
                "bandit".to_string(),
            ],
            item_catalog: vec![
                "practice blade".to_string(),
                "wooden shield".to_string(),
                "healing potion".to_string(),
                "identify scroll".to_string(),
                "fire".to_string(),
            ],
        }
    }
}

pub fn spawner_ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<SpawnerState>,
    mut runtime: ResMut<FrontendRuntime>,
    status: Res<RuntimeStatus>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F7) {
        state.visible = !state.visible;
    }

    if status.app_state != AppState::WizardArena {
        return;
    }

    if !state.visible {
        return;
    }

    egui::SidePanel::right("spawner_panel").default_width(200.0).show(contexts.ctx_mut(), |ui| {
        ui.heading("Wizard Spawner");
        ui.separator();

        ui.horizontal(|ui| {
            ui.selectable_value(&mut state.selected_category, SpawnerCategory::Monster, "Monsters");
            ui.selectable_value(&mut state.selected_category, SpawnerCategory::Item, "Items");
            ui.selectable_value(&mut state.selected_category, SpawnerCategory::Hazard, "Hazards");
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| match state.selected_category {
            SpawnerCategory::Monster => {
                let catalog = state.monster_catalog.clone();
                for id in catalog {
                    ui.selectable_value(&mut state.selected_id, id.clone(), &id);
                }
            }
            SpawnerCategory::Item => {
                let catalog = state.item_catalog.clone();
                for id in catalog {
                    ui.selectable_value(&mut state.selected_id, id.clone(), &id);
                }
            }
            SpawnerCategory::Hazard => {
                ui.label("Fire Trap (WIP)");
            }
        });

        ui.separator();
        ui.label("Click on map to spawn");

        ui.separator();
        ui.heading("Arena Control");
        if ui.button("Clear Monsters").clicked()
            && let Some(session) = runtime.0.session.as_mut()
        {
            session.state.monsters.clear();
        }
        if ui.button("Clear Items").clicked()
            && let Some(session) = runtime.0.session.as_mut()
        {
            session.state.ground_items.clear();
        }
        if let Some(session) = runtime.0.session.as_mut() {
            ui.checkbox(&mut session.state.ai_paused, "Pause AI");
        }
    });
}

pub fn mouse_spawning_system(
    buttons: Res<ButtonInput<MouseButton>>,
    q_map_panel: Query<(&RelativeCursorPosition, &ComputedNode), With<MapPanelCard>>,
    state: Res<SpawnerState>,
    mut runtime: ResMut<FrontendRuntime>,
    status: Res<RuntimeStatus>,
    layout: Res<UiLayoutTokens>,
    readability: Res<UiReadabilityConfig>,
    frame: Res<RuntimeFrame>,
) {
    if status.app_state != AppState::WizardArena {
        return;
    }

    if buttons.just_pressed(MouseButton::Left)
        && let Ok((rel_cursor, node)) = q_map_panel.get_single()
    {
        // Using rel_cursor.normalized.is_some() as proxy for cursor_over
        if rel_cursor.normalized.is_some()
            && let Some(pos) =
                translate_cursor_to_grid(rel_cursor, node, &layout, &readability, &frame)
        {
            let session = runtime.0.session.as_mut().expect("No active session");
            match state.selected_category {
                SpawnerCategory::Monster => {
                    session.state.spawn_monster(
                        &state.selected_id,
                        pos,
                        Stats {
                            hp: 10,
                            max_hp: 10,
                            attack_min: 1,
                            attack_max: 3,
                            defense: 0,
                            weight: 50,
                        },
                    );
                }
                SpawnerCategory::Item => {
                    if state.selected_id == "fire" {
                        if let Some(cell) = session.state.tile_site_at_mut(pos) {
                            cell.flags |= omega_core::TILE_FLAG_BURNING;
                        }
                    } else {
                        session.state.place_item(&state.selected_id, pos);
                    }
                }
                SpawnerCategory::Hazard => {
                    // Logic for hazards/traps if needed
                }
            }
        }
    }
}

fn translate_cursor_to_grid(
    rel_cursor: &RelativeCursorPosition,
    node: &ComputedNode,
    layout: &UiLayoutTokens,
    readability: &UiReadabilityConfig,
    frame: &RuntimeFrame,
) -> Option<Position> {
    let normalized = rel_cursor.normalized?;
    // normalized is (-0.5, -0.5) to (0.5, 0.5)
    let cursor_pixels = (normalized + 0.5) * node.size();

    // Scale font size
    let font_size = layout.map_font_size * readability.scale;

    // Heuristic: Monospace font width is approx 0.6 of height
    let char_w = font_size * 0.6;
    let char_h = font_size;

    // Viewport logic (adapted from tilemap.rs)
    let Some(rendered_frame) = &frame.frame else {
        return None;
    };
    let map_w = rendered_frame.bounds.0 as usize;
    let map_h = rendered_frame.bounds.1 as usize;

    // Current focus (usually player)
    let focus = rendered_frame
        .tiles
        .iter()
        .find(|t| t.kind == crate::TileKind::Player)
        .map(|t| t.position)
        .unwrap_or(Position { x: 25, y: 25 });

    let viewport_w = 58;
    let viewport_h = 30;

    let view_w = map_w.min(viewport_w).max(1);
    let view_h = map_h.min(viewport_h).max(1);

    let max_start_x = map_w.saturating_sub(view_w) as i32;
    let max_start_y = map_h.saturating_sub(view_h) as i32;

    let start_x = (focus.x - (view_w as i32 / 2)).clamp(0, max_start_x);
    let start_y = (focus.y - (view_h as i32 / 2)).clamp(0, max_start_y);

    let padding = layout.spacing_md * readability.scale;

    let grid_x = start_x + ((cursor_pixels.x - padding) / char_w) as i32;
    let grid_y = start_y + ((cursor_pixels.y - padding - font_size) / char_h) as i32;

    let pos = Position { x: grid_x, y: grid_y };
    if grid_x >= 0 && grid_x < map_w as i32 && grid_y >= 0 && grid_y < map_h as i32 {
        Some(pos)
    } else {
        None
    }
}
