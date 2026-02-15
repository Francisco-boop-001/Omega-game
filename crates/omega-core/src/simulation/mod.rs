use serde::{Deserialize, Serialize};

pub mod catastrophe;
pub mod cell;
pub mod decay;
pub mod displacement;
pub mod environmental;
pub mod grid;
pub mod neighborhood;
pub mod reactions;
pub mod snapshot;
pub mod state;
pub mod transitions;
pub mod wind;

pub use cell::Cell;
pub use grid::CaGrid;
pub use reactions::compute_next_cell;
pub use state::{Gas, Liquid, Solid};
pub use wind::{WindGrid, WindVector};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TrajectoryMode {
    #[default]
    HighArc,
    FlatArc,
    Rolling,
    Beam,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProjectilePhysicalProps {
    pub mass: f32,
    pub volume: f32,   // Used for 'Size Matters' interception logic
    pub intensity: u8, // For mid-air negation/negation logic
}
