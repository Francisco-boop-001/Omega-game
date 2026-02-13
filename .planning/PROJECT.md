# Project: Wizard's Arena

## Context
Omega 0.90 requires a robust way to verify new gameplay systems (AI, Magic, Physics) without navigating through procedural dungeons or risking save files. The **Wizard's Arena** serves as a specialized sandbox environment for developers and designers to stress-test mechanics in a controlled, real-time setting.

## Vision
A seamless, interactive "test chamber" accessible from the game menu, allowing the user to manipulate the world state, spawn any entity, and observe complex interactions (like fire spreading) with minimal friction.

## Core Features
- **The Arena:** An open, outdoor map with varied terrain (grass, trees, stone buildings).
- **The Spawner:** A menu-driven interface to select and place monsters, items, and environmental hazards.
- **The Lab:** Tools to inspect and modify spawned entities in real-time.
- **The Simulation:** Discrete control over environmental effects like fire propagation and AI thinking.

## Success Criteria
1. Ability to enter the Arena from the main menu.
2. Spawning a monster at a specific tile using a mouse click.
3. Modifying a monster's health via an egui menu.
4. Toggling fire on a tile and observing its spread to adjacent flammable tiles.
