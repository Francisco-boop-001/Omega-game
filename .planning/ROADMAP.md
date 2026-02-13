# Roadmap: Wizard's Arena

## Overview
This roadmap outlines the development of the Wizard's Arena, a specialized "Test Chamber" for Omega 0.90. It progresses from foundational environment setup through interactive spawning and debugging tools, concluding with complex environmental simulation.

---

## Phase 1: Foundation & The Arena
**Goal:** Establish a dedicated, isolated test environment with a defined map structure.

### Requirements
- REQ-ENV-01: Implement a fixed 50x50 "Arena" map.
- REQ-ENV-02: Terrain must include Grass (flammable), Stone (stable), and Water (fire-resistant).
- REQ-ENV-03: Sparse Wall entities representing ruined buildings.
- NFR-ISOL-01: No entities or state from the Arena should leak into the main InGame state.

### Success Criteria
1. User can transition from the main menu to a dedicated "Test Chamber" state.
2. A 50x50 grid is rendered containing distinct Grass, Stone, and Water tiles.
3. Walls are correctly positioned and block movement/spawning.
4. Exiting the Arena completely clears all test-specific entities and resources from memory.

---

## Phase 2: Spawner Interface
**Goal:** Provide a user-friendly UI for placing entities into the arena.

### Requirements
- REQ-SPW-01: An egui side-panel menu to select entity types (Monster, Item, Hazard).
- REQ-SPW-02: Click-to-spawn: Left-clicking a tile in the arena spawns the selected entity.
- NFR-UI-01: The spawn menu must be responsive and not block the map view.

### Success Criteria
1. An egui side-panel is visible when in the Test Chamber state.
2. User can select categories (Monster, Item) and specific types within those categories.
3. Left-clicking a valid tile places the currently selected entity type instantly.
4. The UI remains pinned to the side, allowing unobstructed view of the central arena.

---

## Phase 3: Inspection & Manipulation
**Goal:** Enable real-time monitoring and control over the spawned entities.

### Requirements
- REQ-SPW-03: Right-clicking an entity opens its property inspector.
- REQ-DBG-01: "Clear All" button to despawn all test entities.
- REQ-DBG-03: Toggle for "AI Pause" to observe monsters without them attacking.

### Success Criteria
1. Right-clicking a monster opens an inspector window displaying its current Health, AI state, and position.
2. Toggling "AI Pause" immediately halts all monster AI logic, freezing them in place.
3. Clicking "Clear All" returns the arena to its baseline state (terrain and walls only).

---

## Phase 4: Environmental Hazards (Fire)
**Goal:** Implement and optimize the fire propagation system for stress testing.

### Requirements
- REQ-DBG-02: "Spawn Fire" brush to manually ignite tiles.
- NFR-PERF-01: Spawning 100+ entities should not drop FPS below 60.

### Success Criteria
1. User can select a "Fire Brush" and click tiles to ignite them.
2. Fire correctly spreads to adjacent Grass tiles while being blocked by Stone/Water.
3. The game maintains 60 FPS during a large-scale fire involving 100+ entities and active propagation.

---

## Progress Table

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Foundation & The Arena | Completed |
| 2 | Spawner Interface | Completed |
| 3 | Inspection & Manipulation | Completed |
| 4 | Environmental Hazards (Fire) | Completed |
