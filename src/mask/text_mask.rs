//! Pure ASCII text rasterization into terminal cell bitmasks.
//! Supports TrueType fonts via fontdue with multi-line layout and automatic system font discovery.

use std::fs;
use crate::core::grid::Grid;
use crate::mask::{BitMask, sampler::Sampler};

pub struct TextMaskRasterizer {
    font: Option<fontdue::Font>,
}

impl TextMaskRasterizer {
    pub fn new() -> Self {
        // Try to find a good system font
        let candidate_paths = [
            "/usr/share/fonts/TTF/JetBrainsMonoNerdFont-Bold.ttf",
            "/usr/share/fonts/TTF/JetBrainsMono-Bold.ttf",
            "/usr/share/fonts/liberation/LiberationSans-Bold.ttf",
            "/usr/share/fonts/noto/NotoSans-Bold.ttf",
            "/usr/share/fonts/TTF/DejaVuSans-Bold.ttf",
            "/usr/share/fonts/gsfonts/NimbusSans-Bold.otf",
        ];

        let mut font = None;
        for path in candidate_paths {
            if let Ok(bytes) = fs::read(path) {
                if let Ok(f) = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default()) {
                    font = Some(f);
                    break;
                }
            }
        }

        Self { font }
    }

    /// Rasterizes a text string (supports multiple lines separated by '\n') into a BitMask.
    pub fn rasterize(&self, text: &str, grid: Grid) -> BitMask {
        if let Some(ref font) = self.font {
            self.rasterize_with_font(text, grid, font)
        } else {
            self.rasterize_procedural_fallback(text, grid)
        }
    }

    fn rasterize_with_font(&self, text: &str, grid: Grid, font: &fontdue::Font) -> BitMask {
        let lines: Vec<&str> = text.lines().collect();
        if lines.is_empty() {
            return BitMask::new(grid, vec![false; grid.len()]);
        }

        let font_px = 110.0f32;
        let line_height_px = font_px * 1.18f32;

        struct LineData {
            glyphs: Vec<(fontdue::Metrics, Vec<u8>)>,
            width_px: f32,
        }

        let mut rendered_lines = Vec::new();
        let mut max_width_px = 0.0f32;

        for line in &lines {
            let mut line_glyphs = Vec::new();
            let mut line_w = 0.0f32;

            for ch in line.chars() {
                let (metrics, bitmap) = font.rasterize(ch, font_px);
                line_w += metrics.advance_width;
                line_glyphs.push((metrics, bitmap));
            }

            max_width_px = max_width_px.max(line_w);
            rendered_lines.push(LineData {
                glyphs: line_glyphs,
                width_px: line_w,
            });
        }

        if max_width_px <= 0.0 {
            return BitMask::new(grid, vec![false; grid.len()]);
        }

        let total_canvas_w = max_width_px.ceil() as usize;
        let total_canvas_h = ((lines.len() as f32) * line_height_px + font_px * 0.25).ceil() as usize;
        let mut canvas = vec![0u8; total_canvas_w * total_canvas_h];

        for (line_idx, line_data) in rendered_lines.into_iter().enumerate() {
            let mut pen_x = ((max_width_px - line_data.width_px) / 2.0).max(0.0);
            let baseline = (line_idx as f32 + 0.85) * line_height_px;

            for (metrics, bitmap) in line_data.glyphs {
                let gx = (pen_x + metrics.xmin as f32).max(0.0) as usize;
                let gy = (baseline - metrics.height as f32 - metrics.ymin as f32).max(0.0) as usize;

                for r in 0..metrics.height {
                    for c in 0..metrics.width {
                        let cy = gy + r;
                        let cx = gx + c;
                        if cx < total_canvas_w && cy < total_canvas_h {
                            let src_val = bitmap[r * metrics.width + c];
                            let dst_idx = cy * total_canvas_w + cx;
                            canvas[dst_idx] = canvas[dst_idx].max(src_val);
                        }
                    }
                }
                pen_x += metrics.advance_width;
            }
        }

        // Fit into terminal cell grid
        let (start_col, start_row, final_w, final_h) =
            Sampler::fit_center(grid, max_width_px, total_canvas_h as f32, 0.78);

        let mut bits = vec![false; grid.len()];

        for row in 0..grid.rows {
            let ry = row as f32;
            if ry < start_row || ry >= start_row + final_h {
                continue;
            }
            let norm_y = (ry - start_row) / final_h;
            let sample_y = ((norm_y * total_canvas_h as f32) as usize).min(total_canvas_h.saturating_sub(1));

            for col in 0..grid.cols {
                let rx = col as f32;
                if rx < start_col || rx >= start_col + final_w {
                    continue;
                }
                let norm_x = (rx - start_col) / final_w;
                let sample_x = ((norm_x * total_canvas_w as f32) as usize).min(total_canvas_w.saturating_sub(1));

                let alpha = canvas[sample_y * total_canvas_w + sample_x];
                if alpha > 80 {
                    let idx = grid.index(col, row);
                    bits[idx] = true;
                }
            }
        }

        BitMask::new(grid, bits)
    }

    /// Procedural fallback for bare environments
    fn rasterize_procedural_fallback(&self, text: &str, grid: Grid) -> BitMask {
        let mut bits = vec![false; grid.len()];
        let first_line = text.lines().next().unwrap_or(text);
        let (start_col, start_row, _w, _h) = Sampler::fit_center(grid, (first_line.len() * 8) as f32, 10.0, 0.6);
        let sc = start_col as usize;
        let sr = start_row as usize;

        for (i, ch) in first_line.chars().enumerate() {
            let offset_col = sc + i * 8 * 2;
            if ch != ' ' {
                for r in 0..8 {
                    for c in 0..12 {
                        let cur_c = (offset_col + c) as u16;
                        let cur_r = (sr + r) as u16;
                        if cur_c < grid.cols && cur_r < grid.rows {
                            bits[grid.index(cur_c, cur_r)] = true;
                        }
                    }
                }
            }
        }

        BitMask::new(grid, bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoticon_rasterization() {
        let rasterizer = TextMaskRasterizer::new();
        let grid = Grid::new(80, 24);
        for emo in ["o_0", "0_o", "._.", "x_x", "+_+"] {
            let mask = rasterizer.rasterize(emo, grid);
            let count = mask.bits.iter().filter(|&&b| b).count();
            assert!(count > 0, "Mask for '{}' has no active cells!", emo);
        }
    }

    #[test]
    fn test_multiline_text_rasterization() {
        let rasterizer = TextMaskRasterizer::new();
        let grid = Grid::new(100, 35);
        let sample = "HELLO\nCYBERPUNK";
        let mask = rasterizer.rasterize(sample, grid);
        let count = mask.bits.iter().filter(|&&b| b).count();
        assert!(count > 0, "Mask for multiline text has no active cells!");
    }
}
