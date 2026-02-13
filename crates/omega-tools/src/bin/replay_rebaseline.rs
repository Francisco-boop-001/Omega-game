use anyhow::{Context, Result, bail};
use omega_tools::replay::{ReplayFixture, collect_fixtures, run_fixture_trace};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn parse_args() -> Result<PathBuf> {
    let mut dir = PathBuf::from("crates/omega-tools/fixtures/replay");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--fixture-dir" => {
                let value =
                    args.next().ok_or_else(|| anyhow::anyhow!("--fixture-dir requires a value"))?;
                dir = PathBuf::from(value);
            }
            other => bail!("unknown argument: {other}"),
        }
    }
    Ok(dir)
}

fn write_fixture(path: &Path, fixture: &omega_tools::replay::ReplayFixture) -> Result<()> {
    let raw = serde_json::to_string_pretty(fixture).context("serialize fixture")?;
    fs::write(path, raw).with_context(|| format!("write {}", path.display()))
}

fn handcrafted_allowlist(name: &str) -> bool {
    matches!(
        name,
        "combat_defeat_monster"
            | "movement_blocked_wall"
            | "pickup_drop_cycle"
            | "r3_site_arena"
            | "r3_site_bank"
            | "r3_site_castle_talk"
            | "r3_site_charity"
            | "r3_site_college"
            | "r3_site_merc_guild"
            | "r3_site_order_talk"
            | "r3_site_shop"
            | "r3_site_sorcerors"
            | "r3_site_temple"
            | "p5_city_service_social"
            | "p5_countryside_hunt_travel"
            | "p6_progression_wizard_branch"
            | "p6_quest_social_branch"
    )
}

fn should_rebaseline(fixture: &ReplayFixture) -> bool {
    if fixture.contract_version != 1 {
        return false;
    }
    if fixture.source == "generated_matrix" {
        return false;
    }
    if fixture.source == "legacy_handcrafted" || fixture.source == "scenario_handcrafted" {
        return handcrafted_allowlist(&fixture.name);
    }
    handcrafted_allowlist(&fixture.name)
}

fn main() -> Result<()> {
    let fixture_dir = parse_args()?;
    let fixtures = collect_fixtures(&fixture_dir)?;
    if fixtures.is_empty() {
        bail!("no fixtures found under {}", fixture_dir.display());
    }

    let mut updated = 0usize;
    let mut skipped_generated = 0usize;
    let mut skipped_unlisted = 0usize;
    let mut skipped_contract = 0usize;
    for (path, mut fixture) in fixtures {
        if fixture.contract_version != 1 {
            skipped_contract += 1;
            continue;
        }
        if fixture.source == "generated_matrix" {
            skipped_generated += 1;
            continue;
        }
        if !should_rebaseline(&fixture) {
            skipped_unlisted += 1;
            continue;
        }
        let trace = run_fixture_trace(&fixture);
        let state = trace.final_state;
        let required_event_kinds: Vec<String> =
            trace.seen_event_kinds.into_iter().collect::<BTreeSet<_>>().into_iter().collect();

        fixture.expected.turn = state.clock.turn;
        fixture.expected.minutes = state.clock.minutes;
        fixture.expected.player_position = state.player.position;
        fixture.expected.player_hp = state.player.stats.hp;
        fixture.expected.monsters_alive = state.monsters.len();
        fixture.expected.inventory_count = state.player.inventory.len();
        fixture.expected.ground_item_count = state.ground_items.len();
        fixture.expected.required_event_kinds = required_event_kinds;
        fixture.expected.world_mode = Some(state.world_mode);
        fixture.expected.guild_rank = Some(state.progression.guild_rank);
        fixture.expected.priest_rank = Some(state.progression.priest_rank);
        fixture.expected.alignment = Some(state.progression.alignment);
        fixture.expected.quest_state = Some(state.progression.quest_state);
        fixture.expected.total_winner_unlocked = Some(state.progression.total_winner_unlocked);
        fixture.expected.gold = Some(state.gold);
        fixture.expected.bank_gold = Some(state.bank_gold);
        fixture.expected.food = Some(state.food);
        fixture.expected.known_site_count = Some(state.known_sites.len());
        fixture.expected.ending = Some(state.progression.ending);
        fixture.expected.high_score_eligible = Some(state.progression.high_score_eligible);

        write_fixture(&path, &fixture)?;
        updated += 1;
    }

    println!(
        "replay rebaseline complete: updated={} skipped_generated={} skipped_unlisted={} skipped_contract={}",
        updated, skipped_generated, skipped_unlisted, skipped_contract
    );
    Ok(())
}
