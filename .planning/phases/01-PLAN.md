# Plan: Phase 1 - Foundation & The Arena

## Goal
Establish a dedicated, isolated test environment with a defined 50x50 map structure.

## Tasks

### Task 1: Core Infrastructure
- [ ] Add `WizardArena` variant to `AppState` in `crates/omega-bevy/src/lib.rs`.
- [ ] Add `WizardArena` variant to `AppState` equivalents (if applicable) in `crates/omega-tui/src/lib.rs`.
- [ ] Define `bootstrap_wizard_arena` in `crates/omega-content/src/lib.rs` to generate a 50x50 map with Grass, Stone, Water, and Walls.

### Task 2: Frontend Integration (TUI)
- [ ] Update `crates/omega-tui/src/lib.rs` to handle `AppState::WizardArena` (or equivalent).
- [ ] Add \"W\" keybinding to the TUI main menu to launch the Wizard Arena.
- [ ] Ensure \"Esc\" or \"Q\" returns to the main menu and clears the session.

### Task 3: Frontend Integration (Bevy)
- [ ] Update `crates/omega-bevy/src/lib.rs` to handle `AppState::WizardArena`.
- [ ] Add \"W\" keybinding to the Bevy main menu to launch the Wizard Arena.
- [ ] Ensure \"Esc\" returns to the main menu and clears the session.

### Task 4: Rendering & Cleanup
- [ ] Verify that `TileKind` and color mapping correctly handle the Arena terrain (Grass, Stone, Water, Walls).
- [ ] Implement explicit cleanup logic when exiting the Arena state to ensure NFR-ISOL-01.

## Verification
- [ ] `cargo test` passes for new bootstrap logic.
- [ ] Manual verification: Enter Arena from menu, see 50x50 map, exit Arena, verify return to menu.
