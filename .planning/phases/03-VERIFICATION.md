# Verification: Phase 3 - Inspection & Manipulation

## Success Criteria Checklist
- [ ] REQ-SPW-03: Right-clicking an entity opens its property inspector.
- [ ] REQ-DBG-01: "Clear All" button functional.
- [ ] REQ-DBG-03: Toggle for "AI Pause" functional.

## Manual Verification Steps
1. Launch Bevy app and enter "Wizard Arena".
2. Spawn 3 goblins and 2 items.
3. Check "Pause AI" in the right panel.
4. Move around with the player; verify goblins do NOT move.
5. Uncheck "Pause AI"; verify goblins start moving again.
6. Right-click a goblin; verify a window appears with "Goblin" and current HP.
7. Click "Despawn" in the goblin inspector; verify that specific goblin disappears.
8. Click "Clear Monsters"; verify all remaining goblins disappear.
9. Click "Clear Items"; verify all items disappear.

## Automated Tests
- [ ] `test_ai_pause_skips_monster_turn`: Unit test in `omega-core` verifying `step` doesn't call `run_monster_turn` when flag is set.
