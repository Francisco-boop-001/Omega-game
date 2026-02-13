use anyhow::{Context, Result, bail};
use omega_bevy::{
    BevyKey, TileKind, build_runtime_app_with_options_and_mode, enqueue_input, runtime_frame,
};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{GameMode, LegacyQuestState};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ProjectionCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ProjectionReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<ProjectionCheck>,
}

fn boot_frame(mode: GameMode, seed: u64) -> Result<omega_bevy::RenderFrame> {
    let (mut bootstrap, _) = bootstrap_game_state_from_default_content().context("bootstrap")?;
    bootstrap.mode = mode;
    bootstrap.progression.quest_state = LegacyQuestState::Active;
    bootstrap.progression.main_quest.objective =
        "Report to the Order hall in Rampart for your next briefing.".to_string();
    let slot = PathBuf::from(format!(
        "target/test-bevy-semantic-projection-{}.json",
        match mode {
            GameMode::Classic => "classic",
            GameMode::Modern => "modern",
        }
    ));
    let mut app = build_runtime_app_with_options_and_mode(seed, mode, bootstrap, slot.clone());
    app.update();
    enqueue_input(&mut app, BevyKey::Enter);
    enqueue_input(&mut app, BevyKey::Char(' '));
    app.update();
    let frame = runtime_frame(&app).context("missing runtime frame after bootstrap")?;
    let _ = fs::remove_file(slot);
    Ok(frame)
}

fn markdown(report: &ProjectionReport) -> String {
    let mut out = Vec::new();
    out.push("# Bevy Semantic Projection Parity".to_string());
    out.push(String::new());
    out.push(format!("- total: `{}`", report.total));
    out.push(format!("- passed: `{}`", report.passed));
    out.push(format!("- failed: `{}`", report.failed));
    out.push(format!("- status: `{}`", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Check | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for check in &report.checks {
        out.push(format!(
            "| {} | {} | {} |",
            check.id,
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let modern = boot_frame(GameMode::Modern, 0xBEEF_5511)?;
    let classic = boot_frame(GameMode::Classic, 0xBEEF_5512)?;

    let modern_lines = omega_bevy::presentation::tilemap::compose_map_lines(&modern, 0);
    let classic_lines = omega_bevy::presentation::tilemap::compose_map_lines(&classic, 0);
    
    let modern_blob: String = modern_lines.iter()
        .map(|row| row.iter().map(|(ch, _)| *ch).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    let classic_blob: String = classic_lines.iter()
        .map(|row| row.iter().map(|(ch, _)| *ch).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    let monster_glyphs: BTreeSet<char> = modern
        .tiles
        .iter()
        .filter(|tile| tile.kind == TileKind::Monster)
        .filter_map(|tile| tile.glyph)
        .collect();
    let guard_identity_present = monster_glyphs.contains(&'G');

    let semantic_glyphs: BTreeSet<char> = modern
        .tiles
        .iter()
        .filter(|tile| matches!(tile.kind, TileKind::Floor | TileKind::Wall | TileKind::Feature))
        .filter_map(|tile| tile.glyph)
        .collect();

    let modern_route_visible = modern_blob.contains(':') || modern_blob.contains(';');
    let modern_route_not_star = !modern_blob.contains('*');
    let classic_route_hidden = !classic_blob.contains(':') && !classic_blob.contains(';');

    let checks = vec![
        ProjectionCheck {
            id: "guard_glyph_identity".to_string(),
            passed: guard_identity_present,
            details: format!("monster_glyphs={monster_glyphs:?}"),
        },
        ProjectionCheck {
            id: "monster_glyph_not_collapsed_to_m".to_string(),
            passed: monster_glyphs.iter().any(|glyph| *glyph != 'm'),
            details: format!("monster_glyphs={monster_glyphs:?}"),
        },
        ProjectionCheck {
            id: "semantic_feature_glyph_diversity".to_string(),
            passed: semantic_glyphs.len() >= 5,
            details: format!("distinct_semantic_glyphs={}", semantic_glyphs.len()),
        },
        ProjectionCheck {
            id: "modern_route_markers_are_subtle".to_string(),
            passed: modern_route_not_star && modern_route_visible,
            details: format!(
                "route_visible={} route_not_star={}",
                modern_route_visible, modern_route_not_star
            ),
        },
        ProjectionCheck {
            id: "classic_hides_modern_route_markers".to_string(),
            passed: classic_route_hidden,
            details: format!("classic_has_route_markers={}", !classic_route_hidden),
        },
        ProjectionCheck {
            id: "player_remains_visible_in_viewport".to_string(),
            passed: modern_blob.contains('@') && classic_blob.contains('@'),
            details: format!(
                "modern_has_player={} classic_has_player={}",
                modern_blob.contains('@'),
                classic_blob.contains('@')
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let report = ProjectionReport { total, passed, failed, pass: failed == 0, checks };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }
    fs::write(
        "target/bevy-semantic-projection-parity.json",
        serde_json::to_string_pretty(&report).context("serialize semantic projection report")?,
    )
    .context("write target/bevy-semantic-projection-parity.json")?;
    fs::write("target/bevy-semantic-projection-parity.md", markdown(&report))
        .context("write target/bevy-semantic-projection-parity.md")?;

    println!(
        "bevy semantic projection parity: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("bevy semantic projection parity failed");
    }
    Ok(())
}
