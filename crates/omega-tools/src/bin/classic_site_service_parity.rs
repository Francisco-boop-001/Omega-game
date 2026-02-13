use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{
    Command, CountryTerrainKind, DeterministicRng, GameState, LegacyEnvironment, MapSemanticKind,
    Position, SITE_AUX_SERVICE_ARENA, SITE_AUX_SERVICE_ARMORER, SITE_AUX_SERVICE_BANK,
    SITE_AUX_SERVICE_BROTHEL, SITE_AUX_SERVICE_CASINO, SITE_AUX_SERVICE_CASTLE,
    SITE_AUX_SERVICE_CHARITY, SITE_AUX_SERVICE_CLUB, SITE_AUX_SERVICE_COLLEGE,
    SITE_AUX_SERVICE_COMMANDANT, SITE_AUX_SERVICE_CONDO, SITE_AUX_SERVICE_CRAPS,
    SITE_AUX_SERVICE_DINER, SITE_AUX_SERVICE_GYM, SITE_AUX_SERVICE_HEALER,
    SITE_AUX_SERVICE_MERC_GUILD, SITE_AUX_SERVICE_MONASTERY, SITE_AUX_SERVICE_ORDER,
    SITE_AUX_SERVICE_PALACE, SITE_AUX_SERVICE_PAWN_SHOP, SITE_AUX_SERVICE_SHOP,
    SITE_AUX_SERVICE_SORCERORS, SITE_AUX_SERVICE_TAVERN, SITE_AUX_SERVICE_TEMPLE,
    SITE_AUX_SERVICE_THIEVES, step,
};
use omega_tools::replay::{load_fixture, run_fixture, run_fixture_trace};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SiteServiceCheck {
    id: String,
    label: String,
    fixture: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SiteServiceParityMatrix {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<SiteServiceCheck>,
}

fn load_and_run(
    path: &str,
) -> Result<(omega_tools::replay::ReplayScenarioResult, omega_tools::replay::FixtureTrace)> {
    let fixture = load_fixture(Path::new(path)).with_context(|| format!("load fixture {path}"))?;
    let result = run_fixture(&fixture);
    let trace = run_fixture_trace(&fixture);
    Ok((result, trace))
}

fn markdown(matrix: &SiteServiceParityMatrix) -> String {
    let mut out = Vec::new();
    out.push("# Classic Site/Service Parity Matrix".to_string());
    out.push(String::new());
    out.push(format!("- Total checks: {}", matrix.total));
    out.push(format!("- Passed: {}", matrix.passed));
    out.push(format!("- Failed: {}", matrix.failed));
    out.push(format!("- Status: {}", if matrix.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Check | Fixture | Status | Details |".to_string());
    out.push("|---|---|---|---|".to_string());
    for check in &matrix.checks {
        out.push(format!(
            "| {} | {} | {} | {} |",
            check.label,
            check.fixture,
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn tile_position_for_aux(state: &GameState, aux: i32) -> Option<Position> {
    let width = usize::try_from(state.bounds.width).ok()?;
    state.site_grid.iter().enumerate().find_map(|(idx, cell)| {
        if cell.aux != aux {
            return None;
        }
        let x = i32::try_from(idx % width).ok()?;
        let y = i32::try_from(idx / width).ok()?;
        Some(Position { x, y })
    })
}

fn choose_service_option(aux: i32) -> &'static str {
    match aux {
        SITE_AUX_SERVICE_SHOP => "1",
        SITE_AUX_SERVICE_ARMORER => "1",
        SITE_AUX_SERVICE_CLUB => "1",
        SITE_AUX_SERVICE_GYM => "1",
        SITE_AUX_SERVICE_HEALER => "1",
        SITE_AUX_SERVICE_CASINO => "1",
        SITE_AUX_SERVICE_COMMANDANT => "1",
        SITE_AUX_SERVICE_DINER => "1",
        SITE_AUX_SERVICE_CRAPS => "1",
        SITE_AUX_SERVICE_TAVERN => "1",
        SITE_AUX_SERVICE_PAWN_SHOP => "1",
        SITE_AUX_SERVICE_BROTHEL => "1",
        SITE_AUX_SERVICE_CONDO => "1",
        SITE_AUX_SERVICE_BANK => "1",
        SITE_AUX_SERVICE_MERC_GUILD => "1",
        SITE_AUX_SERVICE_THIEVES => "1",
        SITE_AUX_SERVICE_TEMPLE => "1",
        SITE_AUX_SERVICE_COLLEGE => "1",
        SITE_AUX_SERVICE_SORCERORS => "1",
        SITE_AUX_SERVICE_CASTLE => "1",
        SITE_AUX_SERVICE_PALACE => "1",
        SITE_AUX_SERVICE_ORDER => "1",
        SITE_AUX_SERVICE_CHARITY => "1",
        SITE_AUX_SERVICE_MONASTERY => "1",
        SITE_AUX_SERVICE_ARENA => "2",
        _ => "1",
    }
}

fn prepare_service_state(state: &mut GameState, aux: i32) {
    state.options.interactive_sites = true;
    match aux {
        SITE_AUX_SERVICE_SHOP => state.gold = state.gold.max(120),
        SITE_AUX_SERVICE_ARMORER => state.gold = state.gold.max(180),
        SITE_AUX_SERVICE_CLUB => {
            state.gold = state.gold.max(120);
            state.legal_heat = state.legal_heat.max(1);
        }
        SITE_AUX_SERVICE_GYM => state.gold = state.gold.max(120),
        SITE_AUX_SERVICE_HEALER => {
            state.gold = state.gold.max(120);
            state.player.stats.hp = (state.player.stats.max_hp - 5).max(1);
            state.status_effects.push(omega_core::StatusEffect {
                id: "poison".to_string(),
                remaining_turns: 3,
                magnitude: 1,
            });
        }
        SITE_AUX_SERVICE_CASINO => state.gold = state.gold.max(120),
        SITE_AUX_SERVICE_COMMANDANT => {
            state.gold = state.gold.max(120);
            state.food = 0;
            state.legal_heat = state.legal_heat.max(2);
        }
        SITE_AUX_SERVICE_DINER => {
            state.gold = state.gold.max(120);
            state.food = 0;
        }
        SITE_AUX_SERVICE_CRAPS => {
            state.gold = state.gold.max(120);
            state.legal_heat = 0;
        }
        SITE_AUX_SERVICE_TAVERN => {
            state.gold = state.gold.max(120);
            state.food = 0;
        }
        SITE_AUX_SERVICE_PAWN_SHOP => {
            state.gold = state.gold.max(120);
            if state.player.inventory.is_empty() {
                state.player.inventory.push(omega_core::Item::new(90_004, "practice trinket"));
            }
        }
        SITE_AUX_SERVICE_BROTHEL => {
            state.gold = state.gold.max(120);
            state.player.stats.hp = (state.player.stats.max_hp - 4).max(1);
        }
        SITE_AUX_SERVICE_CONDO => state.gold = state.gold.max(120),
        SITE_AUX_SERVICE_BANK => state.gold = state.gold.max(120),
        SITE_AUX_SERVICE_MERC_GUILD => {
            state.gold = state.gold.max(200);
            state.monsters_defeated = state.monsters_defeated.max(25);
        }
        SITE_AUX_SERVICE_THIEVES => {
            state.gold = state.gold.max(220);
            state.monsters_defeated = state.monsters_defeated.max(18);
            state.legal_heat = state.legal_heat.max(2);
            state.progression.alignment = omega_core::Alignment::Chaotic;
        }
        SITE_AUX_SERVICE_TEMPLE => {
            state.gold = state.gold.max(200);
            state.progression.deity_favor = state.progression.deity_favor.max(6);
        }
        SITE_AUX_SERVICE_COLLEGE => state.gold = state.gold.max(200),
        SITE_AUX_SERVICE_SORCERORS => state.gold = state.gold.max(200),
        SITE_AUX_SERVICE_CASTLE => {
            state.legal_heat = state.legal_heat.max(3);
            state.gold = state.gold.max(120);
        }
        SITE_AUX_SERVICE_PALACE => {
            state.progression.main_quest.palace_access = true;
            state.progression.main_quest.stage = omega_core::LegacyQuestState::ArtifactRecovered;
            state.gold = state.gold.max(180);
        }
        SITE_AUX_SERVICE_ORDER => {
            state.progression.alignment = omega_core::Alignment::Chaotic;
            state.gold = state.gold.max(120);
            state.legal_heat = state.legal_heat.max(2);
        }
        SITE_AUX_SERVICE_CHARITY => {
            state.player.stats.hp = (state.player.stats.max_hp - 6).max(1);
            state.food = 0;
        }
        SITE_AUX_SERVICE_MONASTERY => {
            state.gold = state.gold.max(120);
            if state.player.inventory.is_empty() {
                state.player.inventory.push(omega_core::Item::new(90_001, "practice bead"));
            }
        }
        SITE_AUX_SERVICE_ARENA => {
            state.progression.arena_rank = 0;
            state.progression.arena_opponent = 3;
        }
        _ => {}
    }
}

fn first_service_aux(state: &GameState) -> Option<i32> {
    let service_order = [
        SITE_AUX_SERVICE_SHOP,
        SITE_AUX_SERVICE_ARMORER,
        SITE_AUX_SERVICE_CLUB,
        SITE_AUX_SERVICE_GYM,
        SITE_AUX_SERVICE_HEALER,
        SITE_AUX_SERVICE_CASINO,
        SITE_AUX_SERVICE_COMMANDANT,
        SITE_AUX_SERVICE_DINER,
        SITE_AUX_SERVICE_CRAPS,
        SITE_AUX_SERVICE_TAVERN,
        SITE_AUX_SERVICE_PAWN_SHOP,
        SITE_AUX_SERVICE_BROTHEL,
        SITE_AUX_SERVICE_CONDO,
        SITE_AUX_SERVICE_BANK,
        SITE_AUX_SERVICE_MERC_GUILD,
        SITE_AUX_SERVICE_THIEVES,
        SITE_AUX_SERVICE_TEMPLE,
        SITE_AUX_SERVICE_COLLEGE,
        SITE_AUX_SERVICE_SORCERORS,
        SITE_AUX_SERVICE_CASTLE,
        SITE_AUX_SERVICE_ORDER,
        SITE_AUX_SERVICE_CHARITY,
        SITE_AUX_SERVICE_MONASTERY,
        SITE_AUX_SERVICE_ARENA,
    ];
    service_order.into_iter().find(|&aux| tile_position_for_aux(state, aux).is_some())
}

fn service_delta_passes(aux: i32, before: &GameState, after: &GameState) -> bool {
    match aux {
        SITE_AUX_SERVICE_SHOP => {
            after.gold < before.gold
                && (after.player.inventory.len() > before.player.inventory.len()
                    || after.ground_items.len() > before.ground_items.len())
        }
        SITE_AUX_SERVICE_ARMORER => {
            after.gold < before.gold
                && (after.player.inventory.len() > before.player.inventory.len()
                    || after.player.stats.defense > before.player.stats.defense)
        }
        SITE_AUX_SERVICE_CLUB => after.gold < before.gold || after.legal_heat < before.legal_heat,
        SITE_AUX_SERVICE_GYM => {
            after.player.stats.max_hp > before.player.stats.max_hp
                || after.progression.quests.merc.xp > before.progression.quests.merc.xp
        }
        SITE_AUX_SERVICE_HEALER => {
            after.player.stats.hp > before.player.stats.hp
                || after.status_effects.len() < before.status_effects.len()
        }
        SITE_AUX_SERVICE_CASINO => after.gold != before.gold || after.bank_gold != before.bank_gold,
        SITE_AUX_SERVICE_COMMANDANT => {
            after.food > before.food || after.legal_heat < before.legal_heat
        }
        SITE_AUX_SERVICE_DINER => {
            after.food > before.food || after.spellbook.mana > before.spellbook.mana
        }
        SITE_AUX_SERVICE_CRAPS => {
            after.gold != before.gold || after.legal_heat != before.legal_heat
        }
        SITE_AUX_SERVICE_TAVERN => {
            after.food > before.food
                || after.gold < before.gold
                || after.legal_heat > before.legal_heat
        }
        SITE_AUX_SERVICE_PAWN_SHOP => {
            after.gold != before.gold
                || after.player.inventory.len() != before.player.inventory.len()
        }
        SITE_AUX_SERVICE_BROTHEL => {
            after.gold < before.gold || after.player.stats.hp > before.player.stats.hp
        }
        SITE_AUX_SERVICE_CONDO => {
            after.gold < before.gold
                || after.bank_gold > before.bank_gold
                || after.player.stats.hp > before.player.stats.hp
        }
        SITE_AUX_SERVICE_BANK => after.bank_gold != before.bank_gold || after.gold != before.gold,
        SITE_AUX_SERVICE_MERC_GUILD => {
            after.progression.guild_rank > before.progression.guild_rank
                || after.player.stats.attack_max > before.player.stats.attack_max
                || after.progression.quest_state != before.progression.quest_state
        }
        SITE_AUX_SERVICE_THIEVES => {
            after.progression.quests.thieves.rank > before.progression.quests.thieves.rank
                || after.progression.quests.thieves.xp > before.progression.quests.thieves.xp
                || after.gold != before.gold
                || after.legal_heat != before.legal_heat
                || after.progression.alignment != before.progression.alignment
        }
        SITE_AUX_SERVICE_TEMPLE => {
            after.progression.priest_rank > before.progression.priest_rank
                || after.progression.deity_favor > before.progression.deity_favor
                || after.gold < before.gold
        }
        SITE_AUX_SERVICE_COLLEGE => {
            let before_known = before.spellbook.spells.iter().filter(|spell| spell.known).count();
            let after_known = after.spellbook.spells.iter().filter(|spell| spell.known).count();
            after.spellbook.max_mana > before.spellbook.max_mana
                || after_known > before_known
                || after.gold < before.gold
        }
        SITE_AUX_SERVICE_SORCERORS => {
            let before_known = before.spellbook.spells.iter().filter(|spell| spell.known).count();
            let after_known = after.spellbook.spells.iter().filter(|spell| spell.known).count();
            after.spellbook.mana != before.spellbook.mana
                || after_known > before_known
                || after.gold < before.gold
        }
        SITE_AUX_SERVICE_CASTLE => {
            after.legal_heat < before.legal_heat
                || after.progression.quest_state != before.progression.quest_state
                || after.gold != before.gold
        }
        SITE_AUX_SERVICE_PALACE => {
            after.progression.main_quest.stage != before.progression.main_quest.stage
                || after.progression.quest_state != before.progression.quest_state
                || after.gold != before.gold
        }
        SITE_AUX_SERVICE_ORDER => {
            after.progression.alignment != before.progression.alignment
                || after.legal_heat < before.legal_heat
                || after.gold < before.gold
        }
        SITE_AUX_SERVICE_CHARITY => {
            after.player.stats.hp > before.player.stats.hp
                || after.food > before.food
                || after.status_effects.len() < before.status_effects.len()
        }
        SITE_AUX_SERVICE_MONASTERY => {
            after.progression.quests.monastery.rank > before.progression.quests.monastery.rank
                || after.progression.quests.monastery.xp > before.progression.quests.monastery.xp
                || after.progression.quests.monastery.quest_flags
                    != before.progression.quests.monastery.quest_flags
                || after.gold < before.gold
        }
        SITE_AUX_SERVICE_ARENA => {
            after.environment == LegacyEnvironment::Arena
                || after.progression.arena_rank > before.progression.arena_rank
                || after.progression.arena_match_active != before.progression.arena_match_active
        }
        _ => false,
    }
}

fn service_delta_details(aux: i32, before: &GameState, after: &GameState) -> String {
    format!(
        "aux={} gold {}->{} bank {}->{} inv {}->{} guild {}->{} priest {}->{} align {:?}->{:?} env {:?}->{:?} map {}->{}",
        aux,
        before.gold,
        after.gold,
        before.bank_gold,
        after.bank_gold,
        before.player.inventory.len(),
        after.player.inventory.len(),
        before.progression.guild_rank,
        after.progression.guild_rank,
        before.progression.priest_rank,
        after.progression.priest_rank,
        before.progression.alignment,
        after.progression.alignment,
        before.environment,
        after.environment,
        before.map_binding.map_id,
        after.map_binding.map_id
    )
}

fn known_spell_count(state: &GameState) -> usize {
    state.spellbook.spells.iter().filter(|spell| spell.known).count()
}

fn has_service_placeholder_noise(state: &GameState) -> bool {
    const NEEDLES: [&str; 3] = ["audience held", "dialogue resolved with", "quest hooks processed"];
    state
        .log
        .iter()
        .any(|line| NEEDLES.iter().any(|needle| line.to_ascii_lowercase().contains(needle)))
}

fn fixture_branch_pass(id: &str, state: &GameState) -> bool {
    match id {
        "shop_service" => state.gold < 250 && !state.player.inventory.is_empty(),
        "bank_service" => state.bank_gold > 0 || state.gold < 250,
        "merc_guild_service" => {
            state.progression.guild_rank >= 1
                && state.progression.quest_state != omega_core::LegacyQuestState::NotStarted
        }
        "temple_service" => state.progression.priest_rank >= 1 || state.progression.deity_favor > 0,
        "college_service" => {
            state.gold < 250
                && (state.spellbook.max_mana > 120
                    || state.spellbook.mana != 120
                    || known_spell_count(state) > 0)
        }
        "sorcerors_service" => {
            state.gold < 250
                && (state.spellbook.mana != 120
                    || known_spell_count(state) > 0
                    || state.progression.quests.sorcerors.rank > 0
                    || state.progression.quests.sorcerors.xp > 0
                    || state
                        .player
                        .inventory
                        .iter()
                        .any(|item| item.family == omega_core::ItemFamily::Stick))
        }
        "charity_service" => {
            state.player.stats.hp >= (state.player.stats.max_hp - 2)
                || state.food >= 3
                || state.status_effects.is_empty()
        }
        "arena_service" => {
            state.environment == LegacyEnvironment::Arena
                || state.progression.arena_rank > 0
                || state.progression.arena_match_active
        }
        "order_talk" => {
            state.progression.alignment == omega_core::Alignment::Lawful
                && state.progression.quest_state != omega_core::LegacyQuestState::NotStarted
        }
        "castle_talk" => state.progression.quest_state != omega_core::LegacyQuestState::NotStarted,
        "city_economy_loop" => {
            state.gold != 250
                || state.bank_gold != 0
                || state.progression.guild_rank > 0
                || state.progression.quest_state != omega_core::LegacyQuestState::NotStarted
        }
        "countryside_exploration" => state.clock.turn > 0 && state.clock.minutes > 0,
        _ => false,
    }
}

fn run_service_delta_check(
    id: &str,
    label: &str,
    fixture: &str,
    base: &GameState,
    aux: i32,
) -> SiteServiceCheck {
    let mut state = base.clone();
    prepare_service_state(&mut state, aux);

    let Some(service_pos) = tile_position_for_aux(&state, aux) else {
        return SiteServiceCheck {
            id: id.to_string(),
            label: label.to_string(),
            fixture: fixture.to_string(),
            passed: false,
            details: format!(
                "service aux {} not present on map_id={}",
                aux, state.map_binding.map_id
            ),
        };
    };

    state.player.position = service_pos;
    let before = state.clone();
    let mut rng = DeterministicRng::seeded(0x5EED_0101);
    let open = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
    let _choose = step(
        &mut state,
        Command::Legacy { token: choose_service_option(aux).to_string() },
        &mut rng,
    );

    let opened_menu = open
        .events
        .iter()
        .any(|event| matches!(event, omega_core::Event::LegacyHandled { token, .. } if token == "interaction"));
    let delta_ok = service_delta_passes(aux, &before, &state);
    let passed = opened_menu && delta_ok;
    let details = format!(
        "opened_menu={} delta_ok={} {}",
        opened_menu,
        delta_ok,
        service_delta_details(aux, &before, &state)
    );
    SiteServiceCheck {
        id: id.to_string(),
        label: label.to_string(),
        fixture: fixture.to_string(),
        passed,
        details,
    }
}

fn village_expected_map(aux: u8) -> Option<u16> {
    match aux {
        1 => Some(14),
        2 => Some(19),
        3 => Some(15),
        4 => Some(17),
        5 => Some(12),
        6 => Some(18),
        _ => None,
    }
}

fn village_position(base: &GameState, aux: u8) -> Option<Position> {
    let width = usize::try_from(base.country_grid.width).ok()?;
    base.country_grid.cells.iter().enumerate().find_map(|(idx, cell)| {
        if cell.base_terrain != CountryTerrainKind::Village || cell.aux != aux {
            return None;
        }
        let x = i32::try_from(idx % width).ok()?;
        let y = i32::try_from(idx / width).ok()?;
        Some(Position { x, y })
    })
}

fn build_fixture_checks() -> Result<Vec<SiteServiceCheck>> {
    let fixtures = vec![
        ("shop_service", "Shop service", "crates/omega-tools/fixtures/replay/r3_site_shop.json"),
        ("bank_service", "Bank service", "crates/omega-tools/fixtures/replay/r3_site_bank.json"),
        (
            "merc_guild_service",
            "Merc guild training",
            "crates/omega-tools/fixtures/replay/r3_site_merc_guild.json",
        ),
        (
            "temple_service",
            "Temple service",
            "crates/omega-tools/fixtures/replay/r3_site_temple.json",
        ),
        (
            "college_service",
            "College service",
            "crates/omega-tools/fixtures/replay/r3_site_college.json",
        ),
        (
            "sorcerors_service",
            "Sorcerors service",
            "crates/omega-tools/fixtures/replay/r3_site_sorcerors.json",
        ),
        (
            "charity_service",
            "Charity service",
            "crates/omega-tools/fixtures/replay/r3_site_charity.json",
        ),
        ("arena_service", "Arena service", "crates/omega-tools/fixtures/replay/r3_site_arena.json"),
        (
            "order_talk",
            "Order dialogue",
            "crates/omega-tools/fixtures/replay/r3_site_order_talk.json",
        ),
        (
            "castle_talk",
            "Castle dialogue",
            "crates/omega-tools/fixtures/replay/r3_site_castle_talk.json",
        ),
        (
            "city_economy_loop",
            "City economy loop",
            "crates/omega-tools/fixtures/replay/p5_city_service_social.json",
        ),
        (
            "countryside_exploration",
            "Countryside exploration/hunt",
            "crates/omega-tools/fixtures/replay/p5_countryside_hunt_travel.json",
        ),
    ];

    let mut checks = Vec::new();
    for (id, label, fixture) in fixtures {
        let (result, trace) = load_and_run(fixture)?;
        let state = trace.final_state;
        let no_open_prompt = state.pending_site_interaction.is_none();
        let branch_pass = fixture_branch_pass(id, &state);
        let no_placeholder_noise = !has_service_placeholder_noise(&state);
        let details = format!(
            "branch_pass={} fixture_pass={} no_open_prompt={} no_placeholder_noise={} final_turn={} gold={} bank={} guild_rank={} priest_rank={} alignment={:?} quest={:?} food={} monsters={}",
            branch_pass,
            result.passed,
            no_open_prompt,
            no_placeholder_noise,
            result.final_turn,
            state.gold,
            state.bank_gold,
            state.progression.guild_rank,
            state.progression.priest_rank,
            state.progression.alignment,
            state.progression.quest_state,
            state.food,
            state.monsters.len()
        );
        checks.push(SiteServiceCheck {
            id: id.to_string(),
            label: label.to_string(),
            fixture: fixture.to_string(),
            passed: result.passed && branch_pass && no_open_prompt && no_placeholder_noise,
            details,
        });
    }
    Ok(checks)
}

fn build_runtime_city_checks(base: &GameState) -> Vec<SiteServiceCheck> {
    let mut checks = Vec::new();
    let required = [
        SITE_AUX_SERVICE_SHOP,
        SITE_AUX_SERVICE_ARMORER,
        SITE_AUX_SERVICE_CLUB,
        SITE_AUX_SERVICE_GYM,
        SITE_AUX_SERVICE_HEALER,
        SITE_AUX_SERVICE_CASINO,
        SITE_AUX_SERVICE_COMMANDANT,
        SITE_AUX_SERVICE_DINER,
        SITE_AUX_SERVICE_CRAPS,
        SITE_AUX_SERVICE_TAVERN,
        SITE_AUX_SERVICE_PAWN_SHOP,
        SITE_AUX_SERVICE_BROTHEL,
        SITE_AUX_SERVICE_CONDO,
        SITE_AUX_SERVICE_BANK,
        SITE_AUX_SERVICE_MERC_GUILD,
        SITE_AUX_SERVICE_THIEVES,
        SITE_AUX_SERVICE_TEMPLE,
        SITE_AUX_SERVICE_ORDER,
        SITE_AUX_SERVICE_CASTLE,
        SITE_AUX_SERVICE_MONASTERY,
        SITE_AUX_SERVICE_ARENA,
    ];
    let present =
        required.iter().filter(|aux| tile_position_for_aux(base, **aux).is_some()).count();
    checks.push(SiteServiceCheck {
        id: "rampart_service_presence".to_string(),
        label: "Rampart required services present".to_string(),
        fixture: "runtime/rampart".to_string(),
        passed: present == required.len(),
        details: format!(
            "present={}/{} map_id={} env={:?}",
            present,
            required.len(),
            base.map_binding.map_id,
            base.environment
        ),
    });

    for aux in required {
        checks.push(run_service_delta_check(
            &format!("rampart_service_delta_{aux}"),
            &format!("Rampart service aux {aux} delta"),
            "runtime/rampart",
            base,
            aux,
        ));
    }
    checks
}

fn build_runtime_village_checks(base: &GameState) -> Vec<SiteServiceCheck> {
    let mut checks = Vec::new();
    for village_aux in 1u8..=6 {
        let id_prefix = format!("village_{}", village_aux);
        let Some(country_pos) = village_position(base, village_aux) else {
            checks.push(SiteServiceCheck {
                id: format!("{id_prefix}_entry"),
                label: format!("Village {} entry", village_aux),
                fixture: "runtime/country".to_string(),
                passed: false,
                details: "village marker not found on country map".to_string(),
            });
            continue;
        };

        let mut state = base.clone();
        state.options.interactive_sites = true;
        let mut rng = DeterministicRng::seeded(0x5EED_0202 + u64::from(village_aux));
        let _ = step(&mut state, Command::Legacy { token: "<".to_string() }, &mut rng);
        state.player.position = country_pos;
        let before_map = state.map_binding.map_id;
        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);

        let expected_map = village_expected_map(village_aux).unwrap_or(0);
        let entry_pass = state.environment == LegacyEnvironment::Village
            && state.map_binding.semantic == MapSemanticKind::Site
            && state.map_binding.map_id == expected_map
            && before_map != state.map_binding.map_id;
        checks.push(SiteServiceCheck {
            id: format!("{id_prefix}_entry"),
            label: format!("Village {} map entry", village_aux),
            fixture: "runtime/country".to_string(),
            passed: entry_pass,
            details: format!(
                "country_pos=({}, {}) expected_map={} actual_map={} env={:?} semantic={:?}",
                country_pos.x,
                country_pos.y,
                expected_map,
                state.map_binding.map_id,
                state.environment,
                state.map_binding.semantic
            ),
        });

        let Some(aux) = first_service_aux(&state) else {
            checks.push(SiteServiceCheck {
                id: format!("{id_prefix}_service"),
                label: format!("Village {} service delta", village_aux),
                fixture: "runtime/village".to_string(),
                passed: false,
                details: "no service tiles found in village map".to_string(),
            });
            continue;
        };

        checks.push(run_service_delta_check(
            &format!("{id_prefix}_service"),
            &format!("Village {} service delta", village_aux),
            "runtime/village",
            &state,
            aux,
        ));
    }
    checks
}

fn main() -> Result<()> {
    let mut checks = build_fixture_checks()?;

    let (base, diagnostics) = bootstrap_game_state_from_default_content()
        .context("bootstrap content state for runtime site matrix")?;
    checks.push(SiteServiceCheck {
        id: "runtime_bootstrap_valid".to_string(),
        label: "Runtime bootstrap baseline".to_string(),
        fixture: "runtime/bootstrap".to_string(),
        passed: base.map_binding.map_id > 0 && !base.country_grid.cells.is_empty(),
        details: format!(
            "map_source={} spawn={} city_map={} country_cells={}",
            diagnostics.map_source,
            diagnostics.player_spawn_source,
            base.map_binding.map_id,
            base.country_grid.cells.len()
        ),
    });
    checks.extend(build_runtime_city_checks(&base));
    checks.extend(build_runtime_village_checks(&base));

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let matrix = SiteServiceParityMatrix { total, passed, failed, pass, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }

    let json_payload =
        serde_json::to_string_pretty(&matrix).context("serialize site/service matrix")?;
    let md_payload = markdown(&matrix);
    let json_path = target.join("classic-site-service-parity-matrix.json");
    let md_path = target.join("classic-site-service-parity-matrix.md");
    fs::write(&json_path, &json_payload)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, &md_payload).with_context(|| format!("write {}", md_path.display()))?;

    let true_json_path = target.join("true-site-service-parity-matrix.json");
    let true_md_path = target.join("true-site-service-parity-matrix.md");
    fs::write(&true_json_path, &json_payload)
        .with_context(|| format!("write {}", true_json_path.display()))?;
    fs::write(&true_md_path, &md_payload)
        .with_context(|| format!("write {}", true_md_path.display()))?;

    println!(
        "classic site/service parity: total={}, passed={}, failed={}",
        matrix.total, matrix.passed, matrix.failed
    );
    if !matrix.pass {
        bail!("classic site/service parity matrix failed");
    }
    Ok(())
}
