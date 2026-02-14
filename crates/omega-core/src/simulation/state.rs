use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Solid {
    Earth,
    Stone,
    Mud,
    Ash,
    Rubble,
    Grass,
    Wood,
}

impl fmt::Display for Solid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Earth => write!(f, "Earth"),
            Self::Stone => write!(f, "Stone"),
            Self::Mud => write!(f, "Mud"),
            Self::Ash => write!(f, "Ash"),
            Self::Rubble => write!(f, "Rubble"),
            Self::Grass => write!(f, "Grass"),
            Self::Wood => write!(f, "Wood"),
        }
    }
}

impl Solid {
    pub fn flash_point(&self) -> Option<u8> {
        match self {
            Self::Grass => Some(120),
            Self::Wood => Some(180),
            Self::Stone => Some(250),
            _ => None,
        }
    }

    pub fn is_combustible(&self) -> bool {
        self.flash_point().is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Liquid {
    Water,
    Oil,
}

impl fmt::Display for Liquid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Water => write!(f, "Water"),
            Self::Oil => write!(f, "Oil"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gas {
    Steam,
    Smoke,
    Fire,
}

impl fmt::Display for Gas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Steam => write!(f, "Steam"),
            Self::Smoke => write!(f, "Smoke"),
            Self::Fire => write!(f, "Fire"),
        }
    }
}

impl Gas {
    pub fn is_flammable(&self) -> bool {
        matches!(self, Self::Fire)
    }
}
