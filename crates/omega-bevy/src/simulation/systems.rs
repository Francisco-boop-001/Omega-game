use bevy::prelude::*;
use omega_core::simulation::{
    grid::CaGrid,
    wind::{WindGrid, apply_wind},
    reactions::compute_next_cell,
    neighborhood::moore_neighbors,
    displacement::{check_explosion_trigger, apply_explosive_displacement},
    decay::apply_full_decay_cycle,
    environmental::{apply_liquid_flow, apply_gas_rise, apply_fire_spread_bias},
    TrajectoryMode,
};
use super::projectiles::Projectile;
use super::particles::{Particle, ParticleKind, VisualCascade};
use omega_core::simulation::displacement::DisplacementEvent;

#[derive(Resource, Default)]
pub struct SimulationTick(pub u64);

pub fn increment_tick(mut tick: ResMut<SimulationTick>) {
    tick.0 = tick.0.wrapping_add(1);
}

pub fn particle_physics_system(
    time: Res<Time<Fixed>>,
    grid: Res<CaGrid>,
    mut query: Query<(&mut Particle, &mut Transform)>,
) {
    let dt = time.delta_secs();
    const GRAVITY: f32 = -20.0;
    const UPWARD_DRIFT: f32 = 2.0;
    const DAMPING: f32 = 0.6;

    for (mut particle, mut transform) in query.iter_mut() {
        // 1. Apply Kind-specific forces
        match particle.kind {
            ParticleKind::Debris => {
                particle.velocity.z += GRAVITY * dt;
            }
            ParticleKind::Smoke | ParticleKind::Steam => {
                particle.velocity.z += UPWARD_DRIFT * dt;
            }
            _ => {}
        }

        // 2. Update Logical Position
        let v = particle.velocity;
        particle.logical_pos += v * dt;

        // 3. Wall Bouncing
        let x = particle.logical_pos.x.round() as i32;
        let y = particle.logical_pos.y.round() as i32;
        if grid.in_bounds(x as isize, y as isize) {
            let cell = grid.get(x as usize, y as usize);
            if matches!(cell.solid, Some(omega_core::simulation::Solid::Stone)) && particle.logical_pos.z < 1.0 {
                // Simplified bounce
                particle.velocity.x *= -DAMPING;
                particle.velocity.y *= -DAMPING;
                particle.logical_pos.x += particle.velocity.x * dt;
                particle.logical_pos.y += particle.velocity.y * dt;
            }
        }

        // 4. Floor limit
        if particle.logical_pos.z < 0.0 {
            particle.logical_pos.z = 0.0;
            particle.velocity.z *= -DAMPING;
        }

        // 5. Visual Mapping (same scale as projectiles)
        transform.translation.x = particle.logical_pos.x;
        transform.translation.y = particle.logical_pos.y + (particle.logical_pos.z * 0.8);
        transform.translation.z = 90.0; 
    }
}

pub fn particle_wind_drift_system(
    time: Res<Time<Fixed>>,
    wind: Res<WindGrid>,
    mut query: Query<&mut Particle>,
) {
    let dt = time.delta_secs();
    for mut particle in query.iter_mut() {
        let x = particle.logical_pos.x.round() as i32;
        let y = particle.logical_pos.y.round() as i32;
        
        if wind.in_bounds(x as isize, y as isize) {
            let vector = wind.get(x as usize, y as usize);
            if vector.strength > 0 {
                let force = Vec3::new(vector.dx as f32, vector.dy as f32, 0.0) * (vector.strength as f32 / 255.0) * 10.0;
                // Heavier particles drift less
                let accel = force / (particle.weight.max(0.1));
                particle.velocity += accel * dt;
            }
        }
    }
}

pub fn particle_lifecycle_system(
    time: Res<Time<Fixed>>,
    mut grid: ResMut<CaGrid>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Particle)>,
) {
    let dt = time.delta_secs();
    for (entity, mut particle) in query.iter_mut() {
        particle.age += dt;
        
        let should_despawn = particle.age >= particle.max_age;
        let is_debris = matches!(particle.kind, ParticleKind::Debris | ParticleKind::Fire);
        
        if should_despawn {
            if is_debris {
                let x = particle.logical_pos.x.round() as i32;
                let y = particle.logical_pos.y.round() as i32;
                if grid.in_bounds(x as isize, y as isize) {
                    let mut cell = *grid.get(x as usize, y as usize);
                    if particle.kind == ParticleKind::Debris {
                        cell.solid = Some(omega_core::simulation::Solid::Ash);
                    }
                    grid.set(x as usize, y as usize, cell);
                }
            }
            commands.entity(entity).despawn();
        }
    }
}

pub fn particle_visual_cascade_system(
    mut query: Query<(&Particle, &VisualCascade, &mut Text2d, &mut TextColor, &mut Transform)>,
) {
    for (particle, cascade, mut text, mut color, mut transform) in query.iter_mut() {
        let progress = (particle.age / particle.max_age).clamp(0.0, 1.0);
        
        // 1. Glyph Morphing
        if !cascade.glyphs.is_empty() {
            let idx = (progress * (cascade.glyphs.len() - 1) as f32).floor() as usize;
            text.0 = cascade.glyphs[idx].to_string();
        }

        // 2. Color Gradient
        if !cascade.colors.is_empty() {
            let idx = (progress * (cascade.colors.len() - 1) as f32).floor() as usize;
            let next_idx = (idx + 1).min(cascade.colors.len() - 1);
            let t = (progress * (cascade.colors.len() - 1) as f32).fract();
            
            // Linear interpolation between two colors
            let c1 = cascade.colors[idx].to_srgba();
            let c2 = cascade.colors[next_idx].to_srgba();
            color.0 = Color::srgba(
                c1.red + (c2.red - c1.red) * t,
                c1.green + (c2.green - c1.green) * t,
                c1.blue + (c2.blue - c1.blue) * t,
                c1.alpha + (c2.alpha - c1.alpha) * t,
            );
        }

        // 3. Physical Shrinking
        let scale = cascade.initial_scale * (1.0 - progress * 0.5);
        transform.scale = Vec3::splat(scale);

        // 4. Rotation
        if cascade.rotate {
            transform.rotate_z(0.1);
        }
    }
}

pub fn projectile_movement_system(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Projectile, &mut Transform)>,
) {
    let dt = time.delta_secs();
    const GRAVITY: f32 = -35.0; 
    const VISUAL_Z_SCALE: f32 = 0.8;

    for (mut projectile, mut transform) in query.iter_mut() {
        if matches!(projectile.mode, TrajectoryMode::HighArc | TrajectoryMode::FlatArc) {
            projectile.velocity.z += GRAVITY * dt;
        }

        let v = projectile.velocity;
        projectile.logical_pos += v * dt;

        if projectile.logical_pos.z < 0.0 {
            projectile.logical_pos.z = 0.0;
        }

        transform.translation.x = projectile.logical_pos.x;
        transform.translation.y = projectile.logical_pos.y + (projectile.logical_pos.z * VISUAL_Z_SCALE);
        transform.translation.z = 100.0; 
    }
}

pub fn projectile_collision_system(
    mut commands: Commands,
    mut grid: ResMut<CaGrid>,
    mut projectiles: Query<(Entity, &Projectile)>,
    monsters: Query<(Entity, &omega_core::Position, &omega_core::Stats)>,
) {
    for (entity, projectile) in projectiles.iter_mut() {
        // 1. World Collision
        if projectile.logical_pos.z <= 0.1 { 
            let x = projectile.logical_pos.x.round() as i32;
            let y = projectile.logical_pos.y.round() as i32;

            if grid.in_bounds(x as isize, y as isize) {
                // Apply Impact to CA Grid
                if let Some(impact_cell) = projectile.element_impact {
                    let mut target = *grid.get(x as usize, y as usize);
                    target.heat = target.heat.saturating_add(impact_cell.heat);
                    target.wet = target.wet.saturating_add(impact_cell.wet);
                    target.pressure = target.pressure.saturating_add(impact_cell.pressure);
                    if target.gas.is_none() { target.gas = impact_cell.gas; }
                    
                    grid.set(x as usize, y as usize, target);

                    if target.pressure > 200 {
                         let event = DisplacementEvent {
                            origin_x: x as usize,
                            origin_y: y as usize,
                            heat: 255,
                            pressure: 255,
                            gas: target.gas,
                            radius: 3,
                            is_violent: true,
                        };
                        apply_explosive_displacement(&mut grid, &event);
                    }
                }
                commands.entity(entity).despawn();
                continue;
            }
        }
        
        // 2. Entity Collision
        for (_m_entity, m_pos, _m_stats) in monsters.iter() {
            let dx = projectile.logical_pos.x - m_pos.x as f32;
            let dy = projectile.logical_pos.y - m_pos.y as f32;
            let dist_sq = dx * dx + dy * dy;
            
            if dist_sq < 0.25 && projectile.logical_pos.z < 1.5 { // Flying check height
                // Apply damage/knockback (simplified for now)
                info!("Projectile hit monster at {:?}", m_pos);
                commands.entity(entity).despawn();
                break;
            }
        }
    }
}

pub fn projectile_interception_system(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Projectile)>,
) {
    let entities: Vec<_> = projectiles.iter().map(|(e, _)| e).collect();
    let mut to_despawn = std::collections::HashSet::new();

    for i in 0..entities.len() {
        for j in i + 1..entities.len() {
            let e1 = entities[i];
            let e2 = entities[j];
            
            if let Ok([p1, p2]) = projectiles.get_many_mut([e1, e2]) {
                let dist = p1.1.logical_pos.distance(p2.1.logical_pos);
                if dist < 0.5 {
                    info!("Projectile interception detected!");
                    // Intersection Logic
                    if p1.1.props.intensity == p2.1.props.intensity {
                        to_despawn.insert(e1);
                        to_despawn.insert(e2);
                    } else if p1.1.props.intensity > p2.1.props.intensity {
                        to_despawn.insert(e2);
                        // p1 deflects p2? For now just kill p2
                    } else {
                        to_despawn.insert(e1);
                    }
                }
            }
        }
    }

    for e in to_despawn {
        commands.entity(e).despawn();
    }
}

pub fn update_ca_cells(mut grid: ResMut<CaGrid>, wind: Res<WindGrid>, tick: Res<SimulationTick>) {
    let (w, h) = (grid.width(), grid.height());
    let is_reclaim_tick = tick.0 % 60 == 0;

    for y in 0..h {
        for x in 0..w {
            let cell = *grid.get(x, y);
            let neighbors = moore_neighbors(&grid, x, y);
            
            // 1. Core reactions and transitions
            let mut next = compute_next_cell(&cell, &neighbors);
            
            // 2. Wind effects
            let (after_wind, _displaced) = apply_wind(&grid, &wind, x, y, &next);
            next = after_wind;
            
            // 3. Decay and Nature Reclaims
            next = apply_full_decay_cycle(&next, is_reclaim_tick);
            
            grid.set(x, y, next);
        }
    }
}

pub fn process_explosions(mut grid: ResMut<CaGrid>) {
    let (w, h) = (grid.width(), grid.height());
    let mut explosions = Vec::new();

    for y in 0..h {
        for x in 0..w {
            if let Some(event) = check_explosion_trigger(grid.get(x, y), x, y) {
                explosions.push(event);
            }
        }
    }

    for event in explosions {
        apply_explosive_displacement(&mut grid, &event);
    }
}

pub fn environmental_behaviors(mut grid: ResMut<CaGrid>) {
    // Order matters: fire affects heat -> heat affects liquid evaporation -> gas rises from evaporation
    apply_fire_spread_bias(&mut grid);  // ENV-01: directional fire heat bias
    apply_liquid_flow(&mut grid);       // ENV-02: bottom-up liquid movement
    apply_gas_rise(&mut grid);          // ENV-03: top-down gas movement
}

pub fn swap_ca_buffers(mut grid: ResMut<CaGrid>) {
    grid.swap_buffers();
}
