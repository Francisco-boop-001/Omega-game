use bevy::prelude::*;
use bevy::diagnostic::{Diagnostic, DiagnosticMeasurement, DiagnosticPath, DiagnosticsStore, FrameTimeDiagnosticsPlugin, RegisterDiagnostic};
use crate::simulation::projectiles::Projectile;
use crate::simulation::particles::Particle;
use crate::simulation::safety::SafetyConfig;
use crate::simulation::systems::SimulationTick;
use omega_core::simulation::grid::CaGrid;
use std::time::Instant;

pub const CA_UPDATE_TIME: DiagnosticPath = DiagnosticPath::const_new("ca/update_ms");
pub const FIXED_UPDATE_TIME: DiagnosticPath = DiagnosticPath::const_new("fixed_update/total_ms");

#[derive(Resource, Default)]
pub struct PerfSnapshot {
    pub fps: f64,
    pub ca_update_ms: f64,
    pub fixed_update_ms: f64,
    pub projectile_count: usize,
    pub particle_count: usize,
    pub active_cells: usize,     // Non-empty cells in CA grid
    pub event_log: Vec<String>,   // Last 20 reaction events
    pub show_logs: bool,          // Toggle for collapsible panel
    pub in_emergency: bool,       // From SafetyConfig
}

#[derive(Event)]
pub struct ReactionLog {
    pub description: String,  // e.g., "Fireball hit Water -> Generated 15 Steam cells"
}

#[derive(Resource, Default)]
pub struct CaTimingState {
    pub start: Option<Instant>,
}

#[derive(Resource, Default)]
pub struct FixedTimingState {
    pub start: Option<Instant>,
}

pub struct CaDiagnosticsPlugin;

impl Plugin for CaDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(CA_UPDATE_TIME).with_max_history_length(20))
           .register_diagnostic(Diagnostic::new(FIXED_UPDATE_TIME).with_max_history_length(20))
           .insert_resource(PerfSnapshot::default())
           .insert_resource(CaTimingState::default())
           .insert_resource(FixedTimingState::default())
           .add_event::<ReactionLog>()
           .add_systems(Update, (update_perf_snapshot_system, collect_reaction_logs_system));
    }
}

pub fn ca_timing_start(mut state: ResMut<CaTimingState>) {
    state.start = Some(Instant::now());
}

pub fn ca_timing_end(
    mut state: ResMut<CaTimingState>,
    mut diagnostics: ResMut<DiagnosticsStore>,
) {
    if let Some(start) = state.start.take() {
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        if let Some(diagnostic) = diagnostics.get_mut(&CA_UPDATE_TIME) {
            diagnostic.add_measurement(DiagnosticMeasurement {
                time: Instant::now(),
                value: elapsed,
            });
        }
    }
}

pub fn fixed_timing_start(mut state: ResMut<FixedTimingState>) {
    state.start = Some(Instant::now());
}

pub fn fixed_timing_end(
    mut state: ResMut<FixedTimingState>,
    mut diagnostics: ResMut<DiagnosticsStore>,
) {
    if let Some(start) = state.start.take() {
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        if let Some(diagnostic) = diagnostics.get_mut(&FIXED_UPDATE_TIME) {
            diagnostic.add_measurement(DiagnosticMeasurement {
                time: Instant::now(),
                value: elapsed,
            });
        }
    }
}

fn update_perf_snapshot_system(
    diagnostics: Res<DiagnosticsStore>,
    projectiles: Query<&Projectile>,
    particles: Query<&Particle>,
    grid: Res<CaGrid>,
    safety: Option<Res<SafetyConfig>>,
    tick: Res<SimulationTick>,
    mut snapshot: ResMut<PerfSnapshot>,
) {
    if tick.0 % 10 != 0 {
        return;
    }

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            snapshot.fps = value;
        }
    }

    if let Some(ca_time) = diagnostics.get(&CA_UPDATE_TIME) {
        if let Some(value) = ca_time.smoothed() {
            snapshot.ca_update_ms = value;
        }
    }
    
    // Also update fixed_update_ms if available
    if let Some(fixed_time) = diagnostics.get(&FIXED_UPDATE_TIME) {
        if let Some(value) = fixed_time.smoothed() {
            snapshot.fixed_update_ms = value;
        }
    }

    snapshot.projectile_count = projectiles.iter().count();
    snapshot.particle_count = particles.iter().count();
    
    // Count active cells: iterate CaGrid, count cells where !cell.is_empty()
    let mut active = 0;
    for cell in grid.front_buffer() {
        if !cell.is_empty() {
            active += 1;
        }
    }
    snapshot.active_cells = active;

    if let Some(safety) = safety {
        snapshot.in_emergency = safety.in_emergency;
    }
}

fn collect_reaction_logs_system(
    mut events: EventReader<ReactionLog>,
    mut snapshot: ResMut<PerfSnapshot>,
) {
    for event in events.read() {
        snapshot.event_log.push(event.description.clone());
    }
    
    if snapshot.event_log.len() > 20 {
        let drain_count = snapshot.event_log.len() - 20;
        snapshot.event_log.drain(0..drain_count);
    }
}
