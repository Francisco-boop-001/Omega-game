# Research Summary: Cellular Automata & Elemental Systems in Bevy

**Domain:** Grid-based Roguelike Elemental Systems
**Researched:** 2024-02-12
**Overall confidence:** HIGH

## Executive Summary

Implementing a high-performance elemental system in a Bevy-based roguelike requires a strict separation between simulation (Cellular Automata) and representation (ECS Entities). Modern Bevy (0.15+) provides excellent tools for this, but naive implementations (e.g., making every gas particle an entity) will quickly hit performance bottlenecks.

The recommended approach uses a **Resource-based Grid** for Cellular Automata (CA) to handle heat, liquid, and gas, while using **ECS Events** to decouple the reaction logic. Projectiles should use a **Logical 3D Coordinate** system that maps to 2D transforms with a Y-offset for visual arcs. For visuals, **Mesh-based Glyph Atlases** are preferred over the native `Text` component for high-density particle effects.

## Key Findings

**Stack:** Bevy 0.15 (Cosmic Text), `ndarray` (grid simulation), `bevy_particle_systems` (visuals).
**Architecture:** Data-parallel CA in Resources + Event-driven Reaction Systems.
**Critical pitfall:** Entity-per-cell overhead for gas/liquid simulations.

## Implications for Roadmap

### Suggested Phase Structure

1. **Phase 1: Core Simulation Engine** - Implement the CA grid as a Bevy Resource.
   - Focus: Double-buffered grid, sleeping/waking cell optimization.
   - Addresses: Heat/Liquid/Gas base simulation.

2. **Phase 2: Decoupled Reaction System** - Event-driven elemental logic.
   - Focus: Proximity detection using the CA grid, `ReactionEvent` processing.
   - Avoids: Spaghetti logic inside movement/combat systems.

3. **Phase 3: Trajectory & Z-Height** - Projectile logic.
   - Focus: Logic-to-visual mapping (Y-offset arcs).
   - Addresses: Projectiles passing over walls/pits.

4. **Phase 4: Visual Polish (Glyph Particles)** - High-performance VFX.
   - Focus: Texture atlas for characters, batched sprite rendering.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Bevy 0.15 is stable; `ndarray` is standard for Rust grids. |
| Features | HIGH | Table stakes for "Noita-like" or "Caves of Qud-like" systems. |
| Architecture | HIGH | Event-driven ECS is the idiomatic Bevy way. |
| Pitfalls | MEDIUM | Performance varies significantly with grid size and update frequency. |

## Gaps to Address

- **GPU Compute:** For massive grids (512x512+), GPU compute shaders would be better, but they introduce complexity in syncing with ECS game state.
- **Multithreading:** While Bevy's ECS is parallel, custom CA logic in Resources needs careful implementation to utilize all cores (e.g., using `rayon`).
