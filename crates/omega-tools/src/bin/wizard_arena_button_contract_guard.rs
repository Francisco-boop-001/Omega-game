use anyhow::Result;
use omega_bevy::presentation::spawner::{SpawnerHazardSpec, SpawnerItemSpec, SpawnerMonsterSpec};
use omega_content::bootstrap_wizard_arena;
use omega_core::simulation::catastrophe::Catastrophe;
use omega_core::simulation::grid::CaGrid;
use omega_core::simulation::state::{Gas, Liquid};
use omega_core::simulation::wind::WindGrid;
use omega_core::{GameState, MapBounds, Position};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
struct CheckResult {
    name: String,
    pass: bool,
    detail: String,
}

#[derive(Debug, Serialize)]
struct GuardReport {
    checks: Vec<CheckResult>,
    passed: usize,
    failed: usize,
}

fn main() -> Result<()> {
    let mut checks = Vec::new();
    checks.extend(check_monster_specs());
    checks.extend(check_item_specs());
    checks.extend(check_hazard_specs());
    checks.extend(check_catastrophe_suite());
    checks.extend(check_arena_bootstrap());

    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = checks.len().saturating_sub(passed);
    let report = GuardReport { checks, passed, failed };

    let output_path = PathBuf::from("target/wizard-arena-button-contract-guard.json");
    if let Some(parent) = output_path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, serde_json::to_vec_pretty(&report)?)?;

    println!(
        "wizard_arena_button_contract_guard: {}/{} checks passed",
        report.passed,
        report.passed + report.failed
    );
    for check in &report.checks {
        let status = if check.pass { "PASS" } else { "FAIL" };
        println!("- [{status}] {}: {}", check.name, check.detail);
    }
    if report.failed > 0 {
        std::process::exit(1);
    }
    Ok(())
}

fn check_monster_specs() -> Vec<CheckResult> {
    SpawnerMonsterSpec::all()
        .iter()
        .map(|spec| {
            let stats = spec.stats();
            let pass = stats.max_hp > 0 && stats.attack_max >= stats.attack_min;
            CheckResult {
                name: format!("monster_spec:{}", spec.label()),
                pass,
                detail: format!(
                    "hp={} atk={}..{} def={}",
                    stats.max_hp, stats.attack_min, stats.attack_max, stats.defense
                ),
            }
        })
        .collect()
}

fn check_item_specs() -> Vec<CheckResult> {
    let mut state = GameState::new(MapBounds { width: 10, height: 10 });
    let mut checks = Vec::new();
    for spec in SpawnerItemSpec::all() {
        if let Some(spawn_name) = spec.spawn_name() {
            let item_id = state.place_item(spawn_name, Position { x: 2, y: 2 });
            let maybe_item = state
                .ground_items
                .iter()
                .find(|entry| entry.item.id == item_id)
                .map(|entry| &entry.item);
            let pass = maybe_item.is_some_and(|item| Some(item.family) == spec.expected_family());
            let detail = if let Some(item) = maybe_item {
                format!("resolved={} family={:?} usef={}", item.name, item.family, item.usef)
            } else {
                "item was not placed".to_string()
            };
            checks.push(CheckResult { name: format!("item_spec:{}", spec.label()), pass, detail });
        } else {
            checks.push(CheckResult {
                name: format!("item_spec:{}", spec.label()),
                pass: true,
                detail: "handled by fire-tile mutation path".to_string(),
            });
        }
    }
    checks
}

fn check_hazard_specs() -> Vec<CheckResult> {
    let mut state = GameState::new(MapBounds { width: 10, height: 10 });
    SpawnerHazardSpec::all()
        .iter()
        .map(|spec| {
            let trap_id =
                state.place_trap(Position { x: 4, y: 4 }, spec.damage(), spec.effect_id());
            let trap = state.traps.iter().find(|entry| entry.id == trap_id);
            let pass = trap.is_some_and(|entry| {
                entry.armed && entry.damage == spec.damage() && entry.effect_id == spec.effect_id()
            });
            let detail = trap
                .map(|entry| {
                    format!(
                        "trap_id={} damage={} effect={}",
                        entry.id, entry.damage, entry.effect_id
                    )
                })
                .unwrap_or_else(|| "trap placement failed".to_string());
            CheckResult { name: format!("hazard_spec:{}", spec.label()), pass, detail }
        })
        .collect()
}

fn check_catastrophe_suite() -> Vec<CheckResult> {
    let mut checks = Vec::new();
    let mut grid = CaGrid::new(50, 50);
    let mut wind = WindGrid::new(50, 50);

    Catastrophe::great_flood(&mut grid, (25, 25));
    let water_cells = grid
        .front_buffer()
        .iter()
        .filter(|cell| cell.liquid == Some(Liquid::Water) || cell.wet >= 100)
        .count();
    checks.push(CheckResult {
        name: "catastrophe:great_flood".to_string(),
        pass: water_cells > 0,
        detail: format!("water_cells={water_cells}"),
    });

    Catastrophe::forest_fire_jump(&mut grid, (25, 25));
    let fire_cells = grid
        .front_buffer()
        .iter()
        .filter(|cell| cell.gas == Some(Gas::Fire) || cell.heat >= 180)
        .count();
    checks.push(CheckResult {
        name: "catastrophe:forest_fire".to_string(),
        pass: fire_cells > 0,
        detail: format!("fire_cells={fire_cells}"),
    });

    Catastrophe::massive_windstorm(&mut wind);
    let wind_cell = wind.get(0, 0);
    checks.push(CheckResult {
        name: "catastrophe:windstorm".to_string(),
        pass: wind_cell.strength > 0,
        detail: format!("vector=({}, {}, {})", wind_cell.dx, wind_cell.dy, wind_cell.strength),
    });

    let cfg = Catastrophe::doomsday(&mut grid, &mut wind);
    checks.push(CheckResult {
        name: "catastrophe:doomsday_config".to_string(),
        pass: cfg.active && cfg.fire_rate_hz >= 20.0,
        detail: format!("turret.active={} turret.fire_rate_hz={}", cfg.active, cfg.fire_rate_hz),
    });
    checks
}

fn check_arena_bootstrap() -> Vec<CheckResult> {
    let mut checks = Vec::new();
    match bootstrap_wizard_arena() {
        Ok((state, _diag)) => {
            let has_bounds = state.bounds.width > 0 && state.bounds.height > 0;
            checks.push(CheckResult {
                name: "bootstrap:wizard_arena_bounds".to_string(),
                pass: has_bounds,
                detail: format!("bounds={}x{}", state.bounds.width, state.bounds.height),
            });
            checks.push(CheckResult {
                name: "bootstrap:wizard_arena_mode".to_string(),
                pass: state.mode == omega_core::GameMode::Modern,
                detail: format!("mode={}", state.mode.as_str()),
            });
        }
        Err(err) => {
            checks.push(CheckResult {
                name: "bootstrap:wizard_arena".to_string(),
                pass: false,
                detail: format!("bootstrap failed: {err}"),
            });
        }
    }
    checks
}
