use anyhow::{Context, Result, bail};
use omega_content::parse_legacy_map_from_str;
use omega_save::{decode_json, decode_state_json};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SecurityCheck {
    name: String,
    status: String,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SecurityAuditReport {
    schema_version: u32,
    dependency_audit_mode: String,
    checks: Vec<SecurityCheck>,
    overall_status: String,
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn count_files(path: &Path) -> Result<usize> {
    let mut count = 0usize;
    for entry in fs::read_dir(path).with_context(|| format!("read {}", path.display()))? {
        let entry = entry?;
        if entry.path().is_file() {
            count += 1;
        }
    }
    Ok(count)
}

fn check_save_slot_path_policy() -> SecurityCheck {
    let tui = omega_tui::default_save_slot_path();
    let bevy = omega_bevy::default_save_slot_path();
    let ok = tui.starts_with("target") && bevy.starts_with("target");
    SecurityCheck {
        name: "save_slot_path_policy".to_string(),
        status: if ok { "PASS" } else { "FAIL" }.to_string(),
        details: format!("tui={}, bevy={}", tui.display(), bevy.display()),
    }
}

fn check_parser_rejection() -> SecurityCheck {
    let malformed_map = "v2\nmap 9\n1 3,2\nabc\nx\n==\n";
    let malformed_save = "{ this is not valid json }";
    let map_rejected = parse_legacy_map_from_str(malformed_map, "malformed.map").is_err();
    let save_rejected =
        decode_json(malformed_save).is_err() && decode_state_json(malformed_save).is_err();
    let ok = map_rejected && save_rejected;
    SecurityCheck {
        name: "malformed_input_rejection".to_string(),
        status: if ok { "PASS" } else { "FAIL" }.to_string(),
        details: format!("map_rejected={}, save_rejected={}", map_rejected, save_rejected),
    }
}

fn check_corpus_presence() -> Result<SecurityCheck> {
    let map_count = count_files(Path::new("crates/omega-tools/fuzz-corpus/content-map"))?;
    let save_count = count_files(Path::new("crates/omega-tools/fuzz-corpus/save-json"))?;
    let ok = map_count > 0 && save_count > 0;
    Ok(SecurityCheck {
        name: "fuzz_corpus_presence".to_string(),
        status: if ok { "PASS" } else { "FAIL" }.to_string(),
        details: format!("map_seeds={}, save_seeds={}", map_count, save_count),
    })
}

fn check_cargo_lock_present() -> SecurityCheck {
    let lock_present = Path::new("Cargo.lock").exists();
    SecurityCheck {
        name: "dependency_lockfile_present".to_string(),
        status: if lock_present { "PASS" } else { "FAIL" }.to_string(),
        details: if lock_present {
            "Cargo.lock present for reproducible dependency resolution".to_string()
        } else {
            "Cargo.lock missing".to_string()
        },
    }
}

fn main() -> Result<()> {
    let checks = vec![
        check_save_slot_path_policy(),
        check_parser_rejection(),
        check_cargo_lock_present(),
        check_corpus_presence()?,
    ];

    let has_fail = checks.iter().any(|check| check.status == "FAIL");
    let report = SecurityAuditReport {
        schema_version: 1,
        dependency_audit_mode: "offline_static_checks".to_string(),
        checks,
        overall_status: if has_fail { "FAIL" } else { "PASS" }.to_string(),
    };

    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }
    let out_path = target.join("m5-security-audit.json");
    fs::write(
        &out_path,
        serde_json::to_string_pretty(&report).context("serialize security audit report")?,
    )
    .with_context(|| format!("write {}", out_path.display()))?;

    println!("m5 security audit: checks={}, status={}", report.checks.len(), report.overall_status);

    if report.overall_status != "PASS" {
        bail!("security audit checks failed");
    }
    Ok(())
}
