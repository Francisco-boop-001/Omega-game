use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Serialize)]
struct LiveCheckResult {
    id: String,
    command: String,
    pass: bool,
    duration_ms: u128,
    exit_code: i32,
    stdout_tail: String,
    stderr_tail: String,
}

#[derive(Debug, Serialize)]
struct LiveChecksReport {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<LiveCheckResult>,
}

struct CheckSpec {
    id: &'static str,
    args: &'static [&'static str],
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn format_command(args: &[&str]) -> String {
    let mut parts = vec!["cargo".to_string()];
    parts.extend(args.iter().map(|arg| (*arg).to_string()));
    parts.join(" ")
}

fn trim_tail(text: &str, max_lines: usize) -> String {
    let lines = text.lines().collect::<Vec<_>>();
    if lines.len() <= max_lines {
        return text.trim().to_string();
    }
    lines[lines.len() - max_lines..].join("\n")
}

fn run_check(spec: &CheckSpec) -> Result<LiveCheckResult> {
    let started = Instant::now();
    let output = Command::new("cargo")
        .args(spec.args)
        .output()
        .with_context(|| format!("spawn {}", format_command(spec.args)))?;
    let duration_ms = started.elapsed().as_millis();
    let status_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(LiveCheckResult {
        id: spec.id.to_string(),
        command: format_command(spec.args),
        pass: output.status.success(),
        duration_ms,
        exit_code: status_code,
        stdout_tail: trim_tail(&stdout, 10),
        stderr_tail: trim_tail(&stderr, 10),
    })
}

fn write_markdown(path: &str, report: &LiveChecksReport) -> Result<()> {
    let mut md = String::new();
    md.push_str("# Live Checks All\n\n");
    md.push_str(&format!(
        "- generated_at_utc: `{}`\n- total: `{}`\n- passed: `{}`\n- failed: `{}`\n- status: `{}`\n\n",
        report.generated_at_utc,
        report.total,
        report.passed,
        report.failed,
        if report.pass { "PASS" } else { "FAIL" }
    ));
    md.push_str("| Check | Status | Duration (ms) | Exit | Command |\n");
    md.push_str("|---|---|---:|---:|---|\n");
    for check in &report.checks {
        md.push_str(&format!(
            "| {} | {} | {} | {} | `{}` |\n",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.duration_ms,
            check.exit_code,
            check.command.replace('|', "\\|")
        ));
    }
    md.push_str("\n## Tails\n\n");
    for check in &report.checks {
        md.push_str(&format!(
            "### {}\n\n- Status: `{}`\n- Command: `{}`\n\n",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.command
        ));
        if !check.stdout_tail.is_empty() {
            md.push_str("```text\n");
            md.push_str(&check.stdout_tail);
            md.push_str("\n```\n\n");
        }
        if !check.stderr_tail.is_empty() {
            md.push_str("```text\n");
            md.push_str(&check.stderr_tail);
            md.push_str("\n```\n\n");
        }
    }
    fs::write(path, md).with_context(|| format!("write {path}"))?;
    Ok(())
}

fn main() -> Result<()> {
    let checks = vec![
        CheckSpec {
            id: "legacy_mechanics_exhaustive_extract",
            args: &["run", "-p", "omega-tools", "--bin", "legacy_mechanics_exhaustive_extract"],
        },
        CheckSpec {
            id: "rust_mechanics_exhaustive_extract",
            args: &["run", "-p", "omega-tools", "--bin", "rust_mechanics_exhaustive_extract"],
        },
        CheckSpec {
            id: "mechanics_mapping_certify",
            args: &["run", "-p", "omega-tools", "--bin", "mechanics_mapping_certify"],
        },
        CheckSpec {
            id: "legacy_headless_replay",
            args: &["run", "-p", "omega-tools", "--bin", "legacy_headless_replay"],
        },
        CheckSpec {
            id: "rust_headless_replay",
            args: &["run", "-p", "omega-tools", "--bin", "rust_headless_replay"],
        },
        CheckSpec {
            id: "mechanics_differential_certify",
            args: &["run", "-p", "omega-tools", "--bin", "mechanics_differential_certify"],
        },
        CheckSpec {
            id: "service_branch_differential_certify",
            args: &["run", "-p", "omega-tools", "--bin", "service_branch_differential_certify"],
        },
        CheckSpec {
            id: "branch_coverage_certify",
            args: &["run", "-p", "omega-tools", "--bin", "branch_coverage_certify"],
        },
        CheckSpec {
            id: "blackbox_adversarial_certify",
            args: &["run", "-p", "omega-tools", "--bin", "blackbox_adversarial_certify"],
        },
        CheckSpec {
            id: "parity_certify",
            args: &["run", "-p", "omega-tools", "--bin", "parity_certify"],
        },
        CheckSpec {
            id: "legacy_mechanics_extract",
            args: &["run", "-p", "omega-tools", "--bin", "legacy_mechanics_extract"],
        },
        CheckSpec {
            id: "rust_mechanics_extract",
            args: &["run", "-p", "omega-tools", "--bin", "rust_mechanics_extract"],
        },
        CheckSpec {
            id: "mechanics_parity_matrix",
            args: &["run", "-p", "omega-tools", "--bin", "mechanics_parity_matrix"],
        },
        CheckSpec {
            id: "mechanics_missing_board",
            args: &["run", "-p", "omega-tools", "--bin", "mechanics_missing_board"],
        },
        CheckSpec {
            id: "mechanics_smoke_suite",
            args: &["run", "-p", "omega-tools", "--bin", "mechanics_smoke_suite"],
        },
        CheckSpec {
            id: "classic_site_service_parity",
            args: &["run", "-p", "omega-tools", "--bin", "classic_site_service_parity"],
        },
        CheckSpec {
            id: "legacy_command_binding_parity",
            args: &["run", "-p", "omega-tools", "--bin", "legacy_command_binding_parity"],
        },
        CheckSpec {
            id: "bevy_semantic_projection_parity",
            args: &["run", "-p", "omega-tools", "--bin", "bevy_semantic_projection_parity"],
        },
        CheckSpec {
            id: "service_branch_oracle",
            args: &["run", "-p", "omega-tools", "--bin", "service_branch_oracle"],
        },
        CheckSpec {
            id: "guild_service_talk_clarity",
            args: &["run", "-p", "omega-tools", "--bin", "guild_service_talk_clarity"],
        },
        CheckSpec {
            id: "service_branch_blackbox_smoke",
            args: &["run", "-p", "omega-tools", "--bin", "service_branch_blackbox_smoke"],
        },
        CheckSpec {
            id: "guild_live_check",
            args: &["run", "-p", "omega-tools", "--bin", "guild_live_check"],
        },
        CheckSpec {
            id: "classic_inventory_parity",
            args: &["run", "-p", "omega-tools", "--bin", "classic_inventory_parity"],
        },
        CheckSpec {
            id: "classic_magic_item_parity",
            args: &["run", "-p", "omega-tools", "--bin", "classic_magic_item_parity"],
        },
        CheckSpec {
            id: "projectile_parity_matrix",
            args: &["run", "-p", "omega-tools", "--bin", "projectile_parity_matrix"],
        },
        CheckSpec {
            id: "projectile_parity_smoke",
            args: &["run", "-p", "omega-tools", "--bin", "projectile_parity_smoke"],
        },
        CheckSpec {
            id: "quest_parity_matrix",
            args: &["run", "-p", "omega-tools", "--bin", "quest_parity_matrix"],
        },
        CheckSpec {
            id: "quest_parity_smoke",
            args: &["run", "-p", "omega-tools", "--bin", "quest_parity_smoke"],
        },
        CheckSpec {
            id: "overworld_location_parity",
            args: &["run", "-p", "omega-tools", "--bin", "overworld_location_parity"],
        },
        CheckSpec {
            id: "legacy_disorientation_parity",
            args: &["run", "-p", "omega-tools", "--bin", "legacy_disorientation_parity"],
        },
        CheckSpec {
            id: "legacy_enchant_bless_decurse_parity",
            args: &["run", "-p", "omega-tools", "--bin", "legacy_enchant_bless_decurse_parity"],
        },
        CheckSpec {
            id: "legacy_victory_parity",
            args: &["run", "-p", "omega-tools", "--bin", "legacy_victory_parity"],
        },
        CheckSpec {
            id: "arena_portcullis_smoke",
            args: &["run", "-p", "omega-tools", "--bin", "arena_portcullis_smoke"],
        },
        CheckSpec {
            id: "runtime_user_regression_smoke",
            args: &["run", "-p", "omega-tools", "--bin", "runtime_user_regression_smoke"],
        },
        CheckSpec {
            id: "classic_mode_drift_guard",
            args: &["run", "-p", "omega-tools", "--bin", "classic_mode_drift_guard"],
        },
        CheckSpec {
            id: "classic_objective_drift_guard",
            args: &["run", "-p", "omega-tools", "--bin", "classic_objective_drift_guard"],
        },
        CheckSpec {
            id: "modern_mode_smoke",
            args: &["run", "-p", "omega-tools", "--bin", "modern_mode_smoke"],
        },
        CheckSpec {
            id: "modern_bevy_visual_smoke",
            args: &["run", "-p", "omega-tools", "--bin", "modern_bevy_visual_smoke"],
        },
        CheckSpec {
            id: "modern_objective_blackbox_smoke",
            args: &["run", "-p", "omega-tools", "--bin", "modern_objective_blackbox_smoke"],
        },
        CheckSpec {
            id: "bevy_visual_blackbox_suite",
            args: &["run", "-p", "omega-tools", "--bin", "bevy_visual_blackbox_suite"],
        },
        CheckSpec {
            id: "dual_mode_blackbox_suite",
            args: &["run", "-p", "omega-tools", "--bin", "dual_mode_blackbox_suite"],
        },
        CheckSpec {
            id: "mode_artifact_integrity_guard",
            args: &["run", "-p", "omega-tools", "--bin", "mode_artifact_integrity_guard"],
        },
        CheckSpec {
            id: "classic_visual_drift_guard",
            args: &["run", "-p", "omega-tools", "--bin", "classic_visual_drift_guard"],
        },
        CheckSpec {
            id: "classic_frontend_workflow_parity",
            args: &["run", "-p", "omega-tools", "--bin", "classic_frontend_workflow_parity"],
        },
        CheckSpec {
            id: "replay_matrix_gen",
            args: &["run", "-p", "omega-tools", "--bin", "replay_matrix_gen"],
        },
        CheckSpec {
            id: "replay_tool",
            args: &[
                "run",
                "-p",
                "omega-tools",
                "--bin",
                "replay_tool",
                "--",
                "--min-scenarios",
                "500",
            ],
        },
        CheckSpec {
            id: "true_parity_refresh",
            args: &["run", "-p", "omega-tools", "--bin", "true_parity_refresh"],
        },
    ];

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }

    let mut results = Vec::with_capacity(checks.len());
    for spec in &checks {
        println!("running {}", spec.id);
        let result = run_check(spec)?;
        println!("  {} ({:?} ms)", if result.pass { "PASS" } else { "FAIL" }, result.duration_ms);
        results.push(result);
    }

    let total = results.len();
    let passed = results.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = LiveChecksReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks: results,
    };

    fs::write(
        "target/live-checks-all.json",
        serde_json::to_string_pretty(&report).context("serialize live checks report")?,
    )
    .context("write target/live-checks-all.json")?;
    write_markdown("target/live-checks-all.md", &report)?;

    println!(
        "live checks all: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("one or more live checks failed");
    }
    Ok(())
}
