# Research: Phase 3 - Inspection & Manipulation

## Current State Analysis
- `GameState` tracks `monsters` and `ground_items`.
- `step` function in `omega-core` handles the main game loop, including `run_monster_turn`.
- Phase 2 established `bevy_egui` integration and mouse-to-grid translation.

## Technical Strategy
1. **AI Pause**: 
    - Introduce `ai_paused: bool` to `GameState`.
    - Modify `step` in `omega-core` to skip `run_monster_turn` when `ai_paused` is active.
    - Note: We should decide if `ai_paused` also stops environmental effects. For a test chamber, stopping AI is the primary goal.
2. **Property Inspector**:
    - Add a `InspectorState` resource to `omega-bevy`.
    - Detect `MouseButton::Right` in a new system.
    - Translate click to grid coordinates.
    - Find the "top-most" entity at that position (Priority: Monster > Item).
    - Store the entity's ID in `InspectorState`.
3. **Inspector UI**:
    - Use `egui::Window` to show properties of the selected entity.
    - Monsters: Show `name`, `id`, `hp`, `max_hp`, `behavior`, `faction`.
    - Items: Show `name`, `id`, `family`.
4. **Global Manipulation**:
    - Add "Clear All Monsters" and "Clear All Items" buttons to the Spawner panel.
    - Add "AI Pause" toggle checkbox to the Spawner panel.

## Implementation Details
- `GameState` modification requires adding the field to the struct and updating `Default` and `serde` attributes.
- Entity lookup helper in `omega-core` or `omega-bevy`.
- Re-use coordinate translation logic from Phase 2.
