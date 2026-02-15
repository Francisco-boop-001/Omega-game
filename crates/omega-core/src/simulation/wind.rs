use super::cell::Cell;
use super::grid::CaGrid;
use super::state::{Gas, Solid};
use bevy_ecs::prelude::Resource;

#[derive(Debug, Clone, Copy, Default)]
pub struct WindVector {
    pub dx: i8,       // -1, 0, 1 direction
    pub dy: i8,       // -1, 0, 1 direction
    pub strength: u8, // 0-255 force magnitude
}

#[derive(Resource, Debug, Clone)]
pub struct WindGrid {
    width: usize,
    height: usize,
    vectors: Vec<WindVector>,
}

impl WindGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, vectors: vec![WindVector::default(); width * height] }
    }

    pub fn get(&self, x: usize, y: usize) -> WindVector {
        debug_assert!(x < self.width && y < self.height);
        self.vectors[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, wind: WindVector) {
        debug_assert!(x < self.width && y < self.height);
        self.vectors[y * self.width + x] = wind;
    }

    pub fn set_global(&mut self, wind: WindVector) {
        for v in self.vectors.iter_mut() {
            *v = wind;
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }
}

pub fn apply_wind(
    grid: &CaGrid,
    wind: &WindGrid,
    x: usize,
    y: usize,
    cell: &Cell,
) -> (Cell, Option<(usize, usize, Cell)>) {
    let vector = wind.get(x, y);
    if vector.strength == 0 {
        return (*cell, None);
    }

    let mut current = *cell;
    let mut displaced = None;

    let tx = x as isize + vector.dx as isize;
    let ty = y as isize + vector.dy as isize;

    if grid.in_bounds(tx, ty) {
        let tx = tx as usize;
        let ty = ty as usize;
        let mut target_part = Cell::default();

        // 1. Gas displacement
        if let Some(gas) = current.gas
            && vector.strength > 35
        {
            target_part.gas = Some(gas);
            current.gas = None;

            // Extreme wind disperses flammable gas into smoke instead of deleting it.
            if vector.strength > 220 {
                target_part.gas = if gas == Gas::Fire { Some(Gas::Smoke) } else { None };
            }
        }

        // 2. Liquid advection
        if let Some(liquid) = current.liquid
            && vector.strength > 125
        {
            target_part.liquid = Some(liquid);
            let wet_transfer = (current.wet / 2).max(40);
            target_part.wet = target_part.wet.max(wet_transfer);
            current.wet = current.wet.saturating_sub((current.wet / 3).max(20));
            if vector.strength > 180 {
                current.liquid = None;
            }
        }

        // 3. Heat and pressure push
        let mut heat_to_move = (current.heat as u16 * vector.strength as u16 / 640) as u8;
        if heat_to_move == 0 && current.heat > 0 && vector.strength >= 120 {
            heat_to_move = 1;
        }
        target_part.heat = target_part.heat.saturating_add(heat_to_move);
        current.heat = current.heat.saturating_sub(heat_to_move);
        let pressure_to_move = (current.pressure as u16 * vector.strength as u16 / 768) as u8;
        target_part.pressure = target_part.pressure.saturating_add(pressure_to_move);
        current.pressure = current.pressure.saturating_sub(pressure_to_move);

        // 4. Fire fanning
        if current.gas == Some(Gas::Fire) && vector.strength >= 120 {
            current.heat = current.heat.saturating_add(8);
            current.pressure = current.pressure.saturating_add(6);
        }

        // 5. Debris interaction (Ash only)
        if matches!(current.solid, Some(Solid::Ash)) && vector.strength > 100 {
            target_part.solid = current.solid;
            current.solid = None;
        }

        if !target_part.is_empty() || target_part.heat > 0 {
            displaced = Some((tx, ty, target_part));
        }
    }

    (current, displaced)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::state::{Gas, Solid};

    #[test]
    fn test_wind_pushes_gas() {
        let grid = CaGrid::new(3, 3);
        let mut wind = WindGrid::new(3, 3);
        wind.set_global(WindVector { dx: 1, dy: 0, strength: 100 });

        let cell = Cell { gas: Some(Gas::Smoke), ..Cell::default() };

        let (current, displaced) = apply_wind(&grid, &wind, 1, 1, &cell);
        assert!(current.gas.is_none());
        let (tx, ty, d_cell) = displaced.unwrap();
        assert_eq!(tx, 2);
        assert_eq!(ty, 1);
        assert_eq!(d_cell.gas, Some(Gas::Smoke));
    }

    #[test]
    fn test_wind_respects_earth_anchoring() {
        let grid = CaGrid::new(3, 3);
        let mut wind = WindGrid::new(3, 3);
        wind.set_global(WindVector { dx: 1, dy: 0, strength: 255 });

        let cell = Cell { solid: Some(Solid::Stone), ..Cell::default() };

        let (current, displaced) = apply_wind(&grid, &wind, 1, 1, &cell);
        assert_eq!(current.solid, Some(Solid::Stone));
        // Displaced might contain heat but not the solid
        if let Some((_, _, d_cell)) = displaced {
            assert_ne!(d_cell.solid, Some(Solid::Stone));
        }
    }

    #[test]
    fn test_wind_advects_liquid_under_strong_gust() {
        let grid = CaGrid::new(3, 3);
        let mut wind = WindGrid::new(3, 3);
        wind.set_global(WindVector { dx: 1, dy: 0, strength: 200 });

        let cell = Cell {
            liquid: Some(crate::simulation::state::Liquid::Water),
            wet: 120,
            ..Cell::default()
        };

        let (_current, displaced) = apply_wind(&grid, &wind, 1, 1, &cell);
        let (_, _, d_cell) = displaced.expect("strong wind should displace content");
        assert_eq!(d_cell.liquid, Some(crate::simulation::state::Liquid::Water));
        assert!(d_cell.wet > 0);
    }

    #[test]
    fn test_extreme_wind_turns_fire_into_smoke_downwind() {
        let grid = CaGrid::new(3, 3);
        let mut wind = WindGrid::new(3, 3);
        wind.set_global(WindVector { dx: 1, dy: 0, strength: 230 });

        let cell = Cell { gas: Some(Gas::Fire), ..Cell::default() };
        let (_current, displaced) = apply_wind(&grid, &wind, 1, 1, &cell);
        let (_, _, d_cell) = displaced.expect("extreme wind should displace gas");
        assert_eq!(d_cell.gas, Some(Gas::Smoke));
    }
}
