//! Background star twinkle effect.

use crate::core::buffer::ColorRgb;
use crate::fx::palette::Palette;
use crate::fx::scramble::Scramble;

pub struct Twinkle;

impl Twinkle {
    /// Returns an optional brightened color if the cell is currently "twinkling" as a background star.
    pub fn check(idx: usize, time: f32, palette: &Palette) -> Option<ColorRgb> {
        let star_seed = (idx as u32).wrapping_mul(7919);
        let cycle_time = time * 0.8 + Scramble::hash(star_seed, 9, 1);
        let cycle_floor = cycle_time.floor() as u32;
        let cycle_frac = cycle_time - cycle_time.floor();

        // 1 in 80 cells can become a star per cycle
        let is_candidate = Scramble::hash(star_seed, cycle_floor, 42) < 0.015;
        if is_candidate {
            // Triangular alpha pulse (0 -> 1 -> 0)
            let alpha = (cycle_frac * std::f32::consts::PI).sin();
            if alpha > 0.1 {
                let tw = palette.twinkle;
                let bg = palette.bg_dim;
                return Some(ColorRgb::new(
                    (bg.r as f32 + (tw.r as f32 - bg.r as f32) * (alpha * 0.75)) as u8,
                    (bg.g as f32 + (tw.g as f32 - bg.g as f32) * (alpha * 0.75)) as u8,
                    (bg.b as f32 + (tw.b as f32 - bg.b as f32) * (alpha * 0.75)) as u8,
                ));
            }
        }

        None
    }
}
