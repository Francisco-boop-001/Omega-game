use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use omega_bevy::{
    OmegaBevyRuntimePlugin, default_save_slot_path_for_mode, presentation::ArcaneCartographerPlugin,
};
use omega_content::bootstrap_game_state_with_mode;
use omega_core::GameMode;

fn parse_mode(args: &[String]) -> GameMode {
    let mut mode = GameMode::Modern;
    let mut idx = 0usize;
    while idx + 1 < args.len() {
        if args[idx] == "--mode" {
            mode = match args[idx + 1].to_ascii_lowercase().as_str() {
                "classic" => GameMode::Classic,
                _ => GameMode::Modern,
            };
            idx += 1;
        }
        idx += 1;
    }
    mode
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let mode = parse_mode(&args);
    let (mut bootstrap, diagnostics) = bootstrap_game_state_with_mode(mode)
        .with_context(|| format!("bootstrap runtime in {} mode", mode.as_str()))?;
    bootstrap.options.interactive_sites = true;
    bootstrap.log.push(format!(
        "Visual bootstrap: source={}, spawn={}",
        diagnostics.map_source, diagnostics.player_spawn_source
    ));

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Omega - Arcane Cartographer ({})", mode.as_str()),
            resolution: WindowResolution::new(1600.0, 960.0),
            resizable: true,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(OmegaBevyRuntimePlugin {
        session_seed: 0xBEE5_1001,
        mode: Some(mode),
        bootstrap_state: Some(bootstrap),
        save_slot: Some(default_save_slot_path_for_mode(mode)),
    })
    .add_plugins(ArcaneCartographerPlugin);

    app.run();
    Ok(())
}
