//! High-performance double-buffered terminal screen renderer.
//! Eliminates tearing and flicker by only writing updated cells with minimal ANSI sequences.

use std::io::{self, Write};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use crate::core::grid::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub fn to_crossterm(self) -> Color {
        Color::Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    /// Linearly interpolates between two RGB colors (0.0 = self, 1.0 = other)
    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t).round() as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t).round() as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t).round() as u8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: ColorRgb,
    pub bg: Option<ColorRgb>,
    pub bold: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: ColorRgb::new(0, 0, 0),
            bg: None,
            bold: false,
        }
    }
}

pub struct DoubleBuffer {
    grid: Grid,
    current: Vec<Cell>,
    previous: Vec<Cell>,
    full_redraw: bool,
}

impl DoubleBuffer {
    pub fn new(grid: Grid) -> Self {
        let size = grid.len();
        Self {
            grid,
            current: vec![Cell::default(); size],
            previous: vec![Cell::default(); size],
            full_redraw: true,
        }
    }

    pub fn resize(&mut self, new_grid: Grid) {
        if self.grid != new_grid {
            self.grid = new_grid;
            let size = new_grid.len();
            self.current = vec![Cell::default(); size];
            self.previous = vec![Cell::default(); size];
            self.full_redraw = true;
        }
    }

    pub fn grid(&self) -> Grid {
        self.grid
    }

    #[inline]
    pub fn set_cell(&mut self, col: u16, row: u16, cell: Cell) {
        if col < self.grid.cols && row < self.grid.rows {
            let idx = self.grid.index(col, row);
            self.current[idx] = cell;
        }
    }

    #[inline]
    pub fn get_cell(&self, col: u16, row: u16) -> Option<&Cell> {
        if col < self.grid.cols && row < self.grid.rows {
            let idx = self.grid.index(col, row);
            Some(&self.current[idx])
        } else {
            None
        }
    }

    /// Flushes changes to stdout using optimal ANSI batching.
    pub fn flush<W: Write>(&mut self, out: &mut W) -> io::Result<()> {
        let mut last_fg: Option<ColorRgb> = None;
        let mut last_bg: Option<ColorRgb> = None;
        let mut cursor_col: Option<u16> = None;
        let mut cursor_row: Option<u16> = None;

        for row in 0..self.grid.rows {
            for col in 0..self.grid.cols {
                let idx = self.grid.index(col, row);
                let cur = self.current[idx];
                let prev = self.previous[idx];

                if self.full_redraw || cur != prev {
                    // Position cursor only if not sequentially next
                    let needs_move = match (cursor_col, cursor_row) {
                        (Some(c), Some(r)) => r != row || c != col,
                        _ => true,
                    };

                    if needs_move {
                        queue!(out, MoveTo(col, row))?;
                    }

                    // Update FG color if changed
                    if last_fg != Some(cur.fg) {
                        queue!(out, SetForegroundColor(cur.fg.to_crossterm()))?;
                        last_fg = Some(cur.fg);
                    }

                    // Update BG color if changed
                    if last_bg != cur.bg {
                        if let Some(bg_color) = cur.bg {
                            queue!(out, SetBackgroundColor(bg_color.to_crossterm()))?;
                        } else {
                            queue!(out, ResetColor)?;
                            // Re-apply FG if ResetColor wiped it
                            queue!(out, SetForegroundColor(cur.fg.to_crossterm()))?;
                        }
                        last_bg = cur.bg;
                    }

                    // Write character
                    write!(out, "{}", cur.ch)?;

                    cursor_col = Some(col + 1);
                    cursor_row = Some(row);
                }
            }
        }

        // Reset colors at the end of frame
        if last_fg.is_some() || last_bg.is_some() {
            queue!(out, ResetColor)?;
        }

        out.flush()?;

        // Swap buffers
        self.previous.copy_from_slice(&self.current);
        self.full_redraw = false;

        Ok(())
    }
}
