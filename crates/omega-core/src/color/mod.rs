//! Color system for Omega.
//!
//! This module provides the color infrastructure for the Omega game engine,
//! including semantic color identifiers, color specifications, themes, and
//! terminal capability detection.
//!
//! # Overview
//!
//! The color system is organized into several layers:
//!
//! 1. **ColorId**: Semantic identifiers for colors (e.g., `MonsterColorId::HostileUndead`)
//! 2. **ColorSpec**: Concrete color values in various color spaces (RGB, ANSI 256, ANSI 16)
//! 3. **ColorTheme**: TOML-based theme definitions mapping ColorIds to ColorSpecs
//! 4. **ColorCapability**: Terminal detection and automatic color depth adaptation
//!
//! # Example
//!
//! ```rust
//! use omega_core::color::{ColorId, EntityColorId, MonsterColorId};
//!
//! // Create a semantic color identifier
//! let color_id = ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead));
//!
//! // Later, this is resolved through a theme to get concrete colors
//! // let color_spec = theme.resolve(color_id);
//! ```

pub mod capability;
pub mod color_id;
pub mod color_spec;
pub mod hex_color;
pub mod theme;

// Re-export the main types for convenient access
pub use capability::{ColorCapability, get_capability, reset_capability};
pub use color_id::{
    ColorId, EffectColorId, EntityColorId, EnvironmentColorId, ItemRarityColorId,
    MonsterColorId, TerrainColorId, UiColorId,
};
pub use color_spec::{AnsiColor, ColorSpec};
pub use hex_color::{HexColor, HexColorError};
pub use theme::{ColorPalette, ColorRef, ColorTheme, SemanticColors, ThemeMetadata};
