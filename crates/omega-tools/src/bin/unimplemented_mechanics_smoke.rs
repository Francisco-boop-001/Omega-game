use anyhow::{Result, bail};
use omega_core::{
    Command, CountryCell, CountryGrid, CountryTerrainKind, DeterministicRng, GameState, Item,
    ItemFamily, LegacyEnvironment, MapBinding, MapBounds, MapSemanticKind, Position, RandomSource,
    SessionStatus, WorldMode, step,
};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
struct SmokeCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize)]
struct SmokeReport {
    checks: Vec<SmokeCheck>,
    pass: bool,
}

#[derive(Debug, Clone)]
struct ScriptedRng {
    values: Vec<i32>,
    cursor: usize,
}

impl ScriptedRng {
    fn new(values: Vec<i32>) -> Self {
        Self { values, cursor: 0 }
    }
}

impl RandomSource for ScriptedRng {
    fn range_inclusive_i32(&mut self, min: i32, max: i32) -> i32 {
        let value = self.values.get(self.cursor).copied().unwrap_or(min);
        self.cursor = self.cursor.saturating_add(1);
        value.clamp(min, max)
    }
}

fn countryside_state(width: i32, height: i32, terrain: CountryTerrainKind) -> GameState {
    let mut state = GameState::new(MapBounds { width, height });
    state.world_mode = WorldMode::Countryside;
    state.environment = LegacyEnvironment::Countryside;
    state.map_binding = MapBinding {
        semantic: MapSemanticKind::Country,
        map_id: 0,
        level_index: 0,
        source: "runtime/smoke".to_string(),
    };
    state.map_rows = vec![".".repeat(width as usize); height as usize];
    state.country_map_rows = state.map_rows.clone();
    state.country_site_grid = vec![Default::default(); (width * height) as usize];
    state.country_grid = CountryGrid {
        width,
        height,
        cells: vec![
            CountryCell {
                glyph: '.',
                base_terrain: terrain,
                current_terrain: terrain,
                aux: 0,
                status: 0
            };
            (width * height) as usize
        ],
    };
    state
}

fn cast_spell(state: &mut GameState, rng: &mut DeterministicRng, spell_name: &str) {
    let _ = step(state, Command::Legacy { token: "m".to_string() }, rng);
    let _ = step(state, Command::Legacy { token: spell_name.to_string() }, rng);
    let _ = step(state, Command::Legacy { token: "<enter>".to_string() }, rng);
    if state.pending_targeting_interaction.is_some() {
        let _ = step(state, Command::Legacy { token: "<enter>".to_string() }, rng);
    }
}

fn markdown(report: &SmokeReport) -> String {
    let mut lines = Vec::new();
    lines.push("# Unimplemented Mechanics Smoke".to_string());
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

    let mut poppy_state = countryside_state(3, 3, CountryTerrainKind::Plains);
    poppy_state.player.position = Position { x: 0, y: 0 };
    let mut poppy_rng = ScriptedRng::new(vec![1, 100]);
    let _ = step(&mut poppy_state, Command::Move(omega_core::Direction::East), &mut poppy_rng);
    checks.push(SmokeCheck {
        id: "poppy_or_weather_can_disorient".to_string(),
        passed: poppy_state.navigation_lost || poppy_state.precipitation > 0,
        details: format!(
            "navigation_lost={} precipitation={}",
            poppy_state.navigation_lost, poppy_state.precipitation
        ),
    });

    let mut chaos_fatal = countryside_state(3, 3, CountryTerrainKind::ChaosSea);
    chaos_fatal.player.position = Position { x: 1, y: 1 };
    let mut chaos_rng = DeterministicRng::seeded(0xFACE_0002);
    let _ = step(&mut chaos_fatal, Command::Move(omega_core::Direction::East), &mut chaos_rng);
    checks.push(SmokeCheck {
        id: "chaos_sea_fatal_unprepared".to_string(),
        passed: chaos_fatal.status == SessionStatus::Lost,
        details: format!("status={:?} source={:?}", chaos_fatal.status, chaos_fatal.death_source),
    });

    let mut chaos_safe = countryside_state(3, 3, CountryTerrainKind::ChaosSea);
    chaos_safe.player.position = Position { x: 1, y: 1 };
    chaos_safe.progression.priest_rank = 1;
    let mut chaos_safe_rng = DeterministicRng::seeded(0xFACE_0003);
    let _ = step(&mut chaos_safe, Command::Move(omega_core::Direction::East), &mut chaos_safe_rng);
    checks.push(SmokeCheck {
        id: "chaos_sea_protection_branch".to_string(),
        passed: chaos_safe.status == SessionStatus::InProgress
            && chaos_safe.chaos_protection_consumed,
        details: format!(
            "status={:?} protection_used={}",
            chaos_safe.status, chaos_safe.chaos_protection_consumed
        ),
    });

    let mut enchant_state = GameState::new(MapBounds { width: 9, height: 9 });
    for spell in &mut enchant_state.spellbook.spells {
        spell.known = true;
    }
    enchant_state.player.inventory.push(Item {
        id: 10,
        name: "volatile mace".to_string(),
        family: ItemFamily::Weapon,
        plus: 13,
        usef: "I_NORMAL_WEAPON".to_string(),
        ..Item::default()
    });
    enchant_state.player.equipment.weapon_hand = Some(10);
    enchant_state.player.equipment.ready_hand = Some(10);
    let mut enchant_rng = DeterministicRng::seeded(0xFACE_0004);
    cast_spell(&mut enchant_state, &mut enchant_rng, "enchantment");
    checks.push(SmokeCheck {
        id: "over_enchant_explosion_branch".to_string(),
        passed: enchant_state.player.inventory.iter().all(|item| item.id != 10),
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
        id: 11,
        name: "cursed relic".to_string(),
        family: ItemFamily::Thing,
        blessing: -3,
        ..Item::default()
    });
    let mut bless_rng = DeterministicRng::seeded(0xFACE_0005);
    cast_spell(&mut bless_state, &mut bless_rng, "blessing");
    checks.push(SmokeCheck {
        id: "bless_disintegration_branch".to_string(),
        passed: bless_state.player.inventory.is_empty(),
        details: format!(
            "inventory_count={} log_tail={:?}",
            bless_state.player.inventory.len(),
            bless_state.log.last()
        ),
    });

    let mut decurse_state = GameState::new(MapBounds { width: 9, height: 9 });
    for spell in &mut decurse_state.spellbook.spells {
        spell.known = true;
    }
    decurse_state.player.inventory.push(Item {
        id: 12,
        name: "cursed ring".to_string(),
        family: ItemFamily::Ring,
        blessing: -3,
        used: true,
        ..Item::default()
    });
    decurse_state.player.equipment.ring_1 = Some(12);
    let mut decurse_rng = DeterministicRng::seeded(0xFACE_0006);
    cast_spell(&mut decurse_state, &mut decurse_rng, "dispelling");
    let retained_curse =
        decurse_state.player.inventory.first().map(|item| item.blessing < 0).unwrap_or(false);
    checks.push(SmokeCheck {
        id: "decurse_failure_branch".to_string(),
        passed: retained_curse,
        details: format!(
            "retained_curse={} log_tail={:?}",
            retained_curse,
            decurse_state.log.last()
        ),
    });

    let report = SmokeReport { pass: checks.iter().all(|check| check.passed), checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target)?;
    }
    fs::write(
        target.join("unimplemented-mechanics-smoke.json"),
        serde_json::to_string_pretty(&report)?,
    )?;
    fs::write(target.join("unimplemented-mechanics-smoke.md"), markdown(&report))?;

    println!(
        "unimplemented mechanics smoke: {}/{} checks passed",
        report.checks.iter().filter(|check| check.passed).count(),
        report.checks.len()
    );

    if !report.pass {
        bail!("unimplemented mechanics smoke failed");
    }
    Ok(())
}
