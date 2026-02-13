use anyhow::{Context, Result, bail};
use omega_tools::replay::{collect_fixtures, run_fixture};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PerfPoint {
    iterations: usize,
    scenarios_per_iteration: usize,
    avg_ms: f64,
    p95_ms: f64,
    max_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PerfBudgetReport {
    schema_version: u32,
    status: String,
    baseline_avg_ms: f64,
    current_avg_ms: f64,
    avg_regression_percent: f64,
    allowed_regression_percent: f64,
    avg_delta_ms: f64,
    allowed_avg_delta_ms: f64,
    baseline: PerfPoint,
    current: PerfPoint,
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn fixture_dir() -> PathBuf {
    PathBuf::from("crates/omega-tools/fixtures/replay")
}

fn measure(iterations: usize) -> Result<PerfPoint> {
    let all_fixtures = collect_fixtures(&fixture_dir())?;
    if all_fixtures.is_empty() {
        bail!("no replay fixtures found");
    }
    let perf_fixtures: Vec<_> = all_fixtures
        .iter()
        .filter(|(_, fixture)| fixture.tags.iter().any(|tag| tag == "perf_smoke"))
        .collect();
    let fixtures: Vec<_> =
        if perf_fixtures.is_empty() { all_fixtures.iter().collect() } else { perf_fixtures };

    let mut samples_ms = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        for (_, fixture) in &fixtures {
            let result = run_fixture(fixture);
            if !result.passed {
                bail!("fixture failed during perf measurement: {}", result.name);
            }
        }
        samples_ms.push(start.elapsed().as_secs_f64() * 1_000.0);
    }
    samples_ms.sort_by(|a, b| a.total_cmp(b));
    let avg_ms = samples_ms.iter().sum::<f64>() / samples_ms.len() as f64;
    let p95_idx = ((samples_ms.len() as f64 * 0.95).ceil() as usize)
        .saturating_sub(1)
        .min(samples_ms.len().saturating_sub(1));
    let p95_ms = samples_ms[p95_idx];
    let max_ms = *samples_ms.last().unwrap_or(&0.0);
    Ok(PerfPoint { iterations, scenarios_per_iteration: fixtures.len(), avg_ms, p95_ms, max_ms })
}

fn load_baseline_perf() -> Result<PerfPoint> {
    let baseline_path = Path::new("target").join("m5-m4-baseline-freeze.json");
    let raw = fs::read_to_string(&baseline_path)
        .with_context(|| format!("read {}", baseline_path.display()))?;
    let normalized = raw.trim_start_matches('\u{feff}');
    let value: Value = serde_json::from_str(normalized).context("parse baseline freeze json")?;
    let perf = value
        .get("perf_baseline")
        .ok_or_else(|| anyhow::anyhow!("baseline freeze missing perf_baseline section"))?;
    let iterations = perf.get("iterations").and_then(Value::as_u64).unwrap_or(200) as usize;
    let scenarios_per_iteration =
        perf.get("scenarios_per_iteration").and_then(Value::as_u64).unwrap_or(0) as usize;
    let avg_ms = perf.get("avg_ms").and_then(Value::as_f64).unwrap_or(0.0);
    let p95_ms = perf.get("p95_ms").and_then(Value::as_f64).unwrap_or(0.0);
    let max_ms = perf.get("max_ms").and_then(Value::as_f64).unwrap_or(0.0);
    Ok(PerfPoint { iterations, scenarios_per_iteration, avg_ms, p95_ms, max_ms })
}

fn markdown(report: &PerfBudgetReport) -> String {
    let mut out = Vec::new();
    out.push("# M5 Perf Budget Report".to_string());
    out.push(String::new());
    out.push(format!("- Status: {}", report.status));
    out.push(format!("- Allowed regression: <= {:.3}%", report.allowed_regression_percent));
    out.push(format!("- Avg regression: {:.3}%", report.avg_regression_percent));
    out.push(format!("- Allowed absolute delta: <= {:.6} ms", report.allowed_avg_delta_ms));
    out.push(format!("- Avg absolute delta: {:.6} ms", report.avg_delta_ms));
    out.push(String::new());
    out.push("| Metric | Baseline | Current |".to_string());
    out.push("|---|---:|---:|".to_string());
    out.push(format!("| Avg ms | {:.6} | {:.6} |", report.baseline.avg_ms, report.current.avg_ms));
    out.push(format!("| P95 ms | {:.6} | {:.6} |", report.baseline.p95_ms, report.current.p95_ms));
    out.push(format!("| Max ms | {:.6} | {:.6} |", report.baseline.max_ms, report.current.max_ms));
    out.push(format!(
        "| Iterations | {} | {} |",
        report.baseline.iterations, report.current.iterations
    ));
    out.push(format!(
        "| Scenarios/iteration | {} | {} |",
        report.baseline.scenarios_per_iteration, report.current.scenarios_per_iteration
    ));
    out.join("\n")
}

fn main() -> Result<()> {
    let baseline = load_baseline_perf()?;
    if baseline.avg_ms <= 0.0 {
        bail!("baseline avg_ms is invalid: {}", baseline.avg_ms);
    }

    let current = measure(200)?;
    let allowed_regression_percent = 5.0f64;
    let allowed_avg_delta_ms = 0.05f64;
    let avg_regression_percent = ((current.avg_ms - baseline.avg_ms) / baseline.avg_ms) * 100.0;
    let avg_delta_ms = current.avg_ms - baseline.avg_ms;
    let status = if avg_regression_percent <= allowed_regression_percent
        || avg_delta_ms <= allowed_avg_delta_ms
    {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };

    let report = PerfBudgetReport {
        schema_version: 1,
        status: status.clone(),
        baseline_avg_ms: baseline.avg_ms,
        current_avg_ms: current.avg_ms,
        avg_regression_percent,
        allowed_regression_percent,
        avg_delta_ms,
        allowed_avg_delta_ms,
        baseline,
        current,
    };

    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }
    let json_path = target.join("m5-perf-budget-report.json");
    let md_path = target.join("m5-perf-budget-report.md");
    fs::write(&json_path, serde_json::to_string_pretty(&report).context("serialize perf report")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "m5 perf budget: baseline_avg_ms={:.6}, current_avg_ms={:.6}, regression={:.3}%, delta_ms={:.6}, status={}",
        report.baseline_avg_ms,
        report.current_avg_ms,
        report.avg_regression_percent,
        report.avg_delta_ms,
        report.status
    );

    if report.status != "PASS" {
        bail!("perf regression exceeds allowed budget");
    }
    Ok(())
}
