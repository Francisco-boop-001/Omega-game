use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_egui::{egui, EguiContexts};
use crate::{AppState, FrontendRuntime, Position, RuntimeFrame, RuntimeStatus};
use crate::presentation::MapPanelCard;
use crate::presentation::theme::UiLayoutTokens;
use crate::presentation::UiReadabilityConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorTarget {
    Monster(u64),
    Item(u32),
}

#[derive(Debug, Clone, Resource, Default)]
pub struct InspectorState {
    pub target: Option<InspectorTarget>,
}

pub fn mouse_inspector_system(
    buttons: Res<ButtonInput<MouseButton>>,
    q_map_panel: Query<(&RelativeCursorPosition, &ComputedNode), With<MapPanelCard>>,
    mut inspector: ResMut<InspectorState>,
    runtime: Res<FrontendRuntime>,
    status: Res<RuntimeStatus>,
    layout: Res<UiLayoutTokens>,
    readability: Res<UiReadabilityConfig>,
    frame: Res<RuntimeFrame>,
) {
    if status.app_state != AppState::WizardArena {
        return;
    }

    if buttons.just_pressed(MouseButton::Right)
        && let Ok((rel_cursor, node)) = q_map_panel.get_single()
            && rel_cursor.normalized.is_some()
                && let Some(pos) = translate_cursor_to_grid(rel_cursor, node, &layout, &readability, &frame)
                    && let Some(session) = &runtime.0.session {
                        inspector.target = find_entity_at(&session.state, pos);
                    }
}

pub fn inspector_ui_system(
    mut contexts: EguiContexts,
    mut inspector: ResMut<InspectorState>,
    mut runtime: ResMut<FrontendRuntime>,
    status: Res<RuntimeStatus>,
) {
    if status.app_state != AppState::WizardArena {
        return;
    }

    let Some(target) = inspector.target else {
        return;
    };

    let session = match runtime.0.session.as_mut() {
        Some(s) => s,
        None => return,
    };

    let mut open = true;
    egui::Window::new("Inspector")
        .open(&mut open)
        .show(contexts.ctx_mut(), |ui| {
            match target {
                InspectorTarget::Monster(id) => {
                    if let Some(index) = session.state.monsters.iter().position(|m| m.id == id) {
                        let monster = &session.state.monsters[index];
                        ui.heading(format!("Monster: {}", monster.name));
                        ui.label(format!("ID: {}", monster.id));
                        ui.label(format!("Position: ({}, {})", monster.position.x, monster.position.y));
                        ui.label(format!("HP: {}/{}", monster.stats.hp, monster.stats.max_hp));
                        ui.label(format!("Behavior: {:?}", monster.behavior));
                        ui.label(format!("Faction: {:?}", monster.faction));
                        
                        ui.separator();
                        if ui.button("Despawn").clicked() {
                            session.state.monsters.remove(index);
                            inspector.target = None;
                        }
                    } else {
                        ui.label("Target lost");
                        inspector.target = None;
                    }
                }
                InspectorTarget::Item(id) => {
                    if let Some(index) = session.state.ground_items.iter().position(|i| i.item.id == id) {
                        let ground_item = &session.state.ground_items[index];
                        ui.heading(format!("Item: {}", ground_item.item.name));
                        ui.label(format!("ID: {}", ground_item.item.id));
                        ui.label(format!("Position: ({}, {})", ground_item.position.x, ground_item.position.y));
                        ui.label(format!("Family: {:?}", ground_item.item.family));
                        
                        ui.separator();
                        if ui.button("Despawn").clicked() {
                            session.state.ground_items.remove(index);
                            inspector.target = None;
                        }
                    } else {
                        ui.label("Target lost");
                        inspector.target = None;
                    }
                }
            }
        });

    if !open {
        inspector.target = None;
    }
}

fn find_entity_at(state: &omega_core::GameState, pos: Position) -> Option<InspectorTarget> {
    // Prioritize monsters
    if let Some(monster) = state.monsters.iter().find(|m| m.position == pos) {
        return Some(InspectorTarget::Monster(monster.id));
    }
    // Then items
    if let Some(item) = state.ground_items.iter().find(|i| i.position == pos) {
        return Some(InspectorTarget::Item(item.item.id));
    }
    None
}

// Re-using coordinate translation logic from spawner.rs 
// In a real refactor, this should be moved to a shared utility module.
// For now, we duplicate it to keep changes isolated and self-contained.
fn translate_cursor_to_grid(
    rel_cursor: &RelativeCursorPosition,
    node: &ComputedNode,
    layout: &UiLayoutTokens,
    readability: &UiReadabilityConfig,
    frame: &RuntimeFrame,
) -> Option<Position> {
    let normalized = rel_cursor.normalized?;
    let cursor_pixels = (normalized + 0.5) * node.size();
    
    let font_size = layout.map_font_size * readability.scale;
    let char_w = font_size * 0.6;
    let char_h = font_size;

    let Some(rendered_frame) = &frame.frame else { return None; };
    let map_w = rendered_frame.bounds.0 as usize;
    let map_h = rendered_frame.bounds.1 as usize;
    
    let focus = rendered_frame.tiles.iter()
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
