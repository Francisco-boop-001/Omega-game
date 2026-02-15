use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;
use omega_bevy::simulation::diagnostics::CA_UPDATE_TIME;
use omega_bevy::simulation::plugin::SimulationPlugin;
use omega_bevy::simulation::projectiles::Projectile;
use omega_bevy::simulation::safety::SafetyConfig;
use omega_bevy::simulation::turret::TurretMode;
use omega_core::simulation::catastrophe::Catastrophe;
use omega_core::simulation::grid::CaGrid;
use omega_core::simulation::wind::WindGrid;
use std::time::{Duration, Instant};

#[derive(Default)]
struct ScenarioResult {
    name: String,
    ca_avg: f64,
    ca_p95: f64,
    ca_max: f64,
    frame_avg: f64,
    frame_p95: f64,
    peak_projectiles: usize,
    passed: bool,
}

fn main() {
    let mut results = Vec::new();

    println!("=== Starting Arena Stress Test ===");

    // Scenario: Baseline
    results.push(run_scenario("Baseline", 200, |_app| {
        // Just setup
    }));

    // Scenario: 100+ Projectiles
    results.push(run_scenario("100+ Projectiles", 400, |app: &mut App| {
        let mut turret = app.world_mut().resource_mut::<TurretMode>();
        turret.active = true;
        turret.fire_rate_hz = 50.0;
    }));

    // Scenario: Great Flood
    results.push(run_scenario("Catastrophe: Great Flood", 200, |app: &mut App| {
        let mut grid = app.world_mut().resource_mut::<CaGrid>();
        Catastrophe::great_flood(&mut grid, (64, 64));
    }));

    // Scenario: Forest Fire Jump
    results.push(run_scenario("Catastrophe: Forest Fire Jump", 200, |app: &mut App| {
        let mut grid = app.world_mut().resource_mut::<CaGrid>();
        Catastrophe::forest_fire_jump(&mut grid, (64, 64));
    }));

    // Scenario: Massive Windstorm
    results.push(run_scenario("Catastrophe: Massive Windstorm", 200, |app: &mut App| {
        let mut wind_grid = app.world_mut().resource_mut::<WindGrid>();
        Catastrophe::massive_windstorm(&mut wind_grid);
    }));

    // Scenario: Fuel Field
    results.push(run_scenario("Catastrophe: Fuel Field", 200, |app: &mut App| {
        let mut grid = app.world_mut().resource_mut::<CaGrid>();
        Catastrophe::fuel_field(&mut grid);
    }));

    // Scenario: Doomsday + Turret
    results.push(run_scenario("Doomsday + Turret", 400, |app: &mut App| {
        app.world_mut().resource_scope::<CaGrid, _>(|world, mut grid| {
            let mut wind_grid = world.resource_mut::<WindGrid>();
            Catastrophe::doomsday(&mut grid, &mut wind_grid);
        });
        let mut turret = app.world_mut().resource_mut::<TurretMode>();
        turret.active = true;
        turret.fire_rate_hz = 20.0;
    }));

    // Scenario: Emergency Recovery
    results.push(run_scenario("Emergency Recovery", 200, |app: &mut App| {
        let mut safety = app.world_mut().resource_mut::<SafetyConfig>();
        safety.cleanup_threshold_fps = 120.0; // Force trigger emergency
    }));

    println!("\n=== Arena Stress Test Report ===\n");
    let mut all_passed = true;
    for res in &results {
        println!("Scenario: {}", res.name);
        println!(
            "  CA Update: avg={:.2}ms  p95={:.2}ms  max={:.2}ms",
            res.ca_avg, res.ca_p95, res.ca_max
        );
        if res.peak_projectiles > 0 || res.frame_avg > 0.0 {
            println!("  Peak Projectile Count: {}", res.peak_projectiles);
            println!("  Frame Time: avg={:.2}ms  p95={:.2}ms", res.frame_avg, res.frame_p95);
        }
        println!("  Result: {}", if res.passed { "PASS" } else { "FAIL" });
        println!();
        if !res.passed {
            all_passed = false;
        }
    }

    println!("=== OVERALL: {} ===", if all_passed { "PASS" } else { "FAIL" });

    if !all_passed {
        std::process::exit(1);
    }
}

fn run_scenario<F>(name: &str, ticks: usize, setup: F) -> ScenarioResult
where
    F: FnOnce(&mut App),
{
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(SimulationPlugin::new(128, 128, 0xA11C_E001));

    setup(&mut app);

    // Warmup
    for _ in 0..100 {
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_secs_f32(1.0 / 64.0));
        }
        app.update();
    }

    let mut ca_times: Vec<f64> = Vec::new();
    let mut frame_times: Vec<f64> = Vec::new();
    let mut peak_projectiles = 0;

    for _ in 0..ticks {
        let start = Instant::now();

        {
            let mut time = app.world_mut().resource_mut::<Time>();
            let dt = Duration::from_secs_f32(1.0 / 64.0);
            time.advance_by(dt);
        }

        app.update();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        frame_times.push(elapsed);

        let diagnostics = app.world().resource::<DiagnosticsStore>();
        if let Some(diag) = diagnostics.get(&CA_UPDATE_TIME)
            && let Some(val) = diag.smoothed().or_else(|| diag.values().last().copied())
        {
            ca_times.push(val);
        }

        let mut query = app.world_mut().query::<&Projectile>();
        let projectiles = query.iter(app.world()).count();
        if projectiles > peak_projectiles {
            peak_projectiles = projectiles;
        }
    }

    if ca_times.is_empty() {
        return ScenarioResult { name: name.to_string(), passed: false, ..Default::default() };
    }

    ca_times.sort_by(|a, b| a.total_cmp(b));
    frame_times.sort_by(|a, b| a.total_cmp(b));

    let ca_avg = ca_times.iter().sum::<f64>() / ca_times.len() as f64;
    let ca_p95 = ca_times[((ca_times.len() as f64 * 0.95) as usize).min(ca_times.len() - 1)];
    let ca_max = *ca_times.last().unwrap_or(&0.0);

    let frame_avg = frame_times.iter().sum::<f64>() / frame_times.len() as f64;
    let frame_p95 =
        frame_times[((frame_times.len() as f64 * 0.95) as usize).min(frame_times.len() - 1)];

    let mut passed = ca_avg < 2.0;

    if name == "100+ Projectiles" {
        if peak_projectiles < 100 {
            passed = false;
        }
        if frame_avg > 16.66 {
            passed = false;
        }
    }

    ScenarioResult {
        name: name.to_string(),
        ca_avg,
        ca_p95,
        ca_max,
        frame_avg,
        frame_p95,
        peak_projectiles,
        passed,
    }
}
