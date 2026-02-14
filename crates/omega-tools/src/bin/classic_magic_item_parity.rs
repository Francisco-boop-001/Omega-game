use anyhow::{Context, Result, bail};
use omega_content::{LegacyItemFamily, legacy_item_prototypes};
use omega_core::{
    Command, DeterministicRng, Direction, GameState, ItemFamily, MapBounds, Position, Stats, step,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

const CORE_SOURCE: &str = include_str!("../../../omega-core/src/lib.rs");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct MagicItemCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct MagicItemParityReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<MagicItemCheck>,
}

fn markdown(report: &MagicItemParityReport) -> String {
    let mut out = Vec::new();
    out.push("# Classic Magic/Item Parity".to_string());
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
    let checks = vec![
        check_prototype_cardinality(),
        check_prototype_field_quality(),
        check_usef_token_coverage(),
        check_no_modeled_fallback_signature(),
        check_runtime_typed_instantiation(),
        check_command_flow_uses_typed_families(),
        check_weighted_burden(),
        check_wizard_wish_typed_output(),
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let report = MagicItemParityReport { total, passed, failed, pass, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("classic-magic-item-parity.json");
    let md_path = target.join("classic-magic-item-parity.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize magic/item parity")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "classic magic/item parity: total={}, passed={}, failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("classic magic/item parity failed");
    }
    Ok(())
}

fn check_usef_token_coverage() -> MagicItemCheck {
    let runtime_tokens = extract_usef_tokens(CORE_SOURCE);
    let legacy_usef: HashSet<String> = legacy_item_prototypes()
        .into_iter()
        .map(|item| item.usef.trim().to_string())
        .filter(|usef| !usef.is_empty() && usef.starts_with("I_"))
        .collect();
    let mut missing = legacy_usef.difference(&runtime_tokens).cloned().collect::<Vec<_>>();
    missing.sort();
    let pass = missing.is_empty();
    MagicItemCheck {
        id: "usef_runtime_token_coverage".to_string(),
        passed: pass,
        details: format!(
            "legacy_usef={} runtime_tokens={} missing={}",
            legacy_usef.len(),
            runtime_tokens.len(),
            if missing.is_empty() { "<none>".to_string() } else { missing.join(",") }
        ),
    }
}

fn check_no_modeled_fallback_signature() -> MagicItemCheck {
    let pass = !CORE_SOURCE.contains("modeled fallback for");
    MagicItemCheck {
        id: "no_modeled_item_fallback_signature".to_string(),
        passed: pass,
        details: if pass {
            "no modeled fallback signature remains in core item dispatch".to_string()
        } else {
            "core still contains modeled fallback signature text".to_string()
        },
    }
}

fn extract_usef_tokens(source: &str) -> HashSet<String> {
    let bytes = source.as_bytes();
    let mut out = HashSet::new();
    let mut i = 0usize;
    while i + 2 <= bytes.len() {
        if bytes[i] == b'I' && i + 1 < bytes.len() && bytes[i + 1] == b'_' {
            let start = i;
            let mut end = i + 2;
            while end < bytes.len() {
                let ch = bytes[end];
                if ch == b'_' || ch.is_ascii_uppercase() || ch.is_ascii_digit() {
                    end += 1;
                } else {
                    break;
                }
            }
            if end > start + 2 {
                out.insert(source[start..end].to_string());
            }
            i = end;
            continue;
        }
        i += 1;
    }
    out
}

fn check_prototype_cardinality() -> MagicItemCheck {
    let items = legacy_item_prototypes();
    let mut by_family: HashMap<LegacyItemFamily, usize> = HashMap::new();
    for item in &items {
        *by_family.entry(item.family).or_insert(0) += 1;
    }
    let pass = items.len() == 223
        && by_family.get(&LegacyItemFamily::Thing).copied().unwrap_or(0) == 31
        && by_family.get(&LegacyItemFamily::Food).copied().unwrap_or(0) == 16
        && by_family.get(&LegacyItemFamily::Scroll).copied().unwrap_or(0) == 24
        && by_family.get(&LegacyItemFamily::Potion).copied().unwrap_or(0) == 18
        && by_family.get(&LegacyItemFamily::Weapon).copied().unwrap_or(0) == 41
        && by_family.get(&LegacyItemFamily::Armor).copied().unwrap_or(0) == 17
        && by_family.get(&LegacyItemFamily::Shield).copied().unwrap_or(0) == 8
        && by_family.get(&LegacyItemFamily::Cloak).copied().unwrap_or(0) == 7
        && by_family.get(&LegacyItemFamily::Boots).copied().unwrap_or(0) == 7
        && by_family.get(&LegacyItemFamily::Ring).copied().unwrap_or(0) == 9
        && by_family.get(&LegacyItemFamily::Stick).copied().unwrap_or(0) == 17
        && by_family.get(&LegacyItemFamily::Artifact).copied().unwrap_or(0) == 26
        && by_family.get(&LegacyItemFamily::Cash).copied().unwrap_or(0) == 1
        && by_family.get(&LegacyItemFamily::Corpse).copied().unwrap_or(0) == 1;

    MagicItemCheck {
        id: "prototype_cardinality".to_string(),
        passed: pass,
        details: format!(
            "total={} weapon={} armor={} potion={} scroll={} ring={} artifact={} corpse={}",
            items.len(),
            by_family.get(&LegacyItemFamily::Weapon).copied().unwrap_or(0),
            by_family.get(&LegacyItemFamily::Armor).copied().unwrap_or(0),
            by_family.get(&LegacyItemFamily::Potion).copied().unwrap_or(0),
            by_family.get(&LegacyItemFamily::Scroll).copied().unwrap_or(0),
            by_family.get(&LegacyItemFamily::Ring).copied().unwrap_or(0),
            by_family.get(&LegacyItemFamily::Artifact).copied().unwrap_or(0),
            by_family.get(&LegacyItemFamily::Corpse).copied().unwrap_or(0),
        ),
    }
}

fn check_prototype_field_quality() -> MagicItemCheck {
    let items = legacy_item_prototypes();
    let missing_usef = items.iter().filter(|item| item.usef.trim().is_empty()).count();
    let missing_names = items.iter().filter(|item| item.truename.trim().is_empty()).count();
    let malformed = items
        .iter()
        .filter(|item| item.id_value > 222 || item.fragility < 0 || item.weight < 0)
        .count();
    let pass = missing_usef == 0 && missing_names == 0 && malformed == 0;
    MagicItemCheck {
        id: "prototype_fields".to_string(),
        passed: pass,
        details: format!(
            "missing_usef={} missing_names={} malformed={}",
            missing_usef, missing_names, malformed
        ),
    }
}

fn check_runtime_typed_instantiation() -> MagicItemCheck {
    let mut state = GameState::new(MapBounds { width: 8, height: 8 });
    let p = state.player.position;
    state.place_item("Victrix", p);
    state.place_item("potion of healing", p);
    state.place_item("scroll of identification", p);
    state.place_item("shield of deflection", p);
    let mut rng = DeterministicRng::seeded(0xC1A5_1001);
    for _ in 0..4 {
        let _ = step(&mut state, Command::Pickup, &mut rng);
    }

    let unknown =
        state.player.inventory.iter().filter(|item| item.family == ItemFamily::Unknown).count();
    let missing_usef = state.player.inventory.iter().filter(|item| item.usef.is_empty()).count();
    let has_victrix = state.player.inventory.iter().any(|item| {
        item.name == "Victrix" && item.family == ItemFamily::Weapon && item.usef == "I_VICTRIX"
    });
    let has_armorish = state
        .player
        .inventory
        .iter()
        .any(|item| item.family == ItemFamily::Shield || item.family == ItemFamily::Armor);
    let pass = unknown == 0 && missing_usef == 0 && has_victrix && has_armorish;

    MagicItemCheck {
        id: "runtime_typed_instantiation".to_string(),
        passed: pass,
        details: format!(
            "inventory={} unknown={} missing_usef={} victrix={} armorish={}",
            state.player.inventory.len(),
            unknown,
            missing_usef,
            has_victrix,
            has_armorish
        ),
    }
}

fn check_command_flow_uses_typed_families() -> MagicItemCheck {
    let mut state = GameState::new(MapBounds { width: 10, height: 10 });
    let p = state.player.position;
    state.player.stats.hp = 8;
    state.player.stats.max_hp = 20;
    state.place_item("potion of healing", p);
    state.place_item("scroll of identification", p);
    state.place_item("staff of missiles", p);
    state.place_item("Star Gem", p);
    state.spawn_monster(
        "test foe",
        Position { x: p.x + 1, y: p.y },
        Stats { hp: 8, max_hp: 8, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
    );

    let mut rng = DeterministicRng::seeded(0xC1A5_2002);
    for _ in 0..4 {
        let _ = step(&mut state, Command::Pickup, &mut rng);
    }
    let hp_before = state.player.stats.hp;
    let inv_before = state.player.inventory.len();
    let _ = step(&mut state, Command::Legacy { token: "q".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "r".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "z".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "A".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Attack(Direction::East), &mut rng);

    let hp_after = state.player.stats.hp;
    let inv_after = state.player.inventory.len();
    let monster_reduced = state.monsters.is_empty()
        || state.monsters.first().map(|m| m.stats.hp < 8).unwrap_or(false);
    let artifact_state = state.progression.quest_state != omega_core::LegacyQuestState::NotStarted;
    let pass = hp_after > hp_before
        && inv_after <= inv_before.saturating_sub(3)
        && monster_reduced
        && artifact_state;

    MagicItemCheck {
        id: "typed_command_flow".to_string(),
        passed: pass,
        details: format!(
            "hp_before={} hp_after={} inv_before={} inv_after={} monsters={} quest={:?}",
            hp_before,
            hp_after,
            inv_before,
            inv_after,
            state.monsters.len(),
            state.progression.quest_state
        ),
    }
}

fn check_weighted_burden() -> MagicItemCheck {
    let mut state = GameState::new(MapBounds { width: 8, height: 8 });
    let p = state.player.position;
    state.place_item("full plate mail", p);
    let mut rng = DeterministicRng::seeded(0xC1A5_3003);
    let _ = step(&mut state, Command::Pickup, &mut rng);

    let burden = state.carry_burden;
    let heavy_item = state
        .player
        .inventory
        .iter()
        .find(|item| item.family == ItemFamily::Armor)
        .map(|item| item.weight)
        .unwrap_or(0);
    let pass = burden >= 20 && heavy_item >= 300;
    MagicItemCheck {
        id: "weighted_burden".to_string(),
        passed: pass,
        details: format!("burden={} armor_weight={}", burden, heavy_item),
    }
}

fn check_wizard_wish_typed_output() -> MagicItemCheck {
    let mut state = GameState::new(MapBounds { width: 9, height: 9 });
    state.wizard.enabled = true;
    let mut rng = DeterministicRng::seeded(0xC1A5_4004);
    let _ = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "get item".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: ")".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

    let item = state.player.inventory.first();
    let pass = item.is_some_and(|item| {
        item.family == ItemFamily::Weapon
            && !item.usef.is_empty()
            && !item.name.contains("wishforged")
            && !item.name.contains("trinket")
    });
    MagicItemCheck {
        id: "wizard_wish_typed_output".to_string(),
        passed: pass,
        details: format!(
            "inventory={} first_family={:?} first_usef={}",
            state.player.inventory.len(),
            item.map(|entry| entry.family).unwrap_or(ItemFamily::Unknown),
            item.map(|entry| entry.usef.clone()).unwrap_or_else(|| "<none>".to_string())
        ),
    }
}
