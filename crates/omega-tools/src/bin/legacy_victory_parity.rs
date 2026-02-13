use anyhow::{Context, Result, bail};
use omega_core::{
    Command, DeterministicRng, EndingKind, Event, GameState, LegacyEnvironment, LegacyQuestState,
    SessionStatus, VictoryTrigger, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Check {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Report {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<Check>,
}

fn event_kinds(events: &[Event]) -> Vec<&'static str> {
    events
        .iter()
        .map(|event| match event {
            Event::Waited => "Waited",
            Event::Moved { .. } => "Moved",
            Event::MoveBlocked { .. } => "MoveBlocked",
            Event::AttackMissed { .. } => "AttackMissed",
            Event::Attacked { .. } => "Attacked",
            Event::MonsterMoved { .. } => "MonsterMoved",
            Event::MonsterAttacked { .. } => "MonsterAttacked",
            Event::MonsterDefeated { .. } => "MonsterDefeated",
            Event::PlayerDefeated => "PlayerDefeated",
            Event::VictoryAchieved => "VictoryAchieved",
            Event::CommandIgnoredTerminal { .. } => "CommandIgnoredTerminal",
            Event::PickedUp { .. } => "PickedUp",
            Event::Dropped { .. } => "Dropped",
            Event::InventoryFull { .. } => "InventoryFull",
            Event::NoItemToPickUp => "NoItemToPickUp",
            Event::InvalidDropSlot { .. } => "InvalidDropSlot",
            Event::LegacyHandled { .. } => "LegacyHandled",
            Event::ConfirmationRequired { .. } => "ConfirmationRequired",
            Event::EconomyUpdated { .. } => "EconomyUpdated",
            Event::DialogueAdvanced { .. } => "DialogueAdvanced",
            Event::QuestAdvanced { .. } => "QuestAdvanced",
            Event::ProgressionUpdated { .. } => "ProgressionUpdated",
            Event::EndingResolved { .. } => "EndingResolved",
            Event::ActionPointsSpent { .. } => "ActionPointsSpent",
            Event::StatusTick { .. } => "StatusTick",
            Event::StatusExpired { .. } => "StatusExpired",
            Event::TurnAdvanced { .. } => "TurnAdvanced",
        })
        .collect()
}

fn markdown(report: &Report) -> String {
    let mut out = Vec::new();
    out.push("# Legacy Victory Parity".to_string());
    out.push(String::new());
    out.push(format!("- Total checks: {}", report.total));
    out.push(format!("- Passed: {}", report.passed));
    out.push(format!("- Failed: {}", report.failed));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Check | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for check in &report.checks {
        out.push(format!(
            "| {} | {} | {} |",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let mut rng = DeterministicRng::seeded(0x51C7_0F21);

    let mut implicit_win_state = GameState::default();
    implicit_win_state.progression.quest_state = LegacyQuestState::Completed;
    implicit_win_state.progression.main_quest.stage = LegacyQuestState::Completed;
    let implicit_out = step(&mut implicit_win_state, Command::Wait, &mut rng);
    let no_implicit_win = implicit_win_state.status == SessionStatus::InProgress
        && implicit_out
            .events
            .iter()
            .all(|event| !matches!(event, Event::VictoryAchieved | Event::EndingResolved { .. }));

    let mut quit_prompt_state = GameState::default();
    let prompt_out =
        step(&mut quit_prompt_state, Command::Legacy { token: "Q".to_string() }, &mut rng);
    let quit_prompt_ok = quit_prompt_state.pending_quit_interaction.is_some()
        && prompt_out.minutes == 0
        && prompt_out.events.iter().all(|event| {
            !matches!(
                event,
                Event::VictoryAchieved
                    | Event::EndingResolved { .. }
                    | Event::CommandIgnoredTerminal { .. }
            )
        });

    let mut quit_cancel_state = GameState::default();
    let _ = step(&mut quit_cancel_state, Command::Legacy { token: "Q".to_string() }, &mut rng);
    let cancel_out =
        step(&mut quit_cancel_state, Command::Legacy { token: "n".to_string() }, &mut rng);
    let quit_cancel_ok = quit_cancel_state.status == SessionStatus::InProgress
        && quit_cancel_state.pending_quit_interaction.is_none()
        && cancel_out
            .events
            .iter()
            .all(|event| !matches!(event, Event::VictoryAchieved | Event::EndingResolved { .. }));

    let mut normal_quit_state = GameState::default();
    let _ = step(&mut normal_quit_state, Command::Legacy { token: "Q".to_string() }, &mut rng);
    let normal_quit_out =
        step(&mut normal_quit_state, Command::Legacy { token: "y".to_string() }, &mut rng);
    let normal_quit_ok = normal_quit_state.status == SessionStatus::Won
        && normal_quit_state.progression.victory_trigger == Some(VictoryTrigger::QuitConfirmed)
        && normal_quit_state.progression.ending == EndingKind::Victory
        && normal_quit_state.progression.high_score_eligible
        && normal_quit_out
            .events
            .iter()
            .any(|event| matches!(event, Event::VictoryAchieved | Event::EndingResolved { .. }));

    let mut adept_quit_state = GameState::default();
    adept_quit_state.progression.adept_rank = 1;
    let _ = step(&mut adept_quit_state, Command::Legacy { token: "Q".to_string() }, &mut rng);
    let _ = step(&mut adept_quit_state, Command::Legacy { token: "y".to_string() }, &mut rng);
    let adept_quit_ok = adept_quit_state.status == SessionStatus::Won
        && adept_quit_state.progression.ending == EndingKind::TotalWinner
        && adept_quit_state.progression.victory_trigger == Some(VictoryTrigger::QuitConfirmed);

    let mut wizard_quit_state = GameState::default();
    let _ = step(&mut wizard_quit_state, Command::Legacy { token: "^g".to_string() }, &mut rng);
    let _ = step(&mut wizard_quit_state, Command::Legacy { token: "y".to_string() }, &mut rng);
    let _ = step(&mut wizard_quit_state, Command::Legacy { token: "Q".to_string() }, &mut rng);
    let _ = step(&mut wizard_quit_state, Command::Legacy { token: "y".to_string() }, &mut rng);
    let wizard_policy_ok = wizard_quit_state.status == SessionStatus::Won
        && !wizard_quit_state.progression.high_score_eligible
        && wizard_quit_state.progression.victory_trigger == Some(VictoryTrigger::QuitConfirmed);

    let mut arena_state = GameState {
        environment: LegacyEnvironment::Arena,
        ..Default::default()
    };
    arena_state.progression.arena_match_active = true;
    arena_state.monsters.clear();
    let arena_out = step(&mut arena_state, Command::Wait, &mut rng);
    let arena_non_terminal = arena_state.status == SessionStatus::InProgress
        && arena_out
            .events
            .iter()
            .all(|event| !matches!(event, Event::VictoryAchieved | Event::EndingResolved { .. }));

    let checks = vec![
        Check {
            id: "no_implicit_win_from_completed_quest_flags".to_string(),
            pass: no_implicit_win,
            details: format!(
                "status={:?} events={:?}",
                implicit_win_state.status,
                event_kinds(&implicit_out.events)
            ),
        },
        Check {
            id: "q_opens_quit_confirmation_prompt".to_string(),
            pass: quit_prompt_ok,
            details: format!(
                "pending_quit={} minutes={} events={:?}",
                quit_prompt_state.pending_quit_interaction.is_some(),
                prompt_out.minutes,
                event_kinds(&prompt_out.events)
            ),
        },
        Check {
            id: "q_cancel_keeps_run_alive".to_string(),
            pass: quit_cancel_ok,
            details: format!(
                "status={:?} pending_quit={} events={:?}",
                quit_cancel_state.status,
                quit_cancel_state.pending_quit_interaction.is_some(),
                event_kinds(&cancel_out.events)
            ),
        },
        Check {
            id: "q_confirm_sets_explicit_victory".to_string(),
            pass: normal_quit_ok,
            details: format!(
                "status={:?} ending={:?} trigger={:?} high_score={}",
                normal_quit_state.status,
                normal_quit_state.progression.ending,
                normal_quit_state.progression.victory_trigger,
                normal_quit_state.progression.high_score_eligible
            ),
        },
        Check {
            id: "adept_quit_yields_total_winner".to_string(),
            pass: adept_quit_ok,
            details: format!(
                "status={:?} ending={:?} trigger={:?}",
                adept_quit_state.status,
                adept_quit_state.progression.ending,
                adept_quit_state.progression.victory_trigger
            ),
        },
        Check {
            id: "wizard_quit_disables_high_score".to_string(),
            pass: wizard_policy_ok,
            details: format!(
                "status={:?} high_score={} trigger={:?}",
                wizard_quit_state.status,
                wizard_quit_state.progression.high_score_eligible,
                wizard_quit_state.progression.victory_trigger
            ),
        },
        Check {
            id: "arena_victory_remains_non_terminal".to_string(),
            pass: arena_non_terminal,
            details: format!(
                "status={:?} events={:?}",
                arena_state.status,
                event_kinds(&arena_out.events)
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let report = Report { total, passed, failed, pass, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("legacy-victory-parity.json");
    let md_path = target.join("legacy-victory-parity.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize legacy victory parity report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "legacy victory parity: status={} checks_passed={}/{}",
        if report.pass { "PASS" } else { "FAIL" },
        report.passed,
        report.total
    );
    println!("report json: {}", json_path.display());
    println!("report md: {}", md_path.display());

    if !report.pass {
        bail!("legacy victory parity failed");
    }
    Ok(())
}
