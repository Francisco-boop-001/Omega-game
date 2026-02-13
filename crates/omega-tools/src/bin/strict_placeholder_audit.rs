use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct AuditCheck {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct AuditReport {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    status: String,
    checks: Vec<AuditCheck>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn read_text(path: &str) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("read {path}"))
}

fn production_source(path: &str) -> Result<String> {
    let source = read_text(path)?;
    let trimmed = source
        .split_once("\n#[cfg(test)]")
        .map_or(source.as_str(), |(prefix, _)| prefix)
        .to_string();
    Ok(trimmed)
}

fn check_absent(path: &str, id: &str, forbidden: &[&str]) -> Result<AuditCheck> {
    let source = read_text(path)?;
    let hits: Vec<&str> =
        forbidden.iter().copied().filter(|pattern| source.contains(pattern)).collect();
    Ok(AuditCheck {
        id: id.to_string(),
        pass: hits.is_empty(),
        details: if hits.is_empty() {
            format!("{path}: no forbidden signature detected")
        } else {
            format!("{path}: forbidden signatures found: {}", hits.join(", "))
        },
    })
}

fn check_bootstrap_fallback_absent() -> Result<AuditCheck> {
    let files = [
        "crates/omega-tui/src/lib.rs",
        "crates/omega-bevy/src/lib.rs",
        "crates/omega-tui/src/bin/omega-tui-app.rs",
        "crates/omega-bevy/src/bin/omega-bevy-app.rs",
    ];
    let forbidden = [
        "bootstrap_game_state_from_default_content().unwrap_or_default()",
        "bootstrap_game_state_from_default_content().unwrap_or_else(|_| GameState::default())",
        "bootstrap_game_state_from_default_content()\n        .unwrap_or_else(|_| GameState::default())",
        "bootstrap_game_state_from_default_content()\n        .unwrap_or_else(|err| GameState::default())",
        "bootstrap_game_state_from_default_content() {\n        Ok(",
    ];
    let mut offenders = Vec::new();
    for path in files {
        let source = production_source(path)?;
        let direct_match = forbidden.iter().any(|pattern| source.contains(pattern));
        let match_style_fallback = source
            .contains("match bootstrap_game_state_from_default_content")
            && source.contains("GameState::default()");
        if direct_match || match_style_fallback {
            offenders.push(path.to_string());
        }
    }
    Ok(AuditCheck {
        id: "bootstrap_fallback_to_default_absent".to_string(),
        pass: offenders.is_empty(),
        details: if offenders.is_empty() {
            "no bootstrap fallback-to-default signatures found in playable frontends".to_string()
        } else {
            format!("fallback signatures present in: {}", offenders.join(", "))
        },
    })
}

fn main() -> Result<()> {
    let checks = vec![
        check_absent(
            "crates/omega-content/src/lib.rs",
            "city_placeholder_assignment_removed",
            &["city_placeholder_assignment"],
        )?,
        check_bootstrap_fallback_absent()?,
        check_absent(
            "crates/omega-core/src/lib.rs",
            "no_generic_noop_legacy_resolution",
            &[
                "legacy command resolved with no additional world effect",
                "frontend handles this command",
            ],
        )?,
        check_absent(
            "crates/omega-tools/src/bin/true_parity_refresh.rs",
            "true_refresh_not_proxy_fed",
            &[
                "target/classic-",
                "classic-command-parity-matrix.json",
                "classic-core-model-parity.json",
                "classic-magic-item-parity.json",
                "classic-combat-encounter-parity.json",
                "classic-site-service-parity-matrix.json",
                "classic-progression-branch-matrix.json",
                "classic-frontend-workflow-parity.json",
                "frontend-command-parity.json",
                "classic-parity-regression-dashboard.json",
                "classic-burnin-window.json",
            ],
        )?,
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = AuditReport {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        status: if failed == 0 { "PASS".to_string() } else { "FAIL".to_string() },
        checks,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("strict-placeholder-audit.json");
    let md_path = target.join("strict-placeholder-audit.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize strict audit report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;

    let mut md = Vec::new();
    md.push("# Strict Placeholder Audit".to_string());
    md.push(String::new());
    md.push(format!("- Status: {}", report.status));
    md.push(format!("- Passed: {}/{}", report.passed, report.total));
    md.push(String::new());
    md.push("| Check | Status | Details |".to_string());
    md.push("|---|---|---|".to_string());
    for check in &report.checks {
        md.push(format!(
            "| {} | {} | {} |",
            check.id,
            if check.pass { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    md.push(String::new());
    fs::write(&md_path, md.join("\n")).with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "strict placeholder audit: status={}, passed={}/{}",
        report.status, report.passed, report.total
    );
    if failed > 0 {
        bail!("strict placeholder audit failed");
    }
    Ok(())
}
