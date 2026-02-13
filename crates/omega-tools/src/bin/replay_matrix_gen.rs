use anyhow::{Context, Result, bail};
use omega_core::{DeterministicRng, GameState, Position, Stats, step};
use omega_tools::replay::{
    ReplayCommand, ReplayDirection, ReplayExpected, ReplayFixture, ReplayInitialState,
    ReplayItemSpec, ReplayMonsterSpec, event_kind,
};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn matrix_dir() -> PathBuf {
    PathBuf::from("crates/omega-tools/fixtures/replay/matrix")
}

fn parse_out_dir() -> PathBuf {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--out-dir"
            && let Some(dir) = args.next()
        {
            return PathBuf::from(dir);
        }
    }
    matrix_dir()
}

fn normalize_tags(tags: &[&str], include_perf_smoke: bool) -> Vec<String> {
    let mut set: BTreeSet<String> = tags.iter().map(|tag| (*tag).to_string()).collect();
    if include_perf_smoke {
        set.insert("perf_smoke".to_string());
    }
    set.into_iter().collect()
}

fn derive_expected(
    seed: u64,
    initial: &ReplayInitialState,
    commands: &[ReplayCommand],
) -> ReplayExpected {
    let mut state = GameState::new(initial.bounds);
    state.player.position = initial.player_position;
    if let Some(stats) = initial.player_stats {
        state.player.stats = stats;
    }
    if let Some(capacity) = initial.inventory_capacity {
        state.player.inventory_capacity = capacity;
    }
    for monster in &initial.monsters {
        state.spawn_monster(monster.name.clone(), monster.position, monster.stats);
    }
    for item in &initial.ground_items {
        state.place_item(item.name.clone(), item.position);
    }

    let mut rng = DeterministicRng::seeded(seed);
    let mut required_event_kinds = BTreeSet::new();
    for command in commands {
        let outcome = step(&mut state, command.clone().into_command(), &mut rng);
        for event in &outcome.events {
            required_event_kinds.insert(event_kind(event).to_string());
        }
    }

    ReplayExpected {
        turn: state.clock.turn,
        minutes: state.clock.minutes,
        player_position: state.player.position,
        player_hp: state.player.stats.hp,
        monsters_alive: state.monsters.len(),
        inventory_count: state.player.inventory.len(),
        ground_item_count: state.ground_items.len(),
        required_event_kinds: required_event_kinds.into_iter().collect(),
        world_mode: None,
        guild_rank: None,
        priest_rank: None,
        alignment: None,
        quest_state: None,
        total_winner_unlocked: None,
        gold: None,
        bank_gold: None,
        food: None,
        known_site_count: None,
        ending: None,
        high_score_eligible: None,
    }
}

fn make_fixture(
    name: String,
    family: &str,
    tags: &[&str],
    include_perf_smoke: bool,
    seed: u64,
    initial: ReplayInitialState,
    commands: Vec<ReplayCommand>,
) -> ReplayFixture {
    let expected = derive_expected(seed, &initial, &commands);
    ReplayFixture {
        contract_version: 1,
        active: true,
        source: "generated_matrix".to_string(),
        name,
        family: family.to_string(),
        tags: normalize_tags(tags, include_perf_smoke),
        seed,
        initial,
        commands,
        expected,
    }
}

fn movement_blocked_fixtures() -> Vec<ReplayFixture> {
    let mut fixtures = Vec::new();
    for i in 0..140usize {
        let width = 5 + (i % 4) as i32;
        let height = 5 + ((i / 4) % 4) as i32;
        let (player_position, direction) = match i % 4 {
            0 => (Position { x: 0, y: 1 }, ReplayDirection::West),
            1 => (Position { x: width - 1, y: 1 }, ReplayDirection::East),
            2 => (Position { x: 1, y: 0 }, ReplayDirection::North),
            _ => (Position { x: 1, y: height - 1 }, ReplayDirection::South),
        };
        let initial = ReplayInitialState {
            bounds: omega_core::MapBounds { width, height },
            player_position,
            player_stats: None,
            inventory_capacity: None,
            monsters: Vec::new(),
            ground_items: Vec::new(),
            world_mode: None,
            environment: None,
            map_rows: None,
            site_aux_grid: None,
            site_flags_grid: None,
            gold: None,
            bank_gold: None,
            food: None,
        };
        fixtures.push(make_fixture(
            format!("matrix_movement_blocked_{i:03}"),
            "movement",
            &["critical_path", "frontend_shared", "movement"],
            i < 8,
            10_000 + i as u64,
            initial,
            vec![ReplayCommand::Move { direction }],
        ));
    }
    fixtures
}

fn movement_open_fixtures() -> Vec<ReplayFixture> {
    let mut fixtures = Vec::new();
    for i in 0..140usize {
        let width = 7 + (i % 4) as i32;
        let height = 7 + ((i / 4) % 4) as i32;
        let direction = match i % 4 {
            0 => ReplayDirection::North,
            1 => ReplayDirection::South,
            2 => ReplayDirection::East,
            _ => ReplayDirection::West,
        };
        let player_position = Position { x: 2 + (i % 3) as i32, y: 2 + ((i / 3) % 3) as i32 };
        let initial = ReplayInitialState {
            bounds: omega_core::MapBounds { width, height },
            player_position,
            player_stats: None,
            inventory_capacity: None,
            monsters: Vec::new(),
            ground_items: Vec::new(),
            world_mode: None,
            environment: None,
            map_rows: None,
            site_aux_grid: None,
            site_flags_grid: None,
            gold: None,
            bank_gold: None,
            food: None,
        };
        fixtures.push(make_fixture(
            format!("matrix_movement_open_{i:03}"),
            "movement",
            &["critical_path", "frontend_shared", "movement"],
            i < 8,
            20_000 + i as u64,
            initial,
            vec![ReplayCommand::Move { direction }],
        ));
    }
    fixtures
}

fn combat_fixtures() -> Vec<ReplayFixture> {
    let mut fixtures = Vec::new();
    for i in 0..120usize {
        let attack = 3 + (i % 5) as i32;
        let defense = 1 + (i % 2) as i32;
        let hp = if i % 3 == 0 { attack - defense + 1 } else { attack * 2 };
        let commands = if i % 3 == 0 {
            vec![ReplayCommand::Attack { direction: ReplayDirection::East }]
        } else {
            vec![
                ReplayCommand::Attack { direction: ReplayDirection::East },
                ReplayCommand::Attack { direction: ReplayDirection::East },
            ]
        };
        let initial = ReplayInitialState {
            bounds: omega_core::MapBounds { width: 9, height: 9 },
            player_position: Position { x: 4, y: 4 },
            player_stats: Some(Stats {
                hp: 20,
                max_hp: 20,
                attack_min: attack,
                attack_max: attack,
                defense: 1,
            }),
            inventory_capacity: None,
            monsters: vec![ReplayMonsterSpec {
                name: format!("matrix-rat-{i}"),
                position: Position { x: 5, y: 4 },
                stats: Stats { hp, max_hp: hp, attack_min: 1, attack_max: 2, defense },
            }],
            ground_items: Vec::new(),
            world_mode: None,
            environment: None,
            map_rows: None,
            site_aux_grid: None,
            site_flags_grid: None,
            gold: None,
            bank_gold: None,
            food: None,
        };
        fixtures.push(make_fixture(
            format!("matrix_combat_{i:03}"),
            "combat",
            &["critical_path", "frontend_shared", "combat"],
            i < 8,
            30_000 + i as u64,
            initial,
            commands,
        ));
    }
    fixtures
}

fn inventory_cycle_fixtures() -> Vec<ReplayFixture> {
    let mut fixtures = Vec::new();
    for i in 0..80usize {
        let initial = ReplayInitialState {
            bounds: omega_core::MapBounds { width: 9, height: 9 },
            player_position: Position { x: 4, y: 4 },
            player_stats: None,
            inventory_capacity: Some(6),
            monsters: Vec::new(),
            ground_items: vec![ReplayItemSpec {
                name: format!("matrix-item-{i}"),
                position: Position { x: 4, y: 4 },
            }],
            world_mode: None,
            environment: None,
            map_rows: None,
            site_aux_grid: None,
            site_flags_grid: None,
            gold: None,
            bank_gold: None,
            food: None,
        };
        fixtures.push(make_fixture(
            format!("matrix_inventory_cycle_{i:03}"),
            "inventory",
            &["critical_path", "frontend_shared", "inventory"],
            i < 4,
            40_000 + i as u64,
            initial,
            vec![ReplayCommand::Pickup, ReplayCommand::Drop { slot: 0 }],
        ));
    }
    fixtures
}

fn inventory_full_fixtures() -> Vec<ReplayFixture> {
    let mut fixtures = Vec::new();
    for i in 0..40usize {
        let initial = ReplayInitialState {
            bounds: omega_core::MapBounds { width: 7, height: 7 },
            player_position: Position { x: 3, y: 3 },
            player_stats: None,
            inventory_capacity: Some(0),
            monsters: Vec::new(),
            ground_items: vec![ReplayItemSpec {
                name: format!("matrix-overflow-{i}"),
                position: Position { x: 3, y: 3 },
            }],
            world_mode: None,
            environment: None,
            map_rows: None,
            site_aux_grid: None,
            site_flags_grid: None,
            gold: None,
            bank_gold: None,
            food: None,
        };
        fixtures.push(make_fixture(
            format!("matrix_inventory_full_{i:03}"),
            "inventory",
            &["critical_path", "frontend_shared", "inventory"],
            i < 2,
            50_000 + i as u64,
            initial,
            vec![ReplayCommand::Pickup],
        ));
    }
    fixtures
}

fn attack_miss_fixtures() -> Vec<ReplayFixture> {
    let mut fixtures = Vec::new();
    for i in 0..40usize {
        let initial = ReplayInitialState {
            bounds: omega_core::MapBounds { width: 9, height: 9 },
            player_position: Position { x: 4, y: 4 },
            player_stats: None,
            inventory_capacity: None,
            monsters: Vec::new(),
            ground_items: Vec::new(),
            world_mode: None,
            environment: None,
            map_rows: None,
            site_aux_grid: None,
            site_flags_grid: None,
            gold: None,
            bank_gold: None,
            food: None,
        };
        fixtures.push(make_fixture(
            format!("matrix_attack_miss_{i:03}"),
            "combat",
            &["frontend_shared", "combat"],
            false,
            60_000 + i as u64,
            initial,
            vec![ReplayCommand::Attack { direction: ReplayDirection::North }],
        ));
    }
    fixtures
}

fn save_compat_fixtures() -> Vec<ReplayFixture> {
    let mut fixtures = Vec::new();
    for i in 0..40usize {
        let initial = ReplayInitialState {
            bounds: omega_core::MapBounds { width: 9, height: 9 },
            player_position: Position { x: 4, y: 4 },
            player_stats: None,
            inventory_capacity: None,
            monsters: Vec::new(),
            ground_items: Vec::new(),
            world_mode: None,
            environment: None,
            map_rows: None,
            site_aux_grid: None,
            site_flags_grid: None,
            gold: None,
            bank_gold: None,
            food: None,
        };
        let command = if i % 2 == 0 {
            ReplayCommand::Wait
        } else {
            ReplayCommand::Move { direction: ReplayDirection::East }
        };
        fixtures.push(make_fixture(
            format!("matrix_save_compat_{i:03}"),
            "save",
            &["critical_path", "frontend_shared", "save_compat"],
            false,
            70_000 + i as u64,
            initial,
            vec![command],
        ));
    }
    fixtures
}

fn legacy_command_fixtures() -> Vec<ReplayFixture> {
    let mut fixtures = Vec::new();
    let tokens = [
        ",", ".", "@", "/", "?", "<", ">", "a", "A", "c", "C", "D", "E", "F", "G", "H", "I", "M",
        "O", "T", "V", "Z", "b", "n", "u", "y", "e", "f", "m", "o", "p", "r", "s", "t", "v", "x",
        "z", "^f", "^g", "^i", "^o", "^p", "^w", "^x",
    ];
    for (i, token) in tokens.iter().enumerate() {
        let initial = ReplayInitialState {
            bounds: omega_core::MapBounds { width: 11, height: 11 },
            player_position: Position { x: 5, y: 5 },
            player_stats: None,
            inventory_capacity: None,
            monsters: Vec::new(),
            ground_items: Vec::new(),
            world_mode: None,
            environment: None,
            map_rows: None,
            site_aux_grid: None,
            site_flags_grid: None,
            gold: None,
            bank_gold: None,
            food: None,
        };
        fixtures.push(make_fixture(
            format!("matrix_legacy_command_{i:03}"),
            "legacy_commands",
            &["legacy_surface", "frontend_shared"],
            i < 6,
            80_000 + i as u64,
            initial,
            vec![ReplayCommand::Legacy { token: (*token).to_string() }],
        ));
    }
    fixtures
}

fn write_fixture(path: &Path, fixture: &ReplayFixture) -> Result<()> {
    let raw = serde_json::to_string_pretty(fixture).context("serialize fixture")?;
    fs::write(path, raw).with_context(|| format!("write {}", path.display()))
}

fn clear_existing_json(dir: &Path) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).with_context(|| format!("read {}", dir.display()))? {
        let path = entry?.path();
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            fs::remove_file(&path).with_context(|| format!("remove {}", path.display()))?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let out_dir = parse_out_dir();
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).with_context(|| format!("create {}", out_dir.display()))?;
    }
    clear_existing_json(&out_dir)?;

    let mut all = Vec::new();
    all.extend(movement_blocked_fixtures());
    all.extend(movement_open_fixtures());
    all.extend(combat_fixtures());
    all.extend(inventory_cycle_fixtures());
    all.extend(inventory_full_fixtures());
    all.extend(attack_miss_fixtures());
    all.extend(save_compat_fixtures());
    all.extend(legacy_command_fixtures());

    if all.len() < 500 {
        bail!("fixture matrix too small: {} < 500", all.len());
    }

    for fixture in &all {
        let file = out_dir.join(format!("{}.json", fixture.name));
        write_fixture(&file, fixture)?;
    }

    println!("generated {} replay fixtures under {}", all.len(), out_dir.display());
    Ok(())
}
