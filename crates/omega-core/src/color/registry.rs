//! Theme registry for managing built-in and user-defined themes.
//!
//! The registry provides a central point for theme discovery, selection,
//! and management, supporting overrides and listing available themes.

use crate::color::loader::ThemeLoader;
use crate::color::theme::ColorTheme;
use std::collections::HashMap;
use std::path::PathBuf;

/// Manages a collection of themes with prioritization.
pub struct ThemeRegistry {
    /// Maps theme names to their definitions.
    themes: HashMap<String, RegisteredTheme>,
}

/// A theme stored in the registry.
#[derive(Clone)]
pub struct RegisteredTheme {
    /// The theme definition.
    pub theme: ColorTheme,
    /// The source path if it was loaded from disk.
    pub path: Option<PathBuf>,
    /// Whether this is a built-in theme.
    pub is_builtin: bool,
}

impl ThemeRegistry {
    /// Creates a new empty registry.
    pub fn new() -> Self {
        Self { themes: HashMap::new() }
    }

    /// Loads all available themes (built-in and user-defined).
    pub fn load_all() -> Self {
        let mut registry = Self::new();
        // Built-in themes should be registered by the caller/frontend
        // as they are typically embedded via include_str!

        // Load user themes from filesystem
        for theme in ThemeLoader::load_user_themes() {
            registry.register_user_theme(theme, None); // Loader currently doesn't return path
        }

        registry
    }

    /// Registers a built-in theme.
    pub fn register_builtin(&mut self, theme: ColorTheme) {
        let name = theme.meta.name.to_lowercase();
        self.themes.insert(name, RegisteredTheme { theme, path: None, is_builtin: true });
    }

    /// Registers a user theme, potentially overriding a built-in one.
    pub fn register_user_theme(&mut self, theme: ColorTheme, path: Option<PathBuf>) {
        let name = theme.meta.name.to_lowercase();
        // User themes always override if they have the same name
        self.themes.insert(name, RegisteredTheme { theme, path, is_builtin: false });
    }

    /// Retrieves a theme by name (case-insensitive).
    pub fn get_theme(&self, name: &str) -> Option<&ColorTheme> {
        self.themes.get(&name.to_lowercase()).map(|rt| &rt.theme)
    }

    /// Returns a list of all registered theme names.
    pub fn list_themes(&self) -> Vec<String> {
        let mut names: Vec<String> =
            self.themes.values().map(|rt| rt.theme.meta.name.clone()).collect();
        names.sort();
        names
    }

    /// Returns detailed metadata for all registered themes.
    pub fn theme_metadata(&self) -> Vec<crate::color::theme::ThemeMetadata> {
        let mut meta: Vec<_> = self.themes.values().map(|rt| rt.theme.meta.clone()).collect();
        meta.sort_by(|a, b| a.name.cmp(&b.name));
        meta
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::theme::ThemeMetadata;

    fn mock_theme(name: &str) -> ColorTheme {
        use crate::color::{ColorPalette, ColorRef, HexColor, SemanticColors};

        let red = HexColor::from_hex("#FF0000").unwrap();
        let black = HexColor::from_hex("#000000").unwrap();

        ColorTheme {
            meta: ThemeMetadata {
                name: name.to_string(),
                author: "Test".to_string(),
                description: "Test".to_string(),
                version: "1.0.0".to_string(),
                variant: "dark".to_string(),
                min_engine_version: "0.1.0".to_string(),
            },
            base: ColorPalette {
                red,
                green: red,
                blue: red,
                yellow: red,
                cyan: red,
                magenta: red,
                white: red,
                black,
                gray: red,
                orange: None,
                purple: None,
                brown: None,
                dark_gray: None,
                light_gray: None,
            },
            semantic: SemanticColors {
                danger: ColorRef::Direct { fg: red, bg: black },
                success: ColorRef::Direct { fg: red, bg: black },
                info: ColorRef::Direct { fg: red, bg: black },
                warning: ColorRef::Direct { fg: red, bg: black },
                magic: ColorRef::Direct { fg: red, bg: black },
                neutral: ColorRef::Direct { fg: red, bg: black },
            },
            entity: HashMap::new(),
            ui: HashMap::new(),
            effect: HashMap::new(),
            animations: HashMap::new(),
        }
    }

    #[test]
    fn test_registry_prioritization() {
        let mut registry = ThemeRegistry::new();
        let builtin = mock_theme("Classic");
        let user = mock_theme("Classic"); // Same name

        registry.register_builtin(builtin);
        assert!(registry.themes.get("classic").unwrap().is_builtin);

        registry.register_user_theme(user, None);
        assert!(!registry.themes.get("classic").unwrap().is_builtin);
    }
}
