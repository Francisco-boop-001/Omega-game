use anyhow::{Context, Result, bail};
use omega_tools::audit_contract::{
    CertDefect, CertDefectBoard, certification_root, ensure_cert_dirs, now_utc_unix, read_json,
    write_json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct ComponentCheck {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Serialize)]
struct ParityCertifyReport {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    components: Vec<ComponentCheck>,
}

#[derive(Debug, Serialize)]
struct BaselineArtifact {
    path: String,
    exists: bool,
    size_bytes: u64,
}

#[derive(Debug, Serialize)]
struct CertificationBaseline {
    generated_at_utc: String,
    total: usize,
    present: usize,
    missing: usize,
    pass: bool,
    artifacts: Vec<BaselineArtifact>,
}

#[derive(Debug, Deserialize)]
struct GenericPass {
    pass: bool,
    total: Option<usize>,
    passed: Option<usize>,
    failed: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct MappingReport {
    pass: bool,
    unresolved: usize,
    unknown: usize,
}

#[derive(Debug, Deserialize)]
struct BranchCoverageReport {
    pass: bool,
    coverage: Coverage,
}

#[derive(Debug, Deserialize)]
struct Coverage {
    missing: usize,
}

fn validate_no_collision() -> Result<(bool, String)> {
    let path = Path::new("target/rust-site-branch-contract.json");
    if !path.exists() {
        return Ok((false, "missing target/rust-site-branch-contract.json".to_string()));
    }
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let json: Value =
        serde_json::from_str(&raw).with_context(|| format!("decode {}", path.display()))?;
    let has_branches = json.get("branches").is_some();
    let has_checks = json.get("checks").is_some();
    let pass = has_branches && !has_checks;
    Ok((pass, format!("has_branches={} has_checks={}", has_branches, has_checks)))
}

fn build_baseline(required: &[&str]) -> Result<CertificationBaseline> {
    let artifacts = required
        .iter()
        .map(|path| {
            let p = Path::new(path);
            if p.exists() {
                let size = fs::metadata(p).map(|m| m.len()).unwrap_or(0);
                BaselineArtifact { path: (*path).to_string(), exists: true, size_bytes: size }
            } else {
                BaselineArtifact { path: (*path).to_string(), exists: false, size_bytes: 0 }
            }
        })
        .collect::<Vec<_>>();
    let total = artifacts.len();
    let present = artifacts.iter().filter(|a| a.exists).count();
    let missing = total.saturating_sub(present);
    Ok(CertificationBaseline {
        generated_at_utc: now_utc_unix(),
        total,
        present,
        missing,
        pass: missing == 0,
        artifacts,
    })
}

fn main() -> Result<()> {
    ensure_cert_dirs()?;

    let required = [
        "target/certification/contracts/legacy-mechanics-ledger.json",
        "target/certification/contracts/rust-mechanics-ledger.json",
        "target/certification/contracts/mechanics_mapping.json",
        "target/certification/diff/legacy-headless-replay.json",
        "target/certification/diff/rust-headless-replay.json",
        "target/certification/diff/mechanics-differential.json",
        "target/certification/diff/service-branch-differential.json",
        "target/certification/coverage/branch-coverage.json",
        "target/certification/smoke/blackbox-adversarial.json",
    ];

    let baseline = build_baseline(&required)?;
    write_json(certification_root().join("baseline.json"), &baseline)?;

    let mapping: MappingReport = read_json("target/certification/contracts/mechanics_mapping.json")
        .context("read mapping")?;
    let mechanics_diff: GenericPass =
        read_json("target/certification/diff/mechanics-differential.json")
            .context("read mechanics differential")?;
    let service_diff: GenericPass =
        read_json("target/certification/diff/service-branch-differential.json")
            .context("read service differential")?;
    let coverage: BranchCoverageReport =
        read_json("target/certification/coverage/branch-coverage.json").context("read coverage")?;
    let blackbox: GenericPass = read_json("target/certification/smoke/blackbox-adversarial.json")
        .context("read blackbox")?;
    let (collision_pass, collision_details) = validate_no_collision()?;

    let components = vec![
        ComponentCheck {
            id: "baseline_artifacts".to_string(),
            pass: baseline.pass,
            details: format!(
                "present={}/{} missing={}",
                baseline.present, baseline.total, baseline.missing
            ),
        },
        ComponentCheck {
            id: "mapping".to_string(),
            pass: mapping.pass && mapping.unknown == 0 && mapping.unresolved == 0,
            details: format!(
                "pass={} unknown={} unresolved={}",
                mapping.pass, mapping.unknown, mapping.unresolved
            ),
        },
        ComponentCheck {
            id: "mechanics_differential".to_string(),
            pass: mechanics_diff.pass,
            details: format!(
                "pass={} passed={}/{} failed={}",
                mechanics_diff.pass,
                mechanics_diff.passed.unwrap_or(0),
                mechanics_diff.total.unwrap_or(0),
                mechanics_diff.failed.unwrap_or(0)
            ),
        },
        ComponentCheck {
            id: "service_branch_differential".to_string(),
            pass: service_diff.pass,
            details: format!(
                "pass={} passed={}/{} failed={}",
                service_diff.pass,
                service_diff.passed.unwrap_or(0),
                service_diff.total.unwrap_or(0),
                service_diff.failed.unwrap_or(0)
            ),
        },
        ComponentCheck {
            id: "branch_coverage".to_string(),
            pass: coverage.pass && coverage.coverage.missing == 0,
            details: format!("pass={} missing={}", coverage.pass, coverage.coverage.missing),
        },
        ComponentCheck {
            id: "adversarial_blackbox".to_string(),
            pass: blackbox.pass,
            details: format!(
                "pass={} passed={}/{} failed={}",
                blackbox.pass,
                blackbox.passed.unwrap_or(0),
                blackbox.total.unwrap_or(0),
                blackbox.failed.unwrap_or(0)
            ),
        },
        ComponentCheck {
            id: "artifact_collision_guard".to_string(),
            pass: collision_pass,
            details: collision_details,
        },
    ];

    let total = components.len();
    let passed = components.iter().filter(|component| component.pass).count();
    let failed = total.saturating_sub(passed);
    let report = ParityCertifyReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        components,
    };
    write_json(certification_root().join("parity-certify.json"), &report)?;

    let defects = report
        .components
        .iter()
        .filter(|component| !component.pass)
        .map(|component| CertDefect {
            id: format!("CERT-{}", component.id),
            severity: "high".to_string(),
            area: "certification".to_string(),
            title: format!("Certification component failed: {}", component.id),
            details: component.details.clone(),
        })
        .collect::<Vec<_>>();
    let board = CertDefectBoard {
        generated_at_utc: now_utc_unix(),
        total: defects.len(),
        open: defects.len(),
        defects,
    };
    write_json(certification_root().join("defect-board.json"), &board)?;

    println!(
        "parity certify: total={} passed={} failed={} pass={}",
        report.total, report.passed, report.failed, report.pass
    );
    if !report.pass {
        bail!("parity certification failed");
    }
    Ok(())
}
