use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DeviationItem {
    id: String,
    track: String,
    severity: String,
    status: String,
    title: String,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DeviationSummary {
    total: usize,
    open: usize,
    closed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DeviationLedger {
    generated_at_utc: String,
    summary: DeviationSummary,
    items: Vec<DeviationItem>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn markdown(ledger: &DeviationLedger) -> String {
    let mut out = Vec::new();
    out.push("# True Parity Deviation Ledger".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", ledger.generated_at_utc));
    out.push(format!("- Total: {}", ledger.summary.total));
    out.push(format!("- Open: {}", ledger.summary.open));
    out.push(format!("- Closed: {}", ledger.summary.closed));
    out.push(String::new());
    out.push("| ID | Track | Severity | Status | Title |".to_string());
    out.push("|---|---|---|---|---|".to_string());
    for item in &ledger.items {
        out.push(format!(
            "| {} | {} | {} | {} | {} |",
            item.id, item.track, item.severity, item.status, item.title
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn read_json(path: &str) -> Option<Value> {
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str::<Value>(&raw).ok()
}

fn artifact_pass(path: &str) -> bool {
    let Some(value) = read_json(path) else {
        return false;
    };
    if let Some(pass) = value.get("pass").and_then(|v| v.as_bool()) {
        return pass;
    }
    if let Some(status) = value.get("status").and_then(|v| v.as_str()) {
        return status == "PASS";
    }
    false
}

fn deviation(track: &str, severity: &str, title: &str, details: &str, pass: bool) -> DeviationItem {
    DeviationItem {
        id: format!("D-{track}-001"),
        track: track.to_string(),
        severity: severity.to_string(),
        status: if pass { "CLOSED" } else { "OPEN" }.to_string(),
        title: title.to_string(),
        details: details.to_string(),
    }
}

fn main() -> Result<()> {
    let items = vec![
        deviation(
            "T2",
            "P0",
            "Environment and transition parity",
            "Requires true environment transition matrix PASS.",
            artifact_pass("target/true-environment-transition-matrix.json"),
        ),
        deviation(
            "T3",
            "P0",
            "Command behavior parity",
            "Requires true command behavior matrix missing/partial/key_conflict all zero.",
            artifact_pass("target/true-command-behavior-matrix.json"),
        ),
        deviation(
            "T4",
            "P0",
            "Spell parity matrix (42)",
            "Requires true spell parity matrix PASS with denominator check.",
            artifact_pass("target/true-spell-parity-matrix.json"),
        ),
        deviation(
            "T5",
            "P0",
            "Item and inventory parity matrix",
            "Requires true item parity matrix PASS.",
            artifact_pass("target/true-item-parity-matrix.json"),
        ),
        deviation(
            "T6",
            "P0",
            "Combat/monster/trap parity matrix",
            "Requires true combat encounter matrix PASS.",
            artifact_pass("target/true-combat-encounter-matrix.json"),
        ),
        deviation(
            "T7",
            "P0",
            "City/site/economy/social parity matrix",
            "Requires true site economy social matrix PASS.",
            artifact_pass("target/true-site-economy-social-matrix.json"),
        ),
        deviation(
            "T8",
            "P1",
            "Progression/ending parity matrix",
            "Requires true progression ending matrix PASS.",
            artifact_pass("target/true-progression-ending-matrix.json"),
        ),
        deviation(
            "T9",
            "P1",
            "Compatibility matrix",
            "Requires true compatibility matrix PASS.",
            artifact_pass("target/true-compatibility-matrix.json"),
        ),
        deviation(
            "T10",
            "P1",
            "Frontend workflow parity",
            "Requires true frontend workflow matrix PASS.",
            artifact_pass("target/true-frontend-workflow-matrix.json"),
        ),
        deviation(
            "T11",
            "P1",
            "Verification hardening",
            "Requires true regression dashboard PASS and true burn-in window PASS.",
            artifact_pass("target/true-parity-regression-dashboard.json")
                && artifact_pass("target/true-burnin-window.json"),
        ),
        deviation(
            "T12",
            "P1",
            "Closure baseline freeze",
            "Requires true parity baseline freeze PASS.",
            artifact_pass("target/true-parity-baseline-freeze.json"),
        ),
    ];

    let open = items.iter().filter(|item| item.status == "OPEN").count();
    let total = items.len();
    let closed = total.saturating_sub(open);
    let ledger = DeviationLedger {
        generated_at_utc: now_utc_unix(),
        summary: DeviationSummary { total, open, closed },
        items,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("true-parity-deviations.json");
    let md_path = target.join("true-parity-deviations.md");
    fs::write(&json_path, serde_json::to_string_pretty(&ledger).context("serialize deviations")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&ledger))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "true parity deviations: total={}, open={}, closed={}",
        ledger.summary.total, ledger.summary.open, ledger.summary.closed
    );
    Ok(())
}
