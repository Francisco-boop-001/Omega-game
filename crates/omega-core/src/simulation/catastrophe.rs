use super::grid::CaGrid;
use super::state::{Gas, Liquid, Solid};
use super::wind::{WindGrid, WindVector};

#[derive(Debug, Clone, Copy)]
pub struct TurretModeConfig {
    pub active: bool,
    pub fire_rate_hz: f32,
}

pub struct Catastrophe;

impl Catastrophe {
    pub fn great_flood(grid: &mut CaGrid, center: (usize, usize)) {
        let (cx, cy) = center;
        let radius = 15;
        let width = grid.width();
        let height = grid.height();

        for y in cy.saturating_sub(radius)..=(cy + radius).min(height - 1) {
            for x in cx.saturating_sub(radius)..=(cx + radius).min(width - 1) {
                let dx = x as isize - cx as isize;
                let dy = y as isize - cy as isize;
                if dx * dx + dy * dy <= (radius * radius) as isize {
                    let mut cell = *grid.get(x, y);
                    cell.wet = 100;
                    cell.liquid = Some(Liquid::Water);
                    grid.set_immediate(x, y, cell);
                }
            }
        }
    }

    pub fn forest_fire_jump(grid: &mut CaGrid, origin: (usize, usize)) {
        let (ox, oy) = origin;
        let radius = 20;
        let width = grid.width();
        let height = grid.height();

        for y in oy.saturating_sub(radius)..=(oy + radius).min(height - 1) {
            for x in ox.saturating_sub(radius)..=(ox + radius).min(width - 1) {
                let dx = x as isize - ox as isize;
                let dy = y as isize - oy as isize;
                if dx * dx + dy * dy <= (radius * radius) as isize {
                    let mut cell = *grid.get(x, y);
                    cell.heat = 200;
                    cell.gas = Some(Gas::Fire);
                    grid.set_immediate(x, y, cell);
                }
            }
        }
    }

    pub fn massive_windstorm(wind_grid: &mut WindGrid) {
        wind_grid.set_global(WindVector { dx: 1, dy: -1, strength: 200 });
    }

    pub fn fuel_field(grid: &mut CaGrid) {
        let width = grid.width();
        let height = grid.height();
        for y in 0..height {
            for x in 0..width {
                let is_border = x < 5 || x >= width - 5 || y < 5 || y >= height - 5;
                let mut cell = *grid.get(x, y);
                if is_border {
                    cell.solid = Some(Solid::Stone);
                } else {
                    cell.solid = Some(Solid::Wood);
                }
                grid.set_immediate(x, y, cell);
            }
        }
    }

    pub fn interception_chaos() -> TurretModeConfig {
        TurretModeConfig { active: true, fire_rate_hz: 20.0 }
    }

    pub fn doomsday(grid: &mut CaGrid, wind_grid: &mut WindGrid) -> TurretModeConfig {
        let width = grid.width();
        let height = grid.height();

        Self::great_flood(grid, (width / 2, height / 2));
        Self::forest_fire_jump(grid, (width / 4, height / 4));
        Self::massive_windstorm(wind_grid);
        Self::fuel_field(grid);

        Self::interception_chaos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_great_flood() {
        let mut grid = CaGrid::new(40, 40);
        Catastrophe::great_flood(&mut grid, (20, 20));
        let cell = grid.get(20, 20);
        assert_eq!(cell.wet, 100);
        assert_eq!(cell.liquid, Some(Liquid::Water));
    }

    #[test]
    fn test_forest_fire_jump() {
        let mut grid = CaGrid::new(40, 40);
        Catastrophe::forest_fire_jump(&mut grid, (20, 20));
        let cell = grid.get(20, 20);
        assert_eq!(cell.heat, 200);
        assert_eq!(cell.gas, Some(Gas::Fire));
    }

    #[test]
    fn test_massive_windstorm() {
        let mut wind_grid = WindGrid::new(10, 10);
        Catastrophe::massive_windstorm(&mut wind_grid);
        let vector = wind_grid.get(0, 0);
        assert_eq!(vector.dx, 1);
        assert_eq!(vector.dy, -1);
        assert_eq!(vector.strength, 200);
    }

    #[test]
    fn test_fuel_field() {
        let mut grid = CaGrid::new(20, 20);
        Catastrophe::fuel_field(&mut grid);
        assert_eq!(grid.get(0, 0).solid, Some(Solid::Stone));
        assert_eq!(grid.get(5, 5).solid, Some(Solid::Wood));
    }

    #[test]
    fn test_interception_chaos() {
        let config = Catastrophe::interception_chaos();
        assert!(config.active);
        assert_eq!(config.fire_rate_hz, 20.0);
    }
}
