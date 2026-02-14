use super::cell::Cell;
use super::state::{Gas, Solid};

pub fn apply_residual_decay(cell: &Cell) -> Cell {
    let mut next = *cell;
    
    // Heat is Residual and decays slowly
    next.heat = next.heat.saturating_sub(1);
    
    // Wet decays if no liquid layer
    if next.liquid.is_none() {
        next.wet = next.wet.saturating_sub(1);
    }
    
    // Pressure decays
    next.pressure = next.pressure.saturating_sub(2);
    
    // Smoke/Steam dissipation proxy
    if matches!(next.gas, Some(Gas::Smoke) | Some(Gas::Steam)) {
        if next.heat < 20 && next.pressure < 20 {
            next.gas = None;
        }
    }

    next
}

pub fn apply_nature_reclaims(cell: &Cell) -> Cell {
    let mut next = *cell;

    // Ash recovery: Ash + moisture + cool -> Earth
    if matches!(next.solid, Some(Solid::Ash)) && next.heat < 10 && next.wet > 50 {
        next.solid = Some(Solid::Earth);
    }

    // Rubble erosion: Rubble + wet -> Earth
    if matches!(next.solid, Some(Solid::Rubble)) && next.wet > 100 {
        next.solid = Some(Solid::Earth);
    }

    // Mud drying: Mud + dry + heat -> Earth
    if matches!(next.solid, Some(Solid::Mud)) && next.wet < 30 && next.heat > 0 {
        next.solid = Some(Solid::Earth);
    }

    // Steam dissipates when cool
    if matches!(next.gas, Some(Gas::Steam)) && next.heat < 50 {
        next.gas = None;
    }

    // Fire burnout: No fuel + cool -> None
    if matches!(next.gas, Some(Gas::Fire)) && (next.solid.is_none() || !next.can_ignite()) && next.heat < 100 {
        next.gas = None;
    }

    next
}

pub fn apply_full_decay_cycle(cell: &Cell, is_reclaim_tick: bool) -> Cell {
    let mut next = apply_residual_decay(cell);
    if is_reclaim_tick {
        next = apply_nature_reclaims(&next);
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::state::Solid;

    #[test]
    fn test_residual_heat_decay() {
        let mut cell = Cell::default();
        cell.heat = 10;
        let next = apply_residual_decay(&cell);
        assert_eq!(next.heat, 9);
    }

    #[test]
    fn test_nature_reclaims_ash() {
        let mut cell = Cell::default();
        cell.solid = Some(Solid::Ash);
        cell.heat = 5;
        cell.wet = 60;
        let next = apply_nature_reclaims(&cell);
        assert_eq!(next.solid, Some(Solid::Earth));
    }

    #[test]
    fn test_nature_reclaims_tick_dependency() {
        let mut cell = Cell::default();
        cell.solid = Some(Solid::Ash);
        cell.heat = 5;
        cell.wet = 60;
        
        let next = apply_full_decay_cycle(&cell, false);
        assert_eq!(next.solid, Some(Solid::Ash));
        
        let next = apply_full_decay_cycle(&cell, true);
        assert_eq!(next.solid, Some(Solid::Earth));
    }
}
