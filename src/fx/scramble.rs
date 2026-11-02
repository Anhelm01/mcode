//! Smooth transition wave, easing curves, and refined transition glyphs.

pub struct Scramble;

impl Scramble {
    /// Sleek, minimalist transition runes that feel cybernetic rather than noisy
    pub const TRANSITION_GLYPHS: &'static [char] = &[
        '·', ':', '+', 'x', 'o', '0', '1', '*', '◇',
    ];

    /// Fast deterministic integer hash returning normalized float [0.0, 1.0)
    #[inline]
    pub fn hash(a: u32, b: u32, c: u32) -> f32 {
        let mut h = a
            .wrapping_mul(374761393)
            .wrapping_add(b.wrapping_mul(668265263))
            .wrapping_add(c.wrapping_mul(2147483647));
        h = (h ^ (h >> 13)).wrapping_mul(1274126177);
        let res = (h ^ (h >> 16)) as f32 / 4294967295.0;
        res.clamp(0.0, 1.0)
    }

    /// Spatial wave arrival delay: sweeps smoothly across the screen from left to right
    /// with subtle organic variance, replacing chaotic white-noise blinking.
    #[inline]
    pub fn wave_delay(col: u16, row: u16, cols: u16, rows: u16, seed: u32) -> f32 {
        let fx = col as f32 / (cols as f32).max(1.0);
        let fy = row as f32 / (rows as f32).max(1.0);

        // Sweeps horizontally with a gentle diagonal tilt
        let wave_pos = fx * 0.70 + fy * 0.15;
        // Mild organic jitter so cells don't move in a rigid straight line
        let jitter = Self::hash(col as u32, row as u32, seed) * 0.15;

        (wave_pos + jitter).clamp(0.0, 1.0)
    }

    /// Standard cubic Hermite interpolation (smoothstep) for silky-smooth fading
    #[inline]
    pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Calm glyph switcher: clocked at a gentle 4-5 Hz rate instead of rapid flicker
    #[inline]
    pub fn calm_glyph(idx: usize, time: f32) -> char {
        let step = (time * 4.5) as usize;
        let choice = (idx.wrapping_mul(13).wrapping_add(step)) % Self::TRANSITION_GLYPHS.len();
        Self::TRANSITION_GLYPHS[choice]
    }
}
