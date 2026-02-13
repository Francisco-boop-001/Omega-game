# Research: Phase 2 - Spawner Interface

## Current State Analysis
- `bevy_egui` is integrated and used for the Theme Editor.
- UI layout uses a `Node` based system with `MapPanelCard` as the map container.
- Map rendering is character-based using `TextSpan` children.
- Input handling is currently keyboard-only.
- `omega-core` provides `spawn_monster` and `place_item` methods.

## Technical Strategy
1. **Selection Resource**: A `SpawnerState` resource will track what the user wants to spawn.
2. **Side Panel**: `egui::SidePanel` is ideal for NFR-UI-01 as it automatically adjusts the central UI area.
3. **Mouse Translation**: 
    - Use `bevy_ui::RelativeCursorPosition` on the `MapPanelCard`.
    - Translate relative pixels to grid coordinates:
        - `grid_x = viewport_start_x + (relative_px_x / font_width)`
        - `grid_y = viewport_start_y + (relative_px_y / font_height)`
    - Viewport offsets come from `centered_view_window` logic.
4. **Direct State Injection**: For the Wizard Arena, we will directly modify the `GameState` within the Bevy frontend to spawn entities, bypassing the standard command loop for immediate feedback.

## Spawning Data
- **Monsters**: "rat", "goblin", "orc", "wolf".
- **Items**: "practice blade", "wooden shield", "healing potion".
- **Hazards**: "fire trap" (using existing `Trap` system).

## Challenges
- **Font Metrics**: Ensuring accurate mapping from pixels to characters. Bevy UI text doesn't always have a 1:1 pixel-to-glyph mapping that is easy to query. We may need to assume a standard character size based on `UiLayoutTokens`.
