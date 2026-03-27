//! `TestBackend` for unit testing widgets
//!
//! This module provides a mock terminal backend that enables testing widgets
//! and UI components without a real terminal. The `TestBackend` captures all
//! rendered output in a buffer that can be inspected and asserted against.
//!
//! # Example
//!
//! ```ignore
//! use ctui_core::backend::test::TestBackend;
//! use ctui_core::{Buffer, Cell, Rect};
//!
//! let mut backend = TestBackend::new(20, 10);
//!
//! // Render content
//! backend.draw(vec![
//!     (0, 0, &Cell::new("H")),
//!     (1, 0, &Cell::new("i")),
//! ].into_iter())?;
//!
//! // Verify output
//! assert_eq!(backend.buffer()[(0, 0)].symbol, "H");
//! assert_eq!(backend.buffer()[(1, 0)].symbol, "i");
//!
//! // Or use assertions
//! let expected = Buffer::empty(Rect::new(0, 0, 20, 10));
//! backend.assert_buffer(&expected);
//! ```

use crate::buffer::Buffer;
use crate::cell::Cell;
use crate::geometry::Rect;
use std::fmt;
use std::io::Result;
use std::ops::{Index, IndexMut};

/// A test backend that captures rendered output in a buffer
///
/// This backend is designed for unit testing widgets without requiring
/// a real terminal. It implements the Backend trait and provides methods
/// to inspect and assert on the rendered content.
#[derive(Clone, Debug)]
pub struct TestBackend {
    /// The main buffer for rendered content
    buffer: Buffer,
    /// Scrollback buffer for terminal history (not used in basic testing)
    scrollback: Buffer,
    /// Current cursor position
    cursor_pos: (u16, u16),
    /// Whether cursor is visible
    cursor_visible: bool,
}

impl TestBackend {
    /// Creates a new `TestBackend` with the given dimensions
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the terminal in columns
    /// * `height` - Height of the terminal in rows
    ///
    /// # Example
    ///
    /// ```ignore
    /// let backend = TestBackend::new(80, 24);
    /// assert_eq!(backend.size(), Rect::new(0, 0, 80, 24));
    /// ```
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        let area = Rect::new(0, 0, width, height);
        Self {
            buffer: Buffer::empty(area),
            scrollback: Buffer::empty(area),
            cursor_pos: (0, 0),
            cursor_visible: false,
        }
    }

    /// Returns a reference to the current buffer
    #[must_use]
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Returns a mutable reference to the current buffer
    pub const fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    /// Returns the dimensions of the backend
    #[must_use]
    pub const fn size(&self) -> Rect {
        self.buffer.area
    }

    /// Clears the buffer to the default state
    pub fn clear_screen(&mut self) {
        self.buffer.reset();
    }

    /// Gets the current cursor position
    #[must_use]
    pub const fn cursor_pos(&self) -> (u16, u16) {
        self.cursor_pos
    }

    /// Sets the cursor position
    pub fn set_cursor_pos(&mut self, x: u16, y: u16) {
        self.cursor_pos = (
            x.min(self.buffer.area.width),
            y.min(self.buffer.area.height),
        );
    }

    /// Hides the cursor
    pub const fn hide_cursor(&mut self) {
        self.cursor_visible = false;
    }

    /// Shows the cursor
    pub const fn show_cursor(&mut self) {
        self.cursor_visible = true;
    }

    /// Returns whether the cursor is visible
    #[must_use]
    pub const fn is_cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    /// Asserts that the buffer matches the expected buffer
    ///
    /// This compares each cell in the buffer and panics with a detailed
    /// message if any cell differs.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut backend = TestBackend::new(10, 5);
    /// let expected = Buffer::empty(Rect::new(0, 0, 10, 5));
    /// backend.assert_buffer(&expected); // passes
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the buffer areas don't match or if any cell differs.
    pub fn assert_buffer(&self, expected: &Buffer) {
        assert_eq!(
            self.buffer.area, expected.area,
            "Area mismatch: got {:?}, expected {:?}",
            self.buffer.area, expected.area
        );

        for y in 0..self.buffer.area.height {
            for x in 0..self.buffer.area.width {
                let actual_cell = &self.buffer[(x, y)];
                let expected_cell = &expected[(x, y)];

                assert_eq!(
                    actual_cell, expected_cell,
                    "Cell mismatch at ({x}, {y}):\n  actual:   {actual_cell:?}\n  expected: {expected_cell:?}"
                );
            }
        }
    }

    /// Asserts that each line of the buffer matches the expected strings
    ///
    /// This is useful for quick assertions where you only care about the
    /// text content, not the styling.
    ///
    /// # Arguments
    ///
    /// * `expected` - An iterator of strings, one per line
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut backend = TestBackend::new(10, 3);
    /// backend.draw(vec![(0, 0, &Cell::new("H")), (1, 0, &Cell::new("i"))].into_iter()).unwrap();
    ///
    /// backend.assert_buffer_lines(["Hi        ", "          ", "          "]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the expected lines exceed buffer height or if any character differs.
    #[allow(clippy::cast_possible_truncation)]
    pub fn assert_buffer_lines<Lines>(&self, expected: Lines)
    where
        Lines: IntoIterator,
        Lines::Item: AsRef<str>,
    {
        let expected_lines: Vec<String> = expected
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();

        for (y, expected_line) in expected_lines.iter().enumerate() {
            let y = y as u16;
            assert!(
                y < self.buffer.area.height,
                "Expected line {y} exceeds buffer height {}",
                self.buffer.area.height
            );

            for (x, expected_char) in expected_line.chars().enumerate() {
                let x = x as u16;
                if x >= self.buffer.area.width {
                    break;
                }

                let actual_cell = &self.buffer[(x, y)];
                let expected_symbol = expected_char.to_string();

                assert_eq!(
                    actual_cell.symbol, expected_symbol,
                    "Character mismatch at ({x}, {y}): got {:?}, expected {:?}",
                    actual_cell.symbol, expected_symbol
                );
            }

            // Check remaining cells in the line are empty
            let expected_line_len = expected_line.len() as u16;
            if expected_line_len < self.buffer.area.width {
                for x in expected_line_len..self.buffer.area.width {
                    let cell = &self.buffer[(x, y)];
                    assert_eq!(
                        cell.symbol, " ",
                        "Expected empty cell at ({x}, {y}), got {:?}",
                        cell.symbol
                    );
                }
            }
        }

        // Check that we don't have more lines than expected
        assert_eq!(
            expected_lines.len(),
            self.buffer.area.height as usize,
            "Expected {} lines, but buffer has {} lines",
            expected_lines.len(),
            self.buffer.area.height
        );
    }

    /// Renders the buffer as a string for snapshot testing
    ///
    /// Each line of the buffer is rendered as a string, with cells
    /// joined together. This is useful for visual debugging and
    /// snapshot testing with tools like `insta`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut backend = TestBackend::new(3, 2);
    /// backend.draw(vec![
    ///     (0, 0, &Cell::new("A")),
    ///     (1, 0, &Cell::new("B")),
    ///     (2, 0, &Cell::new("C")),
    /// ].into_iter()).unwrap();
    ///
    /// let output = backend.to_string();
    /// assert_eq!(output, "ABC\n   ");
    /// ```
    fn render_buffer(&self) -> String {
        let mut output = String::with_capacity(self.buffer.len());

        for y in 0..self.buffer.area.height {
            for x in 0..self.buffer.area.width {
                output.push_str(&self.buffer[(x, y)].symbol);
            }
            if y < self.buffer.area.height - 1 {
                output.push('\n');
            }
        }

        output
    }

    /// Renders a line of the buffer as a string
    ///
    /// # Arguments
    ///
    /// * `y` - The line number (0-indexed)
    ///
    /// # Panics
    ///
    /// Panics if `y` is out of bounds.
    #[must_use]
    pub fn line_to_string(&self, y: u16) -> String {
        assert!(y < self.buffer.area.height, "Line {y} out of bounds");

        let mut line = String::with_capacity(self.buffer.area.width as usize);
        for x in 0..self.buffer.area.width {
            line.push_str(&self.buffer[(x, y)].symbol);
        }
        line
    }

    /// Returns the scrolled content (terminal history)
    #[must_use]
    pub const fn scrollback(&self) -> &Buffer {
        &self.scrollback
    }

    /// Appends current buffer to scrollback
    pub fn scroll_up(&mut self, lines: u16) {
        // Simple scroll implementation: move content up by `lines` rows
        if lines >= self.buffer.area.height {
            self.buffer.reset();
            return;
        }

        for y in 0..(self.buffer.area.height - lines) {
            for x in 0..self.buffer.area.width {
                let src = self.buffer[(x, y + lines)].clone();
                self.buffer[(x, y)] = src;
            }
        }
        // Clear the bottom lines
        for y in (self.buffer.area.height - lines)..self.buffer.area.height {
            for x in 0..self.buffer.area.width {
                self.buffer[(x, y)].reset();
            }
        }
    }
}

impl Index<(u16, u16)> for TestBackend {
    type Output = Cell;

    fn index(&self, pos: (u16, u16)) -> &Self::Output {
        &self.buffer[pos]
    }
}

impl IndexMut<(u16, u16)> for TestBackend {
    fn index_mut(&mut self, pos: (u16, u16)) -> &mut Self::Output {
        &mut self.buffer[pos]
    }
}

impl fmt::Display for TestBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_buffer())
    }
}

impl super::Backend for TestBackend {
    /// Draws content to the buffer
    ///
    /// Each item in the iterator is a tuple of (x, y, cell).
    fn draw<'a, I>(&mut self, content: I) -> Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            if x < self.buffer.area.width && y < self.buffer.area.height {
                self.buffer[(x, y)] = cell.clone();
            }
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        self.buffer.reset();
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        // No-op for test backend
        Ok(())
    }

    fn size(&self) -> Result<Rect> {
        Ok(self.buffer.area)
    }

    fn cursor_pos(&self) -> Result<(u16, u16)> {
        Ok(self.cursor_pos)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        self.cursor_pos = (x, y);
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        self.cursor_visible = true;
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        self.cursor_visible = false;
        Ok(())
    }

    fn scroll_up(&mut self, n: u16) -> Result<()> {
        self.scroll_up(n);
        Ok(())
    }

    fn scroll_down(&mut self, n: u16) -> Result<()> {
        if n >= self.buffer.area.height {
            self.buffer.reset();
            return Ok(());
        }

        for y in (0..(self.buffer.area.height - n)).rev() {
            for x in 0..self.buffer.area.width {
                let src_y = y;
                let dst_y = y + n;
                let src = self.buffer[(x, src_y)].clone();
                self.buffer[(x, dst_y)] = src;
            }
        }
        for y in 0..n {
            for x in 0..self.buffer.area.width {
                self.buffer[(x, y)].reset();
            }
        }
        Ok(())
    }

    fn set_title(&mut self, title: &str) -> Result<()> {
        let _ = title;
        Ok(())
    }

    fn enter_alternate_screen(&mut self) -> Result<()> {
        Ok(())
    }

    fn leave_alternate_screen(&mut self) -> Result<()> {
        Ok(())
    }

    fn is_alternate_screen(&self) -> bool {
        false
    }

    fn clear_region(&mut self, area: Rect) -> Result<()> {
        for y in area.y..area.y.saturating_add(area.height) {
            for x in area.x..area.x.saturating_add(area.width) {
                if let Some(cell) = self.buffer.get_mut(x, y) {
                    cell.reset();
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Backend;
    use super::*;

    #[test]
    fn test_test_backend_new() {
        let backend = TestBackend::new(80, 24);

        assert_eq!(backend.size(), Rect::new(0, 0, 80, 24));
        assert_eq!(backend.buffer().len(), 80 * 24);
        assert_eq!(backend.cursor_pos(), (0, 0));
        assert!(!backend.is_cursor_visible());
    }

    #[test]
    fn test_test_backend_draw() {
        let mut backend = TestBackend::new(10, 5);

        backend
            .draw(
                vec![
                    (0, 0, &Cell::new("H")),
                    (1, 0, &Cell::new("e")),
                    (2, 0, &Cell::new("l")),
                    (3, 0, &Cell::new("l")),
                    (4, 0, &Cell::new("o")),
                ]
                .into_iter(),
            )
            .unwrap();

        assert_eq!(backend[(0, 0)].symbol, "H");
        assert_eq!(backend[(1, 0)].symbol, "e");
        assert_eq!(backend[(2, 0)].symbol, "l");
        assert_eq!(backend[(3, 0)].symbol, "l");
        assert_eq!(backend[(4, 0)].symbol, "o");

        // Check that other cells are empty
        assert_eq!(backend[(5, 0)].symbol, " ");
        assert_eq!(backend[(0, 1)].symbol, " ");
    }

    #[test]
    fn test_test_backend_draw_out_of_bounds() {
        let mut backend = TestBackend::new(5, 5);

        // Drawing out of bounds should not panic
        backend
            .draw(
                vec![
                    (10, 0, &Cell::new("X")), // Out of bounds
                    (0, 10, &Cell::new("Y")), // Out of bounds
                    (2, 2, &Cell::new("Z")),  // In bounds
                ]
                .into_iter(),
            )
            .unwrap();

        // Only the in-bounds cell should be drawn
        assert_eq!(backend[(2, 2)].symbol, "Z");

        // The backend should not have crashed
        assert_eq!(backend.buffer().len(), 25);
    }

    #[test]
    fn test_assert_buffer() {
        let mut backend = TestBackend::new(10, 5);
        let expected = Buffer::empty(Rect::new(0, 0, 10, 5));

        // Should not panic for matching buffers
        backend.assert_buffer(&expected);

        // Draw some content
        backend
            .draw(vec![(0, 0, &Cell::new("X"))].into_iter())
            .unwrap();

        // Should panic for non-matching buffer
        let result = std::panic::catch_unwind(|| {
            backend.assert_buffer(&expected);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_assert_buffer_lines() {
        let mut backend = TestBackend::new(5, 2);

        backend
            .draw(
                vec![
                    (0, 0, &Cell::new("H")),
                    (1, 0, &Cell::new("i")),
                    (0, 1, &Cell::new("B")),
                    (1, 1, &Cell::new("y")),
                    (2, 1, &Cell::new("e")),
                ]
                .into_iter(),
            )
            .unwrap();

        backend.assert_buffer_lines(&["Hi   ", "Bye  "]);
    }

    #[test]
    #[should_panic(expected = "Character mismatch")]
    fn test_assert_buffer_lines_mismatch() {
        let mut backend = TestBackend::new(5, 2);

        backend
            .draw(vec![(0, 0, &Cell::new("H")), (1, 0, &Cell::new("i"))].into_iter())
            .unwrap();

        // This should panic
        backend.assert_buffer_lines(&["Hello", "     "]);
    }

    #[test]
    fn test_to_string() {
        let mut backend = TestBackend::new(5, 2);

        backend
            .draw(
                vec![
                    (0, 0, &Cell::new("A")),
                    (1, 0, &Cell::new("B")),
                    (2, 0, &Cell::new("C")),
                    (0, 1, &Cell::new("D")),
                    (1, 1, &Cell::new("E")),
                ]
                .into_iter(),
            )
            .unwrap();

        let output = backend.to_string();
        assert_eq!(output, "ABC  \nDE   ");
    }

    #[test]
    fn test_line_to_string() {
        let mut backend = TestBackend::new(10, 3);

        backend
            .draw(
                vec![
                    (0, 0, &Cell::new("L")),
                    (1, 0, &Cell::new("1")),
                    (0, 1, &Cell::new("L")),
                    (1, 1, &Cell::new("2")),
                    (0, 2, &Cell::new("L")),
                    (1, 2, &Cell::new("3")),
                ]
                .into_iter(),
            )
            .unwrap();

        assert_eq!(backend.line_to_string(0), "L1        ");
        assert_eq!(backend.line_to_string(1), "L2        ");
        assert_eq!(backend.line_to_string(2), "L3        ");
    }

    #[test]
    fn test_cursor_operations() {
        let mut backend = TestBackend::new(80, 24);

        backend.set_cursor_pos(10, 5);
        assert_eq!(backend.cursor_pos(), (10, 5));

        backend.show_cursor();
        assert!(backend.is_cursor_visible());

        backend.hide_cursor();
        assert!(!backend.is_cursor_visible());
    }

    #[test]
    fn test_scroll() {
        let mut backend = TestBackend::new(5, 3);

        // Fill with different content per line
        backend
            .draw(
                vec![
                    (0, 0, &Cell::new("A")),
                    (0, 1, &Cell::new("B")),
                    (0, 2, &Cell::new("C")),
                ]
                .into_iter(),
            )
            .unwrap();

        backend.scroll_up(1);

        assert_eq!(backend[(0, 0)].symbol, "B");
        assert_eq!(backend[(0, 1)].symbol, "C");
        assert_eq!(backend[(0, 2)].symbol, " ");
    }

    #[test]
    fn test_clear() {
        let mut backend = TestBackend::new(10, 5);

        backend
            .draw(vec![(0, 0, &Cell::new("X"))].into_iter())
            .unwrap();

        backend.clear_screen();

        // All cells should be reset
        for y in 0..5 {
            for x in 0..10 {
                assert_eq!(backend[(x, y)].symbol, " ");
            }
        }
    }

    #[test]
    fn test_backend_trait() {
        let mut backend = TestBackend::new(20, 10);

        // Test draw
        Backend::draw(&mut backend, vec![(0, 0, &Cell::new("T"))].into_iter()).unwrap();
        assert_eq!(backend[(0, 0)].symbol, "T");

        // Test clear
        Backend::clear(&mut backend).unwrap();
        assert_eq!(backend[(0, 0)].symbol, " ");

        // Test size
        assert_eq!(Backend::size(&backend).unwrap(), Rect::new(0, 0, 20, 10));

        // Test cursor
        Backend::set_cursor(&mut backend, 5, 3).unwrap();
        assert_eq!(Backend::cursor_pos(&backend).unwrap(), (5, 3));

        Backend::show_cursor(&mut backend).unwrap();
        Backend::hide_cursor(&mut backend).unwrap();

        // Test flush (no-op)
        Backend::flush(&mut backend).unwrap();
    }
}
