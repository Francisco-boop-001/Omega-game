use bevy::prelude::*;
use omega_core::simulation::{TrajectoryMode, ProjectilePhysicalProps, Cell};

#[derive(Component)]
pub struct Projectile {
    pub logical_pos: Vec3, // x, y are grid, z is height
    pub velocity: Vec3,
    pub mode: TrajectoryMode,
    pub props: ProjectilePhysicalProps,
    pub element_impact: Option<Cell>, // CA state to apply on hit
}
