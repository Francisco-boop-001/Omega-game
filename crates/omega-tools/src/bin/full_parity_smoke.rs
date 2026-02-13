use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SmokeCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FullParitySmoke {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<SmokeCheck>,
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

fn artifact_pass(path: &str) -> Option<bool> {
    let value = read_json(path)?;
    if let Some(pass) = value.get("pass").and_then(Value::as_bool) {
        return Some(pass);
    }
    if let Some(status) = value.get("status").and_then(Value::as_str) {
        return Some(status.eq_ignore_ascii_case("PASS"));
    }
    if let (Some(missing), Some(partial), Some(key_conflict)) = (
        value.get("missing").and_then(Value::as_i64),
        value.get("partial").and_then(Value::as_i64),
        value.get("key_conflict").and_then(Value::as_i64),
    ) {
        return Some(missing == 0 && partial == 0 && key_conflict == 0);
    }
    value
        .get("summary")
        .and_then(|summary| summary.get("open_total"))
        .and_then(Value::as_i64)
        .map(|open| open == 0)
}

fn check_artifact(id: &str, path: &str) -> SmokeCheck {
    let exists = Path::new(path).exists();
    if !exists {
        return SmokeCheck {
            id: id.to_string(),
            passed: false,
            details: format!("missing artifact: {path}"),
        };
    }
    match artifact_pass(path) {
        Some(true) => {
            SmokeCheck { id: id.to_string(), passed: true, details: format!("{path} reports PASS") }
        }
        Some(false) => SmokeCheck {
            id: id.to_string(),
            passed: false,
            details: format!("{path} reports FAIL"),
        },
        None => SmokeCheck {
            id: id.to_string(),
            passed: false,
            details: format!("{path} has no parseable pass/status markers"),
        },
    }
}

fn markdown(report: &FullParitySmoke) -> String {
    let mut out = Vec::new();
    out.push("# Full Parity Smoke".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", report.generated_at_utc));
    out.push(format!("- Total checks: {}", report.total));
    out.push(format!("- Passed: {}", report.passed));
    out.push(format!("- Failed: {}", report.failed));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Check | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for check in &report.checks {
        out.push(format!(
            "| {} | {} | {} |",
            check.id,
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let checks = vec![
        check_artifact("startup_parity", "target/true-startup-parity.json"),
        check_artifact("site_service_parity", "target/classic-site-service-parity-matrix.json"),
        check_artifact("inventory_parity", "target/classic-inventory-contract.json"),
        check_artifact("magic_item_parity", "target/classic-magic-item-parity.json"),
        check_artifact("progression_parity", "target/classic-progression-branch-matrix.json"),
        check_artifact("compatibility_parity", "target/classic-compatibility-matrix.json"),
        check_artifact("arena_gate_smoke", "target/arena-portcullis-smoke.json"),
        check_artifact("magic_subsystem_smoke", "target/magic-subsystem-smoke.json"),
        check_artifact("true_parity_gate", "target/true-parity-gate.json"),
        check_artifact("full_parity_defect_board", "target/full-parity-defect-board.json"),
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let report = FullParitySmoke {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("full-parity-smoke.json");
    let md_path = target.join("full-parity-smoke.md");

    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize full parity smoke")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "full parity smoke: total={}, passed={}, failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("full parity smoke failed");
    }
    Ok(())
}
