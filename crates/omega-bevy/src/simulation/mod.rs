pub mod plugin;
pub mod systems;
pub mod projectiles;
pub mod particles;
pub mod emitters;
pub mod turret;
pub mod safety;
pub mod diagnostics;

pub use plugin::SimulationPlugin;
pub use systems::SimulationTick;
pub use projectiles::Projectile;
pub use particles::{Particle, VisualCascade, ParticleKind};
