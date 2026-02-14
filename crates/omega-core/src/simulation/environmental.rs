use super::grid::CaGrid;
use super::state::Gas;

pub fn apply_liquid_flow(grid: &mut CaGrid) {
    let width = grid.width();
    let height = grid.height();

    // Bottom-up scan to prevent teleportation
    for y in (0..height).rev() {
        for x in 0..width {
            let cell = *grid.get(x, y);
            let liquid_type = match cell.liquid {
                Some(lt) => lt,
                None => continue,
            };

            // 1. Try to flow directly down
            if y + 1 < height {
                let target_x = x;
                let target_y = y + 1;
                let target_cell = grid.get(target_x, target_y);

                if target_cell.solid.is_none() && target_cell.liquid.is_none() {
                    let mut new_source = cell;
                    new_source.liquid = None;
                    new_source.wet = 0;

                    let mut new_dest = *target_cell;
                    new_dest.liquid = Some(liquid_type);
                    new_dest.wet = cell.wet;

                    grid.set(x, y, new_source);
                    grid.set(target_x, target_y, new_dest);
                    continue;
                }

                // 2. Try diagonal down
                let parity = (x + y) % 2;
                let offsets = if parity == 0 { [(-1, 1), (1, 1)] } else { [(1, 1), (-1, 1)] };

                let mut moved = false;
                for (dx, dy) in offsets {
                    let tx = x as isize + dx;
                    let ty = y as isize + dy;

                    if grid.in_bounds(tx, ty) {
                        let tx = tx as usize;
                        let ty = ty as usize;

                        // Check if path is blocked by solid diagonally or horizontally
                        let side_x = x as isize + dx;
                        let side_y = y as isize;
                        let side_cell = grid.get(side_x as usize, side_y as usize);
                        let dest_cell = grid.get(tx, ty);

                        if dest_cell.solid.is_none()
                            && dest_cell.liquid.is_none()
                            && side_cell.solid.is_none()
                        {
                            let mut new_source = cell;
                            new_source.liquid = None;
                            new_source.wet = 0;

                            let mut new_dest = *dest_cell;
                            new_dest.liquid = Some(liquid_type);
                            new_dest.wet = cell.wet;

                            grid.set(x, y, new_source);
                            grid.set(tx, ty, new_dest);
                            moved = true;
                            break;
                        }
                    }
                }
                if moved {
                    continue;
                }
            }

            // 3. Horizontal spread (pooling)
            let parity = (x + y) % 2;
            let h_offsets = if parity == 0 { [-1, 1] } else { [1, -1] };

            for dx in h_offsets {
                let tx = x as isize + dx;
                let ty = y as isize;

                if grid.in_bounds(tx, ty) {
                    let tx = tx as usize;
                    let ty = ty as usize;
                    let dest_cell = grid.get(tx, ty);

                    if dest_cell.solid.is_none() && dest_cell.liquid.is_none() {
                        // Spread: split wetness
                        let spread_wet = cell.wet / 2;
                        if spread_wet > 0 || cell.wet == 0 {
                            let mut new_dest = *dest_cell;
                            new_dest.liquid = Some(liquid_type);
                            new_dest.wet = spread_wet;

                            let mut new_source = cell;
                            new_source.wet = cell.wet - spread_wet;

                            grid.set(x, y, new_source);
                            grid.set(tx, ty, new_dest);
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn apply_gas_rise(grid: &mut CaGrid) {
    let width = grid.width();
    let height = grid.height();

    // Top-down scan to prevent teleportation upward
    for y in 0..height {
        for x in 0..width {
            let cell = *grid.get(x, y);
            let gas_type = match cell.gas {
                Some(gt) if gt != Gas::Fire => gt,
                _ => continue,
            };

            let mut moved = false;

            // 1. Try to rise directly up
            if y > 0 {
                let target_x = x;
                let target_y = y - 1;
                let target_cell = grid.get(target_x, target_y);

                if target_cell.solid.is_none() && target_cell.gas.is_none() {
                    let mut new_source = cell;
                    new_source.gas = None;
                    new_source.pressure = 0;

                    let mut new_dest = *target_cell;
                    new_dest.gas = Some(gas_type);
                    new_dest.pressure = cell.pressure;

                    grid.set(x, y, new_source);
                    grid.set(target_x, target_y, new_dest);
                    moved = true;
                }
            }

            if moved {
                continue;
            }

            // 2. Try diagonal up
            if y > 0 {
                let parity = (x + y) % 2;
                let offsets = if parity == 0 { [(-1, -1), (1, -1)] } else { [(1, -1), (-1, -1)] };

                for (dx, dy) in offsets {
                    let tx = x as isize + dx;
                    let ty = y as isize + dy;

                    if grid.in_bounds(tx, ty) {
                        let tx = tx as usize;
                        let ty = ty as usize;
                        let dest_cell = grid.get(tx, ty);

                        if dest_cell.solid.is_none() && dest_cell.gas.is_none() {
                            let mut new_source = cell;
                            new_source.gas = None;
                            new_source.pressure = 0;

                            let mut new_dest = *dest_cell;
                            new_dest.gas = Some(gas_type);
                            new_dest.pressure = cell.pressure;

                            grid.set(x, y, new_source);
                            grid.set(tx, ty, new_dest);
                            moved = true;
                            break;
                        }
                    }
                }
            }

            if moved {
                continue;
            }

            // 3. Horizontal spread if ceiling-blocked
            let is_at_top = y == 0;
            let blocked_above = !is_at_top && grid.get(x, y - 1).solid.is_some();

            if is_at_top || blocked_above {
                let parity = (x + y) % 2;
                let h_offsets = if parity == 0 { [-1, 1] } else { [1, -1] };

                for dx in h_offsets {
                    let tx = x as isize + dx;
                    let ty = y as isize;

                    if grid.in_bounds(tx, ty) {
                        let tx = tx as usize;
                        let ty = ty as usize;
                        let dest_cell = grid.get(tx, ty);

                        if dest_cell.solid.is_none() && dest_cell.gas.is_none() {
                            // Spread horizontally: split pressure
                            let spread_pressure = cell.pressure / 2;
                            if spread_pressure > 0 {
                                let mut new_dest = *dest_cell;
                                new_dest.gas = Some(gas_type);
                                new_dest.pressure = spread_pressure;

                                let mut new_source = cell;
                                new_source.pressure = cell.pressure - spread_pressure;

                                grid.set(x, y, new_source);
                                grid.set(tx, ty, new_dest);
                                moved = true;
                                break;
                            }
                        }
                    }
                }
            }

            if moved {
                continue;
            }

            // 4. Dissipation if trapped
            let mut new_cell = cell;
            if cell.pressure >= 5 {
                new_cell.pressure -= 5;
            } else {
                new_cell.pressure = 0;
            }

            if new_cell.pressure < 10 {
                new_cell.gas = None;
                new_cell.pressure = 0;
            }
            grid.set(x, y, new_cell);
        }
    }
}

pub fn apply_fire_spread_bias(grid: &mut CaGrid) {
    let width = grid.width();
    let height = grid.height();
    use super::neighborhood::MOORE_OFFSETS;

    for y in 0..height {
        for x in 0..width {
            let mut cell = *grid.get(x, y);
            if !cell.can_ignite() {
                continue;
            }

            let mut heat_add = 0u32;
            for (dx, dy) in MOORE_OFFSETS {
                let tx = x as isize + dx;
                let ty = y as isize + dy;

                if grid.in_bounds(tx, ty) {
                    let neighbor = grid.get(tx as usize, ty as usize);
                    if let Some(Gas::Fire) = neighbor.gas {
                        // dy > 0 means neighbor is BELOW current cell.
                        // Fire rises, so fire below heats more.
                        if dy > 0 {
                            heat_add += 30;
                        } else if dy < 0 {
                            heat_add += 10;
                        } else {
                            heat_add += 20;
                        }
                    }
                }
            }

            if heat_add > 0 {
                cell.heat = cell.heat.saturating_add(heat_add.min(255) as u8);
                grid.set(x, y, cell);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::cell::Cell;
    use super::super::state::{Liquid, Solid};
    use super::*;

    #[test]
    fn test_liquid_falls_down() {
        let mut grid = CaGrid::new(3, 3);
        let water_cell = Cell { liquid: Some(Liquid::Water), ..Default::default() };

        // Place water at (1, 1)
        grid.set(1, 1, water_cell);
        grid.swap_buffers();

        apply_liquid_flow(&mut grid);
        grid.swap_buffers();

        // Should have moved to (1, 2)
        assert!(grid.get(1, 1).liquid.is_none());
        assert_eq!(grid.get(1, 2).liquid, Some(Liquid::Water));
    }

    #[test]
    fn test_liquid_flows_diagonally() {
        let mut grid = CaGrid::new(3, 3);

        // Solid at (1, 2) blocking direct fall from (1, 1)
        let stone = Cell { solid: Some(Solid::Stone), ..Default::default() };
        grid.set(1, 2, stone);

        let water = Cell { liquid: Some(Liquid::Water), ..Default::default() };
        grid.set(1, 1, water);

        grid.swap_buffers();

        apply_liquid_flow(&mut grid);
        grid.swap_buffers();

        // Should have moved to (0, 2) or (2, 2)
        assert!(grid.get(1, 1).liquid.is_none());
        let moved = grid.get(0, 2).liquid.is_some() || grid.get(2, 2).liquid.is_some();
        assert!(moved, "Liquid should have moved diagonally");
    }

    #[test]
    fn test_liquid_spreads_horizontally() {
        let mut grid = CaGrid::new(3, 3);

        // Floor at y=2
        let stone = Cell { solid: Some(Solid::Stone), ..Default::default() };
        grid.set(0, 2, stone);
        grid.set(1, 2, stone);
        grid.set(2, 2, stone);

        let water = Cell { liquid: Some(Liquid::Water), ..Default::default() };
        grid.set(1, 1, water);

        grid.swap_buffers();

        apply_liquid_flow(&mut grid);
        grid.swap_buffers();

        // Should have spread to (0, 1) or (2, 1)
        // Note: horizontal spread in the plan says "Source keeps its liquid, destination gets liquid"
        // so it should be at (1, 1) AND either (0, 1) or (2, 1)
        assert_eq!(grid.get(1, 1).liquid, Some(Liquid::Water));
        let spread = grid.get(0, 1).liquid.is_some() || grid.get(2, 1).liquid.is_some();
        assert!(spread, "Liquid should have spread horizontally");
    }

    #[test]
    fn test_liquid_respects_solids() {
        let mut grid = CaGrid::new(3, 3);

        let stone = Cell { solid: Some(Solid::Stone), ..Default::default() };
        grid.set(1, 2, stone); // Directly below
        grid.set(0, 2, stone); // Diagonal down left
        grid.set(2, 2, stone); // Diagonal down right
        grid.set(0, 1, stone); // Left
        grid.set(2, 1, stone); // Right

        let water = Cell { liquid: Some(Liquid::Water), ..Default::default() };
        grid.set(1, 1, water);

        grid.swap_buffers();

        apply_liquid_flow(&mut grid);
        grid.swap_buffers();

        // Should still be at (1, 1) and nowhere else
        assert_eq!(grid.get(1, 1).liquid, Some(Liquid::Water));
        assert!(grid.get(1, 2).solid.is_some());
        assert!(grid.get(1, 2).liquid.is_none());
    }

    #[test]
    fn test_liquid_does_not_overwrite_liquid() {
        let mut grid = CaGrid::new(3, 3);

        let oil = Cell { liquid: Some(Liquid::Oil), ..Default::default() };
        grid.set(1, 2, oil);

        let water = Cell { liquid: Some(Liquid::Water), ..Default::default() };
        grid.set(1, 1, water);

        grid.swap_buffers();

        apply_liquid_flow(&mut grid);
        grid.swap_buffers();

        // Water at (1,1) should not move to (1,2) because oil is there.
        // It might try to move diagonally though.
        assert_eq!(grid.get(1, 2).liquid, Some(Liquid::Oil));
    }

    #[test]
    fn test_gas_rises_up() {
        let mut grid = CaGrid::new(3, 3);
        let steam = Cell { gas: Some(Gas::Steam), pressure: 50, ..Default::default() };

        // Place steam at (1, 1)
        grid.set(1, 1, steam);
        grid.swap_buffers();

        apply_gas_rise(&mut grid);
        grid.swap_buffers();

        // Should have moved to (1, 0)
        assert!(grid.get(1, 1).gas.is_none());
        assert_eq!(grid.get(1, 0).gas, Some(Gas::Steam));
        assert_eq!(grid.get(1, 0).pressure, 50);
    }

    #[test]
    fn test_fire_does_not_rise() {
        let mut grid = CaGrid::new(3, 3);
        let fire = Cell { gas: Some(Gas::Fire), pressure: 50, ..Default::default() };

        grid.set(1, 1, fire);
        grid.swap_buffers();

        apply_gas_rise(&mut grid);
        grid.swap_buffers();

        // Fire should stay put
        assert_eq!(grid.get(1, 1).gas, Some(Gas::Fire));
        assert!(grid.get(1, 0).gas.is_none());
    }

    #[test]
    fn test_gas_dissipates_when_trapped() {
        let mut grid = CaGrid::new(3, 3);

        // Surround (1, 1) with stone
        let stone = Cell { solid: Some(Solid::Stone), ..Default::default() };
        grid.set(1, 0, stone);
        grid.set(1, 2, stone);
        grid.set(0, 1, stone);
        grid.set(2, 1, stone);
        grid.set(0, 0, stone);
        grid.set(2, 0, stone);
        grid.set(0, 2, stone);
        grid.set(2, 2, stone);

        let smoke = Cell { gas: Some(Gas::Smoke), pressure: 20, ..Default::default() };
        grid.set(1, 1, smoke);

        grid.swap_buffers();

        apply_gas_rise(&mut grid);
        grid.swap_buffers();

        // Pressure should drop by 5
        assert_eq!(grid.get(1, 1).pressure, 15);

        // After few more calls it should disappear
        for _ in 0..2 {
            apply_gas_rise(&mut grid);
            grid.swap_buffers();
        }
        // 15 -> 10 -> 0 (because 5 < 10)
        assert_eq!(grid.get(1, 1).pressure, 0);
        assert!(grid.get(1, 1).gas.is_none());

        apply_gas_rise(&mut grid);
        grid.swap_buffers();
        // 5 < 10, should be None
        assert!(grid.get(1, 1).gas.is_none());
    }

    #[test]
    fn test_fire_spread_bias() {
        let mut grid = CaGrid::new(3, 3);

        // Combustible wood at (1, 1)
        let wood = Cell { solid: Some(Solid::Wood), heat: 0, ..Default::default() };
        grid.set(1, 1, wood);

        // Fire below at (1, 2)
        let fire = Cell { gas: Some(Gas::Fire), ..Default::default() };
        grid.set(1, 2, fire);

        grid.swap_buffers();
        apply_fire_spread_bias(&mut grid);
        grid.swap_buffers();

        // Fire below should add 30 heat
        assert_eq!(grid.get(1, 1).heat, 30);

        // Reset and test fire above
        let mut grid = CaGrid::new(3, 3);
        grid.set(1, 1, wood);
        grid.set(1, 0, fire); // Fire above at (1, 0)

        grid.swap_buffers();
        apply_fire_spread_bias(&mut grid);
        grid.swap_buffers();

        // Fire above should add 10 heat
        assert_eq!(grid.get(1, 1).heat, 10);
    }
}
