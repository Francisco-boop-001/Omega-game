use anyhow::{Context, Result, bail};
use omega_content::{bootstrap_game_state_from_default_content, legacy_catalogs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GuardCheck {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GuardReport {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    status: String,
    checks: Vec<GuardCheck>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn read_text(path: &str) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("read {path}"))
}

fn normalize_newlines(raw: &str) -> String {
    raw.replace("\r\n", "\n")
}

fn markdown(report: &GuardReport) -> String {
    let mut out = Vec::new();
    out.push("# Recovery Contract Guard".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", report.generated_at_utc));
    out.push(format!("- Status: {}", report.status));
    out.push(format!("- Checks: {}/{} pass", report.passed, report.total));
    out.push(String::new());
    out.push("| Check | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for check in &report.checks {
        out.push(format!(
            "| {} | {} | {} |",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn is_placeholder_name(name: &str) -> bool {
    let prefixes = [
        "spell-",
        "monster-",
        "trap-",
        "city-site-",
        "scroll-",
        "potion-",
        "food-",
        "weapon-",
        "armor-",
        "shield-",
        "cloak-",
        "boots-",
        "ring-",
        "stick-",
        "artifact-",
    ];
    prefixes.iter().any(|prefix| name.starts_with(prefix))
}

fn gather_placeholder_catalog_names() -> Vec<String> {
    let catalogs = legacy_catalogs();
    let mut names = Vec::new();

    names.extend(catalogs.spells.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.monsters.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.traps.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.city_sites.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.scrolls.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.potions.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.foods.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.weapons.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.armor.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.shields.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.cloaks.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.boots.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.rings.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.sticks.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.artifacts.iter().map(|entry| entry.name.clone()));

    names.into_iter().filter(|name| is_placeholder_name(name)).collect()
}

fn tui_synthetic_fill_present(source: &str) -> bool {
    let normalized = normalize_newlines(source);
    normalized.contains("} else {\n                '.'\n            };")
}

fn bevy_floor_fill_present(source: &str) -> bool {
    let normalized = normalize_newlines(source);
    normalized.contains("for y in 0..state.bounds.height")
        && normalized.contains("for x in 0..state.bounds.width")
        && normalized.contains("kind: TileKind::Floor")
}

fn main() -> Result<()> {
    let mut checks = Vec::new();

    let placeholder_catalog_names = gather_placeholder_catalog_names();
    let catalog_pass = placeholder_catalog_names.is_empty();
    checks.push(GuardCheck {
        id: "catalogs_no_placeholder_entries".to_string(),
        pass: catalog_pass,
        details: if catalog_pass {
            "no placeholder-prefixed catalog entries detected".to_string()
        } else {
            let sample = placeholder_catalog_names.iter().take(8).cloned().collect::<Vec<_>>();
            format!(
                "placeholder entries detected (count={}): {}",
                placeholder_catalog_names.len(),
                sample.join(", ")
            )
        },
    });

    let bootstrap_check = match bootstrap_game_state_from_default_content() {
        Ok((state, diagnostics)) => {
            let placeholder_monsters = state
                .monsters
                .iter()
                .filter(|monster| {
                    monster.name.starts_with("legacy-monster-")
                        || monster.name.starts_with("legacy-guardian")
                })
                .map(|monster| monster.name.clone())
                .collect::<Vec<_>>();
            let placeholder_items = state
                .ground_items
                .iter()
                .filter(|item| item.item.name.starts_with("legacy-item-"))
                .map(|item| item.item.name.clone())
                .collect::<Vec<_>>();
            let pass = placeholder_monsters.is_empty() && placeholder_items.is_empty();
            let details = if pass {
                format!(
                    "bootstrap source={} spawn={} monsters={} items={}",
                    diagnostics.map_source,
                    diagnostics.player_spawn_source,
                    diagnostics.monster_spawns,
                    diagnostics.item_spawns
                )
            } else {
                let m = placeholder_monsters.into_iter().take(4).collect::<Vec<_>>();
                let i = placeholder_items.into_iter().take(4).collect::<Vec<_>>();
                format!(
                    "placeholder bootstrap entities detected monsters=[{}] items=[{}]",
                    m.join(", "),
                    i.join(", ")
                )
            };
            GuardCheck { id: "bootstrap_no_placeholder_entities".to_string(), pass, details }
        }
        Err(err) => GuardCheck {
            id: "bootstrap_no_placeholder_entities".to_string(),
            pass: false,
            details: format!("bootstrap load failed: {err}"),
        },
    };
    checks.push(bootstrap_check);

    let tui_source = read_text("crates/omega-tui/src/lib.rs")?;
    checks.push(GuardCheck {
        id: "tui_renderer_not_synthetic_dot_fill".to_string(),
        pass: !tui_synthetic_fill_present(&tui_source),
        details: if tui_synthetic_fill_present(&tui_source) {
            "synthetic dot-fill renderer signature detected in map panel".to_string()
        } else {
            "no dot-fill renderer signature detected".to_string()
        },
    });

    let bevy_source = read_text("crates/omega-bevy/src/lib.rs")?;
    checks.push(GuardCheck {
        id: "bevy_renderer_not_floor_fill_projection".to_string(),
        pass: !bevy_floor_fill_present(&bevy_source),
        details: if bevy_floor_fill_present(&bevy_source) {
            "full-bounds floor-fill projection signature detected".to_string()
        } else {
            "no full-bounds floor-fill projection signature detected".to_string()
        },
    });

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = GuardReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        status: if failed == 0 { "PASS".to_string() } else { "FAIL".to_string() },
        checks,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("recovery-contract-guard.json");
    let md_path = target.join("recovery-contract-guard.md");
    fs::write(&json_path, serde_json::to_string_pretty(&report).context("serialize guard report")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "recovery contract guard: status={}, passed={}/{}",
        report.status, report.passed, report.total
    );

    if failed > 0 {
        bail!("recovery contract guard failed");
    }
    Ok(())
}
