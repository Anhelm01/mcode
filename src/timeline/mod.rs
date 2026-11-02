//! Timeline and sequence morphing between shapes.

use crate::config::SequenceItem;
use crate::core::grid::Grid;
use crate::mask::{BitMask, text_mask::TextMaskRasterizer, image_mask::ImageMaskRasterizer};

pub struct TimelineEntry {
    pub mask: BitMask,
    pub duration: f32,
    pub _name: String,
}

pub struct Timeline {
    pub entries: Vec<TimelineEntry>,
    pub total_duration: f32,
    pub loop_sequence: bool,
    pub _switch_duration: f32,
}

impl Timeline {
    pub fn build(
        items: &[SequenceItem],
        grid: Grid,
        loop_sequence: bool,
        switch_duration: f32,
    ) -> Self {
        let text_rasterizer = TextMaskRasterizer::new();
        let mut entries = Vec::new();
        let mut total_duration = 0.0f32;

        for item in items {
            match item {
                SequenceItem::Text(text, dur) => {
                    let mask = text_rasterizer.rasterize(text, grid);
                    let dur_sec = dur.as_secs_f32();
                    total_duration += dur_sec;
                    entries.push(TimelineEntry {
                        mask,
                        duration: dur_sec,
                        _name: text.clone(),
                    });
                }
                SequenceItem::Image(path, dur) => {
                    let mask = ImageMaskRasterizer::rasterize(path, grid)
                        .unwrap_or_else(|_| BitMask::new(grid, vec![false; grid.len()]));
                    let dur_sec = dur.as_secs_f32();
                    total_duration += dur_sec;
                    entries.push(TimelineEntry {
                        mask,
                        duration: dur_sec,
                        _name: path.clone(),
                    });
                }
            }
        }

        // If empty, supply a default
        if entries.is_empty() {
            let mask = text_rasterizer.rasterize("98%", grid);
            total_duration = 6.0;
            entries.push(TimelineEntry {
                mask,
                duration: 6.0,
                _name: "98%".to_string(),
            });
        }

        Self {
            entries,
            total_duration,
            loop_sequence,
            _switch_duration: switch_duration,
        }
    }

    /// Returns: (prev_index, current_index, local_time, step_index)
    pub fn state_at(&self, time: f32) -> (usize, usize, f32, u32) {
        if self.entries.is_empty() {
            return (0, 0, 0.0, 0);
        }

        let effective_time = if self.loop_sequence && self.total_duration > 0.0 {
            time.rem_euclid(self.total_duration)
        } else {
            time.min(self.total_duration)
        };

        let mut accumulated = 0.0f32;
        let mut cur_idx = 0;
        let mut local_t = 0.0f32;

        for (idx, entry) in self.entries.iter().enumerate() {
            if effective_time < accumulated + entry.duration || idx == self.entries.len() - 1 {
                cur_idx = idx;
                local_t = effective_time - accumulated;
                break;
            }
            accumulated += entry.duration;
        }

        let prev_idx = if cur_idx == 0 {
            self.entries.len() - 1
        } else {
            cur_idx - 1
        };

        let step_idx = (time / self.total_duration.max(0.1)).floor() as u32 * (self.entries.len() as u32) + (cur_idx as u32);

        (prev_idx, cur_idx, local_t, step_idx)
    }
}
