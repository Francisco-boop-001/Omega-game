use anyhow::{Context, Result, bail};
use omega_bevy::{BevyKey, build_runtime_app, enqueue_input, runtime_status};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct FrameTimeReport {
    schema_version: u32,
    scene: String,
    samples: usize,
    mean_ms: f64,
    p95_ms: f64,
    p99_ms: f64,
    max_ms: f64,
    threshold_p99_ms: f64,
    status: String,
}

fn target_dir() -> PathBuf {
    PathBuf::from("target")
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() as f64 * p).ceil() as usize)
        .saturating_sub(1)
        .min(sorted.len().saturating_sub(1));
    sorted[idx]
}

fn markdown(report: &FrameTimeReport) -> String {
    let mut out = Vec::new();
    out.push("# M5 Frame-Time Report".to_string());
    out.push(String::new());
    out.push(format!("- Scene: {}", report.scene));
    out.push(format!("- Samples: {}", report.samples));
    out.push(format!("- Mean: {:.6} ms", report.mean_ms));
    out.push(format!("- P95: {:.6} ms", report.p95_ms));
    out.push(format!("- P99: {:.6} ms", report.p99_ms));
    out.push(format!("- Max: {:.6} ms", report.max_ms));
    out.push(format!("- Threshold (P99): <= {:.3} ms", report.threshold_p99_ms));
    out.push(format!("- Status: {}", report.status));
    out.join("\n")
}

fn main() -> Result<()> {
    let threshold_p99_ms = 16.7f64;
    let samples = 1000usize;

    let mut app = build_runtime_app(0xF4A9_0001);
    app.update(); // boot -> menu
    enqueue_input(&mut app, BevyKey::Enter);
    app.update(); // start game

    let mut frame_samples = Vec::with_capacity(samples);
    for i in 0..samples {
        if i % 3 == 0 {
            enqueue_input(&mut app, BevyKey::Char(' '));
        } else if i % 3 == 1 {
            enqueue_input(&mut app, BevyKey::Char('d'));
        } else {
            enqueue_input(&mut app, BevyKey::Char('a'));
        }

        let start = Instant::now();
        app.update();
        frame_samples.push(start.elapsed().as_secs_f64() * 1_000.0);

        let status = runtime_status(&app);
        if status.should_quit {
            bail!("runtime requested quit during frame benchmark");
        }
    }

    frame_samples.sort_by(|a, b| a.total_cmp(b));
    let mean_ms = frame_samples.iter().sum::<f64>() / frame_samples.len() as f64;
    let p95_ms = percentile(&frame_samples, 0.95);
    let p99_ms = percentile(&frame_samples, 0.99);
    let max_ms = *frame_samples.last().unwrap_or(&0.0);
    let status = if p99_ms <= threshold_p99_ms { "PASS" } else { "FAIL" }.to_string();

    let report = FrameTimeReport {
        schema_version: 1,
        scene: "bevy_runtime_headless_projection_stress".to_string(),
        samples,
        mean_ms,
        p95_ms,
        p99_ms,
        max_ms,
        threshold_p99_ms,
        status: status.clone(),
    };

    let target = target_dir();
    if !target.exists() {
        fs::create_dir_all(&target).context("create target directory")?;
    }
    let json_path = target.join("m5-frame-time-report.json");
    let md_path = target.join("m5-frame-time-report.md");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&report).context("serialize frame-time report")?,
    )
    .with_context(|| format!("write {}", json_path.display()))?;
    fs::write(&md_path, markdown(&report))
        .with_context(|| format!("write {}", md_path.display()))?;

    println!(
        "m5 frame-time: samples={}, p99_ms={:.6}, threshold={:.3}, status={}",
        report.samples, report.p99_ms, report.threshold_p99_ms, report.status
    );

    if report.status != "PASS" {
        bail!("frame-time budget exceeded");
    }
    Ok(())
}
