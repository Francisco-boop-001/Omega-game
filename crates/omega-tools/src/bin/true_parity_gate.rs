use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TrackGateStatus {
    track: String,
    items_total: usize,
    items_done: usize,
    complete: bool,
    required_artifacts: Vec<String>,
    missing_artifacts: Vec<String>,
    failing_artifacts: Vec<String>,
    stale_artifacts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TrueParityGateReport {
    generated_at_utc: String,
    pass: bool,
    failed_tracks: usize,
    tracks: Vec<TrackGateStatus>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn parse_track_progress(plan_raw: &str) -> BTreeMap<String, (usize, usize)> {
    let mut current_track: Option<String> = None;
    let mut counts: BTreeMap<String, (usize, usize)> = BTreeMap::new();

    for line in plan_raw.lines() {
        if let Some(rest) = line.strip_prefix("## Track ") {
            let token = rest.split(':').next().unwrap_or("").trim();
            if token.starts_with('T') {
                current_track = Some(token.to_string());
                counts.entry(token.to_string()).or_insert((0, 0));
            } else {
                current_track = None;
            }
            continue;
        }

        let Some(track) = current_track.as_ref() else {
            continue;
        };

        if !line.contains(&format!("`{track}-")) {
            continue;
        }

        let entry = counts.entry(track.clone()).or_insert((0, 0));
        if line.contains("- [x]") {
            entry.0 += 1;
            entry.1 += 1;
        } else if line.contains("- [ ]") {
            entry.1 += 1;
        }
    }

    counts
}

fn artifact_requirements() -> BTreeMap<String, Vec<String>> {
    BTreeMap::from([
        (
            "T0".to_string(),
            vec![
                "target/true-parity-deviations.json".to_string(),
                "docs/migration/FULL_OMEGA_TRUE_PARITY_PLAN.md".to_string(),
                "docs/migration/FULL_OMEGA_TRUE_PARITY_SCORECARD.md".to_string(),
                "docs/migration/FULL_OMEGA_TRUE_PARITY_CLOSURE_REVIEW.md".to_string(),
            ],
        ),
        ("T1".to_string(), vec!["target/true-startup-parity.json".to_string()]),
        ("T2".to_string(), vec!["target/true-environment-transition-matrix.json".to_string()]),
        ("T3".to_string(), vec!["target/true-command-behavior-matrix.json".to_string()]),
        ("T4".to_string(), vec!["target/true-spell-parity-matrix.json".to_string()]),
        ("T5".to_string(), vec!["target/true-item-parity-matrix.json".to_string()]),
        ("T6".to_string(), vec!["target/true-combat-encounter-matrix.json".to_string()]),
        ("T7".to_string(), vec!["target/true-site-economy-social-matrix.json".to_string()]),
        ("T8".to_string(), vec!["target/true-progression-ending-matrix.json".to_string()]),
        ("T9".to_string(), vec!["target/true-compatibility-matrix.json".to_string()]),
        ("T10".to_string(), vec!["target/true-frontend-workflow-matrix.json".to_string()]),
        (
            "T11".to_string(),
            vec![
                "target/true-parity-regression-dashboard.json".to_string(),
                "target/true-burnin-window.json".to_string(),
            ],
        ),
        (
            "T12".to_string(),
            vec![
                "target/true-parity-baseline-freeze.json".to_string(),
                "docs/migration/FULL_OMEGA_TRUE_PARITY_CLOSURE_REVIEW.md".to_string(),
            ],
        ),
    ])
}

fn markdown(report: &TrueParityGateReport) -> String {
    let mut out = Vec::new();
    out.push("# True Parity Gate".to_string());
    out.push(String::new());
    out.push(format!("- Generated: {}", report.generated_at_utc));
    out.push(format!("- Status: {}", if report.pass { "PASS" } else { "FAIL" }));
    out.push(format!("- Failed tracks: {}", report.failed_tracks));
    out.push(String::new());
    out.push("| Track | Done/Total | Complete | Missing | Failing | Stale |".to_string());
    out.push("|---|---:|---|---|---|---|".to_string());
    for track in &report.tracks {
        out.push(format!(
            "| {} | {}/{} | {} | {} | {} | {} |",
            track.track,
            track.items_done,
            track.items_total,
            if track.complete { "YES" } else { "NO" },
            if track.missing_artifacts.is_empty() {
                "-".to_string()
            } else {
                track.missing_artifacts.join(", ")
            },
            if track.failing_artifacts.is_empty() {
                "-".to_string()
            } else {
                track.failing_artifacts.join(", ")
            },
            if track.stale_artifacts.is_empty() {
                "-".to_string()
            } else {
                track.stale_artifacts.join(", ")
            }
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn read_json(path: &str) -> Option<Value> {
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str::<Value>(&raw).ok()
}

fn artifact_is_pass(path: &str) -> bool {
    if !path.ends_with(".json") {
        return true;
    }
    let Some(value) = read_json(path) else {
        return false;
    };
    if let Some(pass) = value.get("pass").and_then(|v| v.as_bool()) {
        return pass;
    }
    if let Some(status) = value.get("status").and_then(|v| v.as_str()) {
        return status == "PASS";
    }
    true
}

fn artifact_is_stale(path: &str, now: SystemTime, max_age: Duration) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return true;
    };
    let Ok(modified) = metadata.modified() else {
        return true;
    };
    match now.duration_since(modified) {
        Ok(age) => age > max_age,
        Err(_) => false,
    }
}

fn main() -> Result<()> {
    let plan_path = Path::new("docs/migration/FULL_OMEGA_TRUE_PARITY_PLAN.md");
    let raw =
        fs::read_to_string(plan_path).with_context(|| format!("read {}", plan_path.display()))?;
    let progress = parse_track_progress(&raw);
    let requirements = artifact_requirements();

    let now = SystemTime::now();
    let max_age = Duration::from_secs(24 * 60 * 60);
    let mut tracks = Vec::new();
    let mut failed_tracks = 0usize;
    for (track, (items_done, items_total)) in progress {
        let complete = items_total > 0 && items_done == items_total;
        let required_artifacts = requirements.get(&track).cloned().unwrap_or_default();
        let (missing_artifacts, failing_artifacts, stale_artifacts) = if complete {
            let missing = required_artifacts
                .iter()
                .filter(|artifact| !Path::new(artifact.as_str()).exists())
                .cloned()
                .collect::<Vec<_>>();
            let failing = required_artifacts
                .iter()
                .filter(|artifact| {
                    Path::new(artifact.as_str()).exists() && !artifact_is_pass(artifact)
                })
                .cloned()
                .collect::<Vec<_>>();
            let stale = required_artifacts
                .iter()
                .filter(|artifact| {
                    Path::new(artifact.as_str()).exists()
                        && artifact_is_stale(artifact, now, max_age)
                })
                .cloned()
                .collect::<Vec<_>>();
            (missing, failing, stale)
        } else {
            (Vec::new(), Vec::new(), Vec::new())
        };

        if complete
            && (!missing_artifacts.is_empty()
                || !failing_artifacts.is_empty()
                || !stale_artifacts.is_empty())
        {
            failed_tracks += 1;
        }
        tracks.push(TrackGateStatus {
            track,
            items_total,
            items_done,
            complete,
            required_artifacts,
            missing_artifacts,
            failing_artifacts,
            stale_artifacts,
        });
    }

    tracks.sort_by(|left, right| left.track.cmp(&right.track));
    let report = TrueParityGateReport {
        generated_at_utc: now_utc_unix(),
        pass: failed_tracks == 0,
        failed_tracks,
        tracks,
    };

    let target = Path::new("target");
    if !target.exists() {
        fs::create_dir_all(target).context("create target directory")?;
    }
    let json_path = target.join("true-parity-gate.json");
    let md_path = target.join("true-parity-gate.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize true parity gate")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "true parity gate: status={}, failed_tracks={}",
        if report.pass { "PASS" } else { "FAIL" },
        report.failed_tracks
    );
    if !report.pass {
        bail!("true parity gate failed for one or more completed tracks");
    }
    Ok(())
}
