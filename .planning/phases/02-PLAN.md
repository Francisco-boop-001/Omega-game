# Plan: Phase 2 - Spawner Interface

## Goal
Provide a user-friendly UI for placing entities into the arena using an egui side-panel and click-to-spawn interaction.

## Tasks

### Task 1: Spawner Resource & Data
- [ ] Create `crates/omega-bevy/src/presentation/spawner.rs`.
- [ ] Define `SpawnerCategory` enum (Monster, Item, Hazard).
- [ ] Define `SpawnerState` resource with `selected_category`, `selected_id`, and visibility toggle.
- [ ] Populate `SpawnerState` with a catalog of test entities (e.g., rat, goblin, practice blade).

### Task 2: Egui Spawner UI
- [ ] Implement `spawner_ui_system` in `spawner.rs`.
- [ ] Use `egui::SidePanel::right` to render the menu.
- [ ] Show categories as tabs or a list.
- [ ] Highlight the currently selected entity.
- [ ] Only show the panel when `AppState == WizardArena`.

### Task 3: Mouse Interaction System
- [ ] Update `setup_arcane_scene` in `scene.rs` to add `RelativeCursorPosition` to the `MapPanelCard` entity.
- [ ] Implement `mouse_spawning_system` in `spawner.rs`.
- [ ] System logic:
    - Check for `MouseButton::Left` just pressed.
    - Validate cursor is within `MapPanelCard`.
    - Calculate grid coordinates using font size and viewport offsets.
    - Use `ResMut<FrontendRuntime>` to spawn the selected entity at the target position.

### Task 4: Integration
- [ ] Register `SpawnerState` and new systems in `ArcaneCartographerPlugin` (`presentation/mod.rs`).
- [ ] Add a keybinding (e.g., `F7`) to toggle the spawner UI manually (optional, but good for UX).

## Verification
- [ ] Spawner panel appears on the right when entering Wizard Arena.
- [ ] Selecting different monsters/items updates the resource state.
- [ ] Left-clicking on a map tile spawns the correct entity.
- [ ] Spawner UI does not obstruct map clicks or standard keyboard input.
