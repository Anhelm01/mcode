//! Dynamic holographic color calculation and moving light beam sweep.

use crate::core::buffer::ColorRgb;
use crate::fx::palette::Palette;

pub struct Shimmer;

impl Shimmer {
    /// Evaluates the holographic color at a given cell coordinate with time t.
    /// Uses ponytail.dev formula for wave oscillation and moving light streak.
    pub fn calculate(
        col: u16,
        row: u16,
        bounds: (u16, u16, u16, u16),
        time: f32,
        sweep_speed: f32,
        palette: &Palette,
    ) -> ColorRgb {
        let (min_col, min_row, max_col, max_row) = bounds;
        let width = (max_col.saturating_sub(min_col) as f32).max(1.0);
        let height = (max_row.saturating_sub(min_row) as f32).max(1.0);

        let fx = (col.saturating_sub(min_col) as f32) / width;
        let fy = (row.saturating_sub(min_row) as f32) / height;

        // Wave phase from ponytail.dev:
        // fx * 0.7 + fy * 0.3 + 0.06 * sin(t * 0.35 + y * 0.05)
        let phase = fx * 0.7 + fy * 0.3 + 0.06 * (time * 0.35 + row as f32 * 0.05).sin();
        let base_color = palette.sample_gradient(phase);

        // Moving diagonal glint / sweep:
        let sweep_cycle = (time * sweep_speed).rem_euclid(1.6) - 0.3;
        let d = (fx - fy * 0.45) - sweep_cycle;
        let glint = 0.5 * (-d * d / 0.005).exp();

        // Blend with white if glint is active
        if glint > 0.02 {
            ColorRgb::new(
                (base_color.r as f32 + (255.0 - base_color.r as f32) * glint).min(255.0) as u8,
                (base_color.g as f32 + (255.0 - base_color.g as f32) * glint).min(255.0) as u8,
                (base_color.b as f32 + (255.0 - base_color.b as f32) * glint).min(255.0) as u8,
            )
        } else {
            base_color
        }
    }
}
