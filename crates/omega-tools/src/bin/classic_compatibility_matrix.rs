use anyhow::{Context, Result, bail};
use omega_core::{
    Command, DeterministicRng, EndingKind, GameState, Item, ItemFamily, LEGACY_STATUS_CHEATED,
    MapBounds, Position, SessionStatus, TILE_FLAG_BLOCK_MOVE, TILE_FLAG_PORTCULLIS, VictoryTrigger,
    renderable_timeline_lines, step,
};
use omega_save::{SAVE_VERSION, decode_json, decode_state_json, encode_json};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CompatibilityCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CompatibilityMatrix {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<CompatibilityCheck>,
}

fn markdown(report: &CompatibilityMatrix) -> String {
    let mut out = Vec::new();
    out.push("# Classic Compatibility Matrix".to_string());
    out.push(String::new());
    out.push(format!("- Total checks: {}", report.total));
    out.push(format!("- Passed: {}", report.passed));
    out.push(format!("- Failed: {}", report.failed));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Check | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for check in &report.checks {
        out.push(format!(
            "| {} | {} | {} |",
            check.id,
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn build_legacy_v0_fixture() -> Result<String> {
    let state = GameState::default();
    let fixture = serde_json::json!({
        "version": 0,
        "payload": state,
        "metadata": {
            "schema": "omega-save-legacy",
            "saved_turn": 1,
            "saved_minutes": 6
        }
    });
    serde_json::to_string(&fixture).context("serialize legacy save fixture")
}

fn artifact_pass_with_details(path: &str) -> (bool, String) {
    let artifact = Path::new(path);
    if !artifact.exists() {
        return (false, format!("missing {}", artifact.display()));
    }
    let raw = match fs::read_to_string(artifact) {
        Ok(raw) => raw,
        Err(err) => return (false, format!("unable to read {}: {err}", artifact.display())),
    };
    let value: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(value) => value,
        Err(err) => return (false, format!("invalid json {}: {err}", artifact.display())),
    };
    let pass = value["pass"].as_bool().or_else(|| value["passed"].as_bool()).unwrap_or(false);
    let total = value["total"].as_u64().unwrap_or(0);
    let passed = value["passed"].as_u64().unwrap_or(0);
    let failed = value["failed"].as_u64().unwrap_or(total.saturating_sub(passed));
    (pass, format!("artifact={} total={} passed={} failed={}", path, total, passed, failed))
}

fn main() -> Result<()> {
    let mut rng = DeterministicRng::seeded(0xC0DE_7001);

    let legacy_raw = build_legacy_v0_fixture()?;
    let migrated = decode_json(&legacy_raw).context("decode legacy v0 fixture")?;
    let migrated_state = decode_state_json(&legacy_raw).context("decode migrated v0 state")?;
    let save_migration_ok = migrated.version == SAVE_VERSION
        && migrated_state.player_name == "Adventurer"
        && migrated_state.scheduler.player_phase == 0;

    let mut options_state = GameState::new(MapBounds { width: 9, height: 9 });
    options_state.player.position = Position { x: 4, y: 4 };
    options_state.place_item("ration", Position { x: 5, y: 4 });
    let before = options_state.options.clone();
    let _ = step(&mut options_state, Command::Legacy { token: "O".to_string() }, &mut rng);
    let _ = step(&mut options_state, Command::Move(omega_core::Direction::East), &mut rng);
    let options_parity_ok = options_state.options.pickup != before.pickup
        && options_state.options.confirm != before.confirm
        && options_state.player.inventory.len() == 1;

    let mut bump_attack_state = GameState::new(MapBounds { width: 7, height: 7 });
    let bump_start = bump_attack_state.player.position;
    let bump_target = Position { x: bump_start.x + 1, y: bump_start.y };
    bump_attack_state.spawn_monster(
        "rat",
        bump_target,
        omega_core::Stats {
            hp: 8,
            max_hp: 8,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 60,
        },
    );
    let bump_out =
        step(&mut bump_attack_state, Command::Move(omega_core::Direction::East), &mut rng);
    let bump_attacked = bump_out.events.iter().any(|event| {
        matches!(
            event,
            omega_core::Event::Attacked { .. } | omega_core::Event::MonsterDefeated { .. }
        )
    });
    let bump_not_blocked =
        bump_out.events.iter().all(|event| !matches!(event, omega_core::Event::MoveBlocked { .. }));
    let bump_position_static = bump_attack_state.player.position == bump_start;
    let bump_move_timing = bump_out.minutes == 5 && bump_attack_state.clock.minutes == 5;
    let bump_attack_parity_ok =
        bump_attacked && bump_not_blocked && bump_position_static && bump_move_timing;

    let mut arena_gate_state = GameState::new(MapBounds { width: 3, height: 3 });
    arena_gate_state.options.interactive_sites = true;
    arena_gate_state.player.position = Position { x: 1, y: 1 };
    arena_gate_state.site_grid = vec![omega_core::TileSiteCell::default(); 9];
    arena_gate_state.city_site_grid = arena_gate_state.site_grid.clone();
    arena_gate_state.site_maps = vec![omega_core::SiteMapDefinition {
        map_id: 1,
        level_index: 0,
        source: "compat/arena.map".to_string(),
        environment: omega_core::LegacyEnvironment::Arena,
        semantic: omega_core::MapSemanticKind::Site,
        spawn: Position { x: 2, y: 7 },
        rows: {
            let width = 64usize;
            let height = 16usize;
            let mut rows = vec!["#".repeat(width); height];
            for row in rows.iter_mut().take(13).skip(3) {
                let mut chars: Vec<char> = row.chars().collect();
                for cell in chars.iter_mut().take(62).skip(2) {
                    *cell = '.';
                }
                *row = chars.into_iter().collect();
            }
            for y in [7usize, 8usize] {
                let mut chars: Vec<char> = rows[y].chars().collect();
                chars[0] = 'X';
                chars[1] = 'P';
                chars[2] = 'P';
                rows[y] = chars.into_iter().collect();
            }
            rows
        },
        site_grid: {
            let width = 64usize;
            let height = 16usize;
            let mut cells = Vec::with_capacity(width * height);
            for y in 0..height {
                for x in 0..width {
                    let mut cell = omega_core::TileSiteCell::default();
                    if y == 7 || y == 8 {
                        if x == 0 {
                            cell.glyph = 'X';
                            cell.aux = omega_core::SITE_AUX_EXIT_ARENA;
                        } else if x == 1 || x == 2 {
                            cell.glyph = 'P';
                            cell.flags |= TILE_FLAG_PORTCULLIS | TILE_FLAG_BLOCK_MOVE;
                        } else if (2..62).contains(&x) {
                            cell.glyph = '.';
                        } else {
                            cell.glyph = '#';
                            cell.flags |= TILE_FLAG_BLOCK_MOVE;
                        }
                    } else if (3..13).contains(&y) && (2..62).contains(&x) {
                        cell.glyph = '.';
                    } else {
                        cell.glyph = '#';
                        cell.flags |= TILE_FLAG_BLOCK_MOVE;
                    }
                    cells.push(cell);
                }
            }
            cells
        },
    }];
    arena_gate_state.site_grid[4].aux = omega_core::SITE_AUX_SERVICE_ARENA;
    arena_gate_state.city_site_grid[4].aux = omega_core::SITE_AUX_SERVICE_ARENA;
    let _ = step(&mut arena_gate_state, Command::Legacy { token: ">".to_string() }, &mut rng);
    let _ = step(&mut arena_gate_state, Command::Legacy { token: "1".to_string() }, &mut rng);
    let closed_before = arena_gate_state
        .site_grid
        .iter()
        .filter(|cell| {
            (cell.flags & TILE_FLAG_PORTCULLIS) != 0 && (cell.flags & TILE_FLAG_BLOCK_MOVE) != 0
        })
        .count();
    if let Some(monster) = arena_gate_state.monsters.first_mut() {
        monster.stats.hp = 1;
        monster.stats.max_hp = 1;
    }
    if let Some(monster_pos) = arena_gate_state.monsters.first().map(|m| m.position) {
        arena_gate_state.player.position = Position { x: monster_pos.x - 1, y: monster_pos.y };
    }
    let _ = step(&mut arena_gate_state, Command::Attack(omega_core::Direction::East), &mut rng);
    let closed_after_win = arena_gate_state
        .site_grid
        .iter()
        .filter(|cell| {
            (cell.flags & TILE_FLAG_PORTCULLIS) != 0 && (cell.flags & TILE_FLAG_BLOCK_MOVE) != 0
        })
        .count();
    let opener_dropped =
        arena_gate_state.ground_items.iter().any(|g| g.item.usef == "I_RAISE_PORTCULLIS");
    if let Some(opener_pos) = arena_gate_state
        .ground_items
        .iter()
        .find(|g| g.item.usef == "I_RAISE_PORTCULLIS")
        .map(|g| g.position)
    {
        arena_gate_state.player.position = opener_pos;
    }
    let _ = step(&mut arena_gate_state, Command::Pickup, &mut rng);
    let _ = step(&mut arena_gate_state, Command::Legacy { token: "a".to_string() }, &mut rng);
    let _ = step(&mut arena_gate_state, Command::Legacy { token: "i".to_string() }, &mut rng);
    let _ = step(&mut arena_gate_state, Command::Legacy { token: "a".to_string() }, &mut rng);
    let closed_after_raise = arena_gate_state
        .site_grid
        .iter()
        .filter(|cell| {
            (cell.flags & TILE_FLAG_PORTCULLIS) != 0 && (cell.flags & TILE_FLAG_BLOCK_MOVE) != 0
        })
        .count();
    let arena_portcullis_lifecycle_ok =
        closed_before > 0 && closed_after_win > 0 && opener_dropped && closed_after_raise == 0;

    let mut wizard_state = GameState::default();
    let _ = step(&mut wizard_state, Command::Legacy { token: "^g".to_string() }, &mut rng);
    let _ = step(&mut wizard_state, Command::Legacy { token: "y".to_string() }, &mut rng);
    let wizard_policy_ok = wizard_state.wizard.enabled
        && !wizard_state.wizard.scoring_allowed
        && !wizard_state.progression.high_score_eligible
        && (wizard_state.legacy_status_flags & LEGACY_STATUS_CHEATED) != 0;

    let mut reveal_state = GameState::new(MapBounds { width: 7, height: 5 });
    reveal_state.wizard.enabled = true;
    reveal_state.known_sites.clear();
    let _ = step(&mut reveal_state, Command::Legacy { token: "^w".to_string() }, &mut rng);
    let map_reveal_ok = reveal_state.known_sites.len() >= 35;

    let mut wish_state = GameState::default();
    wish_state.wizard.enabled = true;
    wish_state.gold = 123;
    let _ = step(&mut wish_state, Command::Legacy { token: "^x".to_string() }, &mut rng);
    let _ = step(&mut wish_state, Command::Legacy { token: "wealth".to_string() }, &mut rng);
    let wish_commit =
        step(&mut wish_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let wish_wealth_ok = wish_state.pending_wizard_interaction.is_none()
        && wish_state.gold >= 10_123
        && wish_commit.minutes >= 5;

    let mut wish_acquisition_state = GameState::default();
    wish_acquisition_state.wizard.enabled = true;
    let _ =
        step(&mut wish_acquisition_state, Command::Legacy { token: "^x".to_string() }, &mut rng);
    let _ = step(
        &mut wish_acquisition_state,
        Command::Legacy { token: "get item".to_string() },
        &mut rng,
    );
    let _ = step(
        &mut wish_acquisition_state,
        Command::Legacy { token: "<enter>".to_string() },
        &mut rng,
    );
    let _ = step(&mut wish_acquisition_state, Command::Legacy { token: ")".to_string() }, &mut rng);
    let _ = step(&mut wish_acquisition_state, Command::Legacy { token: "1".to_string() }, &mut rng);
    let wish_acquire_commit = step(
        &mut wish_acquisition_state,
        Command::Legacy { token: "<enter>".to_string() },
        &mut rng,
    );
    let no_placeholder =
        wish_acquisition_state.player.inventory.iter().all(|item| {
            !item.name.contains("wishforged") && !item.name.contains("acquired trinket")
        });
    let wish_acquisition_ok = wish_acquisition_state.pending_wizard_interaction.is_none()
        && !wish_acquisition_state.player.inventory.is_empty()
        && no_placeholder
        && wish_acquire_commit.minutes >= 5;
    let wish_flow_ok = wish_wealth_ok && wish_acquisition_ok;

    let mut status_editor_state = GameState::default();
    status_editor_state.wizard.enabled = true;
    let _ = step(&mut status_editor_state, Command::Legacy { token: "^k".to_string() }, &mut rng);
    let _ = step(&mut status_editor_state, Command::Legacy { token: "s".to_string() }, &mut rng);
    let _ = step(&mut status_editor_state, Command::Legacy { token: "5".to_string() }, &mut rng);
    let _ =
        step(&mut status_editor_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let status_editor_ok = (status_editor_state.legacy_status_flags & (1u64 << 5)) != 0
        && (status_editor_state.legacy_status_flags & LEGACY_STATUS_CHEATED) != 0;

    let mut stat_editor_state = GameState::default();
    stat_editor_state.wizard.enabled = true;
    let _ = step(&mut stat_editor_state, Command::Legacy { token: "#".to_string() }, &mut rng);
    let _ = step(&mut stat_editor_state, Command::Legacy { token: " ".to_string() }, &mut rng);
    let _ = step(&mut stat_editor_state, Command::Legacy { token: "20".to_string() }, &mut rng);
    let _ =
        step(&mut stat_editor_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let stat_editor_ok = stat_editor_state.attributes.strength == 20;

    let mut normal_victory = GameState::default();
    let _ = step(&mut normal_victory, Command::Legacy { token: "Q".to_string() }, &mut rng);
    let normal_quit_prompt_open = normal_victory.pending_quit_interaction.is_some();
    let _ = step(&mut normal_victory, Command::Legacy { token: "y".to_string() }, &mut rng);

    let mut wizard_victory = GameState::default();
    let _ = step(&mut wizard_victory, Command::Legacy { token: "^g".to_string() }, &mut rng);
    let _ = step(&mut wizard_victory, Command::Legacy { token: "y".to_string() }, &mut rng);
    let _ = step(&mut wizard_victory, Command::Legacy { token: "Q".to_string() }, &mut rng);
    let wizard_quit_prompt_open = wizard_victory.pending_quit_interaction.is_some();
    let _ = step(&mut wizard_victory, Command::Legacy { token: "y".to_string() }, &mut rng);
    let score_policy_ok = normal_quit_prompt_open
        && wizard_quit_prompt_open
        && normal_victory.status == SessionStatus::Won
        && wizard_victory.status == SessionStatus::Won
        && normal_victory.progression.victory_trigger == Some(VictoryTrigger::QuitConfirmed)
        && wizard_victory.progression.victory_trigger == Some(VictoryTrigger::QuitConfirmed)
        && normal_victory.progression.ending == EndingKind::Victory
        && wizard_victory.progression.ending == EndingKind::Victory
        && normal_victory.progression.high_score_eligible
        && !wizard_victory.progression.high_score_eligible
        && normal_victory.progression.score > wizard_victory.progression.score;

    let mut roundtrip_state = GameState::default();
    roundtrip_state.options.pickup = true;
    roundtrip_state.wizard.enabled = true;
    roundtrip_state.wizard.scoring_allowed = false;
    roundtrip_state.legacy_status_flags = LEGACY_STATUS_CHEATED | (1u64 << 9);
    roundtrip_state.pending_wizard_interaction =
        Some(omega_core::WizardInteraction::WishTextEntry { blessing: 1 });
    roundtrip_state.wizard_input_buffer = "wealth".to_string();
    roundtrip_state.progression.score = 777;
    let encoded = encode_json(&roundtrip_state).context("encode roundtrip state")?;
    let decoded = decode_state_json(&encoded).context("decode roundtrip state")?;
    let save_roundtrip_ok = decoded.options.pickup
        && decoded.wizard.enabled
        && !decoded.wizard.scoring_allowed
        && decoded.legacy_status_flags == (LEGACY_STATUS_CHEATED | (1u64 << 9))
        && decoded.pending_wizard_interaction
            == Some(omega_core::WizardInteraction::WishTextEntry { blessing: 1 })
        && decoded.wizard_input_buffer == "wealth"
        && decoded.progression.score == 777;

    let mut inventory_modal_state = GameState::new(MapBounds { width: 7, height: 7 });
    inventory_modal_state.player.inventory.push(Item {
        id: 1,
        name: "practice blade".to_string(),
        family: ItemFamily::Weapon,
        ..Item::default()
    });
    inventory_modal_state.player.equipment.ready_hand = Some(1);
    inventory_modal_state.player.equipment.weapon_hand = Some(1);
    let inventory_open =
        step(&mut inventory_modal_state, Command::Legacy { token: "i".to_string() }, &mut rng);
    let _ = step(&mut inventory_modal_state, Command::Legacy { token: "d".to_string() }, &mut rng);
    let inventory_modal_ok = inventory_open.minutes == 0
        && inventory_modal_state.pending_inventory_interaction.is_some()
        && inventory_modal_state.player.inventory.is_empty()
        && inventory_modal_state.ground_items.len() == 1;

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
    let item_prompt_open =
        step(&mut item_prompt_state, Command::Legacy { token: "q".to_string() }, &mut rng);
    let locked_position = item_prompt_state.player.position;
    let _ = step(&mut item_prompt_state, Command::Move(omega_core::Direction::East), &mut rng);
    let item_prompt_commit =
        step(&mut item_prompt_state, Command::Legacy { token: "a".to_string() }, &mut rng);
    let item_prompt_ok = item_prompt_open.minutes == 0
        && item_prompt_state.player.position == locked_position
        && item_prompt_state.pending_item_prompt.is_none()
        && item_prompt_state.player.stats.hp > 10
        && item_prompt_commit.minutes >= 5;

    let mut prompt_leak_state = GameState::default();
    prompt_leak_state.wizard.enabled = true;
    let _ = step(&mut prompt_leak_state, Command::Legacy { token: "^x".to_string() }, &mut rng);
    let timeline_len_after_open = prompt_leak_state.log.len();
    let _ = step(&mut prompt_leak_state, Command::Legacy { token: "v".to_string() }, &mut rng);
    let _ = step(&mut prompt_leak_state, Command::Legacy { token: "i".to_string() }, &mut rng);
    let _ = step(&mut prompt_leak_state, Command::Legacy { token: "c".to_string() }, &mut rng);
    let timeline_len_after_typing = prompt_leak_state.log.len();
    let filtered_timeline = renderable_timeline_lines(&prompt_leak_state, 32);
    let prompt_leakage_ok = timeline_len_after_typing == timeline_len_after_open
        && filtered_timeline
            .iter()
            .all(|line| !line.contains("prompt active") && !line.starts_with("Wish text:"));

    let mut spell_flow_state = GameState::new(MapBounds { width: 9, height: 9 });
    for spell in &mut spell_flow_state.spellbook.spells {
        spell.known = true;
    }
    spell_flow_state.spawn_monster(
        "spell-foe",
        Position {
            x: spell_flow_state.player.position.x + 1,
            y: spell_flow_state.player.position.y,
        },
        omega_core::Stats {
            hp: 7,
            max_hp: 7,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 60,
        },
    );
    let open_spell =
        step(&mut spell_flow_state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let _ = step(
        &mut spell_flow_state,
        Command::Legacy { token: "magic missile".to_string() },
        &mut rng,
    );
    let select_spell =
        step(&mut spell_flow_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let commit_spell =
        step(&mut spell_flow_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let spell_prompt_opened = open_spell.events.iter().any(|event| {
        matches!(
            event,
            omega_core::Event::LegacyHandled { token, note, .. }
                if token == "m" && note.starts_with("Cast Spell:")
        )
    });
    let targeting_opened = spell_flow_state.pending_targeting_interaction.is_none()
        && select_spell.events.iter().any(|event| {
            matches!(
                event,
                omega_core::Event::LegacyHandled { token, note, .. }
                    if token == "m" && note.starts_with("cast spell#") && note.contains("choose a target")
            )
        });
    let spell_cast_committed = commit_spell.events.iter().any(|event| {
        matches!(
            event,
            omega_core::Event::Attacked { .. } | omega_core::Event::MonsterDefeated { .. }
        )
    });
    let spell_flow_ok = spell_prompt_opened
        && targeting_opened
        && spell_cast_committed
        && spell_flow_state.spellbook.mana < spell_flow_state.spellbook.max_mana;
    let (overworld_location_oracle_ok, overworld_location_oracle_details) =
        artifact_pass_with_details("target/overworld-location-parity.json");
    let (service_branch_oracle_ok, service_branch_oracle_details) =
        artifact_pass_with_details("target/service-branch-oracle.json");

    let checks = vec![
        CompatibilityCheck {
            id: "save_schema_migration".to_string(),
            passed: save_migration_ok,
            details: format!(
                "migrated_version={} player={} scheduler_phase={}",
                migrated.version, migrated_state.player_name, migrated_state.scheduler.player_phase
            ),
        },
        CompatibilityCheck {
            id: "runtime_options_effects".to_string(),
            passed: options_parity_ok,
            details: format!(
                "pickup={} confirm={} inventory_count={}",
                options_state.options.pickup,
                options_state.options.confirm,
                options_state.player.inventory.len()
            ),
        },
        CompatibilityCheck {
            id: "bump_attack_parity".to_string(),
            passed: bump_attack_parity_ok,
            details: format!(
                "attacked={} blocked={} pos_static={} minutes={} clock_minutes={}",
                bump_attacked,
                !bump_not_blocked,
                bump_position_static,
                bump_out.minutes,
                bump_attack_state.clock.minutes
            ),
        },
        CompatibilityCheck {
            id: "arena_portcullis_lifecycle".to_string(),
            passed: arena_portcullis_lifecycle_ok,
            details: format!(
                "closed_before={} closed_after_win={} opener_dropped={} closed_after_raise={}",
                closed_before, closed_after_win, opener_dropped, closed_after_raise
            ),
        },
        CompatibilityCheck {
            id: "wizard_policy_surface".to_string(),
            passed: wizard_policy_ok,
            details: format!(
                "wizard_enabled={} scoring_allowed={} high_score={} legacy_flags={:#x}",
                wizard_state.wizard.enabled,
                wizard_state.wizard.scoring_allowed,
                wizard_state.progression.high_score_eligible,
                wizard_state.legacy_status_flags
            ),
        },
        CompatibilityCheck {
            id: "wizard_map_reveal".to_string(),
            passed: map_reveal_ok,
            details: format!("known_sites={}", reveal_state.known_sites.len()),
        },
        CompatibilityCheck {
            id: "wizard_wish_flow".to_string(),
            passed: wish_flow_ok,
            details: format!(
                "wealth_ok={} acquire_ok={} gold={} acquire_items={} placeholders_blocked={} pending={} wealth_turn={} wealth_minutes={} acquire_turn={} acquire_minutes={}",
                wish_wealth_ok,
                wish_acquisition_ok,
                wish_state.gold,
                wish_acquisition_state.player.inventory.len(),
                no_placeholder,
                wish_state.pending_wizard_interaction.is_some(),
                wish_commit.turn,
                wish_commit.minutes,
                wish_acquire_commit.turn,
                wish_acquire_commit.minutes
            ),
        },
        CompatibilityCheck {
            id: "spell_interaction_prompt_flow".to_string(),
            passed: spell_flow_ok,
            details: format!(
                "prompt_opened={} targeting_opened={} cast_committed={} mana={}/{}",
                spell_prompt_opened,
                targeting_opened,
                spell_cast_committed,
                spell_flow_state.spellbook.mana,
                spell_flow_state.spellbook.max_mana
            ),
        },
        CompatibilityCheck {
            id: "wizard_status_editor".to_string(),
            passed: status_editor_ok,
            details: format!("legacy_flags={:#x}", status_editor_state.legacy_status_flags),
        },
        CompatibilityCheck {
            id: "wizard_stat_editor".to_string(),
            passed: stat_editor_ok,
            details: format!(
                "strength={} attack={}..{} mana={}/{}",
                stat_editor_state.attributes.strength,
                stat_editor_state.player.stats.attack_min,
                stat_editor_state.player.stats.attack_max,
                stat_editor_state.spellbook.mana,
                stat_editor_state.spellbook.max_mana
            ),
        },
        CompatibilityCheck {
            id: "score_policy_normal_vs_wizard".to_string(),
            passed: score_policy_ok,
            details: format!(
                "normal(prompt_open={},status={:?},trigger={:?},high_score={},score={}) wizard(prompt_open={},status={:?},trigger={:?},high_score={},score={})",
                normal_quit_prompt_open,
                normal_victory.status,
                normal_victory.progression.victory_trigger,
                normal_victory.progression.high_score_eligible,
                normal_victory.progression.score,
                wizard_quit_prompt_open,
                wizard_victory.status,
                wizard_victory.progression.victory_trigger,
                wizard_victory.progression.high_score_eligible,
                wizard_victory.progression.score
            ),
        },
        CompatibilityCheck {
            id: "save_roundtrip_policy_fields".to_string(),
            passed: save_roundtrip_ok,
            details: format!(
                "pickup={} wizard={} scoring_allowed={} flags={:#x} pending_wizard={} score={}",
                decoded.options.pickup,
                decoded.wizard.enabled,
                decoded.wizard.scoring_allowed,
                decoded.legacy_status_flags,
                decoded.pending_wizard_interaction.is_some(),
                decoded.progression.score
            ),
        },
        CompatibilityCheck {
            id: "inventory_modal_flow".to_string(),
            passed: inventory_modal_ok,
            details: format!(
                "open_minutes={} pending_inventory={} inventory_remaining={} ground_items={}",
                inventory_open.minutes,
                inventory_modal_state.pending_inventory_interaction.is_some(),
                inventory_modal_state.player.inventory.len(),
                inventory_modal_state.ground_items.len()
            ),
        },
        CompatibilityCheck {
            id: "item_prompt_modal_selection".to_string(),
            passed: item_prompt_ok,
            details: format!(
                "open_minutes={} commit_minutes={} hp={} pending_item_prompt={} pos=({}, {})",
                item_prompt_open.minutes,
                item_prompt_commit.minutes,
                item_prompt_state.player.stats.hp,
                item_prompt_state.pending_item_prompt.is_some(),
                item_prompt_state.player.position.x,
                item_prompt_state.player.position.y
            ),
        },
        CompatibilityCheck {
            id: "interaction_prompt_leakage".to_string(),
            passed: prompt_leakage_ok,
            details: format!(
                "timeline_after_open={} timeline_after_typing={} filtered_lines={}",
                timeline_len_after_open,
                timeline_len_after_typing,
                filtered_timeline.len()
            ),
        },
        CompatibilityCheck {
            id: "overworld_location_oracle".to_string(),
            passed: overworld_location_oracle_ok,
            details: overworld_location_oracle_details,
        },
        CompatibilityCheck {
            id: "service_branch_oracle".to_string(),
            passed: service_branch_oracle_ok,
            details: service_branch_oracle_details,
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let report = CompatibilityMatrix { total, passed, failed, pass, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("classic-compatibility-matrix.json");
    let md_path = target.join("classic-compatibility-matrix.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize compatibility matrix")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "classic compatibility matrix: total={}, passed={}, failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("classic compatibility matrix failed");
    }
    Ok(())
}
