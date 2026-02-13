use anyhow::{Context, Result, bail};
use omega_tools::audit_contract::{CoverageResult, coverage_dir, ensure_cert_dirs, write_json};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
struct LegacyBranch {
    service: String,
    kind: String,
}

#[derive(Debug, Deserialize)]
struct LegacyContract {
    branches: Vec<LegacyBranch>,
}

#[derive(Debug, Clone, Deserialize)]
struct RustBranch {
    service: String,
    kind: String,
}

#[derive(Debug, Deserialize)]
struct RustContract {
    branches: Vec<RustBranch>,
}

#[derive(Debug, Deserialize)]
struct DiffReport {
    pass: bool,
}

#[derive(Debug, serde::Serialize)]
struct BranchCoverageReport {
    generated_at_utc: String,
    pass: bool,
    coverage: CoverageResult,
    per_service: Vec<ServiceCoverageRow>,
}

#[derive(Debug, serde::Serialize)]
struct ServiceCoverageRow {
    service: String,
    legacy_total: usize,
    rust_total: usize,
    legacy_gates: usize,
    rust_gates: usize,
    legacy_effects: usize,
    rust_effects: usize,
    covered: bool,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn count_legacy(rows: &[LegacyBranch], kind: &str) -> usize {
    rows.iter().filter(|row| row.kind == kind).count()
}

fn count_rust(rows: &[RustBranch], kind: &str) -> usize {
    rows.iter().filter(|row| row.kind == kind).count()
}

fn main() -> Result<()> {
    ensure_cert_dirs()?;

    let legacy_path = Path::new("target/legacy-site-branch-contract.json");
    let rust_path = Path::new("target/rust-site-branch-contract.json");
    if !legacy_path.exists() || !rust_path.exists() {
        bail!(
            "missing branch contracts; run legacy_site_branch_extract and rust_site_branch_extract first"
        );
    }

    let legacy_raw = fs::read_to_string(legacy_path)
        .with_context(|| format!("read {}", legacy_path.display()))?;
    let rust_raw =
        fs::read_to_string(rust_path).with_context(|| format!("read {}", rust_path.display()))?;
    let legacy: LegacyContract =
        serde_json::from_str(&legacy_raw).context("decode legacy-site-branch-contract")?;
    let rust: RustContract =
        serde_json::from_str(&rust_raw).context("decode rust-site-branch-contract")?;

    let mut services = BTreeSet::<String>::new();
    for row in &legacy.branches {
        services.insert(row.service.clone());
    }
    let mut legacy_by = BTreeMap::<String, Vec<LegacyBranch>>::new();
    for row in legacy.branches {
        legacy_by.entry(row.service.clone()).or_default().push(row);
    }
    let mut rust_by = BTreeMap::<String, Vec<RustBranch>>::new();
    for row in rust.branches {
        rust_by.entry(row.service.clone()).or_default().push(row);
    }

    let mut rows = Vec::new();
    let mut missing_ids = Vec::new();
    for service in services {
        let legacy_rows = legacy_by.get(&service).cloned().unwrap_or_default();
        let rust_rows = rust_by.get(&service).cloned().unwrap_or_default();
        let legacy_total = legacy_rows.len();
        let rust_total = rust_rows.len();
        let legacy_gates = count_legacy(&legacy_rows, "gate");
        let rust_gates = count_rust(&rust_rows, "gate");
        let legacy_effects = count_legacy(&legacy_rows, "effect");
        let rust_effects = count_rust(&rust_rows, "effect");
        let legacy_entry = count_legacy(&legacy_rows, "entry");
        let rust_entry = count_rust(&rust_rows, "entry");

        let covered = legacy_total > 0
            && rust_total > 0
            && rust_entry >= legacy_entry
            && rust_gates >= legacy_gates
            && rust_effects >= legacy_effects;
        if !covered {
            missing_ids.push(service.clone());
        }
        rows.push(ServiceCoverageRow {
            service,
            legacy_total,
            rust_total,
            legacy_gates,
            rust_gates,
            legacy_effects,
            rust_effects,
            covered,
        });
    }

    rows.sort_by(|a, b| a.service.cmp(&b.service));
    let total = rows.len();
    let covered = rows.iter().filter(|row| row.covered).count();
    let missing = total.saturating_sub(covered);
    let diff_report: DiffReport = serde_json::from_str(
        &fs::read_to_string("target/site-branch-diff.json")
            .context("read target/site-branch-diff.json")?,
    )
    .context("decode target/site-branch-diff.json")?;
    let coverage = CoverageResult { total, covered, missing, pass: missing == 0, missing_ids };
    let report = BranchCoverageReport {
        generated_at_utc: now_utc_unix(),
        pass: coverage.pass && diff_report.pass,
        coverage,
        per_service: rows,
    };

    let out_path = coverage_dir().join("branch-coverage.json");
    write_json(&out_path, &report)?;
    println!(
        "branch coverage certify: total={} covered={} missing={} pass={}",
        report.coverage.total, report.coverage.covered, report.coverage.missing, report.pass
    );
    if !report.pass {
        bail!("branch coverage certification failed");
    }
    Ok(())
}
