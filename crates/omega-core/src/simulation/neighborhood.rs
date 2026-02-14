use super::cell::Cell;
use super::grid::CaGrid;

pub const MOORE_OFFSETS: [(isize, isize); 8] =
    [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

pub const VON_NEUMANN_OFFSETS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

pub fn get_neighbor(grid: &CaGrid, x: usize, y: usize, dx: isize, dy: isize) -> Cell {
    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if grid.in_bounds(nx, ny) { *grid.get(nx as usize, ny as usize) } else { Cell::default() }
}

pub fn moore_neighbors(grid: &CaGrid, x: usize, y: usize) -> [Cell; 8] {
    let mut neighbors = [Cell::default(); 8];
    for (i, &(dx, dy)) in MOORE_OFFSETS.iter().enumerate() {
        neighbors[i] = get_neighbor(grid, x, y, dx, dy);
    }
    neighbors
}

pub fn von_neumann_neighbors(grid: &CaGrid, x: usize, y: usize) -> [Cell; 4] {
    let mut neighbors = [Cell::default(); 4];
    for (i, &(dx, dy)) in VON_NEUMANN_OFFSETS.iter().enumerate() {
        neighbors[i] = get_neighbor(grid, x, y, dx, dy);
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::super::state::Solid;
    use super::*;

    #[test]
    fn test_get_neighbor_edge_cases() {
        let mut grid = CaGrid::new(2, 2);
        let cell = Cell {
            solid: Some(Solid::Stone),
            ..Default::default()
        };
        grid.set(1, 1, cell);
        grid.swap_buffers();

        // (0,0) neighbor at (1,1)
        let n = get_neighbor(&grid, 0, 0, 1, 1);
        assert_eq!(n.solid, Some(Solid::Stone));

        // (0,0) neighbor at (-1,0) (out of bounds)
        let n = get_neighbor(&grid, 0, 0, -1, 0);
        assert_eq!(n.solid, None);
    }

    #[test]
    fn test_moore_neighbors() {
        let grid = CaGrid::new(3, 3);
        let neighbors = moore_neighbors(&grid, 1, 1);
        assert_eq!(neighbors.len(), 8);
    }

    #[test]
    fn test_von_neumann_neighbors() {
        let grid = CaGrid::new(3, 3);
        let neighbors = von_neumann_neighbors(&grid, 1, 1);
        assert_eq!(neighbors.len(), 4);
    }
}
