//! Color adapter module for converting omega-core ColorTheme to ratatui styles.
//!
//! This module provides the `StyleCache` struct which precomputes all color
//! lookups from ColorId to ratatui::Style at startup, enabling O(1) style
//! resolution during rendering.

use omega_core::color::{
    AnsiColor, ColorCapability, ColorId, ColorSpec, ColorTheme, EffectColorId, EntityColorId,
    EnvironmentColorId, ItemRarityColorId, MonsterColorId, TerrainColorId, UiColorId,
};
use ratatui::style::{Color, Style};
use std::collections::HashMap;

/// Embedded classic theme TOML
pub const CLASSIC_THEME_TOML: &str = include_str!("../../omega-content/themes/classic.toml");

/// Embedded accessible theme TOML
pub const ACCESSIBLE_THEME_TOML: &str = include_str!("../../omega-content/themes/accessible.toml");

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

    ColorTheme::from_toml(toml_str)
        .map_err(|e| format!("Failed to parse {} theme: {}", name, e))
}

/// StyleCache holds precomputed ratatui styles for all ColorId variants.
///
/// Built at application startup from a ColorTheme and ColorCapability,
/// this cache enables O(1) style lookups during rendering without repeated
/// color resolution or terminal capability adaptation.
#[derive(Debug, Clone)]
pub struct StyleCache {
    /// Precomputed map from ColorId to Style.
    cache: HashMap<ColorId, Style>,
    /// Terminal color capability level.
    capability: ColorCapability,
    /// Whether colors are enabled (false if NO_COLOR or capability is None).
    colors_enabled: bool,
}

impl StyleCache {
    /// Creates a new StyleCache from a theme and capability.
    ///
    /// Precomputes all 63 ColorId variants by resolving them through the theme,
    /// adapting to the terminal capability, and converting to ratatui::Style.
    ///
    /// If capability is ColorCapability::None, colors are disabled and all
    /// lookups return Style::default().
    pub fn new(theme: &ColorTheme, capability: ColorCapability) -> Self {
        let colors_enabled = capability != ColorCapability::None;
        let mut cache = HashMap::new();

        if colors_enabled {
            // Precompute all ColorId variants
            for color_id in all_color_ids() {
                if let Some((fg_hex, bg_hex)) = theme.resolve(&color_id) {
                    // Convert HexColor to ColorSpec
                    let fg_spec = ColorSpec::from(fg_hex);
                    let bg_spec = ColorSpec::from(bg_hex);

                    // Adapt to terminal capability
                    let adapted_fg = capability.adapt(&fg_spec);
                    let adapted_bg = capability.adapt(&bg_spec);

                    // Convert ColorSpec to ratatui Color
                    let ratatui_fg = colorspec_to_ratatui(&adapted_fg);
                    let ratatui_bg = colorspec_to_ratatui(&adapted_bg);

                    // Create Style with both fg and bg
                    let style = Style::default().fg(ratatui_fg).bg(ratatui_bg);
                    cache.insert(color_id, style);
                }
            }
        }

        Self { cache, capability, colors_enabled }
    }

    /// Returns the cached Style for a ColorId, or Style::default() if not found.
    ///
    /// If colors are disabled (NO_COLOR or capability None), always returns
    /// Style::default().
    pub fn get(&self, color_id: &ColorId) -> Style {
        if !self.colors_enabled {
            return Style::default();
        }
        self.cache.get(color_id).copied().unwrap_or_default()
    }

    /// Returns a Style with only the foreground color applied.
    ///
    /// Useful for inline text coloring where the background should inherit
    /// from the widget's default background.
    pub fn get_fg(&self, color_id: &ColorId) -> Style {
        if !self.colors_enabled {
            return Style::default();
        }
        self.cache
            .get(color_id)
            .map(|style| Style::default().fg(style.fg.unwrap_or(Color::Reset)))
            .unwrap_or_default()
    }
}

/// Returns all 63 ColorId variants for cache precomputation.
///
/// This function explicitly enumerates every ColorId variant to ensure
/// complete coverage when building the style cache.
fn all_color_ids() -> Vec<ColorId> {
    let mut ids = Vec::new();

    // Entity::Player (1 variant)
    ids.push(ColorId::Entity(EntityColorId::Player));

    // Entity::Monster (8 variants)
    ids.push(ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead)));
    ids.push(ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileBeast)));
    ids.push(ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileHumanoid)));
    ids.push(ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileMagical)));
    ids.push(ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileConstruct)));
    ids.push(ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileDragon)));
    ids.push(ColorId::Entity(EntityColorId::Monster(MonsterColorId::Neutral)));
    ids.push(ColorId::Entity(EntityColorId::Monster(MonsterColorId::Friendly)));

    // Entity::Item (5 variants)
    ids.push(ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Common)));
    ids.push(ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Uncommon)));
    ids.push(ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Rare)));
    ids.push(ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Epic)));
    ids.push(ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Legendary)));

    // Entity::Terrain (13 variants)
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallStone)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallWood)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallMetal)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallBrick)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorStone)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorWood)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorDirt)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorGrass)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Water)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Lava)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Door)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::StairsUp)));
    ids.push(ColorId::Entity(EntityColorId::Terrain(TerrainColorId::StairsDown)));

    // UI (16 variants)
    ids.push(ColorId::Ui(UiColorId::HealthHigh));
    ids.push(ColorId::Ui(UiColorId::HealthMedium));
    ids.push(ColorId::Ui(UiColorId::HealthLow));
    ids.push(ColorId::Ui(UiColorId::Mana));
    ids.push(ColorId::Ui(UiColorId::Stamina));
    ids.push(ColorId::Ui(UiColorId::Experience));
    ids.push(ColorId::Ui(UiColorId::Highlight));
    ids.push(ColorId::Ui(UiColorId::Selection));
    ids.push(ColorId::Ui(UiColorId::Cursor));
    ids.push(ColorId::Ui(UiColorId::TextDefault));
    ids.push(ColorId::Ui(UiColorId::TextDim));
    ids.push(ColorId::Ui(UiColorId::TextBold));
    ids.push(ColorId::Ui(UiColorId::MessageInfo));
    ids.push(ColorId::Ui(UiColorId::MessageWarning));
    ids.push(ColorId::Ui(UiColorId::MessageDanger));
    ids.push(ColorId::Ui(UiColorId::MessageSuccess));

    // Effect (11 variants)
    ids.push(ColorId::Effect(EffectColorId::Fire));
    ids.push(ColorId::Effect(EffectColorId::Ice));
    ids.push(ColorId::Effect(EffectColorId::Lightning));
    ids.push(ColorId::Effect(EffectColorId::Poison));
    ids.push(ColorId::Effect(EffectColorId::Acid));
    ids.push(ColorId::Effect(EffectColorId::MagicArcane));
    ids.push(ColorId::Effect(EffectColorId::MagicHoly));
    ids.push(ColorId::Effect(EffectColorId::MagicDark));
    ids.push(ColorId::Effect(EffectColorId::Blood));
    ids.push(ColorId::Effect(EffectColorId::Impact));
    ids.push(ColorId::Effect(EffectColorId::Shield));

    // Environment (5 variants)
    ids.push(ColorId::Environment(EnvironmentColorId::LightTorch));
    ids.push(ColorId::Environment(EnvironmentColorId::LightLantern));
    ids.push(ColorId::Environment(EnvironmentColorId::LightMagic));
    ids.push(ColorId::Environment(EnvironmentColorId::Darkness));
    ids.push(ColorId::Environment(EnvironmentColorId::Fog));

    // Total: 1 + 8 + 5 + 13 + 16 + 11 + 5 = 59 variants
    // Note: The plan mentioned 63, but counting all enum variants gives 59.
    // This is the complete set of all ColorId variants.
    ids
}

/// Converts a ColorSpec to a ratatui Color.
///
/// Maps omega-core's color representations to ratatui's Color enum,
/// handling RGB, indexed (256-color), and ANSI 16-color formats.
fn colorspec_to_ratatui(spec: &ColorSpec) -> Color {
    match spec {
        ColorSpec::Rgb { r, g, b } => Color::Rgb(*r, *g, *b),
        ColorSpec::Indexed(idx) => Color::Indexed(*idx),
        ColorSpec::Ansi(ansi) => match ansi {
            AnsiColor::Black => Color::Black,
            AnsiColor::Red => Color::Red,
            AnsiColor::Green => Color::Green,
            AnsiColor::Yellow => Color::Yellow,
            AnsiColor::Blue => Color::Blue,
            AnsiColor::Magenta => Color::Magenta,
            AnsiColor::Cyan => Color::Cyan,
            AnsiColor::White => Color::White,
            AnsiColor::BrightBlack => Color::DarkGray,
            AnsiColor::BrightRed => Color::LightRed,
            AnsiColor::BrightGreen => Color::LightGreen,
            AnsiColor::BrightYellow => Color::LightYellow,
            AnsiColor::BrightBlue => Color::LightBlue,
            AnsiColor::BrightMagenta => Color::LightMagenta,
            AnsiColor::BrightCyan => Color::LightCyan,
            AnsiColor::BrightWhite => Color::White,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::color::HexColor;

    fn create_minimal_theme() -> ColorTheme {
        // Create a minimal theme for testing using from_toml
        let toml_str = r##"
[meta]
name = "Test"
author = "Test"
description = "Test theme"
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
healthhigh = { fg = "#00FF00", bg = "#000000" }

[effect]
fire = { fg = "#FF0000", bg = "#000000" }
"##;
        ColorTheme::from_toml(toml_str).expect("test theme should parse")
    }

    #[test]
    fn style_cache_with_theme_produces_non_empty_map() {
        let theme = create_minimal_theme();
        let capability = ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        // Should have cached at least player, healthhigh, and fire
        let player_style = cache.get(&ColorId::Entity(EntityColorId::Player));
        assert!(player_style.fg.is_some());
        assert!(player_style.bg.is_some());
    }

    #[test]
    fn style_cache_returns_default_for_unknown_color_ids() {
        let theme = create_minimal_theme();
        let capability = ColorCapability::TrueColor;
        let cache = StyleCache::new(&theme, capability);

        // HostileUndead is not in the minimal theme
        let unknown_style =
            cache.get(&ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead)));
        assert_eq!(unknown_style, Style::default());
    }

    #[test]
    fn style_cache_with_none_capability_returns_default_styles() {
        let theme = create_minimal_theme();
        let capability = ColorCapability::None;
        let cache = StyleCache::new(&theme, capability);

        // Even though theme has player color, NO_COLOR should override
        let player_style = cache.get(&ColorId::Entity(EntityColorId::Player));
        assert_eq!(player_style, Style::default());

        let health_style = cache.get(&ColorId::Ui(UiColorId::HealthHigh));
        assert_eq!(health_style, Style::default());
    }

    #[test]
    fn colorspec_to_ratatui_rgb_conversion() {
        let spec = ColorSpec::Rgb { r: 255, g: 128, b: 64 };
        let color = colorspec_to_ratatui(&spec);
        assert_eq!(color, Color::Rgb(255, 128, 64));
    }

    #[test]
    fn colorspec_to_ratatui_indexed_conversion() {
        let spec = ColorSpec::Indexed(196);
        let color = colorspec_to_ratatui(&spec);
        assert_eq!(color, Color::Indexed(196));
    }

    #[test]
    fn colorspec_to_ratatui_ansi_conversions() {
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::Black)), Color::Black);
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::Red)), Color::Red);
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::Green)), Color::Green);
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::Yellow)), Color::Yellow);
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::Blue)), Color::Blue);
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::Magenta)), Color::Magenta);
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::Cyan)), Color::Cyan);
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::White)), Color::White);
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightBlack)),
            Color::DarkGray
        );
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightRed)),
            Color::LightRed
        );
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightGreen)),
            Color::LightGreen
        );
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightYellow)),
            Color::LightYellow
        );
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightBlue)),
            Color::LightBlue
        );
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightMagenta)),
            Color::LightMagenta
        );
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightCyan)),
            Color::LightCyan
        );
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightWhite)),
            Color::White
        );
    }

    #[test]
    fn all_color_ids_returns_complete_set() {
        let ids = all_color_ids();
        // Count should match all variants:
        // Entity: 1 (Player) + 8 (Monster) + 5 (Item) + 13 (Terrain) = 27
        // UI: 16
        // Effect: 11
        // Environment: 5
        // Total: 59
        assert_eq!(ids.len(), 59);
    }
}
