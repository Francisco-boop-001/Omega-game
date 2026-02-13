use anyhow::{Result, bail};
use omega_core::{
    Command, CountryCell, CountryGrid, CountryTerrainKind, GameState, LegacyEnvironment,
    MapBinding, MapBounds, MapSemanticKind, Position, RandomSource, WorldMode, step,
};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
struct Check {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize)]
struct Report {
    checks: Vec<Check>,
    pass: bool,
}

#[derive(Debug, Clone)]
struct ScriptedRng {
    values: Vec<i32>,
    cursor: usize,
}

impl ScriptedRng {
    fn new(values: Vec<i32>) -> Self {
        Self { values, cursor: 0 }
    }
}

impl RandomSource for ScriptedRng {
    fn range_inclusive_i32(&mut self, min: i32, max: i32) -> i32 {
        let value = self.values.get(self.cursor).copied().unwrap_or(min);
        self.cursor = self.cursor.saturating_add(1);
        value.clamp(min, max)
    }
}

fn countryside_state(width: i32, height: i32, terrain: CountryTerrainKind) -> GameState {
    let mut state = GameState::new(MapBounds { width, height });
    state.world_mode = WorldMode::Countryside;
    state.environment = LegacyEnvironment::Countryside;
    state.map_binding = MapBinding {
        semantic: MapSemanticKind::Country,
        map_id: 0,
        level_index: 0,
        source: "runtime/test-country".to_string(),
    };
    state.map_rows = vec![".".repeat(width as usize); height as usize];
    state.country_map_rows = state.map_rows.clone();
    state.country_site_grid = vec![Default::default(); (width * height) as usize];
    state.country_grid = CountryGrid {
        width,
        height,
        cells: vec![
            CountryCell {
                glyph: '.',
                base_terrain: terrain,
                current_terrain: terrain,
                aux: 0,
                status: 0
            };
            (width * height) as usize
        ],
    };
    state
}

fn markdown(report: &Report) -> String {
    let mut lines = Vec::new();
    lines.push("# Legacy Disorientation Parity".to_string());
    lines.push(String::new());
    lines.push(format!("- status: {}", if report.pass { "PASS" } else { "FAIL" }));
    lines.push(String::new());
    for check in &report.checks {
        lines.push(format!(
            "- {}: {} ({})",
            check.id,
            if check.passed { "PASS" } else { "FAIL" },
            check.details
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn main() -> Result<()> {
    let mut checks = Vec::new();

    let mut poppy_state = countryside_state(3, 3, CountryTerrainKind::Plains);
    poppy_state.player.position = Position { x: 0, y: 0 };
    let mut poppy_rng = ScriptedRng::new(vec![1, 100]);
    let _ = step(&mut poppy_state, Command::Move(omega_core::Direction::East), &mut poppy_rng);
    checks.push(Check {
        id: "poppy_disorients_non_terminal".to_string(),
        passed: poppy_state.navigation_lost
            && poppy_state.status == omega_core::SessionStatus::InProgress,
        details: format!(
            "navigation_lost={} status={:?}",
            poppy_state.navigation_lost, poppy_state.status
        ),
    });

    let mut random_move_state = countryside_state(3, 3, CountryTerrainKind::Plains);
    random_move_state.player.position = Position { x: 1, y: 1 };
    random_move_state.navigation_lost = true;
    for y in 0..3 {
        for x in 0..3 {
            random_move_state.known_sites.push(Position { x, y });
        }
    }
    let mut lost_rng = ScriptedRng::new(vec![0, 250, 100]);
    let _ = step(&mut random_move_state, Command::Move(omega_core::Direction::East), &mut lost_rng);
    checks.push(Check {
        id: "lost_randomizes_direction".to_string(),
        passed: random_move_state.player.position == Position { x: 1, y: 0 },
        details: format!(
            "final_pos=({}, {})",
            random_move_state.player.position.x, random_move_state.player.position.y
        ),
    });

    let mut recover_state = countryside_state(3, 3, CountryTerrainKind::Plains);
    recover_state.player.position = Position { x: 1, y: 1 };
    recover_state.navigation_lost = true;
    recover_state.precipitation = 0;
    recover_state.known_sites.push(Position { x: 2, y: 1 });
    let mut recover_rng = ScriptedRng::new(vec![2, 250, 100]);
    let _ = step(&mut recover_state, Command::Move(omega_core::Direction::East), &mut recover_rng);
    checks.push(Check {
        id: "reorientation_on_seen_clear_weather".to_string(),
        passed: !recover_state.navigation_lost,
        details: format!(
            "navigation_lost={} precipitation={}",
            recover_state.navigation_lost, recover_state.precipitation
        ),
    });

    let mut chaos_state = countryside_state(3, 3, CountryTerrainKind::ChaosSea);
    chaos_state.player.position = Position { x: 1, y: 1 };
    let mut chaos_rng = ScriptedRng::new(vec![250, 100]);
    let _ = step(&mut chaos_state, Command::Move(omega_core::Direction::East), &mut chaos_rng);
    checks.push(Check {
        id: "chaos_sea_unprepared_fatal".to_string(),
        passed: chaos_state.status == omega_core::SessionStatus::Lost
            && chaos_state.death_source.as_deref() == Some("immersion in raw Chaos"),
        details: format!(
            "status={:?} death_source={:?}",
            chaos_state.status, chaos_state.death_source
        ),
    });

    let report = Report { pass: checks.iter().all(|check| check.passed), checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target)?;
    }
    fs::write(
        target.join("legacy-disorientation-parity.json"),
        serde_json::to_string_pretty(&report)?,
    )?;
    fs::write(target.join("legacy-disorientation-parity.md"), markdown(&report))?;

    println!(
        "legacy disorientation parity: {}/{} checks passed",
        report.checks.iter().filter(|check| check.passed).count(),
        report.checks.len()
    );

    if !report.pass {
        bail!("legacy disorientation parity failed");
    }
    Ok(())
}
