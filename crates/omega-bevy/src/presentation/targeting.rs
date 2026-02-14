use bevy::prelude::*;
use omega_core::simulation::{CaGrid, TrajectoryMode};

#[derive(Resource, Default)]
pub struct TargetingState {
    pub active: bool,
    pub start: Vec3,
    pub target: Vec3,
    pub mode: TrajectoryMode,
    pub projected_path: Vec<Vec3>,
}

pub fn update_targeting_visualization(
    mut gizmos: Gizmos,
    targeting: Res<TargetingState>,
    grid: Res<CaGrid>,
) {
    if !targeting.active {
        return;
    }

    // Visualize Path (Temporal Ghost)
    for (i, pos) in targeting.projected_path.iter().enumerate() {
        // Pulse effect
        let pulse = (i as f32 * 0.1).sin() * 0.5 + 0.5;
        let color = Color::srgba(1.0, 0.5, 0.0, 0.5 + pulse * 0.5); // Orange for fire

        // Draw ghost glyph placeholder
        gizmos.sphere(Isometry3d::from_translation(*pos), 0.15, color);
    }

    // Visualize Blast Radius & Reaction Preview
    let tx = targeting.target.x.round() as i32;
    let ty = targeting.target.y.round() as i32;

    if grid.in_bounds(tx as isize, ty as isize) {
        // Draw Blast Radius (linear falloff visualization)
        const MAX_RADIUS: f32 = 3.0;
        for r in 1..=(MAX_RADIUS as i32) {
            let alpha = 1.0 - (r as f32 / MAX_RADIUS);
            gizmos.circle(
                Isometry3d::from_translation(targeting.target + Vec3::Z * 0.01), // Slightly above ground
                r as f32,
                Color::srgba(1.0, 0.2, 0.0, alpha * 0.4),
            );
        }

        if let Some(cell) = grid.get_checked(tx as usize, ty as usize) {
            // Reaction Logic Preview
            if cell.wet > 100 {
                // Predict Steam
                gizmos.rect(
                    Isometry3d::from_translation(targeting.target + Vec3::Y * 0.5),
                    Vec2::splat(0.8),
                    Color::srgba(0.8, 0.8, 1.0, 0.8),
                );
            }
        }
    }
}

pub fn update_projected_path(mut targeting: ResMut<TargetingState>) {
    if !targeting.active {
        return;
    }

    targeting.projected_path.clear();

    let mut current_pos = targeting.start;
    let mut velocity = (targeting.target - targeting.start).normalize() * 15.0; // Fixed speed for preview

    if matches!(targeting.mode, TrajectoryMode::HighArc | TrajectoryMode::FlatArc) {
        // Tune arc to hit target precisely? For now, just a rough parabolic preview
        velocity.z = if targeting.mode == TrajectoryMode::HighArc { 15.0 } else { 5.0 };
    }

    const DT: f32 = 0.05;
    const GRAVITY: f32 = -35.0;

    for _ in 0..100 {
        targeting.projected_path.push(current_pos);

        if matches!(targeting.mode, TrajectoryMode::HighArc | TrajectoryMode::FlatArc) {
            velocity.z += GRAVITY * DT;
        }

        current_pos += velocity * DT;

        if current_pos.z <= 0.0 {
            targeting.projected_path.push(current_pos);
            break;
        }

        // Stop if we hit the target ground area
        if current_pos.distance(targeting.target) < 0.5 {
            break;
        }
    }
}
