//! Animation primitives for semantic color animations.
//!
//! Provides types for defining color behavior over time, including
//! flashing, pulsing, and smooth transitions.

use crate::color::ColorSpec;
use serde::{Deserialize, Serialize};

/// Defines how a color behaves over time.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AnimationKind {
    /// A static color that doesn't change.
    Static(ColorSpec),
    /// Alternates between two colors at a fixed frequency.
    Flash {
        /// The two colors to alternate between.
        colors: (ColorSpec, ColorSpec),
        /// The frequency in Hertz (cycles per second).
        frequency: f32,
    },
    /// Smoothly interpolates between two colors.
    Pulse {
        /// The base color.
        base: ColorSpec,
        /// The target color to pulse toward.
        target: ColorSpec,
        /// The frequency in Hertz (cycles per second).
        frequency: f32,
    },
}

impl AnimationKind {
    /// Resolves the color at a specific timestamp.
    pub fn resolve_at(&self, time: f32) -> ColorSpec {
        match self {
            AnimationKind::Static(spec) => *spec,
            AnimationKind::Flash { colors, frequency } => {
                // Determine phase: 0 or 1
                let phase = ((time * frequency * 2.0) % 2.0).floor() as i32;
                if phase == 0 {
                    colors.0
                } else {
                    colors.1
                }
            }
            AnimationKind::Pulse { base, target, frequency } => {
                // Sine wave normalized to [0, 1]
                let t = (time * frequency * 2.0 * std::f32::consts::PI).sin() * 0.5 + 0.5;
                lerp_color_spec(*base, *target, t)
            }
        }
    }
}

/// Linearly interpolates between two ColorSpecs.
pub fn lerp_color_spec(a: ColorSpec, b: ColorSpec, t: f32) -> ColorSpec {
    // Only support RGB interpolation for now
    let (ar, ag, ab) = match a {
        ColorSpec::Rgb { r, g, b } => (r as f32, g as f32, b as f32),
        _ => {
            // Fallback for non-RGB specs: convert to RGB or just return A if not possible
            return a;
        }
    };

    let (br, bg, bb) = match b {
        ColorSpec::Rgb { r, g, b } => (r as f32, g as f32, b as f32),
        _ => return a,
    };

    ColorSpec::Rgb {
        r: (ar + (br - ar) * t).round() as u8,
        g: (ag + (bg - ag) * t).round() as u8,
        b: (ab + (bb - ab) * t).round() as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flash() {
        let white = ColorSpec::Rgb { r: 255, g: 255, b: 255 };
        let black = ColorSpec::Rgb { r: 0, g: 0, b: 0 };
        let anim = AnimationKind::Flash {
            colors: (white, black),
            frequency: 1.0, // 1 second period, 0.5s white, 0.5s black
        };

        assert_eq!(anim.resolve_at(0.0), white);
        assert_eq!(anim.resolve_at(0.4), white);
        assert_eq!(anim.resolve_at(1.1), white); // 1.1s is next cycle start
        assert_eq!(anim.resolve_at(0.6), black);
        assert_eq!(anim.resolve_at(1.6), black);
    }

    #[test]
    fn test_pulse() {
        let white = ColorSpec::Rgb { r: 255, g: 255, b: 255 };
        let black = ColorSpec::Rgb { r: 0, g: 0, b: 0 };
        let anim = AnimationKind::Pulse {
            base: white,
            target: black,
            frequency: 1.0,
        };

        // At t=0, sin(0)=0, result=(0*0.5)+0.5 = 0.5 (middle?)
        // Let's check the math: sin(time * freq * 2PI)
        // t=0: sin(0)=0 -> 0.5. Lerp(white, black, 0.5)
        let mid = anim.resolve_at(0.0);
        if let ColorSpec::Rgb { r, .. } = mid {
            assert!(r > 120 && r < 130);
        }

        // t=0.25 (1/4 period): sin(PI/2)=1 -> 1.0. Result = black
        assert_eq!(anim.resolve_at(0.25), black);
        
        // t=0.75 (3/4 period): sin(3PI/2)=-1 -> 0.0. Result = white
        assert_eq!(anim.resolve_at(0.75), white);
    }
}
