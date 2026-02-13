use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_with_mode;
use omega_core::{
    Command, DeterministicRng, GameMode, LegacyQuestState, Position, SITE_AUX_SERVICE_ORDER,
    TileSiteCell, active_objective_snapshot, objective_journal, objective_map_hints, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ClassicObjectiveVector {
    mode: String,
    turn: u64,
    minutes: u64,
    quest_state: String,
    objective_summary: String,
    journal_len: usize,
    hint_positions: Vec<Position>,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ClassicObjectiveBaseline {
    final_vector: ClassicObjectiveVector,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CheckRow {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ClassicObjectiveDriftReport {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<CheckRow>,
    baseline_created: bool,
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

fn collect_vector(state: &omega_core::GameState) -> ClassicObjectiveVector {
    let active_summary =
        active_objective_snapshot(state).map(|snapshot| snapshot.summary).unwrap_or_default();
    ClassicObjectiveVector {
        mode: state.mode.as_str().to_string(),
        turn: state.clock.turn,
        minutes: state.clock.minutes,
        quest_state: format!("{:?}", state.progression.quest_state),
        objective_summary: active_summary,
        journal_len: objective_journal(state).len(),
        hint_positions: objective_map_hints(state),
        status: format!("{:?}", state.status),
    }
}

fn run_trace() -> Result<ClassicObjectiveVector> {
    let (mut state, _) =
        bootstrap_game_state_with_mode(GameMode::Classic).context("bootstrap classic mode")?;
    state.progression.quest_state = LegacyQuestState::Active;
    state.progression.main_quest.stage = LegacyQuestState::Active;
    state.progression.main_quest.objective = "Report to the Order hall for duty.".to_string();
    let len = (state.bounds.width.max(0) * state.bounds.height.max(0)) as usize;
    if state.site_grid.len() != len && len > 0 {
        state.site_grid = vec![TileSiteCell::default(); len];
    }
    if !state.site_grid.is_empty() {
        let idx = state.site_grid.len() / 2;
        state.site_grid[idx].aux = SITE_AUX_SERVICE_ORDER;
    }
    let mut rng = DeterministicRng::seeded(0xC105_0010);
    let script = [Command::Wait, Command::Legacy { token: ".".to_string() }, Command::Wait];
    for command in script {
        let _ = step(&mut state, command, &mut rng);
    }
    Ok(collect_vector(&state))
}

fn main() -> Result<()> {
    let first = run_trace()?;
    let second = run_trace()?;
    let baseline_path = "target/classic/classic-objective-baseline.json";

    let mut baseline_created = false;
    let baseline = if Path::new(baseline_path).exists() {
        let raw =
            fs::read_to_string(baseline_path).with_context(|| format!("read {baseline_path}"))?;
        serde_json::from_str::<ClassicObjectiveBaseline>(&raw)
            .with_context(|| format!("decode {baseline_path}"))?
    } else {
        baseline_created = true;
        let created = ClassicObjectiveBaseline { final_vector: first.clone() };
        write_json(baseline_path, &created)?;
        created
    };

    let checks = vec![
        CheckRow {
            id: "classic_mode_bootstrap".to_string(),
            pass: first.mode == GameMode::Classic.as_str(),
            details: format!("mode={}", first.mode),
        },
        CheckRow {
            id: "deterministic_trace".to_string(),
            pass: first == second,
            details: format!(
                "first(turn={},minutes={}) second(turn={},minutes={})",
                first.turn, first.minutes, second.turn, second.minutes
            ),
        },
        CheckRow {
            id: "baseline_vector_match".to_string(),
            pass: baseline.final_vector == first,
            details: format!(
                "baseline_status={} current_status={}",
                baseline.final_vector.status, first.status
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = ClassicObjectiveDriftReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
        baseline_created,
    };

    write_json("target/classic/classic-objective-drift-guard.json", &report)?;
    write_json("target/classic_objective_drift_guard.json", &report)?;

    println!(
        "classic objective drift guard: total={} passed={} failed={} baseline_created={}",
        report.total, report.passed, report.failed, report.baseline_created
    );
    if !report.pass {
        bail!("classic objective drift guard failed");
    }
    Ok(())
}
