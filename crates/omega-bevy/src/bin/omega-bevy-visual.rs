use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use omega_bevy::{
    AppState, FrontendRuntime, InputAction, OmegaBevyRuntimePlugin, RuntimeStatus,
    default_save_slot_path_for_mode,
    presentation::{ArcaneCartographerPlugin, UiBootLatch},
};
use omega_content::bootstrap_game_state_with_mode;
use omega_core::GameMode;

fn parse_args(args: &[String]) -> (GameMode, bool) {
    let mut mode = GameMode::Modern;
    let mut arena = false;
    let mut idx = 0usize;
    while idx < args.len() {
        if args[idx] == "--mode" && idx + 1 < args.len() {
            mode = match args[idx + 1].to_ascii_lowercase().as_str() {
                "classic" => GameMode::Classic,
                _ => GameMode::Modern,
            };
            idx += 1;
        } else if args[idx] == "--arena" {
            arena = true;
        }
        idx += 1;
    }
    (mode, arena)
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let (mode, start_in_arena) = parse_args(&args);
    
    let (mut bootstrap, diagnostics) = if start_in_arena {
        let (mut state, diag) = omega_content::bootstrap_wizard_arena().expect("arena bootstrap");
        state.mode = GameMode::Modern;
        (state, diag)
    } else {
        bootstrap_game_state_with_mode(mode)
            .with_context(|| format!("bootstrap runtime in {} mode", mode.as_str()))?
    };
    
    bootstrap.options.interactive_sites = true;
    if start_in_arena {
        bootstrap.log.push("Visual bootstrap: Wizard's Arena mode enabled".to_string());
    } else {
        bootstrap.log.push(format!(
            "Visual bootstrap: source={}, spawn={}",
            diagnostics.map_source, diagnostics.player_spawn_source
        ));
    }

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Omega - Arcane Cartographer ({})", if start_in_arena { "Arena" } else { mode.as_str() }),
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

    // If starting in arena, we need to ensure the session is started and state is set
    if start_in_arena {
        app.add_systems(PostStartup, |mut runtime: ResMut<FrontendRuntime>, mut status: ResMut<RuntimeStatus>| {
            runtime.0.apply_action(InputAction::StartGame);
            runtime.0.app_state = AppState::WizardArena;
            status.app_state = AppState::WizardArena;
        });
        
        // Ensure UI knows we've already started
        app.insert_resource(UiBootLatch { started_session: true });
    }

    app.run();
    Ok(())
}
