use anyhow::{Context, Result, bail};
use omega_content::{LEGACY_RAMPART_START, bootstrap_game_state_from_default_content};
use omega_core::{
    Command, DeterministicRng, Direction, Event, Position, Stats, StatusEffect, step,
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
struct MagicSubsystemSmokeReport {
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

fn markdown(report: &MagicSubsystemSmokeReport) -> String {
    let mut out = Vec::new();
    out.push("# Magic Subsystem Smoke".to_string());
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

fn note_for_token<'a>(events: &'a [Event], token: &str) -> Option<&'a str> {
    events.iter().find_map(|event| match event {
        Event::LegacyHandled { token: event_token, note, .. } if event_token == token => {
            Some(note.as_str())
        }
        _ => None,
    })
}

fn main() -> Result<()> {
    let (mut state, diagnostics) = bootstrap_game_state_from_default_content()
        .context("bootstrap default content for magic smoke")?;
    let mut rng = DeterministicRng::seeded(0x4D41_4749);
    let mut checks = Vec::new();

    let starts_in_rampart = state.player.position == LEGACY_RAMPART_START;
    checks.push(SmokeCheck {
        id: "bootstrap_starts_in_rampart".to_string(),
        pass: starts_in_rampart,
        details: format!(
            "expected=({}, {}) actual=({}, {})",
            LEGACY_RAMPART_START.x,
            LEGACY_RAMPART_START.y,
            state.player.position.x,
            state.player.position.y
        ),
    });

    for spell in &mut state.spellbook.spells {
        spell.known = true;
    }
    state.spellbook.max_mana = 5000;
    state.spellbook.mana = 5000;

    let (_direction, target) = choose_open_direction(&state)
        .context("no adjacent walkable tile available for magic smoke")?;
    state.monsters.retain(|monster| monster.position != target);
    state.spawn_monster(
        "magic-smoke-foe",
        target,
        Stats { hp: 12, max_hp: 12, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
    );

    let turn_before_open = state.clock.turn;
    let minutes_before_open = state.clock.minutes;
    let open = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let prompt_opened = note_for_token(&open.events, "m")
        .map(|note| note.starts_with("Cast Spell:"))
        .unwrap_or(false);
    let open_non_advancing =
        state.clock.turn == turn_before_open && state.clock.minutes == minutes_before_open;
    checks.push(SmokeCheck {
        id: "spell_prompt_opens_without_advancing_time".to_string(),
        pass: prompt_opened && open_non_advancing,
        details: format!(
            "prompt_opened={} turn_before={} turn_after={} minutes_before={} minutes_after={}",
            prompt_opened,
            turn_before_open,
            state.clock.turn,
            minutes_before_open,
            state.clock.minutes
        ),
    });

    let log_len_before_typing = state.log.len();
    let _ = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "g".to_string() }, &mut rng);
    let typing_non_spam = state.log.len() == log_len_before_typing;
    checks.push(SmokeCheck {
        id: "spell_typing_does_not_spam_timeline".to_string(),
        pass: typing_non_spam,
        details: format!(
            "log_len_before={} log_len_after={}",
            log_len_before_typing,
            state.log.len()
        ),
    });
    let _ = step(&mut state, Command::Legacy { token: "<esc>".to_string() }, &mut rng);

    let mana_before_cast = state.spellbook.mana;
    let _ = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "magic missile".to_string() }, &mut rng);
    let select = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let cast = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let casted = note_for_token(&select.events, "m")
        .map(|note| note.starts_with("cast spell#") && note.contains("magic missile"))
        .unwrap_or(false);
    let mana_spent = state.spellbook.mana < mana_before_cast;
    let monster_hit = cast.events.iter().any(|event| matches!(event, Event::Attacked { .. }));
    checks.push(SmokeCheck {
        id: "interactive_cast_commits_and_hits_target".to_string(),
        pass: casted && mana_spent && monster_hit,
        details: format!("casted={} mana_spent={} monster_hit={}", casted, mana_spent, monster_hit),
    });

    state.status_effects.push(StatusEffect {
        id: "fear".to_string(),
        remaining_turns: 2,
        magnitude: 1,
    });
    let mana_before_fear = state.spellbook.mana;
    let fear_out = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let fear_blocked = note_for_token(&fear_out.events, "m")
        .map(|note| note.contains("too afraid"))
        .unwrap_or(false);
    let fear_mana_unchanged = state.spellbook.mana == mana_before_fear;
    checks.push(SmokeCheck {
        id: "fear_blocks_spellcasting".to_string(),
        pass: fear_blocked && fear_mana_unchanged,
        details: format!("fear_blocked={} mana_unchanged={}", fear_blocked, fear_mana_unchanged),
    });
    state.status_effects.retain(|effect| effect.id != "fear");

    state.progression.lunarity = -1;
    state.spellbook.mana = 15;
    let _ = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "magic missile".to_string() }, &mut rng);
    let moon_out = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let contrary_moon_message = note_for_token(&moon_out.events, "m")
        .map(|note| note.contains("contrary moon"))
        .unwrap_or(false);
    checks.push(SmokeCheck {
        id: "lunarity_modifies_drain_and_blocks_cast".to_string(),
        pass: contrary_moon_message,
        details: format!("contrary_moon_message={}", contrary_moon_message),
    });
    state.progression.lunarity = 0;

    let pass = checks.iter().all(|check| check.pass);
    let report = MagicSubsystemSmokeReport {
        generated_at_utc: now_utc_unix(),
        pass,
        bootstrap_source: diagnostics.map_source,
        spawn_source: diagnostics.player_spawn_source,
        checks,
        timeline_tail: state
            .log
            .iter()
            .rev()
            .take(16)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect(),
    };

    let target_dir = Path::new("target");
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).context("create target directory")?;
    }
    let json_path = target_dir.join("magic-subsystem-smoke.json");
    let md_path = target_dir.join("magic-subsystem-smoke.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize magic subsystem smoke report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "magic subsystem smoke: status={} checks_passed={}/{}",
        if report.pass { "PASS" } else { "FAIL" },
        report.checks.iter().filter(|check| check.pass).count(),
        report.checks.len()
    );
    println!("report json: {}", json_path.display());
    println!("report md: {}", md_path.display());

    if !report.pass {
        bail!("magic subsystem smoke failed");
    }

    Ok(())
}
