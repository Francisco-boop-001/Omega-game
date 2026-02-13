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

/// Resource holding the theme we are transitioning towards.
#[derive(Resource, Debug, Clone)]
struct TargetTheme(Option<BevyTheme>);

/// Progress of the current theme transition (0.0 to 1.0).
#[derive(Resource, Debug, Clone)]
struct ThemeTransitionProgress(f32);

pub mod animation;
pub mod bevy_theme;
pub mod color_adapter;
pub mod editor;
pub mod hud;
pub mod input;
pub mod interaction;
pub mod overlays;
pub mod scene;
pub mod spawner;
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

        app.add_plugins(bevy_egui::EguiPlugin)
            .insert_resource(bevy_theme)
            .insert_resource(theme::UiLayoutTokens::default())
            .insert_resource(theme::UiChromeColors::default())
            .insert_resource(UiReadabilityConfig::default())
            .insert_resource(animation::UiMotionState::default())
            .insert_resource(UiFocusState::default())
            .insert_resource(UiBootLatch::default())
            .insert_resource(ActiveThemeName::default())
            .insert_resource(TargetTheme(None))
            .insert_resource(ThemeTransitionProgress(0.0))
            .insert_resource(editor::ThemeEditorState::default())
            .add_event::<ThemeChangeEvent>()
            .add_systems(Startup, scene::setup_arcane_scene)
            .add_systems(
                Update,
                (
                    animation::advance_ui_motion,
                    input::keyboard_to_runtime_input,
                    handle_theme_change_events,
                    process_theme_transition,
                    monitor_environment_changes,
                    handle_theme_cycle_key,
                    ensure_session_started,
                    update_ui_panels,
                    apply_focus_styles,
                    update_ui_text_colors,
                    editor::theme_editor_ui,
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
    mut commands: Commands,
    status: Res<RuntimeStatus>,
    frame: Res<RuntimeFrame>,
    motion: Res<animation::UiMotionState>,
    bevy_theme: Res<BevyTheme>,
    mut focus: ResMut<UiFocusState>,
    mut text_queries: ParamSet<(
        Query<Entity, With<MapPanelText>>,
        Query<&mut Text, With<CompassPanelText>>,
        Query<&mut Text, With<HudPanelText>>,
        Query<&mut Text, With<InteractionPanelText>>,
        Query<&mut Text, With<TimelinePanelText>>,
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

    let map_data = tilemap::compose_map_lines(frame_ref, motion.frame);
    if let Ok(map_entity) = text_queries.p0().get_single_mut() {
        commands.entity(map_entity).despawn_descendants();
        commands.entity(map_entity).with_children(|parent| {
            for row in map_data {
                for (ch, color_id) in row {
                    let color = bevy_theme.resolve(&color_id);
                    parent.spawn((
                        TextSpan::new(ch.to_string()),
                        TextColor(color),
                    ));
                }
                parent.spawn(TextSpan::new("\n".to_string()));
            }
        });
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
    chrome: Res<theme::UiChromeColors>,
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
    let base_border = if readability.high_contrast { chrome.text_focus } else { chrome.panel_border };
    let highlight_color = bevy_theme.get_ui_highlight();

    if let Ok((mut background, mut border)) = card_queries.p0().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Map {
            blend_color(chrome.map_frame, chrome.objective_halo_calm, intensity * 0.35)
        } else {
            chrome.map_frame
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Map {
            blend_color(base_border, highlight_color, intensity)
        } else {
            base_border
        });
    }

    if let Ok((mut background, mut border)) = card_queries.p1().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Compass {
            blend_color(chrome.panel_surface_alt, chrome.objective_halo_calm, intensity * 0.45)
        } else {
            chrome.panel_surface_alt
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Compass {
            blend_color(base_border, highlight_color, intensity)
        } else {
            base_border
        });
    }

    if let Ok((mut background, mut border)) = card_queries.p2().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Status {
            blend_color(chrome.panel_surface, chrome.panel_surface_focus, intensity * 0.35)
        } else {
            chrome.panel_surface
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Status {
            blend_color(base_border, highlight_color, intensity * 0.75)
        } else {
            base_border
        });
    }

    if let Ok((mut background, mut border)) = card_queries.p3().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Interaction {
            blend_color(chrome.panel_surface_focus, chrome.accent_chaos, intensity * 0.3)
        } else {
            chrome.panel_surface_focus
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Interaction {
            blend_color(base_border, highlight_color, intensity)
        } else {
            base_border
        });
    }

    if let Ok((mut background, mut border)) = card_queries.p4().get_single_mut() {
        *background = BackgroundColor(if focus.active_panel == UiPanelFocus::Timeline {
            blend_color(chrome.panel_brass, chrome.panel_surface_focus, intensity * 0.28)
        } else {
            chrome.panel_brass
        });
        *border = BorderColor(if focus.active_panel == UiPanelFocus::Timeline {
            blend_color(base_border, highlight_color, intensity * 0.7)
        } else {
            base_border
        });
    }
}

/// System that updates UI text components with theme-aware colors.
fn update_ui_text_colors(
    bevy_theme: Res<BevyTheme>,
    mut queries: ParamSet<(
        Query<&mut TextColor, With<MapPanelText>>,
        Query<&mut TextColor, With<CompassPanelText>>,
        Query<&mut TextColor, With<HudPanelText>>,
        Query<&mut TextColor, With<InteractionPanelText>>,
        Query<&mut TextColor, With<TimelinePanelText>>,
    )>,
) {
    let default_color = TextColor(bevy_theme.get_ui_text_default());
    let warning_color = TextColor(bevy_theme.get_ui_message_warning());
    let dim_color = TextColor(bevy_theme.get_ui_text_dim());

    for mut color in queries.p0().iter_mut() {
        *color = default_color.clone();
    }
    for mut color in queries.p1().iter_mut() {
        *color = default_color.clone();
    }
    for mut color in queries.p2().iter_mut() {
        *color = default_color.clone();
    }
    for mut color in queries.p3().iter_mut() {
        *color = warning_color.clone();
    }
    for mut color in queries.p4().iter_mut() {
        *color = dim_color.clone();
    }
}

/// System that handles `ThemeChangeEvent` and starts a transition.
fn handle_theme_change_events(
    mut events: EventReader<ThemeChangeEvent>,
    mut target_theme: ResMut<TargetTheme>,
    mut progress: ResMut<ThemeTransitionProgress>,
    mut active_theme_name: ResMut<ActiveThemeName>,
) {
    for event in events.read() {
        match color_adapter::load_builtin_theme(&event.theme_name) {
            Ok(color_theme) => {
                target_theme.0 = Some(BevyTheme::new(color_theme));
                progress.0 = 0.0;
                active_theme_name.0 = event.theme_name.clone();
                info!("Starting theme transition to: {}", event.theme_name);
            }
            Err(err) => {
                error!("Failed to load theme '{}': {}", event.theme_name, err);
            }
        }
    }
}

/// System that monitors the game environment and triggers theme changes.
fn monitor_environment_changes(
    runtime: Res<FrontendRuntime>,
    active_theme: Res<ActiveThemeName>,
    mut theme_events: EventWriter<ThemeChangeEvent>,
    mut last_env: Local<Option<omega_core::LegacyEnvironment>>,
) {
    if let Some(session) = &runtime.0.session {
        if last_env.is_none() || last_env.unwrap() != session.state.environment {
            let recommended = omega_core::color::ColorTheme::name_for_environment(session.state.environment);
            if active_theme.0 != recommended {
                info!("Environment change detected: {:?}, recommending theme: {}", session.state.environment, recommended);
                theme_events.send(ThemeChangeEvent {
                    theme_name: recommended.to_string(),
                });
            }
            *last_env = Some(session.state.environment);
        }
    }
}

/// System that interpolates colors during a theme transition.
fn process_theme_transition(
    time: Res<Time>,
    mut bevy_theme: ResMut<BevyTheme>,
    mut target_theme: ResMut<TargetTheme>,
    mut progress: ResMut<ThemeTransitionProgress>,
) {
    if let Some(target) = &target_theme.0 {
        progress.0 += time.delta_secs() * 2.0; // 0.5s transition
        info!("Transition progress: {:.2}", progress.0);
        
        if progress.0 >= 1.0 {
            // Transition complete
            info!("Theme transition complete");
            *bevy_theme = target.clone();
            target_theme.0 = None;
            progress.0 = 0.0;
        } else {
            // Interpolate all semantic colors
            bevy_theme.lerp_towards(target, progress.0);
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
        info!("F5 pressed - cycling theme from: {}", active_theme_name.0);
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
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.init_asset::<Shader>();
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
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.init_asset::<Shader>();
        app.add_plugins(ArcaneCartographerPlugin);

        // Verify all expected resources exist
        assert!(
            app.world().get_resource::<BevyTheme>().is_some(),
            "BevyTheme should be inserted"
        );
        assert!(
            app.world().get_resource::<theme::UiLayoutTokens>().is_some(),
            "UiLayoutTokens should be inserted"
        );
        assert!(
            app.world().get_resource::<theme::UiChromeColors>().is_some(),
            "UiChromeColors should be inserted"
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
            tiles: vec![crate::TileRender {
                position: omega_core::Position { x: 5, y: 5 },
                kind: crate::TileKind::TargetCursor,
                sprite: crate::SpriteRef { atlas: "test".to_string(), index: 0 },
                glyph: Some('+'),
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
            tiles: vec![crate::TileRender {
                position: omega_core::Position { x: 5, y: 5 },
                kind: crate::TileKind::ObjectiveMarker,
                sprite: crate::SpriteRef { atlas: "test".to_string(), index: 0 },
                glyph: Some('O'),
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
