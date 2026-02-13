use anyhow::{Context, Result, bail};
use omega_content::parse_legacy_map_from_str;
use omega_save::{decode_json, decode_state_json};
use std::fs;
use std::path::{Path, PathBuf};

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn run_map_corpus(dir: &Path) -> Result<(usize, usize)> {
    let mut total = 0usize;
    let mut accepted = 0usize;
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        total += 1;
        let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        if parse_legacy_map_from_str(&raw, &path.display().to_string()).is_ok() {
            accepted += 1;
        }
    }
    Ok((total, accepted))
}

fn run_save_corpus(dir: &Path) -> Result<(usize, usize)> {
    let mut total = 0usize;
    let mut decoded = 0usize;
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        total += 1;
        let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        if decode_json(&raw).is_ok() || decode_state_json(&raw).is_ok() {
            decoded += 1;
        }
    }
    Ok((total, decoded))
}

fn main() -> Result<()> {
    let map_dir = Path::new("crates/omega-tools/fuzz-corpus/content-map");
    let save_dir = Path::new("crates/omega-tools/fuzz-corpus/save-json");

    let (map_total, map_accepted) = run_map_corpus(map_dir)?;
    let (save_total, save_decoded) = run_save_corpus(save_dir)?;

    if map_total == 0 || save_total == 0 {
        bail!("fuzz corpora cannot be empty");
    }

    let status = if map_accepted > 0 && save_decoded > 0 { "PASS" } else { "FAIL" };
    let mut out = Vec::new();
    out.push("# M5 Weekly Fuzz Report".to_string());
    out.push(String::new());
    out.push("- Campaign: offline corpus smoke replay".to_string());
    out.push(format!("- Map corpus total: {}", map_total));
    out.push(format!("- Map corpus accepted parses: {}", map_accepted));
    out.push(format!("- Save corpus total: {}", save_total));
    out.push(format!("- Save corpus decodable cases: {}", save_decoded));
    out.push(format!("- Status: {}", status));
    out.push(String::new());
    out.push("## Notes".to_string());
    out.push(String::new());
    out.push(
        "- This weekly artifact is generated from checked-in corpora and parser/decoder runs."
            .to_string(),
    );
    out.push("- No untriaged crashes observed during this run.".to_string());

    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }
    let md_path = target.join("m5-fuzz-weekly-report.md");
    fs::write(&md_path, out.join("\n")).with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "m5 fuzz weekly report: map_total={}, save_total={}, status={}",
        map_total, save_total, status
    );

    if status != "PASS" {
        bail!("weekly fuzz report status is FAIL");
    }
    Ok(())
}
