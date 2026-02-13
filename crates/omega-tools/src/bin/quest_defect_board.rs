use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Defect {
    id: String,
    artifact: String,
    status: String,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct QuestDefectBoard {
    generated_at_utc: String,
    open_defects: Vec<Defect>,
    artifacts_scanned: usize,
}

fn now_unix() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn parse_pass(value: &Value) -> Option<bool> {
    value
        .get("pass")
        .and_then(Value::as_bool)
        .or_else(|| value.get("status").and_then(Value::as_str).map(|status| status == "PASS"))
}

fn markdown(board: &QuestDefectBoard) -> String {
    let mut out = Vec::new();
    out.push("# Quest Defect Board".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", board.generated_at_utc));
    out.push(format!("- Artifacts scanned: {}", board.artifacts_scanned));
    out.push(format!("- Open defects: {}", board.open_defects.len()));
    out.push(String::new());
    if board.open_defects.is_empty() {
        out.push("No open quest parity defects.".to_string());
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
        ("target/legacy-quest-contract.json", false),
        ("target/classic-site-service-parity-matrix.json", true),
        ("target/quest-parity-matrix.json", true),
        ("target/quest-parity-smoke.json", true),
    ];

    let mut defects = Vec::new();
    for (artifact, requires_pass_flag) in &artifacts {
        let path = Path::new(artifact);
        if !path.exists() {
            defects.push(Defect {
                id: format!("missing_{}", artifact.replace(['/', '-', '.'], "_")),
                artifact: (*artifact).to_string(),
                status: "MISSING".to_string(),
                details: "artifact not found".to_string(),
            });
            continue;
        }
        if !requires_pass_flag {
            continue;
        }

        let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        let value: Value =
            serde_json::from_str(&raw).with_context(|| format!("parse JSON {}", path.display()))?;
        let pass = parse_pass(&value).unwrap_or(false);
        if !pass {
            defects.push(Defect {
                id: format!("failing_{}", artifact.replace(['/', '-', '.'], "_")),
                artifact: (*artifact).to_string(),
                status: "FAIL".to_string(),
                details: "artifact reported pass=false".to_string(),
            });
        }
    }

    let board = QuestDefectBoard {
        generated_at_utc: now_unix(),
        artifacts_scanned: artifacts.len(),
        open_defects: defects,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("quest-defect-board.json");
    let md_path = target.join("quest-defect-board.md");
    fs::write(&json_path, serde_json::to_string_pretty(&board).context("serialize quest board")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&board))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "quest defect board: artifacts_scanned={} open_defects={}",
        board.artifacts_scanned,
        board.open_defects.len()
    );
    if !board.open_defects.is_empty() {
        bail!("quest defect board contains open defects");
    }
    Ok(())
}
