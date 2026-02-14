//! Theme loading utilities for the Omega color system.
//!
//! Provides functionality for discovering and loading themes from the
//! filesystem, including platform-specific user configuration directories.

use crate::color::theme::{ColorTheme, ThemeError};
use std::fs;
use std::path::{Path, PathBuf};

/// Handles discovery and loading of color themes from the filesystem.
pub struct ThemeLoader;

impl ThemeLoader {
    /// Returns the platform-specific user theme directory.
    ///
    /// Linux:   ~/.config/omega/themes/
    /// macOS:   ~/Library/Application Support/omega/themes/
    /// Windows: %AppData%\omega\themes
    pub fn user_theme_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|mut p| {
            p.push("omega");
            p.push("themes");
            p
        })
    }

    /// Scans the user theme directory for .toml theme files.
    pub fn find_user_themes() -> Vec<PathBuf> {
        let mut themes = Vec::new();
        if let Some(dir) = Self::user_theme_dir()
            && let Ok(entries) = fs::read_dir(dir)
        {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "toml") {
                    themes.push(path);
                }
            }
        }
        themes
    }

    /// Loads all discovered user themes.
    ///
    /// Skips themes that fail to load or validate.
    pub fn load_user_themes() -> Vec<ColorTheme> {
        let mut themes = Vec::new();
        for path in Self::find_user_themes() {
            if let Ok(theme) = ColorTheme::load_from_file(&path) {
                themes.push(theme);
            }
        }
        themes
    }

    /// Loads a theme from a specific path.
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<ColorTheme, ThemeError> {
        ColorTheme::load_from_file(path.as_ref())
    }

    /// Ensures the user theme directory exists.
    pub fn ensure_user_dir() -> std::io::Result<()> {
        if let Some(dir) = Self::user_theme_dir()
            && !dir.exists()
        {
            fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_theme_dir_path() {
        let dir = ThemeLoader::user_theme_dir();
        assert!(dir.is_some());
        let p = dir.unwrap();
        assert!(p.to_string_lossy().contains("omega"));
        assert!(p.to_string_lossy().contains("themes"));
    }
}
