//! Vignette edge fading for the terminal grid.

use crate::core::buffer::ColorRgb;

pub struct Vignette;

impl Vignette {
    /// Applies a smooth edge fade to a given color based on proximity to grid borders.
    pub fn apply(
        col: u16,
        row: u16,
        cols: u16,
        rows: u16,
        fade_amount: f32,
        color: ColorRgb,
    ) -> ColorRgb {
        if fade_amount <= 0.001 {
            return color;
        }

        let fx = col as f32 / (cols as f32).max(1.0);
        let fy = row as f32 / (rows as f32).max(1.0);

        let dist_x = (fx.min(1.0 - fx) / fade_amount).clamp(0.0, 1.0);
        let dist_y = (fy.min(1.0 - fy) / fade_amount).clamp(0.0, 1.0);
        // Soft vignette: never drops below 70% brightness, so background code remains clearly visible everywhere
        let factor = 0.70 + 0.30 * dist_x.min(dist_y);

        ColorRgb::new(
            (color.r as f32 * factor).round() as u8,
            (color.g as f32 * factor).round() as u8,
            (color.b as f32 * factor).round() as u8,
        )
    }
}
