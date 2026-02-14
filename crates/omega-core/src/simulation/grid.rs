use super::cell::Cell;
use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct CaGrid {
    width: usize,
    height: usize,
    front: Vec<Cell>,
    back: Vec<Cell>,
}

impl CaGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            front: vec![Cell::default(); size],
            back: vec![Cell::default(); size],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        debug_assert!(x < self.width && y < self.height);
        &self.front[y * self.width + x]
    }

    pub fn get_checked(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width && y < self.height { Some(&self.front[y * self.width + x]) } else { None }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        debug_assert!(x < self.width && y < self.height);
        self.back[y * self.width + x] = cell;
    }

    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
        self.back.copy_from_slice(&self.front);
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }

    pub fn cell_count(&self) -> usize {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_dimensions() {
        let grid = CaGrid::new(10, 5);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 5);
        assert_eq!(grid.cell_count(), 50);
    }

    #[test]
    fn test_get_set_and_swap() {
        let mut grid = CaGrid::new(2, 2);
        let mut cell = Cell::default();
        cell.heat = 100;

        grid.set(1, 1, cell);
        assert_eq!(grid.get(1, 1).heat, 0); // Read from front

        grid.swap_buffers();
        assert_eq!(grid.get(1, 1).heat, 100); // Read from new front
    }

    #[test]
    fn test_get_checked() {
        let grid = CaGrid::new(2, 2);
        assert!(grid.get_checked(1, 1).is_some());
        assert!(grid.get_checked(2, 1).is_none());
    }

    #[test]
    fn test_in_bounds() {
        let grid = CaGrid::new(2, 2);
        assert!(grid.in_bounds(0, 0));
        assert!(grid.in_bounds(1, 1));
        assert!(!grid.in_bounds(2, 0));
        assert!(!grid.in_bounds(-1, 0));
    }
}
