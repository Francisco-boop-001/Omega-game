use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{
    Command, DeterministicRng, TalkDirectionInteraction, active_talk_direction_prompt, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const LEGACY_COMMAND_FILES: [&str; 3] = [
    "archive/legacy-c-runtime/2026-02-06/command1.c",
    "archive/legacy-c-runtime/2026-02-06/command2.c",
    "archive/legacy-c-runtime/2026-02-06/command3.c",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LegacyCommandBinding {
    token: String,
    action: String,
    file: String,
    line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RustCommandBinding {
    token: String,
    pending_talk_direction: String,
    prompt: String,
    site_modal_open: bool,
    wizard_modal_open: bool,
    turn_delta: i64,
    minute_delta: i64,
    pass: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CommandBindingCheck {
    token: String,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CommandBindingReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<CommandBindingCheck>,
    legacy_contract: Vec<LegacyCommandBinding>,
    rust_contract: Vec<RustCommandBinding>,
}

fn extract_action(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let open = trimmed.find('(')?;
    let before = &trimmed[..open];
    let action = before
        .split_whitespace()
        .last()
        .unwrap_or_default()
        .trim_matches(|ch: char| ch == ':' || ch == ';');
    if action.is_empty() || matches!(action, "if" | "for" | "while" | "switch") {
        return None;
    }
    Some(action.to_string())
}

fn extract_legacy_binding(token: char) -> Result<LegacyCommandBinding> {
    let needle = format!("case '{token}':");
    for file in LEGACY_COMMAND_FILES {
        let raw = fs::read_to_string(file).with_context(|| format!("read {file}"))?;
        let lines: Vec<&str> = raw.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if !line.contains(&needle) {
                continue;
            }
            let mut action = extract_action(line);
            if action.is_none() {
                let end = (idx + 6).min(lines.len());
                for follow in lines.iter().take(end).skip(idx + 1) {
                    action = extract_action(follow);
                    if action.is_some() {
                        break;
                    }
                }
            }
            if let Some(action) = action {
                return Ok(LegacyCommandBinding {
                    token: token.to_string(),
                    action,
                    file: file.to_string(),
                    line: idx + 1,
                });
            }
        }
    }
    bail!("missing legacy command binding for token '{token}'")
}

fn talk_direction_name(value: Option<TalkDirectionInteraction>) -> String {
    match value {
        Some(TalkDirectionInteraction::Talk) => "Talk".to_string(),
        Some(TalkDirectionInteraction::Tunnel) => "Tunnel".to_string(),
        None => "None".to_string(),
    }
}

fn run_rust_binding_check(
    token: &str,
    expected: TalkDirectionInteraction,
    expected_prompt_fragment: &str,
    seed: u64,
) -> Result<RustCommandBinding> {
    let (mut state, _) = bootstrap_game_state_from_default_content().context("bootstrap state")?;
    state.options.interactive_sites = true;
    let before_turn = state.clock.turn as i64;
    let before_minute = state.clock.minutes as i64;
    let mut rng = DeterministicRng::seeded(seed);
    let out = step(&mut state, Command::Legacy { token: token.to_string() }, &mut rng);

    let pending = state.pending_talk_direction;
    let pending_name = talk_direction_name(pending);
    let prompt = active_talk_direction_prompt(&state).unwrap_or_default();
    let turn_delta = out.turn as i64 - before_turn;
    let minute_delta = out.minutes as i64 - before_minute;
    let prompt_ok =
        prompt.to_ascii_lowercase().contains(&expected_prompt_fragment.to_ascii_lowercase());
    let pass = pending == Some(expected)
        && prompt_ok
        && state.pending_site_interaction.is_none()
        && state.pending_wizard_interaction.is_none()
        && turn_delta == 0
        && minute_delta == 0;

    let details = format!(
        "pending={pending_name} prompt_ok={} site_modal_open={} wizard_modal_open={} turn_delta={} minute_delta={}",
        prompt_ok,
        state.pending_site_interaction.is_some(),
        state.pending_wizard_interaction.is_some(),
        turn_delta,
        minute_delta
    );

    Ok(RustCommandBinding {
        token: token.to_string(),
        pending_talk_direction: pending_name,
        prompt,
        site_modal_open: state.pending_site_interaction.is_some(),
        wizard_modal_open: state.pending_wizard_interaction.is_some(),
        turn_delta,
        minute_delta,
        pass,
        details,
    })
}

fn markdown(report: &CommandBindingReport) -> String {
    let mut out = Vec::new();
    out.push("# Legacy Command Binding Parity".to_string());
    out.push(String::new());
    out.push(format!("- total: `{}`", report.total));
    out.push(format!("- passed: `{}`", report.passed));
    out.push(format!("- failed: `{}`", report.failed));
    out.push(format!("- status: `{}`", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Token | Status | Details |".to_string());
    out.push("|---|---|---|".to_string());
    for check in &report.checks {
        out.push(format!(
            "| {} | {} | {} |",
            check.token,
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn main() -> Result<()> {
    let legacy_t = extract_legacy_binding('t')?;
    let legacy_tunnel = extract_legacy_binding('T')?;
    let rust_t = run_rust_binding_check("t", TalkDirectionInteraction::Talk, "talk --", 0x71)?;
    let rust_tunnel =
        run_rust_binding_check("T", TalkDirectionInteraction::Tunnel, "tunnel --", 0x72)?;

    let checks = vec![
        CommandBindingCheck {
            token: "t".to_string(),
            passed: legacy_t.action == "talk" && rust_t.pass,
            details: format!(
                "legacy_action={} ({}, line {}); rust={}",
                legacy_t.action, legacy_t.file, legacy_t.line, rust_t.details
            ),
        },
        CommandBindingCheck {
            token: "T".to_string(),
            passed: legacy_tunnel.action == "tunnel" && rust_tunnel.pass,
            details: format!(
                "legacy_action={} ({}, line {}); rust={}",
                legacy_tunnel.action, legacy_tunnel.file, legacy_tunnel.line, rust_tunnel.details
            ),
        },
    ];

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let report = CommandBindingReport {
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
        legacy_contract: vec![legacy_t, legacy_tunnel],
        rust_contract: vec![rust_t, rust_tunnel],
    };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }
    fs::write(
        "target/legacy-command-contract.json",
        serde_json::to_string_pretty(&report.legacy_contract)
            .context("serialize legacy contract")?,
    )
    .context("write target/legacy-command-contract.json")?;
    fs::write(
        "target/rust-command-contract.json",
        serde_json::to_string_pretty(&report.rust_contract).context("serialize rust contract")?,
    )
    .context("write target/rust-command-contract.json")?;
    fs::write(
        "target/legacy-command-binding-parity.json",
        serde_json::to_string_pretty(&report).context("serialize command parity report")?,
    )
    .context("write target/legacy-command-binding-parity.json")?;
    fs::write("target/legacy-command-binding-parity.md", markdown(&report))
        .context("write target/legacy-command-binding-parity.md")?;

    println!(
        "legacy command binding parity: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("legacy command binding parity failed");
    }
    Ok(())
}
