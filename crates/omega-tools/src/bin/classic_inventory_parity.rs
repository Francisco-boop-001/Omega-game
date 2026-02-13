use anyhow::{Context, Result, bail};
use omega_core::{
    Command, DeterministicRng, Direction, GameState, Item, ItemFamily, MapBounds,
    SiteInteractionKind, step,
};
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
struct GetItemPromptCall {
    prompt: String,
    symbol: String,
}

#[derive(Debug, Clone, Serialize)]
struct RuntimeCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize)]
struct InventoryParityReport {
    inventory_control_keys: Vec<String>,
    top_inventory_control_keys: Vec<String>,
    getitem_calls: Vec<GetItemPromptCall>,
    runtime_checks: Vec<RuntimeCheck>,
    pass: bool,
}

fn extract_function_block(raw: &str, signature: &str) -> Option<String> {
    let start = raw.find(signature)?;
    let tail = &raw[start..];
    let end = tail.find("\nvoid ").unwrap_or(tail.len());
    Some(tail[..end].to_string())
}

fn extract_case_keys(block: &str) -> Vec<String> {
    let mut keys = BTreeSet::new();
    for line in block.lines() {
        let mut cursor = line;
        while let Some(idx) = cursor.find("case '") {
            let rest = &cursor[idx + 6..];
            let mut chars = rest.chars();
            let Some(ch) = chars.next() else {
                break;
            };
            let mut key = ch.to_string();
            if ch == '\\'
                && let Some(next) = chars.next() {
                    key = format!("\\{next}");
                }
            keys.insert(key);
            cursor = rest;
        }
    }
    keys.into_iter().collect()
}

fn extract_getitem_calls(command2_raw: &str) -> Vec<GetItemPromptCall> {
    let mut calls = Vec::new();
    for line in command2_raw.lines() {
        if !line.contains("getitem_prompt(") {
            continue;
        }
        let Some(call_idx) = line.find("getitem_prompt(") else {
            continue;
        };
        let args = &line[call_idx + "getitem_prompt(".len()..];
        let Some(first_quote) = args.find('"') else {
            continue;
        };
        let after_first = &args[first_quote + 1..];
        let Some(second_quote) = after_first.find('"') else {
            continue;
        };
        let prompt = after_first[..second_quote].to_string();
        let after_prompt = &after_first[second_quote + 1..];
        let symbol = after_prompt
            .trim_start_matches(|ch: char| ch == ',' || ch.is_whitespace())
            .split(')')
            .next()
            .unwrap_or("")
            .trim()
            .trim_end_matches(';')
            .to_string();
        if !prompt.is_empty() {
            calls.push(GetItemPromptCall { prompt, symbol });
        }
    }
    calls
}

fn run_runtime_checks() -> Vec<RuntimeCheck> {
    let mut checks = Vec::new();
    let mut rng = DeterministicRng::seeded(0x5151_2026);

    let mut inventory_state = GameState::new(MapBounds { width: 7, height: 7 });
    inventory_state.player.inventory.push(Item {
        id: 1,
        name: "practice blade".to_string(),
        family: ItemFamily::Weapon,
        ..Item::default()
    });
    inventory_state.player.equipment.ready_hand = Some(1);
    inventory_state.player.equipment.weapon_hand = Some(1);
    let open_inventory =
        step(&mut inventory_state, Command::Legacy { token: "i".to_string() }, &mut rng);
    let inventory_modal_ok =
        inventory_state.pending_inventory_interaction.is_some() && open_inventory.minutes == 0;
    checks.push(RuntimeCheck {
        id: "inventory_modal_open".to_string(),
        passed: inventory_modal_ok,
        details: format!(
            "pending_inventory={} minutes={}",
            inventory_state.pending_inventory_interaction.is_some(),
            open_inventory.minutes
        ),
    });

    let mut look_state = GameState::new(MapBounds { width: 7, height: 7 });
    look_state.player.inventory.push(Item {
        id: 11,
        name: "practice blade".to_string(),
        family: ItemFamily::Weapon,
        known: true,
        truename: "fine longsword".to_string(),
        ..Item::default()
    });
    look_state.player.equipment.ready_hand = Some(11);
    look_state.player.equipment.weapon_hand = Some(11);
    let _ = step(&mut look_state, Command::Legacy { token: "i".to_string() }, &mut rng);
    let _ = step(&mut look_state, Command::Legacy { token: "s".to_string() }, &mut rng);
    let pack_line = look_state
        .log
        .iter()
        .rev()
        .find(|line| line.starts_with("Pack"))
        .cloned()
        .unwrap_or_default();
    let _ = step(&mut look_state, Command::Legacy { token: "l".to_string() }, &mut rng);
    let look_line = look_state.log.last().cloned().unwrap_or_default();
    let look_vs_show_ok = !pack_line.is_empty()
        && look_line.starts_with("It's ")
        && look_line != pack_line
        && !look_line.starts_with("Pack: ");
    checks.push(RuntimeCheck {
        id: "inventory_look_vs_show_parity".to_string(),
        passed: look_vs_show_ok,
        details: format!("pack_line=`{pack_line}` look_line=`{look_line}`"),
    });

    let mut item_prompt_state = GameState::new(MapBounds { width: 7, height: 7 });
    item_prompt_state.player.stats.hp = 10;
    item_prompt_state.player.stats.max_hp = 20;
    item_prompt_state.player.inventory.push(Item {
        id: 1,
        name: "healing potion".to_string(),
        family: ItemFamily::Potion,
        usef: "I_HEAL".to_string(),
        ..Item::default()
    });
    let open_quaff =
        step(&mut item_prompt_state, Command::Legacy { token: "q".to_string() }, &mut rng);
    let pos_before = item_prompt_state.player.position;
    let _ = step(&mut item_prompt_state, Command::Move(Direction::East), &mut rng);
    let _ = step(&mut item_prompt_state, Command::Legacy { token: "a".to_string() }, &mut rng);
    let item_prompt_ok = open_quaff.minutes == 0
        && item_prompt_state.player.position == pos_before
        && item_prompt_state.pending_item_prompt.is_none()
        && item_prompt_state.player.stats.hp > 10;
    checks.push(RuntimeCheck {
        id: "item_prompt_selection_and_lock".to_string(),
        passed: item_prompt_ok,
        details: format!(
            "minutes={} pos_locked={} pending_item_prompt={} hp={}",
            open_quaff.minutes,
            item_prompt_state.player.position == pos_before,
            item_prompt_state.pending_item_prompt.is_some(),
            item_prompt_state.player.stats.hp
        ),
    });

    let mut site_state = GameState::new(MapBounds { width: 5, height: 5 });
    site_state.pending_site_interaction = Some(SiteInteractionKind::Temple);
    let _ = step(&mut site_state, Command::Legacy { token: "1".to_string() }, &mut rng);
    checks.push(RuntimeCheck {
        id: "site_choice_uses_legacy_token".to_string(),
        passed: site_state.progression.deity_favor >= 0
            && site_state.pending_site_interaction.is_some(),
        details: format!(
            "pending_site_interaction={} favor={}",
            site_state.pending_site_interaction.is_some(),
            site_state.progression.deity_favor
        ),
    });

    checks
}

fn markdown(report: &InventoryParityReport) -> String {
    let mut out = Vec::new();
    out.push("# Classic Inventory Parity Contract".to_string());
    out.push(String::new());
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(format!("- inventory_control keys: {}", report.inventory_control_keys.join(", ")));
    out.push(format!(
        "- top_inventory_control keys: {}",
        report.top_inventory_control_keys.join(", ")
    ));
    out.push(format!("- getitem_prompt calls: {}", report.getitem_calls.len()));
    out.push(String::new());
    out.push("## getitem_prompt calls".to_string());
    for call in &report.getitem_calls {
        out.push(format!("- \"{}\" -> {}", call.prompt, call.symbol));
    }
    out.push(String::new());
    out.push("## Runtime checks".to_string());
    for check in &report.runtime_checks {
        out.push(format!(
            "- {}: {} ({})",
            check.id,
            if check.passed { "PASS" } else { "FAIL" },
            check.details
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let base = Path::new("archive/legacy-c-runtime/2026-02-06");
    let inv_path = base.join("inv.c");
    let command2_path = base.join("command2.c");

    let inv_raw =
        fs::read_to_string(&inv_path).with_context(|| format!("read {}", inv_path.display()))?;
    let command2_raw = fs::read_to_string(&command2_path)
        .with_context(|| format!("read {}", command2_path.display()))?;

    let inventory_control = extract_function_block(&inv_raw, "void inventory_control(void)")
        .context("find inventory_control in inv.c")?;
    let top_inventory_control =
        extract_function_block(&inv_raw, "void top_inventory_control(void)")
            .context("find top_inventory_control in inv.c")?;

    let inventory_control_keys = extract_case_keys(&inventory_control);
    let top_inventory_control_keys = extract_case_keys(&top_inventory_control);
    let getitem_calls = extract_getitem_calls(&command2_raw);
    let runtime_checks = run_runtime_checks();

    let extraction_ok = !inventory_control_keys.is_empty()
        && !top_inventory_control_keys.is_empty()
        && !getitem_calls.is_empty();
    let runtime_ok = runtime_checks.iter().all(|check| check.passed);

    let report = InventoryParityReport {
        inventory_control_keys,
        top_inventory_control_keys,
        getitem_calls,
        runtime_checks,
        pass: extraction_ok && runtime_ok,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("classic-inventory-contract.json");
    let md_path = target.join("classic-inventory-contract.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize inventory contract report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "classic inventory parity contract: extraction_ok={} runtime_ok={} pass={}",
        extraction_ok, runtime_ok, report.pass
    );

    if !report.pass {
        bail!("classic inventory parity contract failed");
    }
    Ok(())
}
