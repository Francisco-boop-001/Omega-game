use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_with_mode;
use omega_core::{Command, DeterministicRng, Direction, GameMode, GameState, Position, step};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct StateVector {
    mode: String,
    turn: u64,
    minutes: u64,
    position: Position,
    hp: i32,
    mana: i32,
    gold: i32,
    quest_state: String,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ScenarioResult {
    id: String,
    pass: bool,
    input_trace: Vec<String>,
    classic: StateVector,
    modern: StateVector,
    failed_assertions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DualModeBlackboxReport {
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

fn write_json<T: Serialize>(path: &str, value: &T) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, serde_json::to_string_pretty(value).context("serialize json")?)
        .with_context(|| format!("write {path}"))
}

fn vector_for(state: &GameState) -> StateVector {
    StateVector {
        mode: state.mode.as_str().to_string(),
        turn: state.clock.turn,
        minutes: state.clock.minutes,
        position: state.player.position,
        hp: state.player.stats.hp,
        mana: state.spellbook.mana,
        gold: state.gold,
        quest_state: format!("{:?}", state.progression.quest_state),
        status: format!("{:?}", state.status),
    }
}

fn run_trace(mode: GameMode, script: &[Command]) -> Result<StateVector> {
    let (mut state, _) = bootstrap_game_state_with_mode(mode)?;
    let mut rng = DeterministicRng::seeded(0xD001_0000 ^ (mode as u64));
    for command in script {
        let _ = step(&mut state, command.clone(), &mut rng);
    }
    Ok(vector_for(&state))
}

fn command_label(command: &Command) -> String {
    match command {
        Command::Wait => "wait".to_string(),
        Command::Move(direction) => match direction {
            Direction::North => "move:north".to_string(),
            Direction::South => "move:south".to_string(),
            Direction::East => "move:east".to_string(),
            Direction::West => "move:west".to_string(),
        },
        Command::Attack(direction) => match direction {
            Direction::North => "attack:north".to_string(),
            Direction::South => "attack:south".to_string(),
            Direction::East => "attack:east".to_string(),
            Direction::West => "attack:west".to_string(),
        },
        Command::Pickup => "pickup".to_string(),
        Command::Drop { slot } => format!("drop:{slot}"),
        Command::Legacy { token } => format!("legacy:{token}"),
    }
}

fn run_scenario(id: &str, script: &[Command]) -> Result<ScenarioResult> {
    let classic = run_trace(GameMode::Classic, script)?;
    let modern = run_trace(GameMode::Modern, script)?;
    let classic_repeat = run_trace(GameMode::Classic, script)?;
    let modern_repeat = run_trace(GameMode::Modern, script)?;
    let mut failed_assertions = Vec::new();

    if classic.mode != GameMode::Classic.as_str() {
        failed_assertions.push("classic mode tag drifted".to_string());
    }
    if modern.mode != GameMode::Modern.as_str() {
        failed_assertions.push("modern mode tag drifted".to_string());
    }
    if classic != classic_repeat {
        failed_assertions.push("classic run is non-deterministic for same trace".to_string());
    }
    if modern != modern_repeat {
        failed_assertions.push("modern run is non-deterministic for same trace".to_string());
    }

    Ok(ScenarioResult {
        id: id.to_string(),
        pass: failed_assertions.is_empty(),
        input_trace: script.iter().map(command_label).collect(),
        classic,
        modern,
        failed_assertions,
    })
}

fn main() -> Result<()> {
    let scenarios = vec![
        run_scenario(
            "movement_and_wait",
            &[
                Command::Wait,
                Command::Move(Direction::East),
                Command::Move(Direction::West),
                Command::Legacy { token: ".".to_string() },
            ],
        )?,
        run_scenario(
            "inventory_modal_roundtrip",
            &[
                Command::Legacy { token: "i".to_string() },
                Command::Legacy { token: "x".to_string() },
                Command::Wait,
            ],
        )?,
        run_scenario(
            "social_prompt_roundtrip",
            &[
                Command::Legacy { token: "t".to_string() },
                Command::Legacy { token: "x".to_string() },
                Command::Wait,
            ],
        )?,
    ];

    let total = scenarios.len();
    let passed = scenarios.iter().filter(|scenario| scenario.pass).count();
    let failed = total.saturating_sub(passed);
    let report = DualModeBlackboxReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        scenarios,
    };

    write_json("target/dual/dual-mode-blackbox-suite.json", &report)?;
    write_json("target/dual_mode_blackbox_suite.json", &report)?;

    println!(
        "dual mode blackbox suite: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("dual mode blackbox suite failed");
    }
    Ok(())
}
