//! Buffer diffing for zero-allocation incremental rendering
//!
//! This module provides the `BufferDiff` iterator that yields only the cells
//! that have changed between two buffers. This is critical for efficient
//! terminal rendering - we only update what changed.

use crate::cell::Cell;
use crate::geometry::Rect;
use crate::packed_cell::PackedCell;
use crate::symbol_table::SymbolTable;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrailingState {
    Skipping(usize),
}

/// Iterator over cells that differ between two buffers.
///
/// This iterator yields `(x, y, Cell)` tuples for cells that have changed
/// between `prev` and `next` buffers. It efficiently handles multi-width
/// characters by skipping trailing cells that are part of the same character.
#[derive(Debug)]
pub struct BufferDiff<'prev, 'next> {
    prev: &'prev [PackedCell],
    next: &'next [PackedCell],
    area: Rect,
    pos: usize,
    trailing: Option<TrailingState>,
    prev_table: &'prev Arc<RwLock<SymbolTable>>,
    next_table: &'next Arc<RwLock<SymbolTable>>,
}

impl<'prev, 'next> BufferDiff<'prev, 'next> {
    pub(crate) fn new(
        prev: &'prev [PackedCell],
        next: &'next [PackedCell],
        area: Rect,
        prev_table: &'prev Arc<RwLock<SymbolTable>>,
        next_table: &'next Arc<RwLock<SymbolTable>>,
    ) -> Self {
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
            prev_table,
            next_table,
        }
    }

    fn get_current(&self) -> Option<(u16, u16, Cell)> {
        if self.pos >= self.next.len() {
            return None;
        }

        let width = self.area.width as usize;
        let x = (self.pos % width) as u16 + self.area.x;
        let y = (self.pos / width) as u16 + self.area.y;
        let table = self.next_table.read().unwrap();
        let cell = self.next[self.pos].to_cell(&table);

        Some((x, y, cell))
    }

    fn cells_differ(&self) -> bool {
        let prev_cell = self.prev.get(self.pos);
        let next_cell = self.next.get(self.pos);

        match (prev_cell, next_cell) {
            (Some(p), Some(n)) => {
                let prev_table = self.prev_table.read().unwrap();
                let next_table = self.next_table.read().unwrap();
                let prev_unpacked = p.to_cell(&prev_table);
                let next_unpacked = n.to_cell(&next_table);
                prev_unpacked != next_unpacked
            }
            (None, Some(_)) | (Some(_), None) => true,
            (None, None) => false,
        }
    }

    fn skip_trailing_cells(&mut self, width: usize) {
        if width > 1 {
            self.trailing = Some(TrailingState::Skipping(width - 1));
        }
    }
}

impl<'next> Iterator for BufferDiff<'_, 'next> {
    type Item = (u16, u16, Cell);

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

            if self.next[self.pos].skip() {
                self.pos += 1;
                continue;
            }

            if self.prev.get(self.pos).map(|c| c.skip()).unwrap_or(false) {
                self.pos += 1;
                continue;
            }

            if self.cells_differ() {
                let result = self.get_current();
                let table = self.next_table.read().unwrap();
                let cell = self.next[self.pos].to_cell(&table);
                drop(table);
                self.pos += 1;
                let width = cell.width();
                self.skip_trailing_cells(width);
                return result;
            }

            let table = self.next_table.read().unwrap();
            let cell = self.next[self.pos].to_cell(&table);
            drop(table);
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
    use crate::cell::Cell;
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

        buf2.set(0, 0, Cell::new("X"));

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

        buf2.set(0, 0, Cell::new("A"));
        buf2.set(5, 0, Cell::new("B"));
        buf2.set(0, 1, Cell::new("C"));

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 3);
    }

    #[test]
    fn test_cjk_character_handling() {
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        let mut cell = Cell::new("あ");
        cell.skip = false;
        buf2.set(0, 0, cell);

        let mut trailing = Cell::new(" ");
        trailing.skip = true;
        buf2.set(1, 0, trailing);

        let diff: Vec<_> = buf1.diff(&buf2).collect();

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

        buf2.set(5, 0, Cell::new("😀"));
        let mut trailing = Cell::new(" ");
        trailing.skip = true;
        buf2.set(6, 0, trailing);

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

        buf2.set(0, 0, Cell::new("A"));
        buf2.set(1, 0, Cell::new("あ"));
        let mut trailing1 = Cell::new(" ");
        trailing1.skip = true;
        buf2.set(2, 0, trailing1);
        buf2.set(3, 0, Cell::new("B"));
        buf2.set(4, 0, Cell::new("😀"));
        let mut trailing2 = Cell::new(" ");
        trailing2.skip = true;
        buf2.set(5, 0, trailing2);

        let diff: Vec<_> = buf1.diff(&buf2).collect();

        assert_eq!(diff.len(), 4);

        let symbols: Vec<_> = diff.iter().map(|(_, _, c)| c.symbol.as_str()).collect();
        assert_eq!(symbols, vec!["A", "あ", "B", "😀"]);
    }

    #[test]
    fn test_same_multiwidth_no_diff() {
        let area = Rect::new(0, 0, 10, 5);
        let mut buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        buf1.set(0, 0, Cell::new("あ"));
        let mut trailing1 = Cell::new(" ");
        trailing1.skip = true;
        buf1.set(1, 0, trailing1);

        buf2.set(0, 0, Cell::new("あ"));
        let mut trailing2 = Cell::new(" ");
        trailing2.skip = true;
        buf2.set(1, 0, trailing2);

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert!(diff.is_empty());
    }

    #[test]
    fn test_multiwidth_character_changed() {
        let area = Rect::new(0, 0, 10, 5);
        let mut buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        buf1.set(0, 0, Cell::new("あ"));
        let mut trailing1 = Cell::new(" ");
        trailing1.skip = true;
        buf1.set(1, 0, trailing1);

        buf2.set(0, 0, Cell::new("い"));
        let mut trailing2 = Cell::new(" ");
        trailing2.skip = true;
        buf2.set(1, 0, trailing2);

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].2.symbol, "い");
    }

    #[test]
    fn test_style_change_counts_as_diff() {
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        let mut cell = Cell::new(" ");
        cell.fg = crate::style::Color::Red;
        buf2.set(0, 0, cell);

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1);
    }

    #[test]
    fn test_full_row_diff() {
        let area = Rect::new(0, 0, 5, 3);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        for x in 0..5 {
            buf2.set(x, 1, Cell::new("X"));
        }

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 5);

        for (_, y, _) in &diff {
            assert_eq!(*y, 1);
        }
    }

    #[test]
    fn test_different_areas_panic() {
        let buf1 = Buffer::empty(Rect::new(0, 0, 10, 10));
        let buf2 = Buffer::empty(Rect::new(0, 0, 5, 5));

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

        buf2.set(42, 13, Cell::new("X"));

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

        buf2.set(5, 3, Cell::new("X"));

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff[0].0, 5);
        assert_eq!(diff[0].1, 3);
    }

    // ========================================================================
    // Variation Selector Tests (VS15/VS16)
    // ========================================================================

    #[test]
    fn test_vs16_emoji_handling() {
        // VS16 (emoji presentation) should result in width 2
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Paperclip + VS16 = width 2 (emoji presentation)
        buf2.set(0, 0, Cell::new("\u{1F4CE}\u{FE0F}"));
        let mut trailing = Cell::new(" ");
        trailing.skip = true;
        buf2.set(1, 0, trailing);

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1, "VS16 emoji should emit one diff entry");
        assert_eq!(diff[0].0, 0);
        assert_eq!(diff[0].2.symbol, "\u{1F4CE}\u{FE0F}");
    }

    #[test]
    fn test_vs15_text_handling() {
        // VS15 (text presentation) should result in width 1
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Paperclip + VS15 = width 1 (text presentation)
        buf2.set(0, 0, Cell::new("\u{1F4CE}\u{FE0E}"));

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1, "VS15 text should emit one diff entry");
        assert_eq!(diff[0].0, 0);
        assert_eq!(diff[0].2.symbol, "\u{1F4CE}\u{FE0E}");
    }

    #[test]
    fn test_zwj_sequence_diff() {
        // ZWJ sequences should be treated as single width-2 grapheme
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Family emoji (ZWJ sequence)
        buf2.set(0, 0, Cell::new("\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}"));
        let mut trailing = Cell::new(" ");
        trailing.skip = true;
        buf2.set(1, 0, trailing);

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1, "ZWJ sequence should emit one diff entry");
        assert_eq!(diff[0].2.width(), 2);
    }

    #[test]
    fn test_skin_tone_diff() {
        // Skin tone modified emoji should be width 2
        let area = Rect::new(0, 0, 10, 5);
        let buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        // Waving hand with light skin tone
        buf2.set(0, 0, Cell::new("\u{1F44B}\u{1F3FB}"));
        let mut trailing = Cell::new(" ");
        trailing.skip = true;
        buf2.set(1, 0, trailing);

        let diff: Vec<_> = buf1.diff(&buf2).collect();
        assert_eq!(diff.len(), 1, "Skin tone emoji should emit one diff entry");
        assert_eq!(diff[0].2.width(), 2);
    }
}
