//! Style properties for themed components

use crate::color::{Color, NamedColor};
use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    /// Text modifiers for styling
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(try_from = "u16", into = "u16")]
    pub struct Modifier: u16 {
        /// No modifier
        const NONE = 0;
        /// Bold text
        const BOLD = 1 << 0;
        /// Dim/faint text
        const DIM = 1 << 1;
        /// Italic text
        const ITALIC = 1 << 2;
        /// Underlined text
        const UNDERLINE = 1 << 3;
        /// Slow blink
        const SLOW_BLINK = 1 << 4;
        /// Rapid blink
        const RAPID_BLINK = 1 << 5;
        /// Reverse video (swap fg/bg)
        const REVERSED = 1 << 6;
        /// Hidden/invisible text
        const HIDDEN = 1 << 7;
        /// Strikethrough
        const STRIKETHROUGH = 1 << 8;
    }
}

impl Default for Modifier {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<Modifier> for u16 {
    fn from(modifier: Modifier) -> Self {
        modifier.bits()
    }
}

impl TryFrom<u16> for Modifier {
    type Error = String;

    fn try_from(bits: u16) -> Result<Self, Self::Error> {
        Self::from_bits(bits).ok_or_else(|| format!("Invalid modifier bits: {bits}"))
    }
}

/// Spacing values for padding and margin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Spacing {
    /// Top spacing
    pub top: u16,
    /// Right spacing
    pub right: u16,
    /// Bottom spacing
    pub bottom: u16,
    /// Left spacing
    pub left: u16,
}

impl Spacing {
    /// Creates new spacing with all sides equal
    #[must_use]
    pub const fn all(value: u16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Creates spacing with horizontal and vertical values
    #[must_use]
    pub const fn horizontal_vertical(horizontal: u16, vertical: u16) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Creates zero spacing
    #[must_use]
    pub const fn zero() -> Self {
        Self::all(0)
    }

    /// Returns the total horizontal spacing (left + right)
    #[must_use]
    pub const fn horizontal(&self) -> u16 {
        self.left + self.right
    }

    /// Returns the total vertical spacing (top + bottom)
    #[must_use]
    pub const fn vertical(&self) -> u16 {
        self.top + self.bottom
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<u16> for Spacing {
    fn from(value: u16) -> Self {
        Self::all(value)
    }
}

impl From<(u16, u16)> for Spacing {
    fn from((horizontal, vertical): (u16, u16)) -> Self {
        Self::horizontal_vertical(horizontal, vertical)
    }
}

impl From<(u16, u16, u16, u16)> for Spacing {
    fn from((top, right, bottom, left): (u16, u16, u16, u16)) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

/// Border style for components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum BorderStyle {
    /// No border
    #[default]
    None,
    /// Single line border
    Single,
    /// Double line border
    Double,
    /// Rounded corners
    Rounded,
    /// Thick border
    Thick,
}

impl BorderStyle {
    /// Returns the border characters for this style
    #[must_use]
    pub const fn chars(&self) -> BorderChars {
        match self {
            Self::None => BorderChars::empty(),
            Self::Single => BorderChars {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '─',
                vertical: '│',
            },
            Self::Double => BorderChars {
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                horizontal: '═',
                vertical: '║',
            },
            Self::Rounded => BorderChars {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
            },
            Self::Thick => BorderChars {
                top_left: '┏',
                top_right: '┓',
                bottom_left: '┗',
                bottom_right: '┛',
                horizontal: '━',
                vertical: '┃',
            },
        }
    }
}

/// Characters used for drawing borders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BorderChars {
    /// Top-left corner
    pub top_left: char,
    /// Top-right corner
    pub top_right: char,
    /// Bottom-left corner
    pub bottom_left: char,
    /// Bottom-right corner
    pub bottom_right: char,
    /// Horizontal line
    pub horizontal: char,
    /// Vertical line
    pub vertical: char,
}

impl BorderChars {
    /// Creates empty border chars (spaces)
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            top_left: ' ',
            top_right: ' ',
            bottom_left: ' ',
            bottom_right: ' ',
            horizontal: ' ',
            vertical: ' ',
        }
    }
}

/// Complete style for a component
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct Style {
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text modifiers
    pub modifiers: Modifier,
    /// Padding (internal spacing)
    pub padding: Spacing,
    /// Margin (external spacing)
    pub margin: Spacing,
    /// Border style
    pub border_style: BorderStyle,
    /// Border color
    pub border_color: Color,
}

impl Style {
    /// Creates a new style with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the foreground color
    #[must_use]
    pub fn fg(mut self, color: impl Into<Color>) -> Self {
        self.fg = color.into();
        self
    }

    /// Sets the background color
    #[must_use]
    pub fn bg(mut self, color: impl Into<Color>) -> Self {
        self.bg = color.into();
        self
    }

    /// Sets the modifiers
    #[must_use]
    pub const fn modifiers(mut self, modifiers: Modifier) -> Self {
        self.modifiers = modifiers;
        self
    }

    /// Adds a modifier
    #[must_use]
    pub fn add_modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers |= modifier;
        self
    }

    /// Removes a modifier
    #[must_use]
    pub fn remove_modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers &= !modifier;
        self
    }

    /// Sets the padding
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Spacing>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the margin
    #[must_use]
    pub fn margin(mut self, margin: impl Into<Spacing>) -> Self {
        self.margin = margin.into();
        self
    }

    /// Sets the border style
    #[must_use]
    pub const fn border(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Sets the border color
    #[must_use]
    pub fn border_color(mut self, color: impl Into<Color>) -> Self {
        self.border_color = color.into();
        self
    }

    /// Merges this style with another (other takes precedence)
    #[must_use]
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            fg: if other.fg.is_default() {
                self.fg
            } else {
                other.fg
            },
            bg: if other.bg.is_default() {
                self.bg
            } else {
                other.bg
            },
            modifiers: other.modifiers | self.modifiers,
            padding: if other.padding == Spacing::zero() {
                self.padding
            } else {
                other.padding
            },
            margin: if other.margin == Spacing::zero() {
                self.margin
            } else {
                other.margin
            },
            border_style: if other.border_style == BorderStyle::None {
                self.border_style
            } else {
                other.border_style
            },
            border_color: if other.border_color.is_default() {
                self.border_color
            } else {
                other.border_color
            },
        }
    }

    /// Creates a preset style for primary elements
    #[must_use]
    pub fn primary() -> Self {
        Self::new().fg(Color::blue())
    }

    /// Creates a preset style for secondary elements
    #[must_use]
    pub fn secondary() -> Self {
        Self::new().fg(Color::named(NamedColor::BrightBlack))
    }

    /// Creates a preset style for success states
    #[must_use]
    pub fn success() -> Self {
        Self::new().fg(Color::green())
    }

    /// Creates a preset style for warning states
    #[must_use]
    pub fn warning() -> Self {
        Self::new().fg(Color::yellow())
    }

    /// Creates a preset style for error states
    #[must_use]
    pub fn error() -> Self {
        Self::new().fg(Color::red())
    }

    /// Creates a style with bold modifier
    #[must_use]
    pub fn bold() -> Self {
        Self::new().add_modifier(Modifier::BOLD)
    }

    /// Creates a style with italic modifier
    #[must_use]
    pub fn italic() -> Self {
        Self::new().add_modifier(Modifier::ITALIC)
    }

    /// Creates a style with underline modifier
    #[must_use]
    pub fn underlined() -> Self {
        Self::new().add_modifier(Modifier::UNDERLINE)
    }

    /// Creates a style with dim modifier
    #[must_use]
    pub fn dim() -> Self {
        Self::new().add_modifier(Modifier::DIM)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_default() {
        let modifier = Modifier::default();
        assert_eq!(modifier, Modifier::NONE);
    }

    #[test]
    fn test_modifier_combined() {
        let modifier = Modifier::BOLD | Modifier::ITALIC;
        assert!(modifier.contains(Modifier::BOLD));
        assert!(modifier.contains(Modifier::ITALIC));
        assert!(!modifier.contains(Modifier::UNDERLINE));
    }

    #[test]
    fn test_spacing_all() {
        let spacing = Spacing::all(4);
        assert_eq!(spacing.top, 4);
        assert_eq!(spacing.right, 4);
        assert_eq!(spacing.bottom, 4);
        assert_eq!(spacing.left, 4);
    }

    #[test]
    fn test_spacing_horizontal_vertical() {
        let spacing = Spacing::horizontal_vertical(2, 1);
        assert_eq!(spacing.top, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.bottom, 1);
        assert_eq!(spacing.left, 2);
        assert_eq!(spacing.horizontal(), 4);
        assert_eq!(spacing.vertical(), 2);
    }

    #[test]
    fn test_spacing_from_tuple() {
        let spacing: Spacing = (1, 2, 3, 4).into();
        assert_eq!(spacing.top, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 4);
    }

    #[test]
    fn test_border_style_chars() {
        let single = BorderStyle::Single.chars();
        assert_eq!(single.top_left, '┌');
        assert_eq!(single.horizontal, '─');

        let rounded = BorderStyle::Rounded.chars();
        assert_eq!(rounded.top_left, '╭');
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new()
            .fg(Color::red())
            .bg(Color::blue())
            .add_modifier(Modifier::BOLD)
            .padding(Spacing::all(2))
            .border(BorderStyle::Rounded);

        assert_eq!(style.fg, Color::red());
        assert_eq!(style.bg, Color::blue());
        assert!(style.modifiers.contains(Modifier::BOLD));
        assert_eq!(style.padding, Spacing::all(2));
        assert_eq!(style.border_style, BorderStyle::Rounded);
    }

    #[test]
    fn test_style_merge() {
        let base = Style::new().fg(Color::red()).padding(Spacing::all(2));
        let overlay = Style::new().bg(Color::blue()).margin(Spacing::all(1));

        let merged = base.merge(&overlay);
        assert_eq!(merged.fg, Color::red());
        assert_eq!(merged.bg, Color::blue());
        assert_eq!(merged.padding, Spacing::all(2));
        assert_eq!(merged.margin, Spacing::all(1));
    }

    #[test]
    fn test_style_presets() {
        let bold = Style::bold();
        assert!(bold.modifiers.contains(Modifier::BOLD));
        assert!(!bold.modifiers.contains(Modifier::ITALIC));

        let error = Style::error();
        assert_eq!(error.fg, Color::red());
    }

    #[test]
    fn test_style_serde() {
        let style = Style::new()
            .fg(Color::rgb(255, 128, 64))
            .padding(Spacing::all(2));

        let json = serde_json::to_string(&style).unwrap();
        let deserialized: Style = serde_json::from_str(&json).unwrap();
        assert_eq!(style, deserialized);
    }

    #[test]
    fn test_modifier_serde() {
        let modifier = Modifier::BOLD | Modifier::ITALIC;
        let json = serde_json::to_string(&modifier).unwrap();
        let deserialized: Modifier = serde_json::from_str(&json).unwrap();
        assert_eq!(modifier, deserialized);
    }
}
