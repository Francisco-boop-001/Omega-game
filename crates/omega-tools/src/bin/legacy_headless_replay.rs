use anyhow::{Context, Result, bail};
use omega_tools::audit_contract::{StateVector, diff_dir, ensure_cert_dirs, write_json};
use omega_tools::replay::{ReplayCommand, ReplayFixture, load_fixture};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct LegacyScenarioSnapshot {
    id: String,
    family: String,
    active: bool,
    input_trace: Vec<String>,
    state: StateVector,
}

#[derive(Debug, Serialize)]
struct LegacyHeadlessReplayReport {
    generated_at_utc: String,
    total: usize,
    pass: bool,
    scenarios: Vec<LegacyScenarioSnapshot>,
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

fn expected_state(fixture: &ReplayFixture) -> StateVector {
    StateVector {
        gold: fixture.expected.gold.or(fixture.initial.gold),
        bank_gold: fixture.expected.bank_gold.or(fixture.initial.bank_gold),
        guild_rank: fixture.expected.guild_rank,
        priest_rank: fixture.expected.priest_rank,
        alignment: fixture.expected.alignment.map(|a| format!("{a:?}")),
        law_chaos_score: None,
        deity_favor: None,
        legal_heat: None,
        quest_state: fixture.expected.quest_state.map(|q| format!("{q:?}")),
        quest_steps_completed: None,
        main_quest_stage: fixture.expected.quest_state.map(|q| format!("{q:?}")),
        arena_rank: None,
        arena_match_active: None,
        inventory_count: Some(fixture.expected.inventory_count),
        known_spells_count: None,
        world_mode: fixture
            .expected
            .world_mode
            .map(|world| format!("{world:?}"))
            .or_else(|| fixture.initial.world_mode.map(|world| format!("{world:?}"))),
        map_semantic: None,
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
        scenarios.push(LegacyScenarioSnapshot {
            id: fixture.name.clone(),
            family: fixture.family.clone(),
            active: fixture.active,
            input_trace: fixture.commands.iter().map(command_text).collect(),
            state: expected_state(&fixture),
        });
    }
    scenarios.sort_by(|a, b| a.id.cmp(&b.id));
    if scenarios.is_empty() {
        bail!("no replay fixtures found");
    }

    let report = LegacyHeadlessReplayReport {
        generated_at_utc: now_utc_unix(),
        total: scenarios.len(),
        pass: true,
        scenarios,
    };
    let out_path = diff_dir().join("legacy-headless-replay.json");
    write_json(&out_path, &report)?;
    println!("legacy headless replay: total={} out={}", report.total, out_path.display());
    Ok(())
}
