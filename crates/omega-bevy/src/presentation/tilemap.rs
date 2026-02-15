use crate::{Position, RenderFrame, TileKind, TileRender};
use omega_core::GameMode;
use omega_core::color::ColorId;
use std::collections::VecDeque;

pub(crate) const MAP_VIEWPORT_WIDTH: usize = 58;
pub(crate) const MAP_VIEWPORT_HEIGHT: usize = 30;

fn glyph_for_tile(kind: TileKind) -> char {
    match kind {
        TileKind::Floor => '.',
        TileKind::Wall => '#',
        TileKind::Grass => '\"',
        TileKind::Water => '~',
        TileKind::Fire => '*',
        TileKind::Feature => '+',
        TileKind::Player => '@',
        TileKind::Monster => 'm',
        TileKind::GroundItem => '!',
        TileKind::TargetCursor => 'X',
        TileKind::ObjectiveMarker => 'O',
        TileKind::ProjectileTrail => '*',
        TileKind::ProjectileImpact => 'x',
    }
}

fn glyph_for_rendered_tile(tile: &TileRender) -> char {
    tile.glyph.unwrap_or_else(|| glyph_for_tile(tile.kind))
}

fn layer_priority(kind: TileKind) -> u8 {
    match kind {
        TileKind::Floor => 0,
        TileKind::Wall => 1,
        TileKind::Grass => 1,
        TileKind::Water => 1,
        TileKind::Fire => 7,
        TileKind::Feature => 2,
        TileKind::GroundItem => 3,
        TileKind::ObjectiveMarker => 4,
        TileKind::ProjectileTrail => 5,
        TileKind::ProjectileImpact => 6,
        TileKind::TargetCursor => 7,
        TileKind::Monster => 8,
        TileKind::Player => 9,
    }
}

fn clamp_i32(value: i32, min: i32, max: i32) -> i32 {
    value.max(min).min(max)
}

fn centered_view_window(
    width: usize,
    height: usize,
    focus: Position,
) -> (usize, usize, usize, usize) {
    let view_w = width.clamp(1, MAP_VIEWPORT_WIDTH);
    let view_h = height.clamp(1, MAP_VIEWPORT_HEIGHT);
    let max_start_x = width.saturating_sub(view_w) as i32;
    let max_start_y = height.saturating_sub(view_h) as i32;
    let raw_start_x = focus.x - (view_w as i32 / 2);
    let raw_start_y = focus.y - (view_h as i32 / 2);
    let start_x = clamp_i32(raw_start_x, 0, max_start_x) as usize;
    let start_y = clamp_i32(raw_start_y, 0, max_start_y) as usize;
    (start_x, start_y, view_w, view_h)
}

pub(crate) fn visible_map_window(frame: &RenderFrame) -> (i32, i32, usize, usize) {
    let width = frame.bounds.0.max(1) as usize;
    let height = frame.bounds.1.max(1) as usize;
    let focus = frame
        .tiles
        .iter()
        .find(|tile| tile.kind == TileKind::Player)
        .map(|tile| tile.position)
        .unwrap_or(Position { x: (width / 2) as i32, y: (height / 2) as i32 });
    let (start_x, start_y, view_w, view_h) = centered_view_window(width, height, focus);
    (start_x as i32, start_y as i32, view_w, view_h)
}

fn in_bounds(pos: Position, width: usize, height: usize) -> bool {
    pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < width && (pos.y as usize) < height
}

fn terrain_walkable_glyph(glyph: char) -> bool {
    !matches!(glyph, '#' | '=' | '-' | 'D' | 'J' | '7')
}

fn route_target_for_objective(
    player: Position,
    target: Position,
    terrain: &[Vec<char>],
) -> Option<Position> {
    let height = terrain.len();
    let width = terrain.first().map_or(0, Vec::len);
    if width == 0 || height == 0 || !in_bounds(target, width, height) {
        return None;
    }
    let tx = target.x as usize;
    let ty = target.y as usize;
    if terrain_walkable_glyph(terrain[ty][tx]) {
        return Some(target);
    }
    let deltas = [(0, -1), (1, 0), (0, 1), (-1, 0), (1, -1), (1, 1), (-1, 1), (-1, -1)];
    let mut candidates: Vec<Position> = Vec::new();
    for (dx, dy) in deltas {
        let candidate = Position { x: target.x + dx, y: target.y + dy };
        if !in_bounds(candidate, width, height) {
            continue;
        }
        let cx = candidate.x as usize;
        let cy = candidate.y as usize;
        if terrain_walkable_glyph(terrain[cy][cx]) {
            candidates.push(candidate);
        }
    }
    candidates.sort_by_key(|candidate| {
        let player_distance = (candidate.x - player.x).abs() + (candidate.y - player.y).abs();
        let target_distance = (candidate.x - target.x).abs() + (candidate.y - target.y).abs();
        (player_distance, target_distance)
    });
    candidates.into_iter().next()
}

fn bfs_path(start: Position, goal: Position, terrain: &[Vec<char>]) -> Option<Vec<Position>> {
    let height = terrain.len();
    let width = terrain.first().map_or(0, Vec::len);
    if width == 0 || height == 0 {
        return None;
    }
    if !in_bounds(start, width, height) || !in_bounds(goal, width, height) {
        return None;
    }
    if start == goal {
        return Some(vec![start]);
    }
    let mut visited = vec![vec![false; width]; height];
    let mut prev: Vec<Vec<Option<Position>>> = vec![vec![None; width]; height];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited[start.y as usize][start.x as usize] = true;

    let deltas = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    while let Some(current) = queue.pop_front() {
        for (dx, dy) in deltas {
            let next = Position { x: current.x + dx, y: current.y + dy };
            if !in_bounds(next, width, height) {
                continue;
            }
            let nx = next.x as usize;
            let ny = next.y as usize;
            if visited[ny][nx] {
                continue;
            }
            if !terrain_walkable_glyph(terrain[ny][nx]) && next != goal {
                continue;
            }
            visited[ny][nx] = true;
            prev[ny][nx] = Some(current);
            if next == goal {
                let mut path = vec![goal];
                let mut cursor = goal;
                while let Some(parent) = prev[cursor.y as usize][cursor.x as usize] {
                    path.push(parent);
                    if parent == start {
                        break;
                    }
                    cursor = parent;
                }
                path.reverse();
                return Some(path);
            }
            queue.push_back(next);
        }
    }
    None
}

pub fn compose_map_lines(frame: &RenderFrame, pulse_frame: u64) -> Vec<Vec<(char, ColorId)>> {
    let modern_mode = frame.mode == GameMode::Modern;
    let width = frame.bounds.0.max(1) as usize;
    let height = frame.bounds.1.max(1) as usize;
    let mut chars =
        vec![vec![(' ', ColorId::Ui(omega_core::color::UiColorId::TextDefault)); width]; height];
    let mut prio = vec![vec![0u8; width]; height];
    let mut terrain = vec![vec!['.'; width]; height];
    let mut player_pos = None;
    let mut objective_pos = None;

    for tile in &frame.tiles {
        let x = tile.position.x;
        let y = tile.position.y;
        if x < 0 || y < 0 {
            continue;
        }
        let (x, y) = (x as usize, y as usize);
        if y >= height || x >= width {
            continue;
        }
        let p = layer_priority(tile.kind);
        if p >= prio[y][x] {
            chars[y][x] = (glyph_for_rendered_tile(tile), tile.kind.to_color_id());
            prio[y][x] = p;
        }
        if matches!(
            tile.kind,
            TileKind::Floor
                | TileKind::Wall
                | TileKind::Grass
                | TileKind::Water
                | TileKind::Fire
                | TileKind::Feature
        ) {
            terrain[y][x] = glyph_for_rendered_tile(tile);
        }
        if tile.kind == TileKind::Player {
            player_pos = Some(tile.position);
        }
        if objective_pos.is_none() && tile.kind == TileKind::ObjectiveMarker {
            objective_pos = Some(tile.position);
        }
    }

    let interaction_hot = frame
        .interaction_lines
        .iter()
        .any(|line| line.starts_with("ACTIVE:") || line.contains("Targeting"));
    let objective_distance = if let (Some(player), Some(target)) = (player_pos, objective_pos) {
        Some((target.x - player.x).abs() + (target.y - player.y).abs())
    } else {
        None
    };
    let pulse_phase = (pulse_frame / 16).is_multiple_of(2);

    if modern_mode && let Some(player) = player_pos {
        let halo_char = if interaction_hot {
            if pulse_phase { 'O' } else { 'o' }
        } else if let Some(distance) = objective_distance {
            if distance <= 8 {
                if pulse_phase { 'O' } else { 'o' }
            } else if distance <= 18 {
                if pulse_phase { 'o' } else { '+' }
            } else if pulse_phase {
                '+'
            } else {
                '.'
            }
        } else if pulse_phase {
            'o'
        } else {
            '.'
        };
        let halo_color = ColorId::Ui(omega_core::color::UiColorId::Highlight);
        let ring = [
            (0, -1, halo_char),
            (1, -1, halo_char),
            (1, 0, halo_char),
            (1, 1, halo_char),
            (0, 1, halo_char),
            (-1, 1, halo_char),
            (-1, 0, halo_char),
            (-1, -1, halo_char),
        ];
        for (dx, dy, ch) in ring {
            let x = player.x + dx;
            let y = player.y + dy;
            if x < 0 || y < 0 {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if y < height && x < width && chars[y][x].0 == '.' {
                chars[y][x] = (ch, halo_color);
            }
        }
    }

    if modern_mode
        && let (Some(player), Some(target)) = (player_pos, objective_pos)
        && let Some(approach) = route_target_for_objective(player, target, &terrain)
        && let Some(route) = bfs_path(player, approach, &terrain)
    {
        let route_color = ColorId::Ui(omega_core::color::UiColorId::TextDim);
        for (idx, point) in route.into_iter().enumerate() {
            if idx < 2 || point == player || point == target {
                continue;
            }
            if idx % 3 != 0 {
                continue;
            }
            if point.x < 0 || point.y < 0 {
                continue;
            }
            let (x, y) = (point.x as usize, point.y as usize);
            if y < height && x < width && chars[y][x].0 == '.' {
                chars[y][x] = (if pulse_phase { ':' } else { ';' }, route_color);
            }
        }
    }

    let (start_x, start_y, view_w, view_h) = visible_map_window(frame);
    let start_x = start_x as usize;
    let start_y = start_y as usize;
    let mut view = Vec::with_capacity(view_h);
    for row in chars.iter().skip(start_y).take(view_h) {
        let rendered: Vec<(char, ColorId)> =
            row.iter().skip(start_x).take(view_w).cloned().collect();
        view.push(rendered);
    }
    view
}

#[cfg(test)]
mod tests {
    use super::compose_map_lines;
    use crate::{Position, RenderFrame, SpriteRef, TileKind, TileRender};
    use omega_core::GameMode;

    fn tile(position: Position, kind: TileKind) -> TileRender {
        TileRender {
            position,
            kind,
            sprite: SpriteRef { atlas: "test".to_string(), index: 0 },
            glyph: None,
        }
    }

    fn frame_with_player(bounds: (i32, i32), player: Position) -> RenderFrame {
        let mut tiles = Vec::new();
        for y in 0..bounds.1 {
            for x in 0..bounds.0 {
                tiles.push(tile(Position { x, y }, TileKind::Floor));
            }
        }
        tiles.push(tile(player, TileKind::Player));
        RenderFrame {
            mode: GameMode::Modern,
            bounds,
            tiles,
            hud_lines: vec!["Mode modern".to_string()],
            interaction_lines: Vec::new(),
            timeline_lines: Vec::new(),
            event_lines: Vec::new(),
        }
    }

    #[test]
    fn viewport_keeps_player_visible_near_far_edge() {
        let frame = frame_with_player((120, 80), Position { x: 110, y: 72 });
        let view = compose_map_lines(&frame, 0);
        assert!(!view.is_empty());
        assert!(view.iter().all(|line| line.len() <= 58));
        assert!(view.len() <= 30);
        assert!(view.iter().any(|line| line.iter().any(|(ch, _)| *ch == '@')));
    }

    #[test]
    fn viewport_uses_full_small_map() {
        let frame = frame_with_player((10, 6), Position { x: 9, y: 5 });
        let view = compose_map_lines(&frame, 0);
        assert_eq!(view.len(), 6);
        assert!(view.iter().all(|line| line.len() == 10));
        assert!(view.iter().any(|line| line.iter().any(|(ch, _)| *ch == '@')));
    }

    #[test]
    fn modern_objective_route_uses_subtle_markers_not_blinking_stars() {
        let mut frame = frame_with_player((20, 8), Position { x: 2, y: 2 });
        frame.tiles.push(tile(Position { x: 15, y: 2 }, TileKind::ObjectiveMarker));

        let view = compose_map_lines(&frame, 0);
        assert!(view.iter().any(|line| line.iter().any(|(ch, _)| *ch == ':')));
        assert!(!view.iter().any(|line| line.iter().any(|(ch, _)| *ch == '*')));
    }

    #[test]
    fn modern_objective_route_hides_markers_when_no_walkable_path_exists() {
        let mut frame = frame_with_player((20, 8), Position { x: 2, y: 2 });
        frame.tiles.push(tile(Position { x: 17, y: 2 }, TileKind::ObjectiveMarker));
        for y in 0..8 {
            frame.tiles.push(tile(Position { x: 10, y }, TileKind::Wall));
        }

        let view = compose_map_lines(&frame, 0);
        assert!(!view.iter().any(|line| line.iter().any(|(ch, _)| *ch == ':')));
    }
}
