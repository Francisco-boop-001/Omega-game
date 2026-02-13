use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

use super::bevy_theme::BevyTheme;
use super::theme::{UiChromeColors, UiLayoutTokens};
use super::{
    CompassPanelCard, CompassPanelText, HudPanelText, InteractionPanelCard, InteractionPanelText,
    MapPanelCard, MapPanelText, StatusPanelCard, TimelinePanelCard, TimelinePanelText,
    UiReadabilityConfig,
};

pub fn setup_arcane_scene(
    mut commands: Commands,
    layout: Res<UiLayoutTokens>,
    chrome: Res<UiChromeColors>,
    bevy_theme: Res<BevyTheme>,
    readability: Res<UiReadabilityConfig>,
) {
    let scale = readability.scale;
    let spacing_xs = layout.spacing_xs * scale;
    let spacing_sm = layout.spacing_sm * scale;
    let spacing_md = layout.spacing_md * scale;
    let spacing_lg = layout.spacing_lg * scale;

    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(spacing_lg)),
                column_gap: Val::Px(spacing_lg),
                ..default()
            },
            BackgroundColor(chrome.background_haze),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(chrome.background_noise),
            ));

            root.spawn((
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(spacing_md),
                    ..default()
                },
                BackgroundColor(chrome.map_backdrop),
                BorderColor(chrome.panel_border),
            ))
            .with_children(|left| {
                left.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::axes(Val::Px(spacing_md), Val::Px(spacing_sm)),
                        border: UiRect::all(Val::Px(1.0)),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(spacing_xs),
                        ..default()
                    },
                    BackgroundColor(chrome.panel_surface_depth),
                    BorderColor(chrome.panel_border),
                ))
                .with_children(|header| {
                    header.spawn((
                        Text::new("ARCANE CARTOGRAPHER CONSOLE"),
                        TextFont { font_size: 30.0 * scale, ..default() },
                        TextColor(bevy_theme.get_ui_text_bold()),
                    ));
                    header.spawn((
                        Text::new("Occult Navigation Instrument // modern mode"),
                        TextFont { font_size: layout.panel_body_font_size * scale, ..default() },
                        TextColor(bevy_theme.get_ui_text_dim()),
                    ));
                });

                left.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(spacing_md)),
                        row_gap: Val::Px(spacing_sm),
                        border: UiRect::all(Val::Px(1.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    MapPanelCard,
                    RelativeCursorPosition::default(),
                    BackgroundColor(chrome.map_frame),
                    BorderColor(chrome.panel_border),
                ))
                .with_children(|map_card| {
                    map_card.spawn((
                        Text::new("Survey Grid"),
                        TextFont { font_size: layout.panel_title_font_size * scale, ..default() },
                        TextColor(bevy_theme.get_ui_text_bold()),
                    ));
                    map_card.spawn((
                        MapPanelText,
                        Text::new("Calibrating terrain and actor layers..."),
                        TextFont { font_size: layout.map_font_size * scale, ..default() },
                        TextColor(bevy_theme.get_ui_text_default()),
                    ));
                });

                left.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(170.0 * scale),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(spacing_md)),
                        row_gap: Val::Px(spacing_xs),
                        border: UiRect::all(Val::Px(1.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    CompassPanelCard,
                    BackgroundColor(chrome.panel_surface_alt),
                    BorderColor(chrome.panel_border),
                ))
                .with_children(|compass_card| {
                    compass_card.spawn((
                        Text::new("Objective Halo"),
                        TextFont { font_size: layout.panel_title_font_size * scale, ..default() },
                        TextColor(bevy_theme.get_ui_text_bold()),
                    ));
                    compass_card.spawn((
                        CompassPanelText,
                        Text::new("Syncing objective beacons..."),
                        TextFont { font_size: layout.panel_body_font_size * scale, ..default() },
                        TextColor(bevy_theme.get_ui_text_default()),
                    ));
                });
            });

            root.spawn((Node {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(spacing_md),
                overflow: Overflow::clip(),
                ..default()
            },))
                .with_children(|right| {
                    right
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(30.0),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(spacing_md)),
                                row_gap: Val::Px(spacing_xs),
                                border: UiRect::all(Val::Px(1.0)),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            StatusPanelCard,
                            BackgroundColor(chrome.panel_surface),
                            BorderColor(chrome.panel_border),
                        ))
                        .with_children(|status_card| {
                            status_card.spawn((
                                Text::new("Status Deck"),
                                TextFont {
                                    font_size: layout.panel_title_font_size * scale,
                                    ..default()
                                },
                                TextColor(bevy_theme.get_ui_text_bold()),
                            ));
                            status_card.spawn((
                                HudPanelText,
                                Text::new("Loading vitals and quest state..."),
                                TextFont {
                                    font_size: layout.panel_body_font_size * scale,
                                    ..default()
                                },
                                TextColor(bevy_theme.get_ui_text_default()),
                            ));
                        });

                    right
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(28.0),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(spacing_md)),
                                row_gap: Val::Px(spacing_xs),
                                border: UiRect::all(Val::Px(2.0)),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            InteractionPanelCard,
                            BackgroundColor(chrome.panel_surface_focus),
                            BorderColor(bevy_theme.get_ui_highlight()),
                        ))
                        .with_children(|interaction_card| {
                            interaction_card.spawn((
                                Text::new("Interaction Focus"),
                                TextFont {
                                    font_size: layout.panel_title_font_size * scale,
                                    ..default()
                                },
                                TextColor(bevy_theme.get_ui_message_warning()),
                            ));
                            interaction_card.spawn((
                                InteractionPanelText,
                                Text::new("Waiting for active interaction..."),
                                TextFont {
                                    font_size: layout.panel_body_font_size * scale,
                                    ..default()
                                },
                                TextColor(bevy_theme.get_ui_text_default()),
                            ));
                        });

                    right
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(spacing_md)),
                                row_gap: Val::Px(spacing_xs),
                                border: UiRect::all(Val::Px(1.0)),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            TimelinePanelCard,
                            BackgroundColor(chrome.panel_brass),
                            BorderColor(chrome.panel_border),
                        ))
                        .with_children(|timeline_card| {
                            timeline_card.spawn((
                                Text::new("Outcome Timeline"),
                                TextFont {
                                    font_size: layout.panel_title_font_size * scale,
                                    ..default()
                                },
                                TextColor(bevy_theme.get_ui_text_bold()),
                            ));
                            timeline_card.spawn((
                                TimelinePanelText,
                                Text::new("No outcomes captured yet..."),
                                TextFont {
                                    font_size: layout.panel_body_small_font_size * scale,
                                    ..default()
                                },
                                TextColor(bevy_theme.get_ui_text_dim()),
                            ));
                        });
                });
        });
}
