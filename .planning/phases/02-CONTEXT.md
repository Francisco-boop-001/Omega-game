# Context: Phase 2 - Projectile ECS & Trajectory Physics

## Vision & Tone
Projectiles are physical entities that exist in a 3D logical space (X, Y, Z). They are not just "bullets"; they are interactive objects that can collide mid-air, be deflected, or roll along the ground. The "Siege Mage" experience is defined by choosing the right trajectory (High Arc, Flat Arc, or Roll) to bypass or exploit the environment.

## Projectile Behavior & Physics
- **Trajectory Modes:**
  - **High Arc:** High logical Z-height, clears most obstacles, slower travel time.
  - **Flat Arc:** Lower Z-height, fast travel, easily blocked by mid-height obstacles.
  - **Rolling Mode:** Stays at Z=0. Ignores flying enemies but is blocked by low debris/furniture.
- **Interception & Deflection:**
  - **Dynamic Intersections:** Mid-air collisions between projectiles. Equal intensity negates; higher intensity "bats away" the weaker projectile.
  - **Deflection:** Deflected projectiles (e.g., a Firebolt hit by an arrow) bounce and can hit secondary targets, walls, or friendlies.
- **Piercing:** Projectiles can pierce targets based on force. Each pierce reduces velocity/force until the projectile stops or falls.

## The Weight Stat
- **Introduction of Weight:** A new `Weight` parameter is added to the `Stats` struct for all entities (Player, Monsters, Items).
- **Knockback:** Blast vectors from explosions apply impulse based on distance and the target's Weight.
- **Impact Force:** Projectiles falling from a high Z-height gain additional impact force proportional to their Weight.

## Impact & Explosion Dynamics
- **Blast Vectors:** Explosions apply a linear force falloff from the center.
- **Environmental Shock:** Pressure waves can break fragile items (e.g., potions) even if they aren't directly hit by fire.
- **Heavy Thresholds:** Objects with high Weight stats are resistant or immune to displacement from mundane blasts.

## Targeting UX (The "Arcane Cartographer" Interface)
- **Temporal Ghost:** The aiming UI shows pulsing ghost glyphs in the element's color to visualize the trajectory.
- **Predictive Previews:** 
  - Shimmering highlight for the expected Blast Radius.
  - "Reaction Icons" (e.g., a Steam cloud icon) over targets where a collision will trigger a CA reaction.
- **Range Handling:** Aiming beyond max range shows a "dropping" trajectory (visualizing where the projectile will fall short).

## Technical Implementation Notes
- **Z-Height Logic:** Projectiles check collisions against obstacles only if `projectile.z <= obstacle.height`.
- **The "Bat Incident":** Lobbed projectiles can collide with flying entities (high Z-height) mid-flight.
- **Weight Integration:** Update `omega-core::Stats` and ensure serialization/deserialization remains compatible with the Save system.
