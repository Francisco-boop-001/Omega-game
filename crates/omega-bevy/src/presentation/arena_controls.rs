use crate::simulation::diagnostics::PerfSnapshot;
use crate::simulation::turret::TurretMode;
use crate::{AppState, FrontendRuntime, InputAction, RuntimeStatus};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use omega_core::simulation::grid::CaGrid;
use omega_core::simulation::snapshot::{ArenaSnapshot, SnapshotManager};
use omega_core::simulation::wind::WindGrid;
use std::collections::VecDeque;

#[derive(Resource, Default)]
pub struct ArenaSnapshotState {
    pub manager: SnapshotManager,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct ArenaOverlayState {
    pub show_ca_overlay: bool,
}

impl Default for ArenaOverlayState {
    fn default() -> Self {
        Self { show_ca_overlay: true }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct ArenaToolingState {
    pub enabled: bool,
    pub reset_requested: bool,
}

impl Default for ArenaToolingState {
    fn default() -> Self {
        Self { enabled: true, reset_requested: false }
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct ArenaActionFeedback {
    lines: VecDeque<String>,
}

impl ArenaActionFeedback {
    const MAX_LINES: usize = 32;

    pub fn push_line(&mut self, line: String) {
        self.lines.push_back(line);
        while self.lines.len() > Self::MAX_LINES {
            self.lines.pop_front();
        }
    }

    pub fn recent_lines(&self, max: usize) -> Vec<String> {
        let len = self.lines.len();
        let start = len.saturating_sub(max);
        self.lines.iter().skip(start).cloned().collect()
    }
}

#[derive(SystemParam)]
pub struct ArenaControlsParams<'w> {
    status: Res<'w, RuntimeStatus>,
    grid: ResMut<'w, CaGrid>,
    turret: ResMut<'w, TurretMode>,
    perf: Res<'w, PerfSnapshot>,
    snapshots: ResMut<'w, ArenaSnapshotState>,
    overlay: ResMut<'w, ArenaOverlayState>,
    tooling: ResMut<'w, ArenaToolingState>,
    feedback: ResMut<'w, ArenaActionFeedback>,
}

pub fn arena_controls_ui_system(mut contexts: EguiContexts, params: ArenaControlsParams) {
    let ArenaControlsParams {
        status,
        mut grid,
        mut turret,
        perf,
        mut snapshots,
        mut overlay,
        mut tooling,
        mut feedback,
    } = params;

    if status.app_state != AppState::WizardArena {
        return;
    }

    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("Arena Controls")
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
        .default_open(true)
        .default_width(320.0)
        .show(ctx, |ui| {
            let was_enabled = tooling.enabled;
            ui.checkbox(&mut tooling.enabled, "Test Ground Controls (F9)");
            if was_enabled != tooling.enabled {
                feedback.push_line(if tooling.enabled {
                    "Test Ground controls enabled.".to_string()
                } else {
                    "Test Ground controls disabled.".to_string()
                });
            }
            if !tooling.enabled {
                ui.label("Controls disabled. Press F9 to enable.");
                ui.separator();
                ui.label("Action Feedback");
                for line in feedback.recent_lines(6) {
                    ui.label(line);
                }
                return;
            }

            if ui.button("Reset Arena Fixture").clicked() {
                tooling.reset_requested = true;
                feedback.push_line("Arena reset requested.".to_string());
            }
            ui.separator();

            ui.collapsing("Disaster Targeting", |ui| {
                ui.label("Disasters are now click-targeted.");
                ui.label("Use Wizard Test Spawner -> Category: Disasters.");
                ui.label("Then click a map cell to apply the effect there.");
            });

            ui.separator();

            let was_active = turret.active;
            ui.checkbox(&mut turret.active, "Turret Active");
            if was_active != turret.active {
                feedback.push_line(if turret.active {
                    "Turret mode enabled.".to_string()
                } else {
                    "Turret mode disabled.".to_string()
                });
            }
            let before_rate = turret.fire_rate_hz;
            ui.add(egui::Slider::new(&mut turret.fire_rate_hz, 1.0..=20.0).text("Fire Rate (Hz)"));
            if (before_rate - turret.fire_rate_hz).abs() > f32::EPSILON {
                feedback
                    .push_line(format!("Turret fire rate set to {:.1}Hz.", turret.fire_rate_hz));
            }

            ui.separator();
            ui.checkbox(&mut overlay.show_ca_overlay, "Show CA Overlay");

            ui.separator();

            ui.collapsing("Snapshot Controls", |ui| {
                let current_snap_count = snapshots.manager.list().len();
                if ui.button("Save Snapshot").clicked() {
                    snapshots.manager.push(ArenaSnapshot::capture(
                        &grid,
                        format!("Snapshot {}", current_snap_count + 1),
                    ));
                    feedback.push_line(format!("Saved snapshot {}.", current_snap_count + 1));
                }
                if ui.button("Restore Snapshot").clicked() {
                    if let Some(s) = snapshots.manager.pop() {
                        let label = s.label.clone();
                        s.restore(&mut grid);
                        feedback.push_line(format!("Restored {}.", label));
                    } else {
                        feedback.push_line(
                            "Restore Snapshot failed: no snapshots available.".to_string(),
                        );
                    }
                }

                ui.label("Snapshots:");
                for (i, snap) in snapshots.manager.list().iter().enumerate() {
                    ui.label(format!("{}: {}", i + 1, snap.label));
                }
            });

            ui.separator();

            ui.label("Performance");
            let traffic_light = if perf.fps >= 58.0 {
                egui::Color32::GREEN
            } else if perf.fps >= 45.0 {
                egui::Color32::YELLOW
            } else {
                egui::Color32::RED
            };

            ui.horizontal(|ui| {
                ui.colored_label(traffic_light, "o");
                ui.label(format!("FPS: {:.1}", perf.fps));
            });
            ui.label(format!("CA: {:.1}ms", perf.ca_update_ms));
            ui.label(format!("Projectiles: {}", perf.projectile_count));
            ui.label(format!("Particles: {}", perf.particle_count));

            if perf.in_emergency {
                ui.colored_label(egui::Color32::RED, "EMERGENCY STOP ACTIVE");
            }

            ui.separator();
            ui.heading("Action Feedback");
            for line in feedback.recent_lines(6) {
                ui.label(line);
            }
        });
}

pub fn apply_arena_tooling_requests_system(
    status: Res<RuntimeStatus>,
    mut runtime: ResMut<FrontendRuntime>,
    mut tooling: ResMut<ArenaToolingState>,
    mut snapshots: ResMut<ArenaSnapshotState>,
    mut feedback: ResMut<ArenaActionFeedback>,
    mut spawner: ResMut<crate::presentation::spawner::SpawnerState>,
    mut grid: ResMut<CaGrid>,
    mut wind_grid: ResMut<WindGrid>,
) {
    if status.app_state != AppState::WizardArena || !tooling.reset_requested {
        return;
    }

    tooling.reset_requested = false;
    runtime.0.apply_action(InputAction::StartWizardArena);

    if let Some(session) = runtime.0.session.as_ref() {
        let width = usize::try_from(session.state.bounds.width.max(1)).unwrap_or(1);
        let height = usize::try_from(session.state.bounds.height.max(1)).unwrap_or(1);
        *grid = CaGrid::new(width, height);
        *wind_grid = WindGrid::new(width, height);
    }

    *snapshots = ArenaSnapshotState::default();
    *spawner = crate::presentation::spawner::SpawnerState::default();
    *feedback = ArenaActionFeedback::default();
    feedback.push_line("Arena fixture reset.".to_string());
}
