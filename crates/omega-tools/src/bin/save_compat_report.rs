use anyhow::{Context, Result, bail};
use omega_save::{decode_json, decode_state_json};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SaveCaseResult {
    file: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SaveCompatReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass_rate_percent_x100: u64,
    cases: Vec<SaveCaseResult>,
}

fn fixture_dir() -> PathBuf {
    PathBuf::from("crates/omega-tools/fixtures/save-compat")
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn collect_json(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("read {}", dir.display()))? {
        let path = entry?.path();
        if path.is_dir() {
            collect_json(&path, out)?;
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            out.push(path);
        }
    }
    Ok(())
}

fn evaluate_case(path: &Path) -> SaveCaseResult {
    let label = path.display().to_string();
    let raw = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(err) => {
            return SaveCaseResult {
                file: label,
                passed: false,
                details: format!("read failed: {err}"),
            };
        }
    };

    match (decode_json(&raw), decode_state_json(&raw)) {
        (Ok(envelope), Ok(state)) => {
            if envelope.version != 1 {
                return SaveCaseResult {
                    file: label,
                    passed: false,
                    details: format!("migrated version={}, expected 1", envelope.version),
                };
            }
            if envelope.metadata.saved_turn != state.clock.turn
                || envelope.metadata.saved_minutes != state.clock.minutes
            {
                return SaveCaseResult {
                    file: label,
                    passed: false,
                    details: "metadata turn/minutes mismatch after decode".to_string(),
                };
            }
            SaveCaseResult {
                file: label,
                passed: true,
                details: "decoded and migrated to v1".to_string(),
            }
        }
        (Err(err), _) | (_, Err(err)) => {
            SaveCaseResult { file: label, passed: false, details: format!("decode failed: {err}") }
        }
    }
}

fn evaluate(fixtures: &[PathBuf]) -> SaveCompatReport {
    let cases: Vec<SaveCaseResult> = fixtures.iter().map(|path| evaluate_case(path)).collect();
    let total = cases.len();
    let passed = cases.iter().filter(|item| item.passed).count();
    let failed = total.saturating_sub(passed);
    let pass_rate_percent_x100 =
        if total == 0 { 0 } else { ((passed as u64) * 10_000) / (total as u64) };
    SaveCompatReport { total, passed, failed, pass_rate_percent_x100, cases }
}

fn markdown(report: &SaveCompatReport) -> String {
    let mut out = Vec::new();
    out.push("# Save Compatibility Report".to_string());
    out.push(String::new());
    out.push(format!("- Total cases: {}", report.total));
    out.push(format!("- Passed: {}", report.passed));
    out.push(format!("- Failed: {}", report.failed));
    out.push(format!(
        "- Pass rate: {}.{:02}%",
        report.pass_rate_percent_x100 / 100,
        report.pass_rate_percent_x100 % 100
    ));
    out.push(String::new());
    out.push("| Fixture | Result | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for item in &report.cases {
        out.push(format!(
            "| {} | {} | {} |",
            item.file,
            if item.passed { "PASS" } else { "FAIL" },
            item.details.replace('|', "\\|")
        ));
    }
    out.join("\n")
}

fn write_outputs(report: &SaveCompatReport) -> Result<()> {
    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }
    let json_path = target.join("save-compat-report.json");
    let md_path = target.join("save-compat-report.md");
    fs::write(&json_path, serde_json::to_string_pretty(report).context("serialize save report")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(report))
        .with_context(|| format!("write {}", md_path.display()))?;
    Ok(())
}

fn main() -> Result<()> {
    let fixtures_dir = fixture_dir();
    let mut fixtures = Vec::new();
    collect_json(&fixtures_dir, &mut fixtures)?;
    fixtures.sort();
    if fixtures.is_empty() {
        bail!("no save compatibility fixtures found under {}", fixtures_dir.display());
    }

    let report = evaluate(&fixtures);
    write_outputs(&report)?;
    println!(
        "save compatibility: total={}, passed={}, failed={}",
        report.total, report.passed, report.failed
    );
    if report.failed > 0 {
        bail!("save compatibility failures detected");
    }
    Ok(())
}
