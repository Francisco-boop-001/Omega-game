use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct IntegrityCheck {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ArtifactIntegrityReport {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<IntegrityCheck>,
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

fn write_json<T: Serialize>(path: &str, value: &T) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, serde_json::to_string_pretty(value).context("serialize json")?)
        .with_context(|| format!("write {path}"))
}

fn main() -> Result<()> {
    let classic_path = "target/classic/classic-mode-drift-guard.json";
    let modern_path = "target/modern/modern-mode-smoke.json";
    let dual_path = "target/dual/dual-mode-blackbox-suite.json";
    let root_collision_candidates = [
        "target/classic-mode-drift-guard.json",
        "target/modern-mode-smoke.json",
        "target/dual-mode-blackbox-suite.json",
    ];

    let classic = read_json(classic_path)?;
    let modern = read_json(modern_path)?;
    let dual = read_json(dual_path)?;

    let checks = vec![
        IntegrityCheck {
            id: "classic_artifact_present_and_pass".to_string(),
            pass: bool_field(&classic, "pass") && classic["generated_at_utc"].as_str().is_some(),
            details: format!(
                "pass={} generated_at_utc={}",
                bool_field(&classic, "pass"),
                classic["generated_at_utc"].as_str().unwrap_or("missing")
            ),
        },
        IntegrityCheck {
            id: "modern_artifact_present_and_pass".to_string(),
            pass: bool_field(&modern, "pass") && modern["generated_at_utc"].as_str().is_some(),
            details: format!(
                "pass={} generated_at_utc={}",
                bool_field(&modern, "pass"),
                modern["generated_at_utc"].as_str().unwrap_or("missing")
            ),
        },
        IntegrityCheck {
            id: "dual_artifact_present_and_pass".to_string(),
            pass: bool_field(&dual, "pass") && dual["generated_at_utc"].as_str().is_some(),
            details: format!(
                "pass={} generated_at_utc={}",
                bool_field(&dual, "pass"),
                dual["generated_at_utc"].as_str().unwrap_or("missing")
            ),
        },
        IntegrityCheck {
            id: "root_collision_absent".to_string(),
            pass: root_collision_candidates.iter().all(|path| !Path::new(path).exists()),
            details: format!("checked={}", root_collision_candidates.join(",")),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = ArtifactIntegrityReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
    };

    write_json("target/mode-artifact-integrity-guard.json", &report)?;
    write_json("target/mode_artifact_integrity_guard.json", &report)?;

    println!(
        "mode artifact integrity guard: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("mode artifact integrity guard failed");
    }
    Ok(())
}
