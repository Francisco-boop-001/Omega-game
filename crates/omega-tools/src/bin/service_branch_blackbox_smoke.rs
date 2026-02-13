use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{
    Alignment, Command, DeterministicRng, GameState, LegacyQuestState, SITE_AUX_SERVICE_ARMORER,
    SITE_AUX_SERVICE_COMMANDANT, SITE_AUX_SERVICE_MONASTERY, SITE_AUX_SERVICE_ORDER,
    SITE_AUX_SERVICE_PALACE, SITE_AUX_SERVICE_THIEVES, SessionStatus, step,
};
use omega_tools::replay::{ReplayCommand, ReplayFixture, load_fixture, run_fixture_trace};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
struct FinalStateVector {
    gold: i32,
    bank_gold: i32,
    food: i32,
    guild_rank: u8,
    priest_rank: u8,
    alignment: String,
    law_chaos_score: i32,
    deity_favor: i32,
    legal_heat: i32,
    quest_state: String,
    quest_steps_completed: u8,
    main_quest_stage: String,
    arena_rank: i8,
    arena_match_active: bool,
    inventory_count: usize,
    known_spells_count: usize,
    world_mode: String,
    map_semantic: String,
}

#[derive(Debug, Clone, Serialize)]
struct BlackboxScenarioResult {
    id: String,
    pass: bool,
    input_trace: Vec<String>,
    final_state_vector: FinalStateVector,
    failed_assertions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct BlackboxSmokeReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    scenarios: Vec<BlackboxScenarioResult>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn state_vector(state: &GameState) -> FinalStateVector {
    FinalStateVector {
        gold: state.gold,
        bank_gold: state.bank_gold,
        food: state.food,
        guild_rank: state.progression.guild_rank,
        priest_rank: state.progression.priest_rank,
        alignment: format!("{:?}", state.progression.alignment),
        law_chaos_score: state.progression.law_chaos_score,
        deity_favor: state.progression.deity_favor,
        legal_heat: state.legal_heat,
        quest_state: format!("{:?}", state.progression.quest_state),
        quest_steps_completed: state.progression.quest_steps_completed,
        main_quest_stage: format!("{:?}", state.progression.main_quest.stage),
        arena_rank: state.progression.arena_rank,
        arena_match_active: state.progression.arena_match_active,
        inventory_count: state.player.inventory.len(),
        known_spells_count: state.spellbook.spells.iter().filter(|spell| spell.known).count(),
        world_mode: format!("{:?}", state.world_mode),
        map_semantic: format!("{:?}", state.map_binding.semantic),
    }
}

fn replay_command_text(command: &ReplayCommand) -> String {
    match command {
        ReplayCommand::Wait => "wait".to_string(),
        ReplayCommand::Move { direction } => format!("move:{direction:?}"),
        ReplayCommand::Attack { direction } => format!("attack:{direction:?}"),
        ReplayCommand::Pickup => "pickup".to_string(),
        ReplayCommand::Drop { slot } => format!("drop:{slot}"),
        ReplayCommand::Legacy { token } => format!("legacy:{token}"),
    }
}

fn has_placeholder_noise(state: &GameState) -> bool {
    let needles =
        ["audience held", "dialogue resolved with", "quest hooks processed", "generic audience"];
    state.log.iter().any(|line| {
        let lower = line.to_ascii_lowercase();
        needles.iter().any(|needle| lower.contains(needle))
    })
}

fn set_current_site_aux(state: &mut GameState, aux: i32) {
    if let Some(site) = state.tile_site_at_mut(state.player.position) {
        site.aux = aux;
    }
    let width = state.bounds.width as usize;
    let x = state.player.position.x.max(0) as usize;
    let y = state.player.position.y.max(0) as usize;
    let idx = y.saturating_mul(width).saturating_add(x);
    if let Some(site) = state.site_grid.get_mut(idx) {
        site.aux = aux;
    }
    if let Some(site) = state.city_site_grid.get_mut(idx) {
        site.aux = aux;
    }
}

fn run_fixture_scenario<F>(id: &str, fixture_path: &str, check: F) -> Result<BlackboxScenarioResult>
where
    F: Fn(&ReplayFixture, &GameState, &FinalStateVector, &mut Vec<String>),
{
    let fixture = load_fixture(Path::new(fixture_path))
        .with_context(|| format!("load fixture {fixture_path}"))?;
    let trace = run_fixture_trace(&fixture);
    let state = trace.final_state;
    let vector = state_vector(&state);
    let input_trace = fixture.commands.iter().map(replay_command_text).collect::<Vec<_>>();
    let mut failed_assertions = Vec::new();

    if state.status != SessionStatus::InProgress {
        failed_assertions.push(format!("session status is terminal: {:?}", state.status));
    }
    if has_placeholder_noise(&state) {
        failed_assertions.push("placeholder dialogue noise detected in log".to_string());
    }
    check(&fixture, &state, &vector, &mut failed_assertions);

    let pass = failed_assertions.is_empty();
    Ok(BlackboxScenarioResult {
        id: id.to_string(),
        pass,
        input_trace,
        final_state_vector: vector,
        failed_assertions,
    })
}

fn run_manual_scenario<FSetup, FCheck>(
    id: &str,
    aux: i32,
    seed: u64,
    tokens: &[&str],
    setup: FSetup,
    check: FCheck,
) -> Result<BlackboxScenarioResult>
where
    FSetup: FnOnce(&mut GameState),
    FCheck: Fn(&GameState, &FinalStateVector, &FinalStateVector, &mut Vec<String>),
{
    let (mut state, _) = bootstrap_game_state_from_default_content().context("bootstrap state")?;
    state.options.interactive_sites = true;
    setup(&mut state);
    set_current_site_aux(&mut state, aux);
    let before = state_vector(&state);

    let mut rng = DeterministicRng::seeded(seed);
    let mut input_trace = Vec::new();
    for token in tokens {
        input_trace.push(format!("legacy:{token}"));
        let _ = step(&mut state, Command::Legacy { token: (*token).to_string() }, &mut rng);
    }
    let after = state_vector(&state);
    let mut failed_assertions = Vec::new();
    if state.status != SessionStatus::InProgress {
        failed_assertions.push(format!("session status is terminal: {:?}", state.status));
    }
    if has_placeholder_noise(&state) {
        failed_assertions.push("placeholder dialogue noise detected in log".to_string());
    }
    check(&state, &before, &after, &mut failed_assertions);

    let pass = failed_assertions.is_empty();
    Ok(BlackboxScenarioResult {
        id: id.to_string(),
        pass,
        input_trace,
        final_state_vector: after,
        failed_assertions,
    })
}

fn markdown(report: &BlackboxSmokeReport) -> String {
    let mut out = Vec::new();
    out.push("# Service Branch Blackbox Smoke".to_string());
    out.push(String::new());
    out.push(format!("- generated_at_utc: `{}`", now_utc_unix()));
    out.push(format!("- total: `{}`", report.total));
    out.push(format!("- passed: `{}`", report.passed));
    out.push(format!("- failed: `{}`", report.failed));
    out.push(format!("- status: `{}`", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Scenario | Status | Input Count | Failed Assertions |".to_string());
    out.push("|---|---|---:|---:|".to_string());
    for scenario in &report.scenarios {
        out.push(format!(
            "| {} | {} | {} | {} |",
            scenario.id,
            if scenario.pass { "PASS" } else { "FAIL" },
            scenario.input_trace.len(),
            scenario.failed_assertions.len()
        ));
    }
    out.push(String::new());
    for scenario in &report.scenarios {
        if scenario.pass {
            continue;
        }
        out.push(format!("## Failure: {}", scenario.id));
        out.push(String::new());
        out.push("- Input trace:".to_string());
        for token in &scenario.input_trace {
            out.push(format!("  - `{}`", token));
        }
        out.push("- Assertions:".to_string());
        for failure in &scenario.failed_assertions {
            out.push(format!("  - {}", failure));
        }
        out.push(String::new());
    }
    out.join("\n")
}

fn main() -> Result<()> {
    let mut scenarios = Vec::new();

    scenarios.push(run_fixture_scenario(
        "fixture_merc_success",
        "crates/omega-tools/fixtures/replay/r3_site_merc_guild.json",
        |_fixture, _state, vector, failures| {
            if vector.guild_rank < 1 {
                failures.push(format!("expected guild rank >=1, got {}", vector.guild_rank));
            }
            if vector.quest_state != "Active" {
                failures.push(format!("expected quest_state Active, got {}", vector.quest_state));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_bank_flow",
        "crates/omega-tools/fixtures/replay/r3_site_bank.json",
        |_fixture, _state, vector, failures| {
            if vector.bank_gold <= 0 {
                failures.push(format!("expected bank_gold >0, got {}", vector.bank_gold));
            }
            if vector.gold >= 250 {
                failures
                    .push(format!("expected gold to decrease from baseline, got {}", vector.gold));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_castle_audience",
        "crates/omega-tools/fixtures/replay/r3_site_castle_talk.json",
        |_fixture, _state, vector, failures| {
            if vector.quest_state != "Active" {
                failures.push(format!("expected quest_state Active, got {}", vector.quest_state));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_charity_service",
        "crates/omega-tools/fixtures/replay/r3_site_charity.json",
        |_fixture, _state, vector, failures| {
            if vector.food < 4 {
                failures.push(format!("expected food >=4 after charity aid, got {}", vector.food));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_college_service",
        "crates/omega-tools/fixtures/replay/r3_site_college.json",
        |_fixture, _state, vector, failures| {
            if vector.gold >= 250 {
                failures.push(format!("expected tuition/payment gold drop, got {}", vector.gold));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_order_talk",
        "crates/omega-tools/fixtures/replay/r3_site_order_talk.json",
        |_fixture, _state, vector, failures| {
            if vector.alignment != "Lawful" {
                failures.push(format!(
                    "expected lawful alignment after order audience, got {}",
                    vector.alignment
                ));
            }
            if vector.quest_state != "Active" {
                failures.push(format!("expected quest_state Active, got {}", vector.quest_state));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_shop_service",
        "crates/omega-tools/fixtures/replay/r3_site_shop.json",
        |_fixture, _state, vector, failures| {
            if vector.gold >= 250 {
                failures.push(format!("expected purchase gold drop, got {}", vector.gold));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_sorcerors_service",
        "crates/omega-tools/fixtures/replay/r3_site_sorcerors.json",
        |_fixture, _state, vector, failures| {
            if vector.gold >= 250 {
                failures.push(format!("expected initiation/research payment, got {}", vector.gold));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_temple_service",
        "crates/omega-tools/fixtures/replay/r3_site_temple.json",
        |_fixture, _state, vector, failures| {
            if vector.priest_rank < 1 {
                failures.push(format!("expected priest rank >=1, got {}", vector.priest_rank));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_arena_service",
        "crates/omega-tools/fixtures/replay/r3_site_arena.json",
        |_fixture, _state, vector, failures| {
            if vector.arena_rank < 1 && !vector.arena_match_active {
                failures.push(format!(
                    "expected arena to either rank up or start an active match, got rank={} active={}",
                    vector.arena_rank, vector.arena_match_active
                ));
            }
        },
    )?);

    scenarios.push(run_fixture_scenario(
        "fixture_city_service_loop",
        "crates/omega-tools/fixtures/replay/p5_city_service_social.json",
        |_fixture, _state, vector, failures| {
            if vector.guild_rank < 1 {
                failures.push(format!("expected guild progression, got {}", vector.guild_rank));
            }
            if vector.quest_state != "Active" {
                failures.push(format!("expected quest_state Active, got {}", vector.quest_state));
            }
        },
    )?);

    scenarios.push(run_manual_scenario(
        "manual_thieves_success",
        SITE_AUX_SERVICE_THIEVES,
        0xBB01,
        &[">", "1", ">", "2"],
        |state| {
            state.gold = 600;
            state.progression.alignment = Alignment::Neutral;
            state.progression.law_chaos_score = 0;
            state.legal_heat = 0;
        },
        |_state, before, after, failures| {
            if after.law_chaos_score >= before.law_chaos_score
                && after.legal_heat <= before.legal_heat
            {
                failures.push(format!(
                    "expected thieves branch to shift state (law/chaos or legal heat), before_lc={} after_lc={} before_heat={} after_heat={}",
                    before.law_chaos_score, after.law_chaos_score, before.legal_heat, after.legal_heat
                ));
            }
        },
    )?);

    scenarios.push(run_manual_scenario(
        "manual_thieves_denial_lawful",
        SITE_AUX_SERVICE_THIEVES,
        0xBB02,
        &[">", "1"],
        |state| {
            state.gold = 600;
            state.progression.alignment = Alignment::Lawful;
            state.progression.law_chaos_score = 8;
        },
        |state, _before, _after, failures| {
            if state.progression.quests.thieves.rank > 0 {
                failures.push(format!(
                    "expected lawful denial to keep thieves rank 0, got {}",
                    state.progression.quests.thieves.rank
                ));
            }
        },
    )?);

    scenarios.push(run_manual_scenario(
        "manual_palace_progression",
        SITE_AUX_SERVICE_PALACE,
        0xBB03,
        &[">", "2"],
        |state| {
            state.gold = 800;
            state.progression.main_quest.palace_access = true;
            state.progression.quest_state = LegacyQuestState::ArtifactRecovered;
            state.progression.main_quest.stage = LegacyQuestState::ArtifactRecovered;
            state.progression.guild_rank = 2;
            state.progression.priest_rank = 1;
            state.progression.quests.merc.rank = 2;
            state.progression.quests.temple.rank = 1;
        },
        |state, _before, _after, failures| {
            if state.progression.main_quest.stage == LegacyQuestState::ArtifactRecovered {
                failures.push("expected palace petition to advance main quest stage".to_string());
            }
        },
    )?);

    scenarios.push(run_manual_scenario(
        "manual_monastery_donation",
        SITE_AUX_SERVICE_MONASTERY,
        0xBB04,
        &[">", "1"],
        |state| {
            state.gold = 300;
            state.progression.deity_favor = 0;
        },
        |state, before, after, failures| {
            if after.gold > before.gold {
                failures.push(format!(
                    "monastery interaction increased gold unexpectedly: before={} after={}",
                    before.gold, after.gold
                ));
            }
            if state.progression.deity_favor < before.deity_favor {
                failures.push(format!(
                    "monastery interaction reduced favor unexpectedly: before={} after={}",
                    before.deity_favor, state.progression.deity_favor
                ));
            }
        },
    )?);

    scenarios.push(run_manual_scenario(
        "manual_commandant_bucket",
        SITE_AUX_SERVICE_COMMANDANT,
        0xBB05,
        &[">", "1"],
        |state| {
            state.gold = 250;
            state.food = 3;
        },
        |_state, before, after, failures| {
            if after.food <= before.food {
                failures.push(format!(
                    "expected commandant bucket to increase food: before={} after={}",
                    before.food, after.food
                ));
            }
            if after.gold >= before.gold {
                failures.push(format!(
                    "expected commandant bucket to cost gold: before={} after={}",
                    before.gold, after.gold
                ));
            }
        },
    )?);

    scenarios.push(run_manual_scenario(
        "manual_armorer_purchase",
        SITE_AUX_SERVICE_ARMORER,
        0xBB06,
        &[">", "1"],
        |state| {
            state.gold = 260;
        },
        |_state, before, after, failures| {
            if after.inventory_count <= before.inventory_count {
                failures.push(format!(
                    "expected armorer purchase to add inventory item: before={} after={}",
                    before.inventory_count, after.inventory_count
                ));
            }
            if after.gold >= before.gold {
                failures.push(format!(
                    "expected armorer purchase to reduce gold: before={} after={}",
                    before.gold, after.gold
                ));
            }
        },
    )?);

    scenarios.push(run_manual_scenario(
        "manual_order_invalid_then_cancel",
        SITE_AUX_SERVICE_ORDER,
        0xBB07,
        &[">", "9", "x"],
        |state| {
            state.progression.alignment = Alignment::Neutral;
            state.progression.law_chaos_score = 0;
        },
        |state, before, _after, failures| {
            if state.pending_site_interaction.is_some() {
                failures.push("expected cancel path to close pending site interaction".to_string());
            }
            if state.progression.alignment != Alignment::Neutral {
                failures.push("unexpected alignment shift from invalid/cancel flow".to_string());
            }
            if state.progression.law_chaos_score != before.law_chaos_score {
                failures.push("unexpected law/chaos shift from invalid/cancel flow".to_string());
            }
        },
    )?);

    let total = scenarios.len();
    let passed = scenarios.iter().filter(|scenario| scenario.pass).count();
    let failed = total.saturating_sub(passed);
    let report = BlackboxSmokeReport { total, passed, failed, pass: failed == 0, scenarios };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }
    fs::write(
        "target/service-branch-blackbox-smoke.json",
        serde_json::to_string_pretty(&report).context("serialize blackbox smoke report")?,
    )
    .context("write target/service-branch-blackbox-smoke.json")?;
    fs::write("target/service-branch-blackbox-smoke.md", markdown(&report))
        .context("write target/service-branch-blackbox-smoke.md")?;

    println!(
        "service branch blackbox smoke: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("service branch blackbox smoke failed");
    }
    Ok(())
}
