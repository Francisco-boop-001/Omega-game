use anyhow::{Context, Result, bail};
use omega_tools::audit_contract::{contracts_dir, copy_if_exists, ensure_cert_dirs};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

fn run_base_extractor() -> Result<()> {
    let status = Command::new("cargo")
        .args(["run", "-p", "omega-tools", "--bin", "rust_mechanics_extract"])
        .status()
        .context("run rust_mechanics_extract")?;
    if !status.success() {
        bail!("rust_mechanics_extract failed");
    }
    Ok(())
}

fn main() -> Result<()> {
    ensure_cert_dirs()?;
    run_base_extractor()?;

    let src_json = Path::new("target/rust-mechanics-ledger.json");
    let src_md = Path::new("target/rust-mechanics-ledger.md");
    let dst_json = contracts_dir().join("rust-mechanics-ledger.json");
    let dst_md = contracts_dir().join("rust-mechanics-ledger.md");
    if !copy_if_exists(src_json, &dst_json)? {
        bail!("missing source artifact {}", src_json.display());
    }
    let _ = copy_if_exists(src_md, &dst_md)?;

    let raw =
        fs::read_to_string(&dst_json).with_context(|| format!("read {}", dst_json.display()))?;
    let value: Value =
        serde_json::from_str(&raw).with_context(|| format!("decode {}", dst_json.display()))?;
    let total = value["total"].as_u64().unwrap_or(0);
    if total == 0 {
        bail!("rust exhaustive ledger is empty");
    }
    println!("rust mechanics exhaustive extract: total={} out={}", total, dst_json.display());
    Ok(())
}
