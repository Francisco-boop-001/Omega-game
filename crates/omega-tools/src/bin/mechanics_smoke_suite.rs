use anyhow::Result;
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{
    Command, DeterministicRng, Direction, Event, LegacyQuestionnaireAnswers, Position,
    SessionStatus, derive_legacy_questionnaire_creation, step,
};
use omega_save::{decode_state_json, encode_json};
use serde_json::Value;
use std::path::Path;

#[path = "../mechanics_shared.rs"]
mod mechanics_shared;

use mechanics_shared::{
    MechanicsSmokeReport, SmokeCheck, ensure_target_dir, smoke_to_markdown, write_json, write_text,
};

fn artifact_pass(path: &str) -> bool {
    if !Path::new(path).exists() {
        return false;
    }
    let Ok(raw) = std::fs::read_to_string(path) else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<Value>(&raw) else {
        return false;
    };
    value["pass"].as_bool().unwrap_or(false)
        || value["status"].as_str().is_some_and(|value| value == "PASS")
}

fn main() -> Result<()> {
    ensure_target_dir()?;
    let mut checks = Vec::new();

    let questionnaire = derive_legacy_questionnaire_creation(
        "Adventurer".to_string(),
        &LegacyQuestionnaireAnswers::default(),
    );
    checks.push(SmokeCheck {
        id: "main_character_creation_questionnaire".to_string(),
        passed: questionnaire.creation.name == "Adventurer" && questionnaire.profile.strength > 0,
        details: format!("archetype={}", questionnaire.creation.archetype_id),
    });

    let (mut state, _) = bootstrap_game_state_from_default_content()?;
    let mut rng = DeterministicRng::seeded(0x0BAD_5EED);
    let wait = step(&mut state, Command::Wait, &mut rng);
    checks.push(SmokeCheck {
        id: "main_turn_loop_and_time".to_string(),
        passed: wait.turn >= 1 && wait.minutes >= 6,
        details: format!("turn={} minutes={}", wait.turn, wait.minutes),
    });

    let city_to_country = step(&mut state, Command::Legacy { token: "<".to_string() }, &mut rng);
    let country_ok = state.world_mode == omega_core::WorldMode::Countryside
        && city_to_country.status == SessionStatus::InProgress;
    let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
    checks.push(SmokeCheck {
        id: "main_traversal_city_country_city".to_string(),
        passed: country_ok && state.world_mode == omega_core::WorldMode::DungeonCity,
        details: format!("final_world={:?}", state.world_mode),
    });

    state.spawn_monster(
        "training rat",
        Position { x: state.player.position.x + 1, y: state.player.position.y },
        omega_core::Stats { hp: 8, max_hp: 8, attack_min: 1, attack_max: 1, defense: 0 },
    );
    let attack = step(&mut state, Command::Move(Direction::East), &mut rng);
    checks.push(SmokeCheck {
        id: "main_combat_bump_attack".to_string(),
        passed: attack
            .events
            .iter()
            .any(|event| matches!(event, Event::Attacked { .. } | Event::MonsterDefeated { .. })),
        details: format!("events={}", attack.events.len()),
    });

    let (mut magic_state, _) = bootstrap_game_state_from_default_content()?;
    let mut magic_rng = DeterministicRng::seeded(0x0BAD_5EED);
    for spell in &mut magic_state.spellbook.spells {
        spell.known = true;
    }
    magic_state.spawn_monster(
        "imp-mage",
        Position { x: magic_state.player.position.x + 2, y: magic_state.player.position.y },
        omega_core::Stats { hp: 5, max_hp: 5, attack_min: 1, attack_max: 1, defense: 0 },
    );
    let mana_before = magic_state.spellbook.mana;
    let open_spell =
        step(&mut magic_state, Command::Legacy { token: "m".to_string() }, &mut magic_rng);
    let _ = step(
        &mut magic_state,
        Command::Legacy { token: "magic missile".to_string() },
        &mut magic_rng,
    );
    let cast =
        step(&mut magic_state, Command::Legacy { token: "<enter>".to_string() }, &mut magic_rng);
    let opened_prompt = open_spell.events.iter().any(|event| {
        matches!(
            event,
            Event::LegacyHandled { token, note, .. } if token == "m" && note.starts_with("Cast Spell:")
        )
    });
    let committed_cast = cast.events.iter().any(|event| {
        matches!(
            event,
            Event::LegacyHandled { token, note, fully_modeled: true }
                if token == "m" && note.starts_with("cast spell#")
        )
    });
    checks.push(SmokeCheck {
        id: "main_magic_casting".to_string(),
        passed: opened_prompt && committed_cast && magic_state.spellbook.mana < mana_before,
        details: format!(
            "prompt={} cast={} mana_before={} mana_after={}",
            opened_prompt, committed_cast, mana_before, magic_state.spellbook.mana
        ),
    });

    state.place_item("potion of healing", state.player.position);
    let _ = step(&mut state, Command::Pickup, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "q".to_string() }, &mut rng);
    let quaff = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);
    checks.push(SmokeCheck {
        id: "main_item_usage".to_string(),
        passed: quaff.events.iter().any(|event| {
            matches!(event, Event::LegacyHandled { token, note, .. } if token == "q" && note.contains("selected"))
        }) || state.player.inventory.iter().all(|item| item.name != "potion of healing"),
        details: format!("inventory={}", state.player.inventory.len()),
    });

    let inv = step(&mut state, Command::Legacy { token: "i".to_string() }, &mut rng);
    checks.push(SmokeCheck {
        id: "main_inventory_interaction".to_string(),
        passed: state.pending_inventory_interaction.is_some()
            || inv
                .events
                .iter()
                .any(|event| matches!(event, Event::LegacyHandled { token, .. } if token == "i")),
        details: format!("pending_inventory={}", state.pending_inventory_interaction.is_some()),
    });

    let (mut quit_state, _) = bootstrap_game_state_from_default_content()?;
    let mut quit_rng = DeterministicRng::seeded(0x0BAD_5EED);
    let quit = step(&mut quit_state, Command::Legacy { token: "Q".to_string() }, &mut quit_rng);
    let quit_handled = quit
        .events
        .iter()
        .any(|event| matches!(event, Event::LegacyHandled { token, .. } if token == "Q"));
    checks.push(SmokeCheck {
        id: "main_victory_flow_is_explicit".to_string(),
        passed: quit_state.pending_quit_interaction.is_some()
            && quit_state.status == SessionStatus::InProgress
            && quit_handled,
        details: format!(
            "pending_quit={} status={:?} handled={}",
            quit_state.pending_quit_interaction.is_some(),
            quit_state.status,
            quit_handled
        ),
    });

    let encoded = encode_json(&state)?;
    let decoded = decode_state_json(&encoded)?;
    checks.push(SmokeCheck {
        id: "main_save_load_roundtrip".to_string(),
        passed: decoded.player.position == state.player.position && decoded.status == state.status,
        details: format!("status={:?}", decoded.status),
    });

    checks.push(SmokeCheck {
        id: "secondary_site_services_matrix".to_string(),
        passed: artifact_pass("target/true-site-economy-social-matrix.json"),
        details: "target/true-site-economy-social-matrix.json".to_string(),
    });
    checks.push(SmokeCheck {
        id: "secondary_quest_matrix".to_string(),
        passed: artifact_pass("target/quest-parity-matrix.json"),
        details: "target/quest-parity-matrix.json".to_string(),
    });
    checks.push(SmokeCheck {
        id: "secondary_projectile_matrix".to_string(),
        passed: artifact_pass("target/projectile-parity-matrix.json"),
        details: "target/projectile-parity-matrix.json".to_string(),
    });
    checks.push(SmokeCheck {
        id: "secondary_overworld_matrix".to_string(),
        passed: artifact_pass("target/overworld-location-parity.json"),
        details: "target/overworld-location-parity.json".to_string(),
    });
    checks.push(SmokeCheck {
        id: "secondary_disorientation_matrix".to_string(),
        passed: artifact_pass("target/legacy-disorientation-parity.json"),
        details: "target/legacy-disorientation-parity.json".to_string(),
    });
    checks.push(SmokeCheck {
        id: "secondary_transmutation_matrix".to_string(),
        passed: artifact_pass("target/legacy-enchant-bless-decurse-parity.json"),
        details: "target/legacy-enchant-bless-decurse-parity.json".to_string(),
    });

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let report = MechanicsSmokeReport {
        generated_at_utc: mechanics_shared::now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
    };
    write_json("target/mechanics-smoke.json", &report)?;
    write_text("target/mechanics-smoke.md", &smoke_to_markdown(&report, "Mechanics Smoke Suite"))?;

    println!(
        "mechanics smoke: total={} passed={} failed={} pass={}",
        report.total, report.passed, report.failed, report.pass
    );
    Ok(())
}
