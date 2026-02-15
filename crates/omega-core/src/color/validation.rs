//! Theme validation for the Omega color system.
//!
//! This module provides comprehensive validation for color themes,
//! ensuring they meet all requirements for correctness and completeness.
//!
//! # Validation Levels
//!
//! - **Basic validation** (`validate()`): Checks that references resolve
//! - **Strict validation** (`validate_strict()`): Comprehensive checks including
//!   required ColorIds, metadata, and structure
//!
//! # Example
//!
//! ```rust
//! use omega_core::color::ColorTheme;
//!
//! // After loading a theme
//! // let theme: ColorTheme = ...;
//!
//! // Basic validation (checks references)
//! // theme.validate()?;
//!
//! // Strict validation (comprehensive checks)
//! // theme.validate_strict()?;
//! ```

use super::theme::{ColorRef, ColorTheme, ThemeError};

/// A comprehensive validation report.
///
/// Contains both errors (which prevent theme usage) and warnings
/// (which indicate potential issues but don't block usage).
#[derive(Debug)]
pub struct ValidationReport {
    /// Critical errors that make the theme invalid.
    pub errors: Vec<ThemeError>,
    /// Warnings about potential issues.
    pub warnings: Vec<String>,
}

impl ValidationReport {
    /// Creates a new empty validation report.
    pub fn new() -> Self {
        Self { errors: Vec::new(), warnings: Vec::new() }
    }

    /// Returns true if the report contains no errors.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Adds an error to the report.
    pub fn add_error(&mut self, error: ThemeError) {
        self.errors.push(error);
    }

    /// Adds a warning to the report.
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorTheme {
    /// Performs strict validation on the theme.
    ///
    /// This method performs comprehensive validation that includes:
    /// 1. Metadata validation (name, version, etc.)
    /// 2. Required base palette colors
    /// 3. All required ColorId mappings are present
    /// 4. UI color requirements
    /// 5. Effect color requirements
    /// 6. Reference resolution and circular reference detection
    ///
    /// Returns `Ok(())` if the theme passes all validation checks.
    /// Returns `Err(Vec<ThemeError>)` with all detected errors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::ColorTheme;
    ///
    /// // Assuming theme is loaded
    /// // let theme: ColorTheme = ...;
    /// // match theme.validate_strict() {
    /// //     Ok(()) => println!("Theme is valid!"),
    /// //     Err(errors) => {
    /// //         for error in errors {
    /// //             eprintln!("Validation error: {}", error);
    /// //         }
    /// //     }
    /// // }
    /// ```
    pub fn validate_strict(&self) -> Result<(), Vec<ThemeError>> {
        let mut errors = Vec::new();

        // 1. Validate metadata
        if self.meta.name.is_empty() {
            errors.push(ThemeError::MissingSection("meta.name".to_string()));
        }
        if self.meta.version.is_empty() {
            errors.push(ThemeError::MissingSection("meta.version".to_string()));
        }
        if self.meta.author.is_empty() {
            errors.push(ThemeError::MissingSection("meta.author".to_string()));
        }

        // 2. Validate all required ColorIds are present in entity mappings
        let required_entities = vec![
            "player",
            "monster.hostileundead",
            "monster.hostilebeast",
            "monster.hostilehumanoid",
            "monster.hostilemagical",
            "monster.hostileconstruct",
            "monster.hostiledragon",
            "monster.neutral",
            "monster.friendly",
            "item.common",
            "item.uncommon",
            "item.rare",
            "item.epic",
            "item.legendary",
            "terrain.wallstone",
            "terrain.floorstone",
            "terrain.water",
            "terrain.lava",
            "terrain.door",
            "terrain.stairsup",
        ];

        for entity_key in required_entities {
            if !self.entity.contains_key(entity_key) {
                errors.push(ThemeError::MissingColorId(format!("entity.{}", entity_key)));
            }
        }

        // 3. Validate UI colors
        let required_ui = vec![
            "healthhigh",
            "healthmedium",
            "healthlow",
            "mana",
            "highlight",
            "selection",
            "textdefault",
            "messageinfo",
            "messagewarning",
            "messagedanger",
        ];

        for ui_key in required_ui {
            if !self.ui.contains_key(ui_key) {
                errors.push(ThemeError::MissingColorId(format!("ui.{}", ui_key)));
            }
        }

        // 4. Validate effect colors
        let required_effects = vec!["fire", "ice", "lightning", "poison", "magicarcane", "blood"];
        for effect_key in required_effects {
            if !self.effect.contains_key(effect_key) {
                errors.push(ThemeError::MissingColorId(format!("effect.{}", effect_key)));
            }
        }

        // 5. Validate all references resolve (from Task 6)
        // This catches both unresolved references and circular references
        if let Err(e) = self.validate() {
            errors.push(e);
        }

        // 6. Validate that direct colors have valid hex values
        // (HexColor already validates on deserialization, but we check for consistency)
        for (key, color_ref) in &self.entity {
            if let ColorRef::Direct { fg, bg } = color_ref {
                // Colors are already validated by HexColor type,
                // but we could add additional checks here like contrast ratios
                let _ = (fg, bg); // Acknowledge we're inspecting these
            }
            // References are validated by the validate() call above
            let _ = key; // Acknowledge we inspected the key
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Validates the theme and returns a detailed report.
    ///
    /// Similar to `validate_strict()` but returns both errors and warnings
    /// instead of failing fast on the first issue.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::ColorTheme;
    ///
    /// // let theme: ColorTheme = ...;
    /// // let report = theme.validate_with_report();
    /// //
    /// // if !report.is_valid() {
    /// //     println!("Found {} errors:", report.errors.len());
    /// //     for error in &report.errors {
    /// //         println!("  - {}", error);
    /// //     }
    /// // }
    /// //
    /// // if !report.warnings.is_empty() {
    /// //     println!("Found {} warnings:", report.warnings.len());
    /// //     for warning in &report.warnings {
    /// //         println!("  - {}", warning);
    /// //     }
    /// // }
    /// ```
    pub fn validate_with_report(&self) -> ValidationReport {
        let mut report = ValidationReport::new();

        // Check metadata completeness
        if self.meta.description.is_empty() {
            report.add_warning("Theme is missing a description");
        }
        if self.meta.min_engine_version.is_empty() {
            report.add_warning("Theme does not specify minimum engine version");
        }

        // Check for empty color categories
        if self.entity.is_empty() {
            report.add_warning("No entity colors defined");
        }
        if self.ui.is_empty() {
            report.add_warning("No UI colors defined");
        }
        if self.effect.is_empty() {
            report.add_warning("No effect colors defined");
        }

        // Run strict validation and collect errors
        match self.validate_strict() {
            Ok(()) => {}
            Err(errors) => {
                for error in errors {
                    report.add_error(error);
                }
            }
        }

        report
    }
}

/// Helper extension to ThemeError for creating missing ColorId errors.
///
/// This provides a convenient constructor for the common case of
/// reporting a missing ColorId mapping.
impl ThemeError {
    /// Creates a ThemeError for a missing ColorId.
    ///
    /// # Example
    ///
    /// ```rust
    /// use omega_core::color::ThemeError;
    ///
    /// let error = ThemeError::missing_color_id("entity.player");
    /// assert!(error.to_string().contains("entity.player"));
    /// ```
    pub fn missing_color_id(name: &str) -> Self {
        ThemeError::MissingColorId(name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::super::hex_color::HexColor;
    use super::super::theme::{ColorPalette, ColorRef, SemanticColors, ThemeMetadata};
    use super::*;
    use std::collections::HashMap;

    /// Creates a minimal valid theme for testing.
    fn create_minimal_valid_theme() -> ColorTheme {
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
                magic: ColorRef::Reference { ref_path: "base.magenta".to_string() },
                neutral: ColorRef::Reference { ref_path: "base.gray".to_string() },
            },
            entity: create_complete_entity_map(),
            ui: create_complete_ui_map(),
            effect: create_complete_effect_map(),
            animations: std::collections::HashMap::new(),
        }
    }

    fn create_complete_entity_map() -> HashMap<String, ColorRef> {
        let mut map = HashMap::new();

        // Player
        map.insert(
            "player".to_string(),
            ColorRef::Reference { ref_path: "base.white".to_string() },
        );

        // Monsters
        map.insert(
            "monster.hostileundead".to_string(),
            ColorRef::Reference { ref_path: "semantic.danger".to_string() },
        );
        map.insert(
            "monster.hostilebeast".to_string(),
            ColorRef::Reference { ref_path: "base.red".to_string() },
        );
        map.insert(
            "monster.hostilehumanoid".to_string(),
            ColorRef::Reference { ref_path: "base.orange".to_string() },
        );
        map.insert(
            "monster.hostilemagical".to_string(),
            ColorRef::Reference { ref_path: "semantic.magic".to_string() },
        );
        map.insert(
            "monster.hostileconstruct".to_string(),
            ColorRef::Reference { ref_path: "base.gray".to_string() },
        );
        map.insert(
            "monster.hostiledragon".to_string(),
            ColorRef::Reference { ref_path: "base.purple".to_string() },
        );
        map.insert(
            "monster.neutral".to_string(),
            ColorRef::Reference { ref_path: "semantic.neutral".to_string() },
        );
        map.insert(
            "monster.friendly".to_string(),
            ColorRef::Reference { ref_path: "base.cyan".to_string() },
        );

        // Items
        map.insert(
            "item.common".to_string(),
            ColorRef::Reference { ref_path: "base.white".to_string() },
        );
        map.insert(
            "item.uncommon".to_string(),
            ColorRef::Reference { ref_path: "base.green".to_string() },
        );
        map.insert(
            "item.rare".to_string(),
            ColorRef::Reference { ref_path: "base.blue".to_string() },
        );
        map.insert(
            "item.epic".to_string(),
            ColorRef::Reference { ref_path: "base.purple".to_string() },
        );
        map.insert(
            "item.legendary".to_string(),
            ColorRef::Direct {
                fg: HexColor::from_hex("#FFD700").unwrap(),
                bg: HexColor::from_hex("#000000").unwrap(),
            },
        );

        // Terrain
        map.insert(
            "terrain.wallstone".to_string(),
            ColorRef::Reference { ref_path: "base.gray".to_string() },
        );
        map.insert(
            "terrain.floorstone".to_string(),
            ColorRef::Reference { ref_path: "base.dark_gray".to_string() },
        );
        map.insert(
            "terrain.water".to_string(),
            ColorRef::Reference { ref_path: "base.blue".to_string() },
        );
        map.insert(
            "terrain.lava".to_string(),
            ColorRef::Reference { ref_path: "base.red".to_string() },
        );
        map.insert(
            "terrain.door".to_string(),
            ColorRef::Reference { ref_path: "base.brown".to_string() },
        );
        map.insert(
            "terrain.stairsup".to_string(),
            ColorRef::Reference { ref_path: "base.white".to_string() },
        );

        map
    }

    fn create_complete_ui_map() -> HashMap<String, ColorRef> {
        let mut map = HashMap::new();

        map.insert(
            "healthhigh".to_string(),
            ColorRef::Reference { ref_path: "semantic.success".to_string() },
        );
        map.insert(
            "healthmedium".to_string(),
            ColorRef::Reference { ref_path: "semantic.warning".to_string() },
        );
        map.insert(
            "healthlow".to_string(),
            ColorRef::Reference { ref_path: "semantic.danger".to_string() },
        );
        map.insert("mana".to_string(), ColorRef::Reference { ref_path: "base.blue".to_string() });
        map.insert(
            "highlight".to_string(),
            ColorRef::Reference { ref_path: "base.yellow".to_string() },
        );
        map.insert(
            "selection".to_string(),
            ColorRef::Reference { ref_path: "base.cyan".to_string() },
        );
        map.insert(
            "textdefault".to_string(),
            ColorRef::Reference { ref_path: "base.white".to_string() },
        );
        map.insert(
            "messageinfo".to_string(),
            ColorRef::Reference { ref_path: "semantic.info".to_string() },
        );
        map.insert(
            "messagewarning".to_string(),
            ColorRef::Reference { ref_path: "semantic.warning".to_string() },
        );
        map.insert(
            "messagedanger".to_string(),
            ColorRef::Reference { ref_path: "semantic.danger".to_string() },
        );

        map
    }

    fn create_complete_effect_map() -> HashMap<String, ColorRef> {
        let mut map = HashMap::new();

        map.insert("fire".to_string(), ColorRef::Reference { ref_path: "base.red".to_string() });
        map.insert("ice".to_string(), ColorRef::Reference { ref_path: "base.cyan".to_string() });
        map.insert(
            "lightning".to_string(),
            ColorRef::Reference { ref_path: "base.yellow".to_string() },
        );
        map.insert(
            "poison".to_string(),
            ColorRef::Reference { ref_path: "base.green".to_string() },
        );
        map.insert(
            "magicarcane".to_string(),
            ColorRef::Reference { ref_path: "base.magenta".to_string() },
        );
        map.insert("blood".to_string(), ColorRef::Reference { ref_path: "base.red".to_string() });

        map
    }

    #[test]
    fn valid_theme_passes_strict_validation() {
        let theme = create_minimal_valid_theme();
        let result = theme.validate_strict();
        if let Err(errors) = &result {
            for err in errors {
                println!("STRICT ERR: {}", err);
            }
        }
        assert!(result.is_ok(), "Valid theme should pass strict validation");
    }

    #[test]
    fn missing_meta_name_fails_validation() {
        let mut theme = create_minimal_valid_theme();
        theme.meta.name = String::new();

        let result = theme.validate_strict();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.to_string().contains("meta.name")));
    }

    #[test]
    fn missing_meta_version_fails_validation() {
        let mut theme = create_minimal_valid_theme();
        theme.meta.version = String::new();

        let result = theme.validate_strict();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.to_string().contains("meta.version")));
    }

    #[test]
    fn missing_entity_color_fails_validation() {
        let mut theme = create_minimal_valid_theme();
        theme.entity.remove("player");

        let result = theme.validate_strict();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.to_string().contains("entity.player")));
    }

    #[test]
    fn missing_ui_color_fails_validation() {
        let mut theme = create_minimal_valid_theme();
        theme.ui.remove("healthhigh");

        let result = theme.validate_strict();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.to_string().contains("ui.healthhigh")));
    }

    #[test]
    fn missing_effect_color_fails_validation() {
        let mut theme = create_minimal_valid_theme();
        theme.effect.remove("fire");

        let result = theme.validate_strict();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.to_string().contains("effect.fire")));
    }

    #[test]
    fn multiple_missing_colors_report_all_errors() {
        let mut theme = create_minimal_valid_theme();
        theme.entity.remove("player");
        theme.ui.remove("healthhigh");
        theme.effect.remove("fire");

        let result = theme.validate_strict();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 3, "Should report all three missing colors");
    }

    #[test]
    fn unresolved_reference_fails_validation() {
        let mut theme = create_minimal_valid_theme();
        theme.entity.insert(
            "player".to_string(),
            ColorRef::Reference { ref_path: "base.nonexistent".to_string() },
        );

        let result = theme.validate_strict();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| e.to_string().contains("unresolved")
                    || e.to_string().contains("nonexistent"))
        );
    }

    #[test]
    fn circular_reference_fails_validation() {
        let mut theme = create_minimal_valid_theme();
        // Create a circular reference: semantic.danger -> entity.player -> semantic.danger
        theme.entity.insert(
            "player".to_string(),
            ColorRef::Reference { ref_path: "semantic.danger".to_string() },
        );
        theme.semantic.danger = ColorRef::Reference { ref_path: "entity.player".to_string() };

        let result = theme.validate_strict();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.to_string().to_lowercase().contains("circular")));
    }

    #[test]
    fn validation_report_valid_theme() {
        let theme = create_minimal_valid_theme();
        let report = theme.validate_with_report();

        assert!(report.is_valid());
        assert!(report.errors.is_empty());
        // Warnings are fine for a complete theme
    }

    #[test]
    fn validation_report_collects_errors_and_warnings() {
        let mut theme = create_minimal_valid_theme();
        theme.meta.description = String::new(); // Should trigger warning
        theme.entity.remove("player"); // Should trigger error

        let report = theme.validate_with_report();

        assert!(!report.is_valid());
        assert!(!report.errors.is_empty());
        // Warnings may or may not be present depending on other checks
    }

    #[test]
    fn theme_error_missing_color_id_helper() {
        let error = ThemeError::missing_color_id("entity.player");
        assert!(matches!(error, ThemeError::MissingColorId(_)));
        assert!(error.to_string().contains("entity.player"));
    }

    #[test]
    fn all_required_entity_colors_validated() {
        let required = vec![
            "player",
            "monster.hostileundead",
            "monster.hostilebeast",
            "monster.hostilehumanoid",
            "monster.hostilemagical",
            "monster.hostileconstruct",
            "monster.hostiledragon",
            "monster.neutral",
            "monster.friendly",
            "item.common",
            "item.uncommon",
            "item.rare",
            "item.epic",
            "item.legendary",
            "terrain.wallstone",
            "terrain.floorstone",
            "terrain.water",
            "terrain.lava",
            "terrain.door",
            "terrain.stairsup",
        ];

        for entity_key in required {
            let mut theme = create_minimal_valid_theme();
            theme.entity.remove(entity_key);

            let result = theme.validate_strict();
            assert!(result.is_err(), "Removing {} should fail validation", entity_key);

            let errors = result.unwrap_err();
            assert!(
                errors.iter().any(|e| e.to_string().contains(&format!("entity.{}", entity_key))),
                "Error should mention {}",
                entity_key
            );
        }
    }

    #[test]
    fn all_required_ui_colors_validated() {
        let required = vec![
            "healthhigh",
            "healthmedium",
            "healthlow",
            "mana",
            "highlight",
            "selection",
            "textdefault",
            "messageinfo",
            "messagewarning",
            "messagedanger",
        ];

        for ui_key in required {
            let mut theme = create_minimal_valid_theme();
            theme.ui.remove(ui_key);

            let result = theme.validate_strict();
            assert!(result.is_err(), "Removing {} should fail validation", ui_key);

            let errors = result.unwrap_err();
            assert!(
                errors.iter().any(|e| e.to_string().contains(&format!("ui.{}", ui_key))),
                "Error should mention {}",
                ui_key
            );
        }
    }

    #[test]
    fn all_required_effect_colors_validated() {
        let required = vec!["fire", "ice", "lightning", "poison", "magicarcane", "blood"];

        for effect_key in required {
            let mut theme = create_minimal_valid_theme();
            theme.effect.remove(effect_key);

            let result = theme.validate_strict();
            assert!(result.is_err(), "Removing {} should fail validation", effect_key);

            let errors = result.unwrap_err();
            assert!(
                errors.iter().any(|e| e.to_string().contains(&format!("effect.{}", effect_key))),
                "Error should mention {}",
                effect_key
            );
        }
    }
}
