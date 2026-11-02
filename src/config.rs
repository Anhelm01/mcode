//! Configuration for the Code-Matrix Engine.
//! Easily adjustable parameters for speed, colors, masks, and text sources.

use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppMode {
    Clock,    // Live Cyberpunk Clock (HH:MM) - Stock Default
    Sequence, // Custom user sequence / text / image
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteChoice {
    Cyberpunk,  // Electric Blue & Neon Pink (Default)
    Holo,       // Cyan -> Purple -> Pink (ponytail.dev style)
    Matrix,     // Phosphor Neon Green
    Amber,      // Warm CRT terminal orange/amber
    Monochrome, // High-contrast White & Gray
}

#[derive(Clone, Debug)]
pub enum SequenceItem {
    Text(String, Duration),
    Image(String, Duration),
}

#[derive(Clone, Debug)]
pub struct Config {
    /// Active operating mode
    pub mode: AppMode,
    /// Whether to show seconds in clock mode (HH:MM:SS)
    pub clock_seconds: bool,
    /// Target frames per second (e.g. 30 or 60)
    pub fps: u64,
    /// Color theme
    pub palette: PaletteChoice,
    /// Vertical scrolling speed of background code (rows per second)
    pub scroll_speed: f32,
    /// Vignette fade on edges (0.0 = none, 0.15 = soft fade)
    pub edge_fade: f32,
    /// Custom code/text source path (None = use built-in code)
    pub source_path: Option<String>,
    /// Sequence of masks to cycle through (for Sequence mode)
    pub sequence: Vec<SequenceItem>,
    /// Whether to loop the sequence indefinitely
    pub loop_sequence: bool,
    /// Speed of holographic light sweep
    pub sweep_speed: f32,
    /// Transition duration between shapes (seconds)
    pub switch_duration: f32,
    /// Whether random background cells twinkle/flash white (disabled by default)
    pub mod_twinkle: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: AppMode::Clock, // Stock default: Live Cyberpunk Clock
            clock_seconds: false,
            fps: 60,
            palette: PaletteChoice::Cyberpunk,
            scroll_speed: 0.0, // Static by default: code characters stay fixed in place
            edge_fade: 0.08,
            source_path: None,
            sequence: Vec::new(),
            loop_sequence: true,
            sweep_speed: 0.06,     // Slow, peaceful light sweep
            switch_duration: 2.2, // Extended 2.2s cinematic morph transition
            mod_twinkle: false,
        }
    }
}
