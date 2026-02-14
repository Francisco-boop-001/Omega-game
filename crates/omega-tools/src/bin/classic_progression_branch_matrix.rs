use anyhow::{Context, Result, bail};
use omega_core::{
    Command, DeterministicRng, Direction, GameState, MapBounds, Position, Stats, step,
};
use omega_save::{decode_state_json, encode_json};
use omega_tools::replay::{load_fixture, run_fixture, run_fixture_trace};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ProgressionCheck {
    id: String,
    label: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ProgressionBranchMatrix {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<ProgressionCheck>,
}

fn markdown(matrix: &ProgressionBranchMatrix) -> String {
    let mut out = Vec::new();
    out.push("# Classic Progression Branch Matrix".to_string());
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

fn persistence_probe() -> Result<(bool, String)> {
    let mut state = GameState::new(MapBounds { width: 9, height: 9 });
    state.player.position = Position { x: 4, y: 4 };
    state.player.stats = Stats { hp: 24, max_hp: 24, attack_min: 7, attack_max: 7, defense: 1, weight: 60 };
    state.spawn_monster(
        "probe-rat-a",
        Position { x: 5, y: 4 },
        Stats { hp: 4, max_hp: 4, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
    );
    state.spawn_monster(
        "probe-rat-b",
        Position { x: 3, y: 4 },
        Stats { hp: 4, max_hp: 4, attack_min: 1, attack_max: 1, defense: 0, weight: 60 },
    );
    let mut rng = DeterministicRng::seeded(0x6006);

    let script = [
        Command::Legacy { token: "t".to_string() },
        Command::Legacy { token: "^g".to_string() },
        Command::Legacy { token: "y".to_string() },
        Command::Legacy { token: "^x".to_string() },
        Command::Legacy { token: "wealth".to_string() },
        Command::Legacy { token: "<enter>".to_string() },
        Command::Legacy { token: "A".to_string() },
        Command::Legacy { token: "G".to_string() },
        Command::Attack(Direction::East),
    ];
    for command in script {
        let _ = step(&mut state, command, &mut rng);
    }

    let raw = encode_json(&state).context("encode mid-branch save")?;
    let mut loaded = decode_state_json(&raw).context("decode mid-branch save")?;
    let mut rng_loaded = DeterministicRng::seeded(0x6006);
    let _ = step(&mut loaded, Command::Attack(Direction::West), &mut rng_loaded);

    let pass = loaded.progression.quest_state == omega_core::LegacyQuestState::Completed
        && loaded.progression.ending != omega_core::EndingKind::None;
    let details = format!(
        "quest={:?} ending={:?} score={} eligible={}",
        loaded.progression.quest_state,
        loaded.progression.ending,
        loaded.progression.score,
        loaded.progression.high_score_eligible
    );
    Ok((pass, details))
}

fn main() -> Result<()> {
    let wizard_fixture = "crates/omega-tools/fixtures/replay/p6_progression_wizard_branch.json";
    let social_fixture = "crates/omega-tools/fixtures/replay/p6_quest_social_branch.json";

    let wizard = load_fixture(Path::new(wizard_fixture))
        .with_context(|| format!("load {wizard_fixture}"))?;
    let social = load_fixture(Path::new(social_fixture))
        .with_context(|| format!("load {social_fixture}"))?;
    let wizard_result = run_fixture(&wizard);
    let social_result = run_fixture(&social);
    let wizard_trace = run_fixture_trace(&wizard);
    let social_trace = run_fixture_trace(&social);
    let wizard_state = wizard_trace.final_state;
    let social_state = social_trace.final_state;

    let (persistence_pass, persistence_details) = persistence_probe()?;

    let checks = vec![
        ProgressionCheck {
            id: "guild_rank_gates".to_string(),
            label: "Guild rank gates/unlocks".to_string(),
            passed: social_state.progression.guild_rank >= 1,
            details: format!(
                "fixture_pass={} guild_rank={}",
                social_result.passed, social_state.progression.guild_rank
            ),
        },
        ProgressionCheck {
            id: "deity_alignment".to_string(),
            label: "Deity/priest/alignment consequences".to_string(),
            passed: wizard_state.progression.priest_rank >= 1
                && wizard_state.progression.alignment == omega_core::Alignment::Lawful,
            details: format!(
                "fixture_pass={} priest_rank={} alignment={:?}",
                wizard_result.passed,
                wizard_state.progression.priest_rank,
                wizard_state.progression.alignment
            ),
        },
        ProgressionCheck {
            id: "quest_graph".to_string(),
            label: "Quest graph state progression".to_string(),
            passed: social_state.progression.quest_state == omega_core::LegacyQuestState::Completed
                && wizard_state.progression.quest_steps_completed >= 3,
            details: format!(
                "social_quest={:?} wizard_steps={}",
                social_state.progression.quest_state,
                wizard_state.progression.quest_steps_completed
            ),
        },
        ProgressionCheck {
            id: "ending_scoring".to_string(),
            label: "Ending and score eligibility branches".to_string(),
            passed: wizard_state.progression.ending == omega_core::EndingKind::Victory
                && !wizard_state.progression.high_score_eligible
                && social_state.progression.high_score_eligible,
            details: format!(
                "wizard=({:?}, eligible={}) social=({:?}, eligible={})",
                wizard_state.progression.ending,
                wizard_state.progression.high_score_eligible,
                social_state.progression.ending,
                social_state.progression.high_score_eligible
            ),
        },
        ProgressionCheck {
            id: "save_load_mid_branch".to_string(),
            label: "Save/load mid-branch persistence".to_string(),
            passed: persistence_pass,
            details: persistence_details,
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let pass = failed == 0;
    let matrix = ProgressionBranchMatrix { total, passed, failed, pass, checks };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("classic-progression-branch-matrix.json");
    let md_path = target.join("classic-progression-branch-matrix.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&matrix).context("serialize progression matrix")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&matrix))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "classic progression branch parity: total={}, passed={}, failed={}",
        matrix.total, matrix.passed, matrix.failed
    );
    if !matrix.pass {
        bail!("classic progression branch matrix failed");
    }
    Ok(())
}
