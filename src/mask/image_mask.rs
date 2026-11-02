//! Pure ASCII image silhouette rasterizer for terminal cell masks.

use std::path::Path;
use image::GenericImageView;
use crate::core::grid::Grid;
use crate::mask::{BitMask, sampler::Sampler};

pub struct ImageMaskRasterizer;

impl ImageMaskRasterizer {
    /// Loads an image from disk and converts it into a terminal cell BitMask.
    pub fn rasterize(path: &str, grid: Grid) -> Result<BitMask, String> {
        let img = image::open(Path::new(path))
            .map_err(|e| format!("Failed to open image '{}': {}", path, e))?;

        let (img_w, img_h) = img.dimensions();
        if img_w == 0 || img_h == 0 {
            return Ok(BitMask::new(grid, vec![false; grid.len()]));
        }

        let (start_col, start_row, final_w, final_h) =
            Sampler::fit_center(grid, img_w as f32, img_h as f32, 0.75);

        let mut bits = vec![false; grid.len()];

        for row in 0..grid.rows {
            let ry = row as f32;
            if ry < start_row || ry >= start_row + final_h {
                continue;
            }
            let norm_y = (ry - start_row) / final_h;
            let sample_y = ((norm_y * img_h as f32) as u32).min(img_h.saturating_sub(1));

            for col in 0..grid.cols {
                let rx = col as f32;
                if rx < start_col || rx >= start_col + final_w {
                    continue;
                }
                let norm_x = (rx - start_col) / final_w;
                let sample_x = ((norm_x * img_w as f32) as u32).min(img_w.saturating_sub(1));

                let pixel = img.get_pixel(sample_x, sample_y);
                // pixel[0..3] = RGBA
                let r = pixel[0] as f32;
                let g = pixel[1] as f32;
                let b = pixel[2] as f32;
                let a = pixel[3] as f32 / 255.0;

                // Relative luminance (or alpha threshold)
                let luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255.0;

                // Active if non-transparent and bright (or opaque ink)
                if a > 0.3 && (luminance > 0.4 || a > 0.8) {
                    let idx = grid.index(col, row);
                    bits[idx] = true;
                }
            }
        }

        Ok(BitMask::new(grid, bits))
    }
}
