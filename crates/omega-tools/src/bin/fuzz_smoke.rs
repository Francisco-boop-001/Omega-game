use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use omega_content::parse_legacy_map_from_str;
use omega_save::{decode_json, decode_state_json};

fn main() -> Result<()> {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("fuzz-corpus");
    let map_dir = base.join("content-map");
    let save_dir = base.join("save-json");

    let map_count = run_map_corpus(&map_dir)?;
    let save_count = run_save_corpus(&save_dir)?;

    println!("fuzz smoke passed: {} map seeds, {} save seeds", map_count, save_count);
    Ok(())
}

fn run_map_corpus(dir: &Path) -> Result<usize> {
    let mut count = 0usize;
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        let raw = fs::read_to_string(&path)?;
        let _ = parse_legacy_map_from_str(&raw, &path.display().to_string());
        count += 1;
    }
    if count == 0 {
        bail!("map fuzz corpus is empty: {}", dir.display());
    }
    Ok(count)
}

fn run_save_corpus(dir: &Path) -> Result<usize> {
    let mut count = 0usize;
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        let raw = fs::read_to_string(&path)?;
        let _ = decode_json(&raw);
        let _ = decode_state_json(&raw);
        count += 1;
    }
    if count == 0 {
        bail!("save fuzz corpus is empty: {}", dir.display());
    }
    Ok(count)
}
