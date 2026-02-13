use anyhow::{Result, bail};
use omega_tools::audit_contract::{
    DiffResult, StateVector, diff_dir, ensure_cert_dirs, read_json, write_json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ScenarioSnapshot {
    id: String,
    family: String,
    active: bool,
    state: StateVector,
}

#[derive(Debug, Deserialize)]
struct ReplayReport {
    scenarios: Vec<ScenarioSnapshot>,
}

#[derive(Debug, Deserialize)]
struct GenericPass {
    pass: bool,
    total: usize,
    passed: usize,
    failed: usize,
}

#[derive(Debug, Serialize)]
struct ServiceDifferentialReport {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    results: Vec<DiffResult>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn compare_opt<T: PartialEq + std::fmt::Debug>(
    name: &str,
    legacy: &Option<T>,
    rust: &Option<T>,
    out: &mut Vec<String>,
) {
    if let Some(legacy_value) = legacy {
        match rust {
            Some(rust_value) if rust_value == legacy_value => {}
            Some(rust_value) => out.push(format!(
                "{} mismatch: legacy={:?} rust={:?}",
                name, legacy_value, rust_value
            )),
            None => out.push(format!("{} missing in rust snapshot", name)),
        }
    }
}

fn main() -> Result<()> {
    ensure_cert_dirs()?;
    let legacy: ReplayReport = read_json(diff_dir().join("legacy-headless-replay.json"))?;
    let rust: ReplayReport = read_json(diff_dir().join("rust-headless-replay.json"))?;
    let blackbox: GenericPass = read_json("target/service-branch-blackbox-smoke.json")?;

    let mut results = Vec::new();
    for legacy_scenario in legacy
        .scenarios
        .iter()
        .filter(|scenario| scenario.active && scenario.family.contains("site"))
    {
        let mut mismatches = Vec::new();
        let rust_scenario =
            rust.scenarios.iter().find(|scenario| scenario.id == legacy_scenario.id);
        if let Some(rust_scenario) = rust_scenario {
            compare_opt(
                "gold",
                &legacy_scenario.state.gold,
                &rust_scenario.state.gold,
                &mut mismatches,
            );
            compare_opt(
                "bank_gold",
                &legacy_scenario.state.bank_gold,
                &rust_scenario.state.bank_gold,
                &mut mismatches,
            );
            compare_opt(
                "guild_rank",
                &legacy_scenario.state.guild_rank,
                &rust_scenario.state.guild_rank,
                &mut mismatches,
            );
            compare_opt(
                "priest_rank",
                &legacy_scenario.state.priest_rank,
                &rust_scenario.state.priest_rank,
                &mut mismatches,
            );
            compare_opt(
                "alignment",
                &legacy_scenario.state.alignment,
                &rust_scenario.state.alignment,
                &mut mismatches,
            );
            compare_opt(
                "quest_state",
                &legacy_scenario.state.quest_state,
                &rust_scenario.state.quest_state,
                &mut mismatches,
            );
        } else {
            mismatches.push("missing rust service scenario".to_string());
        }

        let pass = mismatches.is_empty();
        results.push(DiffResult {
            id: legacy_scenario.id.clone(),
            pass,
            details: if pass {
                "service branch state aligned on certified fields".to_string()
            } else {
                "service branch mismatch".to_string()
            },
            mismatches,
        });
    }

    results.push(DiffResult {
        id: "blackbox_service_suite".to_string(),
        pass: blackbox.pass,
        details: format!(
            "blackbox pass={} passed={}/{} failed={}",
            blackbox.pass, blackbox.passed, blackbox.total, blackbox.failed
        ),
        mismatches: if blackbox.pass {
            Vec::new()
        } else {
            vec!["service_branch_blackbox_smoke failed".to_string()]
        },
    });

    let total = results.len();
    let passed = results.iter().filter(|result| result.pass).count();
    let failed = total.saturating_sub(passed);
    let report = ServiceDifferentialReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        results,
    };

    let out_path = diff_dir().join("service-branch-differential.json");
    write_json(&out_path, &report)?;
    println!(
        "service branch differential certify: total={} passed={} failed={} pass={}",
        report.total, report.passed, report.failed, report.pass
    );
    if !report.pass {
        bail!("service branch differential certification failed");
    }
    Ok(())
}
