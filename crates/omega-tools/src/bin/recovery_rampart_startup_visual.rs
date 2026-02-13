use anyhow::{Context, Result, bail};
use omega_bevy::{SpriteAtlas, TileKind, project_to_frame};
use omega_content::{LEGACY_RAMPART_START, bootstrap_game_state_from_default_content};
use omega_core::{
    Command, DeterministicRng, GameState, Position, TILE_FLAG_NO_CITY_MOVE, TILE_FLAG_PORTCULLIS,
    step,
};
use omega_tui::{App, render_screen};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct VisualCheck {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RampartStartupVisualReport {
    generated_at_utc: String,
    pass: bool,
    player_position: Position,
    map_window: Vec<String>,
    map_glyph_counts: BTreeMap<String, usize>,
    bevy_tile_kind_counts: BTreeMap<String, usize>,
    checks: Vec<VisualCheck>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn markdown(report: &RampartStartupVisualReport) -> String {
    let mut out = Vec::new();
    out.push("# Recovery Rampart Startup Visual".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", report.generated_at_utc));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(format!(
        "- Player position: ({}, {})",
        report.player_position.x, report.player_position.y
    ));
    out.push(String::new());
    out.push("## Map Window".to_string());
    out.push(String::new());
    out.push("```text".to_string());
    out.extend(report.map_window.clone());
    out.push("```".to_string());
    out.push(String::new());
    out.push("## Checks".to_string());
    out.push(String::new());
    for check in &report.checks {
        out.push(format!(
            "- {}: {} ({})",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.details
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn visible_map_window(state: &GameState, width: i32, height: i32) -> Vec<String> {
    let center = state.player.position;
    let half_w = width / 2;
    let half_h = height / 2;
    let min_x = (center.x - half_w).max(0);
    let max_x = (center.x + half_w).min(state.bounds.width - 1);
    let min_y = (center.y - half_h).max(0);
    let max_y = (center.y + half_h).min(state.bounds.height - 1);

    let mut rows = Vec::new();
    for y in min_y..=max_y {
        let mut row = String::new();
        for x in min_x..=max_x {
            let pos = Position { x, y };
            let ch = if state.player.position == pos {
                '@'
            } else if state.monsters.iter().any(|m| m.position == pos) {
                'm'
            } else if state.ground_items.iter().any(|g| g.position == pos) {
                '*'
            } else {
                state.map_glyph_at(pos)
            };
            row.push(ch);
        }
        rows.push(row);
    }
    rows
}

fn increment(counter: &mut BTreeMap<String, usize>, key: impl Into<String>) {
    *counter.entry(key.into()).or_insert(0) += 1;
}

fn tile_kind_label(kind: TileKind) -> &'static str {
    match kind {
        TileKind::Floor => "floor",
        TileKind::Wall => "wall",
        TileKind::Grass => "grass",
        TileKind::Water => "water",
        TileKind::Fire => "fire",
        TileKind::Feature => "feature",
        TileKind::Player => "player",
        TileKind::Monster => "monster",
        TileKind::GroundItem => "ground_item",
        TileKind::TargetCursor => "target_cursor",
        TileKind::ObjectiveMarker => "objective_marker",
        TileKind::ProjectileTrail => "projectile_trail",
        TileKind::ProjectileImpact => "projectile_impact",
    }
}

fn main() -> Result<()> {
    let (state, diagnostics) = bootstrap_game_state_from_default_content()
        .context("bootstrap default content for visual proof")?;

    let map_window = visible_map_window(&state, 21, 11);
    let mut map_glyph_counts = BTreeMap::new();
    for row in &map_window {
        for ch in row.chars() {
            increment(&mut map_glyph_counts, ch.to_string());
        }
    }

    let bevy_frame = project_to_frame(&state, None, &SpriteAtlas::default());
    let mut bevy_tile_kind_counts = BTreeMap::new();
    for tile in bevy_frame.tiles {
        increment(&mut bevy_tile_kind_counts, tile_kind_label(tile.kind));
    }

    let non_void_glyphs = map_glyph_counts
        .iter()
        .filter(|(glyph, _)| !matches!(glyph.as_str(), "." | "@" | "m" | "*"))
        .map(|(_, count)| *count)
        .sum::<usize>();

    let save_slot = PathBuf::from("target/recovery-startup-visual-slot.json");
    let app = App::with_options(0xDEC0_DED1, state.clone(), state.clone(), save_slot);
    let tui_render = render_screen(&app);
    let width = usize::try_from(state.bounds.width).unwrap_or(0);

    let mut safe_probe = state.clone();
    let hp_before_wait = safe_probe.player.stats.hp;
    let mut safe_rng = DeterministicRng::seeded(0xA11CE);
    let _ = step(&mut safe_probe, Command::Wait, &mut safe_rng);
    let safe_wait = safe_probe.player.stats.hp == hp_before_wait;

    let portcullis_idx =
        state.city_site_grid.iter().position(|cell| (cell.flags & TILE_FLAG_PORTCULLIS) != 0);
    let nocity_idx =
        state.city_site_grid.iter().position(|cell| (cell.flags & TILE_FLAG_NO_CITY_MOVE) != 0);

    let movement_rules_enforced = if let (Some(port_idx), Some(nocity_idx)) =
        (portcullis_idx, nocity_idx)
    {
        let port_pos = Position {
            x: i32::try_from(port_idx % width).unwrap_or(0),
            y: i32::try_from(port_idx / width).unwrap_or(0),
        };
        let mut movement_probe = state.clone();
        movement_probe.monsters.clear();
        movement_probe.player.position = Position {
            x: i32::try_from(nocity_idx % width).unwrap_or(0),
            y: i32::try_from(nocity_idx / width).unwrap_or(0),
        };
        let mut rng = DeterministicRng::seeded(0xBEEF);
        let out = step(&mut movement_probe, Command::Legacy { token: "M".to_string() }, &mut rng);
        let nocity_block = out.events.iter().any(|event| {
            matches!(
                event,
                omega_core::Event::LegacyHandled { token, note, fully_modeled }
                    if token == "M" && *fully_modeled && note.contains("NOCITYMOVE")
            )
        });
        !state.tile_is_walkable(port_pos) && nocity_block
    } else {
        false
    };

    let checks = vec![
        VisualCheck {
            id: "player_starts_at_legacy_rampart_start".to_string(),
            pass: state.player.position == LEGACY_RAMPART_START,
            details: format!(
                "expected=({}, {}) actual=({}, {})",
                LEGACY_RAMPART_START.x,
                LEGACY_RAMPART_START.y,
                state.player.position.x,
                state.player.position.y
            ),
        },
        VisualCheck {
            id: "map_window_is_not_void".to_string(),
            pass: non_void_glyphs > 0,
            details: format!("non_void_glyph_cells={non_void_glyphs}"),
        },
        VisualCheck {
            id: "bevy_projection_has_non_floor_tiles".to_string(),
            pass: bevy_tile_kind_counts.get("wall").copied().unwrap_or(0) > 0
                || bevy_tile_kind_counts.get("feature").copied().unwrap_or(0) > 0,
            details: format!(
                "wall={} feature={}",
                bevy_tile_kind_counts.get("wall").copied().unwrap_or(0),
                bevy_tile_kind_counts.get("feature").copied().unwrap_or(0)
            ),
        },
        VisualCheck {
            id: "tui_render_contains_map_panel".to_string(),
            pass: tui_render.contains("MAP"),
            details: format!(
                "bootstrap_source={} spawn_source={}",
                diagnostics.map_source, diagnostics.player_spawn_source
            ),
        },
        VisualCheck {
            id: "canonical_start_safe_initial_state".to_string(),
            pass: safe_wait,
            details: format!(
                "hp_before_wait={hp_before_wait} hp_after_wait={}",
                safe_probe.player.stats.hp
            ),
        },
        VisualCheck {
            id: "city_movement_rules_enforced".to_string(),
            pass: movement_rules_enforced,
            details: format!(
                "portcullis_present={} nocitymove_present={}",
                portcullis_idx.is_some(),
                nocity_idx.is_some()
            ),
        },
    ];

    let pass = checks.iter().all(|check| check.pass);
    let report = RampartStartupVisualReport {
        generated_at_utc: now_utc_unix(),
        pass,
        player_position: state.player.position,
        map_window,
        map_glyph_counts,
        bevy_tile_kind_counts,
        checks,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("recovery-rampart-startup-visual.json");
    let md_path = target.join("recovery-rampart-startup-visual.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize visual report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "recovery rampart startup visual: status={}, checks_passed={}/{}",
        if pass { "PASS" } else { "FAIL" },
        report.checks.iter().filter(|check| check.pass).count(),
        report.checks.len()
    );

    if !pass {
        bail!("recovery rampart startup visual failed");
    }
    Ok(())
}
