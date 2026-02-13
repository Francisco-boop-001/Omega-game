# Phase 1: Core Elemental State & Reaction System - Research

**Researched:** 2026-02-13
**Domain:** Cellular Automata Simulation with Bevy ECS
**Confidence:** MEDIUM

## Summary

Phase 1 requires implementing a double-buffered cellular automata (CA) grid for elemental simulation in Bevy 0.15, supporting Heat, Wet, and Pressure states with reaction logic. The research reveals that while Bevy provides excellent ECS infrastructure and fixed timestep support for frame-independent simulation, there are no built-in CA primitives. The standard approach uses Resources for grid storage with manual double-buffering, fixed timestep schedules for deterministic updates, and trait-based abstractions for extensible state transitions.

Key challenges include: (1) managing synchronous cell updates without race conditions, (2) implementing efficient neighbor iteration for reaction propagation, (3) balancing performance with the flexibility needed for complex multi-state cells, and (4) handling edge cases in threshold-based state transitions.

**Primary recommendation:** Use a Resource containing Vec2D grid with manual double-buffer swap, schedule CA updates in FixedUpdate at 64Hz, implement state transitions as pure functions with lookup tables for reaction combinations, and leverage Rust's type system for compile-time state validation where possible.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Violent vs. Smoldering Philosophy:**
- Powerful magic (Fireballs) triggers immediate, explosive reactions
- Mundane items (Torches) cause gradual changes based on intensity thresholds
- World is physically reactive, interactive, and persistent

**Intensity Thresholds:**
- Materials have "Flash Points" (e.g., Grass < Wood < Stone)
- High-intensity triggers (Fireball) bypass gradual heat-up

**Gas Layering:**
- Simulation supports gas layers (Steam, Smoke) existing concurrently over liquids (Water, Oil)

**Explosive Displacement:**
- Violent reactions physically push Heat, Pressure, and Gasses into neighboring tiles in a single frame

**State Multiplicity (Layered System):**
- Tiles support concurrent Solid, Liquid, and Gas states (e.g., "Swamp Fire")

**Dousing Logic:**
- "Waterlogged" (max Wet) status grants fire immunity until evaporated

**Wind Vectors:**
- Wind acts as a force map that pushes, pulls, or "cuts" (disperses) Gas and Heat values

**Earth Anchoring:**
- Earth is the structural base; cannot be displaced by Wind/Water, only transformed (Dirt -> Mud, Stone -> Rubble)

**Interactive Debris:**
- Burning leaves "Ash"; breaking leaves "Rubble"
- These are interactive (e.g., Wind blows Ash)

**Combustion Mechanics:**
- Fire expansion governed by fuel density
- High combustible concentrations trigger pressure-based explosions

**Persistence:**
- Heat is "Residual" and decays slowly
- "Nature Reclaims" mechanic where scorched earth or debris eventually recovers/recycles

**Sensory & UI:**
- High-heat tiles exhibit "shimmer/haze" effect
- Active elements (Fire, Lava) bleed color/glow into adjacent tiles
- Log uses descriptive, atmospheric warnings ("The air smells of ozone")
- 'Look' command provides "Vibe" descriptions ("Saturated with moisture," "Scorching") rather than raw percentages

### Claude's Discretion

**Data Structure Optimization:**
- Grid storage layout (Vec of Vec vs flat Vec with index math)
- Memory alignment for cache efficiency
- Whether to use SIMD for parallel cell updates

**State Representation:**
- How to encode multiple concurrent states (bitflags, separate arrays, struct fields)
- Numeric precision for Heat/Wet/Pressure values (f32, u8, fixed-point)

**Neighbor Iteration:**
- Moore vs Von Neumann neighborhood for different reactions
- Edge handling strategy (wrap, clamp, infinite with default values)

**Performance Tuning:**
- Grid update frequency vs frame rate
- Spatial partitioning for large grids
- Early-out optimizations for inactive cells

**Testing Strategy:**
- Unit test coverage for state transitions
- Integration test scenarios
- Performance benchmarking approach

### Deferred Ideas (OUT OF SCOPE)

**Illumination System:**
- Light and Darkness interactions
- Fire illuminating
- Shadows affecting visibility
- Deferred to post-core phase

**Electrical Conductivity:**
- Chain reactions through Water/Metal
- Deferred to later "Shock" phase
</user_constraints>

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| bevy | 0.15 | Game engine and ECS framework | Project requirement; provides Resource system, scheduling, fixed timestep, and multi-threaded execution |
| bevy_ecs | 0.15 | Entity Component System | Core of Bevy; enables data-driven architecture with Resources for grid storage and Systems for CA logic |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| bevy_life | 0.1.0+ | Generic CA plugin reference | Study architecture patterns only; too generic for custom elemental system |
| ringbuffer | Latest | Ring buffer for multi-generation history | If implementing rewind/replay or multi-step rule evaluation |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Manual double-buffer | bevy_life plugin | Plugin is too generic; custom elemental logic (flash points, layered states) requires tailored implementation |
| Vec\<Vec\<Cell\>\> | Flat Vec with index math | Flat Vec offers better cache locality but harder to reason about; use flat Vec if profiling shows cache misses |
| f32 for values | u8 or fixed-point | f32 simplifies gradual transitions but uses more memory; use u8 for values 0-255 if memory-constrained |

**Installation:**
```bash
# Already in Cargo.toml per project structure
# No additional dependencies needed for core CA
```

## Architecture Patterns

### Recommended Project Structure

Based on existing `crates/omega-bevy` structure, add CA module:

```
crates/omega-bevy/src/
├── simulation/              # New module for CA
│   ├── mod.rs              # Module root, exports public API
│   ├── grid.rs             # CaGrid resource, double-buffer logic
│   ├── cell.rs             # Cell struct with Heat/Wet/Pressure
│   ├── state.rs            # State enums and transition logic
│   ├── reactions.rs        # Reaction lookup tables (Fire+Water=Steam)
│   ├── systems.rs          # Bevy systems (update_ca, swap_buffers)
│   └── neighborhood.rs     # Neighbor iteration utilities
└── presentation/           # Existing rendering code
    └── tilemap.rs          # Reads CaGrid for visualization
```

**Rationale:** Separates simulation (domain logic) from presentation (rendering), following project's existing pattern (`omega-core` for logic, `omega-bevy` for Bevy integration).

### Pattern 1: Resource-Based Double Buffer

**What:** Store CA grid as a Bevy Resource with front/back buffers, swap after each update cycle.

**When to use:** When grid is singleton, accessed globally, and updates synchronously.

**Example:**
```rust
// Source: Adapted from Bevy Resource patterns + CA community practices
#[derive(Resource)]
pub struct CaGrid {
    width: usize,
    height: usize,
    front: Vec<Cell>,  // Read from this during update
    back: Vec<Cell>,   // Write to this during update
}

impl CaGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            front: vec![Cell::default(); size],
            back: vec![Cell::default(); size],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.front[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.back[y * self.width + x] = cell;
    }

    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }
}

// System in FixedUpdate schedule
fn update_ca_cells(mut grid: ResMut<CaGrid>) {
    // Iterate all cells, read from front, write to back
    for y in 0..grid.height {
        for x in 0..grid.width {
            let new_cell = compute_next_state(&grid, x, y);
            grid.set(x, y, new_cell);
        }
    }
}

fn swap_ca_buffers(mut grid: ResMut<CaGrid>) {
    grid.swap_buffers();
}

// In app setup
app.insert_resource(CaGrid::new(128, 128))
    .add_systems(FixedUpdate, (
        update_ca_cells,
        swap_ca_buffers.after(update_ca_cells),
    ));
```

**Key insight:** Using `std::mem::swap` is zero-cost; avoids copying entire grid.

### Pattern 2: Fixed Timestep for Deterministic Simulation

**What:** Run CA updates in `FixedUpdate` schedule at consistent interval (default 64Hz).

**When to use:** Always, for CA simulation. Ensures deterministic behavior regardless of frame rate.

**Example:**
```rust
// Source: https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html
app.insert_resource(Time::<Fixed>::from_hz(64.0))  // 64 updates/sec
    .add_systems(FixedUpdate, (
        update_ca_cells,
        swap_ca_buffers.after(update_ca_cells),
    ));

// Access fixed delta in systems
fn update_ca_cells(
    mut grid: ResMut<CaGrid>,
    time: Res<Time>,  // Automatically uses Fixed context
) {
    let delta = time.delta_seconds();  // Always 1/64 = 0.015625
    // Use delta for decay/gradual changes
}
```

**Key insight:** Default 64Hz avoids pathological interaction with 60Hz monitors (alternating 0/2 updates per frame).

### Pattern 3: State Transition with Lookup Tables

**What:** Use lookup tables or match statements for reaction logic instead of complex conditionals.

**When to use:** When reactions are discrete and enumerable (Fire+Water=Steam).

**Example:**
```rust
// Source: Game programming patterns + CA best practices
#[derive(Copy, Clone, Default)]
pub struct Cell {
    pub heat: u8,       // 0-255
    pub wet: u8,        // 0-255
    pub pressure: u8,   // 0-255
    pub material: Material,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Material {
    Air,
    Water,
    Earth,
    Fire,
    Steam,
    Mud,
}

// Reaction lookup
fn apply_reaction(cell: &Cell, neighbors: &[Cell]) -> Cell {
    use Material::*;

    match (cell.material, cell.heat, cell.wet) {
        // Water + High Heat = Steam
        (Water, heat, _) if heat > 200 => Cell {
            material: Steam,
            heat: heat.saturating_sub(50),  // Consumes heat
            wet: 0,
            ..*cell
        },

        // Earth + Water = Mud
        (Earth, _, wet) if wet > 150 => Cell {
            material: Mud,
            ..*cell
        },

        // Fire + Max Wet = Extinguished
        (Fire, _, wet) if wet >= 255 => Cell {
            material: Air,
            heat: cell.heat / 2,  // Residual heat
            wet: 200,  // Some water remains
            ..*cell
        },

        _ => *cell,  // No reaction
    }
}
```

**Key insight:** Explicit pattern matching makes reactions auditable and easy to balance.

### Pattern 4: Threshold-Based State Transitions

**What:** Use threshold values to trigger instant transitions vs. gradual accumulation.

**When to use:** Implementing "Violent vs. Smoldering" philosophy with Flash Points.

**Example:**
```rust
// Flash point thresholds per material
const FLASH_POINTS: &[(Material, u8)] = &[
    (Material::Grass, 120),
    (Material::Wood, 180),
    (Material::Stone, 250),
];

fn check_ignition(cell: &Cell, heat_source: u8) -> Option<Material> {
    for (material, flash_point) in FLASH_POINTS {
        if cell.material == *material && heat_source >= *flash_point {
            return Some(Material::Fire);
        }
    }
    None
}

fn apply_heat(cell: &mut Cell, amount: u8, is_violent: bool) {
    if is_violent {
        // Fireball: instant flash point check
        if let Some(new_material) = check_ignition(cell, amount) {
            cell.material = new_material;
            cell.heat = 255;  // Max heat
        }
    } else {
        // Torch: gradual accumulation
        cell.heat = cell.heat.saturating_add(amount);
        if cell.heat >= get_flash_point(cell.material) {
            cell.material = Material::Fire;
        }
    }
}
```

**Key insight:** `is_violent` flag distinguishes instant reactions (Fireball) from gradual (Torch).

### Pattern 5: Multi-State Layering

**What:** Support concurrent Solid, Liquid, Gas states in a single cell.

**When to use:** Implementing "Swamp Fire" (Fire + Mud), gas over liquid.

**Example:**
```rust
#[derive(Copy, Clone, Default)]
pub struct Cell {
    pub solid: Option<Solid>,    // Earth, Stone, Mud
    pub liquid: Option<Liquid>,  // Water, Oil
    pub gas: Option<Gas>,        // Steam, Smoke
    pub heat: u8,
    pub wet: u8,
    pub pressure: u8,
}

#[derive(Copy, Clone)]
pub enum Solid { Earth, Stone, Mud, Ash, Rubble }

#[derive(Copy, Clone)]
pub enum Liquid { Water, Oil }

#[derive(Copy, Clone)]
pub enum Gas { Steam, Smoke, Fire }

// Gas over liquid over solid
impl Cell {
    pub fn visible_material(&self) -> Option<DisplayMaterial> {
        if let Some(gas) = self.gas {
            return Some(DisplayMaterial::Gas(gas));
        }
        if let Some(liquid) = self.liquid {
            return Some(DisplayMaterial::Liquid(liquid));
        }
        if let Some(solid) = self.solid {
            return Some(DisplayMaterial::Solid(solid));
        }
        None  // Air
    }

    pub fn can_ignite(&self) -> bool {
        // Fire can't exist on water-saturated cells
        self.wet < 255 && self.gas != Some(Gas::Steam)
    }
}
```

**Key insight:** `Option<T>` for each layer; `None` means that layer is empty (e.g., no gas = open air above).

### Anti-Patterns to Avoid

- **Reading and writing same buffer:** Causes temporal artifacts where update order affects results. Always use double-buffer.
- **Frame-dependent CA updates:** Running CA in `Update` instead of `FixedUpdate` makes behavior non-deterministic. Use fixed timestep.
- **Deep neighbor recursion:** Recursive fire spread can stack overflow. Use iterative propagation or queue-based approach.
- **Floating-point for discrete states:** Using f32 for boolean-like states (ignited/not ignited) wastes memory. Use enums or bitflags.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Fixed timestep scheduling | Custom accumulator loop | Bevy's `FixedUpdate` schedule | Bevy handles edge cases (pause, time dilation, multiple catches-up per frame) |
| Neighbor iteration | Manual bounds checking | Utility functions with clamped/wrapped access | Off-by-one errors, edge case bugs (corners have fewer neighbors) |
| Buffer swapping | Manual cell-by-cell copy | `std::mem::swap` | Zero-cost, prevents partial-swap bugs |
| State enum serialization | Manual match arms | Derive `serde::Serialize` | Reduces boilerplate, prevents desync between code and save format |

**Key insight:** CA has many subtle edge cases (corners, synchronization, determinism). Leverage Bevy's battle-tested scheduling and Rust's standard library to avoid reinventing wheels.

## Common Pitfalls

### Pitfall 1: Synchronous Update Race Conditions

**What goes wrong:** Two adjacent cells both try to "claim" a shared neighbor's state, causing non-deterministic results.

**Why it happens:** Reading and writing from same buffer, or using multithreaded iteration without proper synchronization.

**How to avoid:**
- Always use double-buffer (read front, write back)
- Complete ALL cell updates before swapping buffers
- Use `swap_ca_buffers.after(update_ca_cells)` system ordering

**Warning signs:** Flickering patterns, results change when reversing grid iteration order, different outcomes on different machines.

### Pitfall 2: Edge and Corner Handling

**What goes wrong:** Cells at grid edges have fewer neighbors, causing index out-of-bounds or biased behavior.

**Why it happens:** Naive `grid[y-1][x]` indexing without bounds checking.

**How to avoid:**
```rust
fn get_neighbor_safe(grid: &CaGrid, x: usize, y: usize, dx: isize, dy: isize) -> Cell {
    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if nx < 0 || ny < 0 || nx >= grid.width as isize || ny >= grid.height as isize {
        return Cell::default();  // Out of bounds = Air
    }

    grid.get(nx as usize, ny as usize).clone()
}
```

Alternative: Wrap edges (toroidal topology) or add 1-cell border of immutable cells.

**Warning signs:** Crashes near edges, asymmetric propagation patterns.

### Pitfall 3: Threshold Hysteresis

**What goes wrong:** State rapidly oscillates across threshold (e.g., water heats to 201°, becomes steam, cools to 199°, becomes water, repeat).

**Why it happens:** No hysteresis margin between forward and reverse transitions.

**How to avoid:**
```rust
// Forward: Water -> Steam at 200°
if cell.material == Material::Water && cell.heat > 200 {
    cell.material = Material::Steam;
}

// Reverse: Steam -> Water at 180° (20° margin)
if cell.material == Material::Steam && cell.heat < 180 {
    cell.material = Material::Water;
}
```

**Warning signs:** Flickering between states, excessive heat/cool cycles.

### Pitfall 4: Explosive Propagation Causing Infinite Loops

**What goes wrong:** Explosive displacement creates feedback loop where cells keep bouncing heat/pressure infinitely.

**Why it happens:** Immediate neighbor modification without limiting propagation depth.

**How to avoid:**
- Use queue-based propagation with max depth
- Decay heat/pressure slightly on each bounce
- Implement "already visited this frame" flag

**Warning signs:** CPU pegs at 100%, simulation never stabilizes, heat values grow unbounded.

### Pitfall 5: Mixing Fixed and Variable Timesteps

**What goes wrong:** CA updates in `FixedUpdate` but rendering in `Update` causes temporal aliasing (stuttering, torn states).

**Why it happens:** Renderer reads grid mid-update or during buffer swap.

**How to avoid:**
- Always complete CA update + buffer swap before rendering
- Use `.chain()` or explicit ordering: `swap_ca_buffers.before(render_grid)`
- Consider rendering from a separate read-only copy if needed

**Warning signs:** Visual glitches, half-updated frames, screen tearing in grid display.

## Code Examples

Verified patterns from official sources and community best practices:

### Complete CA Update System

```rust
// Core update loop
pub fn update_ca_grid(mut grid: ResMut<CaGrid>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let current = *grid.get(x, y);
            let neighbors = collect_neighbors(&grid, x, y);

            let mut next = current;

            // Apply reactions
            next = apply_reactions(&next, &neighbors);

            // Apply gradual changes (decay, diffusion)
            next = apply_decay(&next, delta);
            next = apply_diffusion(&next, &neighbors, delta);

            grid.set(x, y, next);
        }
    }
}

pub fn swap_ca_grid_buffers(mut grid: ResMut<CaGrid>) {
    grid.swap_buffers();
}

// Collect Moore neighborhood (8 neighbors)
fn collect_neighbors(grid: &CaGrid, x: usize, y: usize) -> [Cell; 8] {
    [
        get_neighbor_safe(grid, x, y, -1, -1),  // NW
        get_neighbor_safe(grid, x, y,  0, -1),  // N
        get_neighbor_safe(grid, x, y,  1, -1),  // NE
        get_neighbor_safe(grid, x, y, -1,  0),  // W
        get_neighbor_safe(grid, x, y,  1,  0),  // E
        get_neighbor_safe(grid, x, y, -1,  1),  // SW
        get_neighbor_safe(grid, x, y,  0,  1),  // S
        get_neighbor_safe(grid, x, y,  1,  1),  // SE
    ]
}
```

### Heat Diffusion with Decay

```rust
// Based on Laplacian operator for heat diffusion
fn apply_diffusion(cell: &Cell, neighbors: &[Cell], delta: f32) -> Cell {
    let k = 0.1;  // Thermal conductivity

    let neighbor_avg_heat: u8 = (neighbors.iter()
        .map(|n| n.heat as u32)
        .sum::<u32>() / neighbors.len() as u32) as u8;

    let diff = (neighbor_avg_heat as i16 - cell.heat as i16) as f32;
    let heat_transfer = (diff * k * delta) as i16;

    let new_heat = (cell.heat as i16 + heat_transfer).clamp(0, 255) as u8;

    Cell {
        heat: new_heat,
        ..*cell
    }
}

fn apply_decay(cell: &Cell, delta: f32) -> Cell {
    let decay_rate = 10.0;  // Heat units per second
    let decay_amount = (decay_rate * delta) as u8;

    Cell {
        heat: cell.heat.saturating_sub(decay_amount),
        ..*cell
    }
}
```

### State Machine with Type Safety

```rust
// Compile-time state validation using type system
pub trait CellState: Copy {}

impl CellState for Material {}

pub struct StateMachine<S: CellState> {
    current: S,
}

impl StateMachine<Material> {
    pub fn transition(&mut self, heat: u8, wet: u8) {
        use Material::*;

        self.current = match (self.current, heat, wet) {
            (Water, h, _) if h > 200 => Steam,
            (Earth, _, w) if w > 150 => Mud,
            (Fire, _, w) if w >= 255 => Air,
            (state, _, _) => state,
        };
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Single buffer CA | Double-buffered updates | Always standard | Prevents read/write conflicts, ensures deterministic synchronous updates |
| Frame-dependent updates | Fixed timestep (64Hz) | Bevy 0.5+ | Deterministic simulation regardless of render FPS |
| Monolithic grid storage | ECS with Resource pattern | Bevy 0.1+ | Decouples simulation from rendering, enables parallel systems |
| Entity-per-cell | Resource grid + entity overlay | Community consensus | Entities for interactive objects (projectiles), Resource for homogeneous grid |
| Manual event scheduling | `FixedUpdate` schedule | Bevy 0.12+ | Automatic handling of catch-up frames and time dilation |

**Deprecated/outdated:**
- **`iyes_loopless` for fixed timestep**: Built into Bevy core since 0.10, use native `FixedUpdate` schedule
- **`bevy_tilemap` (v0.x)**: Unmaintained, use `bevy_ecs_tilemap` for rendering or custom rendering
- **Entity-per-cell ECS approach**: Too much overhead; Resources + sparse entities for special tiles is current best practice

## Open Questions

1. **Memory layout for multi-state cells**
   - What we know: Layered system requires `Option<Solid>`, `Option<Liquid>`, `Option<Gas>` per cell
   - What's unclear: Cache efficiency of 3 Option fields vs. bitpacked enum vs. separate arrays
   - Recommendation: Start with struct-of-arrays (3 separate `Vec<Option<_>>`), profile, then optimize if needed

2. **Explosive displacement implementation**
   - What we know: "Violent reactions physically push Heat, Pressure, and Gasses into neighboring tiles in a single frame"
   - What's unclear: Is this immediate (synchronous) or queued? How to prevent feedback loops?
   - Recommendation: Use queue-based propagation with max depth (e.g., 5 hops), decay on each hop

3. **Wind as force map**
   - What we know: Wind pushes/pulls/disperses Gas and Heat
   - What's unclear: Is Wind a separate grid? Per-cell vector? Global constant?
   - Recommendation: Separate `WindGrid` Resource with per-cell `(dx, dy, strength)` vectors, updated less frequently (e.g., 10Hz)

4. **Nature Reclaims timing**
   - What we know: Scorched earth eventually recovers
   - What's unclear: Timescale (seconds? minutes?), per-cell timer or global check?
   - Recommendation: Per-cell "time since last change" counter, check every 60 seconds of simulation time

## Sources

### Primary (HIGH confidence)

- [Bevy 0.15 Rust Docs](https://docs.rs/bevy/0.15/bevy/) - ECS, Resources, system parameters
- [Fixed Timestep - Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html) - FixedUpdate schedule, configuration
- [Resources - Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/programming/res.html) - Resource patterns, access, best practices

### Secondary (MEDIUM confidence)

- [Cellular Automata Using Rust: Part I-III (Xebia)](https://xebia.com/blog/cellular-automata-using-rust-part-i/) - Rust CA implementation patterns, verified with official docs
- [bevy_life GitHub](https://github.com/ManevilleF/bevy_life) - Architecture reference for trait-based CA, verified active project
- [Learning Rust by Simulating Heat Diffusion (Tej Qu Nair)](https://tejqunair.com/posts/rust-heat/) - Heat propagation formulas, Laplacian operator
- [Physically Based Temperature Simulation For Games (Alexandru Ene)](https://alexene.dev/2020/01/10/Physically-based-temperature-simulation-for-games.html) - Thermal conductivity, diffusion algorithms
- [Bevy Events Documentation (Tainted Coders)](https://taintedcoders.com/bevy/events) - Double-buffered event system patterns

### Tertiary (LOW confidence)

- WebSearch: "cellular automata pitfalls" - General CA synchronization issues, requires validation against project specifics
- WebSearch: "state machine game design" - Generic patterns, adapt to elemental context
- Community discussions on asynchronous CA - Academic, may not apply to real-time game context

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Bevy 0.15 confirmed in project Cargo.toml, fixed timestep verified in official docs
- Architecture: MEDIUM - Resource-based grid is consensus for this use case, but multi-state layering needs validation through prototyping
- Pitfalls: MEDIUM - Standard CA pitfalls (edges, synchronization) are well-documented, but explosive displacement and wind interactions need testing

**Research date:** 2026-02-13
**Valid until:** 2026-03-15 (30 days; Bevy releases quarterly, CA patterns are stable)

**Limitations:**
- Context7 unavailable (API key issue), relied on official docs and verified community sources
- No 2026-specific Bevy CA projects found; extrapolated from 2024-2025 patterns
- Explosive displacement and wind mechanics need design prototyping; no direct precedents found
