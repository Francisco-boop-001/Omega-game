#![allow(dead_code)]

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub const LEGACY_ROOT: &str = "archive/legacy-c-runtime/2026-02-06";
pub const TARGET_DIR: &str = "target";
pub const MAPPING_PATH: &str = "docs/migration/MECHANICS_PARITY_MAPPING.yaml";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum MechanicTier {
    Main,
    Secondary,
    Tertiary,
    Rest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CoverageKind {
    Runtime,
    Data,
    Presentation,
    PlatformOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MechanicEntry {
    pub id: String,
    pub name: String,
    pub tier: MechanicTier,
    pub domain: String,
    pub legacy_anchor: String,
    pub trigger: String,
    pub preconditions: String,
    pub state_delta: String,
    pub outcomes: String,
    pub side_effects: String,
    pub coverage_kind: CoverageKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MechanicsLedger {
    pub source: String,
    pub generated_at_utc: String,
    pub total: usize,
    pub by_tier: BTreeMap<MechanicTier, usize>,
    pub by_domain: BTreeMap<String, usize>,
    pub entries: Vec<MechanicEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ParityStatus {
    Exact,
    Equivalent,
    Partial,
    Missing,
    Unknown,
    ExcludedNonGameplay,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MechanicParityRow {
    pub mechanic_id: String,
    pub legacy_present: bool,
    pub rust_present: bool,
    pub parity_status: ParityStatus,
    pub evidence_static: String,
    pub evidence_dynamic: String,
    pub notes: String,
    pub tier: MechanicTier,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MechanicsParityMatrix {
    pub generated_at_utc: String,
    pub total: usize,
    pub by_status: BTreeMap<ParityStatus, usize>,
    pub unknown: usize,
    pub main_non_equivalent: usize,
    pub unresolved_gameplay: usize,
    pub gameplay_excluded: usize,
    pub pass: bool,
    pub rows: Vec<MechanicParityRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MissingDefect {
    pub id: String,
    pub severity: String,
    pub tier: MechanicTier,
    pub domain: String,
    pub mechanic_id: String,
    pub parity_status: ParityStatus,
    pub anchor: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MissingDefectBoard {
    pub generated_at_utc: String,
    pub total: usize,
    pub open: usize,
    pub defects: Vec<MissingDefect>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SmokeCheck {
    pub id: String,
    pub passed: bool,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MechanicsSmokeReport {
    pub generated_at_utc: String,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub pass: bool,
    pub checks: Vec<SmokeCheck>,
}

pub fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

pub fn ensure_target_dir() -> Result<()> {
    if !Path::new(TARGET_DIR).exists() {
        fs::create_dir_all(TARGET_DIR).context("create target directory")?;
    }
    Ok(())
}

pub fn write_json<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()> {
    let path = path.as_ref();
    let raw = serde_json::to_string_pretty(value).context("serialize json")?;
    fs::write(path, raw).with_context(|| format!("write {}", path.display()))
}

pub fn read_json<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str::<T>(&raw).with_context(|| format!("decode {}", path.display()))
}

pub fn write_text(path: impl AsRef<Path>, body: &str) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent).with_context(|| format!("mkdir {}", parent.display()))?;
    }
    fs::write(path, body).with_context(|| format!("write {}", path.display()))
}

pub fn read_lines(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    Ok(raw.lines().map(|line| line.to_string()).collect())
}

pub fn legacy_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(LEGACY_ROOT);
    for part in parts {
        path.push(part);
    }
    path
}

pub fn parse_help_commands(path: impl AsRef<Path>, context: &str) -> Result<Vec<MechanicEntry>> {
    let path = path.as_ref();
    let lines = read_lines(path)?;
    let mut out = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('_')
            || trimmed.starts_with("key")
            || trimmed.starts_with("DUNGEON")
            || trimmed.starts_with("COUNTRYSIDE")
            || trimmed.starts_with("vi keys")
            || trimmed.starts_with("capitalized vi keys")
        {
            continue;
        }
        if !trimmed.contains(':') {
            continue;
        }
        let mut parts = trimmed.split(':').collect::<Vec<_>>();
        if parts.len() < 2 {
            continue;
        }
        let token = parts.remove(0).split_whitespace().next().unwrap_or("").trim();
        if !is_help_token(token) {
            continue;
        }
        let description = parts.first().map(|value| value.trim()).unwrap_or("");
        let (tier, domain, coverage_kind) = classify_command_token(token);
        out.push(MechanicEntry {
            id: format!("legacy.command.{}", normalize_id_token(token)),
            name: format!("{token} - {description}"),
            tier,
            domain: domain.to_string(),
            legacy_anchor: format!("{}:{}", path.display(), idx + 1),
            trigger: format!("input token `{token}` in {context}"),
            preconditions: format!("context={context}"),
            state_delta: "command-specific".to_string(),
            outcomes: description.to_string(),
            side_effects: "time/action progression by command".to_string(),
            coverage_kind,
        });
    }
    Ok(out)
}

pub fn parse_c_prototypes(path: impl AsRef<Path>) -> Result<Vec<MechanicEntry>> {
    let path = path.as_ref();
    let lines = read_lines(path)?;
    let mut out = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
            || trimmed.starts_with('#')
            || !trimmed.ends_with(");")
            || !trimmed.contains('(')
        {
            continue;
        }
        let before_paren = trimmed.split('(').next().unwrap_or("").trim();
        if before_paren.is_empty() {
            continue;
        }
        let name = before_paren.split_whitespace().last().unwrap_or("");
        if !is_c_identifier(name) {
            continue;
        }
        let (tier, domain, coverage_kind) = classify_function_name(name);
        out.push(MechanicEntry {
            id: format!("legacy.function.{name}"),
            name: name.to_string(),
            tier,
            domain: domain.to_string(),
            legacy_anchor: format!("{}:{}", path.display(), idx + 1),
            trigger: function_trigger(name),
            preconditions: function_preconditions(name),
            state_delta: function_state_delta(name),
            outcomes: function_outcomes(name),
            side_effects: function_side_effects(name),
            coverage_kind,
        });
    }
    Ok(out)
}

pub fn parse_defines(path: impl AsRef<Path>) -> Result<Vec<MechanicEntry>> {
    let path = path.as_ref();
    let lines = read_lines(path)?;
    let mut out = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("#define ") {
            continue;
        }
        let rest = trimmed.trim_start_matches("#define ").trim();
        if rest.is_empty() {
            continue;
        }
        let mut chunks = rest.split_whitespace();
        let name = chunks.next().unwrap_or("");
        if !is_c_identifier(name) || name.contains('(') {
            continue;
        }
        if !(name.starts_with("S_")
            || name.starts_with("L_")
            || name.starts_with("M_")
            || name.starts_with("NUM")
            || name.starts_with("O_")
            || name.starts_with("MAX"))
        {
            continue;
        }
        let value = chunks.collect::<Vec<_>>().join(" ");
        let (tier, domain, coverage_kind) = classify_define_name(name);
        out.push(MechanicEntry {
            id: format!("legacy.define.{name}"),
            name: format!("{name}={}", value.trim()),
            tier,
            domain: domain.to_string(),
            legacy_anchor: format!("{}:{}", path.display(), idx + 1),
            trigger: define_trigger(name),
            preconditions: "constant lookup".to_string(),
            state_delta: "none".to_string(),
            outcomes: "symbolic contract".to_string(),
            side_effects: "controls behavior branches".to_string(),
            coverage_kind,
        });
    }
    Ok(out)
}

pub fn parse_rust_symbols(
    path: impl AsRef<Path>,
    source_label: &str,
) -> Result<Vec<MechanicEntry>> {
    let path = path.as_ref();
    let lines = read_lines(path)?;
    let mut out = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if let Some(name) = rust_fn_name(trimmed) {
            let (tier, domain, coverage_kind) = classify_function_name(name);
            out.push(MechanicEntry {
                id: format!("rust.function.{name}"),
                name: name.to_string(),
                tier,
                domain: domain.to_string(),
                legacy_anchor: format!("{}:{} ({source_label})", path.display(), idx + 1),
                trigger: function_trigger(name),
                preconditions: function_preconditions(name),
                state_delta: function_state_delta(name),
                outcomes: function_outcomes(name),
                side_effects: function_side_effects(name),
                coverage_kind,
            });
        }
        if let Some(variant) = rust_enum_variant(trimmed) {
            out.push(MechanicEntry {
                id: format!("rust.enum.variant.{variant}"),
                name: variant.to_string(),
                tier: MechanicTier::Tertiary,
                domain: "interaction_state".to_string(),
                legacy_anchor: format!("{}:{} ({source_label})", path.display(), idx + 1),
                trigger: "state transition".to_string(),
                preconditions: "modal interaction active".to_string(),
                state_delta: "state machine branch".to_string(),
                outcomes: "input route change".to_string(),
                side_effects: "frontend and timing behavior".to_string(),
                coverage_kind: CoverageKind::Runtime,
            });
        }
    }
    Ok(out)
}

pub fn parse_rust_command_tokens(path: impl AsRef<Path>) -> Result<Vec<MechanicEntry>> {
    let path = path.as_ref();
    let lines = read_lines(path)?;
    let mut out = Vec::new();
    let mut in_match = false;
    let mut brace_depth: i32 = 0;
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !in_match && trimmed.contains("let (note, fully_modeled) = match trimmed") {
            in_match = true;
            brace_depth = brace_delta(trimmed);
            if brace_depth <= 0 {
                in_match = false;
            }
            continue;
        }
        if !in_match {
            continue;
        }

        if trimmed.contains("=>") {
            let lhs = trimmed.split("=>").next().unwrap_or("").trim();
            for piece in lhs.split('|') {
                let token = piece.trim().trim_matches('"');
                if token.is_empty() || token == "_" {
                    continue;
                }
                if !(is_help_token(token) || token.starts_with('<')) {
                    continue;
                }
                let (tier, domain, coverage_kind) = classify_command_token(token);
                out.push(MechanicEntry {
                    id: format!("rust.command.{}", normalize_id_token(token)),
                    name: format!("legacy token {token}"),
                    tier,
                    domain: domain.to_string(),
                    legacy_anchor: format!("{}:{}", path.display(), idx + 1),
                    trigger: format!("Command::Legacy token `{token}`"),
                    preconditions: "in-game command dispatch".to_string(),
                    state_delta: "command-specific".to_string(),
                    outcomes: "legacy command branch".to_string(),
                    side_effects: "turn/log/outcome changes".to_string(),
                    coverage_kind,
                });
            }
        }

        brace_depth += brace_delta(trimmed);
        if brace_depth <= 0 {
            in_match = false;
            brace_depth = 0;
        }
    }
    Ok(out)
}

fn brace_delta(line: &str) -> i32 {
    let mut open = 0i32;
    let mut close = 0i32;
    for ch in line.chars() {
        if ch == '{' {
            open += 1;
        } else if ch == '}' {
            close += 1;
        }
    }
    open - close
}

pub fn dedup_entries(entries: Vec<MechanicEntry>) -> Vec<MechanicEntry> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for entry in entries {
        if seen.insert(entry.id.clone()) {
            deduped.push(entry);
        }
    }
    deduped.sort_by(|a, b| a.id.cmp(&b.id));
    deduped
}

pub fn build_ledger(source: &str, entries: Vec<MechanicEntry>) -> MechanicsLedger {
    let total = entries.len();
    let mut by_tier = BTreeMap::new();
    let mut by_domain = BTreeMap::new();
    for entry in &entries {
        *by_tier.entry(entry.tier.clone()).or_insert(0) += 1;
        *by_domain.entry(entry.domain.clone()).or_insert(0) += 1;
    }
    MechanicsLedger {
        source: source.to_string(),
        generated_at_utc: now_utc_unix(),
        total,
        by_tier,
        by_domain,
        entries,
    }
}

pub fn ledger_to_markdown(ledger: &MechanicsLedger, title: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {title}\n\n"));
    out.push_str(&format!(
        "- source: `{}`\n- generated_at_utc: `{}`\n- total: `{}`\n\n",
        ledger.source, ledger.generated_at_utc, ledger.total
    ));
    out.push_str("## Counts by Tier\n\n| tier | count |\n|---|---:|\n");
    for (tier, count) in &ledger.by_tier {
        out.push_str(&format!("| {:?} | {} |\n", tier, count));
    }
    out.push_str("\n## Counts by Domain\n\n| domain | count |\n|---|---:|\n");
    for (domain, count) in &ledger.by_domain {
        out.push_str(&format!("| {} | {} |\n", domain, count));
    }
    out.push_str("\n## Entries\n\n| id | tier | domain | coverage | anchor | trigger |\n|---|---|---|---|---|---|\n");
    for entry in &ledger.entries {
        out.push_str(&format!(
            "| {} | {:?} | {} | {:?} | `{}` | {} |\n",
            sanitize_table(&entry.id),
            entry.tier,
            sanitize_table(&entry.domain),
            entry.coverage_kind,
            sanitize_table(&entry.legacy_anchor),
            sanitize_table(&entry.trigger)
        ));
    }
    out
}

pub fn matrix_to_markdown(matrix: &MechanicsParityMatrix, title: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {title}\n\n"));
    out.push_str(&format!(
        "- generated_at_utc: `{}`\n- total: `{}`\n- pass: `{}`\n- unknown: `{}`\n- main_non_equivalent: `{}`\n- unresolved_gameplay: `{}`\n- gameplay_excluded: `{}`\n\n",
        matrix.generated_at_utc,
        matrix.total,
        matrix.pass,
        matrix.unknown,
        matrix.main_non_equivalent,
        matrix.unresolved_gameplay,
        matrix.gameplay_excluded
    ));
    out.push_str("## Counts by Status\n\n| status | count |\n|---|---:|\n");
    for (status, count) in &matrix.by_status {
        out.push_str(&format!("| {:?} | {} |\n", status, count));
    }
    out.push_str("\n## Rows\n\n| mechanic_id | status | tier | domain | static evidence | dynamic evidence |\n|---|---|---|---|---|---|\n");
    for row in &matrix.rows {
        out.push_str(&format!(
            "| {} | {:?} | {:?} | {} | {} | {} |\n",
            sanitize_table(&row.mechanic_id),
            row.parity_status,
            row.tier,
            sanitize_table(&row.domain),
            sanitize_table(&row.evidence_static),
            sanitize_table(&row.evidence_dynamic)
        ));
    }
    out
}

pub fn defect_board_to_markdown(board: &MissingDefectBoard, title: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {title}\n\n"));
    out.push_str(&format!(
        "- generated_at_utc: `{}`\n- total: `{}`\n- open: `{}`\n\n",
        board.generated_at_utc, board.total, board.open
    ));
    out.push_str("| id | severity | tier | domain | status | mechanic_id | anchor |\n|---|---|---|---|---|---|---|\n");
    for defect in &board.defects {
        out.push_str(&format!(
            "| {} | {} | {:?} | {} | {:?} | {} | `{}` |\n",
            sanitize_table(&defect.id),
            sanitize_table(&defect.severity),
            defect.tier,
            sanitize_table(&defect.domain),
            defect.parity_status,
            sanitize_table(&defect.mechanic_id),
            sanitize_table(&defect.anchor)
        ));
    }
    out
}

pub fn smoke_to_markdown(report: &MechanicsSmokeReport, title: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {title}\n\n"));
    out.push_str(&format!(
        "- generated_at_utc: `{}`\n- total: `{}`\n- passed: `{}`\n- failed: `{}`\n- pass: `{}`\n\n",
        report.generated_at_utc, report.total, report.passed, report.failed, report.pass
    ));
    out.push_str("| id | passed | details |\n|---|---|---|\n");
    for check in &report.checks {
        out.push_str(&format!(
            "| {} | {} | {} |\n",
            sanitize_table(&check.id),
            check.passed,
            sanitize_table(&check.details)
        ));
    }
    out
}

pub fn write_mapping_yaml(rows: &[MechanicParityRow], legacy: &MechanicsLedger) -> Result<()> {
    let mut legacy_by_id = BTreeMap::new();
    for entry in &legacy.entries {
        legacy_by_id.insert(entry.id.clone(), entry.legacy_anchor.clone());
    }
    let mut body = String::new();
    body.push_str("version: 1\n");
    body.push_str(&format!("generated_at_utc: '{}'\n", now_utc_unix()));
    body.push_str("rows:\n");
    for row in rows {
        let resolution = match row.parity_status {
            ParityStatus::Exact | ParityStatus::Equivalent => "implemented",
            ParityStatus::Partial => "partial_coverage_needs_branch_completion",
            ParityStatus::Missing => "missing_implementation",
            ParityStatus::Unknown => "investigate_unmapped_behavior",
            ParityStatus::ExcludedNonGameplay => "excluded_non_gameplay_surface",
        };
        body.push_str(&format!("  - mechanic_id: '{}'\n", escape_yaml(&row.mechanic_id)));
        body.push_str(&format!("    tier: '{}'\n", format!("{:?}", row.tier).to_lowercase()));
        body.push_str(&format!("    domain: '{}'\n", escape_yaml(&row.domain)));
        body.push_str(&format!(
            "    legacy_anchor: '{}'\n",
            escape_yaml(legacy_by_id.get(&row.mechanic_id).map(String::as_str).unwrap_or(""))
        ));
        body.push_str(&format!(
            "    parity_status: '{}'\n",
            format!("{:?}", row.parity_status).to_lowercase()
        ));
        body.push_str(&format!("    resolution: '{}'\n", resolution));
    }
    write_text(MAPPING_PATH, &body)
}

pub fn normalize_id_token(token: &str) -> String {
    token
        .replace('^', "ctrl_")
        .replace('<', "lt_")
        .replace('>', "gt_")
        .replace('/', "slash")
        .replace('?', "help")
        .replace('.', "dot")
        .replace(',', "comma")
        .replace('@', "at")
        .replace('#', "hash")
        .replace(' ', "_")
}

pub fn canonical_key(entry_id: &str) -> String {
    if let Some(value) = entry_id.strip_prefix("legacy.command.") {
        return format!("command:{value}");
    }
    if let Some(value) = entry_id.strip_prefix("rust.command.") {
        return format!("command:{value}");
    }
    if let Some(value) = entry_id.strip_prefix("legacy.function.") {
        return format!("function:{value}");
    }
    if let Some(value) = entry_id.strip_prefix("legacy.definition.") {
        return format!("function:{value}");
    }
    if let Some(value) = entry_id.strip_prefix("rust.function.") {
        return format!("function:{value}");
    }
    if let Some(value) = entry_id.strip_prefix("legacy.define.") {
        return format!("define:{value}");
    }
    entry_id.to_string()
}

pub fn is_help_token(token: &str) -> bool {
    if token.len() == 1
        && let Some(ch) = token.chars().next() {
            return ch.is_ascii_graphic() && ch != ':';
        }
    token.len() == 2 && token.starts_with('^')
}

pub fn is_c_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn rust_fn_name(line: &str) -> Option<&str> {
    if let Some(rest) = line.strip_prefix("fn ") {
        return rest.split('(').next().map(str::trim);
    }
    if let Some(rest) = line.strip_prefix("pub fn ") {
        return rest.split('(').next().map(str::trim);
    }
    None
}

fn rust_enum_variant(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.starts_with("enum ") || trimmed.starts_with("pub enum ") || !trimmed.ends_with(',') {
        return None;
    }
    let variant = trimmed.trim_end_matches(',').split('(').next()?.trim();
    if !is_c_identifier(variant) {
        return None;
    }
    if variant.chars().next()?.is_ascii_uppercase() {
        return Some(variant);
    }
    None
}

fn classify_command_token(token: &str) -> (MechanicTier, &'static str, CoverageKind) {
    match token {
        "m" | "q" | "r" | "z" | "a" | "A" | "e" => {
            (MechanicTier::Main, "magic_and_items", CoverageKind::Runtime)
        }
        "h" | "j" | "k" | "l" | "y" | "u" | "b" | "n" | "." | "," | "<" | ">" | "v" => {
            (MechanicTier::Main, "movement_and_traversal", CoverageKind::Runtime)
        }
        "f" | "t" | "p" | "G" | "D" | "T" => {
            (MechanicTier::Main, "combat_and_interaction", CoverageKind::Runtime)
        }
        "i" | "I" | "d" | "g" | "C" => {
            (MechanicTier::Main, "inventory_and_equipment", CoverageKind::Runtime)
        }
        "Q" | "S" | "R" => (MechanicTier::Main, "session_and_victory", CoverageKind::Runtime),
        "^g" | "^x" | "^w" | "^k" | "^f" => {
            (MechanicTier::Tertiary, "wizard_and_debug", CoverageKind::Runtime)
        }
        "?" | "/" | "^p" | "^o" | "^l" | "^r" | "V" | "P" | "O" => {
            (MechanicTier::Tertiary, "ui_and_help", CoverageKind::Presentation)
        }
        _ => (MechanicTier::Rest, "misc_commands", CoverageKind::Runtime),
    }
}

fn classify_function_name(name: &str) -> (MechanicTier, &'static str, CoverageKind) {
    if name.starts_with("l_") {
        return (MechanicTier::Secondary, "locations_and_sites", CoverageKind::Runtime);
    }
    if name.starts_with("s_") || name == "cast_spell" || name == "getspell" || name == "spellparse"
    {
        return (MechanicTier::Main, "magic_and_spells", CoverageKind::Runtime);
    }
    if name.starts_with("i_") || name.starts_with("weapon_") || name.starts_with("item_") {
        return (MechanicTier::Main, "items_and_equipment", CoverageKind::Runtime);
    }
    if name.starts_with("m_") || name.starts_with("monster_") {
        return (MechanicTier::Secondary, "monster_ai_and_behaviors", CoverageKind::Runtime);
    }
    if name.contains("inventory") || name.contains("getitem") || name.contains("pack") {
        return (MechanicTier::Main, "inventory_and_equipment", CoverageKind::Runtime);
    }
    if name.contains("guild")
        || name.contains("castle")
        || name.contains("arena")
        || name.contains("thieves")
        || name.contains("order")
        || name.contains("college")
        || name.contains("sorcer")
        || name.contains("temple")
        || name.contains("prayer")
    {
        return (MechanicTier::Secondary, "quests_and_progression", CoverageKind::Runtime);
    }
    if name.contains("save") || name.contains("restore") {
        return (MechanicTier::Main, "save_and_session", CoverageKind::Data);
    }
    if name.contains("display")
        || name.contains("draw")
        || name.contains("print")
        || name.contains("menu")
        || name.contains("cursor")
    {
        return (MechanicTier::Tertiary, "ui_and_logging", CoverageKind::Presentation);
    }
    if name.contains("init") || name.contains("load") || name.contains("map") {
        return (MechanicTier::Secondary, "world_and_generation", CoverageKind::Runtime);
    }
    (MechanicTier::Rest, "misc_runtime", CoverageKind::Runtime)
}

fn classify_define_name(name: &str) -> (MechanicTier, &'static str, CoverageKind) {
    if name.starts_with("S_") || name == "NUMSPELLS" {
        return (MechanicTier::Main, "magic_and_spells", CoverageKind::Data);
    }
    if name.starts_with("L_") || name == "NUMCITYSITES" || name == "NUMTRAPS" {
        return (MechanicTier::Secondary, "locations_and_sites", CoverageKind::Data);
    }
    if name.starts_with("M_") || name == "NUMMONSTERS" {
        return (MechanicTier::Secondary, "monster_ai_and_behaviors", CoverageKind::Data);
    }
    if name.starts_with("NUM") || name.starts_with("MAX") || name.starts_with("O_") {
        return (MechanicTier::Main, "core_system_contract", CoverageKind::Data);
    }
    (MechanicTier::Rest, "misc_constants", CoverageKind::Data)
}

fn function_trigger(name: &str) -> String {
    if name.starts_with("l_") {
        "enter location site or interact with local feature".to_string()
    } else if name.starts_with("i_") {
        "item use/equip/activation pathway".to_string()
    } else if name.starts_with("s_") {
        "spell cast dispatch and effect resolution".to_string()
    } else if name.starts_with("m_") || name.starts_with("monster_") {
        "monster turn, strike, movement or dialogue".to_string()
    } else if name.contains("save") || name.contains("restore") {
        "save/load lifecycle operation".to_string()
    } else {
        "runtime subsystem call path".to_string()
    }
}

fn function_preconditions(name: &str) -> String {
    if name.starts_with("l_") {
        "player is on/adjacent to matching site tile".to_string()
    } else if name.starts_with("i_") {
        "selected item exists and command path allows use".to_string()
    } else if name.starts_with("s_") {
        "spell known, cast allowed, mana/conditions satisfied".to_string()
    } else if name.contains("inventory") {
        "inventory control loop active".to_string()
    } else {
        "subsystem-specific".to_string()
    }
}

fn function_state_delta(name: &str) -> String {
    if name.starts_with("l_") {
        "location, progression, economy, flags, or map state may change".to_string()
    } else if name.starts_with("i_") || name.starts_with("weapon_") {
        "item state, player stats, or target state may change".to_string()
    } else if name.starts_with("s_") {
        "mana, statuses, targets, map and effects may change".to_string()
    } else if name.starts_with("m_") || name.starts_with("monster_") {
        "monster and player combat/position state may change".to_string()
    } else if name.contains("save") || name.contains("restore") {
        "persistent snapshot state changed or loaded".to_string()
    } else {
        "runtime state may change".to_string()
    }
}

fn function_outcomes(name: &str) -> String {
    if name.starts_with("l_") {
        "site-specific branch outcomes".to_string()
    } else if name.starts_with("i_") {
        "item effect outcome".to_string()
    } else if name.starts_with("s_") {
        "spell effect outcome".to_string()
    } else if name.starts_with("m_") || name.starts_with("monster_") {
        "monster behavior outcome".to_string()
    } else if name.contains("save") || name.contains("restore") {
        "session continuity outcome".to_string()
    } else {
        "subsystem outcome".to_string()
    }
}

fn function_side_effects(name: &str) -> String {
    if name.starts_with("l_") || name.starts_with("i_") || name.starts_with("s_") {
        "log/messages/events and resource deltas".to_string()
    } else if name.starts_with("m_") || name.starts_with("monster_") {
        "combat events and AI state".to_string()
    } else if name.contains("draw") || name.contains("display") {
        "presentation update only".to_string()
    } else {
        "may emit events/log lines".to_string()
    }
}

fn define_trigger(name: &str) -> String {
    if name.starts_with("S_") {
        "spell indexing and dispatch contracts".to_string()
    } else if name.starts_with("L_") {
        "site/location function routing".to_string()
    } else if name.starts_with("M_") {
        "monster behavior and attack routing".to_string()
    } else if name.starts_with("NUM") || name.starts_with("MAX") {
        "catalog cardinality and capacity limits".to_string()
    } else {
        "constant referenced by mechanics".to_string()
    }
}

fn sanitize_table(value: &str) -> String {
    value.replace('|', "\\|").replace(['\n', '\r'], " ").trim().to_string()
}

fn escape_yaml(value: &str) -> String {
    value.replace('\'', "''")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_token_filter_accepts_expected_shapes() {
        assert!(is_help_token("q"));
        assert!(is_help_token("^x"));
        assert!(!is_help_token("word"));
        assert!(!is_help_token(""));
    }

    #[test]
    fn c_identifier_filter_works() {
        assert!(is_c_identifier("l_arena"));
        assert!(is_c_identifier("S_MISSILE"));
        assert!(!is_c_identifier("9bad"));
        assert!(!is_c_identifier("bad-name"));
    }

    #[test]
    fn canonical_key_normalizes_prefix() {
        assert_eq!(canonical_key("legacy.command.q"), "command:q");
        assert_eq!(canonical_key("legacy.function.l_arena"), "function:l_arena");
        assert_eq!(canonical_key("legacy.define.S_MISSILE"), "define:S_MISSILE");
    }
}
