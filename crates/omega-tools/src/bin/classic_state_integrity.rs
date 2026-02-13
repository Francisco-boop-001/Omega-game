use anyhow::{Context, Result, bail};
use omega_core::{Command, DeterministicRng, Direction, GameState, SessionStatus, step};
use omega_save::{decode_state_json, encode_json};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct IntegrityScenario {
    id: String,
    iterations: usize,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct StateIntegrityReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    scenarios: Vec<IntegrityScenario>,
}

fn save_load_round_trip(state: &GameState) -> Result<GameState> {
    let raw = encode_json(state).context("encode integrity snapshot")?;
    decode_state_json(&raw).context("decode integrity snapshot")
}

fn scenario_save_load_loops(iterations: usize) -> Result<IntegrityScenario> {
    let mut state = GameState::default();
    let mut rng = DeterministicRng::seeded(0x9911);
    let script = [
        Command::Legacy { token: "<".to_string() },
        Command::Legacy { token: "s".to_string() },
        Command::Legacy { token: ">".to_string() },
        Command::Legacy { token: "t".to_string() },
        Command::Legacy { token: "G".to_string() },
        Command::Wait,
    ];

    for i in 0..iterations {
        let _ = step(&mut state, script[i % script.len()].clone(), &mut rng);
        state = save_load_round_trip(&state)?;
    }

    let passed = state.bounds.contains(state.player.position)
        && state.clock.turn as usize == iterations
        && state.gold >= 0
        && state.food >= 0;
    Ok(IntegrityScenario {
        id: "save_load_loops".to_string(),
        iterations,
        passed,
        details: format!(
            "turn={} minutes={} pos=({}, {}) gold={} food={}",
            state.clock.turn,
            state.clock.minutes,
            state.player.position.x,
            state.player.position.y,
            state.gold,
            state.food
        ),
    })
}

fn scenario_multi_dungeon_progression(iterations: usize) -> Result<IntegrityScenario> {
    let mut state = GameState::default();
    let mut rng = DeterministicRng::seeded(0x9922);
    let script = [
        Command::Legacy { token: "<".to_string() },
        Command::Legacy { token: "H".to_string() },
        Command::Legacy { token: "M".to_string() },
        Command::Legacy { token: ">".to_string() },
        Command::Move(Direction::East),
        Command::Move(Direction::West),
        Command::Legacy { token: "s".to_string() },
    ];

    for i in 0..iterations {
        let _ = step(&mut state, script[i % script.len()].clone(), &mut rng);
        if i % 5 == 4 {
            state = save_load_round_trip(&state)?;
        }
    }

    let passed = state.bounds.contains(state.player.position)
        && !state.known_sites.is_empty()
        && state.clock.turn as usize == iterations;
    Ok(IntegrityScenario {
        id: "multi_dungeon_progression".to_string(),
        iterations,
        passed,
        details: format!(
            "world={:?} known_sites={} turn={} minutes={}",
            state.world_mode,
            state.known_sites.len(),
            state.clock.turn,
            state.clock.minutes
        ),
    })
}

fn scenario_quest_branch_persistence(iterations: usize) -> Result<IntegrityScenario> {
    let mut state = GameState::default();
    let mut rng = DeterministicRng::seeded(0x9933);
    state.spawn_monster(
        "integrity-rat",
        omega_core::Position { x: state.player.position.x + 1, y: state.player.position.y },
        omega_core::Stats { hp: 4, max_hp: 4, attack_min: 1, attack_max: 1, defense: 0 },
    );
    let script = [
        Command::Legacy { token: "t".to_string() },
        Command::Legacy { token: "^g".to_string() },
        Command::Legacy { token: "^x".to_string() },
        Command::Legacy { token: "^x".to_string() },
        Command::Legacy { token: "A".to_string() },
        Command::Legacy { token: "G".to_string() },
        Command::Attack(Direction::East),
    ];

    for i in 0..iterations {
        let _ = step(&mut state, script[i % script.len()].clone(), &mut rng);
        if i % 3 == 2 {
            state = save_load_round_trip(&state)?;
        }
        if state.status != SessionStatus::InProgress {
            break;
        }
    }

    let passed = state.progression.quest_steps_completed >= 1
        && state.progression.quest_state != omega_core::LegacyQuestState::NotStarted
        && state.progression.ending != omega_core::EndingKind::None;
    Ok(IntegrityScenario {
        id: "quest_branch_persistence".to_string(),
        iterations,
        passed,
        details: format!(
            "quest={:?} steps={} ending={:?} eligible={}",
            state.progression.quest_state,
            state.progression.quest_steps_completed,
            state.progression.ending,
            state.progression.high_score_eligible
        ),
    })
}

fn markdown(report: &StateIntegrityReport) -> String {
    let mut out = Vec::new();
    out.push("# Classic State Integrity".to_string());
    out.push(String::new());
    out.push(format!("- Total scenarios: {}", report.total));
    out.push(format!("- Passed: {}", report.passed));
    out.push(format!("- Failed: {}", report.failed));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Scenario | Iterations | Status | Details |".to_string());
    out.push("|---|---:|---|---|".to_string());
    for scenario in &report.scenarios {
        out.push(format!(
            "| {} | {} | {} | {} |",
            scenario.id,
            scenario.iterations,
            if scenario.passed { "PASS" } else { "FAIL" },
            scenario.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let scenarios = vec![
        scenario_save_load_loops(60)?,
        scenario_multi_dungeon_progression(80)?,
        scenario_quest_branch_persistence(40)?,
    ];
    let total = scenarios.len();
    let passed = scenarios.iter().filter(|scenario| scenario.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let report = StateIntegrityReport { total, passed, failed, pass, scenarios };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("classic-state-integrity.json");
    let md_path = target.join("classic-state-integrity.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize integrity report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "classic state integrity: total={}, passed={}, failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("classic state integrity check failed");
    }
    Ok(())
}
