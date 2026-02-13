use bevy::prelude::*;

/// UI Layout configuration for spacing, sizing, and timing values.
///
/// This resource contains all non-color UI constants that were previously
/// part of ThemeTokens. These values control layout, typography sizing,
/// and animation timing, and are independent of the active color theme.
///
/// # Migration from ThemeTokens
///
/// Previously, ThemeTokens contained both colors and layout values.
/// With the introduction of BevyTheme for semantic color support,
/// ThemeTokens has been split:
/// - **BevyTheme**: All colors (semantic, from omega-core themes)
/// - **UiLayoutTokens**: All spacing/sizing/timing (UI-specific)
/// - **UI Chrome Colors**: Hardcoded in scene.rs (UI structural colors)
#[derive(Resource, Clone)]
pub struct UiLayoutTokens {
    pub spacing_xs: f32,
    pub spacing_sm: f32,
    pub spacing_md: f32,
    pub spacing_lg: f32,
    pub spacing_xl: f32,
    pub panel_radius_sm: f32,
    pub panel_radius: f32,
    pub map_font_size: f32,
    pub panel_title_font_size: f32,
    pub panel_body_font_size: f32,
    pub panel_body_small_font_size: f32,
    pub motion_fast: f32,
    pub motion_slow: f32,
}

impl Default for UiLayoutTokens {
    fn default() -> Self {
        Self {
            spacing_xs: 4.0,
            spacing_sm: 8.0,
            spacing_md: 16.0,
            spacing_lg: 24.0,
            spacing_xl: 32.0,
            panel_radius_sm: 8.0,
            panel_radius: 12.0,
            map_font_size: 18.0,
            panel_title_font_size: 17.0,
            panel_body_font_size: 14.0,
            panel_body_small_font_size: 13.0,
            motion_fast: 0.16,
            motion_slow: 0.55,
        }
    }
}

/// UI Chrome colors - structural colors for panels and backgrounds.
///
/// These are UI-specific colors that don't map to semantic game colors
/// from omega-core. They define the "chrome" - the UI framework itself.
/// These values are hardcoded and don't change with theme selection.
#[derive(Resource, Clone)]
pub struct UiChromeColors {
    pub background_ink: Color,
    pub background_haze: Color,
    pub background_noise: Color,
    pub panel_brass: Color,
    pub panel_surface: Color,
    pub panel_surface_alt: Color,
    pub panel_surface_focus: Color,
    pub panel_surface_depth: Color,
    pub panel_border: Color,
    pub map_backdrop: Color,
    pub map_frame: Color,
    pub glyph_backdrop: Color,
    pub text_focus: Color,
    pub focus_ring: Color,
    pub objective_glow: Color,
    pub objective_route: Color,
    pub objective_halo_calm: Color,
    pub objective_halo_hot: Color,
    pub accent_law: Color,
    pub accent_chaos: Color,
}

impl Default for UiChromeColors {
    fn default() -> Self {
        Self {
            background_ink: Color::srgb(0.05, 0.05, 0.08),
            background_haze: Color::srgb(0.08, 0.10, 0.13),
            background_noise: Color::srgba(0.82, 0.76, 0.62, 0.04),
            panel_brass: Color::srgb(0.22, 0.18, 0.11),
            panel_surface: Color::srgb(0.12, 0.13, 0.16),
            panel_surface_alt: Color::srgb(0.14, 0.15, 0.19),
            panel_surface_focus: Color::srgb(0.19, 0.17, 0.12),
            panel_surface_depth: Color::srgb(0.09, 0.10, 0.14),
            panel_border: Color::srgb(0.36, 0.31, 0.19),
            map_backdrop: Color::srgb(0.08, 0.10, 0.13),
            map_frame: Color::srgb(0.11, 0.12, 0.16),
            glyph_backdrop: Color::srgb(0.06, 0.07, 0.10),
            text_focus: Color::srgb(0.99, 0.97, 0.92),
            focus_ring: Color::srgb(0.94, 0.84, 0.52),
            objective_glow: Color::srgb(0.50, 0.88, 0.98),
            objective_route: Color::srgb(0.70, 0.90, 0.96),
            objective_halo_calm: Color::srgb(0.52, 0.74, 0.94),
            objective_halo_hot: Color::srgb(0.94, 0.66, 0.36),
            accent_law: Color::srgb(0.54, 0.82, 0.97),
            accent_chaos: Color::srgb(0.98, 0.52, 0.36),
        }
    }
}
