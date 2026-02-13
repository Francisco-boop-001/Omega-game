use anyhow::{Context, Result, bail};
use omega_core::{
    Command, DeterministicRng, Direction, GameState, LegacyEnvironment, MapBounds, MapSemanticKind,
    Position, SITE_AUX_EXIT_ARENA, SITE_AUX_SERVICE_ARENA, SiteMapDefinition, TILE_FLAG_BLOCK_MOVE,
    TILE_FLAG_PORTCULLIS, TileSiteCell, step,
};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct ArenaPortcullisSmoke {
    closed_before_fight: usize,
    closed_after_win: usize,
    opener_dropped: bool,
    closed_after_opener: usize,
    exited_after_opener: bool,
    pass: bool,
}

fn build_arena_site_map() -> SiteMapDefinition {
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

    let mut site_grid = Vec::with_capacity(width * height);
    for row in &rows {
        for glyph in row.chars() {
            let mut cell = TileSiteCell { glyph, site_id: 0, aux: 0, flags: 0 };
            match glyph {
                'X' => cell.aux = SITE_AUX_EXIT_ARENA,
                'P' => cell.flags |= TILE_FLAG_PORTCULLIS | TILE_FLAG_BLOCK_MOVE,
                '#' => cell.flags |= TILE_FLAG_BLOCK_MOVE,
                _ => {}
            }
            site_grid.push(cell);
        }
    }

    SiteMapDefinition {
        map_id: 1,
        level_index: 0,
        source: "smoke/arena.map".to_string(),
        environment: LegacyEnvironment::Arena,
        semantic: MapSemanticKind::Site,
        spawn: Position { x: 2, y: 7 },
        rows,
        site_grid,
    }
}

fn closed_portcullis_count(state: &GameState) -> usize {
    state
        .site_grid
        .iter()
        .filter(|cell| {
            (cell.flags & TILE_FLAG_PORTCULLIS) != 0 && (cell.flags & TILE_FLAG_BLOCK_MOVE) != 0
        })
        .count()
}

fn markdown(report: &ArenaPortcullisSmoke) -> String {
    [
        "# Arena Portcullis Smoke".to_string(),
        String::new(),
        format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }),
        format!("- closed_before_fight: {}", report.closed_before_fight),
        format!("- closed_after_win: {}", report.closed_after_win),
        format!("- opener_dropped: {}", report.opener_dropped),
        format!("- closed_after_opener: {}", report.closed_after_opener),
        format!("- exited_after_opener: {}", report.exited_after_opener),
        String::new(),
    ]
    .join("\n")
}

fn main() -> Result<()> {
    let mut rng = DeterministicRng::seeded(0xA11E_2026);
    let mut state = GameState::new(MapBounds { width: 3, height: 3 });
    state.options.interactive_sites = true;
    state.player.position = Position { x: 1, y: 1 };
    state.site_grid = vec![TileSiteCell::default(); 9];
    state.city_site_grid = state.site_grid.clone();
    state.site_maps = vec![build_arena_site_map()];
    state.site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
    state.city_site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
    state.player.stats.attack_min = 50;
    state.player.stats.attack_max = 50;

    let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
    let closed_before_fight = closed_portcullis_count(&state);

    let challenger_pos =
        state.monsters.first().map(|m| m.position).context("missing challenger")?;
    state.player.position = Position { x: challenger_pos.x - 1, y: challenger_pos.y };
    let _ = step(&mut state, Command::Attack(Direction::East), &mut rng);

    let closed_after_win = closed_portcullis_count(&state);
    let opener_dropped =
        state.ground_items.iter().any(|entry| entry.item.usef == "I_RAISE_PORTCULLIS");

    let opener_pos = state
        .ground_items
        .iter()
        .find(|entry| entry.item.usef == "I_RAISE_PORTCULLIS")
        .map(|entry| entry.position)
        .context("missing portcullis opener drop")?;
    state.player.position = opener_pos;
    let _ = step(&mut state, Command::Pickup, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "i".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);

    let closed_after_opener = closed_portcullis_count(&state);
    let mut exited_after_opener = state.environment == LegacyEnvironment::City;
    if !exited_after_opener {
        state.player.position = Position { x: 2, y: 7 };
        for _ in 0..3 {
            let _ = step(&mut state, Command::Move(Direction::West), &mut rng);
            if state.environment == LegacyEnvironment::City {
                exited_after_opener = true;
                break;
            }
        }
    }
    let report = ArenaPortcullisSmoke {
        closed_before_fight,
        closed_after_win,
        opener_dropped,
        closed_after_opener,
        exited_after_opener,
        pass: closed_before_fight > 0
            && closed_after_win > 0
            && opener_dropped
            && closed_after_opener == 0
            && exited_after_opener,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("arena-portcullis-smoke.json");
    let md_path = target.join("arena-portcullis-smoke.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize arena smoke report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "arena portcullis smoke: closed_before={} closed_after_win={} opener_dropped={} closed_after_opener={} exited_after_opener={} pass={}",
        report.closed_before_fight,
        report.closed_after_win,
        report.opener_dropped,
        report.closed_after_opener,
        report.exited_after_opener,
        report.pass
    );

    if !report.pass {
        bail!("arena portcullis smoke failed");
    }
    Ok(())
}
