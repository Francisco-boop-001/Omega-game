//! Color theme definitions for the Omega game engine.
//!
//! This module provides the `ColorTheme` struct and related types for
//! TOML-based theme definitions. Themes map semantic `ColorId` values to
//! concrete hex colors using a three-tier architecture:
//!
//! 1. **Base Palette**: Core color definitions (red, green, blue, etc.)
//! 2. **Semantic Mappings**: Meaningful aliases (danger, success, info, etc.)
//! 3. **Usage Categories**: Entity, UI, and Effect color assignments
//!
//! # Three-Tier Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  Tier 3: Usage Categories               │
//! │  (entity.player, ui.health_high, etc.)  │
//! ├─────────────────────────────────────────┤
//! │  Tier 2: Semantic Mappings              │
//! │  (danger → base.red, success → green)   │
//! ├─────────────────────────────────────────┤
//! │  Tier 1: Base Palette                   │
//! │  (red=#FF0000, green=#00FF00, etc.)     │
//! └─────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```rust
//! use omega_core::color::{ColorId, ColorTheme, EntityColorId, MonsterColorId};
//!
//! // Load a theme from TOML (or create programmatically)
//! // let theme: ColorTheme = toml::from_str(theme_toml).unwrap();
//!
//! // Resolve a color through the theme
//! // let color_id = ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead));
//! // let (fg, bg) = theme.resolve(&color_id).unwrap();
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

use super::color_id::{ColorId, EntityColorId};
use super::hex_color::HexColor;

/// Errors that can occur when working with color themes.
#[derive(Error, Debug)]
pub enum ThemeError {
    /// Failed to parse TOML.
    #[error("Failed to parse TOML: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// IO error while reading theme file.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Could not resolve a color reference.
    #[error("Unresolved reference: '{0}'")]
    UnresolvedReference(String),

    /// Circular reference detected.
    #[error("Circular reference detected: '{0}'")]
    CircularReference(String),

    /// Missing required section in theme.
    #[error("Missing required section: '{0}'")]
    MissingSection(String),

    /// Invalid color value.
    #[error("Invalid color value in section '{section}', key '{key}': {reason}")]
    InvalidColorValue {
        /// The section where the error occurred.
        section: String,
        /// The key with the invalid value.
        key: String,
        /// The reason the value is invalid.
        reason: String,
    },

    /// Missing required ColorId mapping.
    #[error("Missing required ColorId mapping: '{0}'")]
    MissingColorId(String),
}

/// Metadata for a color theme.
///
/// Contains descriptive information about the theme including
/// authorship, versioning, and compatibility requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    /// The display name of the theme.
    pub name: String,
    /// The author or creator of the theme.
    pub author: String,
    /// A brief description of the theme's style or purpose.
    pub description: String,
    /// Theme version (semantic versioning recommended).
    pub version: String,
    /// Theme variant: "dark" or "light".
    pub variant: String,
    /// Minimum compatible Omega engine version.
    pub min_engine_version: String,
}

/// Base color palette with core color definitions.
///
/// Defines the fundamental colors available in the theme.
/// Extended palette colors are optional to allow themes to define
/// only the colors they need.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    // Core palette (required)
    /// Primary red color.
    pub red: HexColor,
    /// Primary green color.
    pub green: HexColor,
    /// Primary blue color.
    pub blue: HexColor,
    /// Primary yellow color.
    pub yellow: HexColor,
    /// Primary cyan color.
    pub cyan: HexColor,
    /// Primary magenta color.
    pub magenta: HexColor,
    /// White/off-white color.
    pub white: HexColor,
    /// Black color (often dark gray in practice).
    pub black: HexColor,
    /// Gray mid-tone.
    pub gray: HexColor,

    // Extended palette (optional)
    /// Orange color for warm accents.
    pub orange: Option<HexColor>,
    /// Purple/violet color.
    pub purple: Option<HexColor>,
    /// Brown/earth tone color.
    pub brown: Option<HexColor>,
    /// Dark gray variant.
    pub dark_gray: Option<HexColor>,
    /// Light gray variant.
    pub light_gray: Option<HexColor>,
}

/// Semantic color mappings.
///
/// Provides meaningful aliases for base palette colors.
/// These mappings create a semantic layer between raw colors
/// and their usage, enabling consistent theming.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticColors {
    /// Danger/error states - typically red.
    pub danger: ColorRef,
    /// Success/positive states - typically green.
    pub success: ColorRef,
    /// Information/neutral states - typically blue.
    pub info: ColorRef,
    /// Warning/caution states - typically yellow or orange.
    pub warning: ColorRef,
    /// Magical/arcane elements - typically purple or cyan.
    pub magic: ColorRef,
    /// Neutral/default states - typically gray.
    pub neutral: ColorRef,
}

/// A color reference that can be either a direct hex color pair
/// or a reference to another color in the theme.
///
/// This enum supports two formats in TOML:
/// - Direct: `{ fg = "#FF0000", bg = "#000000" }`
/// - Reference: `{ ref_path = "base.red" }`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorRef {
    /// Reference to another color by path (e.g., "base.red", "semantic.danger").
    Reference {
        /// The reference path using dot notation.
        /// Note: Uses `ref_path` instead of `ref` since `ref` is a reserved keyword.
        #[serde(rename = "ref")]
        ref_path: String,
    },
    /// Direct foreground and background color specification.
    Direct {
        /// Foreground color.
        fg: HexColor,
        /// Background color.
        bg: HexColor,
    },
}

use super::animation::AnimationKind;

/// A complete color theme definition.
///
/// Combines metadata, base palette, semantic mappings, and
/// usage-specific color assignments into a complete theme.
/// Themes can be serialized to/from TOML for easy customization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorTheme {
    /// Theme metadata (name, author, version, etc.).
    pub meta: ThemeMetadata,
    /// Base color palette definitions.
    pub base: ColorPalette,
    /// Semantic color mappings.
    pub semantic: SemanticColors,
    /// Entity-specific color assignments (player, monsters, items, terrain).
    pub entity: HashMap<String, ColorRef>,
    /// UI element color assignments (health bars, text, messages).
    pub ui: HashMap<String, ColorRef>,
    /// Effect color assignments (spells, particles).
    pub effect: HashMap<String, ColorRef>,
    /// Optional animations for specific ColorIds or semantic keys.
    #[serde(default)]
    pub animations: HashMap<String, AnimationKind>,
}

impl ColorTheme {
    /// Resolves a `ColorId` to its concrete foreground and background colors.
    ///
    /// This method traverses the three-tier architecture:
    /// 1. Look up the ColorId in the appropriate usage category (entity/ui/effect)
    /// 2. If the result is a reference, resolve it through semantic or base palette
    /// 3. Return the final (fg, bg) color pair
    ///
    /// Returns `None` if the color ID cannot be resolved.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::{ColorId, ColorTheme, EntityColorId, MonsterColorId};
    ///
    /// // Assuming theme is loaded from TOML or created programmatically
    /// // let theme: ColorTheme = ...;
    /// // let color_id = ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead));
    /// // if let Some((fg, bg)) = theme.resolve(&color_id) {
    /// //     println!("Foreground: {}, Background: {}", fg, bg);
    /// // }
    /// ```
    pub fn resolve(&self, color_id: &ColorId) -> Option<(HexColor, HexColor)> {
        // Convert ColorId to string key for lookup
        let key = color_id_to_key(color_id);

        // Look up in appropriate usage category
        let color_ref = if let Some(stripped) = key.strip_prefix("entity.") {
            self.entity.get(stripped)
        } else if let Some(stripped) = key.strip_prefix("ui.") {
            self.ui.get(stripped)
        } else if let Some(stripped) = key.strip_prefix("effect.") {
            self.effect.get(stripped)
        } else {
            None
        };

        match color_ref {
            Some(ColorRef::Direct { fg, bg }) => Some((*fg, *bg)),
            Some(ColorRef::Reference { ref_path }) => {
                // Resolve reference by looking up in base palette or semantic mappings
                self.resolve_reference(ref_path)
            }
            None => None,
        }
    }

    /// Resolves an animated color for a specific ColorId at a given time.
    pub fn resolve_animated(&self, color_id: &ColorId, time: f32) -> super::color_spec::ColorSpec {
        let key = color_id_to_key(color_id);

        // 1. Check if there's a specific animation for this key
        if let Some(animation) = self.animations.get(&key) {
            return animation.resolve_at(time);
        }

        // 2. Resolve to static colors first
        if let Some((fg, _bg)) = self.resolve(color_id) {
            return super::color_spec::ColorSpec::Rgb {
                r: fg.to_rgb().0,
                g: fg.to_rgb().1,
                b: fg.to_rgb().2,
            };
        }

        // Fallback to default spec
        super::color_spec::ColorSpec::default()
    }

    /// Resolves a reference path to concrete colors.
    ///
    /// Supports paths like:
    /// - `"base.red"` - Direct base palette lookup
    /// - `"semantic.danger"` - Semantic mapping lookup (may chain to base)
    pub fn resolve_reference(&self, ref_path: &str) -> Option<(HexColor, HexColor)> {
        // Parse "base.red" or "semantic.danger"
        let parts: Vec<&str> = ref_path.split('.').collect();
        if parts.len() != 2 {
            return None;
        }

        match parts[0] {
            "base" => self.get_base_color(parts[1]),
            "semantic" => {
                // Resolve semantic to base (following the chain)
                let base_ref = match parts[1] {
                    "danger" => &self.semantic.danger,
                    "success" => &self.semantic.success,
                    "info" => &self.semantic.info,
                    "warning" => &self.semantic.warning,
                    "magic" => &self.semantic.magic,
                    "neutral" => &self.semantic.neutral,
                    _ => return None,
                };

                match base_ref {
                    ColorRef::Reference { ref_path } => self.resolve_reference(ref_path),
                    ColorRef::Direct { fg, bg } => Some((*fg, *bg)),
                }
            }
            _ => None,
        }
    }

    /// Retrieves a color from the base palette.
    ///
    /// Returns the color with black as the default background.
    fn get_base_color(&self, name: &str) -> Option<(HexColor, HexColor)> {
        let color = match name {
            "red" => Some(self.base.red),
            "green" => Some(self.base.green),
            "blue" => Some(self.base.blue),
            "yellow" => Some(self.base.yellow),
            "cyan" => Some(self.base.cyan),
            "magenta" => Some(self.base.magenta),
            "white" => Some(self.base.white),
            "black" => Some(self.base.black),
            "gray" => Some(self.base.gray),
            "orange" => self.base.orange,
            "purple" => self.base.purple,
            "brown" => self.base.brown,
            "dark_gray" => self.base.dark_gray,
            "light_gray" => self.base.light_gray,
            _ => None,
        };

        color.map(|c| (c, self.base.black)) // Default bg to black
    }

    /// Loads a ColorTheme from a TOML string.
    ///
    /// This method parses the TOML string into a ColorTheme and
    /// validates that all references can be resolved.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::ColorTheme;
    ///
    /// let toml_str = r##"
    /// [meta]
    /// name = "Test"
    /// author = "Test"
    /// description = "Test"
    /// version = "1.0.0"
    /// variant = "dark"
    /// min_engine_version = "0.1.0"
    ///
    /// [base]
    /// red = "#FF0000"
    /// green = "#00FF00"
    /// blue = "#0000FF"
    /// yellow = "#FFFF00"
    /// cyan = "#00FFFF"
    /// magenta = "#FF00FF"
    /// white = "#FFFFFF"
    /// black = "#000000"
    /// gray = "#808080"
    ///
    /// [semantic]
    /// danger = { ref = "base.red" }
    /// success = { ref = "base.green" }
    /// info = { ref = "base.blue" }
    /// warning = { ref = "base.yellow" }
    /// magic = { ref = "base.magenta" }
    /// neutral = { ref = "base.gray" }
    ///
    /// [entity]
    /// [ui]
    /// [effect]
    /// "##;
    ///
    /// let theme = ColorTheme::from_toml(toml_str).unwrap();
    /// ```
    pub fn from_toml(toml_str: &str) -> Result<Self, ThemeError> {
        let theme: ColorTheme = toml::from_str(toml_str)?;
        theme.validate()?;
        Ok(theme)
    }

    /// Loads a ColorTheme from a file.
    ///
    /// Reads the file at the given path and parses it as TOML.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use omega_core::color::ColorTheme;
    ///
    /// let theme = ColorTheme::load_from_file(Path::new("themes/dark.toml")).unwrap();
    /// ```
    pub fn load_from_file(path: &Path) -> Result<Self, ThemeError> {
        let content = fs::read_to_string(path)?;
        Self::from_toml(&content)
    }

    /// Validates the theme's structure and references.
    ///
    /// Checks that:
    /// - Required sections are present
    /// - All color references can be resolved
    /// - No circular references exist
    ///
    /// Returns Ok(()) if the theme is valid, or a ThemeError otherwise.
    pub fn validate(&self) -> Result<(), ThemeError> {
        // Check required sections
        if self.meta.name.is_empty() {
            return Err(ThemeError::MissingSection("meta.name".to_string()));
        }

        // Validate all references can be resolved
        let mut visited = std::collections::HashSet::new();

        // Check entity references
        for (key, color_ref) in &self.entity {
            if let ColorRef::Reference { ref_path } = color_ref {
                self.resolve_ref_check(ref_path, &mut visited).map_err(|e| {
                    ThemeError::UnresolvedReference(format!("entity.{}: {}", key, e))
                })?;
            }
        }

        // Check UI references
        for (key, color_ref) in &self.ui {
            if let ColorRef::Reference { ref_path } = color_ref {
                self.resolve_ref_check(ref_path, &mut visited)
                    .map_err(|e| ThemeError::UnresolvedReference(format!("ui.{}: {}", key, e)))?;
            }
        }

        // Check effect references
        for (key, color_ref) in &self.effect {
            if let ColorRef::Reference { ref_path } = color_ref {
                self.resolve_ref_check(ref_path, &mut visited).map_err(|e| {
                    ThemeError::UnresolvedReference(format!("effect.{}: {}", key, e))
                })?;
            }
        }

        Ok(())
    }

    /// Checks that a reference can be resolved.
    ///
    /// This internal method validates that:
    /// - The reference path format is valid (category.name)
    /// - The referenced color exists
    /// - There are no circular references
    fn resolve_ref_check(
        &self,
        ref_path: &str,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<(), String> {
        // Check for circular reference
        if !visited.insert(ref_path.to_string()) {
            return Err(format!("Circular reference: {}", ref_path));
        }

        let parts: Vec<&str> = ref_path.split('.').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid reference format: {}", ref_path));
        }

        match parts[0] {
            "base" => {
                // Check if base color exists
                if self.get_base_color(parts[1]).is_none() {
                    return Err(format!("Base color '{}' not found", parts[1]));
                }
            }
            "semantic" => {
                // Check semantic color and follow if it's a reference
                let semantic_ref = match parts[1] {
                    "danger" => &self.semantic.danger,
                    "success" => &self.semantic.success,
                    "info" => &self.semantic.info,
                    "warning" => &self.semantic.warning,
                    "magic" => &self.semantic.magic,
                    "neutral" => &self.semantic.neutral,
                    _ => return Err(format!("Unknown semantic color: {}", parts[1])),
                };

                if let ColorRef::Reference { ref_path: next_ref } = semantic_ref {
                    self.resolve_ref_check(next_ref, visited)?;
                }
            }
            "entity" | "ui" | "effect" => {
                let map = match parts[0] {
                    "entity" => &self.entity,
                    "ui" => &self.ui,
                    "effect" => &self.effect,
                    _ => unreachable!(),
                };

                if let Some(color_ref) = map.get(parts[1]) {
                    if let ColorRef::Reference { ref_path: next_ref } = color_ref {
                        self.resolve_ref_check(next_ref, visited)?;
                    }
                } else {
                    return Err(format!("Unknown {} color: {}", parts[0], parts[1]));
                }
            }
            _ => return Err(format!("Unknown reference category: {}", parts[0])),
        }

        visited.remove(ref_path);
        Ok(())
    }

    /// Resolves a ColorId to concrete colors, returning an error on failure.
    ///
    /// This is a convenience method that wraps `resolve` and returns a
    /// ThemeError instead of Option when the color cannot be resolved.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::{ColorId, ColorTheme, EntityColorId};
    ///
    /// // Assuming theme is loaded
    /// // let theme: ColorTheme = ...;
    /// // let color_id = ColorId::Entity(EntityColorId::Player);
    /// // let (fg, bg) = theme.resolve_color(&color_id)?;
    /// ```
    pub fn resolve_color(&self, color_id: &ColorId) -> Result<(HexColor, HexColor), ThemeError> {
        self.resolve(color_id)
            .ok_or_else(|| ThemeError::UnresolvedReference(format!("{:?}", color_id)))
    }
}

/// Converts a `ColorId` to a dot-notation string key.
///
/// This function maps the enum structure to hierarchical keys
/// for lookup in the theme's usage category maps.
fn color_id_to_key(color_id: &ColorId) -> String {
    match color_id {
        ColorId::Entity(e) => format!("entity.{}", entity_to_key(e)),
        ColorId::Ui(u) => format!("ui.{:?}", u).to_lowercase(),
        ColorId::Effect(e) => format!("effect.{:?}", e).to_lowercase(),
        ColorId::Environment(_) => String::new(), // Not used in themes
    }
}

use crate::LegacyEnvironment;

impl ColorTheme {
    /// Maps a game environment to a recommended theme name.
    pub fn name_for_environment(env: LegacyEnvironment) -> &'static str {
        match env {
            LegacyEnvironment::City | LegacyEnvironment::Village => "classic",
            LegacyEnvironment::Sewers | LegacyEnvironment::Caves | LegacyEnvironment::Castle => {
                "classic"
            } // Could be a dark theme
            LegacyEnvironment::Abyss | LegacyEnvironment::Volcano => "classic", // Could be a hell theme
            _ => "classic",
        }
    }
}
///
/// Creates dot-notation paths like:
/// - `"player"`
/// - `"monster.hostileundead"`
/// - `"item.legendary"`
/// - `"terrain.wallstone"`
fn entity_to_key(entity: &EntityColorId) -> String {
    match entity {
        EntityColorId::Player => "player".to_string(),
        EntityColorId::Monster(m) => format!("monster.{:?}", m).to_lowercase(),
        EntityColorId::Item(r) => format!("item.{:?}", r).to_lowercase(),
        EntityColorId::Terrain(t) => format!("terrain.{:?}", t).to_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::color_id::{
        EffectColorId, ItemRarityColorId, MonsterColorId, TerrainColorId, UiColorId,
    };
    use super::*;

    /// Creates a minimal valid theme for testing.
    fn create_test_theme() -> ColorTheme {
        ColorTheme {
            meta: ThemeMetadata {
                name: "Test Theme".to_string(),
                author: "Test Author".to_string(),
                description: "A theme for testing".to_string(),
                version: "1.0.0".to_string(),
                variant: "dark".to_string(),
                min_engine_version: "0.1.0".to_string(),
            },
            base: ColorPalette {
                red: HexColor::from_hex("#FF0000").unwrap(),
                green: HexColor::from_hex("#00FF00").unwrap(),
                blue: HexColor::from_hex("#0000FF").unwrap(),
                yellow: HexColor::from_hex("#FFFF00").unwrap(),
                cyan: HexColor::from_hex("#00FFFF").unwrap(),
                magenta: HexColor::from_hex("#FF00FF").unwrap(),
                white: HexColor::from_hex("#FFFFFF").unwrap(),
                black: HexColor::from_hex("#000000").unwrap(),
                gray: HexColor::from_hex("#808080").unwrap(),
                orange: Some(HexColor::from_hex("#FFA500").unwrap()),
                purple: Some(HexColor::from_hex("#800080").unwrap()),
                brown: Some(HexColor::from_hex("#8B4513").unwrap()),
                dark_gray: Some(HexColor::from_hex("#404040").unwrap()),
                light_gray: Some(HexColor::from_hex("#C0C0C0").unwrap()),
            },
            semantic: SemanticColors {
                danger: ColorRef::Reference { ref_path: "base.red".to_string() },
                success: ColorRef::Reference { ref_path: "base.green".to_string() },
                info: ColorRef::Reference { ref_path: "base.blue".to_string() },
                warning: ColorRef::Reference { ref_path: "base.yellow".to_string() },
                magic: ColorRef::Reference { ref_path: "base.purple".to_string() },
                neutral: ColorRef::Reference { ref_path: "base.gray".to_string() },
            },
            entity: {
                let mut map = HashMap::new();
                map.insert(
                    "player".to_string(),
                    ColorRef::Reference { ref_path: "base.white".to_string() },
                );
                map.insert(
                    "monster.hostileundead".to_string(),
                    ColorRef::Reference { ref_path: "semantic.danger".to_string() },
                );
                map.insert(
                    "item.legendary".to_string(),
                    ColorRef::Direct {
                        fg: HexColor::from_hex("#FFD700").unwrap(),
                        bg: HexColor::from_hex("#000000").unwrap(),
                    },
                );
                map
            },
            ui: {
                let mut map = HashMap::new();
                map.insert(
                    "healthhigh".to_string(),
                    ColorRef::Reference { ref_path: "semantic.success".to_string() },
                );
                map.insert(
                    "messagedanger".to_string(),
                    ColorRef::Reference { ref_path: "semantic.danger".to_string() },
                );
                map
            },
            effect: HashMap::new(),
            animations: HashMap::new(),
        }
    }

    #[test]
    fn theme_struct_compiles() {
        // Verify we can create a theme instance
        let theme = create_test_theme();
        assert_eq!(theme.meta.name, "Test Theme");
        assert_eq!(theme.meta.author, "Test Author");
    }

    #[test]
    fn color_palette_core_colors() {
        let theme = create_test_theme();
        assert_eq!(theme.base.red.to_rgb(), (255, 0, 0));
        assert_eq!(theme.base.green.to_rgb(), (0, 255, 0));
        assert_eq!(theme.base.blue.to_rgb(), (0, 0, 255));
    }

    #[test]
    fn color_palette_extended_colors() {
        let theme = create_test_theme();
        assert!(theme.base.orange.is_some());
        assert_eq!(theme.base.orange.unwrap().to_rgb(), (255, 165, 0));
    }

    #[test]
    fn semantic_colors_are_references() {
        let theme = create_test_theme();

        match &theme.semantic.danger {
            ColorRef::Reference { ref_path } => assert_eq!(ref_path, "base.red"),
            _ => panic!("Expected Reference variant"),
        }

        match &theme.semantic.success {
            ColorRef::Reference { ref_path } => assert_eq!(ref_path, "base.green"),
            _ => panic!("Expected Reference variant"),
        }
    }

    #[test]
    fn resolve_base_color() {
        let theme = create_test_theme();
        let color_id = ColorId::Entity(EntityColorId::Player);

        let result = theme.resolve(&color_id);
        assert!(result.is_some());

        let (fg, bg) = result.unwrap();
        assert_eq!(fg.to_rgb(), (255, 255, 255)); // white
        assert_eq!(bg.to_rgb(), (0, 0, 0)); // black
    }

    #[test]
    fn resolve_semantic_reference() {
        let theme = create_test_theme();
        let color_id = ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead));

        let result = theme.resolve(&color_id);
        assert!(result.is_some());

        let (fg, bg) = result.unwrap();
        assert_eq!(fg.to_rgb(), (255, 0, 0)); // red (through semantic.danger)
        assert_eq!(bg.to_rgb(), (0, 0, 0)); // black
    }

    #[test]
    fn resolve_direct_color() {
        let theme = create_test_theme();
        let color_id = ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Legendary));

        let result = theme.resolve(&color_id);
        assert!(result.is_some());

        let (fg, bg) = result.unwrap();
        assert_eq!(fg.to_rgb(), (255, 215, 0)); // gold
        assert_eq!(bg.to_rgb(), (0, 0, 0)); // black
    }

    #[test]
    fn resolve_ui_color() {
        let theme = create_test_theme();
        let color_id = ColorId::Ui(UiColorId::HealthHigh);

        let result = theme.resolve(&color_id);
        assert!(result.is_some());

        let (fg, bg) = result.unwrap();
        assert_eq!(fg.to_rgb(), (0, 255, 0)); // green (through semantic.success)
        assert_eq!(bg.to_rgb(), (0, 0, 0)); // black
    }

    #[test]
    fn resolve_unknown_color() {
        let theme = create_test_theme();
        // Environment colors are not supported in themes
        let color_id = ColorId::Environment(super::super::color_id::EnvironmentColorId::Darkness);

        let result = theme.resolve(&color_id);
        assert!(result.is_none());
    }

    #[test]
    fn serde_roundtrip() {
        let theme = create_test_theme();

        // Serialize to TOML
        let toml_str = toml::to_string(&theme).expect("Failed to serialize to TOML");
        assert!(toml_str.contains("name = \"Test Theme\""));
        assert!(toml_str.contains("red = \"#FF0000\""));

        // Deserialize back
        let deserialized: ColorTheme =
            toml::from_str(&toml_str).expect("Failed to deserialize from TOML");
        assert_eq!(deserialized.meta.name, theme.meta.name);
        assert_eq!(deserialized.base.red, theme.base.red);
    }

    #[test]
    fn color_ref_direct_serde() {
        let color_ref = ColorRef::Direct {
            fg: HexColor::from_hex("#FF5733").unwrap(),
            bg: HexColor::from_hex("#000000").unwrap(),
        };

        let json = serde_json::to_string(&color_ref).unwrap();
        assert!(json.contains("\"fg\":\"#FF5733\""));
        assert!(json.contains("\"bg\":\"#000000\""));

        let deserialized: ColorRef = serde_json::from_str(&json).unwrap();
        match deserialized {
            ColorRef::Direct { fg, bg } => {
                assert_eq!(fg.to_rgb(), (255, 87, 51));
                assert_eq!(bg.to_rgb(), (0, 0, 0));
            }
            _ => panic!("Expected Direct variant"),
        }
    }

    #[test]
    fn color_ref_reference_serde() {
        let color_ref = ColorRef::Reference { ref_path: "base.red".to_string() };

        let json = serde_json::to_string(&color_ref).unwrap();
        // Note: serde renames "ref_path" to "ref" in serialization
        assert!(json.contains("\"ref\""));
        assert!(json.contains("\"base.red\""));

        let deserialized: ColorRef = serde_json::from_str(&json).unwrap();
        match deserialized {
            ColorRef::Reference { ref_path } => {
                assert_eq!(ref_path, "base.red");
            }
            _ => panic!("Expected Reference variant"),
        }
    }

    #[test]
    fn three_tier_resolution_chain() {
        // Test the full chain: usage -> semantic -> base
        let theme = create_test_theme();

        // healthhigh -> semantic.success -> base.green
        let color_id = ColorId::Ui(UiColorId::HealthHigh);
        let (fg, _bg) = theme.resolve(&color_id).unwrap();
        assert_eq!(fg.to_rgb(), (0, 255, 0)); // green
    }

    #[test]
    fn entity_key_generation() {
        let player = EntityColorId::Player;
        assert_eq!(entity_to_key(&player), "player");

        let monster = EntityColorId::Monster(MonsterColorId::HostileUndead);
        assert_eq!(entity_to_key(&monster), "monster.hostileundead");

        let item = EntityColorId::Item(ItemRarityColorId::Legendary);
        assert_eq!(entity_to_key(&item), "item.legendary");

        let terrain = EntityColorId::Terrain(TerrainColorId::WallStone);
        assert_eq!(entity_to_key(&terrain), "terrain.wallstone");
    }

    #[test]
    fn color_id_key_generation() {
        let entity_id = ColorId::Entity(EntityColorId::Player);
        assert_eq!(color_id_to_key(&entity_id), "entity.player");

        let ui_id = ColorId::Ui(UiColorId::HealthHigh);
        assert_eq!(color_id_to_key(&ui_id), "ui.healthhigh");

        let effect_id = ColorId::Effect(EffectColorId::Fire);
        assert_eq!(color_id_to_key(&effect_id), "effect.fire");

        let env_id = ColorId::Environment(super::super::color_id::EnvironmentColorId::Darkness);
        assert_eq!(color_id_to_key(&env_id), ""); // Not supported
    }
}
