use bevy::prelude::*;
use omega_core::simulation::{TrajectoryMode, ProjectilePhysicalProps, Cell, Gas, Liquid};
use omega_core::simulation::grid::CaGrid;
use rand::Rng;
use super::projectiles::Projectile;

#[derive(Resource)]
pub struct TurretMode {
    pub active: bool,
    pub fire_rate_hz: f32,
    pub accumulator: f32,
}

impl Default for TurretMode {
    fn default() -> Self {
        Self {
            active: false,
            fire_rate_hz: 5.0,
            accumulator: 0.0,
        }
    }
}

pub fn turret_mode_system(
    time: Res<Time<Fixed>>,
    grid: Res<CaGrid>,
    mut turret: ResMut<TurretMode>,
    mut commands: Commands,
) {
    if !turret.active {
        return;
    }

    turret.accumulator += time.delta_secs();
    let interval = 1.0 / turret.fire_rate_hz;

    while turret.accumulator >= interval {
        turret.accumulator -= interval;

        let mut rng = rand::rng();
        let width = grid.width() as f32;
        let height = grid.height() as f32;

        for _ in 0..100 { // Fire 100 at once per trigger
            let origin_x = rng.random_range(0.0..width);
            let origin_y = rng.random_range(0.0..height);
            let target_x = rng.random_range(0.0..width);
            let target_y = rng.random_range(0.0..height);

            let element_impact = match rng.random_range(0..3) {
                0 => {
                    // Fire
                    let mut cell = Cell::default();
                    cell.heat = 200;
                    cell.gas = Some(Gas::Fire);
                    Some(cell)
                }
                1 => {
                    // Water
                    let mut cell = Cell::default();
                    cell.wet = 100;
                    cell.liquid = Some(Liquid::Water);
                    Some(cell)
                }
                _ => {
                    // Explosive
                    let mut cell = Cell::default();
                    cell.pressure = 250;
                    Some(cell)
                }
            };

            let origin = Vec3::new(origin_x, origin_y, 20.0); // Start very high
            let target = Vec3::new(target_x, target_y, 0.0);
            let dir = (target - origin).normalize_or_zero();
            let speed = 1.0; // Extremely slow fall
            let final_velocity = Vec3::new(dir.x * speed, dir.y * speed, 2.0);

            commands.spawn((
                Projectile {
                    logical_pos: origin,
                    velocity: final_velocity,
                    mode: TrajectoryMode::HighArc,
                    props: ProjectilePhysicalProps {
                        mass: 1.0,
                        volume: 1.0,
                        intensity: 100,
                    },
                    element_impact,
                },
                Transform::from_translation(origin),
            ));
        }
    }
}
