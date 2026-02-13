use anyhow::{Result, bail};
use omega_tools::audit_contract::{
    DiffResult, StateVector, diff_dir, ensure_cert_dirs, read_json, write_json,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
struct ScenarioSnapshot {
    id: String,
    active: bool,
    input_trace: Vec<String>,
    state: StateVector,
}

#[derive(Debug, Deserialize)]
struct ReplayReport {
    total: usize,
    scenarios: Vec<ScenarioSnapshot>,
}

#[derive(Debug, Serialize)]
struct DifferentialReport {
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

    if legacy.total == 0 || rust.total == 0 {
        bail!("empty replay reports for differential certification");
    }

    let mut rust_by_id = BTreeMap::<String, ScenarioSnapshot>::new();
    for scenario in rust.scenarios {
        rust_by_id.insert(scenario.id.clone(), scenario);
    }

    let mut results = Vec::new();
    for legacy_scenario in legacy.scenarios {
        if !legacy_scenario.active {
            continue;
        }
        let mut mismatches = Vec::new();
        let Some(rust_scenario) = rust_by_id.get(&legacy_scenario.id) else {
            results.push(DiffResult {
                id: legacy_scenario.id,
                pass: false,
                details: "missing rust scenario".to_string(),
                mismatches: vec!["missing scenario in rust replay report".to_string()],
            });
            continue;
        };

        if legacy_scenario.input_trace != rust_scenario.input_trace {
            mismatches.push("input trace mismatch".to_string());
        }

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
        compare_opt(
            "inventory_count",
            &legacy_scenario.state.inventory_count,
            &rust_scenario.state.inventory_count,
            &mut mismatches,
        );
        compare_opt(
            "world_mode",
            &legacy_scenario.state.world_mode,
            &rust_scenario.state.world_mode,
            &mut mismatches,
        );

        let pass = mismatches.is_empty();
        results.push(DiffResult {
            id: legacy_scenario.id,
            pass,
            details: if pass {
                "state vector aligned on certified fields".to_string()
            } else {
                "differential mismatch".to_string()
            },
            mismatches,
        });
    }

    let total = results.len();
    let passed = results.iter().filter(|result| result.pass).count();
    let failed = total.saturating_sub(passed);
    let report = DifferentialReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        results,
    };
    let out_path = diff_dir().join("mechanics-differential.json");
    write_json(&out_path, &report)?;

    println!(
        "mechanics differential certify: total={} passed={} failed={} pass={}",
        report.total, report.passed, report.failed, report.pass
    );
    if !report.pass {
        bail!("mechanics differential certification failed");
    }
    Ok(())
}
