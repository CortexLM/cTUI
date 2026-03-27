//! Buffer diffing for zero-allocation incremental rendering
//!
//! This module provides the `BufferDiff` iterator that yields only the cells
//! that have changed between two buffers. This is critical for efficient
//! terminal rendering - we only update what changed.
//!
//! # Multi-width Character Handling
//!
//! Wide characters (CJK, emoji, etc.) occupy multiple cells. When diffing:
//! - Leading cell contains the full character
//! - Trailing cells are marked with `skip=true`
//! - Diff iterator skips trailing cells for both unchanged and changed cells
//!
//! # Zero-Allocation Design
//!
//! The iterator uses indices and references, never allocating on the heap:
//! - No Vec construction during iteration
//! - All state tracked in the struct
//! - Returns references to cells in the next buffer

use crate::cell::Cell;
use crate::geometry::Rect;

/// State for tracking multi-width character traversal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrailingState {
    /// Currently skipping trailing cells of a multi-width character
    /// The value indicates how many more trailing cells to skip
    Skipping(usize),
}

/// Iterator over cells that differ between two buffers.
///
/// This iterator yields `(x, y, &Cell)` tuples for cells that have changed
/// between `prev` and `next` buffers. It efficiently handles multi-width
/// characters by skipping trailing cells that are part of the same character.
///
/// # Example
///
/// ```ignore
/// let prev = Buffer::empty(area);
/// let next = render_frame();
///
/// for (x, y, cell) in prev.diff(&next) {
///     backend.draw_cell(x, y, cell);
/// }
/// ```
#[derive(Debug)]
pub struct BufferDiff<'prev, 'next> {
    /// The previous buffer (what's currently on screen)
    prev: &'prev [Cell],
    /// The next buffer (what we want to render)
    next: &'next [Cell],
    /// The area covered by these buffers
    area: Rect,
    /// Current position in the flattened cell array
    pos: usize,
    /// State for handling multi-width character trailing cells
    trailing: Option<TrailingState>,
}

impl<'prev, 'next> BufferDiff<'prev, 'next> {
    /// Creates a new buffer diff iterator.
    ///
    /// Both buffers must have the same area. The iterator will yield
    /// cells from `next` that are different from `prev`.
    pub(crate) fn new(prev: &'prev [Cell], next: &'next [Cell], area: Rect) -> Self {
        debug_assert_eq!(prev.len(), next.len(), "Buffer sizes must match");
        debug_assert_eq!(
            prev.len(),
            area.width as usize * area.height as usize,
            "Buffer size must match area"
        );

        Self {
            prev,
            next,
            area,
            pos: 0,
            trailing: None,
        }
    }

    /// Get the (x, y) coordinates and cell for the current position.
    fn get_current(&self) -> Option<(u16, u16, &'next Cell)> {
        if self.pos >= self.next.len() {
            return None;
        }

        let width = self.area.width as usize;
        let x = (self.pos % width) as u16 + self.area.x;
        let y = (self.pos / width) as u16 + self.area.y;
        let cell = &self.next[self.pos];

        Some((x, y, cell))
    }

    /// Check if cells at current position differ.
    fn cells_differ(&self) -> bool {
        self.prev.get(self.pos) != self.next.get(self.pos)
    }

    /// Skip trailing cells of a multi-width character.
    ///
    /// When a wide character is encountered, the following cells are marked
    /// with `skip=true`. We need to skip them in our iteration.
    fn skip_trailing_cells(&mut self, width: usize) {
        if width > 1 {
            self.trailing = Some(TrailingState::Skipping(width - 1));
        }
    }
}

impl<'next> Iterator for BufferDiff<'_, 'next> {
    type Item = (u16, u16, &'next Cell);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(TrailingState::Skipping(remaining)) = self.trailing {
                if remaining == 0 {
                    self.trailing = None;
                } else {
                    self.pos += 1;
                    self.trailing = Some(TrailingState::Skipping(remaining - 1));
                    continue;
                }
            }

            if self.pos >= self.next.len() {
                return None;
            }

            if self.next[self.pos].skip {
                self.pos += 1;
                continue;
            }

            if self.prev.get(self.pos).map(|c| c.skip).unwrap_or(false) {
                self.pos += 1;
                continue;
            }

            if self.cells_differ() {
                let result = self.get_current();
                let cell = &self.next[self.pos];
                self.pos += 1;
                let width = cell.width();
                self.skip_trailing_cells(width);
                return result;
            }

            let cell = &self.next[self.pos];
            let width = cell.width();
            self.pos += 1;
            if width > 1 {
                self.trailing = Some(TrailingState::Skipping(width - 1));
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next.len().saturating_sub(self.pos);
        (0, Some(remaining))
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::Buffer;
    use crate::geometry::Rect;

    #[test]
    fn test_empty_diff() {
        let area = Rect::new(0, 0, 10, 10);
        let buf1 = Buffer::empty(area);
        let buf2 = Buffer::empty(area);

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert!(diff.is_empty());
    }

    #[test]
    fn test_single_cell_change() {
        let area = Rect::new(0, 0, 10, 10);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        buf2.content[0].symbol = "X".to_string();

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 0);
        assert_eq!(diff[0].1, 0);
        assert_eq!(diff[0].2.symbol, "X");
    }

    #[test]
    fn test_multiple_cell_changes() {
        let area = Rect::new(0, 0, 10, 10);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Change cells at different positions
        buf2.content[0].symbol = "A".to_string(); // (0, 0)
        buf2.content[5].symbol = "B".to_string(); // (5, 0)
        buf2.content[10].symbol = "C".to_string(); // (0, 1)

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 3);
    }

    #[test]
    fn test_cjk_character_handling() {
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Place a CJK character (width=2) at position 0
        buf2.content[0].symbol = "あ".to_string();
        buf2.content[0].skip = false;
        // The trailing cell should be marked as skip
        buf2.content[1].skip = true;

        let diff: Vec<_> = buf1.diff(&buf2).collect();

        // Should only yield one cell (the leading cell)
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 0);
        assert_eq!(diff[0].1, 0);
        assert_eq!(diff[0].2.symbol, "あ");
    }

    #[test]
    fn test_emoji_handling() {
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Place an emoji (width=2) at position 5
        buf2.content[5].symbol = "😀".to_string();
        buf2.content[6].skip = true;

        let diff: Vec<_> = buf1.diff(&buf2).collect();

        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 5);
        assert_eq!(diff[0].2.symbol, "😀");
    }

    #[test]
    fn test_mixed_width_characters() {
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Mix of ASCII, CJK, and emoji
        buf2.content[0].symbol = "A".to_string(); // width=1
        buf2.content[1].symbol = "あ".to_string(); // width=2
        buf2.content[2].skip = true; // trailing of あ
        buf2.content[3].symbol = "B".to_string(); // width=1
        buf2.content[4].symbol = "😀".to_string(); // width=2
        buf2.content[5].skip = true; // trailing of 😀

        let diff: Vec<_> = buf1.diff(&buf2).collect();

        // Should yield 4 cells: A, あ, B, 😀 (not the trailing skip cells)
        assert_eq!(diff.len(), 4);

        let symbols: Vec<_> = diff.iter().map(|(_, _, c)| c.symbol.as_str()).collect();
        assert_eq!(symbols, vec!["A", "あ", "B", "😀"]);
    }

    #[test]
    fn test_same_multiwidth_no_diff() {
        let area = Rect::new(0, 0, 10, 5);
        let mut buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Both have the same CJK character at the same position
        buf1.content[0].symbol = "あ".to_string();
        buf1.content[1].skip = true;

        buf2.content[0].symbol = "あ".to_string();
        buf2.content[1].skip = true;

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert!(diff.is_empty());
    }

    #[test]
    fn test_multiwidth_character_changed() {
        let area = Rect::new(0, 0, 10, 5);
        let mut buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // buf1 has CJK character
        buf1.content[0].symbol = "あ".to_string();
        buf1.content[1].skip = true;

        // buf2 has different CJK character
        buf2.content[0].symbol = "い".to_string();
        buf2.content[1].skip = true;

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].2.symbol, "い");
    }

    #[test]
    fn test_style_change_counts_as_diff() {
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Same symbol, different style
        use crate::style::Color;
        buf2.content[0].fg = Color::Red;

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1);
    }

    #[test]
    fn test_full_row_diff() {
        let area = Rect::new(0, 0, 5, 3);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Change entire middle row
        for i in 5..10 {
            buf2.content[i].symbol = "X".to_string();
        }

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 5);

        // All should be on row 1 (y=1)
        for (_, y, _) in &diff {
            assert_eq!(*y, 1);
        }
    }

    #[test]
    fn test_different_areas_panic() {
        let buf1 = Buffer::empty(Rect::new(0, 0, 10, 10));
        let buf2 = Buffer::empty(Rect::new(0, 0, 5, 5));

        // This should panic in debug mode due to size mismatch
        // In release mode, this is undefined behavior
        // We test in debug mode
        #[cfg(debug_assertions)]
        {
            let result = std::panic::catch_unwind(|| {
                let _: Vec<_> = buf1.diff(&buf2).collect();
            });
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_position_calculation() {
        let area = Rect::new(0, 0, 80, 24);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Change cell at specific position (42, 13)
        let idx = 13 * 80 + 42;
        buf2.content[idx].symbol = "X".to_string();

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 42);
        assert_eq!(diff[0].1, 13);
    }

    #[test]
    fn test_area_with_offset() {
        let area = Rect::new(5, 3, 10, 10);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // First cell should report (5, 3)
        buf2.content[0].symbol = "X".to_string();

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff[0].0, 5);
        assert_eq!(diff[0].1, 3);
    }
}
