use anyhow::{Context, Result, bail};
use omega_content::{LEGACY_RAMPART_START, bootstrap_game_state_from_default_content};
use omega_core::{Command, DeterministicRng, Direction, Event, Position, Stats, WorldMode, step};
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
struct AttackPathBumpSmokeReport {
    generated_at_utc: String,
    pass: bool,
    bootstrap_source: String,
    spawn_source: String,
    direction_used: String,
    start_position: Position,
    target_position: Position,
    end_position: Position,
    outcome_turn: u64,
    outcome_minutes: u64,
    game_minutes: u64,
    event_kinds: Vec<String>,
    timeline_tail: Vec<String>,
    checks: Vec<SmokeCheck>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn direction_label(direction: Direction) -> &'static str {
    match direction {
        Direction::North => "north",
        Direction::South => "south",
        Direction::East => "east",
        Direction::West => "west",
    }
}

fn event_kind(event: &Event) -> &'static str {
    match event {
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
    }
}

fn markdown(report: &AttackPathBumpSmokeReport) -> String {
    let mut out = Vec::new();
    out.push("# Attack Path Bump Smoke".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", report.generated_at_utc));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(format!(
        "- Bootstrap: source=`{}` spawn=`{}`",
        report.bootstrap_source, report.spawn_source
    ));
    out.push(format!(
        "- Direction: {} | Start: ({}, {}) | Target: ({}, {}) | End: ({}, {})",
        report.direction_used,
        report.start_position.x,
        report.start_position.y,
        report.target_position.x,
        report.target_position.y,
        report.end_position.x,
        report.end_position.y
    ));
    out.push(format!(
        "- Outcome turn/minutes: {}/{} | Game minutes: {}",
        report.outcome_turn, report.outcome_minutes, report.game_minutes
    ));
    out.push(String::new());
    out.push("## Event Kinds".to_string());
    out.push(String::new());
    for kind in &report.event_kinds {
        out.push(format!("- {}", kind));
    }
    out.push(String::new());
    out.push("## Timeline Tail".to_string());
    out.push(String::new());
    out.push("```text".to_string());
    out.extend(report.timeline_tail.clone());
    out.push("```".to_string());
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
    out.join("\n")
}

fn choose_open_direction(state: &omega_core::GameState) -> Option<(Direction, Position)> {
    for direction in [Direction::East, Direction::West, Direction::South, Direction::North] {
        let target = state.player.position.offset(direction);
        if !state.bounds.contains(target) {
            continue;
        }
        if !state.tile_is_walkable(target) {
            continue;
        }
        if state.monsters.iter().any(|monster| monster.position == target) {
            continue;
        }
        return Some((direction, target));
    }
    None
}

fn main() -> Result<()> {
    let (mut state, diagnostics) = bootstrap_game_state_from_default_content()
        .context("bootstrap default content for bump-attack smoke")?;
    let start = state.player.position;

    let (direction, target) =
        choose_open_direction(&state).context("no adjacent walkable tile available for smoke")?;

    state.monsters.retain(|monster| monster.position != target);
    state.spawn_monster(
        "smoke-rat",
        target,
        Stats { hp: 8, max_hp: 8, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
    );

    let mut rng = DeterministicRng::seeded(0xA771_0001);
    let out = step(&mut state, Command::Move(direction), &mut rng);

    let attacked = out
        .events
        .iter()
        .any(|event| matches!(event, Event::Attacked { .. } | Event::MonsterDefeated { .. }));
    let blocked = out.events.iter().any(|event| matches!(event, Event::MoveBlocked { .. }));
    let no_moved_event = out.events.iter().all(|event| !matches!(event, Event::Moved { .. }));
    let position_unchanged = state.player.position == start;
    let move_time_budget = out.minutes == 5 && state.clock.minutes == 5;
    let rampart_start = start == LEGACY_RAMPART_START;
    let in_city_mode = state.world_mode == WorldMode::DungeonCity;

    let checks = vec![
        SmokeCheck {
            id: "bootstrap_starts_in_rampart".to_string(),
            pass: rampart_start,
            details: format!(
                "expected=({}, {}) actual=({}, {})",
                LEGACY_RAMPART_START.x, LEGACY_RAMPART_START.y, start.x, start.y
            ),
        },
        SmokeCheck {
            id: "city_mode_active".to_string(),
            pass: in_city_mode,
            details: format!("world_mode={:?}", state.world_mode),
        },
        SmokeCheck {
            id: "bump_move_emits_attack".to_string(),
            pass: attacked,
            details: format!("attacked_event_seen={attacked}"),
        },
        SmokeCheck {
            id: "bump_move_not_blocked".to_string(),
            pass: !blocked,
            details: format!("move_blocked_event_seen={blocked}"),
        },
        SmokeCheck {
            id: "bump_move_does_not_move_player".to_string(),
            pass: position_unchanged && no_moved_event,
            details: format!(
                "position_unchanged={} moved_event_seen={}",
                position_unchanged, !no_moved_event
            ),
        },
        SmokeCheck {
            id: "bump_move_uses_move_time_budget".to_string(),
            pass: move_time_budget,
            details: format!("out_minutes={} game_minutes={}", out.minutes, state.clock.minutes),
        },
    ];

    let pass = checks.iter().all(|check| check.pass);
    let report = AttackPathBumpSmokeReport {
        generated_at_utc: now_utc_unix(),
        pass,
        bootstrap_source: diagnostics.map_source,
        spawn_source: diagnostics.player_spawn_source,
        direction_used: direction_label(direction).to_string(),
        start_position: start,
        target_position: target,
        end_position: state.player.position,
        outcome_turn: out.turn,
        outcome_minutes: out.minutes,
        game_minutes: state.clock.minutes,
        event_kinds: out.events.iter().map(|event| event_kind(event).to_string()).collect(),
        timeline_tail: state
            .log
            .iter()
            .rev()
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect(),
        checks,
    };

    let target_dir = Path::new("target");
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).context("create target directory")?;
    }
    let json_path = target_dir.join("attack-path-bump-smoke.json");
    let md_path = target_dir.join("attack-path-bump-smoke.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize attack path bump smoke report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "attack path bump smoke: status={} checks_passed={}/{}",
        if report.pass { "PASS" } else { "FAIL" },
        report.checks.iter().filter(|check| check.pass).count(),
        report.checks.len()
    );
    println!("report json: {}", json_path.display());
    println!("report md: {}", md_path.display());

    if !report.pass {
        bail!("attack path bump smoke failed");
    }
    Ok(())
}
