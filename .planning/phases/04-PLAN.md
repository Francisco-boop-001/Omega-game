# Plan: Phase 4 - Environmental Hazards (Fire)

## Goal
Implement a functional fire propagation system and a manual ignition tool for stress testing the arena.

## Tasks

### Task 1: Core Fire Support
- [ ] Define `TILE_FLAG_BURNING` and `TILE_FLAG_BURNT` in `crates/omega-core/src/lib.rs`.
- [ ] Update `apply_environment_effects` in `crates/omega-core/src/lib.rs` to implement fire spread logic.
    - Fire spreads to adjacent Grass (`"`) tiles.
    - Fire is blocked by Water (`~`) and Stone (`.`).
    - Fire has a chance to burn out and turn into Stone/Floor.

### Task 2: Bevy Presentation
- [ ] Add `Fire` variant to `TileKind` in `crates/omega-bevy/src/lib.rs`.
- [ ] Map `TileKind::Fire` to `EffectColorId::Fire` in `to_color_id`.
- [ ] Update `project_to_frame` in `crates/omega-bevy/src/lib.rs` to detect `TILE_FLAG_BURNING` and render a fire glyph.
- [ ] Update `glyph_for_tile` and `layer_priority` in `crates/omega-bevy/src/presentation/tilemap.rs` for `TileKind::Fire`.

### Task 3: Fire Brush Tool
- [ ] Add "Fire" entry to `item_catalog` (or create a dedicated `hazard_catalog`) in `SpawnerState` (`crates/omega-bevy/src/presentation/spawner.rs`).
- [ ] Update `mouse_spawning_system` to handle `SpawnerCategory::Hazard`.
- [ ] Implement logic to set `TILE_FLAG_BURNING` on the target tile when using the Fire Brush.

### Task 4: Stress Testing & Optimization
- [ ] Verify that spawning 100+ entities (monsters/items) combined with active fire maintains 60 FPS.
- [ ] Ensure fire propagation does not leak into the `InGame` state unless explicitly triggered.

## Verification
- [ ] Igniting a single grass tile spreads to all connected grass tiles over several turns.
- [ ] Fire does not cross water or stone walls.
- [ ] Burning out a tile changes its visual state (e.g., to ash/stone).
- [ ] Performance remains stable during large-scale fires.
