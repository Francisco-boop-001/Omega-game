# Research: Phase 1 - Foundation & The Arena

## Current State Analysis
- The codebase uses a `GameState` struct in `omega-core` to manage the game world.
- Frontends (`omega-bevy`, `omega-tui`) use an `AppState` enum to manage high-level transitions.
- `omega-content` provides bootstrapping logic to initialize `GameState` from legacy map files.
- `LegacyEnvironment` already includes an `Arena` variant.
- Rendering logic in both frontends maps map glyphs to visual tiles.

## Proposed Implementation Strategy
1. **Isolated State**: Add `WizardArena` to `AppState` in both `omega-bevy` and `omega-tui`. This ensures NFR-ISOL-01 by separating the test chamber from standard gameplay states.
2. **Arena Generation**: Implement a `bootstrap_wizard_arena()` function in `omega-content` that:
    - Creates a `GameState` with `MapBounds { width: 50, height: 50 }`.
    - Generates a grid with Grass (`\"`), Stone (`.`), and Water (`~`) tiles.
    - Places sparse Walls (`#`).
    - Sets `environment` to `LegacyEnvironment::Arena`.
3. **Frontend Integration**:
    - Update `App::handle_key` in `omega-tui` and `BevyFrontend::apply_action` in `omega-bevy` to handle the new state.
    - Add a menu option (e.g., press \"W\" in main menu) to launch the Arena.
4. **Cleanup**: Ensure that transitioning back to `Menu` from `WizardArena` drops the arena session, fulfilling the memory clearance requirement.

## Verification Plan
- Unit test for `bootstrap_wizard_arena()` to verify dimensions and terrain composition.
- Integration test for state transition and cleanup.
- Visual verification of rendering in both TUI and Bevy.
