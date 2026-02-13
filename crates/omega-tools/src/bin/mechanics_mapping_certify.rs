use anyhow::{Context, Result, bail};
use omega_tools::audit_contract::{CanonicalMechanic, contracts_dir, ensure_cert_dirs, write_json};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct LedgerEntry {
    id: String,
    name: String,
    tier: String,
    domain: String,
    legacy_anchor: String,
}

#[derive(Debug, Deserialize)]
struct Ledger {
    total: usize,
    entries: Vec<LedgerEntry>,
}

#[derive(Debug, Deserialize)]
struct MatrixRow {
    mechanic_id: String,
    parity_status: String,
    evidence_static: String,
}

#[derive(Debug, Deserialize)]
struct Matrix {
    pass: bool,
    unknown: usize,
    rows: Vec<MatrixRow>,
}

#[derive(Debug, Serialize)]
struct MappingReport {
    generated_at_utc: String,
    total: usize,
    exact_or_equivalent: usize,
    unresolved: usize,
    unknown: usize,
    pass: bool,
    mechanics: Vec<CanonicalMechanic>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str::<T>(&raw).with_context(|| format!("decode {}", path.display()))
}

fn row_status_normalized(status: &str) -> String {
    match status {
        "Exact" => "exact",
        "Equivalent" => "equivalent",
        "Partial" => "partial",
        "Missing" => "missing",
        "Unknown" => "unknown",
        "ExcludedNonGameplay" => "excluded_non_gameplay",
        other => other,
    }
    .to_string()
}

fn main() -> Result<()> {
    ensure_cert_dirs()?;

    let legacy_path = contracts_dir().join("legacy-mechanics-ledger.json");
    let rust_path = contracts_dir().join("rust-mechanics-ledger.json");
    if !legacy_path.exists() || !rust_path.exists() {
        bail!(
            "missing certification ledgers; run legacy_mechanics_exhaustive_extract and rust_mechanics_exhaustive_extract first"
        );
    }
    let matrix_path = Path::new("target/mechanics-parity-matrix.json");
    if !matrix_path.exists() {
        bail!("missing target/mechanics-parity-matrix.json");
    }

    let legacy: Ledger = read_json(&legacy_path)?;
    let matrix: Matrix = read_json(matrix_path)?;
    if legacy.total == 0 {
        bail!("legacy mechanics ledger is empty");
    }

    let mut row_by_id = BTreeMap::<String, &MatrixRow>::new();
    for row in &matrix.rows {
        row_by_id.insert(row.mechanic_id.clone(), row);
    }

    let mut mechanics = Vec::with_capacity(legacy.entries.len());
    let mut exact_or_equivalent = 0usize;
    let mut unresolved = 0usize;
    for entry in legacy.entries {
        let (status, rust_anchor) = if let Some(row) = row_by_id.get(&entry.id) {
            let status = row_status_normalized(&row.parity_status);
            let rust_anchor = if row.evidence_static.starts_with("legacy-only @") {
                None
            } else {
                Some(row.evidence_static.clone())
            };
            (status, rust_anchor)
        } else {
            ("unknown".to_string(), None)
        };
        if status == "exact" || status == "equivalent" {
            exact_or_equivalent += 1;
        } else if status != "excluded_non_gameplay" {
            unresolved += 1;
        }
        mechanics.push(CanonicalMechanic {
            id: entry.id,
            name: entry.name,
            tier: entry.tier,
            domain: entry.domain,
            legacy_anchor: entry.legacy_anchor,
            rust_anchor,
            status,
        });
    }

    let report = MappingReport {
        generated_at_utc: now_utc_unix(),
        total: mechanics.len(),
        exact_or_equivalent,
        unresolved,
        unknown: matrix.unknown,
        pass: matrix.pass && matrix.unknown == 0 && unresolved == 0,
        mechanics,
    };
    let out_path = contracts_dir().join("mechanics_mapping.json");
    write_json(&out_path, &report)?;

    println!(
        "mechanics mapping certify: total={} exact_or_equivalent={} unresolved={} unknown={} pass={}",
        report.total, report.exact_or_equivalent, report.unresolved, report.unknown, report.pass
    );
    if !report.pass {
        bail!("mechanics mapping certification failed");
    }
    Ok(())
}
