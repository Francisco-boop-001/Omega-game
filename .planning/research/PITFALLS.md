# Domain Pitfalls: Elemental & Projectiles

**Domain:** Grid-based Roguelike
**Researched:** 2024-02-12

## Critical Pitfalls

### Pitfall 1: Order-of-Update Artifacts
**What goes wrong:** Fluids flow faster in one direction (e.g., Left-to-Right) because the grid update is sequential.
**Why it happens:** Updating the same grid "in-place" means a cell updated at index 0 affects index 1 in the same frame.
**Consequences:** Weird "bias" in fire spreading or water flowing.
**Prevention:** Use **Double Buffering**. Read from `GridA`, write to `GridB`, then swap.

### Pitfall 2: Entity Bloat in CA
**What goes wrong:** Performance tanks after a few fireballs.
**Why it happens:** Spawning fire particles as entities and never cleaning them up, or spawning too many.
**Prevention:** Cap the maximum number of visual entities. Use a Resource for the "logic" of the fire and only spawn sprites for the *edges* or *intensity* points.

## Moderate Pitfalls

### Pitfall 1: Z-Height Collision Ambiguity
**What goes wrong:** Projectile hits a wall it should have flown over.
**Prevention:** Wall entities should have a `Height` component. The collision system must check: `if projectile.z < wall.height { collide() }`.

### Pitfall 2: Text Rendering Overhead
**What goes wrong:** Using `Text2d` for 500+ small particles causes frame drops.
**Prevention:** Use a `TextureAtlas` of glyphs and render them as `SpriteBundle` (or `Sprite` in 0.15). Sprites are much cheaper to batch than text glyphs which go through a complex layout engine (Cosmic Text).

## Minor Pitfalls

### Pitfall 1: Floating Point Drifting
**What goes wrong:** Grid positions get slightly off (e.g., 1.000001) causing lookup failures.
**Prevention:** Use `IVec2` for grid indices and only use `f32` for sub-tile movement or visual offsets.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| CA Simulation | Sequential Bias | Double-buffering / Swapping. |
| Reaction System | N^2 Complexity | Spatial partitioning (Grid lookup). |
| Visuals | Draw Call Bloat | Use `TextureAtlas` for glyphs. |

## Sources

- "Caves of Qud" dev blogs on performance.
- Bevy Issue Tracker (Text performance discussions).
