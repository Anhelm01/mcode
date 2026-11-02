//! Terminal ASCII Code-Matrix Engine CLI (Cyberpunk Live Clock).
//! Pure monospace code characters with dynamic holographic morphing and mask illumination.

#![allow(dead_code)]

mod app;
mod config;
mod core;
mod fx;
mod mask;
mod timeline;

use std::time::Duration;
use clap::Parser;

use app::App;
use config::{AppMode, Config, PaletteChoice, SequenceItem};

#[derive(Parser, Debug)]
#[command(
    name = "mcode",
    version = "0.1.0",
    about = "Pure ASCII Cyberpunk Live Clock with Organic Melt animation (ponytail.dev style)"
)]
struct Args {
    /// Show seconds in clock mode (HH:MM:SS)
    #[arg(long, default_value_t = false)]
    seconds: bool,

    /// Single text to display (e.g. '98%' or 'o_0')
    #[arg(short, long)]
    text: Option<String>,

    /// Image silhouette file path to display as ASCII mask
    #[arg(short, long)]
    image: Option<String>,

    /// Custom sequence to cycle, e.g. "o_0:6, 0_o:6, ._.:6, x_x:6, +_+:6"
    #[arg(short = 's', long = "sequence", visible_alias = "sequense")]
    sequence: Option<String>,

    /// Color theme: cyberpunk, holo, matrix, amber, monochrome
    #[arg(short, long, default_value = "cyberpunk")]
    palette: String,

    /// Target frames per second (e.g. 30 or 60)
    #[arg(long, default_value_t = 60)]
    fps: u64,

    /// Background code scroll speed (rows per second, default 0.0 for static fixed grid)
    #[arg(long, default_value_t = 0.0)]
    speed: f32,

    /// Custom file path to use as background code stream
    #[arg(long)]
    source: Option<String>,

    /// Edge vignette fade factor (0.0 = none, 0.1 = soft)
    #[arg(long, default_value_t = 0.08)]
    edge_fade: f32,

    /// Duration of the smooth morph transition between shapes in seconds (default 2.2)
    #[arg(long, default_value_t = 2.2)]
    transition: f32,

    /// Enable twinkling white stars in the background (disabled by default)
    #[arg(long, default_value_t = false)]
    twinkle: bool,
}

fn parse_palette(s: &str) -> PaletteChoice {
    match s.to_lowercase().as_str() {
        "holo" => PaletteChoice::Holo,
        "matrix" => PaletteChoice::Matrix,
        "amber" => PaletteChoice::Amber,
        "monochrome" | "mono" => PaletteChoice::Monochrome,
        _ => PaletteChoice::Cyberpunk,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut config = Config::default();
    config.fps = args.fps;
    config.palette = parse_palette(&args.palette);
    config.scroll_speed = args.speed;
    config.source_path = args.source;
    config.edge_fade = args.edge_fade;
    config.switch_duration = args.transition;
    config.mod_twinkle = args.twinkle;
    config.clock_seconds = args.seconds;

    // Determine operating mode
    if args.text.is_some() || args.image.is_some() || args.sequence.is_some() {
        config.mode = AppMode::Sequence;
    } else {
        config.mode = AppMode::Clock; // Stock default
    }

    // Configure sequence if specified
    if let Some(txt) = args.text {
        config.sequence = vec![SequenceItem::Text(txt, Duration::from_secs(600))];
        config.loop_sequence = false;
    } else if let Some(img_path) = args.image {
        config.sequence = vec![SequenceItem::Image(img_path, Duration::from_secs(600))];
        config.loop_sequence = false;
    } else if let Some(seq_str) = args.sequence {
        let mut items = Vec::new();
        for part in seq_str.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let tokens: Vec<&str> = part.split(':').collect();
            if !tokens.is_empty() {
                let name = tokens[0].trim().to_string();
                let secs = tokens.get(1).and_then(|s| s.trim().parse::<u64>().ok()).unwrap_or(6);
                if name.ends_with(".png") || name.ends_with(".jpg") || name.ends_with(".jpeg") {
                    items.push(SequenceItem::Image(name, Duration::from_secs(secs)));
                } else {
                    items.push(SequenceItem::Text(name, Duration::from_secs(secs)));
                }
            }
        }
        if !items.is_empty() {
            config.sequence = items;
            config.loop_sequence = true;
        }
    }

    let mut app = App::new(config)?;
    app.run()?;

    Ok(())
}
