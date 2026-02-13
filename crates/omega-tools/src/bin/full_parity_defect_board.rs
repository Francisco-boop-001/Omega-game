use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DefectEntry {
    id: String,
    artifact: String,
    status: String,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DefectBoard {
    generated_at_utc: String,
    open_defects: Vec<DefectEntry>,
    summary: DefectSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DefectSummary {
    artifacts_scanned: usize,
    open_total: usize,
}

fn now_utc_unix() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn read_json(path: &str) -> Option<Value> {
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str::<Value>(&raw).ok()
}

fn parse_pass(value: &Value) -> Option<bool> {
    value
        .get("pass")
        .and_then(Value::as_bool)
        .or_else(|| {
            value
                .get("status")
                .and_then(Value::as_str)
                .map(|status| status.eq_ignore_ascii_case("PASS"))
        })
        .or_else(|| {
            let missing = value.get("missing").and_then(Value::as_i64)?;
            let partial = value.get("partial").and_then(Value::as_i64)?;
            let key_conflict = value.get("key_conflict").and_then(Value::as_i64)?;
            Some(missing == 0 && partial == 0 && key_conflict == 0)
        })
}

fn markdown(board: &DefectBoard) -> String {
    let mut out = Vec::new();
    out.push("# Full Parity Defect Board".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", board.generated_at_utc));
    out.push(format!("- Artifacts scanned: {}", board.summary.artifacts_scanned));
    out.push(format!("- Open defects: {}", board.summary.open_total));
    out.push(String::new());
    if board.open_defects.is_empty() {
        out.push("No open parity defects detected from strict artifact scan.".to_string());
        out.push(String::new());
        return out.join("\n");
    }
    out.push("| Defect | Artifact | Status | Details |".to_string());
    out.push("|---|---|---|---|".to_string());
    for defect in &board.open_defects {
        out.push(format!(
            "| {} | {} | {} | {} |",
            defect.id,
            defect.artifact,
            defect.status,
            defect.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let artifacts = vec![
        "target/classic-core-model-parity.json",
        "target/classic-command-parity-matrix.json",
        "target/classic-combat-encounter-parity.json",
        "target/classic-inventory-contract.json",
        "target/classic-magic-item-parity.json",
        "target/classic-progression-branch-matrix.json",
        "target/classic-site-service-parity-matrix.json",
        "target/classic-compatibility-matrix.json",
        "target/classic-frontend-workflow-parity.json",
        "target/true-parity-regression-dashboard.json",
        "target/true-parity-gate.json",
    ];

    let mut open_defects = Vec::new();

    for artifact in &artifacts {
        let path = Path::new(artifact);
        if !path.exists() {
            open_defects.push(DefectEntry {
                id: format!("missing_{}", artifact.replace(['/', '-', '.'], "_")),
                artifact: (*artifact).to_string(),
                status: "MISSING".to_string(),
                details: "required artifact not found".to_string(),
            });
            continue;
        }

        let Some(value) = read_json(artifact) else {
            open_defects.push(DefectEntry {
                id: format!("invalid_{}", artifact.replace(['/', '-', '.'], "_")),
                artifact: (*artifact).to_string(),
                status: "INVALID".to_string(),
                details: "artifact exists but JSON could not be parsed".to_string(),
            });
            continue;
        };

        let Some(pass) = parse_pass(&value) else {
            open_defects.push(DefectEntry {
                id: format!("unknown_status_{}", artifact.replace(['/', '-', '.'], "_")),
                artifact: (*artifact).to_string(),
                status: "UNKNOWN".to_string(),
                details: "artifact has no pass/status flag".to_string(),
            });
            continue;
        };

        if !pass {
            let details = value
                .get("failed")
                .and_then(Value::as_i64)
                .map(|failed| format!("failed checks={failed}"))
                .or_else(|| {
                    value
                        .get("failed_tracks")
                        .and_then(Value::as_i64)
                        .map(|failed| format!("failed tracks={failed}"))
                })
                .unwrap_or_else(|| "artifact pass flag is false".to_string());
            open_defects.push(DefectEntry {
                id: format!("failing_{}", artifact.replace(['/', '-', '.'], "_")),
                artifact: (*artifact).to_string(),
                status: "FAIL".to_string(),
                details,
            });
        }
    }

    let board = DefectBoard {
        generated_at_utc: now_utc_unix(),
        summary: DefectSummary {
            artifacts_scanned: artifacts.len(),
            open_total: open_defects.len(),
        },
        open_defects,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("full-parity-defect-board.json");
    let md_path = target.join("full-parity-defect-board.md");

    fs::write(&json_path, serde_json::to_string_pretty(&board).context("serialize defect board")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&board))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "full parity defect board: artifacts_scanned={} open_defects={}",
        board.summary.artifacts_scanned, board.summary.open_total
    );
    if board.summary.open_total > 0 {
        anyhow::bail!("open defects detected in strict artifact scan");
    }
    Ok(())
}
