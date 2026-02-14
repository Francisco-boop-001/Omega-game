use bevy::prelude::*;
use super::projectiles::Projectile;
use super::particles::{Particle, ParticleKind, VisualCascade};
use omega_core::simulation::displacement::DisplacementEvent;

pub fn trail_emitter_system(
    mut commands: Commands,
    query: Query<&Projectile>,
) {
    for projectile in query.iter() {
        // Spawn trail particles along the path (simplified to just current pos)
        commands.spawn((
            Particle {
                logical_pos: projectile.logical_pos,
                velocity: Vec3::new(0.0, 0.0, 1.0), // Slight upward drift
                age: 0.0,
                max_age: 1.0,
                weight: 0.5,
                kind: ParticleKind::Smoke,
            },
            VisualCascade {
                glyphs: vec!['*', '.', ' '],
                colors: vec![Color::srgba(0.5, 0.5, 0.5, 0.8), Color::srgba(0.2, 0.2, 0.2, 0.0)],
                rotate: false,
                initial_scale: 0.5,
            },
            Text2d::new("*"),
            Transform::from_translation(Vec3::ZERO),
        ));
    }
}

pub fn explosion_emitter_system(
    mut commands: Commands,
    mut events: EventReader<DisplacementEvent>,
) {
    for event in events.read() {
        let origin = Vec3::new(event.origin_x as f32, event.origin_y as f32, 0.5);
        for _ in 0..20 {
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            let speed = rand::random::<f32>() * 5.0 + 2.0;
            let velocity = Vec3::new(angle.cos() * speed, angle.sin() * speed, rand::random::<f32>() * 5.0);
            
            commands.spawn((
                Particle {
                    logical_pos: origin,
                    velocity,
                    age: 0.0,
                    max_age: 2.0,
                    weight: 1.0,
                    kind: ParticleKind::Fire,
                },
                VisualCascade {
                    glyphs: vec!['@', '*', '.', ' '],
                    colors: vec![Color::srgba(1.0, 1.0, 1.0, 1.0), Color::srgba(1.0, 0.5, 0.0, 1.0), Color::srgba(0.5, 0.0, 0.0, 0.0)],
                    rotate: true,
                    initial_scale: 1.0,
                },
                Text2d::new("@"),
                Transform::from_translation(origin),
            ));
        }
    }
}
