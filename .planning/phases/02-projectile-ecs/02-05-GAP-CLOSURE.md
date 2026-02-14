---
phase: 02-projectile-ecs
plan: 05-gap-closure
type: execute
wave: 5
depends_on: ["02-04"]
gap_closure: true
files_modified:
  - crates/omega-core/src/lib.rs
  - crates/omega-save/src/lib.rs
  - crates/omega-bevy/src/simulation/systems.rs
  - crates/omega-bevy/src/presentation/targeting.rs
  - crates/omega-bevy/src/simulation/plugin.rs
autonomous: true

must_haves:
  truths:
    - "All Stats initializers in the workspace include the weight field"
    - "Projectile-to-entity (Monster) collision is functional"
    - "Projectile mid-air interception and deflection are implemented"
    - "Targeting UI calculates and displays the predicted trajectory path"
    - "Blast radius visualization includes linear falloff highlighting"
---

<objective>
Resolve the compilation blockers and functional gaps identified in the Phase 2 verification report. This plan fixes broken initializers, implements missing physical interactions (collisions, interception, deflection), and completes the targeting UI prediction logic.

Output: A compiling workspace with fully functional physical projectiles and an informative 'Arcane Cartographer' targeting interface.
</objective>

<tasks>

<task type="auto">
  <name>Task 1: Fix broken Stats initializers across workspace</name>
  <files>
    crates/omega-core/src/lib.rs
    crates/omega-save/src/lib.rs
  </files>
  <action>
Mass update all `Stats` initializers to include `weight`. 
- In `omega-core/src/lib.rs`, search for `Stats {` and ensure every instance has `, weight: 60` (or appropriate value).
- In `omega-save/src/lib.rs`, update the player stats initializer near line 412.
  </action>
</task>

<task type="auto">
  <name>Task 2: Implement Projectile Interception and Deflection</name>
  <files>
    crates/omega-bevy/src/simulation/systems.rs
    crates/omega-bevy/src/simulation/plugin.rs
  </files>
  <action>
Implement `projectile_interception_system` in `systems.rs`:
- Use `Query<(Entity, &mut Projectile, &mut Transform)>`.
- Perform a double-loop check for overlaps (compare `logical_pos` distances).
- Implement negation/deflection logic based on `intensity` and `mass`.
- Register the system in `SimulationPlugin`.
  </action>
</task>

<task type="auto">
  <name>Task 3: Restore Entity Collision Logic</name>
  <files>
    crates/omega-bevy/src/simulation/systems.rs
  </files>
  <action>
Uncomment and implement the `Monster` collision check in `projectile_collision_system`.
- Check distance between `projectile.logical_pos` and monster positions.
- Apply knockback impulse based on `Weight`.
  </action>
</task>

<task type="auto">
  <name>Task 4: Implement Targeting Prediction Logic</name>
  <files>
    crates/omega-bevy/src/presentation/targeting.rs
  </files>
  <action>
Create a helper function to simulate the trajectory math.
Update `TargetingState` to populate `projected_path` during the aiming phase.
Implement blast radius gizmos with linear alpha falloff.
  </action>
</task>

</tasks>

<verification>
- `cargo check --workspace --all-targets` passes without errors
- Gizmos are visible along the predicted path during targeting
- Projectiles collide with monsters and other projectiles
</verification>
