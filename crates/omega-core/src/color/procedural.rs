//! Procedural color generation for dynamic item rarities and effects.
//!
//! Provides utilities for generating visually distinct, accessible colors
//! at runtime using hue rotation and golden ratio conjugation.

use crate::color::ColorSpec;

/// Generator for sequences of distinct colors.
pub struct ProceduralPalette {
    hue: f32,
    saturation: f32,
    lightness: f32,
}

impl ProceduralPalette {
    /// Creates a new generator with starting HSL values.
    pub fn new(start_hue: f32, saturation: f32, lightness: f32) -> Self {
        Self {
            hue: start_hue % 1.0,
            saturation: saturation.clamp(0.0, 1.0),
            lightness: lightness.clamp(0.0, 1.0),
        }
    }

    /// Generates the next distinct color in the sequence.
    ///
    /// Uses the golden ratio conjugate (0.618033988749895) to ensure
    /// maximum perceived distance between consecutive colors.
    pub fn next_color(&mut self) -> ColorSpec {
        const GOLDEN_RATIO_CONJUGATE: f32 = 0.618_034;
        self.hue = (self.hue + GOLDEN_RATIO_CONJUGATE) % 1.0;

        let (r, g, b) = hsl_to_rgb(self.hue, self.saturation, self.lightness);
        ColorSpec::Rgb { r, g, b }
    }

    /// Generates a specific number of distinct colors.
    pub fn generate_n(&mut self, n: usize) -> Vec<ColorSpec> {
        (0..n).map(|_| self.next_color()).collect()
    }
}

/// Internal helper for HSL to RGB conversion.
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let (r, g, b) = if s == 0.0 {
        (l, l, l)
    } else {
        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        (hue_to_rgb(p, q, h + 1.0 / 3.0), hue_to_rgb(p, q, h), hue_to_rgb(p, q, h - 1.0 / 3.0))
    };

    ((r * 255.0).round() as u8, (g * 255.0).round() as u8, (b * 255.0).round() as u8)
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procedural_generation() {
        let mut palette = ProceduralPalette::new(0.0, 0.5, 0.5);
        let colors = palette.generate_n(5);

        assert_eq!(colors.len(), 5);
        // Verify they are distinct (at least not all same)
        assert_ne!(colors[0], colors[1]);
        assert_ne!(colors[1], colors[2]);
    }
}
