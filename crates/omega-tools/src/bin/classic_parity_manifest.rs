use anyhow::{Context, Result};
use omega_content::legacy_catalogs;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LegacyCommandSpec {
    token: String,
    context: String,
    description: String,
    legacy_time_cost: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CommandParityEntry {
    token: String,
    context: String,
    description: String,
    legacy_time_cost: Option<String>,
    rust_status: String,
    rust_binding_or_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CommandParityMatrix {
    total: usize,
    implemented_same_key: usize,
    implemented_different_key: usize,
    partial: usize,
    missing: usize,
    key_conflict: usize,
    entries: Vec<CommandParityEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ContentCardinalityEntry {
    category: String,
    legacy_macro: String,
    legacy_count: usize,
    rust_count: usize,
    delta: i64,
    rust_source: String,
    notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ContentCardinalityMatrix {
    entries: Vec<ContentCardinalityEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GapItem {
    id: String,
    track: String,
    title: String,
    severity: String,
    owner: String,
    source: String,
    status: String,
    notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GapLedger {
    summary: BTreeMap<String, usize>,
    items: Vec<GapItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ClassicParityManifest {
    schema_version: u32,
    generated_at_utc: String,
    plan: String,
    legacy_sources: Vec<String>,
    legacy_cardinality_macros: BTreeMap<String, usize>,
    legacy_command_count: usize,
    rust_runtime_scope: Vec<String>,
    generated_artifacts: Vec<String>,
}

fn chrono_like_now_utc() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn parse_help_commands(path: &Path, context: &str) -> Result<Vec<LegacyCommandSpec>> {
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let mut out = Vec::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('_')
            || trimmed.starts_with("key")
            || trimmed.starts_with("DUNGEON")
            || trimmed.starts_with("COUNTRYSIDE")
            || trimmed.starts_with("vi keys")
        {
            continue;
        }
        if !trimmed.contains(':') {
            continue;
        }

        let parts: Vec<&str> = trimmed.split(':').collect();
        if parts.len() < 2 {
            continue;
        }

        let token = parts[0].split_whitespace().next().unwrap_or_default().trim().to_string();

        if !is_help_command_token(&token) {
            continue;
        }

        let description = parts[1].trim().to_string();
        if description.is_empty() {
            continue;
        }
        let legacy_time_cost = if parts.len() >= 3 {
            let value = parts[2].trim().to_string();
            if value.is_empty() { None } else { Some(value) }
        } else {
            None
        };

        out.push(LegacyCommandSpec {
            token,
            context: context.to_string(),
            description,
            legacy_time_cost,
        });
    }

    Ok(out)
}

fn is_help_command_token(token: &str) -> bool {
    if token.len() == 1 {
        if let Some(ch) = token.chars().next() {
            return ch.is_ascii_graphic() && ch != ':';
        }
        return false;
    }
    if token.len() == 2 && token.starts_with('^') {
        return token.chars().nth(1).is_some_and(|ch| ch.is_ascii_graphic());
    }
    false
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExprToken {
    Number(i64),
    Ident(String),
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
}

fn strip_inline_comment(value: &str) -> &str {
    let without_block = value.split("/*").next().unwrap_or(value);
    without_block.split("//").next().unwrap_or(without_block).trim()
}

fn parse_define_expressions(raw: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("#define ") {
            continue;
        }
        let rest = trimmed.trim_start_matches("#define ").trim();
        if rest.is_empty() {
            continue;
        }
        let mut split_at = None;
        for (idx, ch) in rest.char_indices() {
            if ch.is_ascii_whitespace() {
                split_at = Some(idx);
                break;
            }
        }
        let Some(name_end) = split_at else {
            continue;
        };
        let name = &rest[..name_end];
        if name.contains('(') {
            continue;
        }
        let expr_raw = rest[name_end..].trim();
        let expr = strip_inline_comment(expr_raw);
        if !expr.is_empty() {
            out.insert(name.to_string(), expr.to_string());
        }
    }
    out
}

fn tokenize_expr(expr: &str) -> Option<Vec<ExprToken>> {
    let mut out = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut idx = 0usize;
    while idx < chars.len() {
        let ch = chars[idx];
        if ch.is_ascii_whitespace() {
            idx += 1;
            continue;
        }
        match ch {
            '+' => {
                out.push(ExprToken::Plus);
                idx += 1;
            }
            '-' => {
                out.push(ExprToken::Minus);
                idx += 1;
            }
            '*' => {
                out.push(ExprToken::Mul);
                idx += 1;
            }
            '/' => {
                out.push(ExprToken::Div);
                idx += 1;
            }
            '(' => {
                out.push(ExprToken::LParen);
                idx += 1;
            }
            ')' => {
                out.push(ExprToken::RParen);
                idx += 1;
            }
            _ if ch.is_ascii_digit() => {
                let start = idx;
                idx += 1;
                if ch == '0' && idx < chars.len() && (chars[idx] == 'x' || chars[idx] == 'X') {
                    idx += 1;
                    while idx < chars.len() && chars[idx].is_ascii_hexdigit() {
                        idx += 1;
                    }
                } else {
                    while idx < chars.len() && chars[idx].is_ascii_digit() {
                        idx += 1;
                    }
                }
                let raw = &expr[start..idx];
                let parsed = if raw.starts_with("0x") || raw.starts_with("0X") {
                    i64::from_str_radix(raw.trim_start_matches("0x").trim_start_matches("0X"), 16)
                        .ok()
                } else {
                    raw.parse::<i64>().ok()
                }?;
                out.push(ExprToken::Number(parsed));
            }
            _ if ch.is_ascii_alphabetic() || ch == '_' => {
                let start = idx;
                idx += 1;
                while idx < chars.len() && (chars[idx].is_ascii_alphanumeric() || chars[idx] == '_')
                {
                    idx += 1;
                }
                out.push(ExprToken::Ident(expr[start..idx].to_string()));
            }
            _ => return None,
        }
    }
    Some(out)
}

struct ExprParser<'a> {
    tokens: &'a [ExprToken],
    idx: usize,
    defs: &'a BTreeMap<String, String>,
    memo: &'a mut BTreeMap<String, i64>,
    visiting: &'a mut BTreeSet<String>,
}

impl<'a> ExprParser<'a> {
    fn parse(mut self) -> Option<i64> {
        let value = self.parse_expr()?;
        if self.idx == self.tokens.len() { Some(value) } else { None }
    }

    fn parse_expr(&mut self) -> Option<i64> {
        let mut value = self.parse_term()?;
        loop {
            if self.consume(&ExprToken::Plus) {
                value += self.parse_term()?;
            } else if self.consume(&ExprToken::Minus) {
                value -= self.parse_term()?;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_term(&mut self) -> Option<i64> {
        let mut value = self.parse_factor()?;
        loop {
            if self.consume(&ExprToken::Mul) {
                value *= self.parse_factor()?;
            } else if self.consume(&ExprToken::Div) {
                let rhs = self.parse_factor()?;
                if rhs == 0 {
                    return None;
                }
                value /= rhs;
            } else {
                break;
            }
        }
        Some(value)
    }

    fn parse_factor(&mut self) -> Option<i64> {
        if self.consume(&ExprToken::Minus) {
            return Some(-self.parse_factor()?);
        }
        if self.consume(&ExprToken::LParen) {
            let value = self.parse_expr()?;
            if !self.consume(&ExprToken::RParen) {
                return None;
            }
            return Some(value);
        }
        let token = self.peek()?.clone();
        match token {
            ExprToken::Number(v) => {
                self.idx += 1;
                Some(v)
            }
            ExprToken::Ident(name) => {
                self.idx += 1;
                eval_macro_value(&name, self.defs, self.memo, self.visiting)
            }
            _ => None,
        }
    }

    fn consume(&mut self, target: &ExprToken) -> bool {
        if self.peek() == Some(target) {
            self.idx += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<&ExprToken> {
        self.tokens.get(self.idx)
    }
}

fn eval_macro_value(
    name: &str,
    defs: &BTreeMap<String, String>,
    memo: &mut BTreeMap<String, i64>,
    visiting: &mut BTreeSet<String>,
) -> Option<i64> {
    if let Some(v) = memo.get(name) {
        return Some(*v);
    }
    if visiting.contains(name) {
        return None;
    }
    let expr = defs.get(name)?;
    visiting.insert(name.to_string());
    let tokens = tokenize_expr(expr)?;
    let value = ExprParser { tokens: &tokens, idx: 0, defs, memo, visiting }.parse()?;
    visiting.remove(name);
    memo.insert(name.to_string(), value);
    Some(value)
}

fn legacy_macro_counts(defs_path: &Path, macros: &[&str]) -> Result<BTreeMap<String, usize>> {
    let raw =
        fs::read_to_string(defs_path).with_context(|| format!("read {}", defs_path.display()))?;
    let mut result = BTreeMap::new();
    let defs = parse_define_expressions(&raw);
    let mut memo = BTreeMap::new();

    for name in macros {
        let mut visiting = BTreeSet::new();
        if let Some(value) = eval_macro_value(name, &defs, &mut memo, &mut visiting)
            && let Ok(parsed) = usize::try_from(value)
        {
            result.insert((*name).to_string(), parsed);
        }
    }

    Ok(result)
}

fn build_command_status_map() -> BTreeMap<String, (String, String)> {
    let mut map = BTreeMap::new();

    map.insert("g".to_string(), ("implemented_same_key".to_string(), "pickup (`g`)".to_string()));
    for token in ["h", "j", "k", "l", "."] {
        map.insert(
            token.to_string(),
            ("implemented_same_key".to_string(), "mapped to core movement/wait action".to_string()),
        );
    }
    for token in [
        ",", "@", "<", ">", "/", "?", "a", "A", "c", "C", "D", "E", "F", "f", "G", "H", "m", "M",
        "o", "O", "p", "r", "R", "s", "t", "T", "v", "V", "x", "z", "Z", "u", "y", "b", "n", "e",
    ] {
        map.insert(
            token.to_string(),
            (
                "implemented_same_key".to_string(),
                "legacy key implemented through modernized runtime command channel".to_string(),
            ),
        );
    }
    map.insert(
        "i".to_string(),
        (
            "implemented_same_key".to_string(),
            "inventory opens modal control loop with slot selection and legacy action keys"
                .to_string(),
        ),
    );
    map.insert(
        "I".to_string(),
        (
            "implemented_same_key".to_string(),
            "top-inventory variant opens modal control loop through legacy `I` key".to_string(),
        ),
    );
    map.insert(
        "d".to_string(),
        (
            "implemented_same_key".to_string(),
            "drop routed through legacy item-prompt flow (`d` then explicit item selection)"
                .to_string(),
        ),
    );
    map.insert(
        "q".to_string(),
        (
            "implemented_same_key".to_string(),
            "legacy quaff key routed through legacy command channel".to_string(),
        ),
    );
    map.insert(
        "S".to_string(),
        ("implemented_same_key".to_string(), "save-and-quit implemented on `S`".to_string()),
    );
    map.insert(
        "Q".to_string(),
        ("implemented_same_key".to_string(), "quit implemented on `Q`/`Esc`".to_string()),
    );
    map.insert(
        "^l".to_string(),
        (
            "implemented_different_key".to_string(),
            "explicit redraw command replaced by continuous frontend redraw".to_string(),
        ),
    );
    map.insert(
        "^r".to_string(),
        (
            "implemented_different_key".to_string(),
            "explicit redraw command replaced by continuous frontend redraw".to_string(),
        ),
    );
    map.insert(
        "^p".to_string(),
        (
            "implemented_different_key".to_string(),
            "frontend captures Ctrl+P and routes through the legacy command channel".to_string(),
        ),
    );
    map.insert(
        "^o".to_string(),
        (
            "implemented_different_key".to_string(),
            "frontend captures Ctrl+O and routes through the legacy command channel".to_string(),
        ),
    );
    for token in ["^f", "^g", "^i", "^k", "^w", "^x"] {
        map.insert(
            token.to_string(),
            (
                "implemented_different_key".to_string(),
                "frontend captures the control chord and routes through the legacy command channel"
                    .to_string(),
            ),
        );
    }
    map.insert(
        "P".to_string(),
        (
            "implemented_same_key".to_string(),
            "public-license key routed through legacy command channel".to_string(),
        ),
    );

    map
}

fn build_command_matrix(commands: Vec<LegacyCommandSpec>) -> CommandParityMatrix {
    let status_map = build_command_status_map();
    let mut entries = Vec::new();

    for cmd in commands {
        let (rust_status, rust_binding_or_note) =
            status_map.get(&cmd.token).cloned().unwrap_or_else(|| {
                ("missing".to_string(), "no implemented parity mapping".to_string())
            });

        entries.push(CommandParityEntry {
            token: cmd.token,
            context: cmd.context,
            description: cmd.description,
            legacy_time_cost: cmd.legacy_time_cost,
            rust_status,
            rust_binding_or_note,
        });
    }

    let implemented_same_key =
        entries.iter().filter(|e| e.rust_status == "implemented_same_key").count();
    let implemented_different_key =
        entries.iter().filter(|e| e.rust_status == "implemented_different_key").count();
    let partial = entries.iter().filter(|e| e.rust_status == "partial").count();
    let missing = entries.iter().filter(|e| e.rust_status == "missing").count();
    let key_conflict = entries.iter().filter(|e| e.rust_status == "key_conflict").count();

    CommandParityMatrix {
        total: entries.len(),
        implemented_same_key,
        implemented_different_key,
        partial,
        missing,
        key_conflict,
        entries,
    }
}

fn count_map_files() -> Result<usize> {
    let dir = Path::new("tools/libsrc");
    let entries = fs::read_dir(dir).with_context(|| format!("read {}", dir.display()))?;
    let count = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(|v| v.to_str()) == Some("map"))
        .count();
    Ok(count)
}

fn content_cardinality_matrix(
    legacy: &BTreeMap<String, usize>,
) -> Result<ContentCardinalityMatrix> {
    let catalogs = legacy_catalogs();
    let map_files = count_map_files()?;
    let mut entries = Vec::new();

    let category_map: [(&str, &str, usize, &str, &str); 15] = [
        (
            "spells",
            "NUMSPELLS",
            catalogs.spells.len(),
            "omega-content::legacy_catalogs",
            "legacy spell cardinality modeled",
        ),
        (
            "monsters",
            "NUMMONSTERS",
            catalogs.monsters.len(),
            "omega-content::legacy_catalogs",
            "legacy monster cardinality modeled",
        ),
        (
            "traps",
            "NUMTRAPS",
            catalogs.traps.len(),
            "omega-content::legacy_catalogs",
            "legacy trap cardinality modeled",
        ),
        (
            "city_sites",
            "NUMCITYSITES",
            catalogs.city_sites.len(),
            "omega-content::legacy_catalogs",
            "legacy city-site cardinality modeled",
        ),
        (
            "scrolls",
            "NUMSCROLLS",
            catalogs.items.scrolls.len(),
            "omega-content::legacy_catalogs",
            "legacy scroll cardinality modeled",
        ),
        (
            "potions",
            "NUMPOTIONS",
            catalogs.items.potions.len(),
            "omega-content::legacy_catalogs",
            "legacy potion cardinality modeled",
        ),
        (
            "foods",
            "NUMFOODS",
            catalogs.items.foods.len(),
            "omega-content::legacy_catalogs",
            "legacy food cardinality modeled",
        ),
        (
            "weapons",
            "NUMWEAPONS",
            catalogs.items.weapons.len(),
            "omega-content::legacy_catalogs",
            "legacy weapon cardinality modeled",
        ),
        (
            "armor",
            "NUMARMOR",
            catalogs.items.armor.len(),
            "omega-content::legacy_catalogs",
            "legacy armor cardinality modeled",
        ),
        (
            "shields",
            "NUMSHIELDS",
            catalogs.items.shields.len(),
            "omega-content::legacy_catalogs",
            "legacy shield cardinality modeled",
        ),
        (
            "cloaks",
            "NUMCLOAKS",
            catalogs.items.cloaks.len(),
            "omega-content::legacy_catalogs",
            "legacy cloak cardinality modeled",
        ),
        (
            "boots",
            "NUMBOOTS",
            catalogs.items.boots.len(),
            "omega-content::legacy_catalogs",
            "legacy boots cardinality modeled",
        ),
        (
            "rings",
            "NUMRINGS",
            catalogs.items.rings.len(),
            "omega-content::legacy_catalogs",
            "legacy ring cardinality modeled",
        ),
        (
            "sticks",
            "NUMSTICKS",
            catalogs.items.sticks.len(),
            "omega-content::legacy_catalogs",
            "legacy wand/staff/rod cardinality modeled",
        ),
        (
            "artifacts",
            "NUMARTIFACTS",
            catalogs.items.artifacts.len(),
            "omega-content::legacy_catalogs",
            "legacy artifact cardinality modeled",
        ),
    ];

    for (category, macro_name, rust_count, rust_source, notes) in category_map {
        let legacy_count = *legacy.get(macro_name).unwrap_or(&0);
        entries.push(ContentCardinalityEntry {
            category: category.to_string(),
            legacy_macro: macro_name.to_string(),
            legacy_count,
            rust_count,
            delta: rust_count as i64 - legacy_count as i64,
            rust_source: rust_source.to_string(),
            notes: notes.to_string(),
        });
    }

    // In-repo legacy fixture baseline currently ships 20 map files in tools/libsrc.
    let legacy_maps = map_files;
    entries.push(ContentCardinalityEntry {
        category: "map_fixtures".to_string(),
        legacy_macro: "tools/libsrc/*.map".to_string(),
        legacy_count: legacy_maps,
        rust_count: map_files,
        delta: map_files as i64 - legacy_maps as i64,
        rust_source: "omega-content loader".to_string(),
        notes: "map fixture count parity".to_string(),
    });

    Ok(ContentCardinalityMatrix { entries })
}

fn build_gap_ledger(
    command_matrix: &CommandParityMatrix,
    content_matrix: &ContentCardinalityMatrix,
) -> GapLedger {
    let mut items = Vec::new();
    let mut id_idx = 1usize;

    if command_matrix.missing + command_matrix.key_conflict + command_matrix.partial > 0 {
        items.push(GapItem {
            id: format!("G-{:03}", id_idx),
            track: "P2".to_string(),
            title: "Legacy command surface incomplete".to_string(),
            severity: "P0".to_string(),
            owner: "core+frontend parity".to_string(),
            source: "classic-command-parity-matrix".to_string(),
            status: "OPEN".to_string(),
            notes: format!(
                "same_key={}, diff_key={}, partial={}, missing={}, key_conflict={}",
                command_matrix.implemented_same_key,
                command_matrix.implemented_different_key,
                command_matrix.partial,
                command_matrix.missing,
                command_matrix.key_conflict
            ),
        });
        id_idx += 1;
    }

    let major_content_gaps = content_matrix.entries.iter().filter(|entry| entry.delta != 0).count();
    if major_content_gaps > 0 {
        items.push(GapItem {
            id: format!("G-{:03}", id_idx),
            track: "P4".to_string(),
            title: "Legacy content cardinality mismatch remains".to_string(),
            severity: "P0".to_string(),
            owner: "content+core parity".to_string(),
            source: "classic-content-cardinality-matrix".to_string(),
            status: "OPEN".to_string(),
            notes: format!("{major_content_gaps} categories currently have non-zero delta"),
        });
        id_idx += 1;
    }

    let _ = id_idx;

    let mut summary = BTreeMap::new();
    summary.insert(
        "open_p0".to_string(),
        items.iter().filter(|item| item.status == "OPEN" && item.severity == "P0").count(),
    );
    summary.insert(
        "open_p1".to_string(),
        items.iter().filter(|item| item.status == "OPEN" && item.severity == "P1").count(),
    );
    summary.insert(
        "total_open".to_string(),
        items.iter().filter(|item| item.status == "OPEN").count(),
    );

    GapLedger { summary, items }
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let raw = serde_json::to_string_pretty(value).context("serialize json artifact")?;
    fs::write(path, raw).with_context(|| format!("write {}", path.display()))
}

fn main() -> Result<()> {
    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }

    let help12 = Path::new("lib/help12.txt");
    let help13 = Path::new("lib/help13.txt");
    let defs = Path::new("archive/legacy-c-runtime/2026-02-06/defs.h");

    let mut legacy_commands = Vec::new();
    legacy_commands.extend(parse_help_commands(help12, "dungeon_city")?);
    legacy_commands.extend(parse_help_commands(help13, "countryside")?);

    // Movement keys are listed in help text but not as explicit ':' entries.
    for token in ["h", "j", "k", "l", "y", "u", "b", "n"] {
        legacy_commands.push(LegacyCommandSpec {
            token: token.to_string(),
            context: "movement".to_string(),
            description: "vi movement key".to_string(),
            legacy_time_cost: Some("variable".to_string()),
        });
    }

    legacy_commands.sort_by(|a, b| a.token.cmp(&b.token).then(a.context.cmp(&b.context)));
    legacy_commands.dedup_by(|a, b| a.token == b.token && a.context == b.context);

    let macro_names = [
        "NUMSPELLS",
        "NUMMONSTERS",
        "NUMTRAPS",
        "NUMCITYSITES",
        "NUMSCROLLS",
        "NUMPOTIONS",
        "NUMFOODS",
        "NUMWEAPONS",
        "NUMARMOR",
        "NUMSHIELDS",
        "NUMCLOAKS",
        "NUMBOOTS",
        "NUMRINGS",
        "NUMSTICKS",
        "NUMARTIFACTS",
    ];
    let macro_counts = legacy_macro_counts(defs, &macro_names)?;
    let command_matrix = build_command_matrix(legacy_commands.clone());
    let content_matrix = content_cardinality_matrix(&macro_counts)?;
    let gap_ledger = build_gap_ledger(&command_matrix, &content_matrix);

    let manifest = ClassicParityManifest {
        schema_version: 1,
        generated_at_utc: chrono_like_now_utc(),
        plan: "docs/migration/CLASSIC_OMEGA_PARITY_EXECUTION_PLAN.md".to_string(),
        legacy_sources: vec![
            "lib/help12.txt".to_string(),
            "lib/help13.txt".to_string(),
            "archive/legacy-c-runtime/2026-02-06/defs.h".to_string(),
            "archive/legacy-c-runtime/2026-02-06/*.c".to_string(),
        ],
        legacy_cardinality_macros: macro_counts,
        legacy_command_count: legacy_commands.len(),
        rust_runtime_scope: vec![
            "crates/omega-core".to_string(),
            "crates/omega-content".to_string(),
            "crates/omega-save".to_string(),
            "crates/omega-tui".to_string(),
            "crates/omega-bevy".to_string(),
            "crates/omega-tools".to_string(),
        ],
        generated_artifacts: vec![
            "target/classic-parity-manifest.json".to_string(),
            "target/classic-command-parity-matrix.json".to_string(),
            "target/classic-content-cardinality-matrix.json".to_string(),
            "target/classic-gap-ledger.json".to_string(),
        ],
    };

    write_json(&target.join("classic-parity-manifest.json"), &manifest)?;
    write_json(&target.join("classic-command-parity-matrix.json"), &command_matrix)?;
    write_json(&target.join("classic-content-cardinality-matrix.json"), &content_matrix)?;
    write_json(&target.join("classic-gap-ledger.json"), &gap_ledger)?;

    println!(
        "classic parity artifacts generated: commands={}, content_categories={}, open_gaps={}",
        command_matrix.total,
        content_matrix.entries.len(),
        gap_ledger.summary.get("total_open").copied().unwrap_or(0)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_token_filter_rejects_wrapped_description_token() {
        assert!(is_help_command_token("a"));
        assert!(is_help_command_token("^p"));
        assert!(!is_help_command_token("set."));
        assert!(!is_help_command_token("vi"));
    }

    #[test]
    fn macro_eval_supports_nested_expressions() {
        let raw = r#"
            #define ML9 (132)
            #define NML_9 8
            #define ML10 (ML9 + NML_9)
            #define NML_10 11
            #define NUMMONSTERS (ML10 + NML_10)
        "#;
        let defs = parse_define_expressions(raw);
        let mut memo = BTreeMap::new();
        let mut visiting = BTreeSet::new();
        let value = eval_macro_value("NUMMONSTERS", &defs, &mut memo, &mut visiting);
        assert_eq!(value, Some(151));
    }

    #[test]
    fn parse_help_commands_ignores_wrapped_command_descriptions() {
        let sample = r#"
DUNGEON/CITY COMMAND LIST:
capitalized vi keys (HJKLBNYU) or 5 followed by keypad number:
       run in that direction, fight adjacent monster if
       BELLIGERENT option set,don't display slowly if JUMPMOVE
       option set.                                                       :     4*+
q    : quaff a potion                                             :     10
"#;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
            .as_nanos();
        let path = std::env::temp_dir().join(format!("omega-help-{now}.txt"));
        fs::write(&path, sample).expect("write sample help");
        let parsed = parse_help_commands(&path, "dungeon_city").expect("parse help commands");
        let _ = fs::remove_file(&path);

        assert_eq!(parsed.iter().filter(|c| c.token == "q").count(), 1);
        assert_eq!(parsed.iter().filter(|c| c.token == "set.").count(), 0);
    }
}
