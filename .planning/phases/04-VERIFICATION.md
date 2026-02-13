# Verification: Phase 4 - Environmental Hazards (Fire)

## Success Criteria Checklist
- [ ] REQ-DBG-02: "Spawn Fire" brush is functional and ignites tiles.
- [ ] Fire spreads to adjacent Grass tiles.
- [ ] Fire is blocked by Water and Stone.
- [ ] NFR-PERF-01: Stable 60 FPS with 100+ entities and active fire.

## Manual Verification Steps
1. Launch Bevy app and enter "Wizard Arena".
2. Select "Hazards" -> "Fire".
3. Click a Grass tile; verify it starts burning (flickering or specific glyph).
4. Wait or move around to advance turns; verify the fire spreads to neighboring grass.
5. Place Water or Stone walls; verify the fire does not spread past them.
6. Spawn a large number of monsters (e.g., 50+) and start a fire; monitor FPS.
7. Verify that fire eventually burns out, leaving behind a modified tile.

## Automated Tests
- [ ] `test_fire_propagation`: Unit test in `omega-core` verifying that a burning tile spreads to grass neighbors but not stone/water.
