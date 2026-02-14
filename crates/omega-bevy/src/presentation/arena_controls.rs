use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use omega_core::simulation::grid::CaGrid;
use omega_core::simulation::wind::WindGrid;
use omega_core::simulation::catastrophe::Catastrophe;
use omega_core::simulation::snapshot::{SnapshotManager, ArenaSnapshot};
use crate::simulation::turret::TurretMode;
use crate::simulation::diagnostics::PerfSnapshot;
use crate::{AppState, RuntimeStatus};

#[derive(Resource, Default)]
pub struct ArenaSnapshotState {
    pub manager: SnapshotManager,
}

pub fn arena_controls_ui_system(
    mut contexts: EguiContexts,
    status: Res<RuntimeStatus>,
    mut grid: ResMut<CaGrid>,
    mut wind_grid: ResMut<WindGrid>,
    mut turret: ResMut<TurretMode>,
    perf: Res<PerfSnapshot>,
    mut snapshots: ResMut<ArenaSnapshotState>,
) {
    if status.app_state != AppState::WizardArena {
        return;
    }

    egui::Window::new("Arena Controls")
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -10.0))
        .default_open(true)
        .show(contexts.ctx_mut(), |ui| {
            let (w, h) = (grid.width(), grid.height());
            ui.collapsing("Catastrophe Suite", |ui| {
                if ui.button("Great Flood").clicked() {
                    Catastrophe::great_flood(&mut grid, (w / 2, h / 2));
                }
                if ui.button("Forest Fire").clicked() {
                    Catastrophe::forest_fire_jump(&mut grid, (w / 2, h / 2));
                }
                if ui.button("Windstorm").clicked() {
                    Catastrophe::massive_windstorm(&mut wind_grid);
                }
                if ui.button("Fuel Field").clicked() {
                    Catastrophe::fuel_field(&mut grid);
                }
                ui.separator();
                if ui.add(egui::Button::new("DOOMSDAY").fill(egui::Color32::RED)).clicked() {
                    Catastrophe::doomsday(&mut grid, &mut wind_grid);
                }
            });

            ui.separator();

            ui.checkbox(&mut turret.active, "Turret Active");
            ui.add(egui::Slider::new(&mut turret.fire_rate_hz, 1.0..=20.0).text("Fire Rate (Hz)"));

            ui.separator();

            ui.collapsing("Snapshot Controls", |ui| {
                let current_snap_count = snapshots.manager.list().len();
                if ui.button("Save Snapshot").clicked() {
                    snapshots.manager.push(ArenaSnapshot::capture(&grid, format!("Snapshot {}", current_snap_count + 1)));
                }
                if ui.button("Restore Snapshot").clicked() {
                    if let Some(s) = snapshots.manager.pop() {
                        s.restore(&mut grid);
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
                ui.colored_label(traffic_light, "●");
                ui.label(format!("FPS: {:.1}", perf.fps));
            });
            ui.label(format!("CA: {:.1}ms", perf.ca_update_ms));
            ui.label(format!("Projectiles: {}", perf.projectile_count));
            ui.label(format!("Particles: {}", perf.particle_count));
            
            if perf.in_emergency {
                 ui.colored_label(egui::Color32::RED, "EMERGENCY STOP ACTIVE");
            }
        });
}
