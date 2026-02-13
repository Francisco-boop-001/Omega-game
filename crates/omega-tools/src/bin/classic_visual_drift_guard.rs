use anyhow::{Context, Result, bail};
use omega_bevy::{
    BevyKey, FrontendRuntime, TileKind, build_runtime_app_with_mode, enqueue_input, runtime_frame,
};
use omega_core::GameMode;
use serde::Serialize;
use std::fs;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct Snapshot {
    mode: String,
    turn: u64,
    minutes: u64,
    position: (i32, i32),
    hp: i32,
    mana: i32,
    gold: i32,
    bank_gold: i32,
    inventory_len: usize,
    quest_state: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct Check {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Serialize)]
struct Report {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<Check>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn snapshot_from_runtime(runtime: &FrontendRuntime) -> Snapshot {
    let session = runtime.0.session.as_ref().expect("session should exist");
    let state = &session.state;
    Snapshot {
        mode: state.mode.as_str().to_string(),
        turn: state.clock.turn,
        minutes: state.clock.minutes,
        position: (state.player.position.x, state.player.position.y),
        hp: state.player.stats.hp,
        mana: state.spellbook.mana,
        gold: state.gold,
        bank_gold: state.bank_gold,
        inventory_len: state.player.inventory.len(),
        quest_state: format!("{:?}", state.progression.quest_state),
        status: format!("{:?}", state.status),
    }
}

fn run_trace(consume_frame_each_tick: bool) -> Result<(Snapshot, Option<omega_bevy::RenderFrame>)> {
    let mut app = build_runtime_app_with_mode(0xBEE5_2002, GameMode::Classic);
    app.update();
    enqueue_input(&mut app, BevyKey::Enter);
    app.update();
    let script = [
        BevyKey::Char('w'),
        BevyKey::Char('d'),
        BevyKey::Char('g'),
        BevyKey::Char(' '),
        BevyKey::Char('i'),
        BevyKey::Esc,
        BevyKey::Char('h'),
    ];
    for key in script {
        enqueue_input(&mut app, key);
        app.update();
        if consume_frame_each_tick {
            let _ = runtime_frame(&app);
        }
    }
    let frame = runtime_frame(&app);
    let snap = {
        let runtime = app.world().resource::<FrontendRuntime>();
        snapshot_from_runtime(runtime)
    };
    Ok((snap, frame))
}

fn write_artifacts(report: &Report) -> Result<()> {
    fs::create_dir_all("target/classic").context("create target/classic")?;
    let json =
        serde_json::to_string_pretty(report).context("serialize classic drift guard json")?;
    fs::write("target/classic/classic-visual-drift-guard.json", json)
        .context("write target/classic/classic-visual-drift-guard.json")?;

    let mut md = String::new();
    md.push_str("# Classic Visual Drift Guard\n\n");
    md.push_str(&format!(
        "- generated_at_utc: `{}`\n- total: `{}`\n- passed: `{}`\n- failed: `{}`\n- status: `{}`\n\n",
        report.generated_at_utc,
        report.total,
        report.passed,
        report.failed,
        if report.pass { "PASS" } else { "FAIL" }
    ));
    for check in &report.checks {
        md.push_str(&format!(
            "- [{}] `{}`: {}\n",
            if check.pass { "PASS" } else { "FAIL" },
            check.id,
            check.details
        ));
    }
    fs::write("target/classic/classic-visual-drift-guard.md", md)
        .context("write target/classic/classic-visual-drift-guard.md")?;
    Ok(())
}

fn main() -> Result<()> {
    let (baseline, baseline_frame) = run_trace(false)?;
    let (with_reads, with_reads_frame) = run_trace(true)?;

    let mut checks = Vec::new();
    checks.push(Check {
        id: "classic_state_vector_unchanged_by_projection_reads".to_string(),
        pass: baseline == with_reads,
        details: format!("baseline={:?} with_reads={:?}", baseline, with_reads),
    });

    let frame = with_reads_frame.or(baseline_frame);
    let no_objective_marker = frame
        .as_ref()
        .map(|value| !value.tiles.iter().any(|tile| tile.kind == TileKind::ObjectiveMarker))
        .unwrap_or(false);
    checks.push(Check {
        id: "classic_has_no_modern_objective_markers".to_string(),
        pass: no_objective_marker,
        details: "classic map projection excludes modern objective marker tiles".to_string(),
    });
    let no_objective_hud = frame
        .as_ref()
        .map(|value| !value.hud_lines.iter().any(|line| line.starts_with("Objective ")))
        .unwrap_or(false);
    checks.push(Check {
        id: "classic_has_no_modern_objective_hud_lines".to_string(),
        pass: no_objective_hud,
        details: "classic HUD excludes objective guidance rows".to_string(),
    });

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = Report {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
    };
    write_artifacts(&report)?;

    if !report.pass {
        bail!("classic visual drift guard failed");
    }
    println!("classic_visual_drift_guard: PASS {}/{}", report.passed, report.total);
    Ok(())
}
