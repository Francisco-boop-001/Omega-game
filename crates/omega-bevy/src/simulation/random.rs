use bevy::prelude::Resource;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Resource, Debug, Clone)]
pub struct SimulationRandom {
    rng: StdRng,
}

impl SimulationRandom {
    pub fn seeded(seed: u64) -> Self {
        Self { rng: StdRng::seed_from_u64(seed) }
    }

    pub fn next_f32(&mut self) -> f32 {
        self.rng.random::<f32>()
    }

    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        if max <= min {
            return min;
        }
        self.rng.random_range(min..max)
    }

    pub fn range_u32(&mut self, min: u32, max_exclusive: u32) -> u32 {
        if max_exclusive <= min {
            return min;
        }
        self.rng.random_range(min..max_exclusive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_random_is_reproducible() {
        let mut left = SimulationRandom::seeded(0xC0FFEE);
        let mut right = SimulationRandom::seeded(0xC0FFEE);
        for _ in 0..32 {
            assert_eq!(left.range_u32(0, 1_000), right.range_u32(0, 1_000));
        }
    }
}
