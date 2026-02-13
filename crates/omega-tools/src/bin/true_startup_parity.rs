use anyhow::{Context, Result, bail};
use omega_bevy as bevy_frontend;
use omega_content::{
    LEGACY_CITY_MAP_ID, LEGACY_RAMPART_START, bootstrap_game_state_from_default_content,
};
use omega_core::{
    Command, DeterministicRng, Direction, GameState, LegacyEnvironment, MapSemanticKind, Position,
    SITE_AUX_EXIT_COUNTRYSIDE, SITE_AUX_SERVICE_ARENA, SITE_AUX_SERVICE_BANK,
    SITE_AUX_SERVICE_CASTLE, SITE_AUX_SERVICE_CHARITY, SITE_AUX_SERVICE_COLLEGE,
    SITE_AUX_SERVICE_MERC_GUILD, SITE_AUX_SERVICE_ORDER, SITE_AUX_SERVICE_SHOP,
    SITE_AUX_SERVICE_SORCERORS, SITE_AUX_SERVICE_TEMPLE, WorldMode, step,
};
use omega_tui as tui_frontend;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct StartupCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct StartupParityReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<StartupCheck>,
}

fn markdown(report: &StartupParityReport) -> String {
    let mut out = Vec::new();
    out.push("# True Startup Parity".to_string());
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

fn service_choice(aux: i32) -> &'static str {
    match aux {
        SITE_AUX_SERVICE_SHOP => "1",
        SITE_AUX_SERVICE_BANK => "1",
        SITE_AUX_SERVICE_MERC_GUILD => "1",
        SITE_AUX_SERVICE_TEMPLE => "1",
        SITE_AUX_SERVICE_COLLEGE => "1",
        SITE_AUX_SERVICE_SORCERORS => "1",
        SITE_AUX_SERVICE_CASTLE => "1",
        SITE_AUX_SERVICE_ORDER => "1",
        SITE_AUX_SERVICE_CHARITY => "1",
        SITE_AUX_SERVICE_ARENA => "2",
        _ => "1",
    }
}

fn service_tile(state: &GameState) -> Option<(i32, Position)> {
    let width = usize::try_from(state.bounds.width).ok()?;
    let services = [
        SITE_AUX_SERVICE_SHOP,
        SITE_AUX_SERVICE_BANK,
        SITE_AUX_SERVICE_MERC_GUILD,
        SITE_AUX_SERVICE_TEMPLE,
        SITE_AUX_SERVICE_COLLEGE,
        SITE_AUX_SERVICE_SORCERORS,
        SITE_AUX_SERVICE_CASTLE,
        SITE_AUX_SERVICE_ORDER,
        SITE_AUX_SERVICE_CHARITY,
        SITE_AUX_SERVICE_ARENA,
    ];
    for aux in services {
        let Some((idx, _)) = state.site_grid.iter().enumerate().find(|(_, cell)| cell.aux == aux)
        else {
            continue;
        };
        let x = i32::try_from(idx % width).ok()?;
        let y = i32::try_from(idx / width).ok()?;
        return Some((aux, Position { x, y }));
    }
    None
}

fn exit_tile(state: &GameState) -> Option<Position> {
    let width = usize::try_from(state.bounds.width).ok()?;
    let (idx, _) = state
        .site_grid
        .iter()
        .enumerate()
        .find(|(_, cell)| cell.aux == SITE_AUX_EXIT_COUNTRYSIDE)?;
    let x = i32::try_from(idx % width).ok()?;
    let y = i32::try_from(idx / width).ok()?;
    Some(Position { x, y })
}

fn direction_between(from: Position, to: Position) -> Option<Direction> {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    match (dx, dy) {
        (1, 0) => Some(Direction::East),
        (-1, 0) => Some(Direction::West),
        (0, 1) => Some(Direction::South),
        (0, -1) => Some(Direction::North),
        _ => None,
    }
}

fn closed_door_bump_positions(state: &GameState) -> Option<(Position, Position)> {
    for y in 0..state.bounds.height {
        for x in 0..state.bounds.width {
            let door = Position { x, y };
            let glyph = state.map_glyph_at(door);
            if glyph != '-' && glyph != 'D' && glyph != 'J' {
                continue;
            }
            let candidates = [
                Position { x: door.x + 1, y: door.y },
                Position { x: door.x - 1, y: door.y },
                Position { x: door.x, y: door.y + 1 },
                Position { x: door.x, y: door.y - 1 },
            ];
            for candidate in candidates {
                if state.bounds.contains(candidate) && state.tile_is_walkable(candidate) {
                    return Some((candidate, door));
                }
            }
        }
    }
    None
}

fn startup_door_check(id: &str, mut state: GameState) -> StartupCheck {
    let Some((from, door)) = closed_door_bump_positions(&state) else {
        return StartupCheck {
            id: id.to_string(),
            passed: false,
            details: "no bump-openable closed door found in startup map".to_string(),
        };
    };
    let Some(direction) = direction_between(from, door) else {
        return StartupCheck {
            id: id.to_string(),
            passed: false,
            details: "found door but no adjacent cardinal bump direction".to_string(),
        };
    };
    state.player.position = from;
    let mut rng = DeterministicRng::seeded(0x5EED_1001);
    let out = step(&mut state, Command::Move(direction), &mut rng);
    let opened = state.map_glyph_at(door) == '/';
    let moved = state.player.position == door;
    let passed = opened && moved;
    StartupCheck {
        id: id.to_string(),
        passed,
        details: format!(
            "from=({}, {}) door=({}, {}) opened={} moved={} events={}",
            from.x,
            from.y,
            door.x,
            door.y,
            opened,
            moved,
            out.events.len()
        ),
    }
}

fn startup_service_check(id: &str, mut state: GameState) -> StartupCheck {
    state.options.interactive_sites = true;
    let Some((aux, pos)) = service_tile(&state) else {
        return StartupCheck {
            id: id.to_string(),
            passed: false,
            details: "no service tile found on startup map".to_string(),
        };
    };
    state.player.position = pos;
    let before = state.clone();
    let mut rng = DeterministicRng::seeded(0x5EED_1002);
    let open = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
    let choose =
        step(&mut state, Command::Legacy { token: service_choice(aux).to_string() }, &mut rng);

    let opened = open.events.iter().any(|event| {
        matches!(
            event,
            omega_core::Event::LegacyHandled { token, .. } if token == "interaction"
        )
    });
    let changed = state.gold != before.gold
        || state.bank_gold != before.bank_gold
        || state.progression != before.progression
        || state.player.inventory.len() != before.player.inventory.len()
        || state.environment != before.environment;
    StartupCheck {
        id: id.to_string(),
        passed: opened && changed,
        details: format!(
            "aux={} opened={} changed={} gold {}->{} bank {}->{} inv {}->{} env {:?}->{:?} choose_events={}",
            aux,
            opened,
            changed,
            before.gold,
            state.gold,
            before.bank_gold,
            state.bank_gold,
            before.player.inventory.len(),
            state.player.inventory.len(),
            before.environment,
            state.environment,
            choose.events.len()
        ),
    }
}

fn startup_exit_check(id: &str, mut state: GameState) -> StartupCheck {
    let Some(exit) = exit_tile(&state) else {
        return StartupCheck {
            id: id.to_string(),
            passed: false,
            details: "no countryside exit tile found in startup map".to_string(),
        };
    };
    state.player.position = exit;
    let mut rng = DeterministicRng::seeded(0x5EED_1003);
    let out = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
    let passed = state.world_mode == WorldMode::Countryside
        && state.environment == LegacyEnvironment::Countryside
        && state.map_binding.semantic == MapSemanticKind::Country;
    StartupCheck {
        id: id.to_string(),
        passed,
        details: format!(
            "exit=({}, {}) world={:?} env={:?} semantic={:?} events={}",
            exit.x,
            exit.y,
            state.world_mode,
            state.environment,
            state.map_binding.semantic,
            out.events.len()
        ),
    }
}

fn main() -> Result<()> {
    let (content_state, diagnostics) =
        bootstrap_game_state_from_default_content().context("content bootstrap")?;
    let tui_state = tui_frontend::run_headless_bootstrap().context("tui headless bootstrap")?;
    let bevy_state = bevy_frontend::run_headless_bootstrap().context("bevy headless bootstrap")?;

    let mut checks = vec![
        StartupCheck {
            id: "content_map_is_city".to_string(),
            passed: diagnostics.map_source.ends_with("city.map"),
            details: format!(
                "map_source={} expected_map_id={}",
                diagnostics.map_source, LEGACY_CITY_MAP_ID
            ),
        },
        StartupCheck {
            id: "content_spawn_matches_legacy".to_string(),
            passed: content_state.player.position == LEGACY_RAMPART_START,
            details: format!(
                "spawn=({}, {}) expected=({}, {}) source={}",
                content_state.player.position.x,
                content_state.player.position.y,
                LEGACY_RAMPART_START.x,
                LEGACY_RAMPART_START.y,
                diagnostics.player_spawn_source
            ),
        },
        StartupCheck {
            id: "content_city_context".to_string(),
            passed: content_state.world_mode == WorldMode::DungeonCity
                && content_state.topology.city_site_id == 1,
            details: format!(
                "world={:?} city_site_id={}",
                content_state.world_mode, content_state.topology.city_site_id
            ),
        },
        StartupCheck {
            id: "tui_bootstrap_city_context".to_string(),
            passed: tui_state.player.position == LEGACY_RAMPART_START
                && tui_state.world_mode == WorldMode::DungeonCity
                && tui_state.topology.city_site_id == 1,
            details: format!(
                "spawn=({}, {}) world={:?} city_site_id={}",
                tui_state.player.position.x,
                tui_state.player.position.y,
                tui_state.world_mode,
                tui_state.topology.city_site_id
            ),
        },
        StartupCheck {
            id: "bevy_bootstrap_city_context".to_string(),
            passed: bevy_state.player.position == LEGACY_RAMPART_START
                && bevy_state.world_mode == WorldMode::DungeonCity
                && bevy_state.topology.city_site_id == 1,
            details: format!(
                "spawn=({}, {}) world={:?} city_site_id={}",
                bevy_state.player.position.x,
                bevy_state.player.position.y,
                bevy_state.world_mode,
                bevy_state.topology.city_site_id
            ),
        },
    ];

    checks.push(startup_door_check("content_door_bump_open", content_state.clone()));
    checks.push(startup_service_check("content_service_is_actionable", content_state.clone()));
    checks.push(startup_exit_check("content_exit_to_country", content_state.clone()));

    checks.push(startup_door_check("tui_door_bump_open", tui_state.clone()));
    checks.push(startup_service_check("tui_service_is_actionable", tui_state.clone()));
    checks.push(startup_exit_check("tui_exit_to_country", tui_state.clone()));

    checks.push(startup_door_check("bevy_door_bump_open", bevy_state.clone()));
    checks.push(startup_service_check("bevy_service_is_actionable", bevy_state.clone()));
    checks.push(startup_exit_check("bevy_exit_to_country", bevy_state.clone()));

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let report = StartupParityReport { total, passed, failed, pass, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("true-startup-parity.json");
    let md_path = target.join("true-startup-parity.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize startup parity")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "true startup parity: total={}, passed={}, failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("true startup parity failed");
    }
    Ok(())
}
