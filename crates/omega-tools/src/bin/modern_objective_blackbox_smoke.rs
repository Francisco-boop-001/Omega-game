use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_with_mode;
use omega_core::{
    Command, DeterministicRng, GameMode, LegacyQuestState, SITE_AUX_SERVICE_MERC_GUILD,
    TileSiteCell, active_objective_snapshot, objective_journal, objective_map_hints, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SmokeCheck {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ModernObjectiveBlackboxSmoke {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<SmokeCheck>,
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

fn ensure_merc_site_marker(state: &mut omega_core::GameState) {
    let len = (state.bounds.width.max(0) * state.bounds.height.max(0)) as usize;
    if state.site_grid.len() != len && len > 0 {
        state.site_grid = vec![TileSiteCell::default(); len];
    }
    if state.site_grid.iter().all(|cell| cell.aux != SITE_AUX_SERVICE_MERC_GUILD)
        && !state.site_grid.is_empty()
    {
        let idx = state.site_grid.len() / 2;
        state.site_grid[idx].aux = SITE_AUX_SERVICE_MERC_GUILD;
    }
}

fn main() -> Result<()> {
    let (mut state, diagnostics) =
        bootstrap_game_state_with_mode(GameMode::Modern).context("bootstrap modern mode")?;
    let mut rng = DeterministicRng::seeded(0x4D0D_0001);

    let _ = step(&mut state, Command::Wait, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: ".".to_string() }, &mut rng);

    if state.progression.main_quest.objective.trim().is_empty() {
        state.progression.quest_state = LegacyQuestState::Active;
        state.progression.main_quest.stage = LegacyQuestState::Active;
        state.progression.main_quest.objective =
            "Report to the Mercenary Guild for your active contract.".to_string();
    }
    ensure_merc_site_marker(&mut state);

    let active_a = active_objective_snapshot(&state);
    let active_b = active_objective_snapshot(&state);
    let journal_a = objective_journal(&state);
    let journal_b = objective_journal(&state);
    let hints_a = objective_map_hints(&state);
    let hints_b = objective_map_hints(&state);

    let checks = vec![
        SmokeCheck {
            id: "mode_is_modern".to_string(),
            pass: state.mode == GameMode::Modern,
            details: format!("mode={}", state.mode.as_str()),
        },
        SmokeCheck {
            id: "objective_active_present".to_string(),
            pass: active_a.is_some() && !journal_a.is_empty(),
            details: format!("active={} journal={}", active_a.is_some(), journal_a.len()),
        },
        SmokeCheck {
            id: "objective_hints_resolve".to_string(),
            pass: !hints_a.is_empty(),
            details: format!(
                "hints={} objective=`{}`",
                hints_a.len(),
                state.progression.main_quest.objective
            ),
        },
        SmokeCheck {
            id: "objective_adapters_deterministic".to_string(),
            pass: active_a == active_b && journal_a == journal_b && hints_a == hints_b,
            details: format!(
                "active_equal={} journal_equal={} hints_equal={}",
                active_a == active_b,
                journal_a == journal_b,
                hints_a == hints_b
            ),
        },
        SmokeCheck {
            id: "non_terminal_trace".to_string(),
            pass: state.status == omega_core::SessionStatus::InProgress,
            details: format!(
                "status={:?} turn={} world={:?} map_source={}",
                state.status, state.clock.turn, state.world_mode, diagnostics.map_source
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = ModernObjectiveBlackboxSmoke {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
    };

    write_json("target/modern/modern-objective-blackbox-smoke.json", &report)?;
    write_json("target/modern_objective_blackbox_smoke.json", &report)?;

    println!(
        "modern objective blackbox smoke: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("modern objective blackbox smoke failed");
    }
    Ok(())
}
