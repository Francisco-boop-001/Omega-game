use anyhow::{Context, Result, bail};
use omega_tools::replay::{collect_fixtures, run_fixture_trace};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DeterminismDivergence {
    scenario: String,
    run_index: usize,
    baseline_hash: String,
    observed_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FamilyDeterminism {
    family: String,
    fixture_count: usize,
    runs_per_fixture: usize,
    total_runs: usize,
    divergent_runs: usize,
    passed: bool,
    divergences: Vec<DeterminismDivergence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DeterminismReport {
    required_runs_per_fixture: usize,
    total_families: usize,
    total_fixtures: usize,
    total_runs: usize,
    divergent_runs: usize,
    passed: bool,
    families: Vec<FamilyDeterminism>,
}

fn parse_args() -> Result<(PathBuf, usize)> {
    let mut fixture_dir = PathBuf::from("crates/omega-tools/fixtures/replay");
    let mut runs_per_fixture = 20usize;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--fixture-dir" => {
                let value =
                    args.next().ok_or_else(|| anyhow::anyhow!("--fixture-dir requires a value"))?;
                fixture_dir = PathBuf::from(value);
            }
            "--runs-per-fixture" => {
                let value = args
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("--runs-per-fixture requires a value"))?;
                runs_per_fixture =
                    value.parse::<usize>().context("invalid --runs-per-fixture value")?;
            }
            other => bail!("unknown argument: {other}"),
        }
    }

    if runs_per_fixture < 20 {
        bail!("runs-per-fixture must be >= 20 to satisfy Milestone 4 determinism gate");
    }

    Ok((fixture_dir, runs_per_fixture))
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn hash_trace_json(raw: &[u8]) -> u64 {
    // FNV-1a 64-bit hash for stable, portable hash comparison across repeated runs.
    let mut hash = 0xcbf29ce484222325u64;
    for byte in raw {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn evaluate(fixture_dir: &Path, runs_per_fixture: usize) -> Result<DeterminismReport> {
    let fixtures = collect_fixtures(fixture_dir)?;
    if fixtures.is_empty() {
        bail!("no replay fixtures found under {}", fixture_dir.display());
    }

    let mut by_family: BTreeMap<String, Vec<_>> = BTreeMap::new();
    for (_, fixture) in fixtures {
        by_family.entry(fixture.family.clone()).or_default().push(fixture);
    }

    let mut families = Vec::new();
    let mut total_runs = 0usize;
    let mut divergent_runs = 0usize;
    let mut total_fixtures = 0usize;

    for (family, fixtures) in by_family {
        total_fixtures += fixtures.len();
        let mut divergences = Vec::new();
        let mut family_runs = 0usize;

        for fixture in fixtures {
            let baseline_trace = run_fixture_trace(&fixture);
            let baseline_raw =
                serde_json::to_vec(&baseline_trace).context("serialize baseline trace")?;
            let baseline_hash = hash_trace_json(&baseline_raw);
            let baseline_hash_hex = format!("{baseline_hash:016x}");

            for run_index in 1..=runs_per_fixture {
                family_runs += 1;
                let trace = run_fixture_trace(&fixture);
                let raw = serde_json::to_vec(&trace).context("serialize trace")?;
                let observed_hash = hash_trace_json(&raw);
                if observed_hash != baseline_hash {
                    divergences.push(DeterminismDivergence {
                        scenario: fixture.name.clone(),
                        run_index,
                        baseline_hash: baseline_hash_hex.clone(),
                        observed_hash: format!("{observed_hash:016x}"),
                    });
                }
            }
        }

        let family_divergent = divergences.len();
        total_runs += family_runs;
        divergent_runs += family_divergent;
        families.push(FamilyDeterminism {
            family,
            fixture_count: family_runs / runs_per_fixture,
            runs_per_fixture,
            total_runs: family_runs,
            divergent_runs: family_divergent,
            passed: family_divergent == 0,
            divergences,
        });
    }

    let passed = divergent_runs == 0 && !families.is_empty();
    Ok(DeterminismReport {
        required_runs_per_fixture: runs_per_fixture,
        total_families: families.len(),
        total_fixtures,
        total_runs,
        divergent_runs,
        passed,
        families,
    })
}

fn markdown(report: &DeterminismReport) -> String {
    let mut out = Vec::new();
    out.push("# WS-D Determinism Report".to_string());
    out.push(String::new());
    out.push(format!("- Families: {}", report.total_families));
    out.push(format!("- Fixtures: {}", report.total_fixtures));
    out.push(format!("- Runs per fixture: {}", report.required_runs_per_fixture));
    out.push(format!("- Total runs: {}", report.total_runs));
    out.push(format!("- Divergent runs: {}", report.divergent_runs));
    out.push(format!("- Status: {}", if report.passed { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push(
        "| Family | Fixtures | Runs/Fixture | Total Runs | Divergent Runs | Status |".to_string(),
    );
    out.push("|---|---:|---:|---:|---:|---|".to_string());
    for family in &report.families {
        out.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            family.family,
            family.fixture_count,
            family.runs_per_fixture,
            family.total_runs,
            family.divergent_runs,
            if family.passed { "PASS" } else { "FAIL" }
        ));
    }
    out.push(String::new());
    for family in &report.families {
        if family.divergences.is_empty() {
            continue;
        }
        out.push(format!("## Divergences: {}", family.family));
        for item in family.divergences.iter().take(20) {
            out.push(format!(
                "- {} run#{} baseline={} observed={}",
                item.scenario, item.run_index, item.baseline_hash, item.observed_hash
            ));
        }
        if family.divergences.len() > 20 {
            out.push(format!("- ... {} more", family.divergences.len() - 20));
        }
        out.push(String::new());
    }
    out.join("\n")
}

fn write_outputs(report: &DeterminismReport) -> Result<()> {
    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }
    let json_path = target.join("ws-d-determinism-report.json");
    let md_path = target.join("ws-d-determinism-report.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(report).context("serialize determinism report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(report))
        .with_context(|| format!("write {}", md_path.display()))?;
    Ok(())
}

fn main() -> Result<()> {
    let (fixture_dir, runs_per_fixture) = parse_args()?;
    let report = evaluate(&fixture_dir, runs_per_fixture)?;
    write_outputs(&report)?;

    println!(
        "determinism: families={}, fixtures={}, runs={}, divergent={}, status={}",
        report.total_families,
        report.total_fixtures,
        report.total_runs,
        report.divergent_runs,
        if report.passed { "PASS" } else { "FAIL" }
    );

    if !report.passed {
        bail!("determinism check failed");
    }
    Ok(())
}
