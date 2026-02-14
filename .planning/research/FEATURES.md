# Feature Landscape: Elemental Systems

**Domain:** Grid-based Roguelike
**Researched:** 2024-02-12

## Table Stakes

Features users expect in an elemental-heavy roguelike.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Heat Diffusion | Fire spreads, melts ice. | Low | Standard CA neighbor averaging. |
| Liquid Flow | Water/Lava fills pits, flows down. | Medium | Requires "Sand-falling" algorithm variation. |
| Gas Propagation | Steam/Smoke rises, dissipates. | Medium | Upward diffusion + random horizontal drift. |
| Basic Reactions | Fire + Water = Steam. | Low | Event-driven entity swapping. |

## Differentiators

Features that set the system apart.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Arc Projectiles | Logical Z-height allows shooting over obstacles. | Medium | Separates logic from 2D visual rendering. |
| Material States | Freezing/Boiling points for every tile. | High | Requires shared `MaterialProperties` component. |
| Persistent Particles | Smoke that blocks vision/affects breath. | Medium | CA grid integrated with LOS (Line of Sight) system. |
| Glyph VFX | High-performance "ASCII" particles. | Medium | Using character sprites for fire sparks/bubbles. |

## Anti-Features

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Entity-per-molecule | Too slow for gas/liquid. | Use a background grid (Resource). |
| Realistic Fluid Dynamics | Navier-Stokes is overkill/too heavy. | Use "Cellular Automata" approximations. |
| Global 3D Physics | Too complex for a grid roguelike. | Use 2D + Pseudo-Z (Y-offset). |

## Feature Dependencies

```
Grid Resource → Heat/Liquid CA → Reaction System → VFX System
Projectile Logic → Z-Height Component → Visual Y-Offset
```

## MVP Recommendation

Prioritize:
1. **Heat/Fire CA:** Most visible and impactful elemental mechanic.
2. **Event-based Reactions:** Allow Fire to destroy combustible entities.
3. **Simple Trajectories:** Projectiles with a visual arc (Y-offset).

Defer: Full liquid flow and gas dissipation until core grid performance is verified.

## Sources

- "Caves of Qud" architecture talks
- "Noita" GDC Talk: "Falling Everything"
