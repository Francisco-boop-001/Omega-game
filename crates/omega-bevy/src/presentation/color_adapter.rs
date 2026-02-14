//! Color adapter module for converting omega-core ColorTheme to Bevy colors.
//!
//! This module provides functions to convert omega-core color types to Bevy's
//! Color representation, enabling semantic color support in the Bevy frontend.

use bevy::prelude::Color;
use omega_core::color::{ColorId, ColorTheme, HexColor};

/// Embedded classic theme TOML
pub const CLASSIC_THEME_TOML: &str = include_str!("../../../omega-content/themes/classic.toml");

/// Embedded accessible theme TOML
pub const ACCESSIBLE_THEME_TOML: &str =
    include_str!("../../../omega-content/themes/accessible.toml");

/// Loads a built-in theme by name.
///
/// Supports "classic" and "accessible" themes.
/// Returns an error if the theme name is unknown or the theme fails to parse.
pub fn load_builtin_theme(name: &str) -> Result<ColorTheme, String> {
    let toml_str = match name.to_lowercase().as_str() {
        "classic" => CLASSIC_THEME_TOML,
        "accessible" => ACCESSIBLE_THEME_TOML,
        _ => return Err(format!("Unknown theme '{}'. Available: classic, accessible", name)),
    };

    ColorTheme::from_toml(toml_str).map_err(|e| format!("Failed to parse {} theme: {}", name, e))
}

/// Converts a HexColor to a Bevy Color using sRGB color space.
///
/// Maps omega-core's HexColor (u8 RGB components) to Bevy's Color::srgb
/// which expects f32 values in the range [0.0, 1.0].
///
/// # Color Space Handling
///
/// This function uses `Color::srgb()` which is correct for both UI and sprite rendering
/// in Bevy. Bevy internally converts sRGB to linear RGB when needed for physically-based
/// rendering. The sRGB color space is the standard for authored colors (themes, hex values)
/// and ensures consistent appearance across different rendering contexts.
///
/// **UI Rendering**: Text, backgrounds, borders use sRGB colors directly.
/// **Sprite Rendering**: Sprite tints use sRGB colors, Bevy converts to linear for lighting.
///
/// # Examples
///
/// ```ignore
/// use omega_core::color::HexColor;
/// use omega_bevy::presentation::color_adapter::to_bevy_color;
///
/// let hex = HexColor::new(255, 0, 0).unwrap(); // Red
/// let color = to_bevy_color(&hex);
/// // color is approximately Color::srgb(1.0, 0.0, 0.0)
/// ```
pub fn to_bevy_color(hex: &HexColor) -> Color {
    let (r, g, b) = hex.to_rgb();
    Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

/// Resolves a ColorId through a ColorTheme and converts to a Bevy Color.
///
/// This function:
/// 1. Resolves the ColorId to a (foreground, background) HexColor pair
/// 2. Converts the foreground color to Bevy's Color format
///
/// Returns the foreground color only, as most Bevy rendering uses
/// foreground colors for sprites and text.
///
/// If the ColorId cannot be resolved, returns a default white color.
///
/// # Examples
///
/// ```ignore
/// use omega_core::color::{ColorId, EntityColorId, ColorTheme};
/// use omega_bevy::presentation::color_adapter::resolve_to_bevy_color;
///
/// let theme = ColorTheme::from_toml(toml_str).unwrap();
/// let color_id = ColorId::Entity(EntityColorId::Player);
/// let color = resolve_to_bevy_color(&theme, &color_id);
/// ```
pub fn resolve_to_bevy_color(theme: &ColorTheme, id: &ColorId) -> Color {
    match theme.resolve(id) {
        Some((fg, _bg)) => to_bevy_color(&fg),
        None => Color::srgb(1.0, 1.0, 1.0), // Default to white
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::color::HexColor;

    #[test]
    fn to_bevy_color_converts_pure_red() {
        let hex = HexColor::from_rgb(255, 0, 0);
        let color = to_bevy_color(&hex);
        let srgba = color.to_srgba();
        assert_eq!(srgba.red, 1.0);
        assert_eq!(srgba.green, 0.0);
        assert_eq!(srgba.blue, 0.0);
    }

    #[test]
    fn to_bevy_color_converts_pure_green() {
        let hex = HexColor::from_rgb(0, 255, 0);
        let color = to_bevy_color(&hex);
        let srgba = color.to_srgba();
        assert_eq!(srgba.red, 0.0);
        assert_eq!(srgba.green, 1.0);
        assert_eq!(srgba.blue, 0.0);
    }

    #[test]
    fn to_bevy_color_converts_pure_blue() {
        let hex = HexColor::from_rgb(0, 0, 255);
        let color = to_bevy_color(&hex);
        let srgba = color.to_srgba();
        assert_eq!(srgba.red, 0.0);
        assert_eq!(srgba.green, 0.0);
        assert_eq!(srgba.blue, 1.0);
    }

    #[test]
    fn to_bevy_color_converts_white() {
        let hex = HexColor::from_rgb(255, 255, 255);
        let color = to_bevy_color(&hex);
        let srgba = color.to_srgba();
        assert_eq!(srgba.red, 1.0);
        assert_eq!(srgba.green, 1.0);
        assert_eq!(srgba.blue, 1.0);
    }

    #[test]
    fn to_bevy_color_converts_black() {
        let hex = HexColor::from_rgb(0, 0, 0);
        let color = to_bevy_color(&hex);
        let srgba = color.to_srgba();
        assert_eq!(srgba.red, 0.0);
        assert_eq!(srgba.green, 0.0);
        assert_eq!(srgba.blue, 0.0);
    }

    #[test]
    fn to_bevy_color_converts_gray() {
        let hex = HexColor::from_rgb(128, 128, 128);
        let color = to_bevy_color(&hex);
        let srgba = color.to_srgba();
        // 128/255 ≈ 0.502
        assert!((srgba.red - 0.502).abs() < 0.01);
        assert!((srgba.green - 0.502).abs() < 0.01);
        assert!((srgba.blue - 0.502).abs() < 0.01);
    }

    #[test]
    fn load_builtin_classic_theme() {
        let theme = load_builtin_theme("classic").expect("classic theme should load");
        assert_eq!(theme.meta.name, "Classic");
    }

    #[test]
    fn load_builtin_accessible_theme() {
        let theme = load_builtin_theme("accessible").expect("accessible theme should load");
        assert_eq!(theme.meta.name, "Accessible");
    }

    #[test]
    fn load_builtin_theme_case_insensitive() {
        assert!(load_builtin_theme("CLASSIC").is_ok());
        assert!(load_builtin_theme("Accessible").is_ok());
    }

    #[test]
    fn load_builtin_theme_unknown_returns_error() {
        let result = load_builtin_theme("nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown theme"));
    }

    #[test]
    fn resolve_to_bevy_color_with_classic_theme() {
        use omega_core::color::{ColorId, EntityColorId};

        let theme = load_builtin_theme("classic").expect("classic theme should load");
        let color_id = ColorId::Entity(EntityColorId::Player);
        let color = resolve_to_bevy_color(&theme, &color_id);

        // Player should resolve to a valid color (not default white)
        let srgba = color.to_srgba();
        assert!(srgba.red >= 0.0 && srgba.red <= 1.0);
        assert!(srgba.green >= 0.0 && srgba.green <= 1.0);
        assert!(srgba.blue >= 0.0 && srgba.blue <= 1.0);
    }

    #[test]
    fn resolve_to_bevy_color_fallback_for_missing_id() {
        // Create a minimal theme that doesn't define all colors
        let minimal_toml = r##"
[meta]
name = "Minimal"
author = "Test"
description = "Test"
version = "1.0.0"
variant = "dark"
min_engine_version = "0.1.0"

[base]
red = "#FF0000"
green = "#00FF00"
blue = "#0000FF"
yellow = "#FFFF00"
cyan = "#00FFFF"
magenta = "#FF00FF"
white = "#FFFFFF"
black = "#000000"
gray = "#808080"

[semantic]
danger = { ref = "base.red" }
success = { ref = "base.green" }
info = { ref = "base.blue" }
warning = { ref = "base.yellow" }
magic = { ref = "base.magenta" }
neutral = { ref = "base.gray" }

[entity]
player = { fg = "#FFFFFF", bg = "#000000" }

[ui]
[effect]
"##;
        let theme = ColorTheme::from_toml(minimal_toml).expect("minimal theme should parse");

        // Try to resolve a color not in the theme
        use omega_core::color::{ColorId, EntityColorId, MonsterColorId};
        let color_id = ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead));
        let color = resolve_to_bevy_color(&theme, &color_id);

        // Should return default white
        let srgba = color.to_srgba();
        assert_eq!(srgba.red, 1.0);
        assert_eq!(srgba.green, 1.0);
        assert_eq!(srgba.blue, 1.0);
    }
}
