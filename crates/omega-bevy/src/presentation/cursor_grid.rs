use crate::presentation::UiReadabilityConfig;
use crate::presentation::theme::UiLayoutTokens;
use crate::presentation::tilemap;
use crate::{Position, RuntimeFrame};
use bevy::prelude::{ComputedNode, Vec2};
use bevy::ui::RelativeCursorPosition;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorGridError {
    NoFrame,
    CursorOutsideNode,
    OutOfBounds,
    InvalidGeometry,
}

impl fmt::Display for CursorGridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoFrame => write!(f, "No projected frame is available"),
            Self::CursorOutsideNode => write!(f, "Cursor is outside the map panel"),
            Self::OutOfBounds => write!(f, "Cursor is outside the rendered map viewport"),
            Self::InvalidGeometry => write!(f, "Map panel geometry is invalid"),
        }
    }
}

pub fn map_panel_cursor_to_grid(
    rel_cursor: &RelativeCursorPosition,
    node: &ComputedNode,
    layout: &UiLayoutTokens,
    readability: &UiReadabilityConfig,
    frame: &RuntimeFrame,
) -> Result<Position, CursorGridError> {
    let normalized = rel_cursor.normalized.ok_or(CursorGridError::CursorOutsideNode)?;
    map_panel_normalized_cursor_to_grid(normalized, node.size(), layout, readability, frame)
}

fn map_panel_normalized_cursor_to_grid(
    normalized: Vec2,
    node_size: Vec2,
    layout: &UiLayoutTokens,
    readability: &UiReadabilityConfig,
    frame: &RuntimeFrame,
) -> Result<Position, CursorGridError> {
    let rendered_frame = frame.frame.as_ref().ok_or(CursorGridError::NoFrame)?;
    if !(0.0..=1.0).contains(&normalized.x) || !(0.0..=1.0).contains(&normalized.y) {
        return Err(CursorGridError::CursorOutsideNode);
    }
    if node_size.x <= 0.0 || node_size.y <= 0.0 {
        return Err(CursorGridError::InvalidGeometry);
    }

    let map_w = rendered_frame.bounds.0.max(1);
    let map_h = rendered_frame.bounds.1.max(1);
    let (start_x, start_y, view_w, view_h) = tilemap::visible_map_window(rendered_frame);
    let cursor_pixels = normalized * node_size;

    let padding = layout.spacing_md * readability.scale;
    let title_height = (layout.panel_title_font_size * readability.scale).max(1.0);
    let title_gap = layout.spacing_sm * readability.scale;
    let map_left = padding;
    let map_top = padding + title_height + title_gap;
    let map_width = (node_size.x - padding * 2.0).max(1.0);
    let map_height = (node_size.y - map_top - padding).max(1.0);
    let edge_tolerance = (layout.spacing_sm * readability.scale).max(10.0);

    if cursor_pixels.x < map_left - edge_tolerance
        || cursor_pixels.y < map_top - edge_tolerance
        || cursor_pixels.x >= map_left + map_width + edge_tolerance
        || cursor_pixels.y >= map_top + map_height + edge_tolerance
    {
        return Err(CursorGridError::OutOfBounds);
    }

    let nx = ((cursor_pixels.x - map_left) / map_width).clamp(0.0, 1.0);
    let ny = ((cursor_pixels.y - map_top) / map_height).clamp(0.0, 1.0);
    let local_x = ((nx * view_w as f32).floor() as i32).min(view_w as i32 - 1);
    let local_y = ((ny * view_h as f32).floor() as i32).min(view_h as i32 - 1);
    if local_x < 0 || local_y < 0 || local_x >= view_w as i32 || local_y >= view_h as i32 {
        return Err(CursorGridError::OutOfBounds);
    }

    let grid_x = start_x + local_x;
    let grid_y = start_y + local_y;
    if grid_x < 0 || grid_x >= map_w || grid_y < 0 || grid_y >= map_h {
        return Err(CursorGridError::OutOfBounds);
    }

    Ok(Position { x: grid_x, y: grid_y })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RenderFrame, SpriteRef, TileKind, TileRender};
    use omega_core::GameMode;

    fn test_frame(width: i32, height: i32, player: Position) -> RuntimeFrame {
        RuntimeFrame {
            frame: Some(RenderFrame {
                mode: GameMode::Modern,
                bounds: (width, height),
                tiles: vec![TileRender {
                    position: player,
                    kind: TileKind::Player,
                    sprite: SpriteRef { atlas: "test".to_string(), index: 0 },
                    glyph: Some('@'),
                }],
                hud_lines: Vec::new(),
                interaction_lines: Vec::new(),
                timeline_lines: Vec::new(),
                event_lines: Vec::new(),
            }),
        }
    }

    fn readability() -> UiReadabilityConfig {
        UiReadabilityConfig { scale: 1.0, high_contrast: false, reduced_motion: false }
    }

    #[test]
    fn cursor_translation_rejects_missing_frame() {
        let result = map_panel_normalized_cursor_to_grid(
            Vec2::new(0.5, 0.5),
            Vec2::new(800.0, 600.0),
            &UiLayoutTokens::default(),
            &readability(),
            &RuntimeFrame::default(),
        );
        assert_eq!(result, Err(CursorGridError::NoFrame));
    }

    #[test]
    fn cursor_translation_rejects_outside_normalized_values() {
        let frame = test_frame(50, 50, Position { x: 25, y: 25 });
        let result = map_panel_normalized_cursor_to_grid(
            Vec2::new(1.2, 0.5),
            Vec2::new(800.0, 600.0),
            &UiLayoutTokens::default(),
            &readability(),
            &frame,
        );
        assert_eq!(result, Err(CursorGridError::CursorOutsideNode));
    }

    #[test]
    fn cursor_translation_maps_center_inside_bounds() {
        let frame = test_frame(50, 50, Position { x: 25, y: 25 });
        let pos = map_panel_normalized_cursor_to_grid(
            Vec2::new(0.5, 0.5),
            Vec2::new(800.0, 600.0),
            &UiLayoutTokens::default(),
            &readability(),
            &frame,
        )
        .expect("center cursor should map to a valid position");
        assert!(pos.x >= 0 && pos.x < 50);
        assert!(pos.y >= 0 && pos.y < 50);
    }

    #[test]
    fn cursor_translation_rejects_padding_clicks() {
        let frame = test_frame(50, 50, Position { x: 25, y: 25 });
        let result = map_panel_normalized_cursor_to_grid(
            Vec2::new(0.0, 0.0),
            Vec2::new(800.0, 600.0),
            &UiLayoutTokens::default(),
            &readability(),
            &frame,
        );
        assert_eq!(result, Err(CursorGridError::OutOfBounds));
    }
}
