# Architecture Patterns: Elemental & Projectiles

**Domain:** Bevy Grid-based Roguelike
**Researched:** 2024-02-12

## Recommended Architecture

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| `CaGrid` (Resource) | Stores tile data (Heat, Liquid, Gas). | `CaSimulationSystem` |
| `Element` | Entity marker (Fire, Water, etc.). | `ReactionSystem` |
| `LogicalPos` | 3D Grid coordinate (x, y, z_height). | `MovementSystem`, `TransformSystem` |
| `Projectile` | Trajectory data (velocity, gravity). | `TrajectorySystem` |

### Data Flow

1. **Simulation:** `CaSimulationSystem` updates `CaGrid` Resource using `rayon`.
2. **Detection:** `ReactionSystem` reads `CaGrid` and Entity positions to find overlaps.
3. **Events:** When reactions occur, `ReactionSystem` sends `ReactionEvent`.
4. **Resolution:** Specialized systems handle specific events (e.g., `handle_evaporation`).
5. **Visuals:** `TransformSystem` maps `LogicalPos` (x, y, z) to Bevy `Transform` (x, y - z_offset).

## Patterns to Follow

### Pattern 1: Pseudo-Z Trajectory (The "Y-Offset" Arc)
**What:** Mapping a 3D arc to a 2D view.
**When:** For projectiles that should look like they are flying "up and over".
**Example:**
```rust
fn update_visual_transform(
    query: Query<(&LogicalPos, &mut Transform)>
) {
    for (pos, mut transform) in query.iter_mut() {
        // Map grid (x, y) to world (x, y)
        // Offset world Y by Logical Z to create "height"
        transform.translation.x = pos.x * TILE_SIZE;
        transform.translation.y = (pos.y * TILE_SIZE) + pos.z_height; 
        
        // Use translation.z for rendering layer (Projectiles > Actors > Floor)
        transform.translation.z = 10.0; 
    }
}
```

### Pattern 2: Sleeping Cell CA
**What:** Only update grid cells that have changed or are adjacent to changes.
**Why:** Drastically reduces CPU usage in large maps where most cells are stable.
**Instead of:** Iterating over every tile in a 200x200 grid every frame.

## Anti-Patterns to Avoid

### Anti-Pattern 1: Every Atom is an Entity
**What:** Spawning a Bevy Entity for every pixel of water or steam.
**Why bad:** Entity overhead (components, IDs, storage) will crash performance at scale.
**Instead:** Store fluid data in a `Vec<u8>` or `ndarray` and only spawn entities for "significant" clusters or special effects.

## Scalability Considerations

| Concern | At 100 cells | At 10K cells | At 1M cells |
|---------|--------------|--------------|-------------|
| CPU Usage | Negligible. | Noticeable (1-2ms). | Heavy (10ms+). Needs `bitvec`. |
| Memory | Bytes. | ~1MB. | ~100MB. |
| Rendering | Batching unnecessary. | Batching required. | GPU Shaders required. |

## Sources

- Bevy `Transform` documentation
- "Falling Sand" optimization articles
