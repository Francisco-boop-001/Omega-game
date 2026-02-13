use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

use crate::{AppState, FrontendRuntime, InputAction, RenderFrame, RuntimeFrame, RuntimeStatus};
use omega_core::GameMode;
use std::env;

/// Event for requesting a theme change at runtime.
///
/// Send this event to switch the active theme. The theme name must match
/// a built-in theme ("classic" or "accessible").
#[derive(Event, Debug, Clone)]
pub struct ThemeChangeEvent {
    pub theme_name: String,
}

/// Resource tracking the currently active theme name.
#[derive(Resource, Debug, Clone)]
struct ActiveThemeName(String);

impl Default for ActiveThemeName {
    fn default() -> Self {
        Self("classic".to_string())
    }
}

pub mod animation;
pub mod bevy_theme;
pub mod color_adapter;
pub mod hud;
pub mod input;
pub mod interaction;
pub mod overlays;
pub mod scene;
pub mod theme;
pub mod tilemap;
pub mod timeline;

// Re-export key types for convenience
pub use bevy_theme::BevyTheme;

#[derive(Component)]
pub struct MapPanelText;

#[derive(Component)]
pub struct CompassPanelText;

#[derive(Component)]
pub struct HudPanelText;

#[derive(Component)]
pub struct InteractionPanelText;

#[derive(Component)]
pub struct TimelinePanelText;

#[derive(Component)]
pub struct MapPanelCard;

#[derive(Component)]
pub struct CompassPanelCard;

#[derive(Component)]
pub struct StatusPanelCard;

#[derive(Component)]
pub struct InteractionPanelCard;

#[derive(Component)]
pub struct TimelinePanelCard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiPanelFocus {
    #[default]
    Status,
    Map,
    Compass,
    Interaction,
    Timeline,
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct UiFocusState {
    pub active_panel: UiPanelFocus,
    pub urgency: f32,
}

impl Default for UiFocusState {
    fn default() -> Self {
        Self { active_panel: UiPanelFocus::Status, urgency: 0.35 }
    }
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct UiReadabilityConfig {
    pub scale: f32,
    pub high_contrast: bool,
    pub reduced_motion: bool,
}

impl Default for UiReadabilityConfig {
    fn default() -> Self {
        let scale = env::var("OMEGA_UI_SCALE")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(1.0)
            .clamp(0.85, 1.35);
        let high_contrast = env_flag("OMEGA_UI_HIGH_CONTRAST");
        let reduced_motion = env_flag("OMEGA_UI_REDUCED_MOTION");
        Self { scale, high_contrast, reduced_motion }
    }
}

fn env_flag(name: &str) -> bool {
    env::var(name)
        .ok()
        .map(|value| {
            matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on")
        })
        .unwrap_or(false)
}

#[derive(Resource, Default)]
struct UiBootLatch {
    started_session: bool,
}

pub struct ArcaneCartographerPlugin;

impl Plugin for ArcaneCartographerPlugin {
    fn build(&self, app: &mut App) {
        // Load classic theme as default
        let color_theme = color_adapter::load_builtin_theme("classic")
            .expect("Failed to load classic theme - this should never happen with embedded themes");
        let bevy_theme = BevyTheme::new(color_theme);

        app.insert_resource(bevy_theme)
            .insert_resource(theme::ThemeTokens::default())
            .insert_resource(UiReadabilityConfig::default())
            .insert_resource(animation::UiMotionState::default())
            .insert_resource(UiFocusState::default())
            .insert_resource(UiBootLatch::default())
            .insert_resource(ActiveThemeName::default())
            .add_event::<ThemeChangeEvent>()
            .add_systems(Startup, scene::setup_arcane_scene)
            .add_systems(
                Update,
                (
                    animation::advance_ui_motion,
                    input::keyboard_to_runtime_input,
                    handle_theme_change_events,
                    handle_theme_cycle_key,
                    ensure_session_started,
                    update_ui_panels,
                    apply_focus_styles,
                )
                    .chain(),
            );
    }
}

fn ensure_session_started(mut runtime: ResMut<FrontendRuntime>, mut boot: ResMut<UiBootLatch>) {
    if boot.started_session {
        return;
    }
    if runtime.0.app_state == AppState::Menu && runtime.0.session.is_none() {
        runtime.0.apply_action(InputAction::StartGame);
        boot.started_session = true;
    }
}

fn update_ui_panels(
    status: Res<RuntimeStatus>,
    frame: Res<RuntimeFrame>,
    motion: Res<animation::UiMotionState>,
    mut focus: ResMut<UiFocusState>,
    mut text_queries: ParamSet<(
        Query<
            &mut Text,
            (
                With<MapPanelText>,
                Without<CompassPanelText>,
                Without<HudPanelText>,
                Without<InteractionPanelText>,
                Without<TimelinePanelText>,
            ),
        >,
        Query<
            &mut Text,
            (
                With<CompassPanelText>,
                Without<MapPanelText>,
                Without<HudPanelText>,
                Without<InteractionPanelText>,
                Without<TimelinePanelText>,
            ),
        >,
        Query<
            &mut Text,
            (
                With<HudPanelText>,
                Without<MapPanelText>,
                Without<CompassPanelText>,
                Without<InteractionPanelText>,
                Without<TimelinePanelText>,
            ),
        >,
        Query<
            &mut Text,
            (
                With<InteractionPanelText>,
                Without<MapPanelText>,
                Without<CompassPanelText>,
                Without<HudPanelText>,
                Without<TimelinePanelText>,
            ),
        >,
        Query<
            &mut Text,
            (
                With<TimelinePanelText>,
                Without<MapPanelText>,
                Without<CompassPanelText>,
                Without<HudPanelText>,
                Without<InteractionPanelText>,
            ),
        >,
    )>,
) {
    let fallback = RenderFrame {
        mode: GameMode::Classic,
        bounds: (1, 1),
        tiles: Vec::new(),
        hud_lines: vec!["Loading runtime diagnostics...".to_string()],
        interaction_lines: vec!["Booting interaction systems...".to_string()],
        timeline_lines: vec!["Waiting for first outcome event...".to_string()],
        event_lines: vec!["Waiting for first outcome event...".to_string()],
    };
    let frame_ref = frame.frame.as_ref().unwrap_or(&fallback);

    let map_lines = tilemap::compose_map_lines(frame_ref, motion.frame);
    if let Ok(mut text) = text_queries.p0().get_single_mut() {
        *text = Text::new(map_lines.join("\n"));
    }

    let compass_lines = overlays::compose_compass_lines(frame_ref, motion.frame);
    if let Ok(mut text) = text_queries.p1().get_single_mut() {
        *text = Text::new(compass_lines.join("\n"));
    }

    let hud_lines = hud::compose_hud_lines(frame_ref, status.app_state);
    if let Ok(mut text) = text_queries.p2().get_single_mut() {
        *text = Text::new(hud_lines.join("\n"));
    }

    let interaction_lines = interaction::compose_interaction_lines(frame_ref);
    if let Ok(mut text) = text_queries.p3().get_single_mut() {
        *text = Text::new(interaction_lines.join("\n"));
    }

    let timeline_lines = timeline::compose_timeline_lines(frame_ref);
    if let Ok(mut text) = text_queries.p4().get_single_mut() {
        *text = Text::new(timeline_lines.join("\n"));
    }

    *focus = derive_focus_state(frame_ref);
}

fn derive_focus_state(frame: &RenderFrame) -> UiFocusState {
    if !frame.interaction_lines.is_empty() {
        let urgency = if frame
            .interaction_lines
            .iter()
            .any(|line| line.starts_with("ACTIVE:") || line.contains("Targeting"))
        {
            1.0
        } else {
            0.8
        };
        return UiFocusState { active_panel: UiPanelFocus::Interaction, urgency };
    }
    if frame.tiles.iter().any(|tile| {
        matches!(tile.kind, crate::TileKind::TargetCursor | crate::TileKind::ProjectileImpact)
    }) {
        return UiFocusState { active_panel: UiPanelFocus::Map, urgency: 0.85 };
    }
    if frame.tiles.iter().any(|tile| tile.kind == crate::TileKind::ObjectiveMarker) {
        return UiFocusState { active_panel: UiPanelFocus::Compass, urgency: 0.7 };
    }
    if !frame.timeline_lines.is_empty() {
        return UiFocusState { active_panel: UiPanelFocus::Timeline, urgency: 0.55 };
    }
    UiFocusState::default()
}

fn blend_color(left: Color, right: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let left = left.to_srgba();
    let right = right.to_srgba();
    Color::srgba(
        left.red + (right.red - left.red) * t,
        left.green + (right.green - left.green) * t,
        left.blue + (right.blue - left.blue) * t,
        left.alpha + (right.alpha - left.alpha) * t,
    )
}

fn apply_focus_styles(
    theme: Res<theme::ThemeTokens>,
    bevy_theme: Res<BevyTheme>,
    focus: Res<UiFocusState>,
    motion: Res<animation::UiMotionState>,
    readability: Res<UiReadabilityConfig>,
    mut card_queries: ParamSet<(
        Query<(&mut BackgroundColor, &mut BorderColor), With<MapPanelCard>>,
        Query<(&mut BackgroundColor, &mut BorderColor), With<CompassPanelCard>>,
        Query<(&mut BackgroundColor, &mut BorderColor), With<StatusPanelCard>>,
        Query<(&mut BackgroundColor, &mut BorderColor), With<InteractionPanelCard>>,
        Query<(&mut BackgroundColor, &mut BorderColor), With<TimelinePanelCard>>,
    )>,
) {
    let pulse = if readability.reduced_motion { 0.0 } else { motion.pulse01 };
    let intensity = (0.45 + focus.urgency * 0.4 + pulse * 0.15).clamp(0.0, 1.0);
    let base_border = if readability.high_contrast { theme.text_focus } else { theme.panel_border };
    let highlight_color = bevy_theme.get_ui_highlight();

    if let Ok((mut background, mut border)) = card_queries.p0().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Map {
            blend_color(theme.map_frame, theme.objective_halo_calm, intensity * 0.35)
        } else {
            theme.map_frame
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Map {
            blend_color(base_border, highlight_color, intensity)
        } else {
            base_border
        });
    }

    if let Ok((mut background, mut border)) = card_queries.p1().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Compass {
            blend_color(theme.panel_surface_alt, theme.objective_halo_calm, intensity * 0.45)
        } else {
            theme.panel_surface_alt
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Compass {
            blend_color(base_border, highlight_color, intensity)
        } else {
            base_border
        });
    }

    if let Ok((mut background, mut border)) = card_queries.p2().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Status {
            blend_color(theme.panel_surface, theme.panel_surface_focus, intensity * 0.35)
        } else {
            theme.panel_surface
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Status {
            blend_color(base_border, highlight_color, intensity * 0.75)
        } else {
            base_border
        });
    }

    if let Ok((mut background, mut border)) = card_queries.p3().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Interaction {
            blend_color(theme.panel_surface_focus, theme.accent_chaos, intensity * 0.3)
        } else {
            theme.panel_surface_focus
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Interaction {
            blend_color(base_border, highlight_color, intensity)
        } else {
            base_border
        });
    }

    if let Ok((mut background, mut border)) = card_queries.p4().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Timeline {
            blend_color(theme.panel_brass, theme.panel_surface_focus, intensity * 0.28)
        } else {
            theme.panel_brass
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Timeline {
            blend_color(base_border, highlight_color, intensity * 0.7)
        } else {
            base_border
        });
    }
}

/// System that handles `ThemeChangeEvent` and updates the `BevyTheme` resource.
///
/// When a theme change event is received, this system:
/// 1. Loads the requested theme by name (must be "classic" or "accessible")
/// 2. Creates a new BevyTheme from the loaded ColorTheme
/// 3. Replaces the BevyTheme resource, which triggers UI updates on next frame
fn handle_theme_change_events(
    mut events: EventReader<ThemeChangeEvent>,
    mut bevy_theme: ResMut<BevyTheme>,
    mut active_theme_name: ResMut<ActiveThemeName>,
) {
    for event in events.read() {
        match color_adapter::load_builtin_theme(&event.theme_name) {
            Ok(color_theme) => {
                *bevy_theme = BevyTheme::new(color_theme);
                active_theme_name.0 = event.theme_name.clone();
                info!("Theme changed to: {}", event.theme_name);
            }
            Err(err) => {
                error!("Failed to load theme '{}': {}", event.theme_name, err);
            }
        }
    }
}

/// System that listens for F5 key press and cycles between built-in themes.
///
/// Cycles through themes in this order: Classic → Accessible → Classic
/// This is a debug/accessibility feature for runtime theme testing.
fn handle_theme_cycle_key(
    keys: Res<ButtonInput<KeyCode>>,
    active_theme_name: Res<ActiveThemeName>,
    mut theme_events: EventWriter<ThemeChangeEvent>,
) {
    if keys.just_pressed(KeyCode::F5) {
        let next_theme = match active_theme_name.0.as_str() {
            "classic" => "accessible",
            "accessible" => "classic",
            _ => "classic", // Fallback to classic if unknown
        };
        theme_events.send(ThemeChangeEvent { theme_name: next_theme.to_string() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arcane_cartographer_plugin_inserts_bevy_theme() {
        let mut app = bevy::prelude::App::new();
        app.add_plugins(ArcaneCartographerPlugin);

        // Verify BevyTheme resource exists
        let theme = app.world().get_resource::<BevyTheme>();
        assert!(theme.is_some(), "BevyTheme should be inserted by plugin");

        // Verify it's the classic theme
        let theme = theme.unwrap();
        assert_eq!(theme.theme().meta.name, "Classic");
    }

    #[test]
    fn arcane_cartographer_plugin_inserts_all_resources() {
        let mut app = bevy::prelude::App::new();
        app.add_plugins(ArcaneCartographerPlugin);

        // Verify all expected resources exist
        assert!(
            app.world().get_resource::<BevyTheme>().is_some(),
            "BevyTheme should be inserted"
        );
        assert!(
            app.world().get_resource::<theme::ThemeTokens>().is_some(),
            "ThemeTokens should be inserted"
        );
        assert!(
            app.world().get_resource::<UiReadabilityConfig>().is_some(),
            "UiReadabilityConfig should be inserted"
        );
        assert!(
            app.world().get_resource::<animation::UiMotionState>().is_some(),
            "UiMotionState should be inserted"
        );
        assert!(
            app.world().get_resource::<UiFocusState>().is_some(),
            "UiFocusState should be inserted"
        );
    }

    #[test]
    fn derive_focus_state_identifies_interaction_panel() {
        let frame = RenderFrame {
            mode: GameMode::Classic,
            bounds: (10, 10),
            tiles: Vec::new(),
            hud_lines: Vec::new(),
            interaction_lines: vec!["ACTIVE: Choose target".to_string()],
            timeline_lines: Vec::new(),
            event_lines: Vec::new(),
        };

        let focus = derive_focus_state(&frame);
        assert_eq!(focus.active_panel, UiPanelFocus::Interaction);
        assert_eq!(focus.urgency, 1.0);
    }

    #[test]
    fn derive_focus_state_identifies_map_panel() {
        let frame = RenderFrame {
            mode: GameMode::Classic,
            bounds: (10, 10),
            tiles: vec![crate::RenderTile {
                x: 5,
                y: 5,
                ch: '+',
                kind: crate::TileKind::TargetCursor,
                color: crate::RenderTileColor(Color::WHITE),
            }],
            hud_lines: Vec::new(),
            interaction_lines: Vec::new(),
            timeline_lines: Vec::new(),
            event_lines: Vec::new(),
        };

        let focus = derive_focus_state(&frame);
        assert_eq!(focus.active_panel, UiPanelFocus::Map);
        assert_eq!(focus.urgency, 0.85);
    }

    #[test]
    fn derive_focus_state_identifies_compass_panel() {
        let frame = RenderFrame {
            mode: GameMode::Classic,
            bounds: (10, 10),
            tiles: vec![crate::RenderTile {
                x: 5,
                y: 5,
                ch: 'O',
                kind: crate::TileKind::ObjectiveMarker,
                color: crate::RenderTileColor(Color::YELLOW),
            }],
            hud_lines: Vec::new(),
            interaction_lines: Vec::new(),
            timeline_lines: Vec::new(),
            event_lines: Vec::new(),
        };

        let focus = derive_focus_state(&frame);
        assert_eq!(focus.active_panel, UiPanelFocus::Compass);
        assert_eq!(focus.urgency, 0.7);
    }

    #[test]
    fn derive_focus_state_identifies_timeline_panel() {
        let frame = RenderFrame {
            mode: GameMode::Classic,
            bounds: (10, 10),
            tiles: Vec::new(),
            hud_lines: Vec::new(),
            interaction_lines: Vec::new(),
            timeline_lines: vec!["Player attacked goblin".to_string()],
            event_lines: Vec::new(),
        };

        let focus = derive_focus_state(&frame);
        assert_eq!(focus.active_panel, UiPanelFocus::Timeline);
        assert_eq!(focus.urgency, 0.55);
    }

    #[test]
    fn derive_focus_state_defaults_to_status_panel() {
        let frame = RenderFrame {
            mode: GameMode::Classic,
            bounds: (10, 10),
            tiles: Vec::new(),
            hud_lines: vec!["HP: 100/100".to_string()],
            interaction_lines: Vec::new(),
            timeline_lines: Vec::new(),
            event_lines: Vec::new(),
        };

        let focus = derive_focus_state(&frame);
        assert_eq!(focus.active_panel, UiPanelFocus::Status);
        assert_eq!(focus.urgency, 0.35);
    }
}
