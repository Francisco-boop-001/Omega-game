use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalMechanic {
    pub id: String,
    pub name: String,
    pub tier: String,
    pub domain: String,
    pub legacy_anchor: String,
    pub rust_anchor: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalBranch {
    pub id: String,
    pub service: String,
    pub kind: String,
    pub legacy_anchor: String,
    pub rust_anchor: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateVector {
    pub gold: Option<i32>,
    pub bank_gold: Option<i32>,
    pub guild_rank: Option<u8>,
    pub priest_rank: Option<u8>,
    pub alignment: Option<String>,
    pub law_chaos_score: Option<i32>,
    pub deity_favor: Option<i32>,
    pub legal_heat: Option<i32>,
    pub quest_state: Option<String>,
    pub quest_steps_completed: Option<u8>,
    pub main_quest_stage: Option<String>,
    pub arena_rank: Option<i8>,
    pub arena_match_active: Option<bool>,
    pub inventory_count: Option<usize>,
    pub known_spells_count: Option<usize>,
    pub world_mode: Option<String>,
    pub map_semantic: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiffResult {
    pub id: String,
    pub pass: bool,
    pub details: String,
    pub mismatches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoverageResult {
    pub total: usize,
    pub covered: usize,
    pub missing: usize,
    pub pass: bool,
    pub missing_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CertDefect {
    pub id: String,
    pub severity: String,
    pub area: String,
    pub title: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CertDefectBoard {
    pub generated_at_utc: String,
    pub total: usize,
    pub open: usize,
    pub defects: Vec<CertDefect>,
}

pub fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

pub fn certification_root() -> PathBuf {
    PathBuf::from("target").join("certification")
}

pub fn contracts_dir() -> PathBuf {
    certification_root().join("contracts")
}

pub fn diff_dir() -> PathBuf {
    certification_root().join("diff")
}

pub fn coverage_dir() -> PathBuf {
    certification_root().join("coverage")
}

pub fn smoke_dir() -> PathBuf {
    certification_root().join("smoke")
}

pub fn ensure_cert_dirs() -> Result<()> {
    for dir in [certification_root(), contracts_dir(), diff_dir(), coverage_dir(), smoke_dir()] {
        if !dir.exists() {
            fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
        }
    }
    Ok(())
}

pub fn write_json<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let raw = serde_json::to_string_pretty(value).context("serialize json")?;
    fs::write(path, raw).with_context(|| format!("write {}", path.display()))
}

pub fn read_json<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str::<T>(&raw).with_context(|| format!("decode {}", path.display()))
}

pub fn copy_if_exists(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<bool> {
    let src = src.as_ref();
    if !src.exists() {
        return Ok(false);
    }
    let dest = dest.as_ref();
    if let Some(parent) = dest.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::copy(src, dest).with_context(|| format!("copy {} -> {}", src.display(), dest.display()))?;
    Ok(true)
}
