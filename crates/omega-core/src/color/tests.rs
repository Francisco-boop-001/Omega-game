//! Integration tests for the color module.
//!
//! Tests the complete theme loading pipeline, from TOML parsing
//! to color resolution and validation.

#[cfg(test)]
mod color_tests {
    use crate::color::{ColorId, ColorTheme, EntityColorId, MonsterColorId, UiColorId};

    /// Helper to create a minimal valid theme for testing
    fn create_test_theme() -> ColorTheme {
        use crate::color::{ColorPalette, ColorRef, SemanticColors, ThemeMetadata};

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
                red: crate::color::HexColor::from_hex("#FF0000").unwrap(),
                green: crate::color::HexColor::from_hex("#00FF00").unwrap(),
                blue: crate::color::HexColor::from_hex("#0000FF").unwrap(),
                yellow: crate::color::HexColor::from_hex("#FFFF00").unwrap(),
                cyan: crate::color::HexColor::from_hex("#00FFFF").unwrap(),
                magenta: crate::color::HexColor::from_hex("#FF00FF").unwrap(),
                white: crate::color::HexColor::from_hex("#FFFFFF").unwrap(),
                black: crate::color::HexColor::from_hex("#000000").unwrap(),
                gray: crate::color::HexColor::from_hex("#808080").unwrap(),
                orange: Some(crate::color::HexColor::from_hex("#FFA500").unwrap()),
                purple: Some(crate::color::HexColor::from_hex("#9370DB").unwrap()),
                brown: Some(crate::color::HexColor::from_hex("#8B4513").unwrap()),
                dark_gray: Some(crate::color::HexColor::from_hex("#404040").unwrap()),
                light_gray: Some(crate::color::HexColor::from_hex("#C0C0C0").unwrap()),
            },
            semantic: SemanticColors {
                danger: ColorRef::Reference {
                    ref_path: "base.red".to_string(),
                },
                success: ColorRef::Reference {
                    ref_path: "base.green".to_string(),
                },
                info: ColorRef::Reference {
                    ref_path: "base.blue".to_string(),
                },
                warning: ColorRef::Reference {
                    ref_path: "base.yellow".to_string(),
                },
                magic: ColorRef::Reference {
                    ref_path: "base.purple".to_string(),
                },
                neutral: ColorRef::Reference {
                    ref_path: "base.gray".to_string(),
                },
            },
            entity: {
                let mut map = std::collections::HashMap::new();
                map.insert("player".to_string(), ColorRef::Reference { ref_path: "base.cyan".to_string() });
                map.insert("monster.hostileundead".to_string(), ColorRef::Reference { ref_path: "base.gray".to_string() });
                map.insert("monster.hostilebeast".to_string(), ColorRef::Reference { ref_path: "base.brown".to_string() });
                map.insert("monster.hostilehumanoid".to_string(), ColorRef::Reference { ref_path: "base.red".to_string() });
                map.insert("monster.hostilemagical".to_string(), ColorRef::Reference { ref_path: "base.purple".to_string() });
                map.insert("monster.hostileconstruct".to_string(), ColorRef::Reference { ref_path: "base.dark_gray".to_string() });
                map.insert("monster.hostiledragon".to_string(), ColorRef::Reference { ref_path: "base.orange".to_string() });
                map.insert("monster.neutral".to_string(), ColorRef::Reference { ref_path: "base.light_gray".to_string() });
                map.insert("monster.friendly".to_string(), ColorRef::Reference { ref_path: "base.green".to_string() });
                map.insert("item.common".to_string(), ColorRef::Reference { ref_path: "base.white".to_string() });
                map.insert("item.uncommon".to_string(), ColorRef::Reference { ref_path: "base.green".to_string() });
                map.insert("item.rare".to_string(), ColorRef::Reference { ref_path: "base.blue".to_string() });
                map.insert("item.epic".to_string(), ColorRef::Reference { ref_path: "base.purple".to_string() });
                map.insert("item.legendary".to_string(), ColorRef::Reference { ref_path: "base.yellow".to_string() });
                map.insert("terrain.wallstone".to_string(), ColorRef::Reference { ref_path: "base.gray".to_string() });
                map.insert("terrain.floorstone".to_string(), ColorRef::Reference { ref_path: "base.dark_gray".to_string() });
                map.insert("terrain.water".to_string(), ColorRef::Reference { ref_path: "base.blue".to_string() });
                map.insert("terrain.lava".to_string(), ColorRef::Reference { ref_path: "base.orange".to_string() });
                map.insert("terrain.door".to_string(), ColorRef::Reference { ref_path: "base.brown".to_string() });
                map.insert("terrain.stairsup".to_string(), ColorRef::Reference { ref_path: "base.white".to_string() });
                map
            },
            ui: {
                let mut map = std::collections::HashMap::new();
                map.insert("healthhigh".to_string(), ColorRef::Reference { ref_path: "base.green".to_string() });
                map.insert("healthmedium".to_string(), ColorRef::Reference { ref_path: "base.yellow".to_string() });
                map.insert("healthlow".to_string(), ColorRef::Reference { ref_path: "base.red".to_string() });
                map.insert("mana".to_string(), ColorRef::Reference { ref_path: "base.blue".to_string() });
                map.insert("highlight".to_string(), ColorRef::Reference { ref_path: "base.yellow".to_string() });
                map.insert("selection".to_string(), ColorRef::Reference { ref_path: "base.cyan".to_string() });
                map.insert("textdefault".to_string(), ColorRef::Reference { ref_path: "base.white".to_string() });
                map.insert("messageinfo".to_string(), ColorRef::Reference { ref_path: "base.blue".to_string() });
                map.insert("messagewarning".to_string(), ColorRef::Reference { ref_path: "base.yellow".to_string() });
                map.insert("messagedanger".to_string(), ColorRef::Reference { ref_path: "base.red".to_string() });
                map
            },
            effect: {
                let mut map = std::collections::HashMap::new();
                map.insert("fire".to_string(), ColorRef::Reference { ref_path: "base.red".to_string() });
                map.insert("ice".to_string(), ColorRef::Reference { ref_path: "base.cyan".to_string() });
                map.insert("lightning".to_string(), ColorRef::Reference { ref_path: "base.yellow".to_string() });
                map.insert("poison".to_string(), ColorRef::Reference { ref_path: "base.green".to_string() });
                map.insert("magicarcane".to_string(), ColorRef::Reference { ref_path: "base.purple".to_string() });
                map.insert("blood".to_string(), ColorRef::Reference { ref_path: "base.red".to_string() });
                map
            },
            animations: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_theme_validation_complete_theme() {
        let theme = create_test_theme();
        let result = theme.validate_strict();
        assert!(result.is_ok(), "Complete theme should pass validation");
    }

    #[test]
    fn test_theme_validation_missing_required_color() {
        let mut theme = create_test_theme();
        theme.entity.remove("player");
        let result = theme.validate_strict();
        assert!(result.is_err(), "Missing required color should fail validation");
    }

    #[test]
    fn test_theme_resolution_entity_colors() {
        let theme = create_test_theme();

        // Test player color resolution
        let player_id = ColorId::Entity(EntityColorId::Player);
        if let Some((fg, _bg)) = theme.resolve(&player_id) {
            assert_eq!(fg.to_rgb(), (0, 255, 255)); // Cyan
        } else {
            panic!("Player color should resolve");
        }

        // Test monster color resolution
        let undead_id = ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead));
        if let Some((fg, _bg)) = theme.resolve(&undead_id) {
            // Should resolve to base.gray
            assert_eq!(fg.to_rgb(), (128, 128, 128));
        } else {
            panic!("Undead color should resolve");
        }
    }

    #[test]
    fn test_theme_resolution_ui_colors() {
        let theme = create_test_theme();

        let health_high_id = ColorId::Ui(UiColorId::HealthHigh);
        if let Some((fg, _bg)) = theme.resolve(&health_high_id) {
            assert_eq!(fg.to_rgb(), (0, 255, 0)); // Green
        } else {
            panic!("Health high color should resolve");
        }
    }

    #[test]
    fn test_color_capability_none_mode() {
        use crate::color::{ColorCapability, ColorSpec};

        // Test None capability (NO_COLOR mode)
        let none_cap = ColorCapability::None;
        let color = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
        let adapted = none_cap.adapt(&color);

        // Should return white when in None mode
        match adapted {
            ColorSpec::Rgb { r, g, b } => {
                assert_eq!((r, g, b), (255, 255, 255));
            }
            _ => panic!("Should return RGB color"),
        }
    }

    #[test]
    fn test_color_spec_to_ansi256() {
        use crate::color::ColorSpec;

        // Red should map to ANSI 196
        let red = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
        assert_eq!(red.to_ansi256(), 196);

        // White should map to ANSI 255
        let white = ColorSpec::Rgb { r: 255, g: 255, b: 255 };
        assert_eq!(white.to_ansi256(), 255);

        // Black should map to ANSI 232
        let black = ColorSpec::Rgb { r: 0, g: 0, b: 0 };
        assert_eq!(black.to_ansi256(), 232);
    }

    #[test]
    fn test_color_spec_to_ansi16() {
        use crate::color::{AnsiColor, ColorSpec};

        // Bright red should map to ANSI bright red
        let red = ColorSpec::Rgb { r: 255, g: 0, b: 0 };
        assert_eq!(red.to_ansi16(), AnsiColor::BrightRed);

        // White should map to ANSI white
        let white = ColorSpec::Rgb { r: 255, g: 255, b: 255 };
        assert_eq!(white.to_ansi16(), AnsiColor::BrightWhite);
    }

    #[test]
    fn test_hex_color_edge_cases() {
        use crate::color::HexColor;

        // Test pure black
        let black = HexColor::from_rgb(0, 0, 0);
        assert_eq!(black.to_string(), "#000000");

        // Test pure white
        let white = HexColor::from_rgb(255, 255, 255);
        assert_eq!(white.to_string(), "#FFFFFF");

        // Test grayscale
        let gray = HexColor::from_rgb(128, 128, 128);
        assert_eq!(gray.to_rgb(), (128, 128, 128));
    }
}
