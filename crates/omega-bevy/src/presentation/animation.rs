use bevy::prelude::*;

use super::UiReadabilityConfig;

#[derive(Resource, Default)]
pub struct UiMotionState {
    pub frame: u64,
    pub pulse01: f32,
}

pub fn advance_ui_motion(
    time: Res<Time>,
    readability: Res<UiReadabilityConfig>,
    mut motion: ResMut<UiMotionState>,
) {
    motion.frame = motion.frame.wrapping_add(1);
    if readability.reduced_motion {
        motion.pulse01 = 0.5;
        return;
    }
    let elapsed = time.elapsed_secs();
    motion.pulse01 = ((elapsed * 1.8).sin() + 1.0) * 0.5;
}
