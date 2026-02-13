use anyhow::{Context, Result};
use omega_content::legacy_catalogs;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "../mechanics_shared.rs"]
mod mechanics_shared;

use mechanics_shared::{
    CoverageKind, MechanicEntry, MechanicTier, build_ledger, dedup_entries, ensure_target_dir,
    ledger_to_markdown, parse_rust_command_tokens, parse_rust_symbols, write_json, write_text,
};

fn collect_rust_files(root: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(root).with_context(|| format!("read {}", root.display()))? {
        let entry = entry.with_context(|| format!("read entry under {}", root.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, out)?;
            continue;
        }
        if path.extension().and_then(|v| v.to_str()) == Some("rs") {
            out.push(path);
        }
    }
    Ok(())
}

fn roots() -> Vec<PathBuf> {
    vec![
        PathBuf::from("crates/omega-core/src"),
        PathBuf::from("crates/omega-content/src"),
        PathBuf::from("crates/omega-save/src"),
        PathBuf::from("crates/omega-tui/src"),
        PathBuf::from("crates/omega-bevy/src"),
        PathBuf::from("crates/omega-tools/src"),
    ]
}

fn catalog_entries() -> Vec<MechanicEntry> {
    let catalogs = legacy_catalogs();
    vec![
        MechanicEntry {
            id: "rust.catalog.spells".to_string(),
            name: format!("spells={}", catalogs.spells.len()),
            tier: MechanicTier::Main,
            domain: "magic_and_spells".to_string(),
            legacy_anchor: "omega_content::legacy_catalogs".to_string(),
            trigger: "catalog load".to_string(),
            preconditions: "content bootstrap".to_string(),
            state_delta: "none".to_string(),
            outcomes: "spell catalog available".to_string(),
            side_effects: "spell data drives runtime".to_string(),
            coverage_kind: CoverageKind::Data,
        },
        MechanicEntry {
            id: "rust.catalog.items".to_string(),
            name: format!(
                "items_scrolls={} potions={} weapons={} artifacts={}",
                catalogs.items.scrolls.len(),
                catalogs.items.potions.len(),
                catalogs.items.weapons.len(),
                catalogs.items.artifacts.len()
            ),
            tier: MechanicTier::Main,
            domain: "items_and_equipment".to_string(),
            legacy_anchor: "omega_content::legacy_catalogs".to_string(),
            trigger: "catalog load".to_string(),
            preconditions: "content bootstrap".to_string(),
            state_delta: "none".to_string(),
            outcomes: "item catalog available".to_string(),
            side_effects: "item data drives runtime".to_string(),
            coverage_kind: CoverageKind::Data,
        },
        MechanicEntry {
            id: "rust.catalog.monsters".to_string(),
            name: format!("monsters={}", catalogs.monsters.len()),
            tier: MechanicTier::Secondary,
            domain: "monster_ai_and_behaviors".to_string(),
            legacy_anchor: "omega_content::legacy_catalogs".to_string(),
            trigger: "catalog load".to_string(),
            preconditions: "content bootstrap".to_string(),
            state_delta: "none".to_string(),
            outcomes: "monster catalog available".to_string(),
            side_effects: "monster templates drive runtime".to_string(),
            coverage_kind: CoverageKind::Data,
        },
    ]
}

fn main() -> Result<()> {
    ensure_target_dir()?;
    let mut files = Vec::new();
    for root in roots() {
        if root.exists() {
            collect_rust_files(&root, &mut files)?;
        }
    }
    files.sort();

    let mut entries = Vec::new();
    for file in &files {
        entries.extend(parse_rust_symbols(file, "rust_source")?);
    }
    entries.extend(parse_rust_command_tokens("crates/omega-core/src/lib.rs")?);
    entries.extend(catalog_entries());

    let entries = dedup_entries(entries);
    let ledger = build_ledger("rust_omega_runtime", entries);
    write_json("target/rust-mechanics-ledger.json", &ledger)?;
    write_text(
        "target/rust-mechanics-ledger.md",
        &ledger_to_markdown(&ledger, "Rust Mechanics Ledger"),
    )?;

    println!(
        "rust mechanics extract: files={} total={} tiers={} domains={}",
        files.len(),
        ledger.total,
        ledger.by_tier.len(),
        ledger.by_domain.len()
    );
    Ok(())
}
