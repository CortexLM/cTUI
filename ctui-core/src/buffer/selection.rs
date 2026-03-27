//! Text selection support for terminal buffer
//!
//! This module provides multi-cell text selection capabilities including:
//! - Mouse-based selection (click, drag, double-click, triple-click)
//! - Selection across multiple lines
//! - Proper handling of wide characters (CJK, emoji)
//! - Selection operations (copy, select all, clear)

use crate::geometry::Position;
use crate::style::Style;
use crate::Buffer;

/// Represents the type of selection being performed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// Normal character-by-character selection
    Normal,
    /// Word selection (double-click)
    Word,
    /// Line selection (triple-click)
    Line,
}

/// A text selection within a buffer
///
/// Supports multi-line selection with proper handling of wide characters.
/// The selection is represented by start and end positions, and tracks
/// whether the selection is currently active (being dragged).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSelection {
    /// Starting position of the selection
    start: Position,
    /// Ending position of the selection (current cursor position during drag)
    end: Position,
    /// Whether the selection is currently active
    active: bool,
    /// The mode of selection (normal, word, line)
    mode: SelectionMode,
    /// Style to apply to selected cells when rendering
    selection_style: Style,
}

impl Default for TextSelection {
    fn default() -> Self {
        Self {
            start: Position::origin(),
            end: Position::origin(),
            active: false,
            mode: SelectionMode::Normal,
            selection_style: Style::default(),
        }
    }
}

impl TextSelection {
    /// Creates a new empty selection
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a selection with a custom style
    pub fn with_style(style: Style) -> Self {
        Self {
            selection_style: style,
            ..Self::default()
        }
    }

    /// Sets the selection style
    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.selection_style = style;
        self
    }

    /// Returns the selection style
    pub fn style(&self) -> &Style {
        &self.selection_style
    }

    /// Starts a new selection at the given position
    ///
    /// This is typically called on mouse down.
    pub fn start(&mut self, pos: Position) {
        self.start = pos;
        self.end = pos;
        self.active = true;
        self.mode = SelectionMode::Normal;
    }

    /// Starts a word selection at the given position
    ///
    /// This is called on double-click. The selection will automatically
    /// extend to word boundaries.
    pub fn start_word(&mut self, pos: Position, buffer: &Buffer) {
        self.start = pos;
        self.end = pos;
        self.active = true;
        self.mode = SelectionMode::Word;
        self.expand_to_word(buffer);
    }

    /// Starts a line selection at the given position
    ///
    /// This is called on triple-click. The entire line will be selected.
    pub fn start_line(&mut self, pos: Position, buffer: &Buffer) {
        self.start = Position::new(0, pos.y);
        self.end = Position::new(buffer.area.width.saturating_sub(1), pos.y);
        self.active = true;
        self.mode = SelectionMode::Line;
    }

    /// Extends the selection to the given position
    ///
    /// This is typically called on mouse drag.
    pub fn extend(&mut self, pos: Position) {
        if !self.active {
            return;
        }
        self.end = pos;
    }

    /// Extends the selection, handling word/line mode appropriately
    pub fn extend_with_mode(&mut self, pos: Position, buffer: &Buffer) {
        if !self.active {
            return;
        }
        self.end = pos;

        if self.mode == SelectionMode::Word {
            self.expand_to_word_boundary(buffer);
        }
    }

    /// Ends the selection at the given position
    ///
    /// This is typically called on mouse up.
    pub fn end(&mut self, pos: Position) {
        self.end = pos;
        self.active = false;
    }

    /// Clears the selection
    pub fn clear(&mut self) {
        self.start = Position::origin();
        self.end = Position::origin();
        self.active = false;
        self.mode = SelectionMode::Normal;
    }

    /// Returns true if the selection is active (being dragged)
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns true if the selection is empty (no cells selected)
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns the selection mode
    pub fn mode(&self) -> SelectionMode {
        self.mode
    }

    /// Returns the starting position
    pub fn start_pos(&self) -> Position {
        self.start
    }

    /// Returns the ending position
    pub fn end_pos(&self) -> Position {
        self.end
    }

    /// Returns the normalized range (start <= end)
    ///
    /// Returns (min_position, max_position) where min is top-left
    /// and max is bottom-right of the selection.
    pub fn to_range(&self) -> (Position, Position) {
        if self.start.y < self.end.y || (self.start.y == self.end.y && self.start.x <= self.end.x) {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        }
    }

    /// Checks if a cell at the given position is within the selection
    ///
    /// Handles multi-line selections correctly.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        let (start, end) = self.to_range();

        if y < start.y || y > end.y {
            return false;
        }

        if y == start.y && x < start.x {
            return false;
        }

        if y == end.y && x > end.x {
            return false;
        }

        true
    }

    /// Checks if a cell position should be included, accounting for wide characters
    ///
    /// For wide characters (CJK, emoji), the selection includes all cells
    /// that are part of the same character.
    pub fn contains_with_wide(&self, x: u16, y: u16, buffer: &Buffer) -> bool {
        if self.contains(x, y) {
            return true;
        }

        // Check if this is a trailing cell of a wide character
        // that started in the selection
        if x > 0 {
            let prev_x = x - 1;
            if let Some(prev_cell) = buffer.get(prev_x, y) {
                if prev_cell.skip {
                    // This is a trailing cell, check if leading cell is selected
                    // Find the leading cell
                    let mut lead_x = prev_x;
                    while lead_x > 0 {
                        if let Some(cell) = buffer.get(lead_x, y) {
                            if cell.skip {
                                lead_x -= 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    return self.contains(lead_x, y);
                }
            }
        }

        false
    }

    /// Returns the selected text from the buffer
    ///
    /// Properly handles:
    /// - Multi-line selections
    /// - Wide characters (CJK, emoji)
    /// - Skip cells (trailing cells of wide characters)
    pub fn selected_text(&self, buffer: &Buffer) -> String {
        if self.is_empty() {
            return String::new();
        }

        let (start, end) = self.to_range();
        let mut result = String::new();

        for y in start.y..=end.y {
            if y >= buffer.area.y + buffer.area.height {
                break;
            }

            // Determine x range for this line
            let x_start = if y == start.y { start.x } else { buffer.area.x };
            let x_end = if y == end.y {
                end.x
            } else {
                buffer.area.x + buffer.area.width - 1
            };

            for x in x_start..=x_end {
                if x >= buffer.area.x + buffer.area.width {
                    break;
                }

                let buf_x = x;
                let buf_y = y;

                if let Some(cell) = buffer.get(buf_x, buf_y) {
                    // Skip trailing cells of wide characters
                    if cell.skip {
                        continue;
                    }

                    // Only include if within selection (with wide char handling)
                    if self.contains_with_wide(x, y, buffer) {
                        result.push_str(&cell.symbol);
                    }
                }
            }

            // Add newline between lines (but not after last line)
            if y < end.y {
                result.push('\n');
            }
        }

        result
    }

    /// Selects all text in the buffer
    pub fn select_all(&mut self, buffer: &Buffer) {
        self.start = Position::new(buffer.area.x, buffer.area.y);
        self.end = Position::new(
            buffer.area.x + buffer.area.width.saturating_sub(1),
            buffer.area.y + buffer.area.height.saturating_sub(1),
        );
        self.active = false;
        self.mode = SelectionMode::Normal;
    }

    /// Selects a word at the given position
    ///
    /// A word is defined as a sequence of alphanumeric characters or
    /// a sequence of non-whitespace non-alphanumeric characters.
    pub fn select_word_at(&mut self, pos: Position, buffer: &Buffer) {
        self.start_word(pos, buffer);
    }

    /// Expands selection to word boundaries
    fn expand_to_word(&mut self, buffer: &Buffer) {
        let pos = self.start;

        if let Some(cell) = buffer.get(pos.x, pos.y) {
            let ch = cell.symbol.chars().next().unwrap_or(' ');

            if ch.is_whitespace() {
                return;
            }

            // Find word start
            let mut word_start_x = pos.x;
            while word_start_x > buffer.area.x {
                if let Some(prev_cell) = buffer.get(word_start_x - 1, pos.y) {
                    let prev_ch = prev_cell.symbol.chars().next().unwrap_or(' ');
                    if prev_ch.is_whitespace() || Self::is_word_boundary(ch, prev_ch) {
                        break;
                    }
                    word_start_x -= 1;
                    // Skip trailing cells of wide characters
                    if prev_cell.skip && word_start_x > buffer.area.x {
                        word_start_x -= 1;
                    }
                } else {
                    break;
                }
            }

            // Find word end
            let mut word_end_x = pos.x;
            let max_x = buffer.area.x + buffer.area.width;
            while word_end_x < max_x - 1 {
                if let Some(next_cell) = buffer.get(word_end_x + 1, pos.y) {
                    let next_ch = next_cell.symbol.chars().next().unwrap_or(' ');
                    if next_ch.is_whitespace()
                        || Self::is_word_boundary(ch, next_ch)
                        || next_cell.skip
                    {
                        break;
                    }
                    word_end_x += 1;
                } else {
                    break;
                }
            }

            self.start = Position::new(word_start_x, pos.y);
            self.end = Position::new(word_end_x, pos.y);
        }
    }

    /// Expands to word boundary when extending selection in word mode
    fn expand_to_word_boundary(&mut self, buffer: &Buffer) {
        let end_pos = self.end;

        if let Some(cell) = buffer.get(end_pos.x, end_pos.y) {
            let ch = cell.symbol.chars().next().unwrap_or(' ');

            if ch.is_whitespace() {
                return;
            }

            // Expand to include the word at end position
            let mut word_end_x = end_pos.x;
            let max_x = buffer.area.x + buffer.area.width;

            while word_end_x < max_x - 1 {
                if let Some(next_cell) = buffer.get(word_end_x + 1, end_pos.y) {
                    let next_ch = next_cell.symbol.chars().next().unwrap_or(' ');
                    if next_ch.is_whitespace()
                        || Self::is_word_boundary(ch, next_ch)
                        || next_cell.skip
                    {
                        break;
                    }
                    word_end_x += 1;
                } else {
                    break;
                }
            }

            self.end = Position::new(word_end_x, end_pos.y);
        }
    }

    /// Checks if two characters are on different word boundaries
    fn is_word_boundary(a: char, b: char) -> bool {
        (a.is_alphanumeric() && !b.is_alphanumeric())
            || (!a.is_alphanumeric() && b.is_alphanumeric())
    }

    /// Returns an iterator over all selected positions
    pub fn positions(&self) -> SelectedPositions {
        SelectedPositions::new(self)
    }

    /// Applies selection style to a cell
    ///
    /// Returns true if the style was applied (cell was in selection)
    pub fn apply_to_cell(
        &self,
        x: u16,
        y: u16,
        cell: &mut crate::cell::Cell,
        buffer: &Buffer,
    ) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.contains_with_wide(x, y, buffer) {
            if self.selection_style.fg != crate::style::Color::Reset {
                cell.fg = self.selection_style.fg;
            }
            if self.selection_style.bg != crate::style::Color::Reset {
                cell.bg = self.selection_style.bg;
            }
            cell.modifier |= self.selection_style.modifier;
            return true;
        }

        false
    }

    /// Renders the selection highlight onto the buffer
    ///
    /// This applies the selection style to all cells within the selection.
    pub fn render(&self, buffer: &mut Buffer) {
        if self.is_empty() {
            return;
        }

        let (start, end) = self.to_range();
        let area = buffer.area;

        for y in start.y..=end.y {
            if y >= area.y + area.height {
                break;
            }

            let x_start = if y == start.y { start.x } else { area.x };
            let x_end = if y == end.y {
                end.x
            } else {
                area.x + area.width - 1
            };

            for x in x_start..=x_end {
                if x >= area.x + area.width {
                    break;
                }

                // Check if this cell should be selected (before getting mutable ref)
                let should_style = self.contains_with_wide_impl(x, y, buffer);

                if should_style {
                    if let Some(cell) = buffer.get_mut(x, y) {
                        if self.selection_style.fg != crate::style::Color::Reset {
                            cell.fg = self.selection_style.fg;
                        }
                        if self.selection_style.bg != crate::style::Color::Reset {
                            cell.bg = self.selection_style.bg;
                        }
                        cell.modifier |= self.selection_style.modifier;
                    }
                }
            }
        }
    }

    /// Internal implementation of contains_with_wide that doesn't need a separate borrow
    fn contains_with_wide_impl(&self, x: u16, y: u16, buffer: &Buffer) -> bool {
        if self.contains(x, y) {
            return true;
        }

        if x > 0 {
            let prev_x = x - 1;
            if let Some(prev_cell) = buffer.get(prev_x, y) {
                if prev_cell.skip {
                    let mut lead_x = prev_x;
                    while lead_x > 0 {
                        if let Some(cell) = buffer.get(lead_x, y) {
                            if cell.skip {
                                lead_x -= 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    return self.contains(lead_x, y);
                }
            }
        }

        false
    }
}

/// Iterator over all positions within a selection
pub struct SelectedPositions {
    start: Position,
    end: Position,
    current_x: u16,
    current_y: u16,
    row_start_x: u16,
    done: bool,
}

impl SelectedPositions {
    fn new(selection: &TextSelection) -> Self {
        let (start, end) = selection.to_range();
        Self {
            start,
            end,
            current_x: start.x,
            current_y: start.y,
            row_start_x: start.x,
            done: start == end,
        }
    }
}

impl Iterator for SelectedPositions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let pos = Position::new(self.current_x, self.current_y);

        if self.current_y == self.end.y {
            // On last row - only go to end.x
            if self.current_x >= self.end.x {
                self.done = true;
            } else {
                self.current_x += 1;
            }
        } else {
            // Not on last row
            self.current_x += 1;
            // This assumes a standard terminal width, which may not be accurate
            // For proper handling, we'd need to pass buffer width
            // For now, wrap at a reasonable position
            if self.current_x > self.end.x + 100 {
                self.current_y += 1;
                self.row_start_x = self.start.x;
                self.current_x = self.row_start_x;
            }
        }

        Some(pos)
    }
}

/// Helper for detecting double/triple clicks
#[derive(Debug, Clone)]
pub struct ClickDetector {
    last_click_pos: Option<Position>,
    last_click_time: Option<std::time::Instant>,
    click_count: u8,
    double_click_threshold_ms: u64,
    triple_click_threshold_ms: u64,
}

impl Default for ClickDetector {
    fn default() -> Self {
        Self {
            last_click_pos: None,
            last_click_time: None,
            click_count: 0,
            double_click_threshold_ms: 500,
            triple_click_threshold_ms: 600,
        }
    }
}

impl ClickDetector {
    /// Creates a new click detector with default thresholds
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a click detector with custom thresholds
    pub fn with_thresholds(double_click_ms: u64, triple_click_ms: u64) -> Self {
        Self {
            last_click_pos: None,
            last_click_time: None,
            click_count: 0,
            double_click_threshold_ms: double_click_ms,
            triple_click_threshold_ms: triple_click_ms,
        }
    }

    /// Records a click and returns the type (single, double, or triple)
    pub fn click(&mut self, pos: Position) -> ClickType {
        let now = std::time::Instant::now();

        let is_same_pos = self.last_click_pos.map_or(false, |p| p == pos);

        let click_type = if let Some(last_time) = self.last_click_time {
            let elapsed = now.duration_since(last_time).as_millis() as u64;

            if is_same_pos && elapsed < self.triple_click_threshold_ms {
                self.click_count += 1;
                match self.click_count {
                    2 => ClickType::Double,
                    3.. => {
                        // Cap at 3
                        self.click_count = 3;
                        ClickType::Triple
                    }
                    _ => ClickType::Single,
                }
            } else if is_same_pos && elapsed < self.double_click_threshold_ms {
                self.click_count = 2;
                ClickType::Double
            } else {
                self.click_count = 1;
                ClickType::Single
            }
        } else {
            self.click_count = 1;
            ClickType::Single
        };

        self.last_click_pos = Some(pos);
        self.last_click_time = Some(now);

        click_type
    }

    /// Resets the click detector
    pub fn reset(&mut self) {
        self.last_click_pos = None;
        self.last_click_time = None;
        self.click_count = 0;
    }
}

/// Type of click detected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickType {
    /// Single click
    Single,
    /// Double click (select word)
    Double,
    /// Triple click (select line)
    Triple,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Rect;
    use crate::style::Color;
    use crate::Buffer;

    fn test_buffer() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 20, 5))
    }

    fn test_buffer_with_text() -> Buffer {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 5));

        // Line 0: "Hello World"
        let line0 = "Hello World";
        for (i, ch) in line0.chars().enumerate() {
            buf[(i as u16, 0)].symbol = ch.to_string();
        }

        // Line 1: "Test Line"
        let line1 = "Test Line";
        for (i, ch) in line1.chars().enumerate() {
            buf[(i as u16, 1)].symbol = ch.to_string();
        }

        buf
    }

    #[test]
    fn test_selection_new() {
        let sel = TextSelection::new();
        assert!(sel.is_empty());
        assert!(!sel.is_active());
        assert_eq!(sel.mode(), SelectionMode::Normal);
    }

    #[test]
    fn test_selection_start() {
        let mut sel = TextSelection::new();
        sel.start(Position::new(5, 2));

        assert!(sel.is_active());
        assert!(sel.is_empty()); // start == end
        assert_eq!(sel.start_pos(), Position::new(5, 2));
        assert_eq!(sel.end_pos(), Position::new(5, 2));
    }

    #[test]
    fn test_selection_extend() {
        let mut sel = TextSelection::new();
        sel.start(Position::new(0, 0));
        sel.extend(Position::new(5, 0));

        assert!(sel.is_active());
        assert!(!sel.is_empty());
        assert_eq!(sel.start_pos(), Position::new(0, 0));
        assert_eq!(sel.end_pos(), Position::new(5, 0));
    }

    #[test]
    fn test_selection_end() {
        let mut sel = TextSelection::new();
        sel.start(Position::new(0, 0));
        sel.extend(Position::new(5, 0));
        sel.end(Position::new(10, 0));

        assert!(!sel.is_active());
        assert_eq!(sel.end_pos(), Position::new(10, 0));
    }

    #[test]
    fn test_selection_clear() {
        let mut sel = TextSelection::new();
        sel.start(Position::new(5, 5));
        sel.extend(Position::new(10, 10));
        sel.clear();

        assert!(sel.is_empty());
        assert!(!sel.is_active());
    }

    #[test]
    fn test_selection_contains() {
        let mut sel = TextSelection::new();
        sel.start(Position::new(2, 1));
        sel.extend(Position::new(8, 3));

        // Positions inside selection
        assert!(sel.contains(2, 1)); // start
        assert!(sel.contains(5, 1)); // middle of first row
        assert!(sel.contains(0, 2)); // middle row (full row)
        assert!(sel.contains(8, 3)); // end

        // Positions outside selection
        assert!(!sel.contains(1, 1)); // before start
        assert!(!sel.contains(9, 3)); // after end
        assert!(!sel.contains(0, 0)); // row before
        assert!(!sel.contains(0, 4)); // row after
    }

    #[test]
    fn test_selection_contains_reverse() {
        let mut sel = TextSelection::new();
        sel.start(Position::new(8, 3)); // end position first
        sel.extend(Position::new(2, 1)); // start position later

        // Should still work correctly with reversed selection
        assert!(sel.contains(2, 1));
        assert!(sel.contains(5, 2));
        assert!(sel.contains(8, 3));
        assert!(!sel.contains(0, 0));
    }

    #[test]
    fn test_selection_to_range() {
        let mut sel = TextSelection::new();
        sel.start(Position::new(5, 2));
        sel.extend(Position::new(2, 1));

        let (start, end) = sel.to_range();
        assert_eq!(start, Position::new(2, 1));
        assert_eq!(end, Position::new(5, 2));
    }

    #[test]
    fn test_selection_selected_text_single_line() {
        let buf = test_buffer_with_text();
        let mut sel = TextSelection::new();
        sel.start(Position::new(0, 0));
        sel.extend(Position::new(4, 0)); // "Hello"

        let text = sel.selected_text(&buf);
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_selection_selected_text_multi_line() {
        let buf = test_buffer_with_text();
        let mut sel = TextSelection::new();
        sel.start(Position::new(6, 0)); // "World"
        sel.extend(Position::new(3, 1)); // "Test"

        let text = sel.selected_text(&buf);
        // Should include "World" (from position 6 to end of line 0)
        // and "Test" (from start to position 3 of line 1)
        assert!(text.contains("World"));
        assert!(text.contains("Test"));
    }

    #[test]
    fn test_selection_select_all() {
        let buf = test_buffer();
        let mut sel = TextSelection::new();
        sel.select_all(&buf);

        assert!(!sel.is_empty());
        // First cell should be selected
        assert!(sel.contains(0, 0));
        // Last cell should be selected
        assert!(sel.contains(19, 4));
    }

    #[test]
    fn test_selection_style() {
        let style = Style::new().fg(Color::Red).bg(Color::Blue);

        let sel = TextSelection::with_style(style);
        assert_eq!(sel.style().fg, Color::Red);
        assert_eq!(sel.style().bg, Color::Blue);
    }

    #[test]
    fn test_selection_word_mode() {
        let buf = test_buffer_with_text();
        let mut sel = TextSelection::new();
        sel.start_word(Position::new(1, 0), &buf); // Click on 'e' in "Hello"

        // Should select "Hello"
        let text = sel.selected_text(&buf);
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_selection_line_mode() {
        let buf = test_buffer_with_text();
        let mut sel = TextSelection::new();
        sel.start_line(Position::new(5, 1), &buf);

        assert_eq!(sel.mode(), SelectionMode::Line);
        // Should select entire line 1
        assert!(sel.contains(0, 1));
        assert!(sel.contains(19, 1));
        assert!(!sel.contains(0, 0));
    }

    #[test]
    fn test_selection_positions() {
        let mut sel = TextSelection::new();
        sel.start(Position::new(0, 0));
        sel.extend(Position::new(2, 0));

        let positions: Vec<_> = sel.positions().collect();
        assert_eq!(positions.len(), 3);
        assert_eq!(positions[0], Position::new(0, 0));
        assert_eq!(positions[1], Position::new(1, 0));
        assert_eq!(positions[2], Position::new(2, 0));
    }

    #[test]
    fn test_click_detector_single() {
        let mut detector = ClickDetector::new();

        let click = detector.click(Position::new(5, 5));
        assert_eq!(click, ClickType::Single);
    }

    #[test]
    fn test_click_detector_double() {
        let mut detector = ClickDetector::new();

        detector.click(Position::new(5, 5));
        let click = detector.click(Position::new(5, 5));
        assert_eq!(click, ClickType::Double);
    }

    #[test]
    fn test_click_detector_triple() {
        let mut detector = ClickDetector::new();

        detector.click(Position::new(5, 5));
        detector.click(Position::new(5, 5));
        let click = detector.click(Position::new(5, 5));
        assert_eq!(click, ClickType::Triple);
    }

    #[test]
    fn test_click_detector_position_change() {
        let mut detector = ClickDetector::new();

        detector.click(Position::new(5, 5));
        let click = detector.click(Position::new(10, 5)); // Different position
        assert_eq!(click, ClickType::Single);
    }

    #[test]
    fn test_render_selection() {
        let mut buf = test_buffer_with_text();

        let style = Style::new().bg(Color::Blue);
        let mut sel = TextSelection::with_style(style);
        sel.start(Position::new(0, 0));
        sel.extend(Position::new(4, 0));

        sel.render(&mut buf);

        // Check that selected cells have the style applied
        for x in 0..=4 {
            let cell = &buf[(x, 0)];
            assert_eq!(cell.bg, Color::Blue);
        }

        // Check that non-selected cells don't have the style
        let cell = &buf[(5, 0)];
        assert_ne!(cell.bg, Color::Blue);
    }

    #[test]
    fn test_wide_character_handling() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));

        // Place a CJK character at position 0 (takes 2 cells)
        buf[(0, 0)].symbol = "あ".to_string();
        buf[(1, 0)].skip = true;

        let mut sel = TextSelection::new();
        sel.start(Position::new(0, 0));
        sel.end(Position::new(1, 0));

        // The selection should include both cells of the wide character
        assert!(sel.contains_with_wide(0, 0, &buf));
        assert!(sel.contains_with_wide(1, 0, &buf));
    }
}
