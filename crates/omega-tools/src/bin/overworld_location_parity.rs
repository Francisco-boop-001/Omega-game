use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{
    Command, CountryTerrainKind, DeterministicRng, GameState, LegacyEnvironment, MapSemanticKind,
    Position, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LocationCheck {
    id: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LocationMatrix {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<LocationCheck>,
}

fn markdown(matrix: &LocationMatrix) -> String {
    let mut out = Vec::new();
    out.push("# Overworld Location Parity Matrix".to_string());
    out.push(String::new());
    out.push(format!("- Total checks: {}", matrix.total));
    out.push(format!("- Passed: {}", matrix.passed));
    out.push(format!("- Failed: {}", matrix.failed));
    out.push(format!("- Status: {}", if matrix.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Check | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for check in &matrix.checks {
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

fn find_country_position_for(
    state: &GameState,
    terrain: CountryTerrainKind,
    aux: Option<u8>,
) -> Option<Position> {
    let width = usize::try_from(state.country_grid.width).ok()?;
    state.country_grid.cells.iter().enumerate().find_map(|(idx, cell)| {
        if cell.base_terrain != terrain {
            return None;
        }
        if let Some(required_aux) = aux
            && cell.aux != required_aux
        {
            return None;
        }
        let x = i32::try_from(idx % width).ok()?;
        let y = i32::try_from(idx / width).ok()?;
        Some(Position { x, y })
    })
}

fn check_entry(
    base: &GameState,
    id: &str,
    terrain: CountryTerrainKind,
    aux: Option<u8>,
    expected_map: u16,
    expected_env: LegacyEnvironment,
    expected_semantic: MapSemanticKind,
) -> LocationCheck {
    let Some(pos) = find_country_position_for(base, terrain, aux) else {
        return LocationCheck {
            id: id.to_string(),
            passed: false,
            details: format!("missing terrain {:?} aux {:?}", terrain, aux),
        };
    };

    let mut state = base.clone();
    let mut rng = DeterministicRng::seeded(0x5151_0000 + u64::from(expected_map));
    let _ = step(&mut state, Command::Legacy { token: "<".to_string() }, &mut rng);
    state.player.position = pos;
    let out = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);

    let passed = state.map_binding.map_id == expected_map
        && state.environment == expected_env
        && state.map_binding.semantic == expected_semantic;
    let details = format!(
        "pos=({}, {}) out_turn={} out_minutes={} map={} env={:?} semantic={:?}",
        pos.x,
        pos.y,
        out.turn,
        out.minutes,
        state.map_binding.map_id,
        state.environment,
        state.map_binding.semantic
    );
    LocationCheck { id: id.to_string(), passed, details }
}

fn main() -> Result<()> {
    let (base, _report) = bootstrap_game_state_from_default_content()?;
    let mut checks = vec![
        check_entry(
            &base,
            "city_entry",
            CountryTerrainKind::City,
            None,
            3,
            LegacyEnvironment::City,
            MapSemanticKind::City,
        ),
        check_entry(
            &base,
            "castle_entry",
            CountryTerrainKind::Castle,
            None,
            5,
            LegacyEnvironment::Castle,
            MapSemanticKind::Site,
        ),
        check_entry(
            &base,
            "palace_entry",
            CountryTerrainKind::Palace,
            None,
            5,
            LegacyEnvironment::Palace,
            MapSemanticKind::Site,
        ),
        check_entry(
            &base,
            "caves_entry",
            CountryTerrainKind::Caves,
            None,
            2,
            LegacyEnvironment::Caves,
            MapSemanticKind::Site,
        ),
        check_entry(
            &base,
            "volcano_entry",
            CountryTerrainKind::Volcano,
            None,
            4,
            LegacyEnvironment::Volcano,
            MapSemanticKind::Site,
        ),
        check_entry(
            &base,
            "dragon_lair_entry",
            CountryTerrainKind::DragonLair,
            None,
            6,
            LegacyEnvironment::DragonLair,
            MapSemanticKind::Site,
        ),
        check_entry(
            &base,
            "star_peak_entry",
            CountryTerrainKind::StarPeak,
            None,
            13,
            LegacyEnvironment::StarPeak,
            MapSemanticKind::Site,
        ),
        check_entry(
            &base,
            "magic_isle_entry",
            CountryTerrainKind::MagicIsle,
            None,
            11,
            LegacyEnvironment::MagicIsle,
            MapSemanticKind::Site,
        ),
    ];

    for aux in 1u8..=6u8 {
        checks.push(check_entry(
            &base,
            &format!("village_{aux}_entry"),
            CountryTerrainKind::Village,
            Some(aux),
            match aux {
                1 => 14,
                2 => 19,
                3 => 15,
                4 => 17,
                5 => 12,
                6 => 18,
                _ => 0,
            },
            LegacyEnvironment::Village,
            MapSemanticKind::Site,
        ));
    }

    for aux in 1u8..=6u8 {
        checks.push(check_entry(
            &base,
            &format!("temple_{aux}_entry"),
            CountryTerrainKind::Temple,
            Some(aux),
            16,
            LegacyEnvironment::Temple,
            MapSemanticKind::Site,
        ));
    }

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let matrix = LocationMatrix { total, passed, failed, pass: failed == 0, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("overworld-location-parity.json");
    let md_path = target.join("overworld-location-parity.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&matrix).context("serialize overworld location matrix")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&matrix))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "overworld location parity: total={}, passed={}, failed={}",
        matrix.total, matrix.passed, matrix.failed
    );
    if !matrix.pass {
        bail!("overworld location parity matrix failed");
    }
    Ok(())
}
