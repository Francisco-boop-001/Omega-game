use super::systems::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use omega_core::simulation::grid::CaGrid;
use omega_core::simulation::wind::WindGrid;

use super::diagnostics::*;
use super::emitters::*;
use super::random::SimulationRandom;
use super::safety::*;
use super::turret::*;
use bevy::time::Real;
use omega_core::simulation::displacement::DisplacementEvent;

pub struct SimulationPlugin {
    pub width: usize,
    pub height: usize,
    pub seed: u64,
    pub tick_rate_hz: f64,
}

impl SimulationPlugin {
    pub fn new(width: usize, height: usize, seed: u64) -> Self {
        Self { width, height, seed, tick_rate_hz: 64.0 }
    }
}

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(CaDiagnosticsPlugin)
            .insert_resource(CaGrid::new(self.width, self.height))
            .insert_resource(WindGrid::new(self.width, self.height))
            .insert_resource(SimulationTick::default())
            .insert_resource(SimulationRandom::seeded(self.seed))
            .insert_resource(TurretMode::default())
            .insert_resource(SafetyConfig::default())
            .init_resource::<bevy::core::FrameCount>()
            .init_resource::<Time>()
            .init_resource::<Time<Real>>()
            .insert_resource(Time::<Fixed>::from_hz(self.tick_rate_hz))
            .add_event::<DisplacementEvent>()
            .add_systems(
                FixedUpdate,
                (
                    turret_mode_system,
                    fixed_timing_start,
                    increment_tick,
                    particle_physics_system,
                    particle_wind_drift_system,
                    particle_lifecycle_system,
                    particle_visual_cascade_system,
                    trail_emitter_system,
                    explosion_emitter_system,
                    projectile_movement_system,
                    projectile_collision_system,
                    projectile_interception_system,
                    ca_timing_start,
                    update_ca_cells,
                    process_explosions,
                    environmental_behaviors,
                    swap_ca_buffers,
                    ca_timing_end,
                    fixed_timing_end,
                )
                    .chain(),
            )
            .add_systems(Update, (emergency_cleanup_system, particle_cap_system));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_registration() {
        let mut app = App::new();
        app.add_plugins(SimulationPlugin::new(32, 32, 0x1234));

        assert!(app.world().get_resource::<CaGrid>().is_some());
        assert!(app.world().get_resource::<WindGrid>().is_some());
        assert!(app.world().get_resource::<SimulationTick>().is_some());
        assert!(app.world().get_resource::<SimulationRandom>().is_some());
        assert_eq!(app.world().get_resource::<CaGrid>().unwrap().width(), 32);
    }
}
