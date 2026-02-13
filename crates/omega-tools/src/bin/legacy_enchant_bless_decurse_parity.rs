use anyhow::{Result, bail};
use omega_core::{Command, DeterministicRng, GameState, Item, ItemFamily, MapBounds, step};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
struct Check {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize)]
struct Report {
    checks: Vec<Check>,
    pass: bool,
}

fn cast_spell(state: &mut GameState, rng: &mut DeterministicRng, spell_name: &str) {
    let _ = step(state, Command::Legacy { token: "m".to_string() }, rng);
    let _ = step(state, Command::Legacy { token: spell_name.to_string() }, rng);
    let _ = step(state, Command::Legacy { token: "<enter>".to_string() }, rng);
    if state.pending_targeting_interaction.is_some() {
        let _ = step(state, Command::Legacy { token: "<enter>".to_string() }, rng);
    }
}

fn markdown(report: &Report) -> String {
    let mut lines = Vec::new();
    lines.push("# Legacy Enchant/Bless/Decurse Parity".to_string());
    lines.push(String::new());
    lines.push(format!("- status: {}", if report.pass { "PASS" } else { "FAIL" }));
    lines.push(String::new());
    for check in &report.checks {
        lines.push(format!(
            "- {}: {} ({})",
            check.id,
            if check.passed { "PASS" } else { "FAIL" },
            check.details
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn main() -> Result<()> {
    let mut checks = Vec::new();

    let mut enchant_state = GameState::new(MapBounds { width: 9, height: 9 });
    for spell in &mut enchant_state.spellbook.spells {
        spell.known = true;
    }
    enchant_state.player.inventory.push(Item {
        id: 1,
        name: "unstable sword".to_string(),
        family: ItemFamily::Weapon,
        plus: 13,
        usef: "I_NORMAL_WEAPON".to_string(),
        ..Item::default()
    });
    enchant_state.player.equipment.weapon_hand = Some(1);
    enchant_state.player.equipment.ready_hand = Some(1);
    let mut rng = DeterministicRng::seeded(0xB105_0001);
    cast_spell(&mut enchant_state, &mut rng, "enchantment");
    checks.push(Check {
        id: "over_enchant_explosion".to_string(),
        passed: enchant_state.player.inventory.iter().all(|item| item.id != 1),
        details: format!(
            "inventory_count={} log_tail={:?}",
            enchant_state.player.inventory.len(),
            enchant_state.log.last()
        ),
    });

    let mut bless_state = GameState::new(MapBounds { width: 9, height: 9 });
    for spell in &mut bless_state.spellbook.spells {
        spell.known = true;
    }
    bless_state.player.inventory.push(Item {
        id: 2,
        name: "cursed amulet".to_string(),
        family: ItemFamily::Thing,
        blessing: -3,
        ..Item::default()
    });
    let mut bless_rng = DeterministicRng::seeded(0xB105_0002);
    cast_spell(&mut bless_state, &mut bless_rng, "blessing");
    checks.push(Check {
        id: "bless_disintegrates_strongly_cursed".to_string(),
        passed: bless_state.player.inventory.is_empty(),
        details: format!(
            "inventory_count={} log_tail={:?}",
            bless_state.player.inventory.len(),
            bless_state.log.last()
        ),
    });

    let mut decurse_fail_state = GameState::new(MapBounds { width: 9, height: 9 });
    for spell in &mut decurse_fail_state.spellbook.spells {
        spell.known = true;
    }
    decurse_fail_state.player.inventory.push(Item {
        id: 3,
        name: "cursed ring".to_string(),
        family: ItemFamily::Ring,
        blessing: -3,
        used: true,
        ..Item::default()
    });
    decurse_fail_state.player.equipment.ring_1 = Some(3);
    let mut decurse_rng = DeterministicRng::seeded(0xB105_0003);
    cast_spell(&mut decurse_fail_state, &mut decurse_rng, "dispelling");
    let remaining_blessing =
        decurse_fail_state.player.inventory.first().map(|item| item.blessing).unwrap_or(0);
    checks.push(Check {
        id: "decurse_failure_preserves_curse".to_string(),
        passed: remaining_blessing < 0,
        details: format!(
            "remaining_blessing={} log_tail={:?}",
            remaining_blessing,
            decurse_fail_state.log.last()
        ),
    });

    let mut decurse_ok_state = GameState::new(MapBounds { width: 9, height: 9 });
    for spell in &mut decurse_ok_state.spellbook.spells {
        spell.known = true;
    }
    decurse_ok_state.player.inventory.push(Item {
        id: 4,
        name: "slightly cursed ring".to_string(),
        family: ItemFamily::Ring,
        blessing: -1,
        used: true,
        ..Item::default()
    });
    decurse_ok_state.player.equipment.ring_1 = Some(4);
    let mut decurse_ok_rng = DeterministicRng::seeded(0xB105_0004);
    cast_spell(&mut decurse_ok_state, &mut decurse_ok_rng, "dispelling");
    let cleared_blessing =
        decurse_ok_state.player.inventory.first().map(|item| item.blessing).unwrap_or(-99);
    checks.push(Check {
        id: "decurse_success_clears_weak_curse".to_string(),
        passed: cleared_blessing == 0,
        details: format!(
            "cleared_blessing={} log_tail={:?}",
            cleared_blessing,
            decurse_ok_state.log.last()
        ),
    });

    let report = Report { pass: checks.iter().all(|check| check.passed), checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target)?;
    }
    fs::write(
        target.join("legacy-enchant-bless-decurse-parity.json"),
        serde_json::to_string_pretty(&report)?,
    )?;
    fs::write(target.join("legacy-enchant-bless-decurse-parity.md"), markdown(&report))?;

    println!(
        "legacy enchant/bless/decurse parity: {}/{} checks passed",
        report.checks.iter().filter(|check| check.passed).count(),
        report.checks.len()
    );

    if !report.pass {
        bail!("legacy enchant/bless/decurse parity failed");
    }
    Ok(())
}
