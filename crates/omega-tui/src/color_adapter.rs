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

    ColorTheme::from_toml(toml_str).map_err(|e| format!("Failed to parse {} theme: {}", name, e))
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
    _capability: ColorCapability,
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

        Self { cache, _capability: capability, colors_enabled }
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
    vec![
        // Entity::Player (1 variant)
        ColorId::Entity(EntityColorId::Player),
        // Entity::Monster (8 variants)
        ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead)),
        ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileBeast)),
        ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileHumanoid)),
        ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileMagical)),
        ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileConstruct)),
        ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileDragon)),
        ColorId::Entity(EntityColorId::Monster(MonsterColorId::Neutral)),
        ColorId::Entity(EntityColorId::Monster(MonsterColorId::Friendly)),
        // Entity::Item (5 variants)
        ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Common)),
        ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Uncommon)),
        ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Rare)),
        ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Epic)),
        ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Legendary)),
        // Entity::Terrain (13 variants)
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallStone)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallWood)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallMetal)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::WallBrick)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorStone)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorWood)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorDirt)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::FloorGrass)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Water)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Lava)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Door)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::StairsUp)),
        ColorId::Entity(EntityColorId::Terrain(TerrainColorId::StairsDown)),
        // UI (16 variants)
        ColorId::Ui(UiColorId::HealthHigh),
        ColorId::Ui(UiColorId::HealthMedium),
        ColorId::Ui(UiColorId::HealthLow),
        ColorId::Ui(UiColorId::Mana),
        ColorId::Ui(UiColorId::Stamina),
        ColorId::Ui(UiColorId::Experience),
        ColorId::Ui(UiColorId::Highlight),
        ColorId::Ui(UiColorId::Selection),
        ColorId::Ui(UiColorId::Cursor),
        ColorId::Ui(UiColorId::TextDefault),
        ColorId::Ui(UiColorId::TextDim),
        ColorId::Ui(UiColorId::TextBold),
        ColorId::Ui(UiColorId::MessageInfo),
        ColorId::Ui(UiColorId::MessageWarning),
        ColorId::Ui(UiColorId::MessageDanger),
        ColorId::Ui(UiColorId::MessageSuccess),
        // Effect (11 variants)
        ColorId::Effect(EffectColorId::Fire),
        ColorId::Effect(EffectColorId::Ice),
        ColorId::Effect(EffectColorId::Lightning),
        ColorId::Effect(EffectColorId::Poison),
        ColorId::Effect(EffectColorId::Acid),
        ColorId::Effect(EffectColorId::MagicArcane),
        ColorId::Effect(EffectColorId::MagicHoly),
        ColorId::Effect(EffectColorId::MagicDark),
        ColorId::Effect(EffectColorId::Blood),
        ColorId::Effect(EffectColorId::Impact),
        ColorId::Effect(EffectColorId::Shield),
        // Environment (5 variants)
        ColorId::Environment(EnvironmentColorId::LightTorch),
        ColorId::Environment(EnvironmentColorId::LightLantern),
        ColorId::Environment(EnvironmentColorId::LightMagic),
        ColorId::Environment(EnvironmentColorId::Darkness),
        ColorId::Environment(EnvironmentColorId::Fog),
    ]
}

/// Converts a ColorSpec to a ratatui Color.
///
/// Maps omega-core's color representations to ratatui's Color enum,
/// handling RGB, indexed (256-color), and ANSI 16-color formats.
pub fn colorspec_to_ratatui(spec: &ColorSpec) -> Color {
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
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightBlack)), Color::DarkGray);
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightRed)), Color::LightRed);
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightGreen)),
            Color::LightGreen
        );
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightYellow)),
            Color::LightYellow
        );
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightBlue)), Color::LightBlue);
        assert_eq!(
            colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightMagenta)),
            Color::LightMagenta
        );
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightCyan)), Color::LightCyan);
        assert_eq!(colorspec_to_ratatui(&ColorSpec::Ansi(AnsiColor::BrightWhite)), Color::White);
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

    #[test]
    fn load_builtin_classic_theme() {
        let theme = load_builtin_theme("classic").expect("classic theme should load");
        assert_eq!(theme.meta.name, "Classic");
        let cache = StyleCache::new(&theme, ColorCapability::TrueColor);
        assert!(cache.get(&ColorId::Entity(EntityColorId::Player)).fg.is_some());
    }

    #[test]
    fn load_builtin_accessible_theme() {
        let theme = load_builtin_theme("accessible").expect("accessible theme should load");
        assert_eq!(theme.meta.name, "Accessible");
        let cache = StyleCache::new(&theme, ColorCapability::TrueColor);
        assert!(cache.get(&ColorId::Entity(EntityColorId::Player)).fg.is_some());
    }
}
