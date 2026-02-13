use anyhow::{Context, Result, bail};
use omega_bevy::run_headless_bootstrap as bevy_bootstrap;
use omega_tui::run_headless_bootstrap as tui_bootstrap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FrontendBootStats {
    frontend: String,
    attempts: usize,
    succeeded: usize,
    failed: usize,
    success_rate_percent_x100: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct BootReliabilityReport {
    schema_version: u32,
    threshold_percent_x100: u64,
    attempts_per_frontend: usize,
    total_attempts: usize,
    total_succeeded: usize,
    total_failed: usize,
    success_rate_percent_x100: u64,
    status: String,
    frontends: Vec<FrontendBootStats>,
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn run_frontend(
    label: &str,
    attempts: usize,
    mut f: impl FnMut() -> Result<()>,
) -> FrontendBootStats {
    let mut succeeded = 0usize;
    for _ in 0..attempts {
        if f().is_ok() {
            succeeded += 1;
        }
    }
    let failed = attempts.saturating_sub(succeeded);
    let rate = if attempts == 0 { 0 } else { ((succeeded as u64) * 10_000) / attempts as u64 };
    FrontendBootStats {
        frontend: label.to_string(),
        attempts,
        succeeded,
        failed,
        success_rate_percent_x100: rate,
    }
}

fn markdown(report: &BootReliabilityReport) -> String {
    let mut out = Vec::new();
    out.push("# M5 Boot Reliability Report".to_string());
    out.push(String::new());
    out.push(format!(
        "- Success rate: {}.{:02}%",
        report.success_rate_percent_x100 / 100,
        report.success_rate_percent_x100 % 100
    ));
    out.push(format!(
        "- Threshold: {}.{:02}%",
        report.threshold_percent_x100 / 100,
        report.threshold_percent_x100 % 100
    ));
    out.push(format!(
        "- Attempts: total={} ({} per frontend)",
        report.total_attempts, report.attempts_per_frontend
    ));
    out.push(format!("- Status: {}", report.status));
    out.push(String::new());
    out.push("| Frontend | Attempts | Succeeded | Failed | Success rate |".to_string());
    out.push("|---|---:|---:|---:|---:|".to_string());
    for item in &report.frontends {
        out.push(format!(
            "| {} | {} | {} | {} | {}.{:02}% |",
            item.frontend,
            item.attempts,
            item.succeeded,
            item.failed,
            item.success_rate_percent_x100 / 100,
            item.success_rate_percent_x100 % 100
        ));
    }
    out.join("\n")
}

fn main() -> Result<()> {
    let attempts_per_frontend = 1000usize;
    let threshold_percent_x100 = 9_995u64;

    let tui = run_frontend("tui", attempts_per_frontend, || {
        let _ = tui_bootstrap().context("tui bootstrap failed")?;
        Ok(())
    });
    let bevy = run_frontend("bevy", attempts_per_frontend, || {
        let _ = bevy_bootstrap().context("bevy bootstrap failed")?;
        Ok(())
    });

    let total_attempts = tui.attempts + bevy.attempts;
    let total_succeeded = tui.succeeded + bevy.succeeded;
    let total_failed = total_attempts.saturating_sub(total_succeeded);
    let success_rate_percent_x100 = if total_attempts == 0 {
        0
    } else {
        ((total_succeeded as u64) * 10_000) / total_attempts as u64
    };

    let status = if success_rate_percent_x100 >= threshold_percent_x100 {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };

    let report = BootReliabilityReport {
        schema_version: 1,
        threshold_percent_x100,
        attempts_per_frontend,
        total_attempts,
        total_succeeded,
        total_failed,
        success_rate_percent_x100,
        status: status.clone(),
        frontends: vec![tui, bevy],
    };

    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }
    let json_path = target.join("m5-boot-reliability.json");
    let md_path = target.join("m5-boot-reliability.md");
    fs::write(&json_path, serde_json::to_string_pretty(&report).context("serialize boot report")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "m5 boot reliability: attempts={}, succeeded={}, failed={}, status={}",
        report.total_attempts, report.total_succeeded, report.total_failed, report.status
    );

    if report.status != "PASS" {
        bail!("boot reliability below threshold");
    }
    Ok(())
}
