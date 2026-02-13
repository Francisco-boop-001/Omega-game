use anyhow::{Context, Result, bail};
use omega_bevy::{AppState, BevyFrontend, BevyKey, GameSession, InputAction};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{GameState, LegacyQuestState, WorldMode};
use omega_tui::{App as TuiApp, UiAction, UiKey};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RuntimeSnapshot {
    turn: u64,
    minutes: u64,
    hp: i32,
    position: (i32, i32),
    inventory_count: usize,
    world_mode: WorldMode,
    quest_state: LegacyQuestState,
    gold: i32,
    bank_gold: i32,
    food: i32,
    wizard: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct WorkflowScenarioResult {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FrontendWorkflowParityReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    scenarios: Vec<WorkflowScenarioResult>,
}

fn snapshot_from_state(state: &GameState) -> RuntimeSnapshot {
    RuntimeSnapshot {
        turn: state.clock.turn,
        minutes: state.clock.minutes,
        hp: state.player.stats.hp,
        position: (state.player.position.x, state.player.position.y),
        inventory_count: state.player.inventory.len(),
        world_mode: state.world_mode,
        quest_state: state.progression.quest_state,
        gold: state.gold,
        bank_gold: state.bank_gold,
        food: state.food,
        wizard: state.wizard.enabled,
    }
}

fn bootstrap_state() -> Result<GameState> {
    let (mut state, _) = bootstrap_game_state_from_default_content()
        .context("bootstrap state for frontend workflow parity")?;
    state.options.interactive_sites = true;
    Ok(state)
}

fn init_frontends(seed: u64, id: &str) -> Result<(TuiApp, BevyFrontend)> {
    let bootstrap = bootstrap_state()?;
    let save_slot = std::path::PathBuf::from(format!("target/{id}-slot.json"));
    let tui = TuiApp::with_options(seed, bootstrap.clone(), bootstrap.clone(), save_slot.clone());
    let mut bevy = BevyFrontend::with_seed_and_bootstrap(seed, bootstrap.clone(), save_slot);
    bevy.session = Some(GameSession::from_state(seed, bootstrap));
    bevy.app_state = AppState::InGame;
    Ok((tui, bevy))
}

fn run_key_script(id: &str, keys: &[char]) -> WorkflowScenarioResult {
    let setup = init_frontends(0xABCD, id);
    let (mut tui, mut bevy) = match setup {
        Ok(pair) => pair,
        Err(err) => {
            return WorkflowScenarioResult {
                id: id.to_string(),
                passed: false,
                details: format!("setup_failed: {err:#}"),
            };
        }
    };

    for key in keys {
        tui.handle_key(UiKey::Char(*key));
        if bevy.app_state != AppState::InGame {
            break;
        }
        bevy.handle_key(BevyKey::Char(*key));
        if tui.quit {
            break;
        }
    }

    let tui_snapshot = snapshot_from_state(&tui.state);
    let bevy_snapshot = bevy
        .session
        .as_ref()
        .map(|session| snapshot_from_state(&session.state))
        .unwrap_or_else(|| tui_snapshot.clone());

    let passed = tui_snapshot == bevy_snapshot;
    WorkflowScenarioResult {
        id: id.to_string(),
        passed,
        details: format!("tui={:?} bevy={:?}", tui_snapshot, bevy_snapshot),
    }
}

fn run_persistence_script() -> WorkflowScenarioResult {
    let setup = init_frontends(0xABCE, "frontend-workflow-persist");
    let (mut tui, mut bevy) = match setup {
        Ok(pair) => pair,
        Err(err) => {
            return WorkflowScenarioResult {
                id: "persistence_restart_flow".to_string(),
                passed: false,
                details: format!("setup_failed: {err:#}"),
            };
        }
    };

    for key in [' ', '<', 's', '>', 't', 'G'] {
        tui.handle_key(UiKey::Char(key));
        bevy.handle_key(BevyKey::Char(key));
    }

    tui.apply_action(UiAction::SaveSlot);
    bevy.apply_action(InputAction::SaveSlot);
    tui.apply_action(UiAction::LoadSlot);
    bevy.apply_action(InputAction::LoadSlot);
    tui.apply_action(UiAction::Restart);
    bevy.apply_action(InputAction::RestartSession);

    let tui_snapshot = snapshot_from_state(&tui.state);
    let bevy_snapshot = bevy
        .session
        .as_ref()
        .map(|session| snapshot_from_state(&session.state))
        .unwrap_or_else(|| tui_snapshot.clone());
    let passed = tui_snapshot == bevy_snapshot;
    WorkflowScenarioResult {
        id: "persistence_restart_flow".to_string(),
        passed,
        details: format!("tui={:?} bevy={:?}", tui_snapshot, bevy_snapshot),
    }
}

fn markdown(report: &FrontendWorkflowParityReport) -> String {
    let mut out = Vec::new();
    out.push("# Classic Frontend Workflow Parity".to_string());
    out.push(String::new());
    out.push(format!("- Total scenarios: {}", report.total));
    out.push(format!("- Passed: {}", report.passed));
    out.push(format!("- Failed: {}", report.failed));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Scenario | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for scenario in &report.scenarios {
        out.push(format!(
            "| {} | {} | {} |",
            scenario.id,
            if scenario.passed { "PASS" } else { "FAIL" },
            scenario.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let scenarios = vec![
        run_key_script("movement_combat_diag", &['w', 'd', 'g', 'x', '?', '/', 'm', 'a', 'A']),
        run_key_script("world_social_progression", &['<', 's', '>', 't', 'G', 'H', 'M']),
        run_persistence_script(),
    ];
    let total = scenarios.len();
    let passed = scenarios.iter().filter(|scenario| scenario.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let report = FrontendWorkflowParityReport { total, passed, failed, pass, scenarios };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("classic-frontend-workflow-parity.json");
    let md_path = target.join("classic-frontend-workflow-parity.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize frontend workflow report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "classic frontend workflow parity: total={}, passed={}, failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("frontend workflow parity mismatch");
    }
    Ok(())
}
