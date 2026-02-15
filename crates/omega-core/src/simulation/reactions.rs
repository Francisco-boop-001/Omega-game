use super::cell::Cell;
use super::state::Gas;
use super::transitions::{apply_decay, apply_transitions};

pub fn count_burning_neighbors(neighbors: &[Cell; 8]) -> u8 {
    neighbors.iter().filter(|n| matches!(n.gas, Some(Gas::Fire))).count() as u8
}

pub fn avg_neighbor_heat(neighbors: &[Cell; 8]) -> u8 {
    let sum: u32 = neighbors.iter().map(|n| n.heat as u32).sum();
    (sum / 8) as u8
}

pub fn apply_reactions(cell: &Cell, neighbors: &[Cell; 8]) -> Cell {
    let mut next = *cell;

    // 1. Extinguish: Fire + Wet neighbor -> Steam
    if matches!(next.gas, Some(Gas::Fire)) {
        let max_neighbor_wet = neighbors.iter().map(|n| n.wet).max().unwrap_or(0);
        if max_neighbor_wet > 150 {
            next.gas = Some(Gas::Steam);
            next.heat = next.heat.saturating_sub(50);
        }
    }

    // 2. Spread fire: Combustible + Burning neighbor -> Heat increase
    if next.can_ignite() {
        let burning_count = count_burning_neighbors(neighbors);
        if burning_count > 0 {
            next.heat = next.heat.saturating_add(burning_count.saturating_mul(20));
        }
    }

    // 3. Moisture transfer (diffusion-like)
    let avg_wet = (neighbors.iter().map(|n| n.wet as u32).sum::<u32>() / 8) as u8;
    if avg_wet > next.wet {
        next.wet = next.wet.saturating_add(((avg_wet - next.wet) as f32 * 0.1) as u8);
    }

    // 4. Heat transfer (diffusion)
    let avg_heat = avg_neighbor_heat(neighbors);
    if avg_heat != next.heat {
        let delta = (avg_heat as i16 - next.heat as i16) as f32 * 0.1;
        if delta > 0.0 {
            next.heat = next.heat.saturating_add(delta as u8);
        } else {
            next.heat = next.heat.saturating_sub((-delta) as u8);
        }
    }

    // 5. Pressure from combustion
    if matches!(next.gas, Some(Gas::Fire)) && next.solid.is_some() {
        next.pressure = next.pressure.saturating_add(10);
    }

    next
}

pub fn compute_next_cell(cell: &Cell, neighbors: &[Cell; 8]) -> Cell {
    let mut next = apply_reactions(cell, neighbors);
    next = apply_transitions(&next, neighbors);
    apply_decay(&next, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::state::{Gas, Solid};

    #[test]
    fn test_fire_plus_water_makes_steam() {
        let cell = Cell { gas: Some(Gas::Fire), ..Cell::default() };
        let mut neighbors = [Cell::default(); 8];
        neighbors[0].wet = 200;
        let next = apply_reactions(&cell, &neighbors);
        assert_eq!(next.gas, Some(Gas::Steam));
    }

    #[test]
    fn test_fire_spreads_to_adjacent_combustible() {
        let cell = Cell { solid: Some(Solid::Grass), ..Cell::default() };
        let mut neighbors = [Cell::default(); 8];
        neighbors[0].gas = Some(Gas::Fire);
        let next = apply_reactions(&cell, &neighbors);
        assert!(next.heat > 0);
    }

    #[test]
    fn test_hot_neighbor_transfers_heat() {
        let cell = Cell::default();
        let mut neighbors = [Cell::default(); 8];
        neighbors[0].heat = 200;
        let next = apply_reactions(&cell, &neighbors);
        assert!(next.heat > 0);
    }
}
