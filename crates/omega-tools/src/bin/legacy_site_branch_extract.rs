use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
struct LegacyFunctionContract {
    name: String,
    file: String,
    line_start: usize,
    line_end: usize,
    #[serde(default)]
    gates: Vec<String>,
    #[serde(default)]
    rewards: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct LegacyGuildSiteContract {
    source_snapshot: String,
    #[serde(default)]
    functions: Vec<LegacyFunctionContract>,
}

#[derive(Debug, Clone, Serialize)]
struct LegacyBranchRow {
    branch_id: String,
    service: String,
    source_file: String,
    line_anchor: String,
    kind: String,
    predicate: String,
}

#[derive(Debug, Clone, Serialize)]
struct LegacySiteBranchContract {
    source_snapshot: String,
    total_branches: usize,
    services: usize,
    branches: Vec<LegacyBranchRow>,
    pass: bool,
}

fn slug(name: &str) -> String {
    name.trim().to_ascii_lowercase().replace(' ', "_")
}

fn service_key_from_legacy_fn(name: &str) -> Option<&'static str> {
    match name {
        "l_merc_guild" => Some("merc"),
        "l_thieves_guild" => Some("thieves"),
        "l_college" => Some("college"),
        "l_sorcerors" => Some("sorcerors"),
        "l_order" => Some("order"),
        "l_castle" => Some("castle"),
        "l_arena" => Some("arena"),
        "l_altar" => Some("temple"),
        "l_monastery" => Some("monastery"),
        _ => None,
    }
}

fn markdown(contract: &LegacySiteBranchContract) -> String {
    let mut out = Vec::new();
    out.push("# Legacy Site Branch Contract".to_string());
    out.push(String::new());
    out.push(format!("- source_snapshot: `{}`", contract.source_snapshot));
    out.push(format!("- services: `{}`", contract.services));
    out.push(format!("- total_branches: `{}`", contract.total_branches));
    out.push(String::new());
    out.push("| Branch ID | Service | Kind | Anchor | Predicate |".to_string());
    out.push("|---|---|---|---|---|".to_string());
    for row in &contract.branches {
        out.push(format!(
            "| {} | {} | {} | {} | {} |",
            row.branch_id,
            row.service,
            row.kind,
            row.line_anchor,
            row.predicate.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let legacy_contract_path = Path::new("target/legacy-guild-site-contract.json");
    if !legacy_contract_path.exists() {
        bail!(
            "{} missing; run `cargo run -p omega-tools --bin legacy_guild_site_contract` first",
            legacy_contract_path.display()
        );
    }

    let raw = fs::read_to_string(legacy_contract_path)
        .with_context(|| format!("read {}", legacy_contract_path.display()))?;
    let legacy: LegacyGuildSiteContract = serde_json::from_str(&raw)
        .with_context(|| format!("decode {}", legacy_contract_path.display()))?;

    let mut branches = Vec::new();
    for function in &legacy.functions {
        let Some(service) = service_key_from_legacy_fn(&function.name) else {
            continue;
        };
        let base_anchor =
            format!("{}:{}-{}", function.file, function.line_start, function.line_end);
        branches.push(LegacyBranchRow {
            branch_id: format!("{service}/entry"),
            service: service.to_string(),
            source_file: function.file.clone(),
            line_anchor: base_anchor.clone(),
            kind: "entry".to_string(),
            predicate: "always".to_string(),
        });

        for (idx, gate) in function.gates.iter().enumerate() {
            branches.push(LegacyBranchRow {
                branch_id: format!("{service}/gate/{}", idx + 1),
                service: service.to_string(),
                source_file: function.file.clone(),
                line_anchor: base_anchor.clone(),
                kind: "gate".to_string(),
                predicate: slug(gate),
            });
        }

        for (idx, reward) in function.rewards.iter().enumerate() {
            branches.push(LegacyBranchRow {
                branch_id: format!("{service}/reward/{}", idx + 1),
                service: service.to_string(),
                source_file: function.file.clone(),
                line_anchor: base_anchor.clone(),
                kind: "effect".to_string(),
                predicate: slug(reward),
            });
        }
    }

    let mut services = branches.iter().map(|row| row.service.clone()).collect::<Vec<_>>();
    services.sort();
    services.dedup();

    let report = LegacySiteBranchContract {
        source_snapshot: legacy.source_snapshot,
        total_branches: branches.len(),
        services: services.len(),
        pass: !branches.is_empty(),
        branches,
    };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }
    let json_path = "target/legacy-site-branch-contract.json";
    let md_path = "target/legacy-site-branch-contract.md";
    fs::write(
        json_path,
        serde_json::to_string_pretty(&report).context("serialize legacy site branch contract")?,
    )
    .with_context(|| format!("write {json_path}"))?;
    fs::write(md_path, markdown(&report)).with_context(|| format!("write {md_path}"))?;

    println!(
        "legacy site branch extract: services={} branches={}",
        report.services, report.total_branches
    );
    if !report.pass {
        bail!("legacy site branch contract is empty");
    }
    Ok(())
}
