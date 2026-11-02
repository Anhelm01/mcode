//! Sampling and geometry utilities for converting shapes into terminal cell coordinates.

use crate::core::grid::Grid;

pub struct Sampler;

impl Sampler {
    /// Terminal cells are typically ~1:2 (width is 0.5 of height).
    /// To keep shapes geometrically proportional:
    /// `aspect_ratio = 2.0` compensates for terminal character elongation.
    pub const TERMINAL_ASPECT_RATIO: f32 = 2.0;

    /// Centers and scales a normalized bounding box into terminal grid coordinates.
    pub fn fit_center(
        grid: Grid,
        content_width: f32,
        content_height: f32,
        scale: f32,
    ) -> (f32, f32, f32, f32) {
        // Adjust width by terminal aspect ratio so circle remains circular
        let terminal_w = content_width * Self::TERMINAL_ASPECT_RATIO;
        let terminal_h = content_height;

        let scale_w = (grid.cols as f32 * scale) / terminal_w;
        let scale_h = (grid.rows as f32 * scale) / terminal_h;
        let best_scale = scale_w.min(scale_h);

        let final_w = terminal_w * best_scale;
        let final_h = terminal_h * best_scale;

        let start_x = ((grid.cols as f32 - final_w) / 2.0).max(0.0);
        let start_y = ((grid.rows as f32 - final_h) / 2.0).max(0.0);

        (start_x, start_y, final_w, final_h)
    }
}
