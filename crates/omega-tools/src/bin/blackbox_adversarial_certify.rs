use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{
    Command, DeterministicRng, GameState, SITE_AUX_SERVICE_ARENA, SITE_AUX_SERVICE_ARMORER,
    SITE_AUX_SERVICE_BANK, SITE_AUX_SERVICE_BROTHEL, SITE_AUX_SERVICE_CASINO,
    SITE_AUX_SERVICE_CASTLE, SITE_AUX_SERVICE_CHARITY, SITE_AUX_SERVICE_CLUB,
    SITE_AUX_SERVICE_COLLEGE, SITE_AUX_SERVICE_COMMANDANT, SITE_AUX_SERVICE_CONDO,
    SITE_AUX_SERVICE_CRAPS, SITE_AUX_SERVICE_DINER, SITE_AUX_SERVICE_GYM, SITE_AUX_SERVICE_HEALER,
    SITE_AUX_SERVICE_MERC_GUILD, SITE_AUX_SERVICE_MONASTERY, SITE_AUX_SERVICE_ORDER,
    SITE_AUX_SERVICE_PALACE, SITE_AUX_SERVICE_PAWN_SHOP, SITE_AUX_SERVICE_SHOP,
    SITE_AUX_SERVICE_SORCERORS, SITE_AUX_SERVICE_TAVERN, SITE_AUX_SERVICE_TEMPLE,
    SITE_AUX_SERVICE_THIEVES, SessionStatus, step,
};
use omega_tools::audit_contract::{StateVector, ensure_cert_dirs, smoke_dir, write_json};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ScenarioResult {
    id: String,
    pass: bool,
    input_trace: Vec<String>,
    failures: Vec<String>,
    final_state: StateVector,
}

#[derive(Debug, Serialize)]
struct BlackboxAdversarialReport {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    scenarios: Vec<ScenarioResult>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
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

fn state_vector(state: &GameState) -> StateVector {
    StateVector {
        gold: Some(state.gold),
        bank_gold: Some(state.bank_gold),
        guild_rank: Some(state.progression.guild_rank),
        priest_rank: Some(state.progression.priest_rank),
        alignment: Some(format!("{:?}", state.progression.alignment)),
        law_chaos_score: Some(state.progression.law_chaos_score),
        deity_favor: Some(state.progression.deity_favor),
        legal_heat: Some(state.legal_heat),
        quest_state: Some(format!("{:?}", state.progression.quest_state)),
        quest_steps_completed: Some(state.progression.quest_steps_completed),
        main_quest_stage: Some(format!("{:?}", state.progression.main_quest.stage)),
        arena_rank: Some(state.progression.arena_rank),
        arena_match_active: Some(state.progression.arena_match_active),
        inventory_count: Some(state.player.inventory.len()),
        known_spells_count: Some(state.spellbook.spells.iter().filter(|spell| spell.known).count()),
        world_mode: Some(format!("{:?}", state.world_mode)),
        map_semantic: Some(format!("{:?}", state.map_binding.semantic)),
    }
}

fn has_placeholder_noise(state: &GameState) -> bool {
    let needles = ["audience held", "dialogue resolved with", "quest hooks processed"];
    state.log.iter().any(|line| {
        let lower = line.to_ascii_lowercase();
        needles.iter().any(|needle| lower.contains(needle))
    })
}

fn run_script(aux: i32, seed: u64, tokens: &[&str]) -> Result<GameState> {
    let (mut state, _) = bootstrap_game_state_from_default_content().context("bootstrap game")?;
    state.options.interactive_sites = true;
    state.gold = state.gold.max(600);
    set_current_site_aux(&mut state, aux);
    let mut rng = DeterministicRng::seeded(seed);
    for token in tokens {
        let _ = step(&mut state, Command::Legacy { token: (*token).to_string() }, &mut rng);
    }
    Ok(state)
}

fn compare_for_determinism(a: &GameState, b: &GameState, failures: &mut Vec<String>) {
    if a.gold != b.gold {
        failures.push(format!("non-deterministic gold {} vs {}", a.gold, b.gold));
    }
    if a.bank_gold != b.bank_gold {
        failures.push(format!("non-deterministic bank_gold {} vs {}", a.bank_gold, b.bank_gold));
    }
    if a.progression.guild_rank != b.progression.guild_rank {
        failures.push(format!(
            "non-deterministic guild_rank {} vs {}",
            a.progression.guild_rank, b.progression.guild_rank
        ));
    }
    if a.progression.quest_state != b.progression.quest_state {
        failures.push(format!(
            "non-deterministic quest_state {:?} vs {:?}",
            a.progression.quest_state, b.progression.quest_state
        ));
    }
}

fn run_scenario(id: &str, aux: i32, seed: u64, tokens: &[&str]) -> Result<ScenarioResult> {
    let left = run_script(aux, seed, tokens)?;
    let right = run_script(aux, seed, tokens)?;
    let mut failures = Vec::new();
    if left.status != SessionStatus::InProgress {
        failures.push(format!("terminal status reached: {:?}", left.status));
    }
    if has_placeholder_noise(&left) {
        failures.push("placeholder log noise detected".to_string());
    }
    if left.pending_site_interaction.is_some() && !tokens.contains(&"x") {
        failures.push("site interaction left open without explicit close".to_string());
    }
    compare_for_determinism(&left, &right, &mut failures);
    Ok(ScenarioResult {
        id: id.to_string(),
        pass: failures.is_empty(),
        input_trace: tokens.iter().map(|token| format!("legacy:{token}")).collect(),
        failures,
        final_state: state_vector(&left),
    })
}

fn main() -> Result<()> {
    ensure_cert_dirs()?;

    let services = [
        ("shop", SITE_AUX_SERVICE_SHOP),
        ("armorer", SITE_AUX_SERVICE_ARMORER),
        ("club", SITE_AUX_SERVICE_CLUB),
        ("gym", SITE_AUX_SERVICE_GYM),
        ("healer", SITE_AUX_SERVICE_HEALER),
        ("casino", SITE_AUX_SERVICE_CASINO),
        ("commandant", SITE_AUX_SERVICE_COMMANDANT),
        ("diner", SITE_AUX_SERVICE_DINER),
        ("craps", SITE_AUX_SERVICE_CRAPS),
        ("tavern", SITE_AUX_SERVICE_TAVERN),
        ("pawn_shop", SITE_AUX_SERVICE_PAWN_SHOP),
        ("brothel", SITE_AUX_SERVICE_BROTHEL),
        ("condo", SITE_AUX_SERVICE_CONDO),
        ("bank", SITE_AUX_SERVICE_BANK),
        ("merc", SITE_AUX_SERVICE_MERC_GUILD),
        ("thieves", SITE_AUX_SERVICE_THIEVES),
        ("temple", SITE_AUX_SERVICE_TEMPLE),
        ("college", SITE_AUX_SERVICE_COLLEGE),
        ("sorcerors", SITE_AUX_SERVICE_SORCERORS),
        ("castle", SITE_AUX_SERVICE_CASTLE),
        ("palace", SITE_AUX_SERVICE_PALACE),
        ("order", SITE_AUX_SERVICE_ORDER),
        ("charity", SITE_AUX_SERVICE_CHARITY),
        ("monastery", SITE_AUX_SERVICE_MONASTERY),
        ("arena", SITE_AUX_SERVICE_ARENA),
    ];

    let mut scenarios = Vec::new();
    for (idx, (id, aux)) in services.iter().enumerate() {
        let seed = 0xC0DE_0000u64 + idx as u64;
        let tokens = [">", "9", "1", "2", "x"];
        scenarios.push(run_scenario(&format!("adversarial_{id}"), *aux, seed, &tokens)?);
    }

    let total = scenarios.len();
    let passed = scenarios.iter().filter(|scenario| scenario.pass).count();
    let failed = total.saturating_sub(passed);
    let report = BlackboxAdversarialReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        scenarios,
    };

    let out_path = smoke_dir().join("blackbox-adversarial.json");
    write_json(&out_path, &report)?;
    println!(
        "blackbox adversarial certify: total={} passed={} failed={} pass={}",
        report.total, report.passed, report.failed, report.pass
    );
    if !report.pass {
        bail!("blackbox adversarial certification failed");
    }
    Ok(())
}
