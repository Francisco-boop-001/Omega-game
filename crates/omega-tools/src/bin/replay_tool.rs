use anyhow::{Context, Result};
use omega_tools::replay;
use std::fs;
use std::path::PathBuf;

fn default_fixture_dir() -> PathBuf {
    PathBuf::from("crates/omega-tools/fixtures/replay")
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn write_outputs(dashboard: &replay::RegressionDashboard) -> Result<()> {
    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("failed to create target directory")?;
    }

    let json_path = target.join("ws-d-regression-dashboard.json");
    let md_path = target.join("ws-d-regression-dashboard.md");

    let json = serde_json::to_string_pretty(dashboard).context("failed to serialize dashboard")?;
    fs::write(&json_path, json)
        .with_context(|| format!("failed to write {}", json_path.display()))?;

    let markdown = replay::dashboard_markdown(dashboard);
    fs::write(&md_path, markdown)
        .with_context(|| format!("failed to write {}", md_path.display()))?;

    Ok(())
}

fn main() -> Result<()> {
    let mut fixture_dir = default_fixture_dir();
    let mut min_scenarios = 0usize;
    let mut summary_only = false;
    let mut include_inactive = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--fixture-dir" => {
                let value =
                    args.next().ok_or_else(|| anyhow::anyhow!("--fixture-dir requires a value"))?;
                fixture_dir = PathBuf::from(value);
            }
            "--min-scenarios" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("--min-scenarios requires a value"))?;
                min_scenarios = value.parse::<usize>().context("invalid --min-scenarios")?;
            }
            "--summary-only" => {
                summary_only = true;
            }
            "--include-inactive" => {
                include_inactive = true;
            }
            other => {
                if other.starts_with("--") {
                    anyhow::bail!("unknown argument: {other}");
                }
                fixture_dir = PathBuf::from(other);
            }
        }
    }

    let dashboard = replay::run_dashboard_from_dir(&fixture_dir)
        .with_context(|| format!("failed to run fixtures from {}", fixture_dir.display()))?;

    if !summary_only {
        for scenario in &dashboard.scenarios {
            let status = if scenario.passed { "PASS" } else { "FAIL" };
            let activity = if scenario.active { "active" } else { "inactive" };
            println!(
                "[{}] {} ({}, turn={}, minutes={})",
                status, scenario.name, activity, scenario.final_turn, scenario.final_minutes
            );
            if !scenario.passed {
                for check in &scenario.checks {
                    println!("  - {}", check);
                }
            }
        }
    }

    println!(
        "Summary: total={} passed={} failed={} | active_total={} active_passed={} failed_active={} | inactive_total={} inactive_passed={} failed_inactive={} schema_mismatch={}",
        dashboard.total,
        dashboard.passed,
        dashboard.failed,
        dashboard.active_total,
        dashboard.active_passed,
        dashboard.failed_active,
        dashboard.inactive_total,
        dashboard.inactive_passed,
        dashboard.failed_inactive,
        dashboard.schema_mismatch_total
    );

    let denominator = if include_inactive { dashboard.total } else { dashboard.active_total };
    if denominator < min_scenarios {
        anyhow::bail!(
            "scenario denominator too small: {} < required {}",
            denominator,
            min_scenarios
        );
    }

    write_outputs(&dashboard)?;

    let failures = if include_inactive { dashboard.failed } else { dashboard.failed_active };
    if failures > 0 {
        anyhow::bail!("regression failures detected")
    }

    Ok(())
}
