use anyhow::{Context, Result, bail};
use omega_core::{
    Command, DeterministicRng, GameState, LegacyQuestState, StatusEffect, WorldMode, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CoreModelCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CoreModelParityReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<CoreModelCheck>,
}

fn markdown(report: &CoreModelParityReport) -> String {
    let mut out = Vec::new();
    out.push("# Classic Core Model Parity".to_string());
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
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let mut state = GameState::default();
    let mut rng = DeterministicRng::seeded(0xC0DE_1001);

    let model_surface_ok = state.player_name == "Adventurer"
        && state.gold >= 0
        && state.food >= 0
        && state.spellbook.max_mana >= state.spellbook.mana
        && state.topology.dungeon_level == 0
        && state.scheduler.player_phase == 0
        && state.resistances.poison == 0
        && !state.immunities.poison;

    let _ = step(&mut state, Command::Legacy { token: "<".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
    let world_transition_ok = state.world_mode == WorldMode::DungeonCity
        && state.topology.country_region_id > 0
        && state.topology.city_site_id > 0;

    state.status_effects.push(StatusEffect {
        id: "poison".to_string(),
        remaining_turns: 2,
        magnitude: 1,
    });
    let hp_before = state.player.stats.hp;
    let _ = step(&mut state, Command::Wait, &mut rng);
    let _ = step(&mut state, Command::Wait, &mut rng);
    let status_stack_ok = state.player.stats.hp <= hp_before && state.status_effects.is_empty();

    let scheduler_ok = state.scheduler.player_phase > 0
        && state.scheduler.environment_phase > 0
        && state.scheduler.monster_phase > 0
        && state.scheduler.timed_effect_phase > 0;

    let progression_model_ok = matches!(
        state.progression.quest_state,
        LegacyQuestState::NotStarted
            | LegacyQuestState::Active
            | LegacyQuestState::ArtifactRecovered
            | LegacyQuestState::ReturnToPatron
            | LegacyQuestState::Completed
            | LegacyQuestState::Failed
    ) && state.progression.score >= 0;

    let checks = vec![
        CoreModelCheck {
            id: "model_surface".to_string(),
            passed: model_surface_ok,
            details: format!(
                "name={} gold={} food={} mana={}/{}",
                state.player_name,
                state.gold,
                state.food,
                state.spellbook.mana,
                state.spellbook.max_mana
            ),
        },
        CoreModelCheck {
            id: "world_transitions".to_string(),
            passed: world_transition_ok,
            details: format!(
                "world={:?} region={} city={}",
                state.world_mode, state.topology.country_region_id, state.topology.city_site_id
            ),
        },
        CoreModelCheck {
            id: "status_stack_semantics".to_string(),
            passed: status_stack_ok,
            details: format!("hp={} effects={}", state.player.stats.hp, state.status_effects.len()),
        },
        CoreModelCheck {
            id: "turn_scheduler".to_string(),
            passed: scheduler_ok,
            details: format!(
                "phases p={} e={} m={} t={}",
                state.scheduler.player_phase,
                state.scheduler.environment_phase,
                state.scheduler.monster_phase,
                state.scheduler.timed_effect_phase
            ),
        },
        CoreModelCheck {
            id: "progression_model".to_string(),
            passed: progression_model_ok,
            details: format!(
                "quest={:?} score={} total_winner={}",
                state.progression.quest_state,
                state.progression.score,
                state.progression.total_winner_unlocked
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let report = CoreModelParityReport { total, passed, failed, pass, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("classic-core-model-parity.json");
    let md_path = target.join("classic-core-model-parity.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize core model parity")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "classic core model parity: total={}, passed={}, failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("classic core model parity failed");
    }
    Ok(())
}
