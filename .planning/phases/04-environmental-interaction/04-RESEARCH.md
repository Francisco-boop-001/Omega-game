# Phase 4: Environmental Interaction - Research

**Researched:** 2026-02-13
**Domain:** Cellular Automata Environmental Behaviors (Fire Spread, Liquid Flow, Gas Diffusion)
**Confidence:** MEDIUM-HIGH

## Summary

Phase 4 builds on Phase 1's CA infrastructure to implement emergent environmental behaviors: fire spreading to adjacent combustible cells, liquids flowing downward/pooling, and gases rising vertically. The research reveals three distinct CA movement patterns, each with different update strategies and neighbor-checking rules.

Fire spread uses Moore neighborhood (8 neighbors) with heat diffusion and ignition thresholds already implemented in Phase 1's `apply_reactions`. Liquid flow requires directional gravity-based rules (check down first, then diagonal down, then horizontal spread) with settling behavior. Gas diffusion needs upward bias (inverse gravity) with dispersal and ceiling detection.

The critical challenge is **update order artifacts**: naive top-to-bottom scanning causes liquids to "teleport" downward and gases to instantly rise. The standard solution is **bottom-up updates for falling materials** (liquids) and **top-down updates for rising materials** (gases), while fire spread can use standard raster order since heat diffusion is non-directional.

**Primary recommendation:** Implement three specialized CA passes in separate systems: (1) Fire spread using existing heat transfer with Moore neighborhood, (2) Liquid flow with bottom-up scan checking Von Neumann down/diagonal neighbors, (3) Gas rise with top-down scan checking upward neighbors. Use Phase 1's double-buffer infrastructure to avoid conflicts, run all three in FixedUpdate at 64Hz after the core reaction system.

## Standard Stack

### Core (Already Implemented in Phase 1)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Bevy | 0.15 | Game engine ECS | Industry standard for Rust game development, excellent fixed timestep support |
| bevy_ecs | 0.15 | Entity-component-system | Built into Bevy, provides Resource pattern for grid storage |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| None required | - | All needs met by Phase 1 | Phase 1's CaGrid, Cell, WindGrid, and neighborhood utilities already provide foundation |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Custom CA | bevy_life plugin | bevy_life is generic Game of Life, doesn't support multi-state cells or directional movement |
| FixedUpdate | Update schedule | FixedUpdate ensures consistent simulation rate independent of frame rate |
| Manual pass ordering | Parallel systems | Environmental behaviors have directional dependencies requiring explicit ordering |

**Installation:**
No new dependencies required. Phase 4 extends Phase 1's existing infrastructure.

## Architecture Patterns

### Recommended System Structure
```
crates/omega-core/src/simulation/
├── cell.rs              # Already implemented: Cell with solid/liquid/gas
├── grid.rs              # Already implemented: CaGrid with double-buffer
├── state.rs             # Already implemented: Material enums
├── reactions.rs         # Already implemented: heat diffusion, fire spread
├── transitions.rs       # Already implemented: evaporation, combustion
├── neighborhood.rs      # Already implemented: Moore/Von Neumann utilities
├── environmental.rs     # NEW: Fire/liquid/gas movement rules
└── mod.rs               # Export environmental

crates/omega-bevy/src/simulation/
├── systems.rs           # UPDATE: Add environmental_behavior_system
└── plugin.rs            # UPDATE: Schedule environmental system in FixedUpdate
```

### Pattern 1: Directional Update Order (Critical)
**What:** Scan direction must match material movement direction to avoid teleportation artifacts.
**When to use:** Any CA with directional bias (gravity, buoyancy, flow).
**Example:**
```rust
// Liquid flow: bottom-up scan prevents multi-cell fall per frame
pub fn apply_liquid_flow(grid: &mut CaGrid) {
    let (w, h) = (grid.width(), grid.height());

    // CRITICAL: Reverse iteration for downward movement
    for y in (0..h).rev() {
        for x in 0..w {
            let cell = *grid.get(x, y);
            if let Some(liquid) = cell.liquid {
                let next = try_flow_down(grid, x, y, &cell, liquid);
                grid.set(x, y, next);
            }
        }
    }
}

// Gas rise: top-down scan for upward movement
pub fn apply_gas_rise(grid: &mut CaGrid) {
    let (w, h) = (grid.width(), grid.height());

    // Forward iteration for upward movement
    for y in 0..h {
        for x in 0..w {
            let cell = *grid.get(x, y);
            if let Some(gas) = cell.gas {
                let next = try_rise_up(grid, x, y, &cell, gas);
                grid.set(x, y, next);
            }
        }
    }
}
```

### Pattern 2: Neighbor Priority Checking
**What:** Check neighbors in priority order, move to first valid destination.
**When to use:** Liquid flow (down > diagonal down > horizontal), gas rise (up > diagonal up).
**Example:**
```rust
// Liquid flow priorities from falling sand games
pub fn try_flow_down(grid: &CaGrid, x: usize, y: usize, cell: &Cell, liquid: Liquid) -> Cell {
    // Priority 1: Directly down
    if can_flow_to(grid, x, y + 1) {
        return move_liquid_to(cell, Direction::Down);
    }

    // Priority 2: Diagonal down (randomize left/right to avoid bias)
    let diag = if rand::random() {
        [(x - 1, y + 1), (x + 1, y + 1)]
    } else {
        [(x + 1, y + 1), (x - 1, y + 1)]
    };

    for (nx, ny) in diag {
        if grid.in_bounds(nx as isize, ny as isize) && can_flow_to(grid, nx, ny) {
            return move_liquid_to(cell, Direction::Diagonal);
        }
    }

    // Priority 3: Horizontal spread (equalize pressure)
    for (nx, ny) in [(x - 1, y), (x + 1, y)] {
        if grid.in_bounds(nx as isize, ny as isize) && can_spread_to(grid, nx, ny) {
            return spread_liquid_to(cell, Direction::Horizontal);
        }
    }

    // No movement possible: pooled/settled
    *cell
}
```

### Pattern 3: State Transfer vs State Copy
**What:** Distinguish between moving material (flow) vs spreading influence (heat diffusion).
**When to use:** Flow removes source, diffusion preserves source at reduced intensity.
**Example:**
```rust
// Flow: Transfer entire liquid from source to dest
pub fn move_liquid_to(cell: &Cell, dir: Direction) -> Cell {
    let mut next = *cell;
    next.liquid = None;  // Remove from source
    // Destination set in separate pass
    next
}

// Diffusion: Spread heat to neighbors, source retains partial heat
pub fn diffuse_heat(cell: &Cell, neighbors: &[Cell; 8]) -> Cell {
    let mut next = *cell;
    let avg_heat = avg_neighbor_heat(neighbors);

    // Gradual equalization (already implemented in Phase 1)
    let delta = (avg_heat as i16 - next.heat as i16) as f32 * 0.1;
    if delta > 0.0 {
        next.heat = next.heat.saturating_add(delta as u8);
    }

    next  // Source still has heat
}
```

### Pattern 4: Fire Spread via Heat Accumulation
**What:** Fire doesn't "move" - it ignites new cells when accumulated heat exceeds flash point.
**When to use:** Already implemented in Phase 1's `apply_reactions`.
**Example:**
```rust
// From Phase 1: Fire spreads by heating neighbors above flash point
pub fn apply_reactions(cell: &Cell, neighbors: &[Cell; 8]) -> Cell {
    let mut next = *cell;

    // Count burning neighbors (Moore neighborhood)
    if next.can_ignite() {
        let burning_count = count_burning_neighbors(neighbors);
        if burning_count > 0 {
            // Heat accumulates until flash point reached
            next.heat = next.heat.saturating_add(burning_count * 20);
        }
    }

    // Flash point check in apply_transitions
    next
}
```

### Anti-Patterns to Avoid
- **Top-down liquid scan:** Causes downward teleportation - liquid at row 0 reaches row 50 in one frame.
- **Bottom-up gas scan:** Causes upward teleportation - gas instantly reaches ceiling.
- **Synchronous neighbor movement:** Multiple cells trying to flow into same destination causes conflicts. Use double-buffer: read from front, write to back, swap after full pass.
- **Ignoring waterlogged state:** Phase 1 already implements fire immunity when wet >= 255. Don't duplicate this logic.
- **Moving solid combustibles:** Fire spreads via heat transfer, not by moving burning materials. Solids stay put (except Ash blown by wind, already implemented in Phase 1's `apply_wind`).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Neighbor iteration | Custom 8-way loop | Phase 1's `moore_neighbors()` / `von_neumann_neighbors()` | Already tested, handles edge cases |
| Double buffering | Custom swap logic | Phase 1's `CaGrid::swap_buffers()` | Prevents race conditions, copy-on-swap |
| Heat diffusion | New fire spread logic | Phase 1's `apply_reactions()` | Fire spread already implemented via heat transfer |
| Wind effects | Directional gas push | Phase 1's `apply_wind()` | Wind vector map already displaces gas/heat |
| Flash point checks | New combustion system | Phase 1's `Solid::flash_point()` + `apply_transitions()` | Thresholds already configured (Grass 120, Wood 180, Stone 250) |
| Explosive displacement | Radial gas spread | Phase 1's `apply_explosive_displacement()` | BFS queue handles radius-based pressure/gas push |

**Key insight:** Phase 1 already implements fire spread mechanics (ENV-01 partially complete). Phase 4 focuses on **liquid flow** (ENV-02) and **gas diffusion** (ENV-03), plus refining fire spread to use directional neighbor checks if needed.

## Common Pitfalls

### Pitfall 1: Update Order Teleportation
**What goes wrong:** Liquids fall 50 cells per frame instead of 1 cell per frame.
**Why it happens:** Top-down scan processes row 0, moves liquid to row 1, then processes row 1 (which now has the liquid), moves it to row 2, etc. Same liquid moves multiple times in one frame.
**How to avoid:** Bottom-up iteration for falling materials, top-down for rising materials.
**Warning signs:** Liquids reach bottom instantly, gaps appear in continuous liquid columns.

### Pitfall 2: Liquid Piling Instead of Spreading
**What goes wrong:** Water forms vertical pillars instead of pooling horizontally.
**Why it happens:** Only checking down, not checking horizontal spread to equalize levels.
**How to avoid:** After down/diagonal checks fail, check horizontal neighbors. If neighbor has less liquid (lower pressure), spread horizontally to equalize.
**Warning signs:** Water doesn't fill containers uniformly, creates stalagmite shapes.

### Pitfall 3: Gas Trapped Under Ceilings
**What goes wrong:** Steam doesn't spread sideways when it hits a solid ceiling.
**Why it happens:** Only checking upward movement, no lateral diffusion when blocked.
**How to avoid:** When upward movement blocked by solid, add horizontal spread rules (similar to liquid horizontal spread but for gas).
**Warning signs:** Gas accumulates in single column under ceiling instead of spreading along it.

### Pitfall 4: Ignoring Phase 1's State Transitions
**What goes wrong:** Water doesn't turn to steam at high heat, steam doesn't condense.
**Why it happens:** Forgetting Phase 1's `apply_transitions()` already handles Water -> Steam (heat > 200) and Steam -> Water (heat < 180).
**How to avoid:** Run environmental behaviors AFTER core transitions in system ordering. Don't implement duplicate transition logic.
**Warning signs:** Water sits in fire without boiling, steam never condenses.

### Pitfall 5: Determinism Breaks in Horizontal Spread
**What goes wrong:** Liquid/gas spreads create left/right bias patterns.
**Why it happens:** Always checking left neighbor before right (or vice versa) creates predictable directional bias.
**How to avoid:** Randomize horizontal check order, or alternate left-first/right-first by frame parity, or use both simultaneously with partial transfer.
**Warning signs:** All liquids/gases gradually drift to one side of containers.

### Pitfall 6: Double-Buffer Confusion
**What goes wrong:** Reading back buffer instead of front buffer, or setting front instead of back.
**Why it happens:** Phase 1's CaGrid uses `get()` reads front, `set()` writes back. Must call `swap_buffers()` after full pass.
**How to avoid:** Never mutate front buffer. Read front, compute next state, write back, swap at end of frame. Phase 1's `update_ca_cells` system already does this correctly - follow same pattern.
**Warning signs:** CA updates don't appear on screen, or appear with 1-frame delay, or cells flicker.

## Code Examples

Verified patterns based on Phase 1 implementation and falling sand research:

### Liquid Flow (Bottom-Up Scan)
```rust
// Source: Noita falling sand techniques, W-Shadow fluid simulation
use omega_core::simulation::{grid::CaGrid, cell::Cell, state::Liquid};

pub fn apply_liquid_flow(grid: &mut CaGrid) {
    let (w, h) = (grid.width(), grid.height());

    // Bottom-up: prevents multi-cell fall per frame
    for y in (0..h).rev() {
        for x in 0..w {
            let cell = *grid.get(x, y);

            if let Some(liquid) = cell.liquid {
                let next = compute_liquid_flow(grid, x, y, &cell, liquid);
                grid.set(x, y, next);
            }
        }
    }
}

fn compute_liquid_flow(grid: &CaGrid, x: usize, y: usize, cell: &Cell, liquid: Liquid) -> Cell {
    // Priority 1: Down (Von Neumann)
    if y + 1 < grid.height() {
        let below = grid.get(x, y + 1);
        if below.liquid.is_none() && below.solid.is_none() {
            let mut next = *cell;
            next.liquid = None;  // Move liquid down

            // Write destination (below cell updated in its own iteration)
            let mut dest = *below;
            dest.liquid = Some(liquid);
            grid.set(x, y + 1, dest);

            return next;
        }
    }

    // Priority 2: Diagonal down (randomize to avoid bias)
    let diag = if (x + y) % 2 == 0 {
        [(x.wrapping_sub(1), y + 1), (x + 1, y + 1)]
    } else {
        [(x + 1, y + 1), (x.wrapping_sub(1), y + 1)]
    };

    for (nx, ny) in diag {
        if grid.in_bounds(nx as isize, ny as isize) {
            let target = grid.get(nx, ny);
            if target.liquid.is_none() && target.solid.is_none() {
                let mut next = *cell;
                next.liquid = None;

                let mut dest = *target;
                dest.liquid = Some(liquid);
                grid.set(nx, ny, dest);

                return next;
            }
        }
    }

    // Priority 3: Horizontal spread (equalize)
    for (nx, ny) in [(x.wrapping_sub(1), y), (x + 1, y)] {
        if grid.in_bounds(nx as isize, ny as isize) {
            let target = grid.get(nx, ny);
            // Spread if neighbor is empty or has less liquid
            if target.liquid.is_none() && target.solid.is_none() {
                // Partial transfer for spreading behavior
                let mut next = *cell;
                next.wet = next.wet.saturating_sub(50);  // Reduce source moisture

                let mut dest = *target;
                dest.liquid = Some(liquid);
                dest.wet = 100;  // Partial amount in dest
                grid.set(nx, ny, dest);

                if next.wet < 100 {
                    next.liquid = None;  // Fully transferred
                }
                return next;
            }
        }
    }

    // No movement: pooled
    *cell
}
```

### Gas Rise (Top-Down Scan)
```rust
// Source: Cellular Automata for Physical Modelling (gas buoyancy)
use omega_core::simulation::{grid::CaGrid, cell::Cell, state::Gas};

pub fn apply_gas_rise(grid: &mut CaGrid) {
    let (w, h) = (grid.width(), grid.height());

    // Top-down: prevents multi-cell rise per frame
    for y in 0..h {
        for x in 0..w {
            let cell = *grid.get(x, y);

            if let Some(gas) = cell.gas {
                // Steam/Smoke rise, Fire stays (convection handled by heat)
                if matches!(gas, Gas::Steam | Gas::Smoke) {
                    let next = compute_gas_rise(grid, x, y, &cell, gas);
                    grid.set(x, y, next);
                }
            }
        }
    }
}

fn compute_gas_rise(grid: &CaGrid, x: usize, y: usize, cell: &Cell, gas: Gas) -> Cell {
    // Can't rise above top edge
    if y == 0 {
        // Dissipate at ceiling (or spread horizontally)
        return apply_gas_dissipation(grid, x, y, cell, gas);
    }

    // Priority 1: Directly up
    let above = grid.get(x, y - 1);
    if above.gas.is_none() && above.solid.is_none() {
        let mut next = *cell;
        next.gas = None;  // Remove from current

        let mut dest = *above;
        dest.gas = Some(gas);
        grid.set(x, y - 1, dest);

        return next;
    }

    // Priority 2: Diagonal up (randomize)
    let diag = if (x + y) % 2 == 0 {
        [(x.wrapping_sub(1), y - 1), (x + 1, y - 1)]
    } else {
        [(x + 1, y - 1), (x.wrapping_sub(1), y - 1)]
    };

    for (nx, ny) in diag {
        if grid.in_bounds(nx as isize, ny as isize) {
            let target = grid.get(nx, ny);
            if target.gas.is_none() && target.solid.is_none() {
                let mut next = *cell;
                next.gas = None;

                let mut dest = *target;
                dest.gas = Some(gas);
                grid.set(nx, ny, dest);

                return next;
            }
        }
    }

    // Blocked: dissipate or spread sideways
    apply_gas_dissipation(grid, x, y, cell, gas)
}

fn apply_gas_dissipation(grid: &CaGrid, x: usize, y: usize, cell: &Cell, gas: Gas) -> Cell {
    // When blocked vertically, spread horizontally or dissipate
    let mut next = *cell;

    // Spread to horizontal neighbors with reduced intensity
    for (nx, ny) in [(x.wrapping_sub(1), y), (x + 1, y)] {
        if grid.in_bounds(nx as isize, ny as isize) {
            let target = grid.get(nx, ny);
            if target.gas.is_none() && target.solid.is_none() {
                // Partial spread
                let mut dest = *target;
                dest.gas = Some(gas);
                dest.pressure = cell.pressure / 2;
                grid.set(nx, ny, dest);

                next.pressure = cell.pressure / 2;
                return next;
            }
        }
    }

    // No spread possible: gradual dissipation via Phase 1's decay
    next.pressure = next.pressure.saturating_sub(5);
    if next.pressure < 10 {
        next.gas = None;  // Fully dissipated
    }

    next
}
```

### Fire Spread (Already Implemented)
```rust
// Source: Phase 1 reactions.rs
// Fire spread uses existing heat diffusion + flash point checks
// No new code needed - ENV-01 requirement already satisfied

// Phase 1 already implements:
// 1. count_burning_neighbors() - Moore neighborhood (8 cells)
// 2. apply_reactions() - adds heat based on burning neighbors
// 3. apply_transitions() - checks flash point, ignites when heat exceeds threshold
// 4. Waterlogged immunity - can_ignite() returns false when wet >= 255

// If directional spread bias needed (fire spreads upward faster):
use omega_core::simulation::neighborhood::{moore_neighbors, MOORE_OFFSETS};

pub fn apply_directional_fire_spread(grid: &CaGrid, x: usize, y: usize, cell: &Cell) -> Cell {
    let neighbors = moore_neighbors(grid, x, y);
    let mut next = *cell;

    if next.can_ignite() {
        let mut heat_gain = 0u16;

        for (i, neighbor) in neighbors.iter().enumerate() {
            if matches!(neighbor.gas, Some(Gas::Fire)) {
                // MOORE_OFFSETS: [(-1,-1), (0,-1), (1,-1), (-1,0), (1,0), (-1,1), (0,1), (1,1)]
                let (dx, dy) = MOORE_OFFSETS[i];

                // Vertical bias: fire rises faster than it spreads sideways
                let multiplier = if dy < 0 { 30 } else if dy > 0 { 10 } else { 20 };
                heat_gain += multiplier;
            }
        }

        next.heat = next.heat.saturating_add(heat_gain as u8);
    }

    next
}
```

### System Ordering (Integration)
```rust
// Source: Phase 1 plugin.rs pattern
use bevy::prelude::*;
use omega_core::simulation::grid::CaGrid;

// In SimulationPlugin::build()
app.add_systems(
    FixedUpdate,
    (
        increment_tick,
        particle_physics_system,
        particle_wind_drift_system,
        particle_lifecycle_system,
        particle_visual_cascade_system,
        trail_emitter_system,
        explosion_emitter_system,
        projectile_movement_system,
        projectile_collision_system,
        projectile_interception_system,
        update_ca_cells,           // Phase 1: reactions, transitions, wind
        process_explosions,         // Phase 1: explosive displacement
        environmental_behaviors,    // Phase 4: NEW - fire/liquid/gas movement
        swap_ca_buffers,            // Phase 1: double-buffer swap
    )
        .chain(),  // Explicit ordering prevents parallelism conflicts
);

// Phase 4 system
pub fn environmental_behaviors(mut grid: ResMut<CaGrid>) {
    // Order matters: fire spread affects heat which affects liquids
    apply_fire_spread_refinements(&mut grid);  // Optional directional bias
    apply_liquid_flow(&mut grid);              // ENV-02: bottom-up scan
    apply_gas_rise(&mut grid);                 // ENV-03: top-down scan
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Single-pass CA | Multi-pass directional | Noita (2020), falling sand games | Prevents teleportation, enables realistic liquid/gas physics |
| Synchronous updates | Double-buffered swap | Standard since Conway's Life | Avoids race conditions, deterministic results |
| Random scan order | Directional scan (bottom-up/top-down) | Falling Turnip (2010s), Powder Game | Directional materials move 1 cell/frame, not entire column |
| Global grid updates | Dirty rect / chunk-based | Noita's 64x64 chunks (2019) | Only update active regions, massive performance gain |
| CPU-only CA | GPU compute shaders | Modern falling sand (2020+) | Parallel cell updates, millions of cells at 60+ fps |

**Deprecated/outdated:**
- **Synchronous single-buffer updates:** Causes race conditions when multiple cells try to move into same destination. Modern CA uses double-buffering.
- **Top-down scan for all materials:** Works for diffusion (heat, pressure) but breaks directional movement (liquids fall). Use scan direction matching movement direction.
- **Uniform update rate:** Some falling sand games use adaptive timestep where fast-moving materials get more frequent updates. Not needed for 64Hz fixed timestep.

## Open Questions

1. **Liquid Compression vs Displacement**
   - What we know: Phase 1 has pressure attribute (0-255), used for explosions
   - What's unclear: Should liquids increase pressure when compressed in tight spaces? Should high pressure force liquids upward (fountain effect)?
   - Recommendation: Start with incompressible liquids (can't stack vertically). Defer pressure-based fountains to later if needed.

2. **Gas Layer Stacking**
   - What we know: Phase 1 allows concurrent gas over liquid/solid
   - What's unclear: Can multiple gas types stack (Steam over Smoke)? How do they interact?
   - Recommendation: Single gas per cell (current Phase 1 implementation). Mixing handled by Phase 1's reactions (Fire + Steam -> extinguish).

3. **Fire Spread Through Gas**
   - What we know: Phase 1's Fire is a Gas enum variant
   - What's unclear: Should fire spread faster through flammable gas (Oil vapor)? Should gas itself burn (Oil vapor combustion)?
   - Recommendation: Defer gas combustion to later. Current ENV-01 focuses on solid combustibles (Grass, Wood, Stone).

4. **Liquid-Liquid Mixing**
   - What we know: Phase 1 supports Water and Oil as separate Liquid variants
   - What's unclear: Do they displace each other (Oil floats on Water)? Do they mix (emulsion)?
   - Recommendation: Single liquid per cell (simplifies flow). Displacement/layering deferred to later.

5. **Performance Optimization Threshold**
   - What we know: 64Hz fixed timestep, double-buffered grid
   - What's unclear: At what grid size do we need dirty rect optimization (Noita's 64x64 chunks)?
   - Recommendation: Start with full-grid updates. Profile with target grid size (likely 100x100 or 200x200 for TUI). Add spatial partitioning only if profiling shows <60fps.

6. **Gas Dissipation Rate**
   - What we know: Phase 1's apply_decay reduces pressure/heat gradually
   - What's unclear: Should Steam dissipate faster than Smoke? Should dissipation rate depend on ceiling proximity?
   - Recommendation: Uniform dissipation rate (existing decay system). Differentiate gas lifetimes via pressure attribute if needed.

## Sources

### Primary (HIGH confidence)
- Phase 1 CONTEXT.md - Locked decisions on CA architecture, double-buffering, material states
- Phase 1 RESEARCH.md - Bevy 0.15 ECS patterns, fixed timestep scheduling
- omega-core/src/simulation/*.rs - Existing Cell, CaGrid, reactions, transitions, neighborhood implementations
- omega-bevy/src/simulation/*.rs - SimulationPlugin with FixedUpdate at 64Hz, system ordering

### Secondary (MEDIUM confidence)
- [Noita: a Game Based on Falling Sand Simulation](https://80.lv/articles/noita-a-game-based-on-falling-sand-simulation) - Bottom-up updates, 64x64 chunks, dirty rect optimization
- [2D Liquid Simulator With Cellular Automaton in Unity](http://www.jgallant.com/2d-liquid-simulator-with-cellular-automaton-in-unity/) - Priority checking (down > diagonal > horizontal)
- [How To Make a "Falling Sand" Style Water Simulation](https://w-shadow.com/blog/2009/09/29/falling-sand-style-water-simulation/) - Liquid pooling, pressure equalization
- [Cellular Automata for Physical Modelling](https://tomforsyth1000.github.io/papers/cellular_automata_for_physical_modelling.html) - Gas buoyancy, heat convection, CA physics principles
- [Falling Sand Algorithm](https://smashwaredotcom.wordpress.com/2013/05/21/falling-sand-algorithm/) - Update order artifacts, bottom-up scanning solution
- [Bevy FixedUpdate Schedule](https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html) - Fixed timestep best practices, deterministic physics
- [Adaptive Forest Fire Spread Simulation Algorithm Based on Cellular Automata](https://www.mdpi.com/1999-4907/12/11/1431) - Moore neighborhood fire spread, adaptive time-stepping
- [Surface Water Flow simulation using cellular automata](https://ieeexplore.ieee.org/document/7892561/) - D-infinity algorithm, gravitational flow, ponding
- [Buoyant Mixtures of Cellular Automaton Gases](https://www.complex-systems.com/abstracts/v01_i01_a03/) - Lattice gas buoyancy, temperature effects on gas rise

### Tertiary (LOW confidence)
- [Cellular Automata Theory](https://www.fourmilab.ch/cellab/manual/chap4.html) - General CA synchronous/asynchronous update theory
- [Cellular Automata Optimization](https://cell-auto.com/optimisation/) - Spatial partitioning, chunk-based updates (no version dates)
- [bevy_life plugin](https://github.com/ManevilleF/bevy_life) - Generic CA plugin (not suitable for multi-state directional movement)

## Metadata

**Confidence breakdown:**
- Fire spread (ENV-01): HIGH - Already implemented in Phase 1's apply_reactions, only refinements needed
- Liquid flow (ENV-02): MEDIUM-HIGH - Well-documented falling sand pattern, verified by multiple sources (Noita, W-Shadow, falling sand algorithm posts)
- Gas diffusion (ENV-03): MEDIUM - Buoyancy principles clear, implementation details require testing (ceiling spread, dissipation rates)
- Performance optimization: MEDIUM - Double-buffer pattern solid, chunk optimization deferred until profiling confirms need
- System integration: HIGH - Phase 1's FixedUpdate pipeline provides clear insertion point, system ordering explicit via .chain()

**Research date:** 2026-02-13
**Valid until:** 2026-03-13 (30 days - stable domain, CA patterns unchanged for decades)

**Key uncertainties requiring validation during implementation:**
1. Liquid horizontal spread rate (avoid instant equalization vs too-slow spreading)
2. Gas dissipation threshold (when to remove gas entirely vs keep trace amounts)
3. Fire directional bias strength (if implemented - may not be needed given Phase 1's heat diffusion)
4. Performance at target grid size (profile before optimizing with chunks/dirty rects)
5. Interaction between environmental behaviors and Phase 1's wind/explosive displacement (test with wind + liquid, wind + gas)
