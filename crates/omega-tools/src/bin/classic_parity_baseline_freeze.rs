use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ArtifactSnapshot {
    path: String,
    present: bool,
    size_bytes: u64,
    hash_fnv1a64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct BaselineFreeze {
    generated_at_utc: String,
    status: String,
    total: usize,
    present: usize,
    missing: usize,
    artifacts: Vec<ArtifactSnapshot>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn markdown(freeze: &BaselineFreeze) -> String {
    let mut out = Vec::new();
    out.push("# Classic Parity Baseline Freeze".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", freeze.generated_at_utc));
    out.push(format!("- Status: {}", freeze.status));
    out.push(format!("- Present: {}/{}", freeze.present, freeze.total));
    out.push(format!("- Missing: {}", freeze.missing));
    out.push(String::new());
    out.push("| Artifact | Status | Size (bytes) | FNV-1a 64 |".to_string());
    out.push("|---|---|---:|---|".to_string());
    for artifact in &freeze.artifacts {
        out.push(format!(
            "| {} | {} | {} | {} |",
            artifact.path,
            if artifact.present { "PRESENT" } else { "MISSING" },
            artifact.size_bytes,
            artifact.hash_fnv1a64
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let required = vec![
        "target/classic-parity-manifest.json",
        "target/classic-command-parity-matrix.json",
        "target/classic-content-cardinality-matrix.json",
        "target/classic-gap-ledger.json",
        "target/classic-parity-regression-dashboard.json",
        "target/classic-parity-regression-dashboard.md",
        "target/classic-burnin-window.json",
        "target/classic-burnin-window.md",
        "target/classic-site-service-parity-matrix.json",
        "target/classic-site-service-parity-matrix.md",
        "target/classic-progression-branch-matrix.json",
        "target/classic-progression-branch-matrix.md",
        "target/classic-state-integrity.json",
        "target/classic-state-integrity.md",
        "target/classic-core-model-parity.json",
        "target/classic-core-model-parity.md",
        "target/classic-combat-encounter-parity.json",
        "target/classic-combat-encounter-parity.md",
        "target/classic-magic-item-parity.json",
        "target/classic-magic-item-parity.md",
        "target/classic-compatibility-matrix.json",
        "target/classic-compatibility-matrix.md",
        "target/classic-frontend-workflow-parity.json",
        "target/classic-frontend-workflow-parity.md",
        "docs/migration/CLASSIC_OMEGA_PARITY_SCORECARD.md",
        "docs/migration/CLASSIC_OMEGA_PARITY_CLOSURE_REVIEW.md",
    ];

    let mut artifacts = Vec::new();
    for path in required {
        let p = Path::new(path);
        if p.exists() {
            let bytes = fs::read(p).with_context(|| format!("read {}", p.display()))?;
            artifacts.push(ArtifactSnapshot {
                path: path.to_string(),
                present: true,
                size_bytes: bytes.len() as u64,
                hash_fnv1a64: format!("{:016x}", fnv1a64(&bytes)),
            });
        } else {
            artifacts.push(ArtifactSnapshot {
                path: path.to_string(),
                present: false,
                size_bytes: 0,
                hash_fnv1a64: "missing".to_string(),
            });
        }
    }

    let total = artifacts.len();
    let present = artifacts.iter().filter(|artifact| artifact.present).count();
    let missing = total.saturating_sub(present);
    let status = if missing == 0 { "PASS" } else { "FAIL" }.to_string();
    let freeze = BaselineFreeze {
        generated_at_utc: now_utc_unix(),
        status: status.clone(),
        total,
        present,
        missing,
        artifacts,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("classic-parity-baseline-freeze.json");
    let md_path = target.join("classic-parity-baseline-freeze.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&freeze).context("serialize baseline freeze")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&freeze))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "classic parity baseline freeze: total={}, present={}, missing={}, status={}",
        freeze.total, freeze.present, freeze.missing, freeze.status
    );
    if freeze.status != "PASS" {
        bail!("classic parity baseline freeze has missing artifacts");
    }
    Ok(())
}
