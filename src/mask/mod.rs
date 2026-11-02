pub mod sampler;
pub mod text_mask;
pub mod image_mask;

use crate::core::grid::Grid;

pub trait Mask: Send + Sync {
    /// Returns true if the cell at (col, row) is covered by the mask.
    fn contains(&self, col: u16, row: u16) -> bool;
    
    /// Bounding box: (min_col, min_row, max_col, max_row)
    fn bounds(&self) -> (u16, u16, u16, u16);
    
    /// Returns reference to all active linear cell indices
    fn active_cells(&self) -> &[usize];
}

#[derive(Clone, Debug)]
pub struct BitMask {
    pub grid: Grid,
    pub bits: Vec<bool>,
    pub active_indices: Vec<usize>,
    pub bounds: (u16, u16, u16, u16),
}

impl BitMask {
    pub fn new(grid: Grid, bits: Vec<bool>) -> Self {
        let mut active_indices = Vec::new();
        let mut min_col = grid.cols;
        let mut min_row = grid.rows;
        let mut max_col = 0;
        let mut max_row = 0;

        for (idx, &is_active) in bits.iter().enumerate() {
            if is_active {
                active_indices.push(idx);
                let (c, r) = grid.coord(idx);
                min_col = min_col.min(c);
                min_row = min_row.min(r);
                max_col = max_col.max(c);
                max_row = max_row.max(r);
            }
        }

        let bounds = if active_indices.is_empty() {
            (0, 0, grid.cols, grid.rows)
        } else {
            (min_col, min_row, max_col, max_row)
        };

        Self {
            grid,
            bits,
            active_indices,
            bounds,
        }
    }
}

impl Mask for BitMask {
    #[inline]
    fn contains(&self, col: u16, row: u16) -> bool {
        if col < self.grid.cols && row < self.grid.rows {
            let idx = self.grid.index(col, row);
            self.bits.get(idx).copied().unwrap_or(false)
        } else {
            false
        }
    }

    #[inline]
    fn bounds(&self) -> (u16, u16, u16, u16) {
        self.bounds
    }

    #[inline]
    fn active_cells(&self) -> &[usize] {
        &self.active_indices
    }
}
