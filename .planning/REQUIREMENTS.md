# Requirements: Omega Elemental Systems

## v1 Requirements

### Core Elemental Simulation (ELE)
- **ELE-01:** Implement a double-buffered Cellular Automata (CA) grid for elemental states.
- **ELE-02:** Support Core States: Heat (Temperature), Wet (Saturation), and Pressure (Density).
- **ELE-03:** Implement state transition logic (e.g., solid -> liquid -> gas).
- **ELE-04:** Implement reaction logic: Fire + Water = Steam, Water + Earth = Mud.

### Projectile Systems (PROJ)
- **PROJ-01:** Implement ECS-based projectile entities with lifecycle management.
- **PROJ-02:** Support Arc/Lob trajectories with visual Z-height (Y-offset).
- **PROJ-03:** Support Beam/Instant trajectories using Bresenham's algorithm.
- **PROJ-04:** Projectile-to-Grid interaction: Projectiles modify CA cell values on impact.

### Visual Effects (VFX)
- **VFX-01:** Implement a glyph-based particle system for TUI rendering.
- **VFX-02:** Implement particle trailing for moving projectiles.
- **VFX-03:** Implement explosion/burst effects for elemental events.

### Environmental Interaction (ENV)
- **ENV-01:** Implement Fire Spread logic using CA rules.
- **ENV-02:** Implement Liquid Flow (Water/Mud pooling) using CA rules.
- **ENV-03:** Implement Gas Diffusion (Steam rising/dissipating).

### Integration & Testing (TEST)
- **TEST-01:** Create a "Wizard's Arena" test scene for sandbox interaction.
- **TEST-02:** Implement performance monitoring for CA grid updates.
- **TEST-03:** Stress test: 100+ projectiles and 128x128 grid at 60 FPS.

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| ELE-01 | Phase 1 | Pending |
| ELE-02 | Phase 1 | Pending |
| ELE-03 | Phase 1 | Pending |
| ELE-04 | Phase 1 | Pending |
| PROJ-01 | Phase 2 | Pending |
| PROJ-02 | Phase 2 | Pending |
| PROJ-03 | Phase 2 | Pending |
| PROJ-04 | Phase 2 | Pending |
| VFX-01 | Phase 3 | Pending |
| VFX-02 | Phase 3 | Pending |
| VFX-03 | Phase 3 | Pending |
| ENV-01 | Phase 4 | Pending |
| ENV-02 | Phase 4 | Pending |
| ENV-03 | Phase 4 | Pending |
| TEST-01 | Phase 5 | Pending |
| TEST-02 | Phase 5 | Pending |
| TEST-03 | Phase 5 | Pending |
