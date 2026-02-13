use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
struct CommandParityMatrix {
    missing: usize,
    partial: usize,
    key_conflict: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct GapLedgerSummary {
    total_open: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct GapLedger {
    summary: GapLedgerSummary,
}

#[derive(Debug, Clone, Deserialize)]
struct RegressionDashboard {
    total: usize,
    passed: usize,
    failed: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct DeterminismReport {
    required_runs_per_fixture: usize,
    total_runs: usize,
    divergent_runs: usize,
    passed: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct FrontendParityReport {
    total_cases: usize,
    mismatched_cases: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct SaveCompatReport {
    total: usize,
    failed: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct SiteServiceParityMatrix {
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ProgressionBranchMatrix {
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct StateIntegrityReport {
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct CoreModelParityReport {
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct CombatEncounterParityReport {
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct MagicItemParityReport {
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct CompatibilityMatrixReport {
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct FrontendWorkflowParityReport {
    failed: usize,
    pass: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct BaselineFreeze {
    status: String,
    missing: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ClassicParityRegressionDashboard {
    generated_at_utc: String,
    replay_total: usize,
    replay_passed: usize,
    replay_failed: usize,
    determinism_runs_per_fixture: usize,
    determinism_total_runs: usize,
    determinism_divergent_runs: usize,
    frontend_total_cases: usize,
    frontend_mismatches: usize,
    save_total_cases: usize,
    save_failed_cases: usize,
    site_service_failed: usize,
    progression_failed: usize,
    integrity_failed: usize,
    core_model_failed: usize,
    combat_encounter_failed: usize,
    magic_item_failed: usize,
    compatibility_failed: usize,
    frontend_workflow_failed: usize,
    baseline_freeze_missing: usize,
    command_missing: usize,
    command_partial: usize,
    command_key_conflict: usize,
    open_gap_items: usize,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ClassicBurninWindow {
    generated_at_utc: String,
    required_runs_per_fixture: usize,
    determinism_total_runs: usize,
    replay_total_scenarios: usize,
    replay_failed_scenarios: usize,
    determinism_divergent_runs: usize,
    pass: bool,
    blockers: Vec<String>,
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str::<T>(&raw).with_context(|| format!("decode {}", path.display()))
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn markdown_dashboard(d: &ClassicParityRegressionDashboard) -> String {
    let mut out = Vec::new();
    out.push("# Classic Parity Regression Dashboard".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", d.generated_at_utc));
    out.push(format!("- Status: {}", d.status));
    out.push(format!(
        "- Replay: total={}, passed={}, failed={}",
        d.replay_total, d.replay_passed, d.replay_failed
    ));
    out.push(format!(
        "- Determinism: runs/fixture={}, total_runs={}, divergent={}",
        d.determinism_runs_per_fixture, d.determinism_total_runs, d.determinism_divergent_runs
    ));
    out.push(format!(
        "- Frontend parity: total_cases={}, mismatches={}",
        d.frontend_total_cases, d.frontend_mismatches
    ));
    out.push(format!(
        "- Save compatibility: total_cases={}, failed={}",
        d.save_total_cases, d.save_failed_cases
    ));
    out.push(format!("- Site/service matrix failed checks: {}", d.site_service_failed));
    out.push(format!("- Progression branch matrix failed checks: {}", d.progression_failed));
    out.push(format!("- State integrity failed checks: {}", d.integrity_failed));
    out.push(format!("- Core model parity failed checks: {}", d.core_model_failed));
    out.push(format!("- Combat/encounter parity failed checks: {}", d.combat_encounter_failed));
    out.push(format!("- Magic/item parity failed checks: {}", d.magic_item_failed));
    out.push(format!("- Compatibility matrix failed checks: {}", d.compatibility_failed));
    out.push(format!("- Frontend workflow parity failed checks: {}", d.frontend_workflow_failed));
    out.push(format!("- Baseline freeze missing artifacts: {}", d.baseline_freeze_missing));
    out.push(format!(
        "- Command parity: missing={}, partial={}, key_conflict={}",
        d.command_missing, d.command_partial, d.command_key_conflict
    ));
    out.push(format!("- Open gap items: {}", d.open_gap_items));
    out.push(String::new());
    out.join("\n")
}

fn markdown_burnin(b: &ClassicBurninWindow) -> String {
    let mut out = Vec::new();
    out.push("# Classic Parity Burn-In Window".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", b.generated_at_utc));
    out.push(format!("- Required runs per fixture: {}", b.required_runs_per_fixture));
    out.push(format!("- Determinism total runs: {}", b.determinism_total_runs));
    out.push(format!("- Replay total scenarios: {}", b.replay_total_scenarios));
    out.push(format!("- Replay failed scenarios: {}", b.replay_failed_scenarios));
    out.push(format!("- Determinism divergent runs: {}", b.determinism_divergent_runs));
    out.push(format!("- Status: {}", if b.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    if b.blockers.is_empty() {
        out.push("- Blockers: none".to_string());
    } else {
        out.push("## Blockers".to_string());
        for blocker in &b.blockers {
            out.push(format!("- {blocker}"));
        }
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }

    let command: CommandParityMatrix =
        read_json(&target.join("classic-command-parity-matrix.json"))?;
    let gaps: GapLedger = read_json(&target.join("classic-gap-ledger.json"))?;
    let replay: RegressionDashboard = read_json(&target.join("ws-d-regression-dashboard.json"))?;
    let determinism: DeterminismReport = read_json(&target.join("ws-d-determinism-report.json"))?;
    let frontend: FrontendParityReport = read_json(&target.join("frontend-command-parity.json"))?;
    let save: SaveCompatReport = read_json(&target.join("save-compat-report.json"))?;
    let site_service: SiteServiceParityMatrix =
        read_json(&target.join("classic-site-service-parity-matrix.json"))?;
    let progression: ProgressionBranchMatrix =
        read_json(&target.join("classic-progression-branch-matrix.json"))?;
    let integrity: StateIntegrityReport = read_json(&target.join("classic-state-integrity.json"))?;
    let core_model: CoreModelParityReport =
        read_json(&target.join("classic-core-model-parity.json"))?;
    let combat: CombatEncounterParityReport =
        read_json(&target.join("classic-combat-encounter-parity.json"))?;
    let magic_items: MagicItemParityReport =
        read_json(&target.join("classic-magic-item-parity.json"))?;
    let compatibility: CompatibilityMatrixReport =
        read_json(&target.join("classic-compatibility-matrix.json"))?;
    let workflow: FrontendWorkflowParityReport =
        read_json(&target.join("classic-frontend-workflow-parity.json"))?;
    let freeze: BaselineFreeze = read_json(&target.join("classic-parity-baseline-freeze.json"))?;

    let status = if replay.failed == 0
        && determinism.passed
        && frontend.mismatched_cases == 0
        && save.failed == 0
        && site_service.pass
        && progression.pass
        && integrity.pass
        && core_model.pass
        && combat.pass
        && magic_items.pass
        && compatibility.pass
        && workflow.pass
        && freeze.status == "PASS"
        && command.missing == 0
        && command.partial == 0
        && command.key_conflict == 0
        && gaps.summary.total_open == 0
    {
        "PASS"
    } else {
        "FAIL"
    };

    let dashboard = ClassicParityRegressionDashboard {
        generated_at_utc: now_utc_unix(),
        replay_total: replay.total,
        replay_passed: replay.passed,
        replay_failed: replay.failed,
        determinism_runs_per_fixture: determinism.required_runs_per_fixture,
        determinism_total_runs: determinism.total_runs,
        determinism_divergent_runs: determinism.divergent_runs,
        frontend_total_cases: frontend.total_cases,
        frontend_mismatches: frontend.mismatched_cases,
        save_total_cases: save.total,
        save_failed_cases: save.failed,
        site_service_failed: site_service.failed,
        progression_failed: progression.failed,
        integrity_failed: integrity.failed,
        core_model_failed: core_model.failed,
        combat_encounter_failed: combat.failed,
        magic_item_failed: magic_items.failed,
        compatibility_failed: compatibility.failed,
        frontend_workflow_failed: workflow.failed,
        baseline_freeze_missing: freeze.missing,
        command_missing: command.missing,
        command_partial: command.partial,
        command_key_conflict: command.key_conflict,
        open_gap_items: gaps.summary.total_open,
        status: status.to_string(),
    };

    let mut blockers = Vec::new();
    if replay.failed > 0 {
        blockers.push(format!("replay failures present ({})", replay.failed));
    }
    if determinism.divergent_runs > 0 {
        blockers
            .push(format!("determinism divergent runs present ({})", determinism.divergent_runs));
    }
    if command.missing > 0 || command.partial > 0 || command.key_conflict > 0 {
        blockers.push(format!(
            "command parity not clean (missing={}, partial={}, key_conflict={})",
            command.missing, command.partial, command.key_conflict
        ));
    }
    if site_service.failed > 0 {
        blockers.push(format!("site/service parity failures present ({})", site_service.failed));
    }
    if progression.failed > 0 {
        blockers
            .push(format!("progression branch parity failures present ({})", progression.failed));
    }
    if integrity.failed > 0 {
        blockers.push(format!("state integrity failures present ({})", integrity.failed));
    }
    if core_model.failed > 0 {
        blockers.push(format!("core model parity failures present ({})", core_model.failed));
    }
    if combat.failed > 0 {
        blockers.push(format!("combat/encounter parity failures present ({})", combat.failed));
    }
    if magic_items.failed > 0 {
        blockers.push(format!("magic/item parity failures present ({})", magic_items.failed));
    }
    if compatibility.failed > 0 {
        blockers.push(format!("compatibility matrix failures present ({})", compatibility.failed));
    }
    if workflow.failed > 0 {
        blockers.push(format!("frontend workflow parity failures present ({})", workflow.failed));
    }
    if freeze.missing > 0 {
        blockers.push(format!("baseline freeze missing artifacts ({})", freeze.missing));
    }
    if gaps.summary.total_open > 0 {
        blockers.push(format!("open gap ledger items remain ({})", gaps.summary.total_open));
    }

    let burnin = ClassicBurninWindow {
        generated_at_utc: now_utc_unix(),
        required_runs_per_fixture: determinism.required_runs_per_fixture,
        determinism_total_runs: determinism.total_runs,
        replay_total_scenarios: replay.total,
        replay_failed_scenarios: replay.failed,
        determinism_divergent_runs: determinism.divergent_runs,
        pass: blockers.is_empty(),
        blockers,
    };

    fs::write(
        target.join("classic-parity-regression-dashboard.json"),
        serde_json::to_string_pretty(&dashboard).context("serialize dashboard")?,
    )
    .context("write classic-parity-regression-dashboard.json")?;
    fs::write(
        target.join("classic-parity-regression-dashboard.md"),
        markdown_dashboard(&dashboard),
    )
    .context("write classic-parity-regression-dashboard.md")?;

    fs::write(
        target.join("classic-burnin-window.json"),
        serde_json::to_string_pretty(&burnin).context("serialize burnin")?,
    )
    .context("write classic-burnin-window.json")?;
    fs::write(target.join("classic-burnin-window.md"), markdown_burnin(&burnin))
        .context("write classic-burnin-window.md")?;

    println!(
        "classic parity reports: status={}, replay_failed={}, command_missing={}, open_gaps={}",
        dashboard.status,
        dashboard.replay_failed,
        dashboard.command_missing,
        dashboard.open_gap_items
    );

    Ok(())
}
