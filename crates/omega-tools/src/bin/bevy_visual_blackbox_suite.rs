use anyhow::{Context, Result, bail};
use omega_bevy::{
    BevyKey, FrontendRuntime, TileKind, build_runtime_app_with_mode, enqueue_input, runtime_frame,
};
use omega_core::{
    GameMode, PendingProjectileAction, Position, ProjectileDamageType, ProjectileKind,
    TargetingInteraction,
};
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
struct ScenarioResult {
    id: String,
    pass: bool,
    input_trace: Vec<String>,
    failed_assertions: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Report {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    scenarios: Vec<ScenarioResult>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn write_artifacts(report: &Report) -> Result<()> {
    fs::create_dir_all("target/dual").context("create target/dual")?;
    fs::write(
        "target/dual/bevy-visual-blackbox.json",
        serde_json::to_string_pretty(report).context("serialize bevy visual blackbox json")?,
    )
    .context("write target/dual/bevy-visual-blackbox.json")?;

    let mut md = String::new();
    md.push_str("# Bevy Visual Blackbox Suite\n\n");
    md.push_str(&format!(
        "- generated_at_utc: `{}`\n- total: `{}`\n- passed: `{}`\n- failed: `{}`\n- status: `{}`\n\n",
        report.generated_at_utc,
        report.total,
        report.passed,
        report.failed,
        if report.pass { "PASS" } else { "FAIL" }
    ));
    for scenario in &report.scenarios {
        md.push_str(&format!(
            "## {}\n\n- status: `{}`\n- input_trace: `{}`\n",
            scenario.id,
            if scenario.pass { "PASS" } else { "FAIL" },
            scenario.input_trace.join(" ")
        ));
        if !scenario.failed_assertions.is_empty() {
            md.push_str("- failed_assertions:\n");
            for assertion in &scenario.failed_assertions {
                md.push_str(&format!("  - {}\n", assertion));
            }
        }
        md.push('\n');
    }
    fs::write("target/dual/bevy-visual-blackbox.md", md)
        .context("write target/dual/bevy-visual-blackbox.md")?;
    Ok(())
}

fn main() -> Result<()> {
    let mut scenarios = Vec::new();

    {
        let mut app = build_runtime_app_with_mode(0xBEE5_2003, GameMode::Modern);
        app.update();
        let trace = vec!["<enter>", "w", "d"];
        enqueue_input(&mut app, BevyKey::Enter);
        app.update();
        enqueue_input(&mut app, BevyKey::Char('w'));
        enqueue_input(&mut app, BevyKey::Char('d'));
        app.update();
        let frame = runtime_frame(&app).context("missing frame for movement_outcome_timeline")?;
        let mut failed = Vec::new();
        if !frame.event_lines.iter().any(|line| line.contains("move") || line.contains("turn")) {
            failed.push("timeline did not expose movement/turn outcome lines".to_string());
        }
        scenarios.push(ScenarioResult {
            id: "movement_outcome_timeline".to_string(),
            pass: failed.is_empty(),
            input_trace: trace.into_iter().map(str::to_string).collect(),
            failed_assertions: failed,
        });
    }

    {
        let mut app = build_runtime_app_with_mode(0xBEE5_2004, GameMode::Modern);
        app.update();
        let trace = vec!["<enter>", "i"];
        enqueue_input(&mut app, BevyKey::Enter);
        app.update();
        enqueue_input(&mut app, BevyKey::Char('i'));
        app.update();
        let frame = runtime_frame(&app).context("missing frame for modal_prompt_visibility")?;
        let mut failed = Vec::new();
        if !frame.event_lines.iter().any(|line| line.starts_with("ACTIVE:")) {
            failed.push("interaction prompt was not visible after inventory command".to_string());
        }
        scenarios.push(ScenarioResult {
            id: "modal_prompt_visibility".to_string(),
            pass: failed.is_empty(),
            input_trace: trace.into_iter().map(str::to_string).collect(),
            failed_assertions: failed,
        });
    }

    {
        let mut app = build_runtime_app_with_mode(0xBEE5_2005, GameMode::Modern);
        app.update();
        let trace = vec!["<enter>", "<inject-targeting>", "<inject-projectile-fx>"];
        enqueue_input(&mut app, BevyKey::Enter);
        app.update();
        {
            let mut runtime = app.world_mut().resource_mut::<FrontendRuntime>();
            if let Some(session) = runtime.0.session.as_mut() {
                let player = session.state.player.position;
                session.state.pending_targeting_interaction = Some(TargetingInteraction {
                    origin: player,
                    cursor: Position { x: player.x + 1, y: player.y },
                    mode: ProjectileKind::FireBolt,
                });
                session.state.pending_projectile_action = Some(PendingProjectileAction {
                    source_token: "suite".to_string(),
                    turn_minutes: 5,
                    mode: ProjectileKind::FireBolt,
                    item_id: None,
                    item_name: "suite bolt".to_string(),
                    hit_bonus: 0,
                    damage_bonus: 0,
                    damage_min: 1,
                    damage_max: 2,
                    damage_type: ProjectileDamageType::Flame,
                    max_range: 8,
                    allows_drop: false,
                });
                session.state.transient_projectile_path =
                    vec![Position { x: player.x + 1, y: player.y }];
                session.state.transient_projectile_impact =
                    Some(Position { x: player.x + 2, y: player.y });
            }
        }
        app.update();
        let frame =
            runtime_frame(&app).context("missing frame for targeting_and_projectile_visuals")?;
        let mut failed = Vec::new();
        if !frame.tiles.iter().any(|tile| tile.kind == TileKind::TargetCursor) {
            failed.push("target cursor tile missing".to_string());
        }
        if !frame.tiles.iter().any(|tile| tile.kind == TileKind::ProjectileTrail) {
            failed.push("projectile trail tile missing".to_string());
        }
        if !frame.tiles.iter().any(|tile| tile.kind == TileKind::ProjectileImpact) {
            failed.push("projectile impact tile missing".to_string());
        }
        scenarios.push(ScenarioResult {
            id: "targeting_and_projectile_visuals".to_string(),
            pass: failed.is_empty(),
            input_trace: trace.into_iter().map(str::to_string).collect(),
            failed_assertions: failed,
        });
    }

    let total = scenarios.len();
    let passed = scenarios.iter().filter(|scenario| scenario.pass).count();
    let failed = total.saturating_sub(passed);
    let report = Report {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        scenarios,
    };
    write_artifacts(&report)?;

    if !report.pass {
        bail!("bevy visual blackbox suite failed");
    }
    println!("bevy_visual_blackbox_suite: PASS {}/{}", report.passed, report.total);
    Ok(())
}
