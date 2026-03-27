//! Style types for terminal cells
//!
//! This module provides color, modifier, and style types for styling terminal cells.

use bitflags::bitflags;

/// Represents a color for terminal output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    /// Reset to default terminal color
    #[default]
    Reset,
    /// Black color
    Black,
    /// Red color
    Red,
    /// Green color
    Green,
    /// Yellow color
    Yellow,
    /// Blue color
    Blue,
    /// Magenta color
    Magenta,
    /// Cyan color
    Cyan,
    /// White color
    White,
    /// Bright black color (dark gray)
    DarkGray,
    /// Bright red color
    LightRed,
    /// Bright green color
    LightGreen,
    /// Bright yellow color
    LightYellow,
    /// Bright blue color
    LightBlue,
    /// Bright magenta color
    LightMagenta,
    /// Bright cyan color
    LightCyan,
    /// Bright white color
    Gray,
    /// Index into 256-color palette
    Indexed(u8),
    /// RGB color (24-bit true color)
    Rgb(u8, u8, u8),
}

bitflags! {
    /// Bitflags for text modifiers (bold, italic, etc.)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Modifier: u16 {
        /// No modifier
        const EMPTY = 0;
        /// Bold text
        const BOLD = 1 << 0;
        /// Dim/faint text
        const DIM = 1 << 1;
        /// Italic text
        const ITALIC = 1 << 2;
        /// Underlined text
        const UNDERLINED = 1 << 3;
        /// Slow blink
        const SLOW_BLINK = 1 << 4;
        /// Rapid blink
        const RAPID_BLINK = 1 << 5;
        /// Reverse video (swap fg/bg)
        const REVERSED = 1 << 6;
        /// Hidden/invisible text
        const HIDDEN = 1 << 7;
        /// Crossed out (strikethrough)
        const CROSSED_OUT = 1 << 8;
    }
}

/// Style represents the visual styling of a cell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text modifiers
    pub modifier: Modifier,
}

impl Style {
    /// Creates a new style with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Sets the background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Sets the modifier
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }

    /// Adds a modifier flag
    pub fn add_modifier(mut self, modifier: Modifier) -> Self {
        self.modifier |= modifier;
        self
    }

    /// Removes a modifier flag
    pub fn remove_modifier(mut self, modifier: Modifier) -> Self {
        self.modifier &= !modifier;
        self
    }

    /// Reset style to default
    pub fn reset() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_default() {
        let color = Color::default();
        assert_eq!(color, Color::Reset);
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new()
            .fg(Color::Red)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD);

        assert_eq!(style.fg, Color::Red);
        assert_eq!(style.bg, Color::Blue);
        assert!(style.modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_modifier_flags() {
        let modifier = Modifier::BOLD | Modifier::ITALIC;
        assert!(modifier.contains(Modifier::BOLD));
        assert!(modifier.contains(Modifier::ITALIC));
        assert!(!modifier.contains(Modifier::UNDERLINED));
    }
}
