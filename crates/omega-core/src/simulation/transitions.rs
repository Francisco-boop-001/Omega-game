use super::cell::Cell;
use super::state::{Gas, Liquid, Solid};

pub fn apply_heat(cell: &Cell, amount: u8, is_violent: bool) -> Cell {
    let mut next = *cell;
    if next.is_waterlogged() {
        return next;
    }

    if is_violent
        && let Some(solid) = next.solid
        && let Some(flash) = solid.flash_point()
        && amount >= flash
    {
        next.gas = Some(Gas::Fire);
        next.heat = 255;
        return next;
    }

    next.heat = next.heat.saturating_add(amount);
    if let Some(solid) = next.solid
        && let Some(flash) = solid.flash_point()
        && next.heat >= flash
    {
        next.gas = Some(Gas::Fire);
    }
    next
}

pub fn apply_transitions(cell: &Cell, _neighbors: &[Cell; 8]) -> Cell {
    let mut next = *cell;

    // 1. Evaporation: Water + Heat > 200 -> Steam
    if matches!(next.liquid, Some(Liquid::Water)) && next.heat > 200 {
        next.liquid = None;
        next.gas = Some(Gas::Steam);
        next.heat = next.heat.saturating_sub(50);
    }
    // 2. Condensation: Steam + Heat < 180 -> Water
    else if matches!(next.gas, Some(Gas::Steam)) && next.heat < 180 {
        next.gas = None;
        next.liquid = Some(Liquid::Water);
    }

    // 3. Mud formation: Earth + Wet > 150 -> Mud
    if matches!(next.solid, Some(Solid::Earth)) && next.wet > 150 {
        next.solid = Some(Solid::Mud);
    }
    // 4. Mud drying: Mud + Wet < 100 -> Earth
    else if matches!(next.solid, Some(Solid::Mud)) && next.wet < 100 {
        next.solid = Some(Solid::Earth);
    }

    // 5. Combustion completion: Fire + exhausted -> Ash
    if matches!(next.gas, Some(Gas::Fire))
        && let Some(solid) = next.solid
        && solid.is_combustible()
        && next.heat < 50
    {
        next.gas = None;
        next.solid = Some(Solid::Ash);
    }

    // 6. Fire extinguish: Fire + Wet >= 200 -> Out
    if matches!(next.gas, Some(Gas::Fire)) && next.wet >= 200 {
        next.gas = None;
        next.heat /= 2;
        next.wet = next.wet.saturating_sub(100);
    }

    next
}

pub fn apply_decay(cell: &Cell, decay_rate: u8) -> Cell {
    let mut next = *cell;
    next.heat = next.heat.saturating_sub(decay_rate);
    next.wet = next.wet.saturating_sub(decay_rate / 2);
    next.pressure = next.pressure.saturating_sub(decay_rate);
    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::state::{Gas, Liquid, Solid};

    #[test]
    fn test_fireball_ignites_grass() {
        let cell = Cell { solid: Some(Solid::Grass), ..Default::default() };
        let next = apply_heat(&cell, 200, true);
        assert_eq!(next.gas, Some(Gas::Fire));
        assert_eq!(next.heat, 255);
    }

    #[test]
    fn test_fireball_ignites_wood() {
        let cell = Cell { solid: Some(Solid::Wood), ..Default::default() };
        let next = apply_heat(&cell, 200, true);
        assert_eq!(next.gas, Some(Gas::Fire));
    }

    #[test]
    fn test_fireball_fails_on_stone() {
        let cell = Cell { solid: Some(Solid::Stone), ..Default::default() };
        let next = apply_heat(&cell, 200, true);
        assert_ne!(next.gas, Some(Gas::Fire));
    }

    #[test]
    fn test_torch_accumulates_heat() {
        let cell = Cell { solid: Some(Solid::Grass), heat: 100, ..Default::default() };
        let next = apply_heat(&cell, 30, false);
        assert_eq!(next.heat, 130);
        assert_eq!(next.gas, Some(Gas::Fire)); // 130 > 120
    }

    #[test]
    fn test_torch_below_flash_point() {
        let cell = Cell { solid: Some(Solid::Wood), heat: 100, ..Default::default() };
        let next = apply_heat(&cell, 30, false);
        assert_eq!(next.heat, 130);
        assert_ne!(next.gas, Some(Gas::Fire)); // 130 < 180
    }

    #[test]
    fn test_water_to_steam_at_200() {
        let cell = Cell { liquid: Some(Liquid::Water), heat: 201, ..Default::default() };
        let next = apply_transitions(&cell, &[Cell::default(); 8]);
        assert_eq!(next.liquid, None);
        assert_eq!(next.gas, Some(Gas::Steam));
    }

    #[test]
    fn test_steam_stays_steam_at_185() {
        let cell = Cell { gas: Some(Gas::Steam), heat: 185, ..Default::default() };
        let next = apply_transitions(&cell, &[Cell::default(); 8]);
        assert_eq!(next.gas, Some(Gas::Steam));
    }

    #[test]
    fn test_steam_to_water_at_179() {
        let cell = Cell { gas: Some(Gas::Steam), heat: 179, ..Default::default() };
        let next = apply_transitions(&cell, &[Cell::default(); 8]);
        assert_eq!(next.gas, None);
        assert_eq!(next.liquid, Some(Liquid::Water));
    }

    #[test]
    fn test_waterlogged_blocks_ignition() {
        let cell = Cell { solid: Some(Solid::Grass), wet: 255, ..Default::default() };
        let next = apply_heat(&cell, 255, true);
        assert_ne!(next.gas, Some(Gas::Fire));
    }

    #[test]
    fn test_burning_grass_produces_ash() {
        let cell = Cell {
            solid: Some(Solid::Grass),
            gas: Some(Gas::Fire),
            heat: 40,
            ..Default::default()
        }; // Fuel low
        let next = apply_transitions(&cell, &[Cell::default(); 8]);
        assert_eq!(next.gas, None);
        assert_eq!(next.solid, Some(Solid::Ash));
    }
}
