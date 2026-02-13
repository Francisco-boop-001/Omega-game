use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct ChecklistItem {
    label: &'static str,
    required_path: &'static str,
}

fn main() -> Result<()> {
    let items = vec![
        ChecklistItem {
            label: "TUI launcher binary source exists",
            required_path: "crates/omega-tui/src/bin/omega-tui-app.rs",
        },
        ChecklistItem {
            label: "Bevy launcher binary source exists",
            required_path: "crates/omega-bevy/src/bin/omega-bevy-app.rs",
        },
        ChecklistItem {
            label: "Boot reliability report exists",
            required_path: "target/m5-boot-reliability.json",
        },
        ChecklistItem {
            label: "Perf budget report exists",
            required_path: "target/m5-perf-budget-report.json",
        },
        ChecklistItem {
            label: "Frame-time report exists",
            required_path: "target/m5-frame-time-report.json",
        },
        ChecklistItem {
            label: "Security audit report exists",
            required_path: "target/m5-security-audit.json",
        },
        ChecklistItem {
            label: "Weekly fuzz report exists",
            required_path: "target/m5-fuzz-weekly-report.md",
        },
    ];

    let mut missing = Vec::new();
    let mut lines = vec![
        "# M5 Release Operations Checklist".to_string(),
        String::new(),
        "| Item | Status | Evidence |".to_string(),
        "|---|---|---|".to_string(),
    ];
    for item in &items {
        let exists = Path::new(item.required_path).exists();
        if !exists {
            missing.push(item.required_path.to_string());
        }
        lines.push(format!(
            "| {} | {} | {} |",
            item.label,
            if exists { "PASS" } else { "FAIL" },
            item.required_path
        ));
    }
    lines.push(String::new());
    lines.push("## RC Drill Commands".to_string());
    lines.push(String::new());
    lines.push("1. `cargo run -p omega-tui --bin omega-tui-app`".to_string());
    lines.push("2. `cargo run -p omega-bevy --bin omega-bevy-app`".to_string());
    lines.push(
        "3. In-session smoke: start -> save (`P`) -> load (`L`) -> restart (`R`)".to_string(),
    );
    lines.push("4. Confirm gate: `powershell -ExecutionPolicy Bypass -File ./scripts/run-m5-gate.ps1 -StrictArtifactMode`".to_string());

    let out_path = Path::new("target").join("m5-release-operations-checklist.md");
    if let Some(parent) = out_path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent).context("create target directory")?;
    }
    fs::write(&out_path, lines.join("\n"))
        .with_context(|| format!("write {}", out_path.display()))?;

    println!("m5 release operations checklist: items={}, missing={}", items.len(), missing.len());
    if !missing.is_empty() {
        bail!("release operations checklist has missing evidence: {}", missing.join(", "));
    }
    Ok(())
}
