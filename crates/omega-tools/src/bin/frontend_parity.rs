use anyhow::{Context, Result, bail};
use omega_bevy::{BevyKey, InputAction, map_shared_gameplay_key};
use omega_core::Command;
use omega_tui::{App, UiAction, UiKey};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ParityCaseResult {
    key: String,
    tui: String,
    bevy: String,
    matched: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FrontendParityReport {
    total_cases: usize,
    matched_cases: usize,
    mismatched_cases: usize,
    tui_unique_commands: Vec<String>,
    bevy_unique_commands: Vec<String>,
    mismatches: Vec<ParityCaseResult>,
    cases: Vec<ParityCaseResult>,
}

#[derive(Debug, Clone, Copy)]
enum KeyCase {
    Char(char),
    Up,
    Down,
    Left,
    Right,
}

impl KeyCase {
    fn label(self) -> String {
        match self {
            KeyCase::Char(ch) => format!("char:{ch}"),
            KeyCase::Up => "up".to_string(),
            KeyCase::Down => "down".to_string(),
            KeyCase::Left => "left".to_string(),
            KeyCase::Right => "right".to_string(),
        }
    }

    fn tui(self) -> UiKey {
        match self {
            KeyCase::Char(ch) => UiKey::Char(ch),
            KeyCase::Up => UiKey::Up,
            KeyCase::Down => UiKey::Down,
            KeyCase::Left => UiKey::Left,
            KeyCase::Right => UiKey::Right,
        }
    }

    fn bevy(self) -> BevyKey {
        match self {
            KeyCase::Char(ch) => BevyKey::Char(ch),
            KeyCase::Up => BevyKey::Up,
            KeyCase::Down => BevyKey::Down,
            KeyCase::Left => BevyKey::Left,
            KeyCase::Right => BevyKey::Right,
        }
    }
}

fn command_to_text(command: &Command) -> String {
    format!("{command:?}")
}

fn normalize_tui(action: UiAction) -> String {
    match action {
        UiAction::Dispatch(command) => command_to_text(&command),
        UiAction::Quit => "quit".to_string(),
        UiAction::SaveSlot => "save_slot".to_string(),
        UiAction::SaveAndQuit => "save_and_quit".to_string(),
        UiAction::LoadSlot => "load_slot".to_string(),
        UiAction::Restart => "restart_session".to_string(),
        UiAction::NewGame => "new_game".to_string(),
        UiAction::None => "none".to_string(),
    }
}

fn normalize_bevy(action: InputAction) -> String {
    match action {
        InputAction::Dispatch(command) => command_to_text(&command),
        InputAction::None => "none".to_string(),
        InputAction::StartGame => "start_game".to_string(),
        InputAction::NewGame => "new_game".to_string(),
        InputAction::SaveSlot => "save_slot".to_string(),
        InputAction::SaveAndQuit => "save_and_quit".to_string(),
        InputAction::LoadSlot => "load_slot".to_string(),
        InputAction::RestartSession => "restart_session".to_string(),
        InputAction::ReturnToMenu => "return_to_menu".to_string(),
        InputAction::QuitApp => "quit".to_string(),
        InputAction::TogglePause => "toggle_pause".to_string(),
        InputAction::StartWizardArena => "start_wizard_arena".to_string(),
    }
}

fn key_space() -> Vec<KeyCase> {
    let mut keys = vec![KeyCase::Up, KeyCase::Down, KeyCase::Left, KeyCase::Right];
    for ch in [
        'w', 'a', 's', 'd', 'h', 'j', 'k', 'l', 'W', 'A', 'X', 'D', 'g', ' ', '.', 'q', 'Q', 'P',
        'S', 'L', 'R', 'N', ',', '@', '<', '>', '?', '/', 'e', 'f', 'm', 'o', 'p', 'r', 't', 'v',
        'x', 'z', 'c', 'C', 'E', 'F', 'G', 'H', 'I', 'M', 'O', 'T', 'V', 'Z', 'u', 'y', 'b', 'n',
    ] {
        keys.push(KeyCase::Char(ch));
    }
    for ch in '1'..='9' {
        keys.push(KeyCase::Char(ch));
    }
    keys
}

fn run_report() -> FrontendParityReport {
    let mut cases = Vec::new();
    let mut mismatches = Vec::new();
    let mut tui_unique_commands = BTreeSet::new();
    let mut bevy_unique_commands = BTreeSet::new();

    for key in key_space() {
        let tui_value = normalize_tui(App::map_input(key.tui()));
        let bevy_value = normalize_bevy(map_shared_gameplay_key(key.bevy()));
        let matched = tui_value == bevy_value;
        let result =
            ParityCaseResult { key: key.label(), tui: tui_value, bevy: bevy_value, matched };
        if result.tui != "none" {
            tui_unique_commands.insert(result.tui.clone());
        }
        if result.bevy != "none" {
            bevy_unique_commands.insert(result.bevy.clone());
        }
        if !result.matched {
            mismatches.push(result.clone());
        }
        cases.push(result);
    }

    FrontendParityReport {
        total_cases: cases.len(),
        matched_cases: cases.len() - mismatches.len(),
        mismatched_cases: mismatches.len(),
        tui_unique_commands: tui_unique_commands.into_iter().collect(),
        bevy_unique_commands: bevy_unique_commands.into_iter().collect(),
        mismatches,
        cases,
    }
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn markdown(report: &FrontendParityReport) -> String {
    let mut out = Vec::new();
    out.push("# Frontend Command Parity Report".to_string());
    out.push(String::new());
    out.push(format!("- Total key cases: {}", report.total_cases));
    out.push(format!("- Matched cases: {}", report.matched_cases));
    out.push(format!("- Mismatched cases: {}", report.mismatched_cases));
    out.push(format!("- Status: {}", if report.mismatched_cases == 0 { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Key | TUI | Bevy | Match |".to_string());
    out.push("|---|---|---|---|".to_string());
    for item in &report.cases {
        out.push(format!(
            "| {} | {} | {} | {} |",
            item.key,
            item.tui,
            item.bevy,
            if item.matched { "yes" } else { "no" }
        ));
    }
    out.push(String::new());
    if !report.mismatches.is_empty() {
        out.push("## Mismatches".to_string());
        for item in &report.mismatches {
            out.push(format!("- {}: tui=`{}` bevy=`{}`", item.key, item.tui, item.bevy));
        }
        out.push(String::new());
    }
    out.join("\n")
}

fn write_outputs(report: &FrontendParityReport) -> Result<()> {
    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }
    let json_path = target.join("frontend-command-parity.json");
    let md_path = target.join("frontend-command-parity.md");
    fs::write(&json_path, serde_json::to_string_pretty(report).context("serialize parity report")?)
        .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(report))
        .with_context(|| format!("write {}", md_path.display()))?;
    Ok(())
}

fn main() -> Result<()> {
    let report = run_report();
    write_outputs(&report)?;
    println!(
        "frontend parity: total={}, matched={}, mismatched={}",
        report.total_cases, report.matched_cases, report.mismatched_cases
    );
    if report.mismatched_cases > 0 {
        bail!("frontend shared command mapping mismatch detected");
    }
    Ok(())
}
