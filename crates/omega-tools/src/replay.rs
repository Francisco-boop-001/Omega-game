use anyhow::{Context, Result};
use omega_core::{
    Alignment, Command, DeterministicRng, Direction, EndingKind, Event, GameState,
    LegacyEnvironment, LegacyQuestState, MapBounds, Position, Stats, TileSiteCell, WorldMode, step,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReplayCommand {
    Wait,
    Move { direction: ReplayDirection },
    Attack { direction: ReplayDirection },
    Pickup,
    Drop { slot: usize },
    Legacy { token: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReplayDirection {
    North,
    South,
    East,
    West,
}

impl From<ReplayDirection> for Direction {
    fn from(value: ReplayDirection) -> Self {
        match value {
            ReplayDirection::North => Direction::North,
            ReplayDirection::South => Direction::South,
            ReplayDirection::East => Direction::East,
            ReplayDirection::West => Direction::West,
        }
    }
}

impl ReplayCommand {
    pub fn into_command(self) -> Command {
        match self {
            ReplayCommand::Wait => Command::Wait,
            ReplayCommand::Move { direction } => Command::Move(direction.into()),
            ReplayCommand::Attack { direction } => Command::Attack(direction.into()),
            ReplayCommand::Pickup => Command::Pickup,
            ReplayCommand::Drop { slot } => Command::Drop { slot },
            ReplayCommand::Legacy { token } => Command::Legacy { token },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayMonsterSpec {
    pub name: String,
    pub position: Position,
    pub stats: Stats,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayItemSpec {
    pub name: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayInitialState {
    pub bounds: MapBounds,
    pub player_position: Position,
    pub player_stats: Option<Stats>,
    pub inventory_capacity: Option<usize>,
    pub monsters: Vec<ReplayMonsterSpec>,
    pub ground_items: Vec<ReplayItemSpec>,
    #[serde(default)]
    pub world_mode: Option<WorldMode>,
    #[serde(default)]
    pub environment: Option<LegacyEnvironment>,
    #[serde(default)]
    pub map_rows: Option<Vec<String>>,
    #[serde(default)]
    pub site_aux_grid: Option<Vec<i32>>,
    #[serde(default)]
    pub site_flags_grid: Option<Vec<u16>>,
    #[serde(default)]
    pub gold: Option<i32>,
    #[serde(default)]
    pub bank_gold: Option<i32>,
    #[serde(default)]
    pub food: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayExpected {
    pub turn: u64,
    pub minutes: u64,
    pub player_position: Position,
    pub player_hp: i32,
    pub monsters_alive: usize,
    pub inventory_count: usize,
    pub ground_item_count: usize,
    pub required_event_kinds: Vec<String>,
    #[serde(default)]
    pub world_mode: Option<WorldMode>,
    #[serde(default)]
    pub guild_rank: Option<u8>,
    #[serde(default)]
    pub priest_rank: Option<u8>,
    #[serde(default)]
    pub alignment: Option<Alignment>,
    #[serde(default)]
    pub quest_state: Option<LegacyQuestState>,
    #[serde(default)]
    pub total_winner_unlocked: Option<bool>,
    #[serde(default)]
    pub gold: Option<i32>,
    #[serde(default)]
    pub bank_gold: Option<i32>,
    #[serde(default)]
    pub food: Option<i32>,
    #[serde(default)]
    pub known_site_count: Option<usize>,
    #[serde(default)]
    pub ending: Option<EndingKind>,
    #[serde(default)]
    pub high_score_eligible: Option<bool>,
}

const REPLAY_CONTRACT_VERSION: u16 = 1;

fn default_contract_version() -> u16 {
    REPLAY_CONTRACT_VERSION
}

fn default_active() -> bool {
    true
}

fn default_source() -> String {
    "legacy_handcrafted".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayFixture {
    #[serde(default = "default_contract_version")]
    pub contract_version: u16,
    #[serde(default = "default_active")]
    pub active: bool,
    #[serde(default = "default_source")]
    pub source: String,
    pub name: String,
    #[serde(default)]
    pub family: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub seed: u64,
    pub initial: ReplayInitialState,
    pub commands: Vec<ReplayCommand>,
    pub expected: ReplayExpected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayScenarioResult {
    pub name: String,
    pub family: String,
    pub tags: Vec<String>,
    pub source: String,
    pub active: bool,
    pub deprecated: bool,
    pub schema_mismatch: bool,
    pub passed: bool,
    pub checks: Vec<String>,
    pub final_turn: u64,
    pub final_minutes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rollup {
    pub key: String,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegressionDashboard {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub active_total: usize,
    pub active_passed: usize,
    pub failed_active: usize,
    pub inactive_total: usize,
    pub inactive_passed: usize,
    pub failed_inactive: usize,
    pub schema_mismatch_total: usize,
    pub pass_rate_percent_x100: u64,
    pub critical_path_total: usize,
    pub critical_path_passed: usize,
    pub critical_path_failed: usize,
    pub tag_rollups: Vec<Rollup>,
    pub family_rollups: Vec<Rollup>,
    pub scenarios: Vec<ReplayScenarioResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FixtureTrace {
    pub final_state: GameState,
    pub seen_event_kinds: Vec<String>,
}

pub fn event_kind(event: &Event) -> &'static str {
    match event {
        Event::Waited => "waited",
        Event::Moved { .. } => "moved",
        Event::MoveBlocked { .. } => "move_blocked",
        Event::AttackMissed { .. } => "attack_missed",
        Event::Attacked { .. } => "attacked",
        Event::MonsterMoved { .. } => "monster_moved",
        Event::MonsterAttacked { .. } => "monster_attacked",
        Event::MonsterDefeated { .. } => "monster_defeated",
        Event::PlayerDefeated => "player_defeated",
        Event::VictoryAchieved => "victory_achieved",
        Event::CommandIgnoredTerminal { .. } => "command_ignored_terminal",
        Event::PickedUp { .. } => "picked_up",
        Event::Dropped { .. } => "dropped",
        Event::InventoryFull { .. } => "inventory_full",
        Event::NoItemToPickUp => "no_item_to_pick_up",
        Event::InvalidDropSlot { .. } => "invalid_drop_slot",
        Event::LegacyHandled { .. } => "legacy_handled",
        Event::ConfirmationRequired { .. } => "confirmation_required",
        Event::EconomyUpdated { .. } => "economy_updated",
        Event::DialogueAdvanced { .. } => "dialogue_advanced",
        Event::QuestAdvanced { .. } => "quest_advanced",
        Event::ProgressionUpdated { .. } => "progression_updated",
        Event::EndingResolved { .. } => "ending_resolved",
        Event::ActionPointsSpent { .. } => "action_points_spent",
        Event::StatusTick { .. } => "status_tick",
        Event::StatusExpired { .. } => "status_expired",
        Event::TurnAdvanced { .. } => "turn_advanced",
    }
}

pub fn load_fixture(path: &Path) -> Result<ReplayFixture> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read fixture: {}", path.display()))?;
    let mut fixture: ReplayFixture = serde_json::from_str(&raw)
        .with_context(|| format!("invalid fixture json: {}", path.display()))?;
    normalize_fixture_metadata(&mut fixture);
    Ok(fixture)
}

fn normalize_fixture_metadata(fixture: &mut ReplayFixture) {
    fixture.tags.retain(|tag| !tag.trim().is_empty());
    fixture.tags.sort();
    fixture.tags.dedup();
    if fixture.family.trim().is_empty() {
        fixture.family = "unclassified".to_string();
    }
    if fixture.source.trim().is_empty() {
        fixture.source = default_source();
    }
}

fn is_supported_contract_version(version: u16) -> bool {
    version <= REPLAY_CONTRACT_VERSION
}

fn run_fixture_internal(fixture: &ReplayFixture) -> FixtureTrace {
    let mut state = GameState::new(fixture.initial.bounds);
    state.player.position = fixture.initial.player_position;

    if let Some(stats) = fixture.initial.player_stats {
        state.player.stats = stats;
    }
    if let Some(capacity) = fixture.initial.inventory_capacity {
        state.player.inventory_capacity = capacity;
    }
    if let Some(world_mode) = fixture.initial.world_mode {
        state.world_mode = world_mode;
    }
    if let Some(environment) = fixture.initial.environment {
        state.environment = environment;
    }
    if let Some(map_rows) = fixture.initial.map_rows.clone() {
        state.set_map_rows(map_rows.clone());
        state.city_map_rows = map_rows.clone();
        state.country_map_rows = map_rows;
    }
    if let Some(site_aux_grid) = fixture.initial.site_aux_grid.clone() {
        let flags =
            fixture.initial.site_flags_grid.clone().unwrap_or_else(|| vec![0; site_aux_grid.len()]);
        let mut grid = Vec::with_capacity(site_aux_grid.len());
        for (idx, aux) in site_aux_grid.iter().enumerate() {
            let flag = flags.get(idx).copied().unwrap_or(0);
            grid.push(TileSiteCell { glyph: '.', site_id: 0, aux: *aux, flags: flag });
        }
        state.site_grid = grid.clone();
        state.city_site_grid = grid;
    }
    if let Some(gold) = fixture.initial.gold {
        state.gold = gold;
    }
    if let Some(bank_gold) = fixture.initial.bank_gold {
        state.bank_gold = bank_gold;
    }
    if let Some(food) = fixture.initial.food {
        state.food = food;
    }

    for monster in &fixture.initial.monsters {
        state.spawn_monster(monster.name.clone(), monster.position, monster.stats);
    }
    for item in &fixture.initial.ground_items {
        state.place_item(item.name.clone(), item.position);
    }

    let mut rng = DeterministicRng::seeded(fixture.seed);
    let mut seen_event_kinds = Vec::new();

    for command in &fixture.commands {
        let outcome = step(&mut state, command.clone().into_command(), &mut rng);
        for event in &outcome.events {
            seen_event_kinds.push(event_kind(event).to_string());
        }
    }

    FixtureTrace { final_state: state, seen_event_kinds }
}

pub fn run_fixture_trace(fixture: &ReplayFixture) -> FixtureTrace {
    run_fixture_internal(fixture)
}

pub fn fixture_has_tag(fixture: &ReplayFixture, tag: &str) -> bool {
    fixture.tags.iter().any(|candidate| candidate == tag)
}

pub fn run_fixture(fixture: &ReplayFixture) -> ReplayScenarioResult {
    let schema_mismatch = !is_supported_contract_version(fixture.contract_version);
    if schema_mismatch {
        return ReplayScenarioResult {
            name: fixture.name.clone(),
            family: fixture.family.clone(),
            tags: fixture.tags.clone(),
            source: fixture.source.clone(),
            active: fixture.active,
            deprecated: !fixture.active,
            schema_mismatch: true,
            passed: false,
            checks: vec![format!(
                "schema mismatch: contract_version={} supported_max={}",
                fixture.contract_version, REPLAY_CONTRACT_VERSION
            )],
            final_turn: 0,
            final_minutes: 0,
        };
    }

    let mut checks = Vec::new();
    let trace = run_fixture_internal(fixture);
    let state = &trace.final_state;
    let mut passed = true;

    if state.clock.turn != fixture.expected.turn {
        passed = false;
        checks.push(format!(
            "turn mismatch: expected {}, got {}",
            fixture.expected.turn, state.clock.turn
        ));
    }

    if state.clock.minutes != fixture.expected.minutes {
        passed = false;
        checks.push(format!(
            "minutes mismatch: expected {}, got {}",
            fixture.expected.minutes, state.clock.minutes
        ));
    }

    if state.player.position != fixture.expected.player_position {
        passed = false;
        checks.push(format!(
            "player_position mismatch: expected ({}, {}), got ({}, {})",
            fixture.expected.player_position.x,
            fixture.expected.player_position.y,
            state.player.position.x,
            state.player.position.y
        ));
    }

    if state.player.stats.hp != fixture.expected.player_hp {
        passed = false;
        checks.push(format!(
            "player_hp mismatch: expected {}, got {}",
            fixture.expected.player_hp, state.player.stats.hp
        ));
    }

    if state.monsters.len() != fixture.expected.monsters_alive {
        passed = false;
        checks.push(format!(
            "monsters_alive mismatch: expected {}, got {}",
            fixture.expected.monsters_alive,
            state.monsters.len()
        ));
    }

    if state.player.inventory.len() != fixture.expected.inventory_count {
        passed = false;
        checks.push(format!(
            "inventory_count mismatch: expected {}, got {}",
            fixture.expected.inventory_count,
            state.player.inventory.len()
        ));
    }

    if state.ground_items.len() != fixture.expected.ground_item_count {
        passed = false;
        checks.push(format!(
            "ground_item_count mismatch: expected {}, got {}",
            fixture.expected.ground_item_count,
            state.ground_items.len()
        ));
    }

    if let Some(expected_world_mode) = fixture.expected.world_mode
        && state.world_mode != expected_world_mode
    {
        passed = false;
        checks.push(format!(
            "world_mode mismatch: expected {:?}, got {:?}",
            expected_world_mode, state.world_mode
        ));
    }

    if let Some(expected_guild_rank) = fixture.expected.guild_rank
        && state.progression.guild_rank != expected_guild_rank
    {
        passed = false;
        checks.push(format!(
            "guild_rank mismatch: expected {}, got {}",
            expected_guild_rank, state.progression.guild_rank
        ));
    }

    if let Some(expected_priest_rank) = fixture.expected.priest_rank
        && state.progression.priest_rank != expected_priest_rank
    {
        passed = false;
        checks.push(format!(
            "priest_rank mismatch: expected {}, got {}",
            expected_priest_rank, state.progression.priest_rank
        ));
    }

    if let Some(expected_alignment) = fixture.expected.alignment
        && state.progression.alignment != expected_alignment
    {
        passed = false;
        checks.push(format!(
            "alignment mismatch: expected {:?}, got {:?}",
            expected_alignment, state.progression.alignment
        ));
    }

    if let Some(expected_quest_state) = fixture.expected.quest_state
        && state.progression.quest_state != expected_quest_state
    {
        passed = false;
        checks.push(format!(
            "quest_state mismatch: expected {:?}, got {:?}",
            expected_quest_state, state.progression.quest_state
        ));
    }

    if let Some(expected_total_winner) = fixture.expected.total_winner_unlocked
        && state.progression.total_winner_unlocked != expected_total_winner
    {
        passed = false;
        checks.push(format!(
            "total_winner_unlocked mismatch: expected {}, got {}",
            expected_total_winner, state.progression.total_winner_unlocked
        ));
    }

    if let Some(expected_gold) = fixture.expected.gold
        && state.gold != expected_gold
    {
        passed = false;
        checks.push(format!("gold mismatch: expected {}, got {}", expected_gold, state.gold));
    }

    if let Some(expected_bank_gold) = fixture.expected.bank_gold
        && state.bank_gold != expected_bank_gold
    {
        passed = false;
        checks.push(format!(
            "bank_gold mismatch: expected {}, got {}",
            expected_bank_gold, state.bank_gold
        ));
    }

    if let Some(expected_food) = fixture.expected.food
        && state.food != expected_food
    {
        passed = false;
        checks.push(format!("food mismatch: expected {}, got {}", expected_food, state.food));
    }

    if let Some(expected_known_site_count) = fixture.expected.known_site_count
        && state.known_sites.len() != expected_known_site_count
    {
        passed = false;
        checks.push(format!(
            "known_site_count mismatch: expected {}, got {}",
            expected_known_site_count,
            state.known_sites.len()
        ));
    }

    if let Some(expected_ending) = fixture.expected.ending
        && state.progression.ending != expected_ending
    {
        passed = false;
        checks.push(format!(
            "ending mismatch: expected {:?}, got {:?}",
            expected_ending, state.progression.ending
        ));
    }

    if let Some(expected_high_score_eligible) = fixture.expected.high_score_eligible
        && state.progression.high_score_eligible != expected_high_score_eligible
    {
        passed = false;
        checks.push(format!(
            "high_score_eligible mismatch: expected {}, got {}",
            expected_high_score_eligible, state.progression.high_score_eligible
        ));
    }

    for required in &fixture.expected.required_event_kinds {
        if !trace.seen_event_kinds.iter().any(|seen| seen == required) {
            passed = false;
            checks.push(format!("required event kind missing: {}", required));
        }
    }

    if passed {
        checks.push("all checks passed".to_string());
    }

    ReplayScenarioResult {
        name: fixture.name.clone(),
        family: fixture.family.clone(),
        tags: fixture.tags.clone(),
        source: fixture.source.clone(),
        active: fixture.active,
        deprecated: !fixture.active,
        schema_mismatch: false,
        passed,
        checks,
        final_turn: state.clock.turn,
        final_minutes: state.clock.minutes,
    }
}

pub fn collect_fixtures(dir: &Path) -> Result<Vec<(PathBuf, ReplayFixture)>> {
    let mut paths = Vec::new();
    collect_json_paths_recursive(dir, &mut paths)?;

    paths.sort();

    let mut fixtures = Vec::new();
    for path in paths {
        fixtures.push((path.clone(), load_fixture(&path)?));
    }
    Ok(fixtures)
}

fn collect_json_paths_recursive(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_json_paths_recursive(&path, out)?;
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            out.push(path);
        }
    }
    Ok(())
}

pub fn run_dashboard_from_dir(dir: &Path) -> Result<RegressionDashboard> {
    let fixtures = collect_fixtures(dir)?;
    let scenarios: Vec<ReplayScenarioResult> =
        fixtures.iter().map(|(_, fixture)| run_fixture(fixture)).collect();

    let total = scenarios.len();
    let passed = scenarios.iter().filter(|scenario| scenario.passed).count();
    let failed = total - passed;
    let active_total = scenarios.iter().filter(|scenario| scenario.active).count();
    let active_passed =
        scenarios.iter().filter(|scenario| scenario.active && scenario.passed).count();
    let failed_active = active_total.saturating_sub(active_passed);
    let inactive_total = total.saturating_sub(active_total);
    let inactive_passed =
        scenarios.iter().filter(|scenario| !scenario.active && scenario.passed).count();
    let failed_inactive = inactive_total.saturating_sub(inactive_passed);
    let schema_mismatch_total =
        scenarios.iter().filter(|scenario| scenario.schema_mismatch).count();
    let pass_rate_percent_x100 =
        if total == 0 { 0 } else { ((passed as u64) * 10_000) / (total as u64) };

    let critical_path_total = scenarios
        .iter()
        .filter(|scenario| scenario.tags.iter().any(|tag| tag == "critical_path"))
        .count();
    let critical_path_passed = scenarios
        .iter()
        .filter(|scenario| {
            scenario.passed && scenario.tags.iter().any(|tag| tag == "critical_path")
        })
        .count();
    let critical_path_failed = critical_path_total.saturating_sub(critical_path_passed);

    let tag_rollups = build_tag_rollups(
        scenarios.iter().flat_map(|scenario| scenario.tags.iter().cloned()).collect(),
        &scenarios,
    );
    let family_rollups = build_family_rollups(
        scenarios.iter().map(|scenario| scenario.family.clone()).collect(),
        &scenarios,
    );

    Ok(RegressionDashboard {
        total,
        passed,
        failed,
        active_total,
        active_passed,
        failed_active,
        inactive_total,
        inactive_passed,
        failed_inactive,
        schema_mismatch_total,
        pass_rate_percent_x100,
        critical_path_total,
        critical_path_passed,
        critical_path_failed,
        tag_rollups,
        family_rollups,
        scenarios,
    })
}

fn build_tag_rollups(keys: Vec<String>, scenarios: &[ReplayScenarioResult]) -> Vec<Rollup> {
    let unique: BTreeSet<String> = keys.into_iter().collect();
    let mut rollups = Vec::new();

    for key in unique {
        let mut total = 0usize;
        let mut passed = 0usize;
        for scenario in scenarios {
            let matches = scenario.tags.iter().any(|tag| tag == &key);
            if matches {
                total += 1;
                if scenario.passed {
                    passed += 1;
                }
            }
        }
        let failed = total.saturating_sub(passed);
        rollups.push(Rollup { key, total, passed, failed });
    }
    rollups
}

fn build_family_rollups(keys: Vec<String>, scenarios: &[ReplayScenarioResult]) -> Vec<Rollup> {
    let unique: BTreeSet<String> = keys.into_iter().collect();
    let mut rollups = Vec::new();

    for key in unique {
        let mut total = 0usize;
        let mut passed = 0usize;
        for scenario in scenarios {
            let matches = scenario.family == key;
            if matches {
                total += 1;
                if scenario.passed {
                    passed += 1;
                }
            }
        }
        let failed = total.saturating_sub(passed);
        rollups.push(Rollup { key, total, passed, failed });
    }
    rollups
}

pub fn dashboard_markdown(dashboard: &RegressionDashboard) -> String {
    let mut lines = Vec::new();
    lines.push("# WS-D Regression Dashboard".to_string());
    lines.push(String::new());
    lines.push(format!("- Total scenarios: {}", dashboard.total));
    lines.push(format!("- Passed: {}", dashboard.passed));
    lines.push(format!("- Failed: {}", dashboard.failed));
    lines.push(format!(
        "- Active denominator: total={}, passed={}, failed={}",
        dashboard.active_total, dashboard.active_passed, dashboard.failed_active
    ));
    lines.push(format!(
        "- Inactive scenarios: total={}, passed={}, failed={}",
        dashboard.inactive_total, dashboard.inactive_passed, dashboard.failed_inactive
    ));
    lines.push(format!("- Schema mismatches: {}", dashboard.schema_mismatch_total));
    lines.push(format!(
        "- Pass rate: {}.{:02}%",
        dashboard.pass_rate_percent_x100 / 100,
        dashboard.pass_rate_percent_x100 % 100
    ));
    lines.push(format!(
        "- Critical path: total={}, passed={}, failed={}",
        dashboard.critical_path_total,
        dashboard.critical_path_passed,
        dashboard.critical_path_failed
    ));
    lines.push(String::new());

    if !dashboard.tag_rollups.is_empty() {
        lines.push("## Tag Rollups".to_string());
        lines.push(String::new());
        lines.push("| Tag | Total | Passed | Failed |".to_string());
        lines.push("|---|---:|---:|---:|".to_string());
        for rollup in &dashboard.tag_rollups {
            lines.push(format!(
                "| {} | {} | {} | {} |",
                rollup.key, rollup.total, rollup.passed, rollup.failed
            ));
        }
        lines.push(String::new());
    }

    if !dashboard.family_rollups.is_empty() {
        lines.push("## Family Rollups".to_string());
        lines.push(String::new());
        lines.push("| Family | Total | Passed | Failed |".to_string());
        lines.push("|---|---:|---:|---:|".to_string());
        for rollup in &dashboard.family_rollups {
            lines.push(format!(
                "| {} | {} | {} | {} |",
                rollup.key, rollup.total, rollup.passed, rollup.failed
            ));
        }
        lines.push(String::new());
    }

    lines.push("## Scenario Sample".to_string());
    lines.push(String::new());
    lines.push(
        "| Scenario | Family | Active | Source | Schema | Result | Final Turn | Final Minutes |"
            .to_string(),
    );
    lines.push("|---|---|---|---|---|---|---:|---:|".to_string());

    let sample_limit = 120usize;
    for scenario in dashboard.scenarios.iter().take(sample_limit) {
        let status = if scenario.passed { "PASS" } else { "FAIL" };
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            scenario.name,
            scenario.family,
            if scenario.active { "yes" } else { "no" },
            scenario.source,
            if scenario.schema_mismatch { "mismatch" } else { "ok" },
            status,
            scenario.final_turn,
            scenario.final_minutes
        ));
    }
    if dashboard.scenarios.len() > sample_limit {
        lines.push(format!(
            "| ... {} more | - | - | - | - | - | - | - |",
            dashboard.scenarios.len() - sample_limit
        ));
    }

    lines.push(String::new());

    for scenario in &dashboard.scenarios {
        if !scenario.passed {
            lines.push(format!("## Failure: {}", scenario.name));
            lines.push(format!(
                "- active={} source={} schema_mismatch={}",
                scenario.active, scenario.source, scenario.schema_mismatch
            ));
            for check in &scenario.checks {
                lines.push(format!("- {}", check));
            }
            lines.push(String::new());
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_kind_maps_expected_names() {
        assert_eq!(event_kind(&Event::Waited), "waited");
        assert_eq!(event_kind(&Event::NoItemToPickUp), "no_item_to_pick_up");
    }
}
