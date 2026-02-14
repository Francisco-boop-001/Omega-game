use super::cell::Cell;
use super::grid::CaGrid;
use super::state::Gas;
use bevy_ecs::prelude::*;
use std::collections::VecDeque;

#[derive(Event)]
pub struct DisplacementEvent {
    pub origin_x: usize,
    pub origin_y: usize,
    pub heat: u8,
    pub pressure: u8,
    pub gas: Option<Gas>,
    pub radius: u8,
    pub is_violent: bool,
}

pub fn apply_explosive_displacement(grid: &mut CaGrid, event: &DisplacementEvent) {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; grid.cell_count()];

    let origin_idx = event.origin_y * grid.width() + event.origin_x;
    queue.push_back((event.origin_x, event.origin_y, 0u8));
    visited[origin_idx] = true;

    while let Some((x, y, dist)) = queue.pop_front() {
        if dist > event.radius {
            continue;
        }

        let mut cell = *grid.get(x, y);

        // Decay factors
        let factor = 0.8f32.powi(dist as i32);
        let heat_inc = (event.heat as f32 * factor) as u8;
        let press_inc = (event.pressure as f32 * factor) as u8;

        cell.heat = cell.heat.saturating_add(heat_inc);
        cell.pressure = cell.pressure.saturating_add(press_inc);

        // Push gas outward: only set gas on the "frontier" or outer edges of displacement?
        // Actually, let's just set it if the cell is currently empty of gas.
        if cell.gas.is_none() && dist > 0 {
            cell.gas = event.gas;
        }

        grid.set(x, y, cell);

        if dist < event.radius {
            for &(dx, dy) in crate::simulation::neighborhood::MOORE_OFFSETS.iter() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if grid.in_bounds(nx, ny) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    let nidx = ny * grid.width() + nx;
                    if !visited[nidx] {
                        visited[nidx] = true;
                        queue.push_back((nx, ny, dist + 1));
                    }
                }
            }
        }
    }
}

pub fn check_explosion_trigger(cell: &Cell, x: usize, y: usize) -> Option<DisplacementEvent> {
    if cell.pressure > 200
        && matches!(cell.gas, Some(Gas::Fire))
        && let Some(solid) = cell.solid
        && solid.is_combustible()
    {
        return Some(DisplacementEvent {
            origin_x: x,
            origin_y: y,
            heat: 200,
            pressure: 255,
            gas: Some(Gas::Fire),
            radius: 5,
            is_violent: true,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::state::Solid;

    #[test]
    fn test_explosion_distributes_heat() {
        let mut grid = CaGrid::new(11, 11);
        let event = DisplacementEvent {
            origin_x: 5,
            origin_y: 5,
            heat: 100,
            pressure: 100,
            gas: None,
            radius: 2,
            is_violent: true,
        };

        apply_explosive_displacement(&mut grid, &event);
        grid.swap_buffers();

        assert!(grid.get(5, 5).heat > 0);
        assert!(grid.get(6, 6).heat > 0);
        assert!(grid.get(7, 7).heat > 0);
        assert_eq!(grid.get(8, 8).heat, 0); // Beyond radius 2
    }

    #[test]
    fn test_explosion_trigger() {
        let cell = Cell {
            pressure: 210,
            gas: Some(Gas::Fire),
            solid: Some(Solid::Wood),
            ..Default::default()
        };

        let event = check_explosion_trigger(&cell, 0, 0);
        assert!(event.is_some());
        assert!(event.unwrap().is_violent);
    }
}
