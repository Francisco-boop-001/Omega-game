//! Hex color newtype with validation.
//!
//! Provides a type-safe wrapper for hexadecimal color values with
//! validation, serialization support, and conversion utilities.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// A validated hexadecimal color value.
///
/// Stores colors as a u32 containing the RGB components.
/// Only accepts valid 6-digit hex colors (with or without # prefix).
///
/// # Example
///
/// ```rust
/// use omega_core::color::HexColor;
///
/// // Parse from hex string
/// let color = HexColor::from_hex("#FF5733").unwrap();
/// assert_eq!(color.to_string(), "#FF5733");
///
/// // Convert to RGB
/// let (r, g, b) = color.to_rgb();
/// assert_eq!((r, g, b), (255, 87, 51));
///
/// // Create from RGB
/// let color2 = HexColor::from_rgb(255, 87, 51);
/// assert_eq!(color, color2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct HexColor(u32);

/// Errors that can occur when parsing hex colors.
#[derive(Error, Debug)]
pub enum HexColorError {
    /// The hex string has an invalid length.
    #[error("Invalid hex color length: expected 6 or 7 characters (with #), got {0}")]
    InvalidLength(usize),
    /// The hex string contains non-hexadecimal characters.
    #[error("Invalid hex color format: '{0}' contains non-hexadecimal characters")]
    InvalidFormat(String),
}

impl HexColor {
    /// Parses a hex color string.
    ///
    /// Accepts both formats:
    /// - With `#` prefix: `"#FF5733"`
    /// - Without `#` prefix: `"FF5733"`
    ///
    /// # Errors
    ///
    /// Returns `HexColorError::InvalidLength` if the string is not 6 hex digits
    /// (or 7 with # prefix).
    ///
    /// Returns `HexColorError::InvalidFormat` if non-hex characters are present.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::HexColor;
    ///
    /// let color = HexColor::from_hex("#FF5733").unwrap();
    /// assert_eq!(color.to_rgb(), (255, 87, 51));
    ///
    /// // Without # prefix also works
    /// let color2 = HexColor::from_hex("FF5733").unwrap();
    /// assert_eq!(color, color2);
    /// ```
    pub fn from_hex(s: &str) -> Result<Self, HexColorError> {
        // Remove # prefix if present
        let hex = s.strip_prefix('#').unwrap_or(s);

        // Validate length (must be 6 hex digits)
        if hex.len() != 6 {
            return Err(HexColorError::InvalidLength(s.len()));
        }

        // Parse hex
        match u32::from_str_radix(hex, 16) {
            Ok(value) => Ok(HexColor(value)),
            Err(_) => Err(HexColorError::InvalidFormat(s.to_string())),
        }
    }

    /// Converts the hex color to RGB components.
    ///
    /// Returns a tuple of `(red, green, blue)` where each component
    /// is in the range 0-255.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::HexColor;
    ///
    /// let color = HexColor::from_hex("#00FF80").unwrap();
    /// let (r, g, b) = color.to_rgb();
    /// assert_eq!((r, g, b), (0, 255, 128));
    /// ```
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        let r = ((self.0 >> 16) & 0xFF) as u8;
        let g = ((self.0 >> 8) & 0xFF) as u8;
        let b = (self.0 & 0xFF) as u8;
        (r, g, b)
    }

    /// Creates a hex color from RGB components.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::HexColor;
    ///
    /// let color = HexColor::from_rgb(255, 0, 128);
    /// assert_eq!(color.to_string(), "#FF0080");
    /// ```
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let value = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        HexColor(value)
    }
}

impl fmt::Display for HexColor {
    /// Formats the color as a hex string with # prefix.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::HexColor;
    ///
    /// let color = HexColor::from_rgb(255, 87, 51);
    /// assert_eq!(color.to_string(), "#FF5733");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:06X}", self.0)
    }
}

impl TryFrom<String> for HexColor {
    type Error = HexColorError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        HexColor::from_hex(&s)
    }
}

impl From<HexColor> for String {
    fn from(color: HexColor) -> Self {
        color.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex_with_hash() {
        let color = HexColor::from_hex("#FF5733").unwrap();
        assert_eq!(color.to_rgb(), (255, 87, 51));
    }

    #[test]
    fn test_from_hex_without_hash() {
        let color = HexColor::from_hex("FF5733").unwrap();
        assert_eq!(color.to_rgb(), (255, 87, 51));
    }

    #[test]
    fn test_from_hex_invalid_length() {
        // 3 digits (shorthand) should fail
        let result = HexColor::from_hex("FFF");
        assert!(matches!(result, Err(HexColorError::InvalidLength(3))));

        // 8 digits should fail
        let result = HexColor::from_hex("AABBCCDD");
        assert!(matches!(result, Err(HexColorError::InvalidLength(8))));

        // 5 digits should fail
        let result = HexColor::from_hex("#AABBC");
        assert!(matches!(result, Err(HexColorError::InvalidLength(6))));
    }

    #[test]
    fn test_from_hex_invalid_format() {
        let result = HexColor::from_hex("#GGGGGG");
        assert!(matches!(result, Err(HexColorError::InvalidFormat(_))));

        let result = HexColor::from_hex("not-a-color");
        assert!(matches!(result, Err(HexColorError::InvalidLength(_))));
    }

    #[test]
    fn test_to_rgb() {
        let color = HexColor::from_hex("#00FF80").unwrap();
        assert_eq!(color.to_rgb(), (0, 255, 128));

        let color = HexColor::from_hex("#000000").unwrap();
        assert_eq!(color.to_rgb(), (0, 0, 0));

        let color = HexColor::from_hex("#FFFFFF").unwrap();
        assert_eq!(color.to_rgb(), (255, 255, 255));
    }

    #[test]
    fn test_from_rgb() {
        let color = HexColor::from_rgb(255, 87, 51);
        assert_eq!(color.to_string(), "#FF5733");

        let color = HexColor::from_rgb(0, 0, 0);
        assert_eq!(color.to_string(), "#000000");

        let color = HexColor::from_rgb(255, 255, 255);
        assert_eq!(color.to_string(), "#FFFFFF");
    }

    #[test]
    fn test_display() {
        let color = HexColor::from_rgb(255, 0, 128);
        assert_eq!(format!("{}", color), "#FF0080");
    }

    #[test]
    fn test_serde_roundtrip() {
        let color = HexColor::from_hex("#FF5733").unwrap();

        // Serialize to string
        let json = serde_json::to_string(&color).unwrap();
        assert_eq!(json, "\"#FF5733\"");

        // Deserialize back
        let deserialized: HexColor = serde_json::from_str(&json).unwrap();
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_serde_deserialize_from_string() {
        // Can deserialize from string value
        let color: HexColor = serde_json::from_str("\"#00FF00\"").unwrap();
        assert_eq!(color.to_rgb(), (0, 255, 0));

        // Without # prefix also works
        let color: HexColor = serde_json::from_str("\"0000FF\"").unwrap();
        assert_eq!(color.to_rgb(), (0, 0, 255));
    }

    #[test]
    fn test_serde_invalid_format() {
        let result: Result<HexColor, _> = serde_json::from_str("\"#GGGGGG\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_string() {
        let result = HexColor::try_from("#FF5733".to_string());
        assert!(result.is_ok());

        let result = HexColor::try_from("invalid".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_from_string() {
        let color = HexColor::from_rgb(128, 64, 32);
        let s: String = color.into();
        assert_eq!(s, "#804020");
    }

    #[test]
    fn test_equality_and_hash() {
        let c1 = HexColor::from_hex("#FF5733").unwrap();
        let c2 = HexColor::from_hex("FF5733").unwrap();
        let c3 = HexColor::from_hex("#00FF00").unwrap();

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);

        // Both should have same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        c1.hash(&mut h1);
        c2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_clone_and_copy() {
        let c1 = HexColor::from_hex("#FF5733").unwrap();
        let c2 = c1; // Copy
        let c3 = c1;

        assert_eq!(c1, c2);
        assert_eq!(c1, c3);
    }
}
