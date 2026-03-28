//! Screen buffer for terminal rendering with packed cell storage
//!
//! The buffer uses `PackedCell` internally for memory efficiency (8 bytes per cell
//! vs ~40 bytes for Cell), converting to/from Cell at API boundaries.

mod diff;

pub use diff::BufferDiff;

use crate::cell::Cell;
use crate::geometry::Rect;
use crate::packed_cell::PackedCell;
use crate::style::Style;
use crate::symbol_table::SymbolTable;
use std::ops::Index;
use std::sync::{Arc, RwLock};

/// A buffer representing the terminal screen content.
///
/// Uses `PackedCell` internally for memory efficiency, with conversions
/// to/from `Cell` at API boundaries for backward compatibility.
#[derive(Clone, Debug)]
pub struct Buffer {
    /// The area covered by this buffer
    pub area: Rect,
    /// The content of the buffer, stored as packed cells (8 bytes each)
    pub content: Vec<PackedCell>,
    /// Symbol table for string interning (shared across clones)
    symbol_table: Arc<RwLock<SymbolTable>>,
}

impl Buffer {
    /// Creates a new buffer filled with empty cells.
    pub fn empty(area: Rect) -> Self {
        let symbol_table = Arc::new(RwLock::new(SymbolTable::new()));
        let size = area.width as usize * area.height as usize;
        let default_cell = PackedCell::default();
        Self {
            area,
            content: vec![default_cell; size],
            symbol_table,
        }
    }

    /// Creates a new buffer filled with a specific cell.
    pub fn filled(area: Rect, cell: Cell) -> Self {
        let mut symbol_table = SymbolTable::new();
        let size = area.width as usize * area.height as usize;
        let packed = PackedCell::from_cell(&cell, &mut symbol_table);
        Self {
            area,
            content: vec![packed; size],
            symbol_table: Arc::new(RwLock::new(symbol_table)),
        }
    }

    /// Creates a new buffer with the given area.
    pub fn new(area: Rect) -> Self {
        Self::empty(area)
    }

    /// Returns the area of the buffer.
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Returns a reference to the symbol table.
    pub fn symbol_table(&self) -> &Arc<RwLock<SymbolTable>> {
        &self.symbol_table
    }

    /// Calculates the index in the content vector for the given position.
    pub fn index_of(&self, x: u16, y: u16) -> usize {
        debug_assert!(
            x >= self.area.x
                && x < self.area.x + self.area.width
                && y >= self.area.y
                && y < self.area.y + self.area.height,
            "Attempt to access cell outside buffer: ({}, {}) for area {:?}",
            x,
            y,
            self.area
        );
        (y - self.area.y) as usize * self.area.width as usize + (x - self.area.x) as usize
    }

    /// Gets the cell at the given position.
    ///
    /// Returns `None` if the position is out of bounds.
    /// Returns an owned `Cell` (not a reference, since cells are stored packed).
    pub fn get(&self, x: u16, y: u16) -> Option<Cell> {
        if x >= self.area.x
            && x < self.area.x + self.area.width
            && y >= self.area.y
            && y < self.area.y + self.area.height
        {
            let idx = self.index_of(x, y);
            let table = self.symbol_table.read().unwrap();
            Some(self.content[idx].to_cell(&table))
        } else {
            None
        }
    }

    /// Sets the cell at the given position.
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x >= self.area.x
            && x < self.area.x + self.area.width
            && y >= self.area.y
            && y < self.area.y + self.area.height
        {
            let idx = self.index_of(x, y);
            let mut table = self.symbol_table.write().unwrap();
            self.content[idx] = PackedCell::from_cell(&cell, &mut table);
        }
    }

    /// Modifies the cell at the given position using a closure.
    ///
    /// This is useful when you need to modify a cell in place without
    /// manually reading, modifying, and writing it back.
    pub fn modify_cell<F: FnOnce(&mut Cell)>(&mut self, x: u16, y: u16, f: F) {
        if let Some(mut cell) = self.get(x, y) {
            f(&mut cell);
            self.set(x, y, cell);
        }
    }

    /// Returns the number of cells in the buffer.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Returns an iterator over the cells as owned `Cell` values.
    pub fn iter(&self) -> impl Iterator<Item = Cell> + '_ {
        let table = self.symbol_table.read().unwrap();
        self.content
            .iter()
            .map(move |packed| packed.to_cell(&table))
    }

    /// Resets all cells to their default state.
    pub fn reset(&mut self) {
        for packed in &mut self.content {
            *packed = PackedCell::default();
        }
    }

    /// Resizes the buffer to the new area.
    pub fn resize(&mut self, area: Rect) {
        if self.area == area {
            return;
        }

        let new_size = area.width as usize * area.height as usize;
        let mut new_content = vec![PackedCell::default(); new_size];

        let min_width = area.width.min(self.area.width) as usize;
        let min_height = area.height.min(self.area.height) as usize;

        for y in 0..min_height {
            let src_start = y * self.area.width as usize;
            let dst_start = y * area.width as usize;

            for x in 0..min_width {
                new_content[dst_start + x] = self.content[src_start + x];
            }
        }

        self.area = area;
        self.content = new_content;
    }

    /// Clears the buffer with the given cell value.
    #[allow(clippy::significant_drop_tightening)]
    pub fn clear_with(&mut self, cell: Cell) {
        let mut table = self.symbol_table.write().unwrap();
        let packed = PackedCell::from_cell(&cell, &mut table);
        for c in &mut self.content {
            *c = packed;
        }
    }

    /// Clears the buffer with default cells.
    pub fn clear(&mut self) {
        self.reset();
    }

    /// Compares this buffer with another and returns an iterator over changed cells.
    ///
    /// The iterator yields `(x, y, Cell)` tuples for cells that differ between
    /// the two buffers. This is used for efficient incremental rendering - only
    /// cells that have changed need to be drawn to the terminal.
    ///
    /// Both buffers must have the same area.
    pub fn diff<'a, 'b>(&'a self, other: &'b Buffer) -> BufferDiff<'a, 'b> {
        debug_assert_eq!(self.area, other.area, "Buffer areas must match for diffing");
        debug_assert_eq!(self.area, other.area, "Buffer areas must match for diffing");
        BufferDiff::new(
            &self.content,
            &other.content,
            self.area,
            &self.symbol_table,
            &other.symbol_table,
        )
    }

    /// Fills the entire buffer with the given style.
    ///
    /// The cell symbols remain unchanged, but the style (colors and modifiers)
    /// is applied to all cells.
    #[allow(clippy::significant_drop_tightening, clippy::needless_collect)]
    pub fn fill(&mut self, style: Style) {
        let table = self.symbol_table.read().unwrap();
        let cells: Vec<Cell> = self.content.iter().map(|p| p.to_cell(&table)).collect();
        drop(table);

        let mut table = self.symbol_table.write().unwrap();
        for (packed, cell) in self.content.iter_mut().zip(cells.into_iter()) {
            let mut new_cell = cell;
            new_cell.fg = style.fg;
            new_cell.bg = style.bg;
            new_cell.modifier = style.modifier;
            *packed = PackedCell::from_cell(&new_cell, &mut table);
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
            self.content.clone_from_slice(&other.content);
        }
    }

    /// Returns the cells in a specific row as owned values.
    ///
    /// Returns `None` if the row index is out of bounds.
    pub fn row(&self, y: u16) -> Option<Vec<Cell>> {
        if y >= self.area.height {
            return None;
        }
        let start = (y as usize) * (self.area.width as usize);
        let end = start + (self.area.width as usize);
        let table = self.symbol_table.read().unwrap();
        Some(
            self.content[start..end]
                .iter()
                .map(|p| p.to_cell(&table))
                .collect(),
        )
    }
}

impl Index<(u16, u16)> for Buffer {
    type Output = ();

    fn index(&self, _pos: (u16, u16)) -> &Self::Output {
        panic!("Use .get(x, y) or .set(x, y, cell) instead of index access");
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
    use crate::style::{Color, Modifier};

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
        assert_eq!(buf.get(0, 0).unwrap().symbol, "X");
        assert_eq!(buf.get(4, 4).unwrap().symbol, "X");
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
        buf.set(0, 0, Cell::new("A"));
        buf.set(5, 5, Cell::new("B"));

        buf.resize(Rect::new(0, 0, 20, 20));

        assert_eq!(buf.area.width, 20);
        assert_eq!(buf.area.height, 20);
        assert_eq!(buf.len(), 400);
        assert_eq!(buf.get(0, 0).unwrap().symbol, "A");
        assert_eq!(buf.get(5, 5).unwrap().symbol, "B");
    }

    #[test]
    fn test_buffer_resize_smaller() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        buf.set(0, 0, Cell::new("A"));
        buf.set(8, 8, Cell::new("B"));

        buf.resize(Rect::new(0, 0, 5, 5));

        assert_eq!(buf.area.width, 5);
        assert_eq!(buf.area.height, 5);
        assert_eq!(buf.len(), 25);
        assert_eq!(buf.get(0, 0).unwrap().symbol, "A");
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
        let mut cell = Cell::new(" ");
        cell.fg = Color::Green;
        cell.bg = Color::Yellow;
        cell.modifier = Modifier::BOLD;
        buf.set(2, 3, cell);

        let got = buf.get(2, 3).unwrap();
        // With float-colors, named colors are converted to RGB via f32
        // Green (0.0, 0.5, 0.0) -> Rgb(0, 128, 0)
        // Yellow (0.5, 0.5, 0.0) -> Rgb(128, 128, 0)
        #[cfg(not(feature = "float-colors"))]
        {
            assert_eq!(got.fg, Color::Green);
            assert_eq!(got.bg, Color::Yellow);
        }
        #[cfg(feature = "float-colors")]
        {
            // Named colors become RGB with f32 precision
            assert!(matches!(got.fg, Color::Rgb(_, _, _)));
            assert!(matches!(got.bg, Color::Rgb(_, _, _)));
        }
        assert!(got.modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_buffer_with_offset_area() {
        let mut buf = Buffer::empty(Rect::new(5, 5, 10, 10));
        buf.set(5, 5, Cell::new("A"));
        assert_eq!(buf.get(5, 5).unwrap().symbol, "A");
        assert!(buf.get(4, 5).is_none());
        assert!(buf.get(15, 5).is_none());
    }

    #[test]
    fn test_buffer_zero_area() {
        let buf = Buffer::empty(Rect::default());
        assert_eq!(buf.area.width, 0);
        assert_eq!(buf.area.height, 0);
        assert_eq!(buf.len(), 0);
        assert!(buf.is_empty());

        assert!(buf.get(0, 0).is_none());
    }

    #[test]
    fn test_buffer_one_by_one() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 1));
        assert_eq!(buf.area.width, 1);
        assert_eq!(buf.area.height, 1);
        assert_eq!(buf.len(), 1);
        assert!(!buf.is_empty());

        buf.set(0, 0, Cell::new("X"));
        assert_eq!(buf.get(0, 0).unwrap().symbol, "X");

        assert!(buf.get(1, 0).is_none());
        assert!(buf.get(0, 1).is_none());
    }

    #[test]
    fn test_buffer_very_large() {
        let buf = Buffer::empty(Rect::new(0, 0, 1000, 1000));
        assert_eq!(buf.area.width, 1000);
        assert_eq!(buf.area.height, 1000);
        assert_eq!(buf.len(), 1_000_000);
        assert!(!buf.is_empty());

        assert!(buf.get(0, 0).is_some());
        assert!(buf.get(999, 999).is_some());
        assert!(buf.get(999, 0).is_some());
        assert!(buf.get(0, 999).is_some());

        assert!(buf.get(1000, 0).is_none());
        assert!(buf.get(0, 1000).is_none());
    }

    #[test]
    fn test_buffer_multi_width_char_at_boundary() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 5));

        buf.set(8, 0, Cell::new("😀"));
        assert_eq!(buf.get(8, 0).unwrap().symbol, "😀");
        assert_eq!(buf.get(8, 0).unwrap().width(), 2);

        buf.set(9, 0, Cell::new("🎉"));
        assert_eq!(buf.get(9, 0).unwrap().symbol, "🎉");

        buf.set(5, 2, Cell::new("🚀"));
        assert_eq!(buf.get(5, 2).unwrap().symbol, "🚀");
    }

    #[test]
    fn test_buffer_symbol_longer_than_4_chars() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 10));

        let long_symbol = "👨‍👩‍👧‍👦";
        buf.set(0, 0, Cell::new(long_symbol));
        assert_eq!(buf.get(0, 0).unwrap().symbol, long_symbol);

        let another_long = "🇬🇧🇺🇸";
        buf.set(5, 5, Cell::new(another_long));
        assert_eq!(buf.get(5, 5).unwrap().symbol, another_long);

        let text_symbol = "Hello";
        buf.set(10, 0, Cell::new(text_symbol));
        assert_eq!(buf.get(10, 0).unwrap().symbol, text_symbol);
    }

    #[test]
    fn test_buffer_multi_width_cjk_at_boundary() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 5, 5));

        buf.set(3, 0, Cell::new("あ"));
        assert_eq!(buf.get(3, 0).unwrap().symbol, "あ");

        buf.set(4, 1, Cell::new("日"));
        assert_eq!(buf.get(4, 1).unwrap().symbol, "日");

        buf.set(0, 0, Cell::new("中"));
        assert_eq!(buf.get(0, 0).unwrap().symbol, "中");
    }

    #[test]
    fn test_buffer_skip_flag_placement() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));

        let mut wide_cell = Cell::new("😀");
        wide_cell.skip = true;

        buf.set(0, 0, wide_cell.clone());

        let cell = buf.get(0, 0).unwrap();
        assert!(cell.skip);
        assert_eq!(cell.symbol, "😀");

        let mut cell = buf.get(0, 0).unwrap();
        cell.skip = false;
        buf.set(0, 0, cell);
        assert!(!buf.get(0, 0).unwrap().skip);
    }
}
