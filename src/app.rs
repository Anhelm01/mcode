//! Main application loop and frame compositor.
//! Clean, dedicated terminal engine for Live Cyberpunk Clock and Custom Sequences.

use std::io::{self, stdout};
use std::time::{Duration, Instant};
use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        size as term_size,
    },
};

use crate::config::{AppMode, Config};
use crate::core::buffer::ColorRgb;
use crate::core::{Cell, ClockTime, DoubleBuffer, Grid, SourceStream};
use crate::fx::{Melt, Palette, Shimmer, Twinkle, Vignette};
use crate::mask::{BitMask, Mask, text_mask::TextMaskRasterizer};
use crate::timeline::Timeline;

pub struct App {
    config: Config,
    palette: Palette,
    source: SourceStream,
    rasterizer: TextMaskRasterizer,
    grid: Grid,
    buffer: DoubleBuffer,
    timeline: Timeline,
    running: bool,
    start_time: Instant,

    // Dynamic state for Clock mode
    current_label: String,
    current_mask: BitMask,
    prev_mask: BitMask,
    transition_start: Instant,
    step_idx: u32,
}

impl App {
    pub fn new(config: Config) -> io::Result<Self> {
        let (cols, rows) = term_size()?;
        let grid = Grid::new(cols, rows);
        let buffer = DoubleBuffer::new(grid);
        let palette = Palette::from_choice(&config.palette);
        let source = SourceStream::new(config.source_path.as_deref());
        let rasterizer = TextMaskRasterizer::new();

        let timeline = Timeline::build(
            &config.sequence,
            grid,
            config.loop_sequence,
            config.switch_duration,
        );

        // Determine initial label and mask based on mode
        let current_label = match config.mode {
            AppMode::Clock => ClockTime::now_str(config.clock_seconds),
            AppMode::Sequence => String::new(),
        };

        let current_mask = if !current_label.is_empty() {
            rasterizer.rasterize(&current_label, grid)
        } else {
            BitMask::new(grid, vec![false; grid.len()])
        };

        let prev_mask = current_mask.clone();

        Ok(Self {
            config,
            palette,
            source,
            rasterizer,
            grid,
            buffer,
            timeline,
            running: false,
            start_time: Instant::now(),
            current_label,
            current_mask,
            prev_mask,
            transition_start: Instant::now() - Duration::from_secs(10),
            step_idx: 0,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut out = stdout();
        execute!(out, EnterAlternateScreen, Hide)?;

        self.running = true;
        self.start_time = Instant::now();

        let frame_duration = Duration::from_nanos(1_000_000_000 / self.config.fps);

        while self.running {
            let frame_start = Instant::now();

            self.handle_events()?;
            if !self.running {
                break;
            }

            self.render_frame(&mut out)?;

            let elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }

        execute!(out, LeaveAlternateScreen, crossterm::cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        while event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                        self.running = false;
                        return Ok(());
                    }
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                            self.running = false;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
                Event::Resize(cols, rows) => {
                    let new_grid = Grid::new(cols, rows);
                    self.grid = new_grid;
                    self.buffer.resize(new_grid);
                    self.timeline = Timeline::build(
                        &self.config.sequence,
                        new_grid,
                        self.config.loop_sequence,
                        self.config.switch_duration,
                    );
                    if !self.current_label.is_empty() {
                        self.current_mask = self.rasterizer.rasterize(&self.current_label, new_grid);
                        self.prev_mask = self.current_mask.clone();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn render_frame<W: io::Write>(&mut self, out: &mut W) -> io::Result<()> {
        let t = self.start_time.elapsed().as_secs_f32();
        let cols = self.grid.cols;
        let rows = self.grid.rows;

        // Update live clock
        if self.config.mode == AppMode::Clock {
            let now_str = ClockTime::now_str(self.config.clock_seconds);
            if now_str != self.current_label {
                self.prev_mask = self.current_mask.clone();
                self.current_mask = self.rasterizer.rasterize(&now_str, self.grid);
                self.current_label = now_str;
                self.transition_start = Instant::now();
                self.step_idx = self.step_idx.wrapping_add(1);
            }
        }

        let switch_dur = self.config.switch_duration;
        let sweep_spd = self.config.sweep_speed;
        let scroll_offset = if self.config.scroll_speed > 0.0 {
            (t * self.config.scroll_speed) as usize
        } else {
            0
        };

        // Determine active and previous masks + local time
        let (prev_mask, cur_mask, local_t, step_idx, bounds) = match self.config.mode {
            AppMode::Clock => {
                let local_t = self.transition_start.elapsed().as_secs_f32();
                (
                    &self.prev_mask,
                    &self.current_mask,
                    local_t,
                    self.step_idx,
                    self.current_mask.bounds(),
                )
            }
            AppMode::Sequence => {
                let (prev_idx, cur_idx, local_t, step_idx) = self.timeline.state_at(t);
                let prev_entry = &self.timeline.entries[prev_idx];
                let cur_entry = &self.timeline.entries[cur_idx];
                (
                    &prev_entry.mask,
                    &cur_entry.mask,
                    local_t,
                    step_idx,
                    cur_entry.mask.bounds(),
                )
            }
        };

        for row in 0..rows {
            let row_src_offset = (row as usize + scroll_offset) * (cols as usize);

            for col in 0..cols {
                let idx = self.grid.index(col, row);
                let was_active = prev_mask.contains(col, row);
                let will_be_active = cur_mask.contains(col, row);
                let raw_ch = self.source.char_at(row_src_offset + col as usize);

                // Organic 2D Perlin noise ink dissolve (Melt)
                let eval = Melt::evaluate(
                    col,
                    row,
                    was_active,
                    will_be_active,
                    local_t,
                    switch_dur,
                    step_idx,
                    t,
                    raw_ch,
                );

                let intensity = eval.intensity;
                let ch = if eval.ch == ' ' && intensity > 0.5 {
                    ' '
                } else {
                    eval.ch
                };

                // Continuous TrueColor blending
                let active_fg = Shimmer::calculate(col, row, bounds, t, sweep_spd, &self.palette);
                let dim_fg = if self.config.mod_twinkle {
                    Twinkle::check(idx, t, &self.palette).unwrap_or_else(|| {
                        Vignette::apply(col, row, cols, rows, self.config.edge_fade, self.palette.bg_dim)
                    })
                } else {
                    Vignette::apply(col, row, cols, rows, self.config.edge_fade, self.palette.bg_dim)
                };

                let mut fg = dim_fg.lerp(active_fg, intensity);

                // Extra subtle burning edge glow
                if eval.glow > 0.05 {
                    fg = fg.lerp(ColorRgb::new(255, 255, 255), eval.glow);
                }

                let cell = Cell {
                    ch,
                    fg,
                    bg: None, // Keep natural transparent background without boxy artifacts
                    bold: intensity > 0.65,
                };

                self.buffer.set_cell(col, row, cell);
            }
        }

        self.buffer.flush(out)?;
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = execute!(stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
        let _ = disable_raw_mode();
    }
}
