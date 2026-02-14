use super::state::{Solid, Liquid, Gas};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cell {
    pub solid: Option<Solid>,
    pub liquid: Option<Liquid>,
    pub gas: Option<Gas>,
    pub heat: u8,
    pub wet: u8,
    pub pressure: u8,
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        self.solid.is_none() && self.liquid.is_none() && self.gas.is_none()
    }

    pub fn is_waterlogged(&self) -> bool {
        self.wet >= 255
    }

    pub fn can_ignite(&self) -> bool {
        if self.is_waterlogged() {
            return false;
        }
        if matches!(self.gas, Some(Gas::Steam)) {
            return false;
        }
        if let Some(solid) = self.solid {
            return solid.is_combustible();
        }
        false
    }

    pub fn visible_material(&self) -> &str {
        if let Some(gas) = self.gas {
            match gas {
                Gas::Steam => "Steam",
                Gas::Smoke => "Smoke",
                Gas::Fire => "Fire",
            }
        } else if let Some(liquid) = self.liquid {
            match liquid {
                Liquid::Water => "Water",
                Liquid::Oil => "Oil",
            }
        } else if let Some(solid) = self.solid {
            match solid {
                Solid::Earth => "Earth",
                Solid::Stone => "Stone",
                Solid::Mud => "Mud",
                Solid::Ash => "Ash",
                Solid::Rubble => "Rubble",
                Solid::Grass => "Grass",
                Solid::Wood => "Wood",
            }
        } else {
            "Air"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cell_is_empty_air() {
        let cell = Cell::default();
        assert!(cell.is_empty());
        assert_eq!(cell.visible_material(), "Air");
    }

    #[test]
    fn test_waterlogged_detection() {
        let mut cell = Cell::default();
        cell.wet = 255;
        assert!(cell.is_waterlogged());
    }

    #[test]
    fn test_can_ignite() {
        let mut cell = Cell::default();
        cell.solid = Some(Solid::Grass);
        assert!(cell.can_ignite());

        cell.wet = 255;
        assert!(!cell.can_ignite());

        cell.wet = 0;
        cell.gas = Some(Gas::Steam);
        assert!(!cell.can_ignite());

        cell.gas = None;
        cell.solid = Some(Solid::Stone);
        assert!(cell.can_ignite()); // Stone is combustible (250 flash point)

        cell.solid = Some(Solid::Ash);
        assert!(!cell.can_ignite());
    }

    #[test]
    fn test_visible_material_priority() {
        let mut cell = Cell::default();
        cell.solid = Some(Solid::Earth);
        assert_eq!(cell.visible_material(), "Earth");

        cell.liquid = Some(Liquid::Water);
        assert_eq!(cell.visible_material(), "Water");

        cell.gas = Some(Gas::Smoke);
        assert_eq!(cell.visible_material(), "Smoke");
    }
}
