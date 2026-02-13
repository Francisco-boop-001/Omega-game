use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DefectFamily {
    family: String,
    total: usize,
    passed: usize,
    failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FrontendDefect {
    id: String,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FixDefectBoard {
    generated_at_utc: String,
    frontend_workflow_pass: bool,
    replay_active_failed: usize,
    replay_active_total: usize,
    top_replay_families: Vec<DefectFamily>,
    frontend_failures: Vec<FrontendDefect>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn read_json(path: &str) -> Result<Value> {
    let raw = fs::read_to_string(path).with_context(|| format!("read {path}"))?;
    serde_json::from_str(&raw).with_context(|| format!("decode {path}"))
}

fn usize_field(value: &Value, key: &str) -> usize {
    value[key].as_u64().unwrap_or(0) as usize
}

fn main() -> Result<()> {
    let frontend = read_json("target/classic-frontend-workflow-parity.json")?;
    let replay = read_json("target/ws-d-regression-dashboard.json")?;

    let frontend_workflow_pass = frontend["pass"].as_bool().unwrap_or(false);
    let replay_active_failed = if replay.get("failed_active").is_some() {
        usize_field(&replay, "failed_active")
    } else {
        usize_field(&replay, "failed")
    };
    let replay_active_total = if replay.get("active_total").is_some() {
        usize_field(&replay, "active_total")
    } else {
        usize_field(&replay, "total")
    };

    let mut top_replay_families = Vec::new();
    if let Some(families) = replay["family_rollups"].as_array() {
        for family in families {
            let failed = usize_field(family, "failed");
            if failed == 0 {
                continue;
            }
            top_replay_families.push(DefectFamily {
                family: family["key"].as_str().unwrap_or("unknown").to_string(),
                total: usize_field(family, "total"),
                passed: usize_field(family, "passed"),
                failed,
            });
        }
        top_replay_families.sort_by(|a, b| b.failed.cmp(&a.failed).then(a.family.cmp(&b.family)));
    }

    let mut frontend_failures = Vec::new();
    if let Some(scenarios) = frontend["scenarios"].as_array() {
        for scenario in scenarios {
            if scenario["passed"].as_bool().unwrap_or(false) {
                continue;
            }
            frontend_failures.push(FrontendDefect {
                id: scenario["id"].as_str().unwrap_or("unknown").to_string(),
                details: scenario["details"].as_str().unwrap_or("no_details").to_string(),
            });
        }
    }

    let board = FixDefectBoard {
        generated_at_utc: now_utc_unix(),
        frontend_workflow_pass,
        replay_active_failed,
        replay_active_total,
        top_replay_families,
        frontend_failures,
    };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }

    fs::write(
        "target/fix-defect-board.json",
        serde_json::to_string_pretty(&board).context("serialize defect board")?,
    )
    .context("write target/fix-defect-board.json")?;

    let mut md = String::new();
    md.push_str("# Fix Defect Board\n\n");
    md.push_str(&format!("- generated_at_utc: `{}`\n", board.generated_at_utc));
    md.push_str(&format!("- frontend_workflow_pass: `{}`\n", board.frontend_workflow_pass));
    md.push_str(&format!(
        "- replay_active_failed: `{}` / `{}`\n\n",
        board.replay_active_failed, board.replay_active_total
    ));
    md.push_str("## Frontend Failures\n\n");
    if board.frontend_failures.is_empty() {
        md.push_str("- none\n\n");
    } else {
        for failure in &board.frontend_failures {
            md.push_str(&format!("- `{}`: {}\n", failure.id, failure.details));
        }
        md.push('\n');
    }
    md.push_str("## Replay Families\n\n");
    if board.top_replay_families.is_empty() {
        md.push_str("- none\n");
    } else {
        md.push_str("| Family | Total | Passed | Failed |\n");
        md.push_str("|---|---:|---:|---:|\n");
        for family in &board.top_replay_families {
            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                family.family, family.total, family.passed, family.failed
            ));
        }
    }
    fs::write("target/fix-defect-board.md", md).context("write target/fix-defect-board.md")?;

    println!(
        "fix defect board: frontend_pass={} replay_failed_active={}/{} families={}",
        board.frontend_workflow_pass,
        board.replay_active_failed,
        board.replay_active_total,
        board.top_replay_families.len()
    );
    Ok(())
}
