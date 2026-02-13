use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const CORE_SOURCE: &str = include_str!("../../../omega-core/src/lib.rs");

#[derive(Debug, Clone, Serialize)]
struct RustBranchRow {
    branch_id: String,
    service: String,
    handler: String,
    kind: String,
    predicate: String,
}

#[derive(Debug, Clone, Serialize)]
struct RustSiteBranchContract {
    source: String,
    total_branches: usize,
    services: usize,
    pass: bool,
    branches: Vec<RustBranchRow>,
}

fn handler_for_service(service: &str) -> &'static str {
    match service {
        "merc" => "apply_merc_talk_command",
        "thieves" => "apply_thieves_talk_command",
        "college" => "apply_college_talk_command",
        "sorcerors" => "apply_sorcerors_talk_command",
        "order" => "apply_order_talk_command",
        "castle" => "apply_castle_talk_command",
        "arena" => "apply_arena_talk_command",
        "temple" => "apply_temple_talk_command",
        "monastery" => "apply_monastery_talk_command",
        "palace" => "apply_palace_talk_command",
        _ => "",
    }
}

fn service_from_kind(kind: &str) -> Option<&'static str> {
    match kind {
        "MercGuild" => Some("merc"),
        "ThievesGuild" => Some("thieves"),
        "College" => Some("college"),
        "Sorcerors" => Some("sorcerors"),
        "Order" => Some("order"),
        "Castle" => Some("castle"),
        "Palace" => Some("palace"),
        "Arena" => Some("arena"),
        "Temple" | "Altar" => Some("temple"),
        "Monastery" => Some("monastery"),
        _ => None,
    }
}

fn extract_function_body(source: &str, fn_name: &str) -> Option<String> {
    let needle = format!("fn {fn_name}(");
    let start = source.find(&needle)?;
    let mut brace_start = None;
    for (idx, ch) in source[start..].char_indices() {
        if ch == '{' {
            brace_start = Some(start + idx);
            break;
        }
    }
    let brace_start = brace_start?;
    let mut depth = 0i32;
    for (idx, ch) in source[brace_start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let end = brace_start + idx + 1;
                    return Some(source[brace_start..end].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

fn extract_site_choice_segments(source: &str) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::<String, String>::new();
    let Some(body) = extract_function_body(source, "apply_site_interaction_choice") else {
        return out;
    };
    let mut current: Option<String> = None;
    for raw in body.lines() {
        let line = raw.trim_start();
        if let Some(pos) = line.find("SiteInteractionKind::")
            && let Some(arrow) = line[pos..].find("=>")
        {
            let token_start = pos + "SiteInteractionKind::".len();
            let token_end = pos + arrow;
            let token_raw = line[token_start..token_end].trim();
            let token = token_raw
                .trim_end_matches(',')
                .split_whitespace()
                .next()
                .unwrap_or(token_raw)
                .split('{')
                .next()
                .unwrap_or(token_raw)
                .trim();
            current = service_from_kind(token).map(|service| service.to_string());
        }
        if let Some(service) = &current {
            out.entry(service.clone()).or_default().push_str(raw);
            out.entry(service.clone()).or_default().push('\n');
        }
    }
    out
}

fn normalize_ws(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_gate_line(line: &str) -> bool {
    if line.starts_with("if ")
        || line.starts_with("if(")
        || line.starts_with("if (")
        || line.starts_with("else if ")
        || line.starts_with("else if(")
        || line.starts_with("} else if ")
        || line == "else {"
        || line == "} else {"
        || line.starts_with("match ")
    {
        return true;
    }

    // Count every explicit match arm as a branch node, not only block-form arms.
    if line.contains("=>") {
        return true;
    }

    false
}

fn is_effect_line(line: &str) -> bool {
    let touches_state = line.contains("state.");
    let plain_assign = (line.contains(" = ") || line.ends_with(" =") || line.contains(" ="))
        && !line.contains("==")
        && !line.contains("!=")
        && !line.contains(">=")
        && !line.contains("<=")
        && !line.contains("=>");
    let has_assignment = plain_assign
        || line.contains("+=")
        || line.contains("-=")
        || line.contains("*=")
        || line.contains("/=")
        || line.contains("|=")
        || line.contains("&=");

    let mutating_state_call = line.contains("state.status_effects.retain(")
        || line.contains("state.player.inventory.retain(")
        || line.contains("state.player.inventory.push(")
        || line.contains("state.log.push(")
        || line.contains("state.map_spells_to_known")
        || line.contains("state.pending_site_interaction =")
        || line.contains("keep_open = ");

    let mutating_helper_call = line.contains("add_item_to_inventory_or_ground")
        || line.contains("remove_item_by_id(")
        || line.contains("start_arena_challenge(")
        || line.contains("start_main_quest_from_dialogue(")
        || line.contains("teach_first_unknown_from_pool(")
        || line.contains("apply_altar_")
        || line.contains("mark_player_defeated(")
        || line.contains("set_inventory_slot_item_id(");

    // Player-visible outcome emission is part of branch behavior parity.
    let outcome_emission = line.contains("events.push(")
        || line.contains("notes.push(")
        || line.contains("format!(")
        || line.ends_with(".to_string(),")
        || line.ends_with(".to_string()");

    (touches_state && has_assignment)
        || mutating_state_call
        || mutating_helper_call
        || outcome_emission
}

fn collect_rows(service: &str, handler: &str, body: &str, bucket: &str) -> Vec<RustBranchRow> {
    let mut rows = Vec::new();
    rows.push(RustBranchRow {
        branch_id: format!("{service}/{bucket}/entry"),
        service: service.to_string(),
        handler: handler.to_string(),
        kind: "entry".to_string(),
        predicate: "always".to_string(),
    });

    let mut gate_idx = 0usize;
    let mut effect_idx = 0usize;
    for raw in body.lines() {
        let line = normalize_ws(raw);
        if is_gate_line(&line) {
            gate_idx += 1;
            rows.push(RustBranchRow {
                branch_id: format!("{service}/{bucket}/gate/{gate_idx}"),
                service: service.to_string(),
                handler: handler.to_string(),
                kind: "gate".to_string(),
                predicate: line,
            });
            continue;
        }
        if is_effect_line(&line) {
            effect_idx += 1;
            rows.push(RustBranchRow {
                branch_id: format!("{service}/{bucket}/effect/{effect_idx}"),
                service: service.to_string(),
                handler: handler.to_string(),
                kind: "effect".to_string(),
                predicate: line,
            });
        }
    }
    rows
}

fn markdown(contract: &RustSiteBranchContract) -> String {
    let mut out = Vec::new();
    out.push("# Rust Site Branch Contract".to_string());
    out.push(String::new());
    out.push(format!("- services: `{}`", contract.services));
    out.push(format!("- total_branches: `{}`", contract.total_branches));
    out.push(String::new());
    out.push("| Branch ID | Service | Handler | Kind | Predicate |".to_string());
    out.push("|---|---|---|---|---|".to_string());
    for row in &contract.branches {
        out.push(format!(
            "| {} | {} | {} | {} | {} |",
            row.branch_id,
            row.service,
            row.handler,
            row.kind,
            row.predicate.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let services = [
        "merc",
        "thieves",
        "college",
        "sorcerors",
        "order",
        "castle",
        "palace",
        "arena",
        "temple",
        "monastery",
    ];

    let mut rows = Vec::new();
    let choice_segments = extract_site_choice_segments(CORE_SOURCE);
    for service in services {
        let handler = handler_for_service(service);
        if handler.is_empty() {
            continue;
        }
        let Some(body) = extract_function_body(CORE_SOURCE, handler) else {
            rows.push(RustBranchRow {
                branch_id: format!("{service}/missing"),
                service: service.to_string(),
                handler: handler.to_string(),
                kind: "missing".to_string(),
                predicate: "handler_not_found".to_string(),
            });
            continue;
        };
        rows.extend(collect_rows(service, handler, &body, "talk"));
        if let Some(segment) = choice_segments.get(service) {
            rows.extend(collect_rows(
                service,
                &format!("apply_site_interaction_choice::{service}"),
                segment,
                "choice",
            ));
        }
    }

    let services_count = rows.iter().map(|row| row.service.clone()).collect::<HashSet<_>>().len();
    let report = RustSiteBranchContract {
        source: "crates/omega-core/src/lib.rs".to_string(),
        total_branches: rows.len(),
        services: services_count,
        pass: !rows.iter().any(|row| row.kind == "missing"),
        branches: rows,
    };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }
    let json_path = "target/rust-site-branch-contract.json";
    let md_path = "target/rust-site-branch-contract.md";
    fs::write(
        json_path,
        serde_json::to_string_pretty(&report).context("serialize rust site branch contract")?,
    )
    .with_context(|| format!("write {json_path}"))?;
    fs::write(md_path, markdown(&report)).with_context(|| format!("write {md_path}"))?;

    println!(
        "rust site branch extract: services={} branches={}",
        report.services, report.total_branches
    );
    if !report.pass {
        bail!("one or more required guild handlers are missing");
    }
    Ok(())
}
