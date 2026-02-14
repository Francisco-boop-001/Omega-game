# Roadmap: Omega Elemental Systems

## Overview
This roadmap outlines the implementation of a Cellular Automata based elemental system integrated with Bevy's ECS. The project moves from core simulation to projectile mechanics, visual feedback, environmental complexity, and finally performance validation.

## Phases

### Phase 1: Core Elemental State & Reaction System
**Goal:** Establish the simulation foundation for elemental properties and their basic interactions.
- **Dependencies:** None
- **Requirements:** ELE-01, ELE-02, ELE-03, ELE-04
- **Plans:** 4 plans
  - [ ] 01-01-PLAN.md — Cell data model, CaGrid double-buffer, neighborhood utilities
  - [ ] 01-02-PLAN.md — State transitions with flash points, reaction logic (TDD)
  - [ ] 01-03-PLAN.md — Wind force map, explosive displacement, decay/Nature Reclaims
  - [ ] 01-04-PLAN.md — Bevy SimulationPlugin with FixedUpdate at 64Hz
- **Success Criteria:**
  - Simulation grid exists as a Bevy Resource and updates independently of frame rate.
  - Cells can hold Heat, Wet, and Pressure values.
  - Applying Heat to a Wet cell produces a Steam cell state.
  - Applying Water to an Earth cell produces a Mud cell state.

### Phase 2: Projectile ECS & Trajectory Physics
**Goal:** Enable users to project elements into the world with realistic physical behaviors.
- **Dependencies:** Phase 1
- **Requirements:** PROJ-01, PROJ-02, PROJ-03, PROJ-04
- **Success Criteria:**
  - Projectiles can be spawned as ECS entities with distinct "lob" or "beam" behaviors.
  - Lobbed projectiles follow a parabolic path using a visual Y-offset to simulate Z-height.
  - Beam projectiles instantly calculate their path using Bresenham's algorithm.
  - Projectile impacts modify the local CA grid (e.g., a Fireball increases Heat in target cells).

### Phase 3: Visual FX System
**Goal:** Provide high-fidelity visual feedback for elemental actions using glyph-based particles.
- **Dependencies:** Phase 2
- **Requirements:** VFX-01, VFX-02, VFX-03
- **Success Criteria:**
  - A performant particle system renders glyphs at projectile locations.
  - Projectiles leave distinct trails (e.g., smoke for fire, droplets for water).
  - Impact events trigger explosion patterns of glyph particles.

### Phase 4: Environmental Interaction
**Goal:** Create emergent environmental behaviors where elements spread and flow.
- **Dependencies:** Phase 1, Phase 3
- **Requirements:** ENV-01, ENV-02, ENV-03
- **Plans:** 2 plans
  - [ ] 04-01-PLAN.md — Liquid flow, gas rise/dissipation, fire spread bias (TDD)
  - [ ] 04-02-PLAN.md — Bevy FixedUpdate pipeline integration
- **Success Criteria:**
  - Fire spreads to adjacent flammable cells based on Heat and combustible CA rules.
  - Liquids (Water/Mud) flow "downward" into empty or lower-pressure cells.
  - Steam (Gas) rises vertically and dissipates or pools against "ceilings".

### Phase 5: Wizard's Arena Integration & Stress Test
**Goal:** Validate system stability, performance, and fun in a controlled sandbox environment.
- **Dependencies:** Phase 4
- **Requirements:** TEST-01, TEST-02, TEST-03
- **Success Criteria:**
  - "Wizard's Arena" scene allows testing all elemental combinations via user input.
  - Simulation maintains 60 FPS with 100+ active projectiles.
  - 128x128 CA grid updates occur within a <2ms frame budget.

## Progress Table

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Core Elemental State & Reaction System | Pending |
| 2 | Projectile ECS & Trajectory Physics | Pending |
| 3 | Visual FX System | Pending |
| 4 | Environmental Interaction | Pending |
| 5 | Wizard's Arena Integration & Stress Test | Pending |
