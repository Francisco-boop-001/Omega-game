# Technology Stack: Elemental Roguelike

**Project:** Omega (Bevy Modernization)
**Researched:** 2024-02-12

## Recommended Stack

### Core Framework
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Bevy | 0.15.x | Game Engine | Latest stable, improved Text2d (Cosmic Text), Entity-based Spans. |
| ndarray | 0.15.x | Simulation Grid | Efficient n-dimensional arrays for CA simulation. |

### Simulation & Physics
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| bitvec | 1.0 | Optimization | Tracking "active" cells in CA grid with minimal memory. |
| rayon | 1.8 | Parallelism | Multithreaded CA grid updates. |

### Visuals
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| bevy_particle_systems | 0.12+ | VFX | Efficient batched sprite particles. |
| TextureAtlas | Internal | Glyph Rendering | Batching characters into a single draw call. |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Grid | `ndarray` | Bevy Entities | Entities have 100-1000x more overhead per cell. |
| Particles | TextureAtlas Sprites | `Text2d` / `TextSpan` | Even with 0.15 improvements, thousands of text entities impact FPS. |
| Collision | Custom Grid-based | `bevy_rapier2d` | Roguelike grid collision is simpler and more predictable. |

## Installation

```bash
# Core Dependencies
cargo add bevy ndarray bitvec rayon
cargo add bevy_particle_systems
```

## Sources

- Bevy 0.15 Release Notes (Cosmic Text)
- "Noita-style" simulation post-mortems
- Bevy community performance benchmarks for Text2d
