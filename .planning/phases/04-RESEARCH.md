# Research: Phase 4 - Environmental Hazards (Fire)

## Current State Analysis
- Fire is currently only modeled as projectiles or damage types, not as a persistent environmental hazard.
- `TileSiteCell` in `omega-core` has a `flags: u16` field that can be extended.
- `apply_environment_effects` exists but only handles traps.
- `omega-bevy` tilemap rendering is character-based but can be overlaid with specific `TileKind` variants.

## Technical Strategy
1. **Fire State**: 
    - Define `TILE_FLAG_BURNING: u16 = 0x0020;`
    - Define `TILE_FLAG_BURNT: u16 = 0x0040;`
2. **Propagation Logic**:
    - In `apply_environment_effects`, perform a cellular automata step:
        - For each tile:
            - If `BURNING`:
                - Check neighbors (ortho/diagonal).
                - If neighbor is `Grass` glyph and NOT `BURNING`, set it to `BURNING` with a certain probability (e.g., 30%).
                - Increment a local burnout counter (maybe stored in `aux` or just a fixed chance per turn).
                - With a small chance (e.g., 10%), unset `BURNING` and set `BURNT`.
            - If `BURNT`, change glyph to ash/rubble (e.g., `.`) or just keep as a flag for rendering.
3. **Frontend Presentation**:
    - Add `TileKind::Fire` to `omega-bevy`.
    - Map `Fire` to a flickering red/orange color or a flame glyph (`*` or `f`).
    - In `project_to_frame`, if a tile has `TILE_FLAG_BURNING`, push a `Fire` tile on top of the base terrain.
4. **Fire Brush Tool**:
    - Add `SpawnerCategory::Hazard` support.
    - Implement a "Fire" tool that sets the `BURNING` flag on clicked tiles.

## Performance
- Iterating 50x50 tiles per turn is trivial (~2500 iterations).
- To avoid "teleporting" fire, the propagation should use a double-buffer or a deferred update list.
- Bevy's `ArcaneCartographerPlugin` already handles per-frame updates; fire propagation happens at the turn-level (`step` function).

## Optimization (NFR-PERF-01)
- The requirement mentions 100+ entities. Fire-as-flags is much more performant than fire-as-entities. 
- We can stress test by filling the 50x50 arena with monsters and then igniting it.
