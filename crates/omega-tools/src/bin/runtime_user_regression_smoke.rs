use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct SmokeCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Serialize)]
struct RuntimeUserRegressionSmoke {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<SmokeCheck>,
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

fn bool_field(value: &Value, key: &str) -> bool {
    value[key].as_bool().unwrap_or(false)
}

fn main() -> Result<()> {
    let arena = read_json("target/arena-portcullis-smoke.json")?;
    let inventory = read_json("target/classic-inventory-contract.json")?;
    let service = read_json("target/service-branch-oracle.json")?;
    let quest = read_json("target/quest-parity-smoke.json")?;
    let victory = read_json("target/legacy-victory-parity.json")?;

    let inventory_look_show_ok = inventory["runtime_checks"]
        .as_array()
        .map(|checks| {
            checks.iter().any(|row| {
                row["id"].as_str() == Some("inventory_look_vs_show_parity")
                    && row["passed"].as_bool().unwrap_or(false)
            })
        })
        .unwrap_or(false);

    let inventory_show_ok = inventory["runtime_checks"]
        .as_array()
        .map(|checks| {
            checks.iter().any(|row| {
                row["id"].as_str() == Some("inventory_modal_open")
                    && row["passed"].as_bool().unwrap_or(false)
            })
        })
        .unwrap_or(false);

    let checks = vec![
        SmokeCheck {
            id: "arena_non_terminal_and_exit_after_opener".to_string(),
            passed: bool_field(&arena, "pass")
                && bool_field(&arena, "opener_dropped")
                && bool_field(&arena, "exited_after_opener"),
            details: format!(
                "pass={} opener_dropped={} exited_after_opener={}",
                bool_field(&arena, "pass"),
                bool_field(&arena, "opener_dropped"),
                bool_field(&arena, "exited_after_opener")
            ),
        },
        SmokeCheck {
            id: "inventory_show_and_look".to_string(),
            passed: bool_field(&inventory, "pass") && inventory_show_ok && inventory_look_show_ok,
            details: format!(
                "pass={} inventory_modal_open={} inventory_look_vs_show_parity={}",
                bool_field(&inventory, "pass"),
                inventory_show_ok,
                inventory_look_show_ok
            ),
        },
        SmokeCheck {
            id: "service_branch_oracle".to_string(),
            passed: bool_field(&service, "pass"),
            details: format!(
                "passed={}/{}",
                service["passed"].as_u64().unwrap_or(0),
                service["total"].as_u64().unwrap_or(0)
            ),
        },
        SmokeCheck {
            id: "quest_smoke".to_string(),
            passed: bool_field(&quest, "pass"),
            details: format!(
                "steps={} pass={}",
                quest["steps"].as_array().map(|steps| steps.len()).unwrap_or(0),
                bool_field(&quest, "pass")
            ),
        },
        SmokeCheck {
            id: "victory_parity".to_string(),
            passed: bool_field(&victory, "pass"),
            details: format!(
                "passed={}/{}",
                victory["passed"].as_u64().unwrap_or(0),
                victory["total"].as_u64().unwrap_or(0)
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let report = RuntimeUserRegressionSmoke {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
    };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }
    fs::write(
        "target/runtime-user-regression-smoke.json",
        serde_json::to_string_pretty(&report).context("serialize runtime smoke")?,
    )
    .context("write target/runtime-user-regression-smoke.json")?;

    let mut md = String::new();
    md.push_str("# Runtime User Regression Smoke\n\n");
    md.push_str(&format!(
        "- generated_at_utc: `{}`\n- total: `{}`\n- passed: `{}`\n- failed: `{}`\n- status: `{}`\n\n",
        report.generated_at_utc,
        report.total,
        report.passed,
        report.failed,
        if report.pass { "PASS" } else { "FAIL" }
    ));
    md.push_str("| Check | Status | Details |\n");
    md.push_str("|---|---|---|\n");
    for check in &report.checks {
        md.push_str(&format!(
            "| {} | {} | {} |\n",
            check.id,
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    fs::write("target/runtime-user-regression-smoke.md", md)
        .context("write target/runtime-user-regression-smoke.md")?;

    println!(
        "runtime user regression smoke: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("runtime user regression smoke failed");
    }
    Ok(())
}
