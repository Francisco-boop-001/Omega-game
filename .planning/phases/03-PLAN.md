# Plan: Phase 3 - Inspection & Manipulation

## Goal
Enable real-time monitoring and control over spawned entities through a property inspector and global arena manipulation tools.

## Tasks

### Task 1: Core AI Pause Support
- [ ] Add `pub ai_paused: bool` to `GameState` in `crates/omega-core/src/lib.rs` (with `#[serde(default)]`).
- [ ] Initialize `ai_paused` to `false` in `GameState::new`.
- [ ] Update `step` in `crates/omega-core/src/lib.rs` to skip `run_monster_turn` if `state.ai_paused` is true.

### Task 2: Entity Selection System
- [ ] Create `crates/omega-bevy/src/presentation/inspector.rs`.
- [ ] Define `InspectorTarget` enum (Monster(u64), Item(u32)).
- [ ] Define `InspectorState` resource with `pub target: Option<InspectorTarget>`.
- [ ] Implement `mouse_inspector_system` to handle right-clicks and update `InspectorState`.
- [ ] Implement helper to find entity at a given `Position`.

### Task 3: Egui Inspector UI
- [ ] Implement `inspector_ui_system` in `inspector.rs`.
- [ ] Render a draggable `egui::Window` when an entity is selected.
- [ ] Display detailed stats for the selected entity.
- [ ] Add a "Despawn" button to the inspector for individual entity removal.

### Task 4: Arena Manipulation UI
- [ ] Update `spawner_ui_system` in `spawner.rs` to add global controls.
- [ ] Add "Clear Monsters" button.
- [ ] Add "Clear Items" button.
- [ ] Add "Pause AI" toggle checkbox (binding to `state.ai_paused`).

### Task 5: Integration
- [ ] Register `InspectorState` and new systems in `ArcaneCartographerPlugin`.

## Verification
- [ ] Right-clicking a monster opens the inspector with correct data.
- [ ] Toggling "Pause AI" prevents monsters from moving or attacking when the player moves.
- [ ] "Clear Monsters" removes all monsters from the arena.
- [ ] "Clear Items" removes all ground items from the arena.
- [ ] Closing the inspector window works correctly.
