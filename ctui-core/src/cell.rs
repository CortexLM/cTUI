//! Cell type for terminal buffer

use crate::style::{Color, Modifier, Style};
use std::fmt;
use unicode_width::UnicodeWidthStr;

/// A single cell in the terminal buffer
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    /// The character(s) to display (supports multi-width Unicode)
    pub symbol: String,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text modifiers (bold, italic, etc.)
    pub modifier: Modifier,
    /// Whether this cell is skipped (used by trailing cells of wide characters)
    pub skip: bool,
}

impl Cell {
    /// Creates a new cell with the given symbol
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            fg: Color::default(),
            bg: Color::default(),
            modifier: Modifier::default(),
            skip: false,
        }
    }

    /// Returns the display width of the cell's symbol
    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self.symbol.as_str())
    }

    /// Sets the symbol
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
        self
    }

    /// Sets the foreground color
    pub fn set_fg(&mut self, color: Color) -> &mut Self {
        self.fg = color;
        self
    }

    /// Sets the background color
    pub fn set_bg(&mut self, color: Color) -> &mut Self {
        self.bg = color;
        self
    }

    /// Sets the style (fg, bg, and modifier)
    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.fg = style.fg;
        self.bg = style.bg;
        self.modifier = style.modifier;
        self
    }

    /// Sets whether this cell should be skipped
    pub fn set_skip(&mut self, skip: bool) -> &mut Self {
        self.skip = skip;
        self
    }

    /// Resets the cell to default state
    pub fn reset(&mut self) {
        self.symbol.clear();
        self.symbol.push(' ');
        self.fg = Color::default();
        self.bg = Color::default();
        self.modifier = Modifier::default();
        self.skip = false;
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: " ".to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
            skip: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_new() {
        let cell = Cell::new("A");
        assert_eq!(cell.symbol, "A");
        assert_eq!(cell.fg, Color::Reset);
        assert_eq!(cell.bg, Color::Reset);
        assert_eq!(cell.modifier, Modifier::empty());
        assert!(!cell.skip);
    }

    #[test]
    fn test_cell_width_ascii() {
        let cell = Cell::new("A");
        assert_eq!(cell.width(), 1);
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
    fn test_cell_setters() {
        let mut cell = Cell::new("X");
        cell.set_fg(Color::Red)
            .set_bg(Color::Blue)
            .set_style(Style::new().fg(Color::Green).bg(Color::Yellow));

        assert_eq!(cell.fg, Color::Green);
        assert_eq!(cell.bg, Color::Yellow);
    }

    #[test]
    fn test_cell_reset() {
        let mut cell = Cell::new("Test");
        cell.fg = Color::Red;
        cell.bg = Color::Blue;
        cell.modifier = Modifier::BOLD;
        cell.skip = true;

        cell.reset();

        assert_eq!(cell.symbol, " ");
        assert_eq!(cell.fg, Color::Reset);
        assert_eq!(cell.bg, Color::Reset);
        assert_eq!(cell.modifier, Modifier::empty());
        assert!(!cell.skip);
    }
}
