use anyhow::{Context, Result, bail};
use omega_content::{LegacyItemFamily, LegacyItemPrototype, legacy_item_prototypes};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const LEGACY_ITEM_C: &str = include_str!("../../../../archive/legacy-c-runtime/2026-02-06/item.c");
const LEGACY_IINIT_H: &str =
    include_str!("../../../../archive/legacy-c-runtime/2026-02-06/iinit.h");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SpecReport {
    total_items: usize,
    by_family: BTreeMap<String, usize>,
    parse_drops: usize,
    items: Vec<LegacyItemPrototype>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct UsefBinding {
    usef: String,
    handler: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct UsefReport {
    total_bindings: usize,
    bindings: Vec<UsefBinding>,
    unmatched_usef_from_items: Vec<String>,
    coverage_complete: bool,
}

fn main() -> Result<()> {
    let items = legacy_item_prototypes();
    if items.is_empty() {
        bail!("legacy item prototype parser returned zero entries");
    }
    let candidate_rows = LEGACY_IINIT_H
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.starts_with('{') && (trimmed.ends_with("},") || trimmed.ends_with('}'))
        })
        .count();
    let parse_drops = candidate_rows.saturating_sub(items.len());
    if parse_drops > 0 {
        bail!(
            "legacy item prototype parsing dropped {} rows (candidates={} parsed={})",
            parse_drops,
            candidate_rows,
            items.len()
        );
    }

    let mut by_family = BTreeMap::new();
    for item in &items {
        *by_family.entry(family_name(item.family).to_string()).or_insert(0) += 1;
    }

    let bindings = parse_item_usef_bindings();
    let mapped: BTreeSet<String> = bindings.iter().map(|entry| entry.usef.clone()).collect();
    let usef_in_items: BTreeSet<String> = items
        .iter()
        .map(|item| item.usef.trim().to_string())
        .filter(|usef| usef.starts_with("I_"))
        .collect();
    let unmatched_usef_from_items =
        usef_in_items.into_iter().filter(|usef| !mapped.contains(usef)).collect::<Vec<_>>();

    let spec_report =
        SpecReport { total_items: items.len(), by_family, parse_drops, items: items.clone() };
    let usef_report = UsefReport {
        total_bindings: bindings.len(),
        bindings,
        unmatched_usef_from_items,
        coverage_complete: false,
    };
    let mut usef_report = usef_report;
    usef_report.coverage_complete = usef_report.unmatched_usef_from_items.is_empty();

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }

    let spec_json = target.join("legacy-item-spec.json");
    let usef_json = target.join("legacy-item-usef-map.json");
    fs::write(
        &spec_json,
        serde_json::to_string_pretty(&spec_report).context("serialize legacy item spec report")?,
    )
    .with_context(|| format!("write {}", spec_json.display()))?;
    fs::write(
        &usef_json,
        serde_json::to_string_pretty(&usef_report).context("serialize legacy usef report")?,
    )
    .with_context(|| format!("write {}", usef_json.display()))?;

    println!(
        "legacy item spec extracted: total_items={} usef_bindings={} unmatched_usef={}",
        spec_report.total_items,
        usef_report.total_bindings,
        usef_report.unmatched_usef_from_items.len()
    );
    Ok(())
}

fn parse_item_usef_bindings() -> Vec<UsefBinding> {
    let mut out = Vec::new();
    for line in LEGACY_ITEM_C.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("case I_") {
            continue;
        }
        let Some(after_case) = trimmed.strip_prefix("case ") else {
            continue;
        };
        let Some((usef_raw, rhs)) = after_case.split_once(':') else {
            continue;
        };
        let usef = usef_raw.trim().to_string();
        let Some(handler_start) = rhs.find("i_") else {
            continue;
        };
        let handler_slice = &rhs[handler_start..];
        let Some(paren) = handler_slice.find('(') else {
            continue;
        };
        let handler = handler_slice[..paren].trim().to_string();
        out.push(UsefBinding { usef, handler });
    }
    // Legacy constants represented numerically or handled outside item_use switch still need a map entry.
    for (usef, handler) in [
        ("I_NO_OP", "i_no_op"),
        ("I_NOTHING", "i_nothing"),
        ("I_HINT", "i_hint"),
        ("I_BOOTS_JUMPING", "i_boots_jumping"),
        ("I_BOOTS_7LEAGUE", "i_boots_7league"),
    ] {
        if !out.iter().any(|entry| entry.usef == usef) {
            out.push(UsefBinding { usef: usef.to_string(), handler: handler.to_string() });
        }
    }
    out.sort_by(|a, b| a.usef.cmp(&b.usef));
    out.dedup_by(|a, b| a.usef == b.usef && a.handler == b.handler);
    out
}

fn family_name(family: LegacyItemFamily) -> &'static str {
    match family {
        LegacyItemFamily::Thing => "thing",
        LegacyItemFamily::Food => "food",
        LegacyItemFamily::Scroll => "scroll",
        LegacyItemFamily::Potion => "potion",
        LegacyItemFamily::Weapon => "weapon",
        LegacyItemFamily::Armor => "armor",
        LegacyItemFamily::Shield => "shield",
        LegacyItemFamily::Cloak => "cloak",
        LegacyItemFamily::Boots => "boots",
        LegacyItemFamily::Ring => "ring",
        LegacyItemFamily::Stick => "stick",
        LegacyItemFamily::Artifact => "artifact",
        LegacyItemFamily::Cash => "cash",
        LegacyItemFamily::Corpse => "corpse",
    }
}
