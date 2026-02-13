use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct StepResult {
    name: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct JourneyRun {
    frontend: String,
    status: String,
    simulated_game_over: bool,
    started_turn: u64,
    saved_turn: u64,
    loaded_turn: u64,
    restart_turn: u64,
    steps: Vec<StepResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct E2EJourneyReport {
    schema_version: u32,
    generated_at_utc: String,
    overall_status: String,
    total_runs: usize,
    passed_runs: usize,
    failed_runs: usize,
    pending_frontends: Vec<String>,
    runs: Vec<JourneyRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct MetricParity {
    name: String,
    tui_value: String,
    bevy_value: String,
    matched: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LifecycleStepParity {
    step: String,
    tui_passed: bool,
    bevy_passed: bool,
    matched: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LifecycleParityReport {
    schema_version: u32,
    source_report: String,
    overall_status: String,
    frontends: Vec<String>,
    metrics: Vec<MetricParity>,
    shared_step_parity: Vec<LifecycleStepParity>,
    notes: Vec<String>,
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn load_e2e_report(path: &PathBuf) -> Result<E2EJourneyReport> {
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str::<E2EJourneyReport>(&raw)
        .with_context(|| format!("parse {}", path.display()))
}

fn step_map(run: &JourneyRun) -> HashMap<String, bool> {
    let mut map = HashMap::new();
    for step in &run.steps {
        map.insert(step.name.clone(), step.passed);
    }
    map
}

fn markdown(report: &LifecycleParityReport) -> String {
    let mut out = Vec::new();
    out.push("# M5 Lifecycle Parity Report".to_string());
    out.push(String::new());
    out.push(format!("- Overall status: {}", report.overall_status));
    out.push(format!("- Source report: {}", report.source_report));
    out.push(format!("- Frontends: {}", report.frontends.join(", ")));
    out.push(String::new());
    out.push("## Metrics".to_string());
    out.push(String::new());
    out.push("| Metric | TUI | Bevy | Match |".to_string());
    out.push("|---|---|---|---|".to_string());
    for metric in &report.metrics {
        out.push(format!(
            "| {} | {} | {} | {} |",
            metric.name,
            metric.tui_value,
            metric.bevy_value,
            if metric.matched { "yes" } else { "no" }
        ));
    }
    out.push(String::new());
    out.push("## Shared Step Parity".to_string());
    out.push(String::new());
    out.push("| Step | TUI | Bevy | Match |".to_string());
    out.push("|---|---|---|---|".to_string());
    for step in &report.shared_step_parity {
        out.push(format!(
            "| {} | {} | {} | {} |",
            step.step,
            if step.tui_passed { "PASS" } else { "FAIL" },
            if step.bevy_passed { "PASS" } else { "FAIL" },
            if step.matched { "yes" } else { "no" }
        ));
    }
    out.push(String::new());
    if !report.notes.is_empty() {
        out.push("## Notes".to_string());
        out.push(String::new());
        for note in &report.notes {
            out.push(format!("- {note}"));
        }
    }
    out.join("\n")
}

fn main() -> Result<()> {
    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }

    let source_path = target.join("m5-e2e-journey-report.json");
    let e2e = load_e2e_report(&source_path)?;
    if e2e.overall_status != "PASS" {
        bail!("lifecycle parity requires passing e2e source report");
    }

    let tui = e2e
        .runs
        .iter()
        .find(|run| run.frontend == "tui")
        .ok_or_else(|| anyhow!("missing tui run in e2e report"))?;
    let bevy = e2e
        .runs
        .iter()
        .find(|run| run.frontend == "bevy")
        .ok_or_else(|| anyhow!("missing bevy run in e2e report"))?;

    let mut metrics = vec![
        MetricParity {
            name: "startup_turn".to_string(),
            tui_value: tui.started_turn.to_string(),
            bevy_value: bevy.started_turn.to_string(),
            matched: tui.started_turn == bevy.started_turn,
        },
        MetricParity {
            name: "saved_turn".to_string(),
            tui_value: tui.saved_turn.to_string(),
            bevy_value: bevy.saved_turn.to_string(),
            matched: tui.saved_turn == bevy.saved_turn,
        },
        MetricParity {
            name: "loaded_turn".to_string(),
            tui_value: tui.loaded_turn.to_string(),
            bevy_value: bevy.loaded_turn.to_string(),
            matched: tui.loaded_turn == bevy.loaded_turn,
        },
        MetricParity {
            name: "restart_turn".to_string(),
            tui_value: tui.restart_turn.to_string(),
            bevy_value: bevy.restart_turn.to_string(),
            matched: tui.restart_turn == bevy.restart_turn,
        },
        MetricParity {
            name: "save_load_consistency".to_string(),
            tui_value: (tui.saved_turn == tui.loaded_turn).to_string(),
            bevy_value: (bevy.saved_turn == bevy.loaded_turn).to_string(),
            matched: (tui.saved_turn == tui.loaded_turn) == (bevy.saved_turn == bevy.loaded_turn)
                && tui.saved_turn == tui.loaded_turn
                && bevy.saved_turn == bevy.loaded_turn,
        },
        MetricParity {
            name: "frontend_run_status".to_string(),
            tui_value: tui.status.clone(),
            bevy_value: bevy.status.clone(),
            matched: tui.status == "PASS" && bevy.status == "PASS",
        },
    ];

    let tui_steps = step_map(tui);
    let bevy_steps = step_map(bevy);
    let lifecycle_steps = ["new_game", "save", "load", "game_over", "restart"];
    let mut shared_step_parity = Vec::new();
    for step in lifecycle_steps {
        let tui_passed = *tui_steps.get(step).unwrap_or(&false);
        let bevy_passed = *bevy_steps.get(step).unwrap_or(&false);
        shared_step_parity.push(LifecycleStepParity {
            step: step.to_string(),
            tui_passed,
            bevy_passed,
            matched: tui_passed == bevy_passed && tui_passed && bevy_passed,
        });
    }

    let has_bevy_start_session = bevy_steps.contains_key("start_session");
    metrics.push(MetricParity {
        name: "bevy_start_session_present".to_string(),
        tui_value: "n/a".to_string(),
        bevy_value: has_bevy_start_session.to_string(),
        matched: has_bevy_start_session,
    });
    metrics.push(MetricParity {
        name: "natural_game_over".to_string(),
        tui_value: (!tui.simulated_game_over).to_string(),
        bevy_value: (!bevy.simulated_game_over).to_string(),
        matched: !tui.simulated_game_over && !bevy.simulated_game_over,
    });

    let mut overall_status = "PASS".to_string();
    if metrics.iter().any(|m| !m.matched) || shared_step_parity.iter().any(|s| !s.matched) {
        overall_status = "FAIL".to_string();
    }

    let game_over_note = if tui.simulated_game_over || bevy.simulated_game_over {
        "Game-over is still simulated in at least one frontend."
    } else {
        "Game-over path is natural in both frontends."
    };

    let report = LifecycleParityReport {
        schema_version: 1,
        source_report: "target/m5-e2e-journey-report.json".to_string(),
        overall_status: overall_status.clone(),
        frontends: vec!["tui".to_string(), "bevy".to_string()],
        metrics,
        shared_step_parity,
        notes: vec![
            "Startup/save/load/restart parity is computed from the latest E2E journey report."
                .to_string(),
            game_over_note.to_string(),
        ],
    };

    let json_path = target.join("m5-lifecycle-parity-report.json");
    let md_path = target.join("m5-lifecycle-parity-report.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize lifecycle parity report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "m5 lifecycle parity: metrics={}, steps={}, status={}",
        report.metrics.len(),
        report.shared_step_parity.len(),
        report.overall_status
    );

    if report.overall_status != "PASS" {
        bail!("lifecycle parity mismatches detected");
    }
    Ok(())
}
