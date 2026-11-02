//! Organic 2D Perlin noise ink dissolve transition (Melt).
//! The shape dissolves and emerges through natural fluid noise contours
//! with an illuminated glowing burning edge.

use crate::fx::scramble::Scramble;

pub struct MeltResult {
    /// Normalized intensity 0.0 (dim background) to 1.0 (full illumination)
    pub intensity: f32,
    /// Character to display in this cell
    pub ch: char,
    /// Additional white/glow boost factor (0.0 .. 1.0) along the burning edge
    pub glow: f32,
}

pub struct Melt;

impl Melt {
    /// Evaluates the organic noise ink dissolve for a cell.
    pub fn evaluate(
        col: u16,
        row: u16,
        was_active: bool,
        will_be_active: bool,
        local_t: f32,
        switch_dur: f32,
        step_idx: u32,
        t: f32,
        raw_ch: char,
    ) -> MeltResult {
        // Cells shared between both shapes stay solid with 0 flicker
        if was_active && will_be_active {
            return MeltResult {
                intensity: 1.0,
                ch: raw_ch,
                glow: 0.0,
            };
        }
        if !was_active && !will_be_active {
            return MeltResult {
                intensity: 0.0,
                ch: raw_ch,
                glow: 0.0,
            };
        }

        // If transition is complete, snap to destination
        if local_t >= switch_dur {
            let active = will_be_active;
            return MeltResult {
                intensity: if active { 1.0 } else { 0.0 },
                ch: raw_ch,
                glow: 0.0,
            };
        }

        let progress = (local_t / switch_dur.max(0.1)).clamp(0.0, 1.0);

        // Sample 2D Perlin noise with aspect-ratio compensation (col is ~0.5 of row height)
        let nx = col as f32 * 0.09;
        let ny = row as f32 * 0.18;
        let noise_val = perlin_2d(nx + (step_idx as f32 * 17.3), ny + (step_idx as f32 * 31.7));

        // Threshold boundary: noise ranges 0.0 .. 1.0
        let threshold = progress;
        let diff = noise_val - threshold;

        // Balanced transition edge width (golden middle between sharp 0.18 and blurry 0.35)
        let edge_width = 0.25;
        let dist = diff.abs();

        if !was_active && will_be_active {
            // Emerging shape
            if noise_val <= threshold {
                let cell_age = (threshold - noise_val) / edge_width;
                let intensity = Scramble::smoothstep(0.0, 1.0, cell_age);
                let glow = (1.0 - cell_age).clamp(0.0, 1.0) * 0.38;
                let ch = if cell_age < 0.35 {
                    Scramble::calm_glyph(col as usize + row as usize, t)
                } else {
                    raw_ch
                };
                MeltResult { intensity, ch, glow }
            } else {
                MeltResult { intensity: 0.0, ch: raw_ch, glow: 0.0 }
            }
        } else {
            // Dissolving departing shape
            if noise_val >= threshold {
                let cell_life = (noise_val - threshold) / edge_width;
                let intensity = Scramble::smoothstep(0.0, 1.0, cell_life);
                let glow = (1.0 - cell_life).clamp(0.0, 1.0) * 0.38;
                let ch = if dist < edge_width * 0.4 {
                    Scramble::calm_glyph(col as usize + row as usize, t)
                } else {
                    raw_ch
                };
                MeltResult { intensity, ch, glow }
            } else {
                MeltResult { intensity: 0.0, ch: raw_ch, glow: 0.0 }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Simple, fast, deterministic 2D Perlin Noise (zero external dependencies)
// ----------------------------------------------------------------------------
fn perlin_2d(x: f32, y: f32) -> f32 {
    let xi0 = x.floor() as i32;
    let yi0 = y.floor() as i32;
    let xf = x - x.floor();
    let yf = y - y.floor();

    let u = fade(xf);
    let v = fade(yf);

    let g00 = grad(hash_coord(xi0, yi0), xf, yf);
    let g10 = grad(hash_coord(xi0 + 1, yi0), xf - 1.0, yf);
    let g01 = grad(hash_coord(xi0, yi0 + 1), xf, yf - 1.0);
    let g11 = grad(hash_coord(xi0 + 1, yi0 + 1), xf - 1.0, yf - 1.0);

    let x1 = g00 + u * (g10 - g00);
    let x2 = g01 + u * (g11 - g01);
    let res = x1 + v * (x2 - x1);

    // Normalize from [-1, 1] to [0, 1]
    ((res * 0.7071) + 1.0) * 0.5
}

#[inline]
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline]
fn hash_coord(x: i32, y: i32) -> u32 {
    let mut h = (x as u32).wrapping_mul(374761393) ^ (y as u32).wrapping_mul(668265263);
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    h ^ (h >> 16)
}

#[inline]
fn grad(hash: u32, x: f32, y: f32) -> f32 {
    match hash & 7 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        3 => -x - y,
        4 => x,
        5 => -x,
        6 => y,
        _ => -y,
    }
}
