use anyhow::{Context, Result, bail};
use omega_core::{
    Alignment, Command, DeterministicRng, Direction, Faction, GameState, MapBounds,
    MonsterBehavior, Position, Stats, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CombatCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CombatParityReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<CombatCheck>,
}

fn markdown(report: &CombatParityReport) -> String {
    let mut out = Vec::new();
    out.push("# Classic Combat/Encounter Parity".to_string());
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
    let mut rng = DeterministicRng::seeded(0xC0DE_3001);

    let mut maneuver_state = GameState::new(MapBounds { width: 9, height: 9 });
    maneuver_state.player.position = Position { x: 4, y: 4 };
    maneuver_state.spawn_monster(
        "brute",
        Position { x: 5, y: 4 },
        Stats { hp: 6, max_hp: 6, attack_min: 2, attack_max: 2, defense: 1, weight: 60 },
    );
    let _ = step(&mut maneuver_state, Command::Legacy { token: "F".to_string() }, &mut rng);
    let sequence_mutated = maneuver_state.combat_sequence.len() > 1;
    let out_maneuver = step(&mut maneuver_state, Command::Attack(Direction::East), &mut rng);
    let attacked =
        out_maneuver.events.iter().any(|event| matches!(event, omega_core::Event::Attacked { .. }));
    let maneuver_ok = sequence_mutated && attacked;

    let mut behavior_state = GameState::new(MapBounds { width: 9, height: 9 });
    behavior_state.player.position = Position { x: 4, y: 4 };
    behavior_state.progression.alignment = Alignment::Lawful;
    let id_social = behavior_state.spawn_monster(
        "oracle-priest",
        Position { x: 5, y: 4 },
        Stats { hp: 8, max_hp: 8, attack_min: 2, attack_max: 2, defense: 1, weight: 60 },
    );
    if let Some(monster) =
        behavior_state.monsters.iter_mut().find(|monster| monster.id == id_social)
    {
        monster.behavior = MonsterBehavior::Social;
        monster.faction = Faction::Law;
    }
    let out_behavior = step(&mut behavior_state, Command::Wait, &mut rng);
    let behavior_ok = out_behavior
        .events
        .iter()
        .any(|event| matches!(event, omega_core::Event::DialogueAdvanced { .. }))
        && behavior_state.player.stats.hp == behavior_state.player.stats.max_hp;

    let mut trap_state = GameState::new(MapBounds { width: 9, height: 9 });
    trap_state.player.position = Position { x: 4, y: 4 };
    trap_state.traps = vec![omega_core::Trap {
        id: 42,
        position: Position { x: 5, y: 4 },
        damage: 3,
        effect_id: "poison".to_string(),
        armed: true,
    }];
    let hp_before = trap_state.player.stats.hp;
    let _ = step(&mut trap_state, Command::Move(Direction::East), &mut rng);
    let trap_ok = trap_state.player.stats.hp < hp_before;

    let mut faction_state = GameState::new(MapBounds { width: 9, height: 9 });
    faction_state.player.position = Position { x: 4, y: 4 };
    faction_state.progression.alignment = Alignment::Lawful;
    let law_id = faction_state.spawn_monster(
        "law-guardian",
        Position { x: 5, y: 4 },
        Stats { hp: 3, max_hp: 3, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
    );
    if let Some(monster) = faction_state.monsters.iter_mut().find(|monster| monster.id == law_id) {
        monster.faction = Faction::Law;
    }
    let _ = step(&mut faction_state, Command::Attack(Direction::East), &mut rng);
    let faction_ok = faction_state.legal_heat > 0 && faction_state.progression.law_chaos_score < 0;

    let checks = vec![
        CombatCheck {
            id: "combat_maneuver_sequence".to_string(),
            passed: maneuver_ok,
            details: "F sequence influences attack flow".to_string(),
        },
        CombatCheck {
            id: "monster_behavior_families".to_string(),
            passed: behavior_ok,
            details: "social/law monster dialogue branch honored".to_string(),
        },
        CombatCheck {
            id: "trap_hazard_interactions".to_string(),
            passed: trap_ok,
            details: format!("hp_before={} hp_after={}", hp_before, trap_state.player.stats.hp),
        },
        CombatCheck {
            id: "faction_alignment_consequences".to_string(),
            passed: faction_ok,
            details: format!(
                "legal_heat={} law_chaos={}",
                faction_state.legal_heat, faction_state.progression.law_chaos_score
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let report = CombatParityReport { total, passed, failed, pass, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("classic-combat-encounter-parity.json");
    let md_path = target.join("classic-combat-encounter-parity.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize combat parity")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "classic combat parity: total={}, passed={}, failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("classic combat/encounter parity failed");
    }
    Ok(())
}
