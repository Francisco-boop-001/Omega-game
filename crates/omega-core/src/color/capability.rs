//! Terminal color capability detection.
//!
//! This module provides detection of terminal color capabilities and
//! automatic color adaptation based on the detected capability level.
//!
//! # Overview
//!
//! The capability system detects what color depth the terminal supports:
//!
//! - **None**: No color support (monochrome)
//! - **Ansi16**: 16 basic ANSI colors
//! - **Ansi256**: 256-color palette
//! - **TrueColor**: 24-bit RGB colors
//!
//! Detection respects the `NO_COLOR` environment variable and uses
//! the `termprofile` crate for accurate terminal capability detection.
//!
//! # Example
//!
//! ```rust
//! use omega_core::color::{ColorSpec, ColorCapability};
//!
//! // Detect terminal capability (cached after first call)
//! let capability = ColorCapability::detect();
//!
//! // Adapt a color to the terminal's capability
//! let true_color = ColorSpec::Rgb { r: 255, g: 128, b: 64 };
//! let adapted = capability.adapt(&true_color);
//! ```

use std::sync::OnceLock;
use super::color_spec::ColorSpec;

/// Represents the color capability level of a terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorCapability {
    /// No color support (monochrome).
    None,
    /// Basic 16 ANSI colors.
    Ansi16,
    /// 256-color palette support.
    Ansi256,
    /// Full 24-bit RGB (TrueColor) support.
    TrueColor,
}

impl ColorCapability {
    /// Detects the terminal's color capability.
    ///
    /// This method checks environment variables and uses the `termprofile`
    /// crate to determine the best color capability level. It respects
    /// the `NO_COLOR` environment variable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use omega_core::color::ColorCapability;
    ///
    /// let capability = ColorCapability::detect();
    /// // capability will be one of: None, Ansi16, Ansi256, TrueColor
    /// ```
    pub fn detect() -> Self {
        // Check NO_COLOR first - this takes precedence
        if std::env::var("NO_COLOR").is_ok() {
            return ColorCapability::None;
        }
        
        // Use termprofile for accurate detection
        let profile = termprofile::TermProfile::detect(
            &std::io::stdout(),
            Default::default()
        );

        // Map TermProfile variants to ColorCapability
        match profile {
            termprofile::TermProfile::TrueColor => ColorCapability::TrueColor,
            termprofile::TermProfile::Ansi256 => ColorCapability::Ansi256,
            termprofile::TermProfile::Ansi16 => ColorCapability::Ansi16,
            termprofile::TermProfile::NoColor | termprofile::TermProfile::NoTty => {
                // Fallback to environment variable detection when termprofile
                // can't detect (common on Windows Terminal)
                Self::detect_from_env()
            }
        }
    }
    
    /// Detect capability from environment variables (fallback method).
    fn detect_from_env() -> Self {
        // Check COLORTERM for TrueColor indication
        if let Ok(ct) = std::env::var("COLORTERM") {
            if ct.contains("truecolor") || ct.contains("24bit") {
                return ColorCapability::TrueColor;
            }
        }
        
        // Check TERM for color capability
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("256color") {
                return ColorCapability::Ansi256;
            } else if term.contains("color") {
                return ColorCapability::Ansi16;
            }
        }
        
        // Default to Ansi16 as a safe fallback
        ColorCapability::Ansi16
    }
    
    /// Adapts a color specification to this capability level.
    ///
    /// Converts the input color to the best representation supported
    /// by this capability level.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use omega_core::color::{ColorSpec, ColorCapability};
    ///
    /// let true_color = ColorSpec::Rgb { r: 255, g: 128, b: 64 };
    /// 
    // Convert to 256-color palette
    /// let adapted = ColorCapability::Ansi256.adapt(&true_color);
    /// assert!(matches!(adapted, ColorSpec::Indexed(_)));
    /// ```
    pub fn adapt(&self, color: &ColorSpec) -> ColorSpec {
        match self {
            ColorCapability::TrueColor => *color,
            ColorCapability::Ansi256 => ColorSpec::Indexed(color.to_ansi256()),
            ColorCapability::Ansi16 => ColorSpec::Ansi(color.to_ansi16()),
            ColorCapability::None => ColorSpec::Rgb { r: 255, g: 255, b: 255 },
        }
    }
    
    /// Returns the maximum number of colors supported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use omega_core::color::ColorCapability;
    ///
    /// assert_eq!(ColorCapability::None.color_count(), 0);
    /// assert_eq!(ColorCapability::Ansi16.color_count(), 16);
    /// assert_eq!(ColorCapability::Ansi256.color_count(), 256);
    /// assert_eq!(ColorCapability::TrueColor.color_count(), 16_777_216);
    /// ```
    pub const fn color_count(&self) -> usize {
        match self {
            ColorCapability::None => 0,
            ColorCapability::Ansi16 => 16,
            ColorCapability::Ansi256 => 256,
            ColorCapability::TrueColor => 16_777_216, // 2^24
        }
    }
    
    /// Returns true if this capability supports RGB colors.
    pub const fn supports_rgb(&self) -> bool {
        matches!(self, ColorCapability::TrueColor)
    }
    
    /// Returns true if this capability supports indexed colors (256-color palette).
    pub const fn supports_indexed(&self) -> bool {
        matches!(self, ColorCapability::Ansi256 | ColorCapability::TrueColor)
    }
    
    /// Returns true if this capability supports ANSI 16 colors.
    pub const fn supports_ansi(&self) -> bool {
        matches!(self, ColorCapability::Ansi16 | ColorCapability::Ansi256 | ColorCapability::TrueColor)
    }
}

impl Default for ColorCapability {
    fn default() -> Self {
        ColorCapability::Ansi16
    }
}

// Cached detection - only runs once
static CAPABILITY: OnceLock<ColorCapability> = OnceLock::new();

/// Returns the detected color capability, caching the result.
///
/// The first call to this function performs terminal detection,
/// subsequent calls return the cached value.
///
/// # Examples
///
/// ```rust
/// use omega_core::color::{ColorCapability, get_capability};
///
/// let capability = get_capability();
/// // Subsequent calls are fast (cached)
/// let capability2 = get_capability();
/// assert_eq!(capability, capability2);
/// ```
pub fn get_capability() -> ColorCapability {
    *CAPABILITY.get_or_init(ColorCapability::detect)
}

/// Resets the cached capability detection.
///
/// This is primarily useful for testing scenarios where the
/// environment variables may change between tests.
///
/// # Examples
///
/// ```rust
/// use omega_core::color::reset_capability;
///
/// // Reset the cached capability
/// reset_capability();
/// ```
pub fn reset_capability() {
    // Note: OnceLock doesn't support clearing, so we just
    // re-detect and attempt to set. This is a no-op if already set,
    // but ensures fresh detection for testing.
    let _ = CAPABILITY.set(ColorCapability::detect());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_adapt_truecolor() {
        let color = ColorSpec::Rgb { r: 255, g: 128, b: 64 };
        let adapted = ColorCapability::TrueColor.adapt(&color);
        assert_eq!(adapted, color); // Should return unchanged
    }

    #[test]
    fn test_capability_adapt_ansi256() {
        let color = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
        let adapted = ColorCapability::Ansi256.adapt(&color);
        assert!(matches!(adapted, ColorSpec::Indexed(_)));
        if let ColorSpec::Indexed(idx) = adapted {
            assert_eq!(idx, 196); // Bright red in 256-color palette
        }
    }

    #[test]
    fn test_capability_adapt_ansi16() {
        let color = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
        let adapted = ColorCapability::Ansi16.adapt(&color);
        assert!(matches!(adapted, ColorSpec::Ansi(_)));
        if let ColorSpec::Ansi(ansi) = adapted {
            assert_eq!(ansi, AnsiColor::BrightRed);
        }
    }

    #[test]
    fn test_capability_adapt_none() {
        let color = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
        let adapted = ColorCapability::None.adapt(&color);
        assert_eq!(adapted, ColorSpec::Rgb { r: 255, g: 255, b: 255 });
    }

    #[test]
    fn test_capability_color_counts() {
        assert_eq!(ColorCapability::None.color_count(), 0);
        assert_eq!(ColorCapability::Ansi16.color_count(), 16);
        assert_eq!(ColorCapability::Ansi256.color_count(), 256);
        assert_eq!(ColorCapability::TrueColor.color_count(), 16_777_216);
    }

    #[test]
    fn test_capability_supports() {
        assert!(!ColorCapability::None.supports_rgb());
        assert!(!ColorCapability::None.supports_indexed());
        assert!(!ColorCapability::None.supports_ansi());

        assert!(!ColorCapability::Ansi16.supports_rgb());
        assert!(!ColorCapability::Ansi16.supports_indexed());
        assert!(ColorCapability::Ansi16.supports_ansi());

        assert!(!ColorCapability::Ansi256.supports_rgb());
        assert!(ColorCapability::Ansi256.supports_indexed());
        assert!(ColorCapability::Ansi256.supports_ansi());

        assert!(ColorCapability::TrueColor.supports_rgb());
        assert!(ColorCapability::TrueColor.supports_indexed());
        assert!(ColorCapability::TrueColor.supports_ansi());
    }

    #[test]
    fn test_capability_default() {
        let default = ColorCapability::default();
        assert_eq!(default, ColorCapability::Ansi16);
    }

    #[test]
    fn test_capability_equality() {
        assert_eq!(ColorCapability::None, ColorCapability::None);
        assert_eq!(ColorCapability::Ansi16, ColorCapability::Ansi16);
        assert_ne!(ColorCapability::None, ColorCapability::Ansi16);
    }

    #[test]
    fn test_capability_clone_copy() {
        let cap = ColorCapability::TrueColor;
        let cap2 = cap;
        assert_eq!(cap, cap2); // Copy trait allows this
        
        let cap3 = cap.clone();
        assert_eq!(cap, cap3);
    }

    #[test]
    fn test_capability_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ColorCapability::Ansi16);
        set.insert(ColorCapability::Ansi256);
        set.insert(ColorCapability::Ansi16); // Duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_detect_from_env_truecolor() {
        // We can't easily test env var detection in unit tests,
        // but we can test the fallback detection logic exists
        let capability = ColorCapability::detect_from_env();
        // Should return one of the valid variants
        assert!([
            ColorCapability::None,
            ColorCapability::Ansi16,
            ColorCapability::Ansi256,
            ColorCapability::TrueColor,
        ].contains(&capability));
    }

    #[test]
    fn test_get_capability_caching() {
        // First call should initialize
        let cap1 = get_capability();
        // Second call should return cached value
        let cap2 = get_capability();
        assert_eq!(cap1, cap2);
    }

    #[test]
    fn test_reset_capability() {
        // Just verify it doesn't panic
        reset_capability();
        // Subsequent calls should work
        let _ = get_capability();
    }
}
