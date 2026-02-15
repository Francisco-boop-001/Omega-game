use crate::presentation::arena_controls::ArenaActionFeedback;
use crate::presentation::cursor_grid::{CursorGridError, map_panel_cursor_to_grid};
use crate::presentation::theme::UiLayoutTokens;
use crate::presentation::{MapPanelCard, UiReadabilityConfig};
use crate::{AppState, FrontendRuntime, Position, RuntimeFrame, RuntimeStatus};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_egui::{EguiContexts, egui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorTarget {
    Monster(u64),
    Item(u32),
}

#[derive(Debug, Clone, Resource, Default)]
pub struct InspectorState {
    pub target: Option<InspectorTarget>,
}

#[derive(SystemParam)]
pub struct MouseInspectorParams<'w, 's> {
    contexts: EguiContexts<'w, 's>,
    buttons: Res<'w, ButtonInput<MouseButton>>,
    q_map_panel:
        Query<'w, 's, (&'static RelativeCursorPosition, &'static ComputedNode), With<MapPanelCard>>,
    inspector: ResMut<'w, InspectorState>,
    runtime: Res<'w, FrontendRuntime>,
    status: Res<'w, RuntimeStatus>,
    layout: Res<'w, UiLayoutTokens>,
    readability: Res<'w, UiReadabilityConfig>,
    frame: Res<'w, RuntimeFrame>,
    feedback: ResMut<'w, ArenaActionFeedback>,
}

pub fn mouse_inspector_system(params: MouseInspectorParams) {
    let MouseInspectorParams {
        mut contexts,
        buttons,
        q_map_panel,
        mut inspector,
        runtime,
        status,
        layout,
        readability,
        frame,
        mut feedback,
    } = params;

    if status.app_state != AppState::WizardArena {
        return;
    }

    if !buttons.just_pressed(MouseButton::Right) {
        return;
    }
    if let Some(ctx) = contexts.try_ctx_mut()
        && ctx.wants_pointer_input()
    {
        return;
    }
    let Ok((rel_cursor, node)) = q_map_panel.get_single() else {
        return;
    };
    let pos = match map_panel_cursor_to_grid(rel_cursor, node, &layout, &readability, &frame) {
        Ok(pos) => pos,
        Err(err @ (CursorGridError::NoFrame | CursorGridError::InvalidGeometry)) => {
            feedback.push_line(format!("Inspector canceled: {}", cursor_error_message(err)));
            return;
        }
        Err(CursorGridError::CursorOutsideNode | CursorGridError::OutOfBounds) => {
            return;
        }
    };
    let Some(session) = &runtime.0.session else {
        feedback.push_line("Inspector failed: no active session.".to_string());
        return;
    };
    inspector.target = find_entity_at(&session.state, pos);
    if inspector.target.is_none() {
        feedback.push_line(format!("No entity at ({}, {}).", pos.x, pos.y));
    }
}

pub fn inspector_ui_system(
    mut contexts: EguiContexts,
    mut inspector: ResMut<InspectorState>,
    mut runtime: ResMut<FrontendRuntime>,
    status: Res<RuntimeStatus>,
    mut feedback: ResMut<ArenaActionFeedback>,
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
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };
    egui::Window::new("Inspector").open(&mut open).show(ctx, |ui| match target {
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
                    feedback.push_line(format!("Despawned monster {}", id));
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
                ui.label(format!(
                    "Position: ({}, {})",
                    ground_item.position.x, ground_item.position.y
                ));
                ui.label(format!("Family: {:?}", ground_item.item.family));

                ui.separator();
                if ui.button("Despawn").clicked() {
                    session.state.ground_items.remove(index);
                    inspector.target = None;
                    feedback.push_line(format!("Despawned item {}", id));
                }
            } else {
                ui.label("Target lost");
                inspector.target = None;
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

fn cursor_error_message(err: CursorGridError) -> &'static str {
    match err {
        CursorGridError::NoFrame => "runtime frame unavailable",
        CursorGridError::CursorOutsideNode => "cursor is outside map panel",
        CursorGridError::OutOfBounds => "click is outside rendered map viewport",
        CursorGridError::InvalidGeometry => "map panel geometry is invalid",
    }
}
