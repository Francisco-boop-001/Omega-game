use super::particles::Particle;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use omega_core::simulation::grid::CaGrid;

#[derive(Resource)]
pub struct SafetyConfig {
    pub cleanup_threshold_fps: f64,
    pub recovery_threshold_fps: f64,
    pub cooldown_secs: f32,
    pub particle_cap: usize,
    pub in_emergency: bool,
    pub cooldown_timer: f32,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            cleanup_threshold_fps: 20.0,
            recovery_threshold_fps: 30.0,
            cooldown_secs: 5.0,
            particle_cap: 500,
            in_emergency: false,
            cooldown_timer: 0.0,
        }
    }
}

pub fn emergency_cleanup_system(
    diagnostics: Res<DiagnosticsStore>,
    mut config: ResMut<SafetyConfig>,
    mut grid: ResMut<CaGrid>,
    time: Res<Time>,
    particle_query: Query<(Entity, &Particle)>,
    mut commands: Commands,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(60.0);

    if !config.in_emergency && fps < config.cleanup_threshold_fps {
        config.in_emergency = true;
        config.cooldown_timer = config.cooldown_secs;

        warn!("Emergency cleanup triggered! FPS: {:.2}", fps);

        // Clear all gas and liquid from CaGrid
        let width = grid.width();
        let height = grid.height();
        for y in 0..height {
            for x in 0..width {
                let mut cell = *grid.get(x, y);
                cell.gas = None;
                cell.liquid = None;
                cell.pressure = 0;
                cell.wet = 0;
                grid.set_immediate(x, y, cell);
            }
        }

        // Despawn all particles beyond cap
        let mut particles: Vec<_> = particle_query.iter().collect();
        if particles.len() > config.particle_cap {
            particles.sort_by(|a, b| b.1.age.partial_cmp(&a.1.age).unwrap());
            for (entity, _) in particles.iter().take(particles.len() - config.particle_cap) {
                commands.entity(*entity).despawn();
            }
        }
    }

    if config.in_emergency {
        config.cooldown_timer -= time.delta_secs();
        if fps > config.recovery_threshold_fps && config.cooldown_timer <= 0.0 {
            config.in_emergency = false;
            info!("Emergency recovery complete. FPS: {:.2}", fps);
        }
    }
}

pub fn particle_cap_system(
    config: Res<SafetyConfig>,
    particle_query: Query<(Entity, &Particle)>,
    mut commands: Commands,
) {
    let mut particles: Vec<_> = particle_query.iter().collect();
    if particles.len() > config.particle_cap {
        particles.sort_by(|a, b| b.1.age.partial_cmp(&a.1.age).unwrap());
        for (entity, _) in particles.iter().take(particles.len() - config.particle_cap) {
            commands.entity(*entity).despawn();
        }
    }
}
