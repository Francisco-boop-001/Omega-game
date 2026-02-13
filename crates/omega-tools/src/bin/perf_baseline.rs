use std::fs;
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use omega_tools::replay::{collect_fixtures, run_fixture};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerfBudgets {
    max_avg_ms: f64,
    max_p95_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerfReport {
    iterations: usize,
    scenarios_per_iteration: usize,
    avg_ms: f64,
    p95_ms: f64,
    max_ms: f64,
}

fn main() -> Result<()> {
    let mut iterations = 50usize;
    let mut enforce_budget = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--iterations" => {
                let value =
                    args.next().ok_or_else(|| anyhow::anyhow!("--iterations requires a value"))?;
                iterations = value.parse().context("invalid --iterations value")?;
            }
            "--check" => {
                enforce_budget = true;
            }
            _ => bail!("unknown argument: {arg}"),
        }
    }

    let fixture_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures").join("replay");
    let all_fixtures = collect_fixtures(&fixture_dir)?;
    if all_fixtures.is_empty() {
        bail!("no replay fixtures found under {}", fixture_dir.display());
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
                bail!("replay fixture failed during perf run: {}", result.name);
            }
        }
        samples_ms.push(start.elapsed().as_secs_f64() * 1_000.0);
    }

    samples_ms.sort_by(|a, b| a.total_cmp(b));
    let avg_ms = samples_ms.iter().sum::<f64>() / samples_ms.len() as f64;
    let p95_idx = ((samples_ms.len() as f64 * 0.95).ceil() as usize)
        .saturating_sub(1)
        .min(samples_ms.len() - 1);
    let p95_ms = samples_ms[p95_idx];
    let max_ms = *samples_ms.last().unwrap_or(&0.0);

    let report =
        PerfReport { iterations, scenarios_per_iteration: fixtures.len(), avg_ms, p95_ms, max_ms };
    println!("{}", serde_json::to_string_pretty(&report)?);

    if enforce_budget {
        let budget_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("docs")
            .join("quality")
            .join("perf-budgets.json");
        let raw = fs::read_to_string(&budget_path)
            .with_context(|| format!("reading {}", budget_path.display()))?;
        let budget: PerfBudgets = serde_json::from_str(&raw)
            .with_context(|| format!("invalid budget file {}", budget_path.display()))?;

        if report.avg_ms > budget.max_avg_ms {
            bail!(
                "perf budget exceeded: avg_ms {:.3} > max_avg_ms {:.3}",
                report.avg_ms,
                budget.max_avg_ms
            );
        }
        if report.p95_ms > budget.max_p95_ms {
            bail!(
                "perf budget exceeded: p95_ms {:.3} > max_p95_ms {:.3}",
                report.p95_ms,
                budget.max_p95_ms
            );
        }
    }

    Ok(())
}
