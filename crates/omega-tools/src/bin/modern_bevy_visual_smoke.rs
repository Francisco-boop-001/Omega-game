use anyhow::{Context, Result, bail};
use omega_bevy::{
    BevyKey, FrontendRuntime, TileKind, build_runtime_app_with_mode, enqueue_input, runtime_frame,
    runtime_status,
};
use omega_core::{
    GameMode, PendingProjectileAction, Position, ProjectileDamageType, ProjectileKind,
    TargetingInteraction,
};
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
struct Check {
    id: String,
    pass: bool,
    details: String,
}

#[derive(Debug, Serialize)]
struct Report {
    generated_at_utc: String,
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<Check>,
}

fn now_utc_unix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("unix:{}", now.as_secs())
}

fn write_artifacts(report: &Report) -> Result<()> {
    fs::create_dir_all("target/modern").context("create target/modern")?;
    let json = serde_json::to_string_pretty(report).context("serialize modern bevy smoke json")?;
    fs::write("target/modern/bevy-visual-smoke.json", json)
        .context("write target/modern/bevy-visual-smoke.json")?;

    let mut md = String::new();
    md.push_str("# Modern Bevy Visual Smoke\n\n");
    md.push_str(&format!(
        "- generated_at_utc: `{}`\n- total: `{}`\n- passed: `{}`\n- failed: `{}`\n- status: `{}`\n\n",
        report.generated_at_utc,
        report.total,
        report.passed,
        report.failed,
        if report.pass { "PASS" } else { "FAIL" }
    ));
    for check in &report.checks {
        md.push_str(&format!(
            "- [{}] `{}`: {}\n",
            if check.pass { "PASS" } else { "FAIL" },
            check.id,
            check.details
        ));
    }
    fs::write("target/modern/bevy-visual-smoke.md", md)
        .context("write target/modern/bevy-visual-smoke.md")?;
    Ok(())
}

fn main() -> Result<()> {
    let mut app = build_runtime_app_with_mode(0xBEE5_2001, GameMode::Modern);
    app.update();
    enqueue_input(&mut app, BevyKey::Enter);
    app.update();
    enqueue_input(&mut app, BevyKey::Char('w'));
    app.update();

    {
        let mut runtime = app.world_mut().resource_mut::<FrontendRuntime>();
        if let Some(session) = runtime.0.session.as_mut() {
            let player = session.state.player.position;
            session.state.pending_targeting_interaction = Some(TargetingInteraction {
                origin: player,
                cursor: Position { x: player.x + 2, y: player.y },
                mode: ProjectileKind::MagicMissile,
            });
            session.state.pending_projectile_action = Some(PendingProjectileAction {
                source_token: "smoke".to_string(),
                turn_minutes: 5,
                mode: ProjectileKind::MagicMissile,
                item_id: None,
                item_name: "test missile".to_string(),
                hit_bonus: 0,
                damage_bonus: 0,
                damage_min: 1,
                damage_max: 2,
                damage_type: ProjectileDamageType::Magic,
                max_range: 6,
                allows_drop: false,
            });
            session.state.transient_projectile_path = vec![
                Position { x: player.x + 1, y: player.y },
                Position { x: player.x + 2, y: player.y },
            ];
            session.state.transient_projectile_impact =
                Some(Position { x: player.x + 2, y: player.y });
        }
    }
    app.update();

    let status = runtime_status(&app);
    let frame =
        runtime_frame(&app).context("runtime frame missing after modern smoke bootstrap")?;

    let mut checks = Vec::new();
    checks.push(Check {
        id: "session_started_ingame".to_string(),
        pass: matches!(
            status.app_state,
            omega_bevy::AppState::InGame | omega_bevy::AppState::GameOver
        ),
        details: format!("app_state={:?}", status.app_state),
    });
    checks.push(Check {
        id: "mana_line_visible".to_string(),
        pass: frame.hud_lines.iter().any(|line| line.starts_with("Mana ")),
        details: "hud contains current/max mana line".to_string(),
    });
    checks.push(Check {
        id: "modern_objective_line_visible".to_string(),
        pass: frame.hud_lines.iter().any(|line| line.starts_with("Objective ")),
        details: "modern projection includes objective summary line".to_string(),
    });
    checks.push(Check {
        id: "target_cursor_visible".to_string(),
        pass: frame.tiles.iter().any(|tile| tile.kind == TileKind::TargetCursor),
        details: "target cursor tile rendered".to_string(),
    });
    checks.push(Check {
        id: "projectile_fx_visible".to_string(),
        pass: frame.tiles.iter().any(|tile| tile.kind == TileKind::ProjectileTrail)
            && frame.tiles.iter().any(|tile| tile.kind == TileKind::ProjectileImpact),
        details: "projectile trail and impact tiles rendered".to_string(),
    });

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.pass).count();
    let failed = total.saturating_sub(passed);
    let report = Report {
        generated_at_utc: now_utc_unix(),
        total,
        passed,
        failed,
        pass: failed == 0,
        checks,
    };
    write_artifacts(&report)?;

    if !report.pass {
        bail!("modern bevy visual smoke failed");
    }
    println!("modern_bevy_visual_smoke: PASS {}/{}", report.passed, report.total);
    Ok(())
}
