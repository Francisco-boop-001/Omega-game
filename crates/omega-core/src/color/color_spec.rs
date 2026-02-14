//! Color specification supporting multiple color spaces.
//!
//! Provides conversion between RGB, ANSI 256-color, and ANSI 16-color spaces.

use super::hex_color::HexColor;
use serde::{Deserialize, Serialize};

/// A color specification that can represent colors in multiple color spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorSpec {
    /// 24-bit RGB color.
    Rgb { r: u8, g: u8, b: u8 },
    /// ANSI 256-color palette index (0-255).
    Indexed(u8),
    /// ANSI 16-color value.
    Ansi(AnsiColor),
}

/// ANSI 16-color palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnsiColor {
    /// Black (0).
    Black,
    /// Red (1).
    Red,
    /// Green (2).
    Green,
    /// Yellow (3).
    Yellow,
    /// Blue (4).
    Blue,
    /// Magenta (5).
    Magenta,
    /// Cyan (6).
    Cyan,
    /// White (7).
    White,
    /// Bright black / gray (8).
    BrightBlack,
    /// Bright red (9).
    BrightRed,
    /// Bright green (10).
    BrightGreen,
    /// Bright yellow (11).
    BrightYellow,
    /// Bright blue (12).
    BrightBlue,
    /// Bright magenta (13).
    BrightMagenta,
    /// Bright cyan (14).
    BrightCyan,
    /// Bright white (15).
    BrightWhite,
}

impl ColorSpec {
    /// Converts this color spec to an ANSI 256-color index (0-255).
    ///
    /// # Examples
    ///
    /// ```
    /// use omega_core::color::{ColorSpec, AnsiColor};
    ///
    /// let red = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
    /// assert_eq!(red.to_ansi256(), 196); // Bright red in 256-color palette
    /// ```
    pub fn to_ansi256(&self) -> u8 {
        match self {
            ColorSpec::Rgb { r, g, b } => {
                // Convert RGB to ANSI 256 color index
                // Use standard algorithm for RGB to 256-color conversion
                if *r == *g && *g == *b {
                    // Grayscale
                    if *r < 8 {
                        232
                    } else if *r > 248 {
                        255
                    } else {
                        232 + ((*r - 8) / 10)
                    }
                } else {
                    // Color cube
                    let r = (6 * *r as u16 / 256) as u8;
                    let g = (6 * *g as u16 / 256) as u8;
                    let b = (6 * *b as u16 / 256) as u8;
                    16 + 36 * r + 6 * g + b
                }
            }
            ColorSpec::Indexed(idx) => *idx,
            ColorSpec::Ansi(ansi) => ansi.to_ansi256(),
        }
    }

    /// Converts this color spec to an ANSI 16-color value.
    ///
    /// # Examples
    ///
    /// ```
    /// use omega_core::color::{ColorSpec, AnsiColor};
    ///
    /// let red = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
    /// assert_eq!(red.to_ansi16(), AnsiColor::BrightRed);
    /// ```
    pub fn to_ansi16(&self) -> AnsiColor {
        match self {
            ColorSpec::Rgb { r, g, b } => {
                // Convert to ANSI 16 by finding closest match
                let idx = self.to_ansi256();
                if idx < 16 {
                    AnsiColor::from_index(idx)
                } else {
                    // Map 256 colors to 16
                    AnsiColor::from_rgb(*r, *g, *b)
                }
            }
            ColorSpec::Indexed(idx) => AnsiColor::from_index(*idx % 16),
            ColorSpec::Ansi(ansi) => *ansi,
        }
    }
}

impl AnsiColor {
    /// Converts this ANSI color to its 256-color palette index.
    ///
    /// # Examples
    ///
    /// ```
    /// use omega_core::color::AnsiColor;
    ///
    /// assert_eq!(AnsiColor::Red.to_ansi256(), 1);
    /// assert_eq!(AnsiColor::BrightRed.to_ansi256(), 9);
    /// ```
    pub fn to_ansi256(&self) -> u8 {
        match self {
            AnsiColor::Black => 0,
            AnsiColor::Red => 1,
            AnsiColor::Green => 2,
            AnsiColor::Yellow => 3,
            AnsiColor::Blue => 4,
            AnsiColor::Magenta => 5,
            AnsiColor::Cyan => 6,
            AnsiColor::White => 7,
            AnsiColor::BrightBlack => 8,
            AnsiColor::BrightRed => 9,
            AnsiColor::BrightGreen => 10,
            AnsiColor::BrightYellow => 11,
            AnsiColor::BrightBlue => 12,
            AnsiColor::BrightMagenta => 13,
            AnsiColor::BrightCyan => 14,
            AnsiColor::BrightWhite => 15,
        }
    }

    /// Creates an ANSI color from a 0-15 index.
    ///
    /// # Examples
    ///
    /// ```
    /// use omega_core::color::AnsiColor;
    ///
    /// assert_eq!(AnsiColor::from_index(1), AnsiColor::Red);
    /// assert_eq!(AnsiColor::from_index(9), AnsiColor::BrightRed);
    /// ```
    pub fn from_index(idx: u8) -> Self {
        match idx % 16 {
            0 => AnsiColor::Black,
            1 => AnsiColor::Red,
            2 => AnsiColor::Green,
            3 => AnsiColor::Yellow,
            4 => AnsiColor::Blue,
            5 => AnsiColor::Magenta,
            6 => AnsiColor::Cyan,
            7 => AnsiColor::White,
            8 => AnsiColor::BrightBlack,
            9 => AnsiColor::BrightRed,
            10 => AnsiColor::BrightGreen,
            11 => AnsiColor::BrightYellow,
            12 => AnsiColor::BrightBlue,
            13 => AnsiColor::BrightMagenta,
            14 => AnsiColor::BrightCyan,
            15 => AnsiColor::BrightWhite,
            _ => unreachable!(),
        }
    }

    /// Creates an ANSI color from RGB values by finding the closest match.
    ///
    /// Uses a simple algorithm based on the highest component value.
    ///
    /// # Examples
    ///
    /// ```
    /// use omega_core::color::AnsiColor;
    ///
    /// assert_eq!(AnsiColor::from_rgb(255, 0, 0), AnsiColor::BrightRed);
    /// assert_eq!(AnsiColor::from_rgb(128, 0, 0), AnsiColor::Red);
    /// ```
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        // Simple algorithm: use highest component
        let max = r.max(g).max(b);
        if max < 64 {
            return AnsiColor::Black;
        }

        let bright = max > 192;
        let idx = if r > 200 && g > 200 && b > 200 {
            7 // White
        } else if r > g && r > b {
            1 // Red
        } else if g > r && g > b {
            2 // Green
        } else if b > r && b > g {
            4 // Blue
        } else if r > 200 && g > 200 {
            3 // Yellow
        } else if r > 200 && b > 200 {
            5 // Magenta
        } else if g > 200 && b > 200 {
            6 // Cyan
        } else {
            7 // White
        };

        AnsiColor::from_index(if bright { idx + 8 } else { idx })
    }
}

impl Default for ColorSpec {
    fn default() -> Self {
        ColorSpec::Rgb { r: 255, g: 255, b: 255 }
    }
}

impl From<HexColor> for ColorSpec {
    fn from(hex: HexColor) -> Self {
        let (r, g, b) = hex.to_rgb();
        ColorSpec::Rgb { r, g, b }
    }
}

impl From<AnsiColor> for ColorSpec {
    fn from(ansi: AnsiColor) -> Self {
        ColorSpec::Ansi(ansi)
    }
}

impl From<u8> for ColorSpec {
    /// Creates an indexed color from a u8 value.
    fn from(idx: u8) -> Self {
        ColorSpec::Indexed(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_ansi256_red() {
        let red = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
        assert_eq!(red.to_ansi256(), 196);
    }

    #[test]
    fn test_rgb_to_ansi256_white() {
        let white = ColorSpec::Rgb { r: 255, g: 255, b: 255 };
        assert_eq!(white.to_ansi256(), 255);
    }

    #[test]
    fn test_ansi_red_to_ansi256() {
        let red = ColorSpec::Ansi(AnsiColor::Red);
        // AnsiColor::Red maps to index 1, not 196
        assert_eq!(red.to_ansi256(), 1);
    }

    #[test]
    fn test_ansi_bright_red_to_ansi256() {
        let bright_red = ColorSpec::Ansi(AnsiColor::BrightRed);
        // AnsiColor::BrightRed maps to index 9
        assert_eq!(bright_red.to_ansi256(), 9);
    }

    #[test]
    fn test_indexed_to_ansi256() {
        let indexed = ColorSpec::Indexed(100);
        assert_eq!(indexed.to_ansi256(), 100);
    }

    #[test]
    fn test_default() {
        let default = ColorSpec::default();
        assert_eq!(default, ColorSpec::Rgb { r: 255, g: 255, b: 255 });
    }

    #[test]
    fn test_from_hexcolor() {
        let hex = HexColor::from_rgb(255, 0, 0);
        let spec: ColorSpec = hex.into();
        assert_eq!(spec, ColorSpec::Rgb { r: 255, g: 0, b: 0 });
    }

    #[test]
    fn test_from_ansi() {
        let ansi = AnsiColor::Red;
        let spec: ColorSpec = ansi.into();
        assert_eq!(spec, ColorSpec::Ansi(AnsiColor::Red));
    }

    #[test]
    fn test_ansi_to_ansi256() {
        assert_eq!(AnsiColor::Black.to_ansi256(), 0);
        assert_eq!(AnsiColor::Red.to_ansi256(), 1);
        assert_eq!(AnsiColor::Green.to_ansi256(), 2);
        assert_eq!(AnsiColor::Yellow.to_ansi256(), 3);
        assert_eq!(AnsiColor::Blue.to_ansi256(), 4);
        assert_eq!(AnsiColor::Magenta.to_ansi256(), 5);
        assert_eq!(AnsiColor::Cyan.to_ansi256(), 6);
        assert_eq!(AnsiColor::White.to_ansi256(), 7);
        assert_eq!(AnsiColor::BrightBlack.to_ansi256(), 8);
        assert_eq!(AnsiColor::BrightRed.to_ansi256(), 9);
        assert_eq!(AnsiColor::BrightGreen.to_ansi256(), 10);
        assert_eq!(AnsiColor::BrightYellow.to_ansi256(), 11);
        assert_eq!(AnsiColor::BrightBlue.to_ansi256(), 12);
        assert_eq!(AnsiColor::BrightMagenta.to_ansi256(), 13);
        assert_eq!(AnsiColor::BrightCyan.to_ansi256(), 14);
        assert_eq!(AnsiColor::BrightWhite.to_ansi256(), 15);
    }

    #[test]
    fn test_from_index() {
        assert_eq!(AnsiColor::from_index(0), AnsiColor::Black);
        assert_eq!(AnsiColor::from_index(1), AnsiColor::Red);
        assert_eq!(AnsiColor::from_index(9), AnsiColor::BrightRed);
        assert_eq!(AnsiColor::from_index(16), AnsiColor::Black); // wraps around
    }

    #[test]
    fn test_ansi_from_rgb() {
        assert_eq!(AnsiColor::from_rgb(255, 0, 0), AnsiColor::BrightRed);
        assert_eq!(AnsiColor::from_rgb(128, 0, 0), AnsiColor::Red);
        assert_eq!(AnsiColor::from_rgb(0, 255, 0), AnsiColor::BrightGreen);
        assert_eq!(AnsiColor::from_rgb(0, 0, 255), AnsiColor::BrightBlue);
        assert_eq!(AnsiColor::from_rgb(255, 255, 0), AnsiColor::BrightYellow);
        assert_eq!(AnsiColor::from_rgb(0, 0, 0), AnsiColor::Black);
    }

    #[test]
    fn test_rgb_to_ansi16() {
        let red = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
        assert_eq!(red.to_ansi16(), AnsiColor::BrightRed);

        let indexed = ColorSpec::Indexed(1);
        assert_eq!(indexed.to_ansi16(), AnsiColor::Red);

        let ansi = ColorSpec::Ansi(AnsiColor::Blue);
        assert_eq!(ansi.to_ansi16(), AnsiColor::Blue);
    }

    #[test]
    fn test_serialization() {
        let red = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
        let json = serde_json::to_string(&red).unwrap();
        assert!(json.contains("Rgb"));

        let ansi = ColorSpec::Ansi(AnsiColor::Red);
        let json = serde_json::to_string(&ansi).unwrap();
        assert!(json.contains("Ansi"));
        assert!(json.contains("Red"));

        let indexed = ColorSpec::Indexed(100);
        let json = serde_json::to_string(&indexed).unwrap();
        assert!(json.contains("Indexed"));
    }
}
