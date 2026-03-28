//! Screen buffer for terminal rendering

mod diff;

pub use diff::BufferDiff;

use crate::cell::Cell;
use crate::geometry::Rect;
use crate::style::Style;
use std::ops::{Index, IndexMut};

/// A buffer representing the terminal screen content
#[derive(Clone, Debug)]
pub struct Buffer {
    /// The area covered by this buffer
    pub area: Rect,
    /// The content of the buffer, stored row by row
    pub content: Vec<Cell>,
}

impl Buffer {
    /// Creates a new buffer filled with empty cells
    pub fn empty(area: Rect) -> Self {
        let size = area.width as usize * area.height as usize;
        Self {
            area,
            content: vec![Cell::default(); size],
        }
    }

    /// Creates a new buffer filled with a specific cell
    pub fn filled(area: Rect, cell: Cell) -> Self {
        let size = area.width as usize * area.height as usize;
        Self {
            area,
            content: vec![cell; size],
        }
    }

    /// Creates a new buffer with the given area
    pub fn new(area: Rect) -> Self {
        Self::empty(area)
    }

    /// Returns the area of the buffer
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Calculates the index in the content vector for the given position
    pub fn index(&self, x: u16, y: u16) -> usize {
        debug_assert!(
            x < self.area.width && y < self.area.height,
            "Attempt to access cell outside buffer: ({}, {}) for area {:?}",
            x,
            y,
            self.area
        );
        (y - self.area.y) as usize * self.area.width as usize + (x - self.area.x) as usize
    }

    /// Gets a reference to the cell at the given position
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        if x >= self.area.x
            && x < self.area.x + self.area.width
            && y >= self.area.y
            && y < self.area.y + self.area.height
        {
            let idx = self.index(x, y);
            self.content.get(idx)
        } else {
            None
        }
    }

    /// Gets a mutable reference to the cell at the given position
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x >= self.area.x
            && x < self.area.x + self.area.width
            && y >= self.area.y
            && y < self.area.y + self.area.height
        {
            let idx = self.index(x, y);
            self.content.get_mut(idx)
        } else {
            None
        }
    }

    /// Sets the cell at the given position
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(c) = self.get_mut(x, y) {
            *c = cell;
        }
    }

    /// Returns the number of cells in the buffer
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Returns true if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Returns an iterator over the cells
    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.content.iter()
    }

    /// Returns a mutable iterator over the cells
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.content.iter_mut()
    }

    /// Resets all cells to their default state
    pub fn reset(&mut self) {
        for cell in &mut self.content {
            cell.reset();
        }
    }

    /// Resizes the buffer to the new area
    pub fn resize(&mut self, area: Rect) {
        if self.area == area {
            return;
        }

        let new_size = area.width as usize * area.height as usize;
        let mut new_content = vec![Cell::default(); new_size];

        let min_width = area.width.min(self.area.width) as usize;
        let min_height = area.height.min(self.area.height) as usize;

        for y in 0..min_height {
            let src_start = y * self.area.width as usize;
            let dst_start = y * area.width as usize;

            for x in 0..min_width {
                new_content[dst_start + x] = self.content[src_start + x].clone();
            }
        }

        self.area = area;
        self.content = new_content;
    }

    /// Clears the buffer with the given cell value
    pub fn clear_with(&mut self, cell: Cell) {
        for c in &mut self.content {
            *c = cell.clone();
        }
    }

    /// Clears the buffer with default cells
    pub fn clear(&mut self) {
        self.reset();
    }

    /// Compares this buffer with another and returns an iterator over changed cells.
    ///
    /// The iterator yields `(x, y, &Cell)` tuples for cells that differ between
    /// the two buffers. This is used for efficient incremental rendering - only
    /// cells that have changed need to be drawn to the terminal.
    ///
    /// Both buffers must have the same area.
    pub fn diff<'a, 'b>(&'a self, other: &'b Buffer) -> BufferDiff<'a, 'b> {
        debug_assert_eq!(self.area, other.area, "Buffer areas must match for diffing");
        BufferDiff::new(&self.content, &other.content, self.area)
    }

    /// Fills the entire buffer with the given style.
    ///
    /// The cell symbols remain unchanged, but the style (colors and modifiers)
    /// is applied to all cells.
    pub fn fill(&mut self, style: Style) {
        for cell in &mut self.content {
            cell.fg = style.fg;
            cell.bg = style.bg;
            cell.modifier = style.modifier;
        }
    }

    /// Copies content from another buffer.
    ///
    /// This is more efficient than cloning because it reuses the existing
    /// allocation when possible. The source buffer must have the same area.
    pub fn copy_from(&mut self, other: &Buffer) {
        debug_assert_eq!(
            self.area, other.area,
            "Buffer areas must match for copy_from"
        );
        if self.area == other.area {
            for (i, cell) in other.content.iter().enumerate() {
                self.content[i] = cell.clone();
            }
        }
    }

    /// Returns a slice of the buffer content for a specific row.
    ///
    /// Returns `None` if the row index is out of bounds.
    pub fn row(&self, y: u16) -> Option<&[Cell]> {
        if y >= self.area.height {
            return None;
        }
        let start = (y as usize) * (self.area.width as usize);
        let end = start + (self.area.width as usize);
        Some(&self.content[start..end])
    }

    /// Returns a mutable slice of the buffer content for a specific row.
    ///
    /// Returns `None` if the row index is out of bounds.
    pub fn row_mut(&mut self, y: u16) -> Option<&mut [Cell]> {
        if y >= self.area.height {
            return None;
        }
        let start = (y as usize) * (self.area.width as usize);
        let end = start + (self.area.width as usize);
        Some(&mut self.content[start..end])
    }
}

impl Index<(u16, u16)> for Buffer {
    type Output = Cell;

    fn index(&self, (x, y): (u16, u16)) -> &Self::Output {
        let idx = self.index(x, y);
        &self.content[idx]
    }
}

impl IndexMut<(u16, u16)> for Buffer {
    fn index_mut(&mut self, (x, y): (u16, u16)) -> &mut Self::Output {
        let idx = self.index(x, y);
        &mut self.content[idx]
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::empty(Rect::new(0, 0, 80, 24))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Color, Modifier, Style};

    #[test]
    fn test_buffer_empty() {
        let buf = Buffer::empty(Rect::new(0, 0, 10, 5));
        assert_eq!(buf.area.width, 10);
        assert_eq!(buf.area.height, 5);
        assert_eq!(buf.len(), 50);
    }

    #[test]
    fn test_buffer_filled() {
        let cell = Cell::new("X");
        let buf = Buffer::filled(Rect::new(0, 0, 5, 5), cell.clone());
        assert_eq!(buf.len(), 25);
        assert_eq!(buf[(0, 0)].symbol, "X");
        assert_eq!(buf[(4, 4)].symbol, "X");
    }

    #[test]
    fn test_buffer_index() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        buf[(0, 0)].symbol = "A".to_string();
        buf[(5, 3)].symbol = "B".to_string();

        assert_eq!(buf[(0, 0)].symbol, "A");
        assert_eq!(buf[(5, 3)].symbol, "B");

        let expected_idx = 3 * 10 + 5;
        assert_eq!(buf.index(5, 3), expected_idx);
    }

    #[test]
    fn test_buffer_set_get() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        let cell = Cell::new("X");

        buf.set(5, 5, cell.clone());

        let got = buf.get(5, 5).unwrap();
        assert_eq!(got.symbol, "X");
    }

    #[test]
    fn test_buffer_get_out_of_bounds() {
        let buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        assert!(buf.get(10, 0).is_none());
        assert!(buf.get(0, 10).is_none());
        assert!(buf.get(15, 15).is_none());
    }

    #[test]
    fn test_buffer_resize() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        buf[(0, 0)].symbol = "A".to_string();
        buf[(5, 5)].symbol = "B".to_string();

        buf.resize(Rect::new(0, 0, 20, 20));

        assert_eq!(buf.area.width, 20);
        assert_eq!(buf.area.height, 20);
        assert_eq!(buf.len(), 400);
        assert_eq!(buf[(0, 0)].symbol, "A");
        assert_eq!(buf[(5, 5)].symbol, "B");
    }

    #[test]
    fn test_buffer_resize_smaller() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        buf[(0, 0)].symbol = "A".to_string();
        buf[(8, 8)].symbol = "B".to_string();

        buf.resize(Rect::new(0, 0, 5, 5));

        assert_eq!(buf.area.width, 5);
        assert_eq!(buf.area.height, 5);
        assert_eq!(buf.len(), 25);
        assert_eq!(buf[(0, 0)].symbol, "A");
    }

    #[test]
    fn test_buffer_clear() {
        let mut buf = Buffer::filled(Rect::new(0, 0, 5, 5), Cell::new("X"));
        buf.clear();

        for cell in buf.iter() {
            assert_eq!(cell.symbol, " ");
        }
    }

    #[test]
    fn test_cell_width_cjk() {
        let cell = Cell::new("あ");
        assert_eq!(cell.width(), 2);
    }

    #[test]
    fn test_cell_width_emoji() {
        let cell = Cell::new("😀");
        assert_eq!(cell.width(), 2);
    }

    #[test]
    fn test_buffer_with_style() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        buf[(2, 3)].set_fg(Color::Red);
        buf[(2, 3)].set_bg(Color::Blue);
        buf[(2, 3)].set_style(
            Style::new()
                .fg(Color::Green)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

        assert_eq!(buf[(2, 3)].fg, Color::Green);
        assert_eq!(buf[(2, 3)].bg, Color::Yellow);
        assert!(buf[(2, 3)].modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_buffer_with_offset_area() {
        let mut buf = Buffer::empty(Rect::new(5, 5, 10, 10));
        buf[(5, 5)].symbol = "A".to_string();
        assert_eq!(buf.get(5, 5).unwrap().symbol, "A");
        assert!(buf.get(4, 5).is_none());
        assert!(buf.get(15, 5).is_none());
    }

    // ===== Edge case tests =====

    #[test]
    fn test_buffer_zero_area() {
        // Zero-area buffer should have no content
        let buf = Buffer::empty(Rect::default());
        assert_eq!(buf.area.width, 0);
        assert_eq!(buf.area.height, 0);
        assert_eq!(buf.len(), 0);
        assert!(buf.is_empty());

        // Operations on zero-area buffer should return None
        assert!(buf.get(0, 0).is_none());
    }

    #[test]
    fn test_buffer_one_by_one() {
        // Minimal 1x1 buffer
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 1));
        assert_eq!(buf.area.width, 1);
        assert_eq!(buf.area.height, 1);
        assert_eq!(buf.len(), 1);
        assert!(!buf.is_empty());

        // Should be able to set and get the single cell
        buf.set(0, 0, Cell::new("X"));
        assert_eq!(buf.get(0, 0).unwrap().symbol, "X");

        // Out of bounds should return None
        assert!(buf.get(1, 0).is_none());
        assert!(buf.get(0, 1).is_none());
    }

    #[test]
    fn test_buffer_very_large() {
        // Large 1000x1000 buffer (1 million cells)
        let buf = Buffer::empty(Rect::new(0, 0, 1000, 1000));
        assert_eq!(buf.area.width, 1000);
        assert_eq!(buf.area.height, 1000);
        assert_eq!(buf.len(), 1_000_000);
        assert!(!buf.is_empty());

        // Should be able to access corners
        assert!(buf.get(0, 0).is_some());
        assert!(buf.get(999, 999).is_some());
        assert!(buf.get(999, 0).is_some());
        assert!(buf.get(0, 999).is_some());

        // Just outside should be None
        assert!(buf.get(1000, 0).is_none());
        assert!(buf.get(0, 1000).is_none());
    }

    #[test]
    fn test_buffer_multi_width_char_at_boundary() {
        // Test emoji (2-cell width) at the right edge of buffer
        // Emoji at x=width-2 should fit, at x=width-1 it would overflow
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));

        // Place emoji at valid position (leaving 2 cells)
        buf.set(8, 0, Cell::new("😀"));
        assert_eq!(buf.get(8, 0).unwrap().symbol, "😀");
        assert_eq!(buf.get(8, 0).unwrap().width(), 2);

        // Position at the last cell - multi-width char would overflow visually
        // The buffer allows setting but renderer must handle this edge case
        buf.set(9, 0, Cell::new("🎉"));
        assert_eq!(buf.get(9, 0).unwrap().symbol, "🎉");

        // Emoji in middle of buffer works normally
        buf.set(5, 2, Cell::new("🚀"));
        assert_eq!(buf.get(5, 2).unwrap().symbol, "🚀");
    }

    #[test]
    fn test_buffer_symbol_longer_than_4_chars() {
        // Symbols can be longer than 4 characters (e.g., multiple emoji or ligatures)
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 10));

        // Multi-character symbol (combination emoji)
        let long_symbol = "👨‍👩‍👧‍👦"; // Family emoji (actually rendered as multiple chars but stored as single symbol)
        buf.set(0, 0, Cell::new(long_symbol));
        assert_eq!(buf.get(0, 0).unwrap().symbol, long_symbol);

        // Another long symbol example
        let another_long = "🇬🇧🇺🇸"; // Flag sequence
        buf.set(5, 5, Cell::new(another_long));
        assert_eq!(buf.get(5, 5).unwrap().symbol, another_long);

        // Text longer than 4 chars
        let text_symbol = "Hello";
        buf.set(10, 0, Cell::new(text_symbol));
        assert_eq!(buf.get(10, 0).unwrap().symbol, text_symbol);
    }

    #[test]
    fn test_buffer_multi_width_cjk_at_boundary() {
        // Test CJK character (2-cell width) at buffer boundaries
        let mut buf = Buffer::empty(Rect::new(0, 0, 5, 5));

        // CJK at last valid position for 2-width char
        buf.set(3, 0, Cell::new("あ")); // at x=3, occupies cells 3 and 4
        assert_eq!(buf.get(3, 0).unwrap().symbol, "あ");

        // CJK at position that would overflow
        buf.set(4, 1, Cell::new("日")); // at x=4, but width is 2
        assert_eq!(buf.get(4, 1).unwrap().symbol, "日");

        // Regular positions work fine
        buf.set(0, 0, Cell::new("中"));
        assert_eq!(buf.get(0, 0).unwrap().symbol, "中");
    }

    #[test]
    fn test_buffer_skip_flag_placement() {
        // Test the skip flag for trailing cells of wide characters
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));

        // Create a cell with skip flag set
        let mut wide_cell = Cell::new("😀");
        wide_cell.set_skip(true);

        buf.set(0, 0, wide_cell.clone());

        let cell = buf.get(0, 0).unwrap();
        assert!(cell.skip);
        assert_eq!(cell.symbol, "😀");

        // Set skip back to false
        buf.get_mut(0, 0).unwrap().set_skip(false);
        assert!(!buf.get(0, 0).unwrap().skip);
    }
}
