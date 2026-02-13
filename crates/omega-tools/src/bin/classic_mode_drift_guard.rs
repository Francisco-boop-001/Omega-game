use anyhow::{Context, Result, bail};
use omega_content::{bootstrap_game_state_with_mode, classic_content_fingerprint};
use omega_core::{Command, DeterministicRng, GameMode, GameState, Position, SessionStatus, step};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ClassicStateVector {
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
struct ClassicBaseline {
    content_fingerprint: String,
    final_vector: ClassicStateVector,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CheckRow {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ClassicModeDriftReport {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<CheckRow>,
    baseline_created: bool,
    content_fingerprint: String,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn collect_state_vector(state: &GameState) -> ClassicStateVector {
    ClassicStateVector {
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

fn run_classic_trace() -> Result<ClassicStateVector> {
    let (mut state, _) =
        bootstrap_game_state_with_mode(GameMode::Classic).context("bootstrap classic mode")?;
    let mut rng = DeterministicRng::seeded(0xC1A5_5100);
    let script = [
        Command::Wait,
        Command::Legacy { token: ".".to_string() },
        Command::Wait,
        Command::Legacy { token: ",".to_string() },
        Command::Wait,
    ];
    for command in script {
        let _ = step(&mut state, command, &mut rng);
    }
    Ok(collect_state_vector(&state))
}

fn write_json<T: Serialize>(path: &str, value: &T) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, serde_json::to_string_pretty(value).context("serialize json")?)
        .with_context(|| format!("write {path}"))
}

fn main() -> Result<()> {
    let fingerprint = classic_content_fingerprint().context("classic content fingerprint")?;
    let first_vector = run_classic_trace()?;
    let second_vector = run_classic_trace()?;
    let baseline_path = "target/classic/classic-mode-baseline.json";

    let mut baseline_created = false;
    let baseline = if Path::new(baseline_path).exists() {
        let raw =
            fs::read_to_string(baseline_path).with_context(|| format!("read {baseline_path}"))?;
        serde_json::from_str::<ClassicBaseline>(&raw)
            .with_context(|| format!("decode {baseline_path}"))?
    } else {
        baseline_created = true;
        let created = ClassicBaseline {
            content_fingerprint: fingerprint.clone(),
            final_vector: first_vector.clone(),
        };
        write_json(baseline_path, &created)?;
        created
    };

    let checks = vec![
        CheckRow {
            id: "classic_mode_bootstrap".to_string(),
            pass: first_vector.mode == GameMode::Classic.as_str(),
            details: format!("mode={}", first_vector.mode),
        },
        CheckRow {
            id: "deterministic_trace".to_string(),
            pass: first_vector == second_vector,
            details: format!(
                "first(turn={},minutes={}) second(turn={},minutes={})",
                first_vector.turn, first_vector.minutes, second_vector.turn, second_vector.minutes
            ),
        },
        CheckRow {
            id: "content_fingerprint_match".to_string(),
            pass: baseline.content_fingerprint == fingerprint,
            details: format!("baseline={} current={}", baseline.content_fingerprint, fingerprint),
        },
        CheckRow {
            id: "state_vector_match".to_string(),
            pass: baseline.final_vector == first_vector,
            details: format!(
                "baseline_status={} current_status={}",
                baseline.final_vector.status, first_vector.status
            ),
        },
        CheckRow {
            id: "non_terminal_trace".to_string(),
            pass: first_vector.status == format!("{:?}", SessionStatus::InProgress),
            details: format!("status={}", first_vector.status),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = ClassicModeDriftReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
        baseline_created,
        content_fingerprint: fingerprint,
    };

    write_json("target/classic/classic-mode-drift-guard.json", &report)?;
    write_json("target/classic_mode_drift_guard.json", &report)?;

    println!(
        "classic mode drift guard: total={} passed={} failed={} baseline_created={}",
        report.total, report.passed, report.failed, report.baseline_created
    );
    if !report.pass {
        bail!("classic mode drift guard failed");
    }
    Ok(())
}
