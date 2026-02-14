use bevy_ecs::prelude::Resource;
use super::cell::Cell;
use super::grid::CaGrid;
use super::state::Solid;

#[derive(Debug, Clone, Copy, Default)]
pub struct WindVector {
    pub dx: i8,        // -1, 0, 1 direction
    pub dy: i8,        // -1, 0, 1 direction
    pub strength: u8,  // 0-255 force magnitude
}

#[derive(Resource, Debug, Clone)]
pub struct WindGrid {
    width: usize,
    height: usize,
    vectors: Vec<WindVector>,
}

impl WindGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            vectors: vec![WindVector::default(); width * height],
        }
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

pub fn apply_wind(grid: &CaGrid, wind: &WindGrid, x: usize, y: usize, cell: &Cell) -> (Cell, Option<(usize, usize, Cell)>) {
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
        if current.gas.is_some() && vector.strength > 50 {
            target_part.gas = current.gas;
            current.gas = None;
            
            // Strong wind disperses gas faster (simulated by not moving it all)
            if vector.strength > 200 {
                target_part.gas = None; 
            }
        }

        // 2. Heat push
        let heat_to_move = (current.heat as u16 * vector.strength as u16 / 1024) as u8;
        target_part.heat = heat_to_move;
        current.heat = current.heat.saturating_sub(heat_to_move);

        // 3. Debris interaction (Ash only)
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
        
        let mut cell = Cell::default();
        cell.gas = Some(Gas::Smoke);
        
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
        
        let mut cell = Cell::default();
        cell.solid = Some(Solid::Stone);
        
        let (current, displaced) = apply_wind(&grid, &wind, 1, 1, &cell);
        assert_eq!(current.solid, Some(Solid::Stone));
        // Displaced might contain heat but not the solid
        if let Some((_, _, d_cell)) = displaced {
            assert_ne!(d_cell.solid, Some(Solid::Stone));
        }
    }
}
