use anyhow::{Context, Result};
use omega_core::legacy_projectile_contract;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
struct LegacyProjectileContractReport {
    source_root: String,
    ob_longbow: i32,
    ob_crossbow: i32,
    ob_arrow: i32,
    ob_bolt: i32,
    i_arrow: i32,
    i_bolt: i32,
    i_scythe: i32,
    loaded: i32,
    unloaded: i32,
    hit_rule: String,
    statmod_rule: String,
    references: Vec<String>,
}

fn markdown(report: &LegacyProjectileContractReport) -> String {
    let mut out = Vec::new();
    out.push("# Legacy Projectile Contract".to_string());
    out.push(String::new());
    out.push(format!("- Source root: `{}`", report.source_root));
    out.push(String::new());
    out.push("## Constants".to_string());
    out.push(String::new());
    out.push(format!("- `OB_LONGBOW`: {}", report.ob_longbow));
    out.push(format!("- `OB_CROSSBOW`: {}", report.ob_crossbow));
    out.push(format!("- `OB_ARROW`: {}", report.ob_arrow));
    out.push(format!("- `OB_BOLT`: {}", report.ob_bolt));
    out.push(format!("- `I_ARROW`: {}", report.i_arrow));
    out.push(format!("- `I_BOLT`: {}", report.i_bolt));
    out.push(format!("- `I_SCYTHE`: {}", report.i_scythe));
    out.push(format!("- `LOADED`: {}", report.loaded));
    out.push(format!("- `UNLOADED`: {}", report.unloaded));
    out.push(String::new());
    out.push("## Rules".to_string());
    out.push(String::new());
    out.push(format!("- Hit rule: {}", report.hit_rule));
    out.push(format!("- Stat modifier rule: {}", report.statmod_rule));
    out.push(String::new());
    out.push("## Legacy References".to_string());
    out.push(String::new());
    for reference in &report.references {
        out.push(format!("- `{}`", reference));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let contract = legacy_projectile_contract();
    let report = LegacyProjectileContractReport {
        source_root: "archive/legacy-c-runtime/2026-02-06".to_string(),
        ob_longbow: contract.ob_longbow,
        ob_crossbow: contract.ob_crossbow,
        ob_arrow: contract.ob_arrow,
        ob_bolt: contract.ob_bolt,
        i_arrow: contract.i_arrow,
        i_bolt: contract.i_bolt,
        i_scythe: contract.i_scythe,
        loaded: contract.loaded,
        unloaded: contract.unloaded,
        hit_rule: contract.hit_rule.clone(),
        statmod_rule: contract.statmod_rule.clone(),
        references: contract.references.clone(),
    };

    let target_dir = Path::new("target");
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).context("create target directory")?;
    }
    let json_path = target_dir.join("legacy-projectile-contract.json");
    let md_path = target_dir.join("legacy-projectile-contract.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize legacy projectile contract")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!("legacy projectile contract extracted");
    println!("report json: {}", json_path.display());
    println!("report md: {}", md_path.display());
    Ok(())
}
