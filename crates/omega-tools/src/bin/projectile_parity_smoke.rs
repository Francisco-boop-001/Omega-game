use anyhow::{Context, Result, bail};
use omega_core::{
    Command, DeterministicRng, Event, GameState, Item, ItemFamily, MapBounds, Position, Stats,
    TILE_FLAG_BLOCK_MOVE, TILE_FLAG_PORTCULLIS, TileSiteCell, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SmokeCheck {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ProjectileSmokeReport {
    generated_at_utc: String,
    pass: bool,
    checks: Vec<SmokeCheck>,
    timeline_tail: Vec<String>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn markdown(report: &ProjectileSmokeReport) -> String {
    let mut out = Vec::new();
    out.push("# Projectile Parity Smoke".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", report.generated_at_utc));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("## Checks".to_string());
    out.push(String::new());
    for check in &report.checks {
        out.push(format!(
            "- {}: {} ({})",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.details
        ));
    }
    out.push(String::new());
    out.push("## Timeline Tail".to_string());
    out.push(String::new());
    out.push("```text".to_string());
    out.extend(report.timeline_tail.clone());
    out.push("```".to_string());
    out.push(String::new());
    out.join("\n")
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

fn make_longbow(id: u32) -> Item {
    Item {
        id,
        name: "longbow".to_string(),
        family: ItemFamily::Weapon,
        legacy_id: omega_core::legacy_projectile_contract().ob_longbow,
        item_type: "MISSILE".to_string(),
        dmg: 12,
        hit: 15,
        ..Item::default()
    }
}

fn make_arrow(id: u32) -> Item {
    Item {
        id,
        name: "arrow".to_string(),
        family: ItemFamily::Weapon,
        legacy_id: omega_core::legacy_projectile_contract().ob_arrow,
        aux: omega_core::legacy_projectile_contract().i_arrow,
        number: 4,
        dmg: 3,
        hit: 3,
        ..Item::default()
    }
}

fn main() -> Result<()> {
    let mut checks = Vec::new();
    let mut rng = DeterministicRng::seeded(0x5052_4A53);

    let mut arrow_state = base_state();
    arrow_state.player.inventory.push(make_longbow(1));
    arrow_state.player.inventory.push(make_arrow(2));
    arrow_state.player.equipment.weapon_hand = Some(1);
    arrow_state.player.equipment.ready_hand = Some(1);
    arrow_state.spawn_monster(
        "arrow-dummy",
        Position { x: 2, y: 2 },
        Stats { hp: 18, max_hp: 18, attack_min: 1, attack_max: 1, defense: 0 },
    );
    let turn_before_fire = arrow_state.clock.turn;
    let minutes_before_fire = arrow_state.clock.minutes;
    let open_fire = step(&mut arrow_state, Command::Legacy { token: "f".to_string() }, &mut rng);
    let arrow_key = choose_prompt_item_key(&mut arrow_state, &mut rng, "arrow")
        .context("resolve arrow choice key from prompt")?;
    let _ = step(&mut arrow_state, Command::Legacy { token: arrow_key.to_string() }, &mut rng);
    let fire_commit =
        step(&mut arrow_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    checks.push(SmokeCheck {
        id: "arrow_fire_flow".to_string(),
        pass: open_fire.minutes == minutes_before_fire
            && fire_commit.turn == turn_before_fire + 1
            && fire_commit.minutes == minutes_before_fire + 5
            && arrow_state.pending_targeting_interaction.is_none(),
        details: format!(
            "open_minutes={} commit_turn={} commit_minutes={} attacked={}",
            open_fire.minutes,
            fire_commit.turn,
            fire_commit.minutes,
            event_attacked(&fire_commit.events)
        ),
    });

    let mut spell_state = base_state();
    for spell in &mut spell_state.spellbook.spells {
        spell.known = true;
    }
    spell_state.spawn_monster(
        "spell-dummy",
        Position { x: 3, y: 2 },
        Stats { hp: 12, max_hp: 12, attack_min: 1, attack_max: 1, defense: 0 },
    );
    let turn_before_spell = spell_state.clock.turn;
    let minutes_before_spell = spell_state.clock.minutes;
    let _ = step(&mut spell_state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let _ =
        step(&mut spell_state, Command::Legacy { token: "magic missile".to_string() }, &mut rng);
    let select_spell =
        step(&mut spell_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let commit_spell =
        step(&mut spell_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    checks.push(SmokeCheck {
        id: "magic_missile_target_then_commit".to_string(),
        pass: select_spell.turn == turn_before_spell
            && select_spell.minutes == minutes_before_spell
            && commit_spell.turn == turn_before_spell + 1
            && commit_spell.minutes == minutes_before_spell + 20
            && spell_state.pending_targeting_interaction.is_none(),
        details: format!(
            "select_turn={} select_minutes={} commit_turn={} commit_minutes={} attacked={} pending_targeting={}",
            select_spell.turn,
            select_spell.minutes,
            commit_spell.turn,
            commit_spell.minutes,
            event_attacked(&commit_spell.events),
            spell_state.pending_targeting_interaction.is_some()
        ),
    });

    let mut blocked_state = base_state();
    for spell in &mut blocked_state.spellbook.spells {
        spell.known = true;
    }
    blocked_state.spawn_monster(
        "blocked-dummy",
        Position { x: 4, y: 2 },
        Stats { hp: 12, max_hp: 12, attack_min: 1, attack_max: 1, defense: 0 },
    );
    blocked_state.site_grid[2 * 9 + 2].flags |= TILE_FLAG_PORTCULLIS | TILE_FLAG_BLOCK_MOVE;
    blocked_state.city_site_grid = blocked_state.site_grid.clone();
    let hp_before = blocked_state.monsters[0].stats.hp;
    let _ = step(&mut blocked_state, Command::Legacy { token: "m".to_string() }, &mut rng);
    let _ =
        step(&mut blocked_state, Command::Legacy { token: "magic missile".to_string() }, &mut rng);
    let _ = step(&mut blocked_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    let blocked_commit =
        step(&mut blocked_state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
    checks.push(SmokeCheck {
        id: "projectile_los_block_smoke".to_string(),
        pass: !event_attacked(&blocked_commit.events)
            && blocked_state.monsters[0].stats.hp == hp_before,
        details: format!(
            "attacked={} hp_before={} hp_after={}",
            event_attacked(&blocked_commit.events),
            hp_before,
            blocked_state.monsters[0].stats.hp
        ),
    });

    let pass = checks.iter().all(|check| check.pass);
    let report = ProjectileSmokeReport {
        generated_at_utc: now_utc_unix(),
        pass,
        checks,
        timeline_tail: blocked_state
            .log
            .iter()
            .rev()
            .take(16)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect(),
    };

    let target_dir = Path::new("target");
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).context("create target directory")?;
    }
    let json_path = target_dir.join("projectile-parity-smoke.json");
    let md_path = target_dir.join("projectile-parity-smoke.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report)
            .context("serialize projectile parity smoke report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "projectile parity smoke: status={} checks_passed={}/{}",
        if report.pass { "PASS" } else { "FAIL" },
        report.checks.iter().filter(|check| check.pass).count(),
        report.checks.len()
    );
    println!("report json: {}", json_path.display());
    println!("report md: {}", md_path.display());

    if !report.pass {
        bail!("projectile parity smoke failed");
    }
    Ok(())
}
