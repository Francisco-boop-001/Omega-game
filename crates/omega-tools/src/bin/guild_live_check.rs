use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
struct SiteBranchDiffReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct GenericPassReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct QuestSmokeStep {
    label: String,
    passed: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct QuestSmokeReport {
    pass: bool,
    #[serde(default)]
    steps: Vec<QuestSmokeStep>,
}

#[derive(Debug, Clone, Serialize)]
struct GuildLiveCheck {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<GuildLiveCheckRow>,
}

#[derive(Debug, Clone, Serialize)]
struct GuildLiveCheckRow {
    id: String,
    pass: bool,
    details: String,
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T> {
    let raw = fs::read_to_string(path).with_context(|| format!("read {path}"))?;
    serde_json::from_str::<T>(&raw).with_context(|| format!("decode {path}"))
}

fn markdown(report: &GuildLiveCheck) -> String {
    let mut out = Vec::new();
    out.push("# Guild Live Check".to_string());
    out.push(String::new());
    out.push(format!("- total: `{}`", report.total));
    out.push(format!("- passed: `{}`", report.passed));
    out.push(format!("- failed: `{}`", report.failed));
    out.push(format!("- status: `{}`", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Check | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for check in &report.checks {
        out.push(format!(
            "| {} | {} | {} |",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let site_branch_diff: SiteBranchDiffReport = read_json("target/site-branch-diff.json")?;
    let site_matrix: GenericPassReport =
        read_json("target/classic-site-service-parity-matrix.json")?;
    let oracle: GenericPassReport = read_json("target/service-branch-oracle.json")?;
    let talk_clarity: GenericPassReport = read_json("target/guild-service-talk-clarity.json")?;
    let quest_smoke: QuestSmokeReport = read_json("target/quest-parity-smoke.json")?;
    let blackbox: GenericPassReport = read_json("target/service-branch-blackbox-smoke.json")?;

    let has_step =
        |label: &str| quest_smoke.steps.iter().any(|step| step.label == label && step.passed);
    let quest_briefings_ok = quest_smoke.pass
        && has_step("merc_contract")
        && has_step("order_talk_no_generic_placeholders")
        && has_step("castle_talk_no_generic_placeholders");

    let checks = vec![
        GuildLiveCheckRow {
            id: "site_branch_diff".to_string(),
            pass: site_branch_diff.pass,
            details: format!(
                "total={} passed={} failed={}",
                site_branch_diff.total, site_branch_diff.passed, site_branch_diff.failed
            ),
        },
        GuildLiveCheckRow {
            id: "classic_site_service_parity".to_string(),
            pass: site_matrix.pass,
            details: format!(
                "total={} passed={} failed={}",
                site_matrix.total, site_matrix.passed, site_matrix.failed
            ),
        },
        GuildLiveCheckRow {
            id: "service_branch_oracle".to_string(),
            pass: oracle.pass,
            details: format!(
                "total={} passed={} failed={}",
                oracle.total, oracle.passed, oracle.failed
            ),
        },
        GuildLiveCheckRow {
            id: "guild_service_talk_clarity".to_string(),
            pass: talk_clarity.pass,
            details: format!(
                "total={} passed={} failed={}",
                talk_clarity.total, talk_clarity.passed, talk_clarity.failed
            ),
        },
        GuildLiveCheckRow {
            id: "quest_briefing_clarity".to_string(),
            pass: quest_briefings_ok,
            details: format!(
                "quest_pass={} merc_contract={} order_talk={} castle_talk={}",
                quest_smoke.pass,
                has_step("merc_contract"),
                has_step("order_talk_no_generic_placeholders"),
                has_step("castle_talk_no_generic_placeholders")
            ),
        },
        GuildLiveCheckRow {
            id: "service_branch_blackbox_smoke".to_string(),
            pass: blackbox.pass,
            details: format!(
                "total={} passed={} failed={}",
                blackbox.total, blackbox.passed, blackbox.failed
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = GuildLiveCheck { total, passed, failed, pass: failed == 0, checks };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }
    fs::write(
        "target/guild-live-check.json",
        serde_json::to_string_pretty(&report).context("serialize guild live check")?,
    )
    .context("write target/guild-live-check.json")?;
    fs::write("target/guild-live-check.md", markdown(&report))
        .context("write target/guild-live-check.md")?;

    println!(
        "guild live check: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("guild live check failed");
    }
    Ok(())
}
