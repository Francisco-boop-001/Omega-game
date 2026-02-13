use crate::{RenderFrame, TileKind};
use omega_core::GameMode;

pub fn compose_compass_lines(frame: &RenderFrame, _pulse_frame: u64) -> Vec<String> {
    let modern_mode = frame.mode == GameMode::Modern;
    if !modern_mode {
        return vec!["Classic mode: objective compass hidden.".to_string()];
    }

    let mut player = None;
    let mut markers = Vec::new();
    for tile in &frame.tiles {
        if tile.kind == TileKind::Player {
            player = Some(tile.position);
        } else if tile.kind == TileKind::ObjectiveMarker {
            markers.push(tile.position);
        }
    }

    let mut lines = vec!["[Compass Instrument]".to_string()];
    let Some(player) = player else {
        lines.push("No player position.".to_string());
        return lines;
    };
    if markers.is_empty() {
        lines.push("No active objective marker.".to_string());
        return lines;
    }

    let target = markers[0];
    let dx = target.x - player.x;
    let dy = target.y - player.y;
    let heading = match (dx.signum(), dy.signum()) {
        (0, -1) => "N",
        (1, -1) => "NE",
        (1, 0) => "E",
        (1, 1) => "SE",
        (0, 1) => "S",
        (-1, 1) => "SW",
        (-1, 0) => "W",
        (-1, -1) => "NW",
        _ => "HERE",
    };

    let distance = dx.abs() + dy.abs();
    lines.push(format!("Bearing {heading}"));
    lines.push(format!("Vector dx={} dy={}", dx, dy));
    lines.push(format!("Range {} tiles", distance));
    if distance <= 1 {
        lines.push("Approach locked. Objective at hand.".to_string());
    } else {
        lines.push("Follow halo pips on the survey grid.".to_string());
    }
    lines
}
