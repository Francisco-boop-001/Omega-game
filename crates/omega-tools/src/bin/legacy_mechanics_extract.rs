use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[path = "../mechanics_shared.rs"]
mod mechanics_shared;

use mechanics_shared::{
    CoverageKind, MechanicEntry, MechanicTier, build_ledger, dedup_entries, ensure_target_dir,
    ledger_to_markdown, legacy_path, parse_c_prototypes, parse_defines, parse_help_commands,
    write_json, write_text,
};

fn is_reserved_c_keyword(name: &str) -> bool {
    matches!(
        name,
        "if" | "else"
            | "for"
            | "while"
            | "switch"
            | "case"
            | "default"
            | "do"
            | "return"
            | "break"
            | "continue"
            | "goto"
            | "sizeof"
    )
}

fn parse_c_definitions(path: &Path) -> Result<Vec<MechanicEntry>> {
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let mut entries = Vec::new();
    for (idx, line) in raw.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
            || trimmed.starts_with("//")
            || trimmed.starts_with('#')
            || trimmed.starts_with("if ")
            || trimmed.starts_with("if(")
            || trimmed.starts_with("for ")
            || trimmed.starts_with("for(")
            || trimmed.starts_with("while ")
            || trimmed.starts_with("while(")
            || trimmed.starts_with("switch ")
            || trimmed.starts_with("switch(")
            || trimmed.starts_with("return ")
            || !trimmed.ends_with('{')
            || !trimmed.contains('(')
            || !trimmed.contains(')')
        {
            continue;
        }
        let before_paren = trimmed.split('(').next().unwrap_or("").trim();
        let name = before_paren.split_whitespace().last().unwrap_or("");
        if !mechanics_shared::is_c_identifier(name) {
            continue;
        }
        if is_reserved_c_keyword(name) {
            continue;
        }
        let (tier, domain, coverage_kind) = classify_definition_name(name);
        entries.push(MechanicEntry {
            id: format!("legacy.definition.{name}"),
            name: name.to_string(),
            tier,
            domain: domain.to_string(),
            legacy_anchor: format!("{}:{}", path.display(), idx + 1),
            trigger: "legacy runtime function entry".to_string(),
            preconditions: "call-site specific".to_string(),
            state_delta: "function dependent".to_string(),
            outcomes: "legacy behavior branch".to_string(),
            side_effects: "events, state deltas, and logs".to_string(),
            coverage_kind,
        });
    }
    Ok(entries)
}

fn classify_definition_name(name: &str) -> (MechanicTier, &'static str, CoverageKind) {
    if name.starts_with("l_") {
        return (MechanicTier::Secondary, "locations_and_sites", CoverageKind::Runtime);
    }
    if name.starts_with("s_") || name == "cast_spell" || name == "spellparse" || name == "getspell"
    {
        return (MechanicTier::Main, "magic_and_spells", CoverageKind::Runtime);
    }
    if name.starts_with("i_") || name.starts_with("weapon_") {
        return (MechanicTier::Main, "items_and_equipment", CoverageKind::Runtime);
    }
    if name.starts_with("m_") || name.starts_with("monster_") {
        return (MechanicTier::Secondary, "monster_ai_and_behaviors", CoverageKind::Runtime);
    }
    if name.contains("save") || name.contains("restore") {
        return (MechanicTier::Main, "save_and_session", CoverageKind::Data);
    }
    if name.contains("quest")
        || name.contains("guild")
        || name.contains("arena")
        || name.contains("order")
        || name.contains("thieves")
        || name.contains("castle")
        || name.contains("palace")
        || name.contains("temple")
    {
        return (MechanicTier::Secondary, "quests_and_progression", CoverageKind::Runtime);
    }
    if name.contains("draw")
        || name.contains("display")
        || name.contains("print")
        || name.contains("menu")
    {
        return (MechanicTier::Tertiary, "ui_and_logging", CoverageKind::Presentation);
    }
    (MechanicTier::Rest, "misc_runtime", CoverageKind::Runtime)
}

fn legacy_scan_files() -> Vec<PathBuf> {
    vec![
        legacy_path(&["command1.c"]),
        legacy_path(&["command2.c"]),
        legacy_path(&["command3.c"]),
        legacy_path(&["move.c"]),
        legacy_path(&["movef.c"]),
        legacy_path(&["mmove.c"]),
        legacy_path(&["site1.c"]),
        legacy_path(&["site2.c"]),
        legacy_path(&["guild1.c"]),
        legacy_path(&["guild2.c"]),
        legacy_path(&["priest.c"]),
        legacy_path(&["spell.c"]),
        legacy_path(&["item.c"]),
        legacy_path(&["itemf1.c"]),
        legacy_path(&["itemf2.c"]),
        legacy_path(&["itemf3.c"]),
        legacy_path(&["trap.c"]),
        legacy_path(&["mon.c"]),
        legacy_path(&["mmelee.c"]),
        legacy_path(&["mspec.c"]),
        legacy_path(&["mstrike.c"]),
        legacy_path(&["mtalk.c"]),
        legacy_path(&["country.c"]),
        legacy_path(&["city.c"]),
        legacy_path(&["village.c"]),
        legacy_path(&["save.c"]),
    ]
}

fn main() -> Result<()> {
    ensure_target_dir()?;

    let mut entries = Vec::new();
    entries.extend(parse_help_commands("lib/help12.txt", "dungeon_city")?);
    entries.extend(parse_help_commands("lib/help13.txt", "countryside")?);
    entries.extend(parse_c_prototypes(legacy_path(&["extern.h"]))?);
    entries.extend(parse_defines(legacy_path(&["defs.h"]))?);

    for file in legacy_scan_files() {
        if file.exists() {
            entries.extend(parse_c_definitions(&file)?);
        }
    }

    let entries = dedup_entries(entries);
    let ledger = build_ledger("legacy_omega_source_snapshot", entries);
    write_json("target/legacy-mechanics-ledger.json", &ledger)?;
    write_text(
        "target/legacy-mechanics-ledger.md",
        &ledger_to_markdown(&ledger, "Legacy Mechanics Ledger"),
    )?;

    println!(
        "legacy mechanics extract: total={} tiers={} domains={}",
        ledger.total,
        ledger.by_tier.len(),
        ledger.by_domain.len()
    );
    Ok(())
}
