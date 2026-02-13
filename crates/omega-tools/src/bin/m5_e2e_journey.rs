use anyhow::{Context, Result, anyhow, bail};
use omega_bevy::{
    AppState, BevyKey, FrontendRuntime, build_runtime_app, enqueue_input, runtime_status,
};
use omega_core::{Position, SessionStatus, Stats};
use omega_save::{decode_state_json, encode_json};
use omega_tui::{App, UiKey};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct StepResult {
    name: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct JourneyRun {
    frontend: String,
    status: String,
    simulated_game_over: bool,
    started_turn: u64,
    saved_turn: u64,
    loaded_turn: u64,
    restart_turn: u64,
    steps: Vec<StepResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct E2EJourneyReport {
    schema_version: u32,
    generated_at_utc: String,
    overall_status: String,
    total_runs: usize,
    passed_runs: usize,
    failed_runs: usize,
    pending_frontends: Vec<String>,
    runs: Vec<JourneyRun>,
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn push_step(steps: &mut Vec<StepResult>, name: &str, passed: bool, details: impl Into<String>) {
    steps.push(StepResult { name: name.to_string(), passed, details: details.into() });
}

fn adjacent_spawn(origin: Position, width: i32, height: i32) -> Option<Position> {
    let candidates = [
        Position { x: origin.x + 1, y: origin.y },
        Position { x: origin.x - 1, y: origin.y },
        Position { x: origin.x, y: origin.y + 1 },
        Position { x: origin.x, y: origin.y - 1 },
    ];
    candidates.into_iter().find(|p| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height)
}

fn run_tui_journey() -> Result<JourneyRun> {
    let mut steps = Vec::new();
    let mut app = App::new(0xE2E5_0001);
    let started_turn = app.state.clock.turn;

    let new_ok = app.state.clock.turn == 0
        && app.state.clock.minutes == 0
        && app.state.player.stats.hp == app.state.player.stats.max_hp
        && !app.quit;
    push_step(
        &mut steps,
        "new_game",
        new_ok,
        format!(
            "turn={}, minutes={}, hp={}/{}",
            app.state.clock.turn,
            app.state.clock.minutes,
            app.state.player.stats.hp,
            app.state.player.stats.max_hp
        ),
    );
    if !new_ok {
        bail!("new_game validation failed");
    }

    for key in [UiKey::Char(' '), UiKey::Char('d'), UiKey::Char('g')] {
        app.handle_key(key);
    }
    if app.quit {
        bail!("unexpected quit during scripted gameplay");
    }

    let saved_state = app.state.clone();
    let saved_turn = saved_state.clock.turn;
    let raw_save = encode_json(&saved_state).context("encode save during journey")?;
    let save_ok = !raw_save.is_empty();
    push_step(
        &mut steps,
        "save",
        save_ok,
        format!("saved_turn={}, payload_bytes={}", saved_turn, raw_save.len()),
    );
    if !save_ok {
        bail!("save step produced empty payload");
    }

    for key in [UiKey::Char('a'), UiKey::Char(' ')] {
        app.handle_key(key);
    }
    let mutated_turn = app.state.clock.turn;
    let mutate_ok = mutated_turn > saved_turn;
    push_step(
        &mut steps,
        "mutate_after_save",
        mutate_ok,
        format!("saved_turn={}, mutated_turn={}", saved_turn, mutated_turn),
    );
    if !mutate_ok {
        bail!("state did not change after post-save mutations");
    }

    let loaded_state = decode_state_json(&raw_save).context("decode save during journey load")?;
    app.state = loaded_state.clone();
    app.last_outcome = None;
    let load_ok = app.state == saved_state;
    let loaded_turn = app.state.clock.turn;
    push_step(
        &mut steps,
        "load",
        load_ok,
        format!(
            "loaded_turn={}, restored_state_match={}",
            loaded_turn,
            if load_ok { "yes" } else { "no" }
        ),
    );
    if !load_ok {
        bail!("loaded state does not match saved snapshot");
    }

    app.state.player.stats.hp = 3;
    app.state.player.stats.max_hp = app.state.player.stats.max_hp.max(3);
    if let Some(spawn) =
        adjacent_spawn(app.state.player.position, app.state.bounds.width, app.state.bounds.height)
    {
        app.state.spawn_monster(
            "e2e-lethal",
            spawn,
            Stats { hp: 6, max_hp: 6, attack_min: 4, attack_max: 4, defense: 0 },
        );
    }
    let hp_before = app.state.player.stats.hp;
    app.handle_key(UiKey::Char(' '));
    let game_over_ok =
        app.state.status == SessionStatus::Lost && app.state.player.stats.hp < hp_before;
    push_step(
        &mut steps,
        "game_over",
        game_over_ok,
        format!(
            "natural defeat via enemy turn: status={:?}, hp={} -> {}",
            app.state.status, hp_before, app.state.player.stats.hp
        ),
    );
    if !game_over_ok {
        bail!("natural game_over validation failed");
    }

    app = App::new(0xE2E5_0002);
    let restart_turn = app.state.clock.turn;
    let restart_ok = restart_turn == 0
        && app.state.player.stats.hp == app.state.player.stats.max_hp
        && !app.quit;
    push_step(
        &mut steps,
        "restart",
        restart_ok,
        format!(
            "turn={}, hp={}/{}",
            restart_turn, app.state.player.stats.hp, app.state.player.stats.max_hp
        ),
    );
    if !restart_ok {
        bail!("restart validation failed");
    }

    Ok(JourneyRun {
        frontend: "tui".to_string(),
        status: "PASS".to_string(),
        simulated_game_over: false,
        started_turn,
        saved_turn,
        loaded_turn,
        restart_turn,
        steps,
    })
}

fn run_bevy_journey() -> Result<JourneyRun> {
    let mut steps = Vec::new();
    let mut app = build_runtime_app(0xE2E5_1001);
    app.update(); // Boot -> Menu

    let initial_status = runtime_status(&app);
    let new_ok = initial_status.app_state == AppState::Menu && !initial_status.should_quit;
    push_step(
        &mut steps,
        "new_game",
        new_ok,
        format!(
            "boot_state={:?}, should_quit={}",
            initial_status.app_state, initial_status.should_quit
        ),
    );
    if !new_ok {
        bail!("bevy boot/new_game validation failed");
    }

    enqueue_input(&mut app, BevyKey::Enter);
    app.update();
    let ingame_status = runtime_status(&app);
    let started_turn = {
        let runtime = app.world().resource::<FrontendRuntime>();
        let session = runtime
            .0
            .session
            .as_ref()
            .ok_or_else(|| anyhow!("bevy session missing after start"))?;
        session.state.clock.turn
    };
    let start_ok = ingame_status.app_state == AppState::InGame;
    push_step(
        &mut steps,
        "start_session",
        start_ok,
        format!("state={:?}, turn={}", ingame_status.app_state, started_turn),
    );
    if !start_ok {
        bail!("bevy session did not enter InGame");
    }

    enqueue_input(&mut app, BevyKey::Char(' '));
    enqueue_input(&mut app, BevyKey::Char('d'));
    enqueue_input(&mut app, BevyKey::Char('g'));
    app.update();

    let saved_state = {
        let runtime = app.world().resource::<FrontendRuntime>();
        let session = runtime
            .0
            .session
            .as_ref()
            .ok_or_else(|| anyhow!("bevy session missing before save"))?;
        session.state.clone()
    };
    let saved_turn = saved_state.clock.turn;
    let raw_save = encode_json(&saved_state).context("encode bevy save during journey")?;
    let save_ok = !raw_save.is_empty();
    push_step(
        &mut steps,
        "save",
        save_ok,
        format!("saved_turn={}, payload_bytes={}", saved_turn, raw_save.len()),
    );
    if !save_ok {
        bail!("bevy save step produced empty payload");
    }

    enqueue_input(&mut app, BevyKey::Char('a'));
    enqueue_input(&mut app, BevyKey::Char(' '));
    app.update();
    let mutated_turn = {
        let runtime = app.world().resource::<FrontendRuntime>();
        let session = runtime
            .0
            .session
            .as_ref()
            .ok_or_else(|| anyhow!("bevy session missing after mutation"))?;
        session.state.clock.turn
    };
    let mutate_ok = mutated_turn > saved_turn;
    push_step(
        &mut steps,
        "mutate_after_save",
        mutate_ok,
        format!("saved_turn={}, mutated_turn={}", saved_turn, mutated_turn),
    );
    if !mutate_ok {
        bail!("bevy state did not change after post-save mutation");
    }

    let loaded_state = decode_state_json(&raw_save).context("decode bevy save during load")?;
    {
        let mut runtime = app.world_mut().resource_mut::<FrontendRuntime>();
        let session = runtime
            .0
            .session
            .as_mut()
            .ok_or_else(|| anyhow!("bevy session missing during load"))?;
        session.state = loaded_state.clone();
        session.last_outcome = None;
    }
    let current_loaded_state = {
        let runtime = app.world().resource::<FrontendRuntime>();
        let session =
            runtime.0.session.as_ref().ok_or_else(|| anyhow!("bevy session missing after load"))?;
        session.state.clone()
    };
    let loaded_turn = current_loaded_state.clock.turn;
    let load_ok = current_loaded_state == saved_state;
    push_step(
        &mut steps,
        "load",
        load_ok,
        format!(
            "loaded_turn={}, restored_state_match={}",
            loaded_turn,
            if load_ok { "yes" } else { "no" }
        ),
    );
    if !load_ok {
        bail!("bevy loaded state does not match saved snapshot");
    }

    let hp_before = {
        let mut runtime = app.world_mut().resource_mut::<FrontendRuntime>();
        let session = runtime
            .0
            .session
            .as_mut()
            .ok_or_else(|| anyhow!("bevy session missing before game over"))?;
        session.state.player.stats.hp = 3;
        if let Some(spawn) = adjacent_spawn(
            session.state.player.position,
            session.state.bounds.width,
            session.state.bounds.height,
        ) {
            session.state.spawn_monster(
                "bevy-e2e-lethal",
                spawn,
                Stats { hp: 6, max_hp: 6, attack_min: 4, attack_max: 4, defense: 0 },
            );
        }
        session.state.player.stats.hp
    };
    enqueue_input(&mut app, BevyKey::Char(' '));
    app.update();
    let game_over_status = runtime_status(&app);
    let state_after = {
        let runtime = app.world().resource::<FrontendRuntime>();
        let session = runtime
            .0
            .session
            .as_ref()
            .ok_or_else(|| anyhow!("bevy session missing after game_over step"))?;
        session.state.clone()
    };
    let game_over_ok = game_over_status.app_state == AppState::GameOver
        && state_after.status == SessionStatus::Lost
        && state_after.player.stats.hp < hp_before;
    push_step(
        &mut steps,
        "game_over",
        game_over_ok,
        format!(
            "natural defeat via enemy turn: app_state={:?}, status={:?}, hp={} -> {}",
            game_over_status.app_state, state_after.status, hp_before, state_after.player.stats.hp
        ),
    );
    if !game_over_ok {
        bail!("bevy game_over transition failed");
    }

    enqueue_input(&mut app, BevyKey::Enter);
    app.update();
    let restart_status = runtime_status(&app);
    let restart_state = {
        let runtime = app.world().resource::<FrontendRuntime>();
        let session = runtime
            .0
            .session
            .as_ref()
            .ok_or_else(|| anyhow!("bevy session missing after restart"))?;
        session.state.clone()
    };
    let restart_turn = restart_state.clock.turn;
    let restart_ok = restart_status.app_state == AppState::InGame
        && restart_turn == 0
        && restart_state.player.stats.hp == restart_state.player.stats.max_hp;
    push_step(
        &mut steps,
        "restart",
        restart_ok,
        format!(
            "state={:?}, turn={}, hp={}/{}",
            restart_status.app_state,
            restart_turn,
            restart_state.player.stats.hp,
            restart_state.player.stats.max_hp
        ),
    );
    if !restart_ok {
        bail!("bevy restart validation failed");
    }

    Ok(JourneyRun {
        frontend: "bevy".to_string(),
        status: "PASS".to_string(),
        simulated_game_over: false,
        started_turn,
        saved_turn,
        loaded_turn,
        restart_turn,
        steps,
    })
}

fn markdown(report: &E2EJourneyReport) -> String {
    let mut out = Vec::new();
    out.push("# M5 E2E Journey Report".to_string());
    out.push(String::new());
    out.push(format!("- Generated at (UTC): {}", report.generated_at_utc));
    out.push(format!("- Overall status: {}", report.overall_status));
    out.push(format!("- Total runs: {}", report.total_runs));
    out.push(format!("- Passed runs: {}", report.passed_runs));
    out.push(format!("- Failed runs: {}", report.failed_runs));
    out.push(format!(
        "- Pending frontends: {}",
        if report.pending_frontends.is_empty() {
            "none".to_string()
        } else {
            report.pending_frontends.join(", ")
        }
    ));
    out.push(String::new());
    for run in &report.runs {
        out.push(format!("## Run: {}", run.frontend));
        out.push(String::new());
        out.push(format!("- Status: {}", run.status));
        out.push(format!("- Simulated game over: {}", run.simulated_game_over));
        out.push(format!("- Started turn: {}", run.started_turn));
        out.push(format!("- Saved turn: {}", run.saved_turn));
        out.push(format!("- Loaded turn: {}", run.loaded_turn));
        out.push(format!("- Restart turn: {}", run.restart_turn));
        out.push(String::new());
        out.push("| Step | Result | Details |".to_string());
        out.push("|---|---|---|".to_string());
        for step in &run.steps {
            out.push(format!(
                "| {} | {} | {} |",
                step.name,
                if step.passed { "PASS" } else { "FAIL" },
                step.details.replace('|', "\\|")
            ));
        }
        out.push(String::new());
    }
    out.join("\n")
}

fn write_outputs(report: &E2EJourneyReport) -> Result<()> {
    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }
    let json_path = target.join("m5-e2e-journey-report.json");
    let md_path = target.join("m5-e2e-journey-report.md");
    fs::write(&json_path, serde_json::to_string_pretty(report).context("serialize e2e report")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(report))
        .with_context(|| format!("write {}", md_path.display()))?;
    Ok(())
}

fn main() -> Result<()> {
    let runs = vec![run_tui_journey()?, run_bevy_journey()?];
    let passed_runs = runs.iter().filter(|run| run.status == "PASS").count();
    let failed_runs = runs.len().saturating_sub(passed_runs);
    let overall_status = if failed_runs == 0 { "PASS" } else { "FAIL" }.to_string();

    let report = E2EJourneyReport {
        schema_version: 1,
        generated_at_utc: chrono_like_now_utc(),
        overall_status,
        total_runs: runs.len(),
        passed_runs,
        failed_runs,
        pending_frontends: Vec::new(),
        runs,
    };

    write_outputs(&report)?;
    println!(
        "m5 e2e journey: total_runs={}, passed_runs={}, failed_runs={}",
        report.total_runs, report.passed_runs, report.failed_runs
    );

    if report.failed_runs > 0 {
        bail!("m5 e2e journey failures detected");
    }
    Ok(())
}

fn chrono_like_now_utc() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    // Keep dependency surface minimal: RFC3339-like UTC string from std timestamp seconds.
    // Exact formatting precision is not critical for gate artifacts.
    let secs = now.as_secs();
    format!("unix:{secs}")
}
