use anyhow::{Context, Result, bail};
use omega_tools::audit_contract::{StateVector, diff_dir, ensure_cert_dirs, write_json};
use omega_tools::replay::{ReplayCommand, load_fixture, run_fixture_trace};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct RustScenarioSnapshot {
    id: String,
    family: String,
    active: bool,
    input_trace: Vec<String>,
    state: StateVector,
}

#[derive(Debug, Serialize)]
struct RustHeadlessReplayReport {
    generated_at_utc: String,
    total: usize,
    pass: bool,
    scenarios: Vec<RustScenarioSnapshot>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn command_text(command: &ReplayCommand) -> String {
    match command {
        ReplayCommand::Wait => "wait".to_string(),
        ReplayCommand::Move { direction } => format!("move:{direction:?}"),
        ReplayCommand::Attack { direction } => format!("attack:{direction:?}"),
        ReplayCommand::Pickup => "pickup".to_string(),
        ReplayCommand::Drop { slot } => format!("drop:{slot}"),
        ReplayCommand::Legacy { token } => format!("legacy:{token}"),
    }
}

fn final_state_vector(state: &omega_core::GameState) -> StateVector {
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

fn main() -> Result<()> {
    ensure_cert_dirs()?;
    let fixture_dir = Path::new("crates/omega-tools/fixtures/replay");
    if !fixture_dir.exists() {
        bail!("missing fixture dir {}", fixture_dir.display());
    }

    let mut scenarios = Vec::new();
    for entry in
        fs::read_dir(fixture_dir).with_context(|| format!("read {}", fixture_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let fixture = load_fixture(&path).with_context(|| format!("load {}", path.display()))?;
        let trace = run_fixture_trace(&fixture);
        scenarios.push(RustScenarioSnapshot {
            id: fixture.name.clone(),
            family: fixture.family.clone(),
            active: fixture.active,
            input_trace: fixture.commands.iter().map(command_text).collect(),
            state: final_state_vector(&trace.final_state),
        });
    }
    scenarios.sort_by(|a, b| a.id.cmp(&b.id));
    if scenarios.is_empty() {
        bail!("no replay fixtures found");
    }

    let report = RustHeadlessReplayReport {
        generated_at_utc: now_utc_unix(),
        total: scenarios.len(),
        pass: true,
        scenarios,
    };
    let out_path = diff_dir().join("rust-headless-replay.json");
    write_json(&out_path, &report)?;
    println!("rust headless replay: total={} out={}", report.total, out_path.display());
    Ok(())
}
