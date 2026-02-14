use anyhow::{Context, Result, bail};
use omega_core::{
    Command, DeterministicRng, Event, Faction, GameState, Item, ItemFamily, MapBounds,
    MonsterBehavior, Position, Stats, TILE_FLAG_BLOCK_MOVE, TILE_FLAG_PORTCULLIS, TileSiteCell,
    legacy_projectile_contract, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ProjectileCheck {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ProjectileParityMatrix {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<ProjectileCheck>,
}

fn markdown(report: &ProjectileParityMatrix) -> String {
    let mut out = Vec::new();
    out.push("# Projectile Parity Matrix".to_string());
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
            if check.pass { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn event_attacked(events: &[Event]) -> bool {
    events
        .iter()
        .any(|event| matches!(event, Event::Attacked { .. } | Event::MonsterDefeated { .. }))
}

fn parse_choice_for_item_name(note: &str, name: &str) -> Option<char> {
    let lowered_name = name.to_ascii_lowercase();
    for segment in note.split(',') {
        let trimmed = segment.trim();
        let Some(idx) = trimmed.find(')') else {
            continue;
        };
        let key = trimmed[..idx].chars().last()?.to_ascii_lowercase();
        let label = trimmed[idx + 1..].trim().to_ascii_lowercase();
        if label.contains(&lowered_name) {
            return Some(key);
        }
    }
    None
}

fn choose_prompt_item_key(
    state: &mut GameState,
    rng: &mut DeterministicRng,
    name: &str,
) -> Option<char> {
    let out = step(state, Command::Legacy { token: "?".to_string() }, rng);
    let note = out.events.iter().find_map(|event| match event {
        Event::LegacyHandled { token, note, .. } if token == "item_prompt" => Some(note.as_str()),
        _ => None,
    })?;
    parse_choice_for_item_name(note, name)
}

fn base_state() -> GameState {
    let mut state = GameState::new(MapBounds { width: 9, height: 5 });
    state.map_rows = vec![
        ".........".to_string(),
        ".........".to_string(),
        ".........".to_string(),
        ".........".to_string(),
        ".........".to_string(),
    ];
    state.city_map_rows = state.map_rows.clone();
    state.site_grid = vec![TileSiteCell::default(); 45];
    state.city_site_grid = state.site_grid.clone();
    state.player.position = Position { x: 1, y: 2 };
    state
}

fn make_arrow(id: u32) -> Item {
    let contract = legacy_projectile_contract();
    Item {
        id,
        name: "arrow".to_string(),
        legacy_id: contract.ob_arrow,
        family: ItemFamily::Weapon,
        aux: contract.i_arrow,
        dmg: 3,
        hit: 3,
        number: 6,
        ..Item::default()
    }
}

fn make_bolt(id: u32) -> Item {
    let contract = legacy_projectile_contract();
    Item {
        id,
        name: "bolt".to_string(),
        legacy_id: contract.ob_bolt,
        family: ItemFamily::Weapon,
        aux: contract.i_bolt,
        dmg: 3,
        hit: 0,
        number: 6,
        ..Item::default()
    }
}

fn make_crossbow(id: u32, loaded: bool) -> Item {
    let contract = legacy_projectile_contract();
    Item {
        id,
        name: "crossbow".to_string(),
        legacy_id: contract.ob_crossbow,
        family: ItemFamily::Weapon,
        aux: if loaded { contract.loaded } else { contract.unloaded },
        dmg: 20,
        hit: 15,
        weight: 150,
        item_type: "MISSILE".to_string(),
        ..Item::default()
    }
}

fn make_longbow(id: u32) -> Item {
    let contract = legacy_projectile_contract();
    Item {
        id,
        name: "longbow".to_string(),
        legacy_id: contract.ob_longbow,
        family: ItemFamily::Weapon,
        dmg: 12,
        hit: 15,
        weight: 100,
        item_type: "MISSILE".to_string(),
        ..Item::default()
    }
}

fn main() -> Result<()> {
    let mut checks = Vec::new();

    let mut rng = DeterministicRng::seeded(0x5052_4A31);

    let mut fire_open_state = base_state();
    fire_open_state.player.inventory.push(make_arrow(1));
    let fire_open =
        step(&mut fire_open_state, Command::Legacy { token: "f".to_string() }, &mut rng);
    checks.push(ProjectileCheck {
        id: "fire_command_opens_item_prompt".to_string(),
        pass: fire_open_state.pending_item_prompt.is_some()
            && fire_open.minutes == 0
            && !event_attacked(&fire_open.events),
        details: format!(
            "pending_item_prompt={} minutes={} attacked_events={}",
            fire_open_state.pending_item_prompt.is_some(),
            fire_open.minutes,
            event_attacked(&fire_open.events)
        ),
    });

    let mut spell_target_state = base_state();
    for spell in &mut spell_target_state.spellbook.spells {
        spell.known = true;
    }
    spell_target_state.spawn_monster(
        "target-dummy",
        Position {
            x: spell_target_state.player.position.x + 2,
            y: spell_target_state.player.position.y,
        },
        Stats { hp: 10, max_hp: 10, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
    );
    let turn_before = spell_target_state.clock.turn;
    let minutes_before = spell_target_state.clock.minutes;
    let _ = step(&mut spell_target_state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let _ = step(
        &mut spell_target_state,
        Command::Legacy { token: "magic missile".to_string() },
        &mut rng,
    );
    let spell_select =
        step(&mut spell_target_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    checks.push(ProjectileCheck {
        id: "magic_missile_requires_targeting_modal".to_string(),
        pass: spell_target_state.pending_targeting_interaction.is_some()
            && spell_select.turn == turn_before
            && spell_select.minutes == minutes_before
            && !event_attacked(&spell_select.events),
        details: format!(
            "pending_targeting={} turn={} minutes={} attacked={}",
            spell_target_state.pending_targeting_interaction.is_some(),
            spell_select.turn,
            spell_select.minutes,
            event_attacked(&spell_select.events)
        ),
    });
    let cancel =
        step(&mut spell_target_state, Command::Legacy { token: "<esc>".to_string() }, &mut rng);
    checks.push(ProjectileCheck {
        id: "targeting_cancel_is_non_advancing".to_string(),
        pass: spell_target_state.pending_targeting_interaction.is_none()
            && cancel.turn == turn_before
            && cancel.minutes == minutes_before,
        details: format!(
            "pending_targeting={} turn={} minutes={}",
            spell_target_state.pending_targeting_interaction.is_some(),
            cancel.turn,
            cancel.minutes
        ),
    });

    let mut blocked_state = base_state();
    for spell in &mut blocked_state.spellbook.spells {
        spell.known = true;
    }
    blocked_state.spawn_monster(
        "portcullis-dummy",
        Position { x: 4, y: 2 },
        Stats { hp: 12, max_hp: 12, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
    );
    let blocked_idx = 2 * 9 + 2;
    blocked_state.site_grid[blocked_idx].flags |= TILE_FLAG_PORTCULLIS | TILE_FLAG_BLOCK_MOVE;
    blocked_state.city_site_grid = blocked_state.site_grid.clone();
    let hp_before = blocked_state.monsters[0].stats.hp;
    let _ = step(&mut blocked_state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let _ =
        step(&mut blocked_state, Command::Legacy { token: "magic missile".to_string() }, &mut rng);
    let _ = step(&mut blocked_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let blocked_cast =
        step(&mut blocked_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let hp_after = blocked_state.monsters[0].stats.hp;
    checks.push(ProjectileCheck {
        id: "projectile_los_blocks_on_portcullis".to_string(),
        pass: !event_attacked(&blocked_cast.events) && hp_after == hp_before,
        details: format!(
            "attacked={} hp_before={} hp_after={}",
            event_attacked(&blocked_cast.events),
            hp_before,
            hp_after
        ),
    });

    let mut monster_projectile_state = base_state();
    monster_projectile_state.player.position = Position { x: 2, y: 2 };
    monster_projectile_state.player.stats.hp = 30;
    monster_projectile_state.player.stats.max_hp = 30;
    let monster_id = monster_projectile_state.spawn_monster(
        "warlock",
        Position { x: 6, y: 2 },
        Stats { hp: 12, max_hp: 12, attack_min: 6, attack_max: 6, defense: 0, weight: 60 },
    );
    if let Some(monster) =
        monster_projectile_state.monsters.iter_mut().find(|monster| monster.id == monster_id)
    {
        monster.behavior = MonsterBehavior::Caster;
        monster.faction = Faction::Wild;
    }
    let blocker_idx = 2 * 9 + 4;
    monster_projectile_state.site_grid[blocker_idx].flags |=
        TILE_FLAG_PORTCULLIS | TILE_FLAG_BLOCK_MOVE;
    let _ = monster_projectile_state.set_map_glyph_at(Position { x: 4, y: 2 }, '=');
    monster_projectile_state.city_site_grid = monster_projectile_state.site_grid.clone();
    let player_hp_before = monster_projectile_state.player.stats.hp;
    let monster_projectile_out = step(&mut monster_projectile_state, Command::Wait, &mut rng);
    let blocked_log = monster_projectile_state.log.iter().any(|line| line.contains("blocked"));
    checks.push(ProjectileCheck {
        id: "monster_projectile_los_uses_projectile_resolver".to_string(),
        pass: monster_projectile_state.player.stats.hp == player_hp_before
            && monster_projectile_out
                .events
                .iter()
                .all(|event| !matches!(event, Event::MonsterAttacked { .. }))
            && blocked_log,
        details: format!(
            "hp_before={} hp_after={} attacked_events={} blocked_log={}",
            player_hp_before,
            monster_projectile_state.player.stats.hp,
            monster_projectile_out
                .events
                .iter()
                .any(|event| matches!(event, Event::MonsterAttacked { .. })),
            blocked_log
        ),
    });

    let mut load_state = base_state();
    let crossbow = make_crossbow(1, false);
    let bolt = make_bolt(2);
    load_state.player.inventory.push(crossbow);
    load_state.player.inventory.push(bolt);
    load_state.player.equipment.weapon_hand = Some(1);
    load_state.player.equipment.ready_hand = Some(1);
    let _ = step(&mut load_state, Command::Legacy { token: "f".to_string() }, &mut rng);
    let bolt_key = choose_prompt_item_key(&mut load_state, &mut rng, "bolt")
        .context("resolve bolt prompt key for load check")?;
    let load_out = step(&mut load_state, Command::Legacy { token: bolt_key.to_string() }, &mut rng);
    let crossbow_aux = load_state
        .player
        .inventory
        .iter()
        .find(|item| item.id == 1)
        .map(|item| item.aux)
        .unwrap_or_default();
    checks.push(ProjectileCheck {
        id: "crossbow_bolt_selection_loads_when_unloaded".to_string(),
        pass: crossbow_aux == legacy_projectile_contract().loaded
            && load_state.pending_targeting_interaction.is_none()
            && load_out.minutes >= 5,
        details: format!(
            "crossbow_aux={} loaded={} pending_targeting={} minutes={}",
            crossbow_aux,
            legacy_projectile_contract().loaded,
            load_state.pending_targeting_interaction.is_some(),
            load_out.minutes
        ),
    });

    let mut unload_ok = false;
    let mut unload_details = String::new();
    for seed in 0xAB10_u64..0xAB30_u64 {
        let mut shot_state = base_state();
        shot_state.player.stats.attack_min = 18;
        shot_state.player.stats.attack_max = 28;
        shot_state.player.inventory.push(make_crossbow(1, true));
        shot_state.player.inventory.push(make_bolt(2));
        shot_state.player.equipment.weapon_hand = Some(1);
        shot_state.player.equipment.ready_hand = Some(1);
        shot_state.spawn_monster(
            "bolt-dummy",
            Position { x: shot_state.player.position.x + 1, y: shot_state.player.position.y },
            Stats { hp: 20, max_hp: 20, attack_min: 1, attack_max: 1, defense: -20, weight: 60 },
        );
        let mut seeded = DeterministicRng::seeded(seed);
        let _ = step(&mut shot_state, Command::Legacy { token: "f".to_string() }, &mut seeded);
        let bolt_key = choose_prompt_item_key(&mut shot_state, &mut seeded, "bolt")
            .context("resolve bolt prompt key for unload check")?;
        let _ = step(&mut shot_state, Command::Legacy { token: bolt_key.to_string() }, &mut seeded);
        if shot_state.pending_targeting_interaction.is_none() {
            continue;
        }
        let fire_out =
            step(&mut shot_state, Command::Legacy { token: "<enter>".to_string() }, &mut seeded);
        let attacked = event_attacked(&fire_out.events);
        let weapon_aux = shot_state
            .player
            .inventory
            .iter()
            .find(|item| item.id == 1)
            .map(|item| item.aux)
            .unwrap_or_default();
        unload_details = format!("seed={} attacked={} weapon_aux={}", seed, attacked, weapon_aux);
        if attacked && weapon_aux == legacy_projectile_contract().unloaded {
            unload_ok = true;
            break;
        }
    }
    checks.push(ProjectileCheck {
        id: "loaded_crossbow_bolt_hit_unloads_weapon".to_string(),
        pass: unload_ok,
        details: unload_details,
    });

    let mut arrow_state = base_state();
    arrow_state.player.stats.attack_min = 8;
    arrow_state.player.stats.attack_max = 14;
    arrow_state.player.inventory.push(make_longbow(1));
    arrow_state.player.inventory.push(make_arrow(2));
    arrow_state.player.equipment.weapon_hand = Some(1);
    arrow_state.player.equipment.ready_hand = Some(1);
    arrow_state.spawn_monster(
        "arrow-dummy",
        Position { x: arrow_state.player.position.x + 1, y: arrow_state.player.position.y },
        Stats { hp: 30, max_hp: 30, attack_min: 1, attack_max: 1, defense: -5, weight: 60 },
    );
    let _ = step(&mut arrow_state, Command::Legacy { token: "f".to_string() }, &mut rng);
    let arrow_key = choose_prompt_item_key(&mut arrow_state, &mut rng, "arrow")
        .context("resolve arrow prompt key")?;
    let _ = step(&mut arrow_state, Command::Legacy { token: arrow_key.to_string() }, &mut rng);
    let arrow_shot =
        step(&mut arrow_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    checks.push(ProjectileCheck {
        id: "arrow_fire_uses_targeting_resolution".to_string(),
        pass: event_attacked(&arrow_shot.events)
            || arrow_state.pending_targeting_interaction.is_none(),
        details: format!(
            "attacked={} pending_targeting={} minutes={}",
            event_attacked(&arrow_shot.events),
            arrow_state.pending_targeting_interaction.is_some(),
            arrow_shot.minutes
        ),
    });

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let report = ProjectileParityMatrix { total, passed, failed, pass, checks };

    let target_dir = Path::new("target");
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).context("create target directory")?;
    }
    let json_path = target_dir.join("projectile-parity-matrix.json");
    let md_path = target_dir.join("projectile-parity-matrix.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize projectile parity matrix")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "projectile parity matrix: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    println!("report json: {}", json_path.display());
    println!("report md: {}", md_path.display());

    if !report.pass {
        bail!("projectile parity matrix failed");
    }
    Ok(())
}
