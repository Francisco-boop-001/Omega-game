# Verification: Phase 2 - Spawner Interface

## Success Criteria Checklist
- [ ] REQ-SPW-01: An egui side-panel menu is functional and contains categories.
- [ ] REQ-SPW-02: Left-clicking a map tile spawns the selected entity at that position.
- [ ] NFR-UI-01: The menu is responsive and does not block the map view (using SidePanel).

## Manual Verification Steps
1. Launch Bevy app and enter "Wizard Arena".
2. Observe spawner panel on the right.
3. Select "Monster" -> "Goblin".
4. Left-click an empty tile in the arena.
5. Verify a 'g' (or 'm') glyph appears at that position.
6. Select "Item" -> "Practice Blade".
7. Click another tile and verify an '!' glyph appears.
8. Verify that keyboard movement still works while the panel is open.

## Automated Tests
- [ ] `test_coordinate_translation`: Verify that relative pixels map to correct grid coordinates.
