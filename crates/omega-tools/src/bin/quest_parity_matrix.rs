use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{
    Command, CountryTerrainKind, DeterministicRng, GameState, LegacyEnvironment, Position,
    SITE_AUX_SERVICE_MERC_GUILD, SITE_AUX_SERVICE_THIEVES, SiteInteractionKind, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct QuestCheck {
    id: String,
    label: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct QuestParityMatrix {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<QuestCheck>,
}

fn markdown(matrix: &QuestParityMatrix) -> String {
    let mut out = Vec::new();
    out.push("# Quest Parity Matrix".to_string());
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
            check.label,
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn find_city_tile_with_aux(state: &GameState, aux: i32) -> Option<Position> {
    let width = usize::try_from(state.bounds.width).ok()?;
    state.site_grid.iter().enumerate().find_map(|(idx, cell)| {
        if cell.aux != aux {
            return None;
        }
        let x = i32::try_from(idx % width).ok()?;
        let y = i32::try_from(idx / width).ok()?;
        Some(Position { x, y })
    })
}

fn find_country_terrain(state: &GameState, terrain: CountryTerrainKind) -> Option<Position> {
    let width = usize::try_from(state.country_grid.width).ok()?;
    state.country_grid.cells.iter().enumerate().find_map(|(idx, cell)| {
        if cell.base_terrain != terrain {
            return None;
        }
        let x = i32::try_from(idx % width).ok()?;
        let y = i32::try_from(idx / width).ok()?;
        Some(Position { x, y })
    })
}

fn probe_country_entry(terrain: CountryTerrainKind) -> Result<(LegacyEnvironment, u16, String)> {
    let (mut state, _diagnostics) =
        bootstrap_game_state_from_default_content().context("bootstrap state")?;
    state.activate_country_view();
    let Some(pos) = find_country_terrain(&state, terrain) else {
        bail!("terrain {:?} not found in country grid", terrain);
    };
    state.player.position = pos;
    let mut rng = DeterministicRng::seeded(0x51E7E);
    let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
    Ok((state.environment, state.map_binding.map_id, format!("entry_pos=({}, {})", pos.x, pos.y)))
}

fn probe_thieves_track() -> Result<(bool, String)> {
    let (mut state, _diagnostics) =
        bootstrap_game_state_from_default_content().context("bootstrap state")?;
    state.options.interactive_sites = true;
    state.gold = 260;
    state.monsters_defeated = 28;
    state.progression.alignment = omega_core::Alignment::Chaotic;
    state.pending_site_interaction = Some(SiteInteractionKind::ThievesGuild);
    let mut rng = DeterministicRng::seeded(0x71E7E5);

    let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "2".to_string() }, &mut rng);
    let _ = step(&mut state, Command::Legacy { token: "3".to_string() }, &mut rng);

    let pass = state.progression.quests.thieves.rank >= 2
        && state.progression.quests.thieves.xp > 0
        && state.progression.main_quest.chaos_path;
    let details = format!(
        "rank={} xp={} chaos_path={} gold={} legal_heat={}",
        state.progression.quests.thieves.rank,
        state.progression.quests.thieves.xp,
        state.progression.main_quest.chaos_path,
        state.gold,
        state.legal_heat
    );
    Ok((pass, details))
}

fn probe_progression_migration_sync() -> Result<(bool, String)> {
    let (mut state, _diagnostics) =
        bootstrap_game_state_from_default_content().context("bootstrap state")?;
    state.progression.guild_rank = 3;
    state.progression.priest_rank = 2;
    state.progression.arena_rank = 2;
    let mut rng = DeterministicRng::seeded(0xABCDEF);
    let _ = step(&mut state, Command::Wait, &mut rng);
    let from_legacy_ok = state.progression.quests.merc.rank >= 3
        && state.progression.quests.temple.rank >= 2
        && state.progression.quests.arena.rank >= 2;

    state.progression.quests.merc.rank = 4;
    state.progression.quests.temple.rank = 3;
    state.progression.quests.arena.rank = 4;
    let _ = step(&mut state, Command::Wait, &mut rng);
    let to_legacy_ok = state.progression.guild_rank >= 4 && state.progression.priest_rank >= 3;

    let pass = from_legacy_ok && to_legacy_ok;
    let details = format!(
        "legacy->tracks={} tracks->legacy={} g={} p={} arena={} track(g/p/a)=({}/{}/{})",
        from_legacy_ok,
        to_legacy_ok,
        state.progression.guild_rank,
        state.progression.priest_rank,
        state.progression.arena_rank,
        state.progression.quests.merc.rank,
        state.progression.quests.temple.rank,
        state.progression.quests.arena.rank
    );
    Ok((pass, details))
}

fn probe_city_service_split() -> Result<(bool, String)> {
    let (state, _diagnostics) =
        bootstrap_game_state_from_default_content().context("bootstrap state")?;
    let thieves = find_city_tile_with_aux(&state, SITE_AUX_SERVICE_THIEVES);
    let merc = find_city_tile_with_aux(&state, SITE_AUX_SERVICE_MERC_GUILD);
    let pass = thieves.is_some() && merc.is_some() && thieves != merc;
    let details = format!("thieves={:?} merc={:?}", thieves, merc);
    Ok((pass, details))
}

fn main() -> Result<()> {
    let mut checks = Vec::new();

    let contract_exists = Path::new("target/legacy-quest-contract.json").exists();
    checks.push(QuestCheck {
        id: "legacy_contract_artifact".to_string(),
        label: "Legacy quest contract artifact exists".to_string(),
        passed: contract_exists,
        details: "target/legacy-quest-contract.json".to_string(),
    });

    let (castle_env, castle_map_id, castle_note) = probe_country_entry(CountryTerrainKind::Castle)?;
    checks.push(QuestCheck {
        id: "castle_entry_environment".to_string(),
        label: "Castle terrain enters castle environment".to_string(),
        passed: castle_env == LegacyEnvironment::Castle && castle_map_id == 5,
        details: format!("env={:?} map_id={} {}", castle_env, castle_map_id, castle_note),
    });

    let (palace_env, palace_map_id, palace_note) = probe_country_entry(CountryTerrainKind::Palace)?;
    checks.push(QuestCheck {
        id: "palace_entry_environment".to_string(),
        label: "Palace terrain enters palace environment".to_string(),
        passed: palace_env == LegacyEnvironment::Palace && palace_map_id == 5,
        details: format!("env={:?} map_id={} {}", palace_env, palace_map_id, palace_note),
    });

    let (service_split_ok, service_split_details) = probe_city_service_split()?;
    checks.push(QuestCheck {
        id: "thieves_service_split".to_string(),
        label: "Thieves and merc services are distinct in city map".to_string(),
        passed: service_split_ok,
        details: service_split_details,
    });

    let (thieves_ok, thieves_details) = probe_thieves_track()?;
    checks.push(QuestCheck {
        id: "thieves_track_progression".to_string(),
        label: "Thieves guild progression updates track state".to_string(),
        passed: thieves_ok,
        details: thieves_details,
    });

    let (sync_ok, sync_details) = probe_progression_migration_sync()?;
    checks.push(QuestCheck {
        id: "progression_schema_sync".to_string(),
        label: "Legacy scalar progression syncs with per-track schema".to_string(),
        passed: sync_ok,
        details: sync_details,
    });

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;

    let matrix = QuestParityMatrix { total, passed, failed, pass, checks };
    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }

    let json_path = target.join("quest-parity-matrix.json");
    let md_path = target.join("quest-parity-matrix.md");
    fs::write(&json_path, serde_json::to_string_pretty(&matrix).context("serialize quest matrix")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&matrix))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "quest parity matrix: total={}, passed={}, failed={}",
        matrix.total, matrix.passed, matrix.failed
    );
    if !matrix.pass {
        bail!("quest parity matrix failed");
    }
    Ok(())
}
