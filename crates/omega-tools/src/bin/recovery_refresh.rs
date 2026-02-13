use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use omega_content::{bootstrap_game_state_from_default_content, legacy_catalogs};
use omega_core::{Command, DeterministicRng, GameState, MapBounds, step};
use serde_json::{Value, json};

const LEGACY_SPELL_COSTS: [i32; 42] = [
    3, 3, 10, 20, 20, 25, 15, 30, 40, 30, 15, 40, 10, 20, 15, 50, 30, 30, 20, 20, 20, 90, 10, 20,
    10, 50, 15, 20, 75, 20, 50, 15, 50, 15, 20, 75, 10, 40, 25, 10, 100, 25,
];

fn stamp() -> String {
    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    format!("unix:{unix}")
}

fn read_json(path: impl AsRef<Path>) -> Result<Value> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))
}

fn write_json(path: impl AsRef<Path>, value: &Value) -> Result<()> {
    let path = path.as_ref();
    fs::write(path, serde_json::to_string_pretty(value).context("serialize json")?)
        .with_context(|| format!("write {}", path.display()))
}

fn bool_field(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn usize_field(value: &Value, key: &str) -> usize {
    value.get(key).and_then(Value::as_u64).and_then(|n| usize::try_from(n).ok()).unwrap_or(0)
}

fn capture_between(raw: &str, marker: char) -> Option<String> {
    let first = raw.find(marker)?;
    let rest = &raw[(first + marker.len_utf8())..];
    let second = rest.find(marker)?;
    Some(rest[..second].to_string())
}

fn capture_effect_note(raw: &str) -> Option<String> {
    let start = raw.rfind('(')?;
    let end = raw.rfind(')')?;
    (end > start + 1).then(|| raw[(start + 1)..end].to_string())
}

fn spell_parity_matrix() -> Value {
    let catalogs = legacy_catalogs();
    let mut state = GameState::new(MapBounds { width: 80, height: 25 });
    state.spellbook.max_mana = 5000;
    state.spellbook.mana = 5000;
    state.spellbook.next_spell_index = 0;
    let mut rng = DeterministicRng::seeded(0x5EED_4242);

    let mut seen_spell_names = HashSet::new();
    let mut seen_effect_notes = HashSet::new();
    let mut all_casts_modeled = true;
    for _ in 0..LEGACY_SPELL_COSTS.len() {
        state.status = omega_core::SessionStatus::InProgress;
        state.player.stats.hp = state.player.stats.max_hp;
        state.monsters.clear();
        let out = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
        let maybe_note = out.events.iter().find_map(|event| {
            if let omega_core::Event::LegacyHandled { token, note, fully_modeled } = event
                && token == "m"
            {
                if !fully_modeled {
                    all_casts_modeled = false;
                }
                return Some(note.clone());
            }
            None
        });
        if let Some(note) = maybe_note {
            if let Some(name) = capture_between(&note, '`') {
                seen_spell_names.insert(name);
            }
            if let Some(effect_note) = capture_effect_note(&note) {
                seen_effect_notes.insert(effect_note);
            }
        } else {
            all_casts_modeled = false;
        }
        state.status = omega_core::SessionStatus::InProgress;
        state.player.stats.hp = state.player.stats.max_hp;
        state.monsters.clear();
    }

    let total_catalog_spells = catalogs.spells.len();
    let no_placeholders = catalogs.spells.iter().all(|entry| {
        !entry.name.starts_with("spell-") && !entry.name.starts_with("legacy-monster-")
    });
    let expected_mana_spend: i32 = LEGACY_SPELL_COSTS.iter().sum();
    let observed_mana_spend = state.spellbook.max_mana - state.spellbook.mana;
    let checks = vec![
        json!({
            "id": "spell_catalog_denominator_42",
            "passed": total_catalog_spells == 42,
            "details": format!("legacy_catalog_spells={total_catalog_spells}"),
        }),
        json!({
            "id": "spell_catalog_no_placeholders",
            "passed": no_placeholders,
            "details": "catalog spell names do not contain placeholder signatures",
        }),
        json!({
            "id": "spell_dispatch_cycles_all_42_names",
            "passed": seen_spell_names.len() == 42 && all_casts_modeled,
            "details": format!("unique_names={} modeled={}", seen_spell_names.len(), all_casts_modeled),
        }),
        json!({
            "id": "spell_mana_cost_profile_matches_legacy_vector",
            "passed": observed_mana_spend == expected_mana_spend,
            "details": format!("observed_spend={observed_mana_spend} expected_spend={expected_mana_spend}"),
        }),
        json!({
            "id": "spell_effect_note_diversity",
            "passed": seen_effect_notes.len() >= 24,
            "details": format!("distinct_effect_notes={}", seen_effect_notes.len()),
        }),
    ];

    let passed = checks.iter().filter(|check| bool_field(check, "passed")).count();
    let failed = checks.len().saturating_sub(passed);
    json!({
        "generated_at_utc": stamp(),
        "total": checks.len(),
        "passed": passed,
        "failed": failed,
        "pass": failed == 0,
        "checks": checks,
    })
}

fn map_matrix(source: &str) -> Result<Value> {
    let matrix = read_json(source)?;
    Ok(json!({
        "generated_at_utc": stamp(),
        "source_artifact": source,
        "pass": bool_field(&matrix, "pass") || bool_field(&matrix, "passed"),
        "matrix": matrix,
    }))
}

fn build_gap_ledger() -> Result<Value> {
    let classic_gap = read_json("target/classic-gap-ledger.json")?;
    let catalogs = legacy_catalogs();

    let placeholder_hits = catalogs
        .spells
        .iter()
        .chain(catalogs.monsters.iter())
        .filter(|entry| {
            let lower = entry.name.to_ascii_lowercase();
            lower.starts_with("spell-")
                || lower.starts_with("monster-")
                || lower.starts_with("legacy-monster-")
        })
        .count();

    let open_p0 = classic_gap["summary"]["open_p0"].as_u64().unwrap_or(0);
    let open_p1 = classic_gap["summary"]["open_p1"].as_u64().unwrap_or(0);
    let total_open = classic_gap["summary"]["total_open"].as_u64().unwrap_or(0);
    let item_families = [
        catalogs.items.scrolls.len(),
        catalogs.items.potions.len(),
        catalogs.items.foods.len(),
        catalogs.items.weapons.len(),
        catalogs.items.armor.len(),
        catalogs.items.shields.len(),
        catalogs.items.cloaks.len(),
        catalogs.items.boots.len(),
        catalogs.items.rings.len(),
        catalogs.items.sticks.len(),
        catalogs.items.artifacts.len(),
    ]
    .into_iter()
    .filter(|count| *count > 0)
    .count();

    let (state, _) = bootstrap_game_state_from_default_content()
        .context("bootstrap default content for recovery gap ledger")?;
    let site_grid_expected = usize::try_from(state.bounds.width.max(0)).unwrap_or(0)
        * usize::try_from(state.bounds.height.max(0)).unwrap_or(0);
    let city_site_grid_ok =
        !state.city_site_grid.is_empty() && state.city_site_grid.len() == site_grid_expected;
    let country_grid_ok = state.country_grid.width > 0
        && state.country_grid.height > 0
        && state.country_grid.cells.len()
            == (usize::try_from(state.country_grid.width.max(0)).unwrap_or(0)
                * usize::try_from(state.country_grid.height.max(0)).unwrap_or(0));

    let mut transition_probe = state.clone();
    let mut rng = DeterministicRng::seeded(0x2222);
    let _ = step(&mut transition_probe, Command::Legacy { token: "<".to_string() }, &mut rng);
    let country_semantic_ok = format!("{:?}", transition_probe.map_binding.semantic) == "Country";
    let country_view_ok = transition_probe.map_rows == transition_probe.country_map_rows;
    let _ = step(&mut transition_probe, Command::Legacy { token: ">".to_string() }, &mut rng);
    let city_semantic_ok = format!("{:?}", transition_probe.map_binding.semantic) == "City";
    let city_view_ok = transition_probe.map_rows == transition_probe.city_map_rows;

    let r1_checks = vec![
        json!({
            "id": "tile_site_grid_loaded_from_city_map",
            "passed": city_site_grid_ok,
            "details": format!("site_grid_cells={} expected={site_grid_expected}", state.city_site_grid.len()),
        }),
        json!({
            "id": "country_grid_loaded_from_country_map",
            "passed": country_grid_ok,
            "details": format!(
                "country_dims={}x{} cells={}",
                state.country_grid.width,
                state.country_grid.height,
                state.country_grid.cells.len()
            ),
        }),
        json!({
            "id": "semantic_map_binding_transitions",
            "passed": country_semantic_ok && city_semantic_ok,
            "details": format!("country_semantic={} city_semantic={}", country_semantic_ok, city_semantic_ok),
        }),
        json!({
            "id": "map_rows_switch_with_semantic_binding",
            "passed": country_view_ok && city_view_ok,
            "details": format!("country_view={} city_view={}", country_view_ok, city_view_ok),
        }),
    ];
    let r1_passed = r1_checks.iter().filter(|check| bool_field(check, "passed")).count();
    let r1_failed = r1_checks.len().saturating_sub(r1_passed);
    let r1_pass = r1_failed == 0;

    Ok(json!({
        "generated_at_utc": stamp(),
        "summary": {
            "open_p0": open_p0,
            "open_p1": open_p1,
            "total_open": total_open,
            "placeholder_signatures": placeholder_hits,
            "r1_checks_total": r1_checks.len(),
            "r1_checks_failed": r1_failed,
        },
        "catalog_cardinality": {
            "spells": catalogs.spells.len(),
            "monsters": catalogs.monsters.len(),
            "traps": catalogs.traps.len(),
            "city_sites": catalogs.city_sites.len(),
            "item_families": item_families,
        },
        "r1_checks": r1_checks,
        "items": classic_gap.get("items").cloned().unwrap_or_else(|| json!([])),
        "pass": open_p0 == 0
            && open_p1 == 0
            && total_open == 0
            && placeholder_hits == 0
            && r1_pass,
    }))
}

fn build_differential_trace_report() -> Result<Value> {
    let replay = read_json("target/ws-d-regression-dashboard.json")?;
    let determinism = read_json("target/ws-d-determinism-report.json")?;
    let replay_failed = replay.get("failed").and_then(Value::as_u64).unwrap_or(1);
    let replay_total = replay.get("total").and_then(Value::as_u64).unwrap_or(0);
    let det_divergent = determinism.get("divergent_runs").and_then(Value::as_u64).unwrap_or(1);
    let det_total_runs = determinism.get("total_runs").and_then(Value::as_u64).unwrap_or(0);
    let pass = replay_failed == 0 && det_divergent == 0;

    Ok(json!({
        "generated_at_utc": stamp(),
        "trace_mode": "fixture_differential_vs_legacy_contract",
        "pass": pass,
        "replay": {
            "total": replay_total,
            "failed": replay_failed,
            "critical_path_total": replay.get("critical_path_total").and_then(Value::as_u64).unwrap_or(0),
            "critical_path_failed": replay.get("critical_path_failed").and_then(Value::as_u64).unwrap_or(0),
        },
        "determinism": {
            "total_runs": det_total_runs,
            "divergent_runs": det_divergent,
            "required_runs_per_fixture": determinism.get("required_runs_per_fixture").and_then(Value::as_u64).unwrap_or(0),
        },
        "sources": [
            "target/ws-d-regression-dashboard.json",
            "target/ws-d-determinism-report.json"
        ]
    }))
}

fn build_baseline_freeze(recovery_artifacts: &[&str]) -> Value {
    let mut artifacts = Vec::new();
    let mut present = 0usize;
    for path in recovery_artifacts {
        let fs_path = PathBuf::from(path);
        let (is_present, size_bytes) = match fs::metadata(&fs_path) {
            Ok(meta) => (true, meta.len()),
            Err(_) => (false, 0),
        };
        if is_present {
            present += 1;
        }
        artifacts.push(json!({
            "path": path,
            "present": is_present,
            "size_bytes": size_bytes,
        }));
    }
    let total = recovery_artifacts.len();
    let missing = total.saturating_sub(present);
    let status = if missing == 0 { "PASS" } else { "FAIL" };
    json!({
        "generated_at_utc": stamp(),
        "status": status,
        "total": total,
        "present": present,
        "missing": missing,
        "artifacts": artifacts,
    })
}

fn main() -> Result<()> {
    fs::create_dir_all("target").context("create target directory")?;

    let gap = build_gap_ledger()?;
    write_json("target/recovery-gap-ledger.json", &gap)?;

    let city_site = map_matrix("target/true-site-economy-social-matrix.json")?;
    write_json("target/recovery-city-site-matrix.json", &city_site)?;

    let overworld = map_matrix("target/true-environment-transition-matrix.json")?;
    write_json("target/recovery-overworld-transition-matrix.json", &overworld)?;

    let spell_matrix = spell_parity_matrix();
    write_json("target/recovery-spell-parity-matrix.json", &spell_matrix)?;

    let item_matrix = map_matrix("target/true-item-parity-matrix.json")?;
    write_json("target/recovery-item-parity-matrix.json", &item_matrix)?;

    let combat_trap = map_matrix("target/true-combat-encounter-matrix.json")?;
    write_json("target/recovery-monster-combat-trap-matrix.json", &combat_trap)?;

    let progression = map_matrix("target/true-progression-ending-matrix.json")?;
    write_json("target/recovery-progression-ending-matrix.json", &progression)?;

    let save_options = map_matrix("target/true-compatibility-matrix.json")?;
    write_json("target/recovery-save-options-wizard-matrix.json", &save_options)?;

    let frontend = map_matrix("target/true-frontend-workflow-matrix.json")?;
    write_json("target/recovery-frontend-equivalence-matrix.json", &frontend)?;

    let differential = build_differential_trace_report()?;
    write_json("target/recovery-differential-trace-report.json", &differential)?;

    let burnin = map_matrix("target/true-burnin-window.json")?;
    write_json("target/recovery-burnin-window.json", &burnin)?;

    let required = [
        "target/recovery-gap-ledger.json",
        "target/recovery-contract-guard.json",
        "target/recovery-rampart-startup-visual.json",
        "target/recovery-city-site-matrix.json",
        "target/recovery-overworld-transition-matrix.json",
        "target/recovery-spell-parity-matrix.json",
        "target/recovery-item-parity-matrix.json",
        "target/recovery-monster-combat-trap-matrix.json",
        "target/recovery-progression-ending-matrix.json",
        "target/recovery-save-options-wizard-matrix.json",
        "target/recovery-frontend-equivalence-matrix.json",
        "target/recovery-differential-trace-report.json",
        "target/recovery-burnin-window.json",
        "docs/migration/FULL_OMEGA_PARITY_RECOVERY_SCORECARD.md",
        "docs/migration/FULL_OMEGA_PARITY_RECOVERY_CLOSURE_REVIEW.md",
    ];
    let freeze = build_baseline_freeze(&required);
    write_json("target/recovery-baseline-freeze.json", &freeze)?;

    let pass = bool_field(&gap, "pass")
        && bool_field(&city_site, "pass")
        && bool_field(&overworld, "pass")
        && bool_field(&spell_matrix, "pass")
        && bool_field(&item_matrix, "pass")
        && bool_field(&combat_trap, "pass")
        && bool_field(&progression, "pass")
        && bool_field(&save_options, "pass")
        && bool_field(&frontend, "pass")
        && bool_field(&differential, "pass")
        && bool_field(&burnin, "pass")
        && freeze.get("status").and_then(Value::as_str) == Some("PASS");

    let summary = json!({
        "generated_at_utc": stamp(),
        "pass": pass,
        "artifacts": {
            "gap_pass": bool_field(&gap, "pass"),
            "city_site_pass": bool_field(&city_site, "pass"),
            "overworld_pass": bool_field(&overworld, "pass"),
            "spell_pass": bool_field(&spell_matrix, "pass"),
            "item_pass": bool_field(&item_matrix, "pass"),
            "combat_trap_pass": bool_field(&combat_trap, "pass"),
            "progression_pass": bool_field(&progression, "pass"),
            "save_options_wizard_pass": bool_field(&save_options, "pass"),
            "frontend_pass": bool_field(&frontend, "pass"),
            "differential_pass": bool_field(&differential, "pass"),
            "burnin_pass": bool_field(&burnin, "pass"),
            "baseline_status": freeze.get("status").and_then(Value::as_str).unwrap_or("FAIL"),
            "spell_checks_passed": usize_field(&spell_matrix, "passed"),
            "spell_checks_total": usize_field(&spell_matrix, "total"),
        }
    });
    write_json("target/recovery-summary.json", &summary)?;

    let status = if pass { "PASS" } else { "FAIL" };
    println!(
        "recovery refresh: status={status}, spell_checks={}/{}, replay_differential_failed={}",
        usize_field(&spell_matrix, "passed"),
        usize_field(&spell_matrix, "total"),
        differential["replay"]["failed"].as_u64().unwrap_or(0)
    );

    if !pass {
        bail!("recovery refresh produced FAIL status");
    }
    Ok(())
}
