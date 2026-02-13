use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
struct ArenaRosterEntry {
    case_index: i32,
    monster_symbol: String,
}

#[derive(Debug, Clone, Serialize)]
struct FunctionContract {
    name: String,
    file: String,
    line_start: usize,
    line_end: usize,
    menu_prompts: Vec<String>,
    gates: Vec<String>,
    rewards: Vec<String>,
    rank_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct LegacyGuildSiteContract {
    source_snapshot: String,
    files_scanned: Vec<String>,
    inventory_keymap: String,
    functions: Vec<FunctionContract>,
    arena_roster: Vec<ArenaRosterEntry>,
}

#[derive(Debug, Clone)]
struct FunctionSlice {
    name: String,
    line_start: usize,
    line_end: usize,
    lines: Vec<String>,
}

fn normalize_ws(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn quoted_strings(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut start = None;
    for (idx, ch) in line.char_indices() {
        if ch == '"' {
            if let Some(open) = start.take() {
                if idx > open + 1 {
                    out.push(line[open + 1..idx].to_string());
                }
            } else {
                start = Some(idx);
            }
        }
    }
    out
}

fn extract_function_slices(content: &str) -> Vec<FunctionSlice> {
    let lines: Vec<&str> = content.lines().collect();
    let mut out = Vec::new();
    let mut idx = 0usize;
    while idx < lines.len() {
        let line = lines[idx].trim_start();
        if !line.starts_with("void l_") {
            idx += 1;
            continue;
        }
        let Some(open_paren) = line.find('(') else {
            idx += 1;
            continue;
        };
        let name = line["void ".len()..open_paren].trim().to_string();
        let mut open_line = idx;
        let mut found_open = false;
        while open_line < lines.len() {
            if lines[open_line].contains('{') {
                found_open = true;
                break;
            }
            open_line += 1;
        }
        if !found_open {
            idx += 1;
            continue;
        }

        let mut depth = 0i32;
        let mut end_line = open_line;
        let mut started = false;
        while end_line < lines.len() {
            for ch in lines[end_line].chars() {
                if ch == '{' {
                    depth += 1;
                    started = true;
                } else if ch == '}' && started {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            if started && depth == 0 {
                break;
            }
            end_line += 1;
        }
        if end_line >= lines.len() {
            idx += 1;
            continue;
        }

        let section = lines[idx..=end_line].iter().map(|line| (*line).to_string()).collect();
        out.push(FunctionSlice {
            name,
            line_start: idx + 1,
            line_end: end_line + 1,
            lines: section,
        });
        idx = end_line + 1;
    }
    out
}

fn looks_like_gate(line: &str) -> bool {
    line.contains("if (")
        && (line.contains("Player.")
            || line.contains("nighttime()")
            || line.contains("rank[")
            || line.contains("alignment")
            || line.contains("guildxp")
            || line.contains("Spellsleft"))
}

fn looks_like_reward(line: &str) -> bool {
    line.contains("gain_item(")
        || line.contains("Spells[")
        || line.contains("Player.rank[")
        || line.contains("Gymcredit")
        || line.contains("SalaryAmount")
        || line.contains("Player.cash")
        || line.contains("Player.max")
        || line.contains("Player.str")
        || line.contains("Player.con")
        || line.contains("Player.iq")
        || line.contains("Player.pow")
}

fn looks_like_rank_rule(line: &str) -> bool {
    line.contains("Player.rank[") && line.contains('=')
}

fn collect_function_contracts(path: &Path) -> Result<Vec<FunctionContract>> {
    let content = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let slices = extract_function_slices(&content);
    let file_label = path
        .file_name()
        .and_then(|s| s.to_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| path.display().to_string());
    let mut contracts = Vec::new();
    for slice in slices {
        let mut menu_prompts = Vec::new();
        let mut gates = Vec::new();
        let mut rewards = Vec::new();
        let mut rank_rules = Vec::new();

        for raw in &slice.lines {
            let line = normalize_ws(raw);
            if line.is_empty() {
                continue;
            }
            if line.contains("print") || line.contains("menuprint") {
                for quoted in quoted_strings(&line) {
                    if !quoted.trim().is_empty() {
                        menu_prompts.push(quoted.trim().to_string());
                    }
                }
            }
            if looks_like_gate(&line) {
                gates.push(line.clone());
            }
            if looks_like_reward(&line) {
                rewards.push(line.clone());
            }
            if looks_like_rank_rule(&line) {
                rank_rules.push(line);
            }
        }

        menu_prompts.sort();
        menu_prompts.dedup();
        gates.sort();
        gates.dedup();
        rewards.sort();
        rewards.dedup();
        rank_rules.sort();
        rank_rules.dedup();

        contracts.push(FunctionContract {
            name: slice.name,
            file: file_label.clone(),
            line_start: slice.line_start,
            line_end: slice.line_end,
            menu_prompts,
            gates,
            rewards,
            rank_rules,
        });
    }
    Ok(contracts)
}

fn extract_inventory_keymap(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("signed char inventory_keymap[]") {
            continue;
        }
        if let Some(start) = trimmed.find('"')
            && let Some(end) = trimmed[start + 1..].find('"')
        {
            return Ok(trimmed[start + 1..start + 1 + end].to_string());
        }
    }
    Ok(String::new())
}

fn extract_arena_roster(path: &Path) -> Result<Vec<ArenaRosterEntry>> {
    let content = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let lines: Vec<&str> = content.lines().collect();
    let mut in_arena = false;
    let mut current_case: Option<i32> = None;
    let mut map: BTreeMap<i32, String> = BTreeMap::new();

    for raw in lines {
        let line = normalize_ws(raw);
        if line.starts_with("void l_arena(") {
            in_arena = true;
            continue;
        }
        if !in_arena {
            continue;
        }
        if line.starts_with("void ") && !line.starts_with("void l_arena(") {
            break;
        }
        if line.starts_with("case ") {
            let number = line
                .trim_start_matches("case ")
                .split(':')
                .next()
                .unwrap_or_default()
                .trim()
                .parse::<i32>()
                .ok();
            current_case = number;
            continue;
        }
        if line.starts_with("default:") {
            current_case = None;
            continue;
        }
        if let Some(case_index) = current_case
            && let Some(mon_idx) = line.find("Monsters[")
            && let Some(end_idx) = line[mon_idx + "Monsters[".len()..].find(']')
        {
            let start = mon_idx + "Monsters[".len();
            let symbol = line[start..start + end_idx].trim().to_string();
            map.entry(case_index).or_insert(symbol);
        }
    }

    Ok(map
        .into_iter()
        .map(|(case_index, monster_symbol)| ArenaRosterEntry { case_index, monster_symbol })
        .collect())
}

fn main() -> Result<()> {
    let root = PathBuf::from("archive/legacy-c-runtime/2026-02-06");
    let files = [root.join("guild1.c"),
        root.join("guild2.c"),
        root.join("priest.c"),
        root.join("move.c"),
        root.join("inv.c"),
        root.join("command1.c"),
        root.join("command2.c"),
        root.join("defs.h")];

    let mut contracts = Vec::new();
    for path in [&files[0], &files[1], &files[2]] {
        contracts.extend(collect_function_contracts(path)?);
    }
    contracts.sort_by(|a, b| a.name.cmp(&b.name));

    let arena_roster = extract_arena_roster(&files[0])?;
    let inventory_keymap = extract_inventory_keymap(&files[4])?;

    let report = LegacyGuildSiteContract {
        source_snapshot: root.display().to_string(),
        files_scanned: files.iter().map(|path| path.display().to_string()).collect(),
        inventory_keymap,
        functions: contracts,
        arena_roster,
    };

    let target_dir = Path::new("target");
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).context("create target directory")?;
    }
    let json_payload =
        serde_json::to_string_pretty(&report).context("serialize guild/site contract")?;
    let json_paths = [
        target_dir.join("legacy-guild-site-contract.json"),
        target_dir.join("legacy-quest-contract.json"),
        target_dir.join("legacy-site-branch-contract.json"),
    ];
    for json_path in &json_paths {
        fs::write(json_path, &json_payload)
            .with_context(|| format!("write {}", json_path.display()))?;
    }

    let mut md = Vec::new();
    md.push("# Legacy Guild/Site Contract".to_string());
    md.push(String::new());
    md.push(format!("- Source snapshot: `{}`", report.source_snapshot));
    md.push(format!("- Files scanned: {}", report.files_scanned.len()));
    md.push(format!("- Functions extracted: {}", report.functions.len()));
    md.push(format!("- Arena roster entries: {}", report.arena_roster.len()));
    md.push(format!("- Inventory keymap: `{}`", report.inventory_keymap));
    md.push(String::new());
    md.push("| Function | File | Gates | Rewards | Prompts |".to_string());
    md.push("|---|---|---:|---:|---:|".to_string());
    for function in &report.functions {
        md.push(format!(
            "| {} | {}:{}-{} | {} | {} | {} |",
            function.name,
            function.file,
            function.line_start,
            function.line_end,
            function.gates.len(),
            function.rewards.len(),
            function.menu_prompts.len()
        ));
    }
    md.push(String::new());
    let md_payload = md.join("\n");
    let md_paths = [
        target_dir.join("legacy-guild-site-contract.md"),
        target_dir.join("legacy-quest-contract.md"),
        target_dir.join("legacy-site-branch-contract.md"),
    ];
    for md_path in &md_paths {
        fs::write(md_path, &md_payload).with_context(|| format!("write {}", md_path.display()))?;
    }

    println!(
        "legacy guild/site contract extracted: functions={} arena_roster={} keymap_len={}",
        report.functions.len(),
        report.arena_roster.len(),
        report.inventory_keymap.len()
    );
    Ok(())
}
