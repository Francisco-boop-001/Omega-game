# Requirements: Wizard's Arena

## Functional Requirements
### 1. Environment & Navigation
- **REQ-ENV-01:** Implement a fixed 50x50 "Arena" map. (Phase 1)
- **REQ-ENV-02:** Terrain must include `Grass` (flammable), `Stone` (stable), and `Water` (fire-resistant). (Phase 1)
- **REQ-ENV-03:** Sparse `Wall` entities representing ruined buildings. (Phase 1)

### 2. Spawning System
- **REQ-SPW-01:** An egui side-panel menu to select entity types (Monster, Item, Hazard). (Phase 2)
- **REQ-SPW-02:** Click-to-spawn: Left-clicking a tile in the arena spawns the selected entity. (Phase 2)
- **REQ-SPW-03:** Right-clicking an entity opens its property inspector. (Phase 3)

### 3. Debug Tools
- **REQ-DBG-01:** "Clear All" button to despawn all test entities. (Phase 3)
- **REQ-DBG-02:** "Spawn Fire" brush to manually ignite tiles. (Phase 4)
- **REQ-DBG-03:** Toggle for "AI Pause" to observe monsters without them attacking. (Phase 3)

## Non-Functional Requirements
- **NFR-PERF-01:** Spawning 100+ entities should not drop FPS below 60. (Phase 4)
- **NFR-ISOL-01:** No entities or state from the Arena should leak into the main `InGame` state. (Phase 1)
- **NFR-UI-01:** The spawn menu must be responsive and not block the map view. (Phase 2)

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| REQ-ENV-01 | Phase 1 | Pending |
| REQ-ENV-02 | Phase 1 | Pending |
| REQ-ENV-03 | Phase 1 | Pending |
| REQ-SPW-01 | Phase 2 | Pending |
| REQ-SPW-02 | Phase 2 | Pending |
| REQ-SPW-03 | Phase 3 | Pending |
| REQ-DBG-01 | Phase 3 | Pending |
| REQ-DBG-02 | Phase 4 | Pending |
| REQ-DBG-03 | Phase 3 | Pending |
| NFR-PERF-01 | Phase 4 | Pending |
| NFR-ISOL-01 | Phase 1 | Pending |
| NFR-UI-01 | Phase 2 | Pending |
