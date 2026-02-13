use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_with_mode;
use omega_core::{Command, DeterministicRng, GameMode, SessionStatus, step};
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
struct ModernModeSmoke {
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

fn main() -> Result<()> {
    let (mut state, diagnostics) =
        bootstrap_game_state_with_mode(GameMode::Modern).context("bootstrap modern mode")?;
    let mut rng = DeterministicRng::seeded(0xA11C_E105);

    let initial_mode = state.mode;
    let initial_pos = state.player.position;
    let _ = step(&mut state, Command::Wait, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: ".".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Move(omega_core::Direction::East), &mut rng);

    let checks = vec![
        SmokeCheck {
            id: "mode_is_modern".to_string(),
            pass: initial_mode == GameMode::Modern && state.mode == GameMode::Modern,
            details: format!("initial={} current={}", initial_mode.as_str(), state.mode.as_str()),
        },
        SmokeCheck {
            id: "bootstrap_context_populated".to_string(),
            pass: !diagnostics.map_source.trim().is_empty()
                && !diagnostics.player_spawn_source.trim().is_empty(),
            details: format!(
                "world={:?} map_source={} spawn={}",
                state.world_mode, diagnostics.map_source, diagnostics.player_spawn_source
            ),
        },
        SmokeCheck {
            id: "run_remains_non_terminal".to_string(),
            pass: state.status == SessionStatus::InProgress,
            details: format!(
                "status={:?} turn={} minutes={}",
                state.status, state.clock.turn, state.clock.minutes
            ),
        },
        SmokeCheck {
            id: "player_state_mutated_by_trace".to_string(),
            pass: state.clock.turn > 0 || state.player.position != initial_pos,
            details: format!(
                "start=({}, {}) current=({}, {}) turn={}",
                initial_pos.x,
                initial_pos.y,
                state.player.position.x,
                state.player.position.y,
                state.clock.turn
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = ModernModeSmoke {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
    };

    write_json("target/modern/modern-mode-smoke.json", &report)?;
    write_json("target/modern_mode_smoke.json", &report)?;

    println!(
        "modern mode smoke: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("modern mode smoke failed");
    }
    Ok(())
}
