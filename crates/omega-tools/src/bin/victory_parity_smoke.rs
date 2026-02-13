use anyhow::{Context, Result, bail};
use omega_content::{LEGACY_RAMPART_START, bootstrap_game_state_from_default_content};
use omega_core::{
    Command, DeterministicRng, EndingKind, Event, LegacyEnvironment, SessionStatus, VictoryTrigger,
    step,
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
struct SmokeReport {
    generated_at_utc: String,
    pass: bool,
    bootstrap_source: String,
    spawn_source: String,
    checks: Vec<SmokeCheck>,
    timeline_tail: Vec<String>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn event_names(events: &[Event]) -> Vec<&'static str> {
    events
        .iter()
        .map(|event| match event {
            Event::VictoryAchieved => "VictoryAchieved",
            Event::EndingResolved { .. } => "EndingResolved",
            Event::TurnAdvanced { .. } => "TurnAdvanced",
            Event::LegacyHandled { .. } => "LegacyHandled",
            Event::Waited => "Waited",
            _ => "Other",
        })
        .collect()
}

fn markdown(report: &SmokeReport) -> String {
    let mut out = Vec::new();
    out.push("# Victory Parity Smoke".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", report.generated_at_utc));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(format!(
        "- Bootstrap: source=`{}` spawn=`{}`",
        report.bootstrap_source, report.spawn_source
    ));
    out.push(String::new());
    out.push("## Checks".to_string());
    out.push(String::new());
    for check in &report.checks {
        out.push(format!(
            "- {}: {} ({})",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.details
        ));
    }
    out.push(String::new());
    out.push("## Timeline Tail".to_string());
    out.push(String::new());
    out.push("```text".to_string());
    out.extend(report.timeline_tail.clone());
    out.push("```".to_string());
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let (mut state, diagnostics) =
        bootstrap_game_state_from_default_content().context("bootstrap for victory smoke")?;
    let mut rng = DeterministicRng::seeded(0x51C7_0AA1);

    let start_ok = state.player.position == LEGACY_RAMPART_START;

    let q_open = step(&mut state, Command::Legacy { token: "Q".to_string() }, &mut rng);
    let q_open_ok = state.pending_quit_interaction.is_some() && q_open.minutes == 0;

    let q_cancel = step(&mut state, Command::Legacy { token: "n".to_string() }, &mut rng);
    let q_cancel_ok = state.status == SessionStatus::InProgress
        && state.pending_quit_interaction.is_none()
        && q_cancel
            .events
            .iter()
            .all(|event| !matches!(event, Event::VictoryAchieved | Event::EndingResolved { .. }));

    state.environment = LegacyEnvironment::Arena;
    state.progression.arena_match_active = true;
    state.monsters.clear();
    let arena_tick = step(&mut state, Command::Wait, &mut rng);
    let arena_non_terminal = state.status == SessionStatus::InProgress
        && arena_tick
            .events
            .iter()
            .all(|event| !matches!(event, Event::VictoryAchieved | Event::EndingResolved { .. }));

    state.progression.adept_rank = 1;
    let _ = step(&mut state, Command::Legacy { token: "Q".to_string() }, &mut rng);
    let q_confirm = step(&mut state, Command::Legacy { token: "y".to_string() }, &mut rng);
    let q_confirm_ok = state.status == SessionStatus::Won
        && state.progression.victory_trigger == Some(VictoryTrigger::QuitConfirmed)
        && state.progression.ending == EndingKind::TotalWinner
        && q_confirm
            .events
            .iter()
            .any(|event| matches!(event, Event::VictoryAchieved | Event::EndingResolved { .. }));

    let checks = vec![
        SmokeCheck {
            id: "bootstrap_starts_in_rampart".to_string(),
            pass: start_ok,
            details: format!(
                "expected=({}, {}) actual=({}, {})",
                LEGACY_RAMPART_START.x,
                LEGACY_RAMPART_START.y,
                state.player.position.x,
                state.player.position.y
            ),
        },
        SmokeCheck {
            id: "q_opens_prompt".to_string(),
            pass: q_open_ok,
            details: format!("pending_quit={} events={:?}", q_open_ok, event_names(&q_open.events)),
        },
        SmokeCheck {
            id: "q_cancel_non_terminal".to_string(),
            pass: q_cancel_ok,
            details: format!(
                "status={:?} events={:?}",
                state.status,
                event_names(&q_cancel.events)
            ),
        },
        SmokeCheck {
            id: "arena_victory_non_terminal".to_string(),
            pass: arena_non_terminal,
            details: format!(
                "status={:?} events={:?}",
                state.status,
                event_names(&arena_tick.events)
            ),
        },
        SmokeCheck {
            id: "q_confirm_sets_explicit_total_winner".to_string(),
            pass: q_confirm_ok,
            details: format!(
                "status={:?} ending={:?} trigger={:?} events={:?}",
                state.status,
                state.progression.ending,
                state.progression.victory_trigger,
                event_names(&q_confirm.events)
            ),
        },
    ];

    let pass = checks.iter().all(|check| check.pass);
    let report = SmokeReport {
        generated_at_utc: now_utc_unix(),
        pass,
        bootstrap_source: diagnostics.map_source,
        spawn_source: diagnostics.player_spawn_source,
        checks,
        timeline_tail: state
            .log
            .iter()
            .rev()
            .take(14)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect(),
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("victory-parity-smoke.json");
    let md_path = target.join("victory-parity-smoke.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize victory smoke report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "victory parity smoke: status={} checks_passed={}/{}",
        if report.pass { "PASS" } else { "FAIL" },
        report.checks.iter().filter(|check| check.pass).count(),
        report.checks.len()
    );
    println!("report json: {}", json_path.display());
    println!("report md: {}", md_path.display());

    if !report.pass {
        bail!("victory parity smoke failed");
    }
    Ok(())
}
