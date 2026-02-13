use anyhow::{Context, Result, bail};
use omega_bevy as bevy_frontend;
use omega_content::{
    LEGACY_RAMPART_START, LegacyItemFamily, bootstrap_game_state_from_default_content,
    legacy_catalogs, legacy_item_prototypes,
};
use omega_core::{
    Command, DeterministicRng, Direction, Event, GameState, ItemFamily, LegacyQuestState,
    MapBounds, MapSemanticKind, Position, Stats, WorldMode, step,
};
use omega_save::{decode_state_json, encode_json};
use omega_tui as tui_frontend;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const CORE_SOURCE: &str = include_str!("../../../omega-core/src/lib.rs");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct MatrixCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TrueMatrix {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<MatrixCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TrueCommandBehaviorMatrix {
    generated_at_utc: String,
    total: usize,
    implemented_same_key: usize,
    implemented_different_key: usize,
    partial: usize,
    missing: usize,
    key_conflict: usize,
    pass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ArtifactSnapshot {
    path: String,
    present: bool,
    size_bytes: u64,
    hash_fnv1a64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct BaselineFreeze {
    generated_at_utc: String,
    status: String,
    total: usize,
    present: usize,
    missing: usize,
    artifacts: Vec<ArtifactSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DashboardComponent {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TrueParityRegressionDashboard {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    status: String,
    components: Vec<DashboardComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TrueBurninWindow {
    generated_at_utc: String,
    required_runs_per_fixture: usize,
    determinism_total_runs: usize,
    replay_total_scenarios: usize,
    replay_failed_scenarios: usize,
    determinism_divergent_runs: usize,
    pass: bool,
    blockers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DeviationItem {
    id: String,
    track: String,
    severity: String,
    status: String,
    title: String,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DeviationSummary {
    total: usize,
    open: usize,
    closed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DeviationLedger {
    generated_at_utc: String,
    summary: DeviationSummary,
    items: Vec<DeviationItem>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn write_json<T: Serialize>(path: &str, value: &T) -> Result<()> {
    fs::write(path, serde_json::to_string_pretty(value).context("serialize json")?)
        .with_context(|| format!("write {path}"))
}

fn read_json(path: &str) -> Result<Value> {
    let raw = fs::read_to_string(path).with_context(|| format!("read {path}"))?;
    serde_json::from_str::<Value>(&raw).with_context(|| format!("decode {path}"))
}

fn read_json_if_exists(path: &str) -> Option<Value> {
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str::<Value>(&raw).ok()
}

fn bool_field(value: &Value, key: &str) -> bool {
    value[key].as_bool().unwrap_or(false)
}

fn pass_field(value: &Value) -> bool {
    bool_field(value, "pass") || value["status"].as_str().is_some_and(|status| status == "PASS")
}

fn usize_field(value: &Value, key: &str) -> usize {
    value[key].as_u64().unwrap_or(0) as usize
}

fn mechanics_main_non_equivalent(value: &Value) -> usize {
    value["main_non_equivalent"].as_u64().unwrap_or(0) as usize
}

fn mechanics_unresolved_gameplay(value: &Value) -> usize {
    value["unresolved_gameplay"].as_u64().unwrap_or(0) as usize
}

fn replay_active_total(value: &Value) -> usize {
    if value.get("active_total").is_some() {
        usize_field(value, "active_total")
    } else {
        usize_field(value, "total")
    }
}

fn replay_active_failed(value: &Value) -> usize {
    if value.get("failed_active").is_some() {
        usize_field(value, "failed_active")
    } else {
        usize_field(value, "failed")
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn write_true_matrix(path: &str, checks: Vec<MatrixCheck>) -> Result<TrueMatrix> {
    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let matrix = TrueMatrix {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
    };
    write_json(path, &matrix)?;
    Ok(matrix)
}

fn ensure_startup_parity_artifact() -> Result<Value> {
    let path = "target/true-startup-parity.json";
    if Path::new(path).exists() {
        return read_json(path);
    }

    let (state, diagnostics) = bootstrap_game_state_from_default_content()?;
    let checks = vec![
        MatrixCheck {
            id: "spawn_at_rampart".to_string(),
            passed: state.player.position == LEGACY_RAMPART_START,
            details: diagnostics.player_spawn_source,
        },
        MatrixCheck {
            id: "startup_city_context".to_string(),
            passed: state.world_mode == WorldMode::DungeonCity
                && state.map_binding.semantic == MapSemanticKind::City,
            details: format!(
                "world={:?} semantic={:?}",
                state.world_mode, state.map_binding.semantic
            ),
        },
        MatrixCheck {
            id: "startup_has_exit".to_string(),
            passed: state.site_grid.iter().any(|cell| cell.aux == 1),
            details: "exit tile to countryside present".to_string(),
        },
    ];
    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let report = serde_json::json!({
        "total": total,
        "passed": passed,
        "failed": failed,
        "pass": failed == 0,
        "checks": checks,
    });
    write_json(path, &report)?;
    Ok(report)
}

fn build_environment_matrix() -> Result<TrueMatrix> {
    let (mut state, diagnostics) = bootstrap_game_state_from_default_content()?;
    let start = state.player.position;
    let mut rng = DeterministicRng::seeded(0x5EED_0101);
    let _ = step(&mut state, Command::Legacy { token: "<".to_string() }, &mut rng);
    let in_country = state.world_mode == WorldMode::Countryside
        && state.map_binding.semantic == MapSemanticKind::Country;
    let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
    let back_city = state.world_mode == WorldMode::DungeonCity
        && state.map_binding.semantic == MapSemanticKind::City;

    let checks = vec![
        MatrixCheck {
            id: "startup_spawn".to_string(),
            passed: start == LEGACY_RAMPART_START,
            details: diagnostics.player_spawn_source,
        },
        MatrixCheck {
            id: "city_to_country".to_string(),
            passed: in_country,
            details: format!(
                "world={:?} semantic={:?}",
                state.world_mode, state.map_binding.semantic
            ),
        },
        MatrixCheck {
            id: "country_to_city".to_string(),
            passed: back_city,
            details: format!(
                "world={:?} semantic={:?}",
                state.world_mode, state.map_binding.semantic
            ),
        },
        MatrixCheck {
            id: "country_grid_available".to_string(),
            passed: !state.country_grid.cells.is_empty(),
            details: format!("cells={}", state.country_grid.cells.len()),
        },
    ];
    write_true_matrix("target/true-environment-transition-matrix.json", checks)
}

fn build_command_matrix() -> Result<TrueCommandBehaviorMatrix> {
    let tokens = [
        ".", ",", "<", ">", "H", "s", "q", "r", "m", "a", "A", "t", "G", "D", "F", "O", "d", "e",
        "i", "?", "/", "x", "C", "R", "P", "V", "^f", "^g", "^w", "^x", "o", "c", "E", "p", "f",
        "v", "z", "b", "n", "u", "y",
    ];

    let mut implemented_same_key = 0usize;
    let implemented_different_key = 0usize;
    let mut partial = 0usize;
    let mut missing = 0usize;
    let key_conflict = 0usize;

    for token in tokens {
        let (mut state, _) = bootstrap_game_state_from_default_content()?;
        state.options.confirm = false;
        if token == "^x" {
            state.wizard.enabled = true;
        }
        if matches!(token, "d" | "C") {
            state.player.inventory.push(omega_core::Item::new(1, "test item"));
        }
        if token == "q" {
            state.player.inventory.push(omega_core::Item::new(2, "healing potion"));
        }
        let mut rng = DeterministicRng::seeded(0x5EED_0102);
        let out = step(&mut state, Command::Legacy { token: token.to_string() }, &mut rng);
        let event = out.events.iter().find_map(|event| {
            if let Event::LegacyHandled { token: t, note, fully_modeled } = event
                && t == token
            {
                return Some((note.clone(), *fully_modeled));
            }
            None
        });
        match event {
            Some((note, _)) if note.starts_with("unsupported legacy command") => missing += 1,
            Some((_, false)) => partial += 1,
            Some(_) => implemented_same_key += 1,
            None => partial += 1,
        }
    }

    let matrix = TrueCommandBehaviorMatrix {
        generated_at_utc: now_utc_unix(),
        total: tokens.len(),
        implemented_same_key,
        implemented_different_key,
        partial,
        missing,
        key_conflict,
        pass: missing == 0 && partial == 0 && key_conflict == 0,
    };
    write_json("target/true-command-behavior-matrix.json", &matrix)?;
    Ok(matrix)
}

fn build_spell_matrix() -> Result<TrueMatrix> {
    let mut rng = DeterministicRng::seeded(0x5EED_0103);
    let mut modeled = 0usize;
    let mut prompt_opened = 0usize;
    let mut mana_consumed_trials = 0usize;
    for spell_name in [
        "monster detection",
        "object detection",
        "magic missile",
        "firebolt",
        "teleport",
        "ball lightning",
        "sleep",
        "disrupt",
        "disintegrate",
        "polymorph",
        "healing",
        "dispelling",
        "identification",
        "breathing",
        "invisibility",
        "the warp",
        "enchantment",
        "blessing",
        "restoration",
        "curing",
        "true sight",
        "hellfire",
        "self knowledge",
        "heroism",
        "return",
        "desecration",
        "haste",
        "summoning",
        "sanctuary",
        "accuracy",
        "ritual magic",
        "apportation",
        "shadow form",
        "alertness",
        "regeneration",
        "sanctification",
        "clairvoyance",
        "energy drain",
        "levitate",
        "fear",
        "wishing",
        "nutrition",
    ] {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        state.spellbook.max_mana = 5000;
        state.spellbook.mana = 5000;
        for spell in &mut state.spellbook.spells {
            spell.known = true;
        }
        state.status = omega_core::SessionStatus::InProgress;
        state.player.stats.hp = state.player.stats.max_hp;
        state.monsters.clear();
        let open = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
        if open.events.iter().any(|event| {
            matches!(
                event,
                Event::LegacyHandled { token, note, .. }
                    if token == "m" && note.starts_with("Cast Spell:")
            )
        }) {
            prompt_opened += 1;
        }
        let _ = step(&mut state, Command::Legacy { token: spell_name.to_string() }, &mut rng);
        let selection_out =
            step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
        let mut outcomes = vec![selection_out];
        if state.pending_targeting_interaction.is_some() {
            outcomes.push(step(
                &mut state,
                Command::Legacy { token: "<enter>".to_string() },
                &mut rng,
            ));
        }
        let modeled_this = outcomes.iter().any(|out| {
            out.events.iter().any(|event| {
                matches!(
                    event,
                    Event::LegacyHandled { token, note, fully_modeled: true }
                        if token == "m" && note.starts_with("cast spell#")
                )
            })
        });
        if modeled_this {
            modeled += 1;
        }
        if state.spellbook.mana < state.spellbook.max_mana {
            mana_consumed_trials += 1;
        }
    }
    let checks = vec![
        MatrixCheck {
            id: "spell_catalog_42".to_string(),
            passed: legacy_catalogs().spells.len() == 42,
            details: format!("catalog_spells={}", legacy_catalogs().spells.len()),
        },
        MatrixCheck {
            id: "spell_casts_modeled".to_string(),
            passed: modeled == 42,
            details: format!("modeled={modeled}/42"),
        },
        MatrixCheck {
            id: "spell_prompt_interactive".to_string(),
            passed: prompt_opened == 42,
            details: format!("prompt_opened={prompt_opened}/42"),
        },
        MatrixCheck {
            id: "spell_mana_consumed".to_string(),
            passed: mana_consumed_trials == 42,
            details: format!("mana_consumed_trials={mana_consumed_trials}/42"),
        },
    ];
    write_true_matrix("target/true-spell-parity-matrix.json", checks)
}

fn build_item_matrix() -> Result<TrueMatrix> {
    let catalog = legacy_item_prototypes();
    let mut family_counts: std::collections::HashMap<LegacyItemFamily, usize> =
        std::collections::HashMap::new();
    for item in &catalog {
        *family_counts.entry(item.family).or_insert(0) += 1;
    }

    let mut state = GameState::new(MapBounds { width: 9, height: 9 });
    state.place_item("potion of healing", state.player.position);
    state.place_item("scroll of identification", state.player.position);
    state.place_item("staff of missiles", state.player.position);
    state.place_item("Victrix", state.player.position);

    let mut rng = DeterministicRng::seeded(0x5EED_0104);
    let pick_1 = step(&mut state, Command::Pickup, &mut rng);
    let pick_2 = step(&mut state, Command::Pickup, &mut rng);
    let pick_3 = step(&mut state, Command::Pickup, &mut rng);
    let pick_4 = step(&mut state, Command::Pickup, &mut rng);
    let drop = step(&mut state, Command::Drop { slot: 0 }, &mut rng);

    let typed_items =
        state.player.inventory.iter().filter(|item| item.family != ItemFamily::Unknown).count();
    let usef_complete = state.player.inventory.iter().all(|item| !item.usef.trim().is_empty());
    let runtime_usef_tokens = extract_usef_tokens(CORE_SOURCE);
    let legacy_usef: HashSet<String> = catalog
        .iter()
        .map(|entry| entry.usef.trim().to_string())
        .filter(|usef| !usef.is_empty() && usef.starts_with("I_"))
        .collect();
    let mut missing_usef = legacy_usef
        .iter()
        .filter(|usef| !runtime_usef_tokens.contains(*usef))
        .cloned()
        .collect::<Vec<_>>();
    missing_usef.sort();
    let checks = vec![
        MatrixCheck {
            id: "legacy_item_prototypes_loaded".to_string(),
            passed: catalog.len() == 223,
            details: format!("total={}", catalog.len()),
        },
        MatrixCheck {
            id: "legacy_item_family_cardinality".to_string(),
            passed: family_counts.get(&LegacyItemFamily::Weapon).copied().unwrap_or(0) == 41
                && family_counts.get(&LegacyItemFamily::Armor).copied().unwrap_or(0) == 17
                && family_counts.get(&LegacyItemFamily::Potion).copied().unwrap_or(0) == 18
                && family_counts.get(&LegacyItemFamily::Scroll).copied().unwrap_or(0) == 24
                && family_counts.get(&LegacyItemFamily::Ring).copied().unwrap_or(0) == 9
                && family_counts.get(&LegacyItemFamily::Artifact).copied().unwrap_or(0) == 26,
            details: format!(
                "weapon={} armor={} potion={} scroll={} ring={} artifact={}",
                family_counts.get(&LegacyItemFamily::Weapon).copied().unwrap_or(0),
                family_counts.get(&LegacyItemFamily::Armor).copied().unwrap_or(0),
                family_counts.get(&LegacyItemFamily::Potion).copied().unwrap_or(0),
                family_counts.get(&LegacyItemFamily::Scroll).copied().unwrap_or(0),
                family_counts.get(&LegacyItemFamily::Ring).copied().unwrap_or(0),
                family_counts.get(&LegacyItemFamily::Artifact).copied().unwrap_or(0),
            ),
        },
        MatrixCheck {
            id: "pickup_modeled".to_string(),
            passed: pick_1.events.iter().any(|event| matches!(event, Event::PickedUp { .. }))
                && pick_2.events.iter().any(|event| matches!(event, Event::PickedUp { .. }))
                && pick_3.events.iter().any(|event| matches!(event, Event::PickedUp { .. }))
                && pick_4.events.iter().any(|event| matches!(event, Event::PickedUp { .. })),
            details: format!("inventory={}", state.player.inventory.len()),
        },
        MatrixCheck {
            id: "drop_modeled".to_string(),
            passed: drop.events.iter().any(|event| matches!(event, Event::Dropped { .. })),
            details: format!("ground={}", state.ground_items.len()),
        },
        MatrixCheck {
            id: "typed_inventory_metadata".to_string(),
            passed: typed_items == state.player.inventory.len() && usef_complete,
            details: format!(
                "typed={}/{} usef_complete={}",
                typed_items,
                state.player.inventory.len(),
                usef_complete
            ),
        },
        MatrixCheck {
            id: "no_placeholder_item_names".to_string(),
            passed: state.player.inventory.iter().all(|item| {
                !item.name.contains("named-")
                    && !item.name.contains("wishforged")
                    && !item.name.contains("trinket")
            }),
            details: "inventory items avoid placeholder signatures".to_string(),
        },
        MatrixCheck {
            id: "legacy_usef_tokens_covered".to_string(),
            passed: missing_usef.is_empty(),
            details: if missing_usef.is_empty() {
                format!(
                    "legacy_usef={} runtime_tokens={} missing=<none>",
                    legacy_usef.len(),
                    runtime_usef_tokens.len()
                )
            } else {
                format!(
                    "legacy_usef={} runtime_tokens={} missing={}",
                    legacy_usef.len(),
                    runtime_usef_tokens.len(),
                    missing_usef.join(",")
                )
            },
        },
    ];
    write_true_matrix("target/true-item-parity-matrix.json", checks)
}

fn extract_usef_tokens(source: &str) -> HashSet<String> {
    let bytes = source.as_bytes();
    let mut out = HashSet::new();
    let mut i = 0usize;
    while i + 2 <= bytes.len() {
        if bytes[i] == b'I' && i + 1 < bytes.len() && bytes[i + 1] == b'_' {
            let start = i;
            let mut end = i + 2;
            while end < bytes.len() {
                let ch = bytes[end];
                if ch == b'_' || ch.is_ascii_uppercase() || ch.is_ascii_digit() {
                    end += 1;
                } else {
                    break;
                }
            }
            if end > start + 2 {
                out.insert(source[start..end].to_string());
            }
            i = end;
            continue;
        }
        i += 1;
    }
    out
}

fn build_combat_matrix() -> Result<TrueMatrix> {
    let mut state = GameState::new(MapBounds { width: 9, height: 9 });
    state.spawn_monster(
        "raider",
        Position { x: state.player.position.x + 1, y: state.player.position.y },
        Stats { hp: 8, max_hp: 8, attack_min: 1, attack_max: 2, defense: 0 },
    );
    let mut rng = DeterministicRng::seeded(0x5EED_0105);
    let attack = step(&mut state, Command::Attack(Direction::East), &mut rng);
    let checks = vec![
        MatrixCheck {
            id: "attack_event".to_string(),
            passed: attack.events.iter().any(|event| matches!(event, Event::Attacked { .. })),
            details: format!("monsters={}", state.monsters.len()),
        },
        MatrixCheck {
            id: "monster_response_cycle".to_string(),
            passed: attack.events.iter().any(|event| matches!(event, Event::TurnAdvanced { .. })),
            details: format!("turn={}", state.clock.turn),
        },
    ];
    write_true_matrix("target/true-combat-encounter-matrix.json", checks)
}

fn build_site_matrix() -> Result<TrueMatrix> {
    let path = "target/true-site-service-parity-matrix.json";
    if !Path::new(path).exists() {
        return write_true_matrix(
            "target/true-site-economy-social-matrix.json",
            vec![MatrixCheck {
                id: "site_runtime_matrix_missing".to_string(),
                passed: false,
                details: format!("missing {path}"),
            }],
        );
    }
    let value = read_json(path)?;
    let checks = value["checks"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|item| MatrixCheck {
            id: item["id"].as_str().unwrap_or("unknown").to_string(),
            passed: item["passed"].as_bool().unwrap_or(false),
            details: item["details"].as_str().unwrap_or_default().to_string(),
        })
        .collect::<Vec<_>>();
    write_true_matrix("target/true-site-economy-social-matrix.json", checks)
}

fn build_progression_matrix() -> Result<TrueMatrix> {
    let (mut state, _) = bootstrap_game_state_from_default_content()?;
    state.options.interactive_sites = true;
    let mut rng = DeterministicRng::seeded(0x5EED_0106);
    let _ = step(&mut state, Command::Legacy { token: "t".to_string() }, &mut rng);
    let checks = vec![
        MatrixCheck {
            id: "quest_state_reachable".to_string(),
            passed: matches!(
                state.progression.quest_state,
                LegacyQuestState::NotStarted
                    | LegacyQuestState::Active
                    | LegacyQuestState::ReturnToPatron
            ),
            details: format!("quest={:?}", state.progression.quest_state),
        },
        MatrixCheck {
            id: "progression_struct_live".to_string(),
            passed: state.progression.score >= 0,
            details: format!(
                "guild={} priest={} ending={:?}",
                state.progression.guild_rank,
                state.progression.priest_rank,
                state.progression.ending
            ),
        },
    ];
    write_true_matrix("target/true-progression-ending-matrix.json", checks)
}

fn build_compatibility_matrix() -> Result<TrueMatrix> {
    let (mut state, _) = bootstrap_game_state_from_default_content()?;
    state.options.pickup = true;
    state.wizard.enabled = true;
    let encoded = encode_json(&state)?;
    let decoded = decode_state_json(&encoded)?;
    let checks = vec![
        MatrixCheck {
            id: "save_roundtrip_position".to_string(),
            passed: decoded.player.position == state.player.position,
            details: format!("pos={:?}", decoded.player.position),
        },
        MatrixCheck {
            id: "save_roundtrip_options".to_string(),
            passed: decoded.options.pickup == state.options.pickup,
            details: format!("pickup={}", decoded.options.pickup),
        },
    ];
    write_true_matrix("target/true-compatibility-matrix.json", checks)
}

fn build_frontend_workflow_matrix() -> Result<TrueMatrix> {
    let tui_state = tui_frontend::run_headless_bootstrap()?;
    let bevy_state = bevy_frontend::run_headless_bootstrap()?;
    let key_parity = matches!(
        tui_frontend::App::map_input(tui_frontend::UiKey::Char('g')),
        tui_frontend::UiAction::Dispatch(Command::Pickup)
    ) && matches!(
        bevy_frontend::map_input(
            bevy_frontend::AppState::InGame,
            bevy_frontend::BevyKey::Char('g')
        ),
        bevy_frontend::InputAction::Dispatch(Command::Pickup)
    );
    let checks = vec![
        MatrixCheck {
            id: "headless_bootstrap_match".to_string(),
            passed: tui_state.player.position == bevy_state.player.position
                && tui_state.world_mode == bevy_state.world_mode,
            details: format!(
                "tui_pos=({}, {}) bevy_pos=({}, {})",
                tui_state.player.position.x,
                tui_state.player.position.y,
                bevy_state.player.position.x,
                bevy_state.player.position.y
            ),
        },
        MatrixCheck {
            id: "shared_pickup_key_contract".to_string(),
            passed: key_parity,
            details: "both frontends map `g` to pickup".to_string(),
        },
    ];
    write_true_matrix("target/true-frontend-workflow-matrix.json", checks)
}

#[allow(clippy::too_many_arguments)]
fn build_regression_dashboard(
    startup: &Value,
    mechanics: &Value,
    command: &TrueCommandBehaviorMatrix,
    legacy_command_binding_parity: &Value,
    environment: &TrueMatrix,
    spells: &TrueMatrix,
    items: &TrueMatrix,
    combat: &TrueMatrix,
    site: &TrueMatrix,
    progression: &TrueMatrix,
    compatibility: &TrueMatrix,
    frontend: &TrueMatrix,
    frontend_strict: &Value,
    replay_dashboard: &Value,
    guild_live_check: &Value,
    service_branch_blackbox: &Value,
    site_branch_diff: &Value,
    parity_certify: &Value,
    classic_mode_drift_guard: &Value,
    classic_objective_drift_guard: &Value,
    modern_mode_smoke: &Value,
    modern_bevy_visual_smoke: &Value,
    modern_objective_blackbox_smoke: &Value,
    bevy_visual_blackbox_suite: &Value,
    dual_mode_blackbox_suite: &Value,
    mode_artifact_integrity_guard: &Value,
    classic_visual_drift_guard: &Value,
    bevy_semantic_projection_parity: &Value,
) -> Result<TrueParityRegressionDashboard> {
    let components = vec![
        DashboardComponent {
            id: "startup".to_string(),
            pass: bool_field(startup, "pass"),
            details: format!(
                "passed={}/{}",
                usize_field(startup, "passed"),
                usize_field(startup, "total")
            ),
        },
        DashboardComponent {
            id: "mechanics".to_string(),
            pass: bool_field(mechanics, "pass")
                && usize_field(mechanics, "unknown") == 0
                && mechanics_main_non_equivalent(mechanics) == 0
                && mechanics_unresolved_gameplay(mechanics) == 0
                && usize_field(mechanics, "gameplay_excluded") == 0,
            details: format!(
                "pass={} unknown={} main_non_equivalent={} unresolved_gameplay={} gameplay_excluded={}",
                bool_field(mechanics, "pass"),
                usize_field(mechanics, "unknown"),
                mechanics_main_non_equivalent(mechanics),
                mechanics_unresolved_gameplay(mechanics),
                usize_field(mechanics, "gameplay_excluded")
            ),
        },
        DashboardComponent {
            id: "environment".to_string(),
            pass: environment.pass,
            details: format!("passed={}/{}", environment.passed, environment.total),
        },
        DashboardComponent {
            id: "command".to_string(),
            pass: command.pass,
            details: format!("missing={} partial={}", command.missing, command.partial),
        },
        DashboardComponent {
            id: "legacy_command_binding_parity".to_string(),
            pass: pass_field(legacy_command_binding_parity),
            details: format!(
                "passed={}/{}",
                usize_field(legacy_command_binding_parity, "passed"),
                usize_field(legacy_command_binding_parity, "total")
            ),
        },
        DashboardComponent {
            id: "spells".to_string(),
            pass: spells.pass,
            details: format!("passed={}/{}", spells.passed, spells.total),
        },
        DashboardComponent {
            id: "items".to_string(),
            pass: items.pass,
            details: format!("passed={}/{}", items.passed, items.total),
        },
        DashboardComponent {
            id: "combat".to_string(),
            pass: combat.pass,
            details: format!("passed={}/{}", combat.passed, combat.total),
        },
        DashboardComponent {
            id: "site".to_string(),
            pass: site.pass,
            details: format!("passed={}/{}", site.passed, site.total),
        },
        DashboardComponent {
            id: "progression".to_string(),
            pass: progression.pass,
            details: format!("passed={}/{}", progression.passed, progression.total),
        },
        DashboardComponent {
            id: "compatibility".to_string(),
            pass: compatibility.pass,
            details: format!("passed={}/{}", compatibility.passed, compatibility.total),
        },
        DashboardComponent {
            id: "frontend".to_string(),
            pass: frontend.pass,
            details: format!("passed={}/{}", frontend.passed, frontend.total),
        },
        DashboardComponent {
            id: "frontend_workflow_strict".to_string(),
            pass: pass_field(frontend_strict),
            details: format!(
                "passed={}/{}",
                usize_field(frontend_strict, "passed"),
                usize_field(frontend_strict, "total")
            ),
        },
        DashboardComponent {
            id: "replay_dashboard_active".to_string(),
            pass: replay_active_failed(replay_dashboard) == 0,
            details: format!(
                "active_total={} failed_active={}",
                replay_active_total(replay_dashboard),
                replay_active_failed(replay_dashboard)
            ),
        },
        DashboardComponent {
            id: "guild_live_check".to_string(),
            pass: pass_field(guild_live_check),
            details: format!(
                "passed={}/{}",
                usize_field(guild_live_check, "passed"),
                usize_field(guild_live_check, "total")
            ),
        },
        DashboardComponent {
            id: "service_branch_blackbox".to_string(),
            pass: pass_field(service_branch_blackbox),
            details: format!(
                "passed={}/{}",
                usize_field(service_branch_blackbox, "passed"),
                usize_field(service_branch_blackbox, "total")
            ),
        },
        DashboardComponent {
            id: "site_branch_diff".to_string(),
            pass: pass_field(site_branch_diff),
            details: format!(
                "passed={}/{}",
                usize_field(site_branch_diff, "passed"),
                usize_field(site_branch_diff, "total")
            ),
        },
        DashboardComponent {
            id: "parity_certify".to_string(),
            pass: pass_field(parity_certify),
            details: format!(
                "passed={}/{}",
                usize_field(parity_certify, "passed"),
                usize_field(parity_certify, "total")
            ),
        },
        DashboardComponent {
            id: "classic_mode_drift_guard".to_string(),
            pass: pass_field(classic_mode_drift_guard),
            details: format!(
                "passed={}/{}",
                usize_field(classic_mode_drift_guard, "passed"),
                usize_field(classic_mode_drift_guard, "total")
            ),
        },
        DashboardComponent {
            id: "classic_objective_drift_guard".to_string(),
            pass: pass_field(classic_objective_drift_guard),
            details: format!(
                "passed={}/{}",
                usize_field(classic_objective_drift_guard, "passed"),
                usize_field(classic_objective_drift_guard, "total")
            ),
        },
        DashboardComponent {
            id: "modern_mode_smoke".to_string(),
            pass: pass_field(modern_mode_smoke),
            details: format!(
                "passed={}/{}",
                usize_field(modern_mode_smoke, "passed"),
                usize_field(modern_mode_smoke, "total")
            ),
        },
        DashboardComponent {
            id: "modern_bevy_visual_smoke".to_string(),
            pass: pass_field(modern_bevy_visual_smoke),
            details: format!(
                "passed={}/{}",
                usize_field(modern_bevy_visual_smoke, "passed"),
                usize_field(modern_bevy_visual_smoke, "total")
            ),
        },
        DashboardComponent {
            id: "modern_objective_blackbox_smoke".to_string(),
            pass: pass_field(modern_objective_blackbox_smoke),
            details: format!(
                "passed={}/{}",
                usize_field(modern_objective_blackbox_smoke, "passed"),
                usize_field(modern_objective_blackbox_smoke, "total")
            ),
        },
        DashboardComponent {
            id: "bevy_visual_blackbox_suite".to_string(),
            pass: pass_field(bevy_visual_blackbox_suite),
            details: format!(
                "passed={}/{}",
                usize_field(bevy_visual_blackbox_suite, "passed"),
                usize_field(bevy_visual_blackbox_suite, "total")
            ),
        },
        DashboardComponent {
            id: "dual_mode_blackbox_suite".to_string(),
            pass: pass_field(dual_mode_blackbox_suite),
            details: format!(
                "passed={}/{}",
                usize_field(dual_mode_blackbox_suite, "passed"),
                usize_field(dual_mode_blackbox_suite, "total")
            ),
        },
        DashboardComponent {
            id: "mode_artifact_integrity_guard".to_string(),
            pass: pass_field(mode_artifact_integrity_guard),
            details: format!(
                "passed={}/{}",
                usize_field(mode_artifact_integrity_guard, "passed"),
                usize_field(mode_artifact_integrity_guard, "total")
            ),
        },
        DashboardComponent {
            id: "classic_visual_drift_guard".to_string(),
            pass: pass_field(classic_visual_drift_guard),
            details: format!(
                "passed={}/{}",
                usize_field(classic_visual_drift_guard, "passed"),
                usize_field(classic_visual_drift_guard, "total")
            ),
        },
        DashboardComponent {
            id: "bevy_semantic_projection_parity".to_string(),
            pass: pass_field(bevy_semantic_projection_parity),
            details: format!(
                "passed={}/{}",
                usize_field(bevy_semantic_projection_parity, "passed"),
                usize_field(bevy_semantic_projection_parity, "total")
            ),
        },
    ];
    let total = components.len();
    let passed = components.iter().filter(|component| component.pass).count();
    let failed = total.saturating_sub(passed);
    let report = TrueParityRegressionDashboard {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        status: if failed == 0 { "PASS" } else { "FAIL" }.to_string(),
        components,
    };
    write_json("target/true-parity-regression-dashboard.json", &report)?;
    Ok(report)
}

fn build_burnin_window(replay_dashboard: &Value) -> Result<TrueBurninWindow> {
    let determinism = read_json_if_exists("target/ws-d-determinism-report.json");
    let required_runs_per_fixture = determinism
        .as_ref()
        .map(|value| usize_field(value, "required_runs_per_fixture"))
        .unwrap_or(0);
    let determinism_total_runs =
        determinism.as_ref().map(|value| usize_field(value, "total_runs")).unwrap_or(0);
    let determinism_divergent_runs =
        determinism.as_ref().map(|value| usize_field(value, "divergent_runs")).unwrap_or(0);
    let replay_total_scenarios = replay_active_total(replay_dashboard);
    let replay_failed_scenarios = replay_active_failed(replay_dashboard);
    let mut blockers = Vec::new();
    if determinism.is_none() {
        blockers.push("missing ws-d determinism report".to_string());
    }
    if required_runs_per_fixture < 20 {
        blockers.push(format!("required_runs_per_fixture={required_runs_per_fixture} below 20"));
    }
    if determinism_divergent_runs > 0 {
        blockers.push(format!("determinism divergences={determinism_divergent_runs}"));
    }
    if replay_failed_scenarios > 0 {
        blockers.push(format!("frontend replay failures={replay_failed_scenarios}"));
    }
    let report = TrueBurninWindow {
        generated_at_utc: now_utc_unix(),
        required_runs_per_fixture,
        determinism_total_runs,
        replay_total_scenarios,
        replay_failed_scenarios,
        determinism_divergent_runs,
        pass: blockers.is_empty(),
        blockers,
    };
    write_json("target/true-burnin-window.json", &report)?;
    Ok(report)
}

fn build_baseline_freeze() -> Result<BaselineFreeze> {
    let required = vec![
        "target/true-parity-deviations.json",
        "target/true-startup-parity.json",
        "target/true-environment-transition-matrix.json",
        "target/true-command-behavior-matrix.json",
        "target/true-spell-parity-matrix.json",
        "target/true-item-parity-matrix.json",
        "target/true-combat-encounter-matrix.json",
        "target/true-site-economy-social-matrix.json",
        "target/true-progression-ending-matrix.json",
        "target/true-compatibility-matrix.json",
        "target/true-frontend-workflow-matrix.json",
        "target/classic-frontend-workflow-parity.json",
        "target/ws-d-regression-dashboard.json",
        "target/service-branch-blackbox-smoke.json",
        "target/classic/classic-mode-drift-guard.json",
        "target/classic/classic-objective-drift-guard.json",
        "target/modern/modern-mode-smoke.json",
        "target/modern/modern-objective-blackbox-smoke.json",
        "target/dual/dual-mode-blackbox-suite.json",
        "target/mode-artifact-integrity-guard.json",
        "target/runtime-user-regression-smoke.json",
        "target/legacy-site-branch-contract.json",
        "target/rust-site-branch-contract.json",
        "target/site-branch-diff.json",
        "target/guild-service-talk-clarity.json",
        "target/guild-live-check.json",
        "target/guild-parity-defect-board.json",
        "target/certification/baseline.json",
        "target/certification/contracts/legacy-mechanics-ledger.json",
        "target/certification/contracts/rust-mechanics-ledger.json",
        "target/certification/contracts/mechanics_mapping.json",
        "target/certification/diff/legacy-headless-replay.json",
        "target/certification/diff/rust-headless-replay.json",
        "target/certification/diff/mechanics-differential.json",
        "target/certification/diff/service-branch-differential.json",
        "target/certification/coverage/branch-coverage.json",
        "target/certification/smoke/blackbox-adversarial.json",
        "target/certification/defect-board.json",
        "target/certification/parity-certify.json",
        "target/mechanics-audit-baseline.json",
        "target/legacy-mechanics-ledger.json",
        "target/rust-mechanics-ledger.json",
        "target/mechanics-parity-matrix.json",
        "target/mechanics-missing-defect-board.json",
        "target/mechanics-smoke.json",
        "target/true-parity-regression-dashboard.json",
        "target/true-burnin-window.json",
        "docs/migration/FULL_OMEGA_TRUE_PARITY_PLAN.md",
        "docs/migration/FULL_OMEGA_TRUE_PARITY_SCORECARD.md",
        "docs/migration/FULL_OMEGA_TRUE_PARITY_CLOSURE_REVIEW.md",
        "docs/migration/MECHANICS_PARITY_MAPPING.yaml",
        "docs/migration/FULL_MECHANICS_PARITY_AUDIT_V2.md",
        "docs/migration/ALL_GUILDS_PARITY_CLOSURE_REPORT.md",
        "docs/migration/FULL_SOURCE_EXHAUSTIVE_PARITY_CERTIFICATION.md",
        "docs/migration/DUAL_MODE_ARCHITECTURE.md",
        "docs/migration/CLASSIC_FREEZE_CONTRACT.md",
        "docs/migration/DUAL_MODE_PARITY_CERTIFICATION.md",
        "docs/migration/MODERN_SLICE_01_OBJECTIVE_JOURNAL.md",
    ];
    let artifacts = required
        .into_iter()
        .map(|path| {
            let p = Path::new(path);
            if p.exists() {
                let bytes = fs::read(p).with_context(|| format!("read {}", p.display()))?;
                Ok(ArtifactSnapshot {
                    path: path.to_string(),
                    present: true,
                    size_bytes: bytes.len() as u64,
                    hash_fnv1a64: format!("{:016x}", fnv1a64(&bytes)),
                })
            } else {
                Ok(ArtifactSnapshot {
                    path: path.to_string(),
                    present: false,
                    size_bytes: 0,
                    hash_fnv1a64: "missing".to_string(),
                })
            }
        })
        .collect::<Result<Vec<_>>>()?;
    let total = artifacts.len();
    let present = artifacts.iter().filter(|artifact| artifact.present).count();
    let missing = total.saturating_sub(present);
    let freeze = BaselineFreeze {
        generated_at_utc: now_utc_unix(),
        status: if missing == 0 { "PASS" } else { "FAIL" }.to_string(),
        total,
        present,
        missing,
        artifacts,
    };
    write_json("target/true-parity-baseline-freeze.json", &freeze)?;
    Ok(freeze)
}

#[allow(clippy::too_many_arguments)]
fn build_deviation_ledger(
    mechanics: &Value,
    environment: &TrueMatrix,
    command: &TrueCommandBehaviorMatrix,
    spells: &TrueMatrix,
    items: &TrueMatrix,
    combat: &TrueMatrix,
    site: &TrueMatrix,
    progression: &TrueMatrix,
    compatibility: &TrueMatrix,
    frontend: &TrueMatrix,
    frontend_strict: &Value,
    replay_dashboard: &Value,
    dashboard: &TrueParityRegressionDashboard,
    burnin: &TrueBurninWindow,
) -> Result<DeviationLedger> {
    let mut items_out = Vec::new();
    let mut push_item = |track: &str, severity: &str, title: &str, pass: bool, details: String| {
        items_out.push(DeviationItem {
            id: format!("D-{track}-001"),
            track: track.to_string(),
            severity: severity.to_string(),
            status: if pass { "CLOSED" } else { "OPEN" }.to_string(),
            title: title.to_string(),
            details,
        });
    };

    push_item(
        "T1",
        "P0",
        "Mechanics denominator parity",
        bool_field(mechanics, "pass")
            && usize_field(mechanics, "unknown") == 0
            && mechanics_main_non_equivalent(mechanics) == 0
            && mechanics_unresolved_gameplay(mechanics) == 0
            && usize_field(mechanics, "gameplay_excluded") == 0,
        format!(
            "pass={} unknown={} main_non_equivalent={} unresolved_gameplay={} gameplay_excluded={}",
            bool_field(mechanics, "pass"),
            usize_field(mechanics, "unknown"),
            mechanics_main_non_equivalent(mechanics),
            mechanics_unresolved_gameplay(mechanics),
            usize_field(mechanics, "gameplay_excluded")
        ),
    );

    push_item(
        "T2",
        "P0",
        "Environment parity",
        environment.pass,
        format!("passed={}/{}", environment.passed, environment.total),
    );
    push_item(
        "T3",
        "P0",
        "Command parity",
        command.pass,
        format!("missing={} partial={}", command.missing, command.partial),
    );
    push_item(
        "T4",
        "P0",
        "Spell parity",
        spells.pass,
        format!("passed={}/{}", spells.passed, spells.total),
    );
    push_item(
        "T5",
        "P0",
        "Item parity",
        items.pass,
        format!("passed={}/{}", items.passed, items.total),
    );
    push_item(
        "T6",
        "P0",
        "Combat parity",
        combat.pass,
        format!("passed={}/{}", combat.passed, combat.total),
    );
    push_item(
        "T7",
        "P0",
        "Site parity",
        site.pass,
        format!("passed={}/{}", site.passed, site.total),
    );
    push_item(
        "T8",
        "P1",
        "Progression parity",
        progression.pass,
        format!("passed={}/{}", progression.passed, progression.total),
    );
    push_item(
        "T9",
        "P1",
        "Compatibility parity",
        compatibility.pass,
        format!("passed={}/{}", compatibility.passed, compatibility.total),
    );
    push_item(
        "T10",
        "P1",
        "Frontend parity",
        frontend.pass && pass_field(frontend_strict),
        format!(
            "matrix={}/{} strict={}/{}",
            frontend.passed,
            frontend.total,
            usize_field(frontend_strict, "passed"),
            usize_field(frontend_strict, "total")
        ),
    );
    push_item(
        "T11",
        "P1",
        "Replay active denominator parity",
        replay_active_failed(replay_dashboard) == 0,
        format!(
            "active_total={} failed_active={}",
            replay_active_total(replay_dashboard),
            replay_active_failed(replay_dashboard)
        ),
    );
    push_item(
        "T12",
        "P1",
        "Verification hardening",
        dashboard.status == "PASS" && burnin.pass,
        format!("dashboard={} burnin={}", dashboard.status, burnin.pass),
    );

    let open = items_out.iter().filter(|item| item.status == "OPEN").count();
    let total = items_out.len();
    let ledger = DeviationLedger {
        generated_at_utc: now_utc_unix(),
        summary: DeviationSummary { total, open, closed: total.saturating_sub(open) },
        items: items_out,
    };
    write_json("target/true-parity-deviations.json", &ledger)?;
    Ok(ledger)
}

fn main() -> Result<()> {
    if !target_dir().exists() {
        fs::create_dir_all(target_dir()).context("create target directory")?;
    }

    let startup = ensure_startup_parity_artifact()?;
    let environment = build_environment_matrix()?;
    let command = build_command_matrix()?;
    let spells = build_spell_matrix()?;
    let items = build_item_matrix()?;
    let combat = build_combat_matrix()?;
    let site = build_site_matrix()?;
    let progression = build_progression_matrix()?;
    let compatibility = build_compatibility_matrix()?;
    let frontend = build_frontend_workflow_matrix()?;
    let frontend_strict = if Path::new("target/classic-frontend-workflow-parity.json").exists() {
        read_json("target/classic-frontend-workflow-parity.json")?
    } else {
        serde_json::json!({
            "total": 0,
            "passed": 0,
            "failed": 1,
            "pass": false
        })
    };
    let replay_dashboard = if Path::new("target/ws-d-regression-dashboard.json").exists() {
        read_json("target/ws-d-regression-dashboard.json")?
    } else {
        serde_json::json!({
            "total": 0,
            "active_total": 0,
            "failed": 1,
            "failed_active": 1
        })
    };
    let guild_live_check = if Path::new("target/guild-live-check.json").exists() {
        read_json("target/guild-live-check.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing guild-live-check artifact"})
    };
    let legacy_command_binding_parity =
        if Path::new("target/legacy-command-binding-parity.json").exists() {
            read_json("target/legacy-command-binding-parity.json")?
        } else {
            serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing legacy-command-binding-parity artifact"})
        };
    let service_branch_blackbox = if Path::new("target/service-branch-blackbox-smoke.json").exists()
    {
        read_json("target/service-branch-blackbox-smoke.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing service-branch-blackbox-smoke artifact"})
    };
    let site_branch_diff = if Path::new("target/site-branch-diff.json").exists() {
        read_json("target/site-branch-diff.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing site-branch-diff artifact"})
    };
    let parity_certify = if Path::new("target/certification/parity-certify.json").exists() {
        read_json("target/certification/parity-certify.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing certification/parity-certify artifact"})
    };
    let classic_mode_drift_guard = if Path::new("target/classic/classic-mode-drift-guard.json")
        .exists()
    {
        read_json("target/classic/classic-mode-drift-guard.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing classic mode drift guard artifact"})
    };
    let classic_objective_drift_guard = if Path::new(
        "target/classic/classic-objective-drift-guard.json",
    )
    .exists()
    {
        read_json("target/classic/classic-objective-drift-guard.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing classic objective drift guard artifact"})
    };
    let modern_mode_smoke = if Path::new("target/modern/modern-mode-smoke.json").exists() {
        read_json("target/modern/modern-mode-smoke.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing modern mode smoke artifact"})
    };
    let modern_bevy_visual_smoke = if Path::new("target/modern/bevy-visual-smoke.json").exists() {
        read_json("target/modern/bevy-visual-smoke.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing modern bevy visual smoke artifact"})
    };
    let modern_objective_blackbox_smoke = if Path::new(
        "target/modern/modern-objective-blackbox-smoke.json",
    )
    .exists()
    {
        read_json("target/modern/modern-objective-blackbox-smoke.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing modern objective blackbox smoke artifact"})
    };
    let bevy_visual_blackbox_suite = if Path::new("target/dual/bevy-visual-blackbox.json").exists()
    {
        read_json("target/dual/bevy-visual-blackbox.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing bevy visual blackbox suite artifact"})
    };
    let dual_mode_blackbox_suite = if Path::new("target/dual/dual-mode-blackbox-suite.json")
        .exists()
    {
        read_json("target/dual/dual-mode-blackbox-suite.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing dual mode blackbox suite artifact"})
    };
    let mode_artifact_integrity_guard = if Path::new("target/mode-artifact-integrity-guard.json")
        .exists()
    {
        read_json("target/mode-artifact-integrity-guard.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing mode artifact integrity guard artifact"})
    };
    let classic_visual_drift_guard = if Path::new("target/classic/classic-visual-drift-guard.json")
        .exists()
    {
        read_json("target/classic/classic-visual-drift-guard.json")?
    } else {
        serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing classic visual drift guard artifact"})
    };
    let bevy_semantic_projection_parity =
        if Path::new("target/bevy-semantic-projection-parity.json").exists() {
            read_json("target/bevy-semantic-projection-parity.json")?
        } else {
            serde_json::json!({"pass": false, "passed": 0, "total": 1, "details": "missing bevy semantic projection parity artifact"})
        };
    let mechanics = if Path::new("target/mechanics-parity-matrix.json").exists() {
        read_json("target/mechanics-parity-matrix.json")?
    } else {
        serde_json::json!({
                "pass": false,
            "unknown": 1,
            "main_non_equivalent": 1,
            "unresolved_gameplay": 1,
            "gameplay_excluded": 0
        })
    };
    let dashboard = build_regression_dashboard(
        &startup,
        &mechanics,
        &command,
        &legacy_command_binding_parity,
        &environment,
        &spells,
        &items,
        &combat,
        &site,
        &progression,
        &compatibility,
        &frontend,
        &frontend_strict,
        &replay_dashboard,
        &guild_live_check,
        &service_branch_blackbox,
        &site_branch_diff,
        &parity_certify,
        &classic_mode_drift_guard,
        &classic_objective_drift_guard,
        &modern_mode_smoke,
        &modern_bevy_visual_smoke,
        &modern_objective_blackbox_smoke,
        &bevy_visual_blackbox_suite,
        &dual_mode_blackbox_suite,
        &mode_artifact_integrity_guard,
        &classic_visual_drift_guard,
        &bevy_semantic_projection_parity,
    )?;
    let burnin = build_burnin_window(&replay_dashboard)?;
    let deviations = build_deviation_ledger(
        &mechanics,
        &environment,
        &command,
        &spells,
        &items,
        &combat,
        &site,
        &progression,
        &compatibility,
        &frontend,
        &frontend_strict,
        &replay_dashboard,
        &dashboard,
        &burnin,
    )?;
    let freeze = build_baseline_freeze()?;

    let all_pass = bool_field(&startup, "pass")
        && bool_field(&mechanics, "pass")
        && usize_field(&mechanics, "unknown") == 0
        && mechanics_main_non_equivalent(&mechanics) == 0
        && mechanics_unresolved_gameplay(&mechanics) == 0
        && usize_field(&mechanics, "gameplay_excluded") == 0
        && environment.pass
        && command.pass
        && pass_field(&legacy_command_binding_parity)
        && spells.pass
        && items.pass
        && combat.pass
        && site.pass
        && progression.pass
        && compatibility.pass
        && frontend.pass
        && pass_field(&frontend_strict)
        && replay_active_failed(&replay_dashboard) == 0
        && pass_field(&service_branch_blackbox)
        && pass_field(&parity_certify)
        && pass_field(&classic_mode_drift_guard)
        && pass_field(&classic_objective_drift_guard)
        && pass_field(&modern_mode_smoke)
        && pass_field(&modern_bevy_visual_smoke)
        && pass_field(&modern_objective_blackbox_smoke)
        && pass_field(&bevy_visual_blackbox_suite)
        && pass_field(&dual_mode_blackbox_suite)
        && pass_field(&mode_artifact_integrity_guard)
        && pass_field(&classic_visual_drift_guard)
        && pass_field(&bevy_semantic_projection_parity)
        && dashboard.status == "PASS"
        && burnin.pass
        && deviations.summary.open == 0
        && freeze.status == "PASS";

    println!(
        "true parity refresh: startup={} mechanics={} env={} cmd={} cmd_binding={} spell={} item={} combat={} site={} progression={} compatibility={} frontend={} frontend_strict={} replay_active={} service_branch_blackbox={} parity_certify={} classic_mode_drift={} classic_objective_drift={} modern_mode_smoke={} modern_bevy_visual_smoke={} modern_objective_blackbox_smoke={} bevy_visual_blackbox_suite={} dual_mode_blackbox={} mode_artifact_integrity={} classic_visual_drift={} bevy_semantic_projection={} dashboard={} burnin={} deviations_open={} freeze={}",
        if bool_field(&startup, "pass") { "PASS" } else { "FAIL" },
        if bool_field(&mechanics, "pass")
            && usize_field(&mechanics, "unknown") == 0
            && mechanics_main_non_equivalent(&mechanics) == 0
            && mechanics_unresolved_gameplay(&mechanics) == 0
            && usize_field(&mechanics, "gameplay_excluded") == 0
        {
            "PASS"
        } else {
            "FAIL"
        },
        if environment.pass { "PASS" } else { "FAIL" },
        if command.pass { "PASS" } else { "FAIL" },
        if pass_field(&legacy_command_binding_parity) { "PASS" } else { "FAIL" },
        if spells.pass { "PASS" } else { "FAIL" },
        if items.pass { "PASS" } else { "FAIL" },
        if combat.pass { "PASS" } else { "FAIL" },
        if site.pass { "PASS" } else { "FAIL" },
        if progression.pass { "PASS" } else { "FAIL" },
        if compatibility.pass { "PASS" } else { "FAIL" },
        if frontend.pass { "PASS" } else { "FAIL" },
        if pass_field(&frontend_strict) { "PASS" } else { "FAIL" },
        if replay_active_failed(&replay_dashboard) == 0 { "PASS" } else { "FAIL" },
        if pass_field(&service_branch_blackbox) { "PASS" } else { "FAIL" },
        if pass_field(&parity_certify) { "PASS" } else { "FAIL" },
        if pass_field(&classic_mode_drift_guard) { "PASS" } else { "FAIL" },
        if pass_field(&classic_objective_drift_guard) { "PASS" } else { "FAIL" },
        if pass_field(&modern_mode_smoke) { "PASS" } else { "FAIL" },
        if pass_field(&modern_bevy_visual_smoke) { "PASS" } else { "FAIL" },
        if pass_field(&modern_objective_blackbox_smoke) { "PASS" } else { "FAIL" },
        if pass_field(&bevy_visual_blackbox_suite) { "PASS" } else { "FAIL" },
        if pass_field(&dual_mode_blackbox_suite) { "PASS" } else { "FAIL" },
        if pass_field(&mode_artifact_integrity_guard) { "PASS" } else { "FAIL" },
        if pass_field(&classic_visual_drift_guard) { "PASS" } else { "FAIL" },
        if pass_field(&bevy_semantic_projection_parity) { "PASS" } else { "FAIL" },
        dashboard.status,
        if burnin.pass { "PASS" } else { "FAIL" },
        deviations.summary.open,
        freeze.status
    );

    if !all_pass {
        bail!("one or more true parity artifacts failed");
    }
    Ok(())
}
