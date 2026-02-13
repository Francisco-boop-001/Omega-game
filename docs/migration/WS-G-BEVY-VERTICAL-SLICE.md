# WS-G Execution: omega-bevy Vertical Slice

Status: Implemented (frontend adapter slice over `omega-core`)

## Scope from modernization plan

- Bevy app states (`Boot`, `Menu`, `InGame`, `Pause`, `GameOver`).
- Map rendering and sprite/tile pipeline.
- Input mapping parity with TUI.

## Delivered in this change

- `omega-bevy` app-state model and transition handling:
  - `AppState`
  - `BevyFrontend` with `boot`, `handle_key`, `apply_action`
- Shared gameplay input mapping parity (same command intents as TUI):
  - `map_shared_gameplay_key`
  - `map_input`
- Graphical projection/render contract suitable for Bevy ECS systems:
  - `TileKind`
  - `SpriteRef` and `SpriteAtlas`
  - `TileRender`
  - `RenderFrame`
  - `project_to_frame`
- Core dispatch integration:
  - `GameSession::dispatch` calls `omega_core::step(state, command, rng)`
- Bootstrap helper for smoke flows:
  - `run_headless_bootstrap`

## Tests added

- App state flow: `Boot -> Menu -> InGame -> Pause -> InGame`
- Shared input parity checks (`w`, `D`, `g`, numeric drop slots)
- Render projection checks (player tile + HUD lines)

## Known follow-up

- Wire these state/input/render contracts into actual Bevy runtime systems:
  - app-state resources/schedules
  - camera + tile sprite spawn/update systems
  - event-driven HUD and log UI systems
