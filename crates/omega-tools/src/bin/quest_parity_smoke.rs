use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{
    Command, DeterministicRng, GameState, LegacyQuestState, SITE_AUX_SERVICE_CASTLE,
    SITE_AUX_SERVICE_MERC_GUILD, SITE_AUX_SERVICE_ORDER, SITE_AUX_SERVICE_PALACE,
    SITE_AUX_SERVICE_TEMPLE, SITE_AUX_SERVICE_THIEVES, SiteInteractionKind, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SmokeStep {
    label: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct QuestSmokeReport {
    pass: bool,
    steps: Vec<SmokeStep>,
    final_state: String,
}

fn run_choice(
    state: &mut omega_core::GameState,
    rng: &mut DeterministicRng,
    kind: SiteInteractionKind,
    choice: &str,
) {
    let site_aux = match kind {
        SiteInteractionKind::MercGuild => SITE_AUX_SERVICE_MERC_GUILD,
        SiteInteractionKind::Temple => SITE_AUX_SERVICE_TEMPLE,
        SiteInteractionKind::ThievesGuild => SITE_AUX_SERVICE_THIEVES,
        SiteInteractionKind::Castle => SITE_AUX_SERVICE_CASTLE,
        SiteInteractionKind::Palace => SITE_AUX_SERVICE_PALACE,
        SiteInteractionKind::Order => SITE_AUX_SERVICE_ORDER,
        _ => 0,
    };
    if site_aux != 0
        && let Some(site) = state.tile_site_at_mut(state.player.position)
    {
        site.aux = site_aux;
    }
    state.pending_site_interaction = Some(kind);
    let _ = step(state, Command::Legacy { token: choice.to_string() }, rng);
}

fn latest_log_line(state: &GameState) -> String {
    state.log.last().cloned().unwrap_or_default()
}

fn markdown(report: &QuestSmokeReport) -> String {
    let mut out = Vec::new();
    out.push("# Quest Parity Smoke".to_string());
    out.push(String::new());
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Step | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for step in &report.steps {
        out.push(format!(
            "| {} | {} | {} |",
            step.label,
            if step.passed { "PASS" } else { "FAIL" },
            step.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.push(format!("- Final state: {}", report.final_state));
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let (mut state, _diagnostics) =
        bootstrap_game_state_from_default_content().context("bootstrap state")?;
    let mut rng = DeterministicRng::seeded(0x51504B45);
    state.options.interactive_sites = true;
    state.gold = 600;
    state.monsters_defeated = 40;
    state.progression.deity_favor = 6;
    state.progression.main_quest.palace_access = true;

    let mut steps = Vec::new();

    run_choice(&mut state, &mut rng, SiteInteractionKind::MercGuild, "2");
    let merc_objective = state.progression.main_quest.objective.clone();
    let merc_line = latest_log_line(&state);
    steps.push(SmokeStep {
        label: "merc_contract".to_string(),
        passed: state.progression.quests.merc.rank >= 1
            && state.progression.quest_state == LegacyQuestState::Active
            && merc_objective.contains("order:")
            && !merc_line.contains("Accepted guild contract."),
        details: format!(
            "merc_rank={} quest={:?} objective={} line={}",
            state.progression.quests.merc.rank,
            state.progression.quest_state,
            merc_objective,
            merc_line
        ),
    });

    run_choice(&mut state, &mut rng, SiteInteractionKind::Temple, "1");
    steps.push(SmokeStep {
        label: "temple_tithe".to_string(),
        passed: state.progression.quests.temple.rank >= 1 && state.progression.priest_rank >= 1,
        details: format!(
            "temple_rank={} priest_rank={} favor={}",
            state.progression.quests.temple.rank,
            state.progression.priest_rank,
            state.progression.deity_favor
        ),
    });

    run_choice(&mut state, &mut rng, SiteInteractionKind::ThievesGuild, "1");
    run_choice(&mut state, &mut rng, SiteInteractionKind::ThievesGuild, "2");
    steps.push(SmokeStep {
        label: "thieves_join_heist".to_string(),
        passed: state.progression.quests.thieves.rank >= 1
            && state.progression.quests.thieves.xp > 0
            && state.progression.main_quest.chaos_path,
        details: format!(
            "thieves_rank={} thieves_xp={} chaos_path={} legal_heat={}",
            state.progression.quests.thieves.rank,
            state.progression.quests.thieves.xp,
            state.progression.main_quest.chaos_path,
            state.legal_heat
        ),
    });

    run_choice(&mut state, &mut rng, SiteInteractionKind::Order, "3");
    let order_line = latest_log_line(&state);
    steps.push(SmokeStep {
        label: "order_talk_no_generic_placeholders".to_string(),
        passed: !order_line.contains("Order audience held")
            && !order_line.contains("dialogue resolved with")
            && !order_line.trim().is_empty(),
        details: order_line,
    });

    run_choice(&mut state, &mut rng, SiteInteractionKind::Castle, "2");
    let castle_line = latest_log_line(&state);
    steps.push(SmokeStep {
        label: "castle_talk_no_generic_placeholders".to_string(),
        passed: !castle_line.contains("Castle audience held")
            && !castle_line.contains("dialogue resolved with")
            && (castle_line.contains("castle")
                || castle_line.contains("Grace")
                || castle_line.contains("castellan"))
            && !castle_line.trim().is_empty(),
        details: castle_line,
    });

    state.progression.quest_state = LegacyQuestState::ArtifactRecovered;
    state.progression.main_quest.stage = LegacyQuestState::ArtifactRecovered;
    run_choice(&mut state, &mut rng, SiteInteractionKind::Palace, "2");
    steps.push(SmokeStep {
        label: "palace_petition_stage1".to_string(),
        passed: state.progression.main_quest.stage == LegacyQuestState::ReturnToPatron
            && state.progression.quest_state == LegacyQuestState::ReturnToPatron,
        details: format!(
            "main_stage={:?} quest={:?}",
            state.progression.main_quest.stage, state.progression.quest_state
        ),
    });

    state.progression.guild_rank = state.progression.guild_rank.max(2);
    state.progression.priest_rank = state.progression.priest_rank.max(1);
    state.progression.quests.merc.rank = state.progression.quests.merc.rank.max(2);
    state.progression.quests.temple.rank = state.progression.quests.temple.rank.max(1);
    run_choice(&mut state, &mut rng, SiteInteractionKind::Palace, "2");
    steps.push(SmokeStep {
        label: "palace_petition_stage2".to_string(),
        passed: state.progression.main_quest.stage == LegacyQuestState::Completed
            && state.progression.quest_state == LegacyQuestState::Completed
            && state.progression.total_winner_unlocked,
        details: format!(
            "main_stage={:?} quest={:?} unlocked={} score={}",
            state.progression.main_quest.stage,
            state.progression.quest_state,
            state.progression.total_winner_unlocked,
            state.progression.score
        ),
    });

    let pass = steps.iter().all(|step| step.passed);
    let report = QuestSmokeReport {
        pass,
        final_state: format!(
            "quest={:?} main_stage={:?} g={} p={} thieves={} score={} gold={}",
            state.progression.quest_state,
            state.progression.main_quest.stage,
            state.progression.guild_rank,
            state.progression.priest_rank,
            state.progression.quests.thieves.rank,
            state.progression.score,
            state.gold
        ),
        steps,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("quest-parity-smoke.json");
    let md_path = target.join("quest-parity-smoke.md");
    fs::write(&json_path, serde_json::to_string_pretty(&report).context("serialize smoke report")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!("quest parity smoke: steps={} status={}", report.steps.len(), report.pass);
    if !report.pass {
        bail!("quest parity smoke failed");
    }
    Ok(())
}
