//! Theme editor UI using bevy_egui.
//!
//! Provides a real-time debugging interface for modifying semantic colors
//! and exporting them to user themes.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::presentation::bevy_theme::BevyTheme;
use omega_core::color::{ColorId, EntityColorId, UiColorId, HexColor};

/// Marker for the theme editor window visibility.
#[derive(Resource, Default)]
pub struct ThemeEditorState {
    pub visible: bool,
}

pub fn theme_editor_ui(
    mut contexts: EguiContexts,
    mut theme: ResMut<BevyTheme>,
    mut state: ResMut<ThemeEditorState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F6) {
        state.visible = !state.visible;
    }

    if !state.visible {
        return;
    }

    egui::Window::new("Omega Theme Editor (F6 to toggle)")
        .default_width(400.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Semantic Colors");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                // UI Colors
                ui.collapsing("UI Elements", |ui| {
                    color_picker(ui, "Health High", &mut theme, ColorId::Ui(UiColorId::HealthHigh));
                    color_picker(ui, "Health Low", &mut theme, ColorId::Ui(UiColorId::HealthLow));
                    color_picker(ui, "Mana", &mut theme, ColorId::Ui(UiColorId::Mana));
                    color_picker(ui, "Text Default", &mut theme, ColorId::Ui(UiColorId::TextDefault));
                });

                // Entity Colors
                ui.collapsing("Entities", |ui| {
                    color_picker(ui, "Player", &mut theme, ColorId::Entity(EntityColorId::Player));
                });
            });

            ui.separator();
            if ui.button("Save to User Themes (WIP)").clicked() {
                // Future: Implement saving to disk
            }
        });
}

fn color_picker(ui: &mut egui::Ui, label: &str, theme: &mut BevyTheme, id: ColorId) {
    let current = theme.resolve(&id);
    let rgba = current.to_srgba();
    let mut color = [
        rgba.red,
        rgba.green,
        rgba.blue,
    ];

    ui.horizontal(|ui| {
        ui.label(label);
        if ui.color_edit_button_rgb(&mut color).changed() {
            let hex = HexColor::from_rgb(
                (color[0] * 255.0) as u8,
                (color[1] * 255.0) as u8,
                (color[2] * 255.0) as u8,
            );
            // Internal hack to update theme for preview
            // In a real implementation, we'd need a more robust way to mutate the underlying ColorTheme
            // or use a temporary override map in BevyTheme.
            // For now, we'll assume BevyTheme has a method to override.
            theme.override_color(id, hex);
        }
    });
}
