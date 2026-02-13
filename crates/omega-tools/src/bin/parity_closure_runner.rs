use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone, Serialize)]
struct CheckRun {
    id: String,
    command: String,
    pass: bool,
    exit_code: i32,
    duration_ms: u128,
    stdout_tail: String,
    stderr_tail: String,
}

#[derive(Debug, Clone, Serialize)]
struct DefectItem {
    id: String,
    severity: String,
    source: String,
    details: String,
    status: String,
}

#[derive(Debug, Clone, Serialize)]
struct GuildParityDefectBoard {
    generated_at_utc: String,
    total_checks: usize,
    failed_checks: usize,
    open: usize,
    closed: usize,
    items: Vec<DefectItem>,
    checks: Vec<CheckRun>,
}

#[derive(Debug, Clone, Deserialize)]
struct SiteBranchDiffRow {
    service: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SiteBranchDiffReport {
    #[serde(default)]
    rows: Vec<SiteBranchDiffRow>,
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
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() <= max_lines {
        return text.trim().to_string();
    }
    lines[lines.len() - max_lines..].join("\n")
}

fn run_check(spec: &CheckSpec) -> Result<CheckRun> {
    let started = Instant::now();
    let output = Command::new("cargo")
        .args(spec.args)
        .output()
        .with_context(|| format!("spawn {}", format_command(spec.args)))?;
    Ok(CheckRun {
        id: spec.id.to_string(),
        command: format_command(spec.args),
        pass: output.status.success(),
        exit_code: output.status.code().unwrap_or(-1),
        duration_ms: started.elapsed().as_millis(),
        stdout_tail: trim_tail(&String::from_utf8_lossy(&output.stdout), 10),
        stderr_tail: trim_tail(&String::from_utf8_lossy(&output.stderr), 10),
    })
}

fn read_json(path: &str) -> Result<Value> {
    let raw = fs::read_to_string(path).with_context(|| format!("read {path}"))?;
    serde_json::from_str::<Value>(&raw).with_context(|| format!("decode {path}"))
}

fn write_markdown(path: &str, board: &GuildParityDefectBoard) -> Result<()> {
    let mut md = String::new();
    md.push_str("# Guild Parity Defect Board\n\n");
    md.push_str(&format!(
        "- generated_at_utc: `{}`\n- total_checks: `{}`\n- failed_checks: `{}`\n- open: `{}`\n- closed: `{}`\n\n",
        board.generated_at_utc, board.total_checks, board.failed_checks, board.open, board.closed
    ));
    md.push_str("## Defects\n\n");
    if board.items.is_empty() {
        md.push_str("- none\n\n");
    } else {
        md.push_str("| ID | Severity | Source | Status | Details |\n");
        md.push_str("|---|---|---|---|---|\n");
        for item in &board.items {
            md.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                item.id,
                item.severity,
                item.source,
                item.status,
                item.details.replace('|', "\\|")
            ));
        }
        md.push('\n');
    }
    md.push_str("## Check Runs\n\n");
    md.push_str("| Check | Status | Exit | Duration (ms) |\n");
    md.push_str("|---|---|---:|---:|\n");
    for check in &board.checks {
        md.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.exit_code,
            check.duration_ms
        ));
    }
    fs::write(path, md).with_context(|| format!("write {path}"))?;
    Ok(())
}

fn main() -> Result<()> {
    let checks = vec![
        CheckSpec {
            id: "legacy_guild_site_contract",
            args: &["run", "-p", "omega-tools", "--bin", "legacy_guild_site_contract"],
        },
        CheckSpec {
            id: "legacy_site_branch_extract",
            args: &["run", "-p", "omega-tools", "--bin", "legacy_site_branch_extract"],
        },
        CheckSpec {
            id: "rust_site_branch_extract",
            args: &["run", "-p", "omega-tools", "--bin", "rust_site_branch_extract"],
        },
        CheckSpec {
            id: "site_branch_diff",
            args: &["run", "-p", "omega-tools", "--bin", "site_branch_diff"],
        },
        CheckSpec {
            id: "service_branch_oracle",
            args: &["run", "-p", "omega-tools", "--bin", "service_branch_oracle"],
        },
        CheckSpec {
            id: "classic_site_service_parity",
            args: &["run", "-p", "omega-tools", "--bin", "classic_site_service_parity"],
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
            id: "parity_certify",
            args: &["run", "-p", "omega-tools", "--bin", "parity_certify"],
        },
    ];

    let mut runs = Vec::with_capacity(checks.len());
    for check in &checks {
        println!("running {}", check.id);
        runs.push(run_check(check)?);
    }

    let mut items = Vec::new();
    for run in &runs {
        if !run.pass {
            items.push(DefectItem {
                id: format!("run-{}", run.id),
                severity: "P0".to_string(),
                source: run.id.clone(),
                details: format!("command failed: {}", run.command),
                status: "OPEN".to_string(),
            });
        }
    }

    if Path::new("target/site-branch-diff.json").exists() {
        let diff_raw = fs::read_to_string("target/site-branch-diff.json")
            .context("read target/site-branch-diff.json")?;
        let diff: SiteBranchDiffReport =
            serde_json::from_str(&diff_raw).context("decode target/site-branch-diff.json")?;
        for row in diff.rows.into_iter().filter(|row| !row.pass) {
            items.push(DefectItem {
                id: format!("branch-{}", row.service),
                severity: "P0".to_string(),
                source: "site_branch_diff".to_string(),
                details: row.details,
                status: "OPEN".to_string(),
            });
        }
    }

    if Path::new("target/guild-live-check.json").exists() {
        let check_json = read_json("target/guild-live-check.json")?;
        if !check_json["pass"].as_bool().unwrap_or(false) {
            items.push(DefectItem {
                id: "guild-live-check".to_string(),
                severity: "P0".to_string(),
                source: "guild_live_check".to_string(),
                details: format!(
                    "failed={} of total={}",
                    check_json["failed"].as_u64().unwrap_or(0),
                    check_json["total"].as_u64().unwrap_or(0)
                ),
                status: "OPEN".to_string(),
            });
        }
    }

    if Path::new("target/service-branch-blackbox-smoke.json").exists() {
        let check_json = read_json("target/service-branch-blackbox-smoke.json")?;
        if !check_json["pass"].as_bool().unwrap_or(false) {
            items.push(DefectItem {
                id: "service-branch-blackbox-smoke".to_string(),
                severity: "P0".to_string(),
                source: "service_branch_blackbox_smoke".to_string(),
                details: format!(
                    "failed={} of total={}",
                    check_json["failed"].as_u64().unwrap_or(0),
                    check_json["total"].as_u64().unwrap_or(0)
                ),
                status: "OPEN".to_string(),
            });
        }
    } else {
        items.push(DefectItem {
            id: "service-branch-blackbox-smoke".to_string(),
            severity: "P0".to_string(),
            source: "service_branch_blackbox_smoke".to_string(),
            details: "missing target/service-branch-blackbox-smoke.json".to_string(),
            status: "OPEN".to_string(),
        });
    }

    if Path::new("target/certification/parity-certify.json").exists() {
        let check_json = read_json("target/certification/parity-certify.json")?;
        if !check_json["pass"].as_bool().unwrap_or(false) {
            items.push(DefectItem {
                id: "certification-parity".to_string(),
                severity: "P0".to_string(),
                source: "parity_certify".to_string(),
                details: format!(
                    "failed={} of total={}",
                    check_json["failed"].as_u64().unwrap_or(0),
                    check_json["total"].as_u64().unwrap_or(0)
                ),
                status: "OPEN".to_string(),
            });
        }
    } else {
        items.push(DefectItem {
            id: "certification-parity".to_string(),
            severity: "P0".to_string(),
            source: "parity_certify".to_string(),
            details: "missing target/certification/parity-certify.json".to_string(),
            status: "OPEN".to_string(),
        });
    }

    let failed_checks = runs.iter().filter(|run| !run.pass).count();
    let open = items.iter().filter(|item| item.status == "OPEN").count();
    let closed = items.len().saturating_sub(open);
    let board = GuildParityDefectBoard {
        generated_at_utc: now_utc_unix(),
        total_checks: runs.len(),
        failed_checks,
        open,
        closed,
        items,
        checks: runs,
    };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }
    fs::write(
        "target/guild-parity-defect-board.json",
        serde_json::to_string_pretty(&board).context("serialize guild parity defect board")?,
    )
    .context("write target/guild-parity-defect-board.json")?;
    write_markdown("target/guild-parity-defect-board.md", &board)?;

    println!(
        "parity closure runner: checks={} failed_checks={} open_defects={}",
        board.total_checks, board.failed_checks, board.open
    );
    if board.open > 0 {
        bail!("guild parity defects remain");
    }
    Ok(())
}
