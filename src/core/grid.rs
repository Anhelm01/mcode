//! Terminal grid geometry and aspect-ratio utilities.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Grid {
    pub cols: u16,
    pub rows: u16,
}

impl Grid {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            cols: cols.max(1),
            rows: rows.max(1),
        }
    }

    /// Total number of character cells
    #[inline]
    pub fn len(&self) -> usize {
        (self.cols as usize) * (self.rows as usize)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.cols == 0 || self.rows == 0
    }

    /// Converts 2D (col, row) to linear index
    #[inline]
    pub fn index(&self, col: u16, row: u16) -> usize {
        (row as usize) * (self.cols as usize) + (col as usize)
    }

    /// Converts linear index to (col, row)
    #[inline]
    pub fn coord(&self, idx: usize) -> (u16, u16) {
        let cols = self.cols as usize;
        let row = (idx / cols) as u16;
        let col = (idx % cols) as u16;
        (col, row)
    }

    /// Standard terminal characters are ~1:2 aspect ratio (twice as high as wide).
    /// This returns normalized coordinates (0.0..1.0) taking aspect ratio into account.
    #[inline]
    pub fn aspect_normalized(&self, col: u16, row: u16) -> (f32, f32) {
        let x = col as f32 / self.cols as f32;
        let y = row as f32 / self.rows as f32;
        (x, y)
    }
}
