//! TrueColor RGB palettes for the Code-Matrix Engine.

use crate::config::PaletteChoice;
use crate::core::buffer::ColorRgb;

#[derive(Clone, Debug)]
pub struct Palette {
    pub name: &'static str,
    pub stops: Vec<ColorRgb>,
    pub bg_dim: ColorRgb,
    pub bg_active_tint: Option<ColorRgb>,
    pub twinkle: ColorRgb,
}

impl Palette {
    pub fn from_choice(choice: &PaletteChoice) -> Self {
        match choice {
            PaletteChoice::Holo => Self::holo(),
            PaletteChoice::Matrix => Self::matrix(),
            PaletteChoice::Amber => Self::amber(),
            PaletteChoice::Cyberpunk => Self::cyberpunk(),
            PaletteChoice::Monochrome => Self::monochrome(),
        }
    }

    /// Ponytail.dev signature holographic theme: Cyan -> Purple -> Pink
    pub fn holo() -> Self {
        Self {
            name: "Holo",
            stops: vec![
                ColorRgb::new(127, 227, 245), // Electric Cyan
                ColorRgb::new(180, 156, 255), // Soft Purple
                ColorRgb::new(255, 159, 216), // Pastel Pink
            ],
            bg_dim: ColorRgb::new(48, 45, 42),
            bg_active_tint: None,
            twinkle: ColorRgb::new(245, 235, 220),
        }
    }

    /// Classic phosphor neon green terminal
    pub fn matrix() -> Self {
        Self {
            name: "Matrix",
            stops: vec![
                ColorRgb::new(30, 255, 120),
                ColorRgb::new(140, 255, 190),
                ColorRgb::new(60, 220, 90),
            ],
            bg_dim: ColorRgb::new(20, 44, 26),
            bg_active_tint: None,
            twinkle: ColorRgb::new(200, 255, 220),
        }
    }

    /// Warm amber CRT vintage terminal
    pub fn amber() -> Self {
        Self {
            name: "Amber",
            stops: vec![
                ColorRgb::new(255, 165, 0),
                ColorRgb::new(255, 215, 70),
                ColorRgb::new(240, 130, 10),
            ],
            bg_dim: ColorRgb::new(52, 34, 16),
            bg_active_tint: None,
            twinkle: ColorRgb::new(255, 240, 190),
        }
    }

    /// Signature Cyberpunk palette: Electric Cyan, Neon Pink, Deep Violet
    pub fn cyberpunk() -> Self {
        Self {
            name: "Cyberpunk",
            stops: vec![
                ColorRgb::new(0, 240, 255),  // Electric Cyan
                ColorRgb::new(255, 0, 128),  // Hot Neon Pink
                ColorRgb::new(190, 0, 255),  // Neon Violet
            ],
            // Subtle, atmospheric code texture sitting comfortably in the background
            bg_dim: ColorRgb::new(36, 40, 56),
            bg_active_tint: None,
            twinkle: ColorRgb::new(255, 240, 255),
        }
    }

    pub fn monochrome() -> Self {
        Self {
            name: "Monochrome",
            stops: vec![
                ColorRgb::new(255, 255, 255),
                ColorRgb::new(190, 190, 190),
                ColorRgb::new(240, 240, 240),
            ],
            bg_dim: ColorRgb::new(45, 45, 45),
            bg_active_tint: Some(ColorRgb::new(20, 20, 20)),
            twinkle: ColorRgb::new(255, 255, 255),
        }
    }

    /// Samples the gradient at normalized position t (0.0..1.0, wraps around)
    pub fn sample_gradient(&self, t: f32) -> ColorRgb {
        if self.stops.is_empty() {
            return ColorRgb::new(255, 255, 255);
        }
        if self.stops.len() == 1 {
            return self.stops[0];
        }

        let wrapped = t.rem_euclid(1.0);
        let n_segments = self.stops.len() - 1;
        let scaled = wrapped * n_segments as f32;
        let idx = (scaled.floor() as usize).min(n_segments - 1);
        let frac = scaled - idx as f32;

        let c1 = self.stops[idx];
        let c2 = self.stops[idx + 1];

        ColorRgb::new(
            (c1.r as f32 + (c2.r as f32 - c1.r as f32) * frac).round() as u8,
            (c1.g as f32 + (c2.g as f32 - c1.g as f32) * frac).round() as u8,
            (c1.b as f32 + (c2.b as f32 - c1.b as f32) * frac).round() as u8,
        )
    }
}
