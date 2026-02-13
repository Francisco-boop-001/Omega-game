use anyhow::{Context, Result, bail};
use omega_tools::replay::{load_fixture, run_fixture, run_fixture_trace};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct BranchCheck {
    id: String,
    fixture: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct BranchMatrix {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<BranchCheck>,
}

fn markdown(matrix: &BranchMatrix) -> String {
    let mut out = Vec::new();
    out.push("# Service Branch Oracle".to_string());
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
            check.id,
            check.fixture,
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn has_service_placeholder_noise(state: &omega_core::GameState) -> bool {
    const NEEDLES: [&str; 3] = ["audience held", "dialogue resolved with", "quest hooks processed"];
    state
        .log
        .iter()
        .any(|line| NEEDLES.iter().any(|needle| line.to_ascii_lowercase().contains(needle)))
}

fn run_fixture_check(id: &str, fixture: &str) -> Result<BranchCheck> {
    let path = Path::new(fixture);
    let scenario =
        load_fixture(path).with_context(|| format!("load fixture {}", path.display()))?;
    let result = run_fixture(&scenario);
    let trace = run_fixture_trace(&scenario);
    let state = trace.final_state;
    let no_open_prompt = state.pending_site_interaction.is_none();
    let no_placeholder_noise = !has_service_placeholder_noise(&state);
    let passed = result.passed && no_open_prompt && no_placeholder_noise;
    let details = format!(
        "fixture_pass={} no_open_prompt={} no_placeholder_noise={} final_turn={} gold={} bank={} guild_rank={} priest_rank={} alignment={:?} quest={:?} food={}",
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
        state.food
    );
    Ok(BranchCheck { id: id.to_string(), fixture: fixture.to_string(), passed, details })
}

fn main() -> Result<()> {
    let fixtures = [
        ("shop_service", "crates/omega-tools/fixtures/replay/r3_site_shop.json"),
        ("bank_service", "crates/omega-tools/fixtures/replay/r3_site_bank.json"),
        ("merc_service", "crates/omega-tools/fixtures/replay/r3_site_merc_guild.json"),
        ("temple_service", "crates/omega-tools/fixtures/replay/r3_site_temple.json"),
        ("college_service", "crates/omega-tools/fixtures/replay/r3_site_college.json"),
        ("sorcerors_service", "crates/omega-tools/fixtures/replay/r3_site_sorcerors.json"),
        ("charity_service", "crates/omega-tools/fixtures/replay/r3_site_charity.json"),
        ("arena_service", "crates/omega-tools/fixtures/replay/r3_site_arena.json"),
        ("order_talk", "crates/omega-tools/fixtures/replay/r3_site_order_talk.json"),
        ("castle_talk", "crates/omega-tools/fixtures/replay/r3_site_castle_talk.json"),
        ("city_economy_loop", "crates/omega-tools/fixtures/replay/p5_city_service_social.json"),
    ];

    let mut checks = Vec::new();
    for (id, fixture) in fixtures {
        checks.push(run_fixture_check(id, fixture)?);
    }

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let matrix = BranchMatrix { total, passed, failed, pass: failed == 0, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("service-branch-oracle.json");
    let md_path = target.join("service-branch-oracle.md");
    let parity_json_path = target.join("site-branch-parity-matrix.json");
    let parity_md_path = target.join("site-branch-parity-matrix.md");
    let json_payload =
        serde_json::to_string_pretty(&matrix).context("serialize service branch oracle")?;
    let md_payload = markdown(&matrix);
    fs::write(&json_path, &json_payload)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, &md_payload).with_context(|| format!("write {}", md_path.display()))?;
    fs::write(&parity_json_path, &json_payload)
        .with_context(|| format!("write {}", parity_json_path.display()))?;
    fs::write(&parity_md_path, &md_payload)
        .with_context(|| format!("write {}", parity_md_path.display()))?;

    println!(
        "service branch oracle: total={}, passed={}, failed={}",
        matrix.total, matrix.passed, matrix.failed
    );
    if !matrix.pass {
        bail!("service branch oracle failed");
    }
    Ok(())
}
