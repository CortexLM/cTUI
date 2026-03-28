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

/// RGBA color with f32 components (0.0-1.0 range)
///
/// Provides high-precision color representation for advanced use cases
/// like alpha blending, gradients, and color interpolation.
///
/// This type is only available with the `float-colors` feature.
#[cfg(feature = "float-colors")]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color32 {
    /// Red channel (0.0-1.0)
    pub r: f32,
    /// Green channel (0.0-1.0)
    pub g: f32,
    /// Blue channel (0.0-1.0)
    pub b: f32,
    /// Alpha channel (0.0-1.0, fully opaque at 1.0)
    pub a: f32,
}

#[cfg(feature = "float-colors")]
impl Color32 {
    /// Creates a new Color32 from RGB values
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a new Color32 from RGBA values
    pub fn new_with_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a transparent color
    pub fn transparent() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }
}

/// Converts a Color to Color32 by normalizing u8 values to f32 range
#[cfg(feature = "float-colors")]
impl From<Color> for Color32 {
    fn from(color: Color) -> Self {
        let (r, g, b) = match color {
            Color::Reset => (0.0, 0.0, 0.0),     // Default terminal color
            Color::Black => (0.0, 0.0, 0.0),
            Color::Red => (0.5, 0.0, 0.0),
            Color::Green => (0.0, 0.5, 0.0),
            Color::Yellow => (0.5, 0.5, 0.0),
            Color::Blue => (0.0, 0.0, 0.5),
            Color::Magenta => (0.5, 0.0, 0.5),
            Color::Cyan => (0.0, 0.5, 0.5),
            Color::White => (0.75, 0.75, 0.75),
            Color::DarkGray => (0.5, 0.5, 0.5),
            Color::LightRed => (1.0, 0.0, 0.0),
            Color::LightGreen => (0.0, 1.0, 0.0),
            Color::LightYellow => (1.0, 1.0, 0.0),
            Color::LightBlue => (0.0, 0.0, 1.0),
            Color::LightMagenta => (1.0, 0.0, 1.0),
            Color::LightCyan => (0.0, 1.0, 1.0),
            Color::Gray => (0.75, 0.75, 0.75),
            Color::Indexed(_) => (0.0, 0.0, 0.0), // Indexed colors need palette lookup
            Color::Rgb(r, g, b) => (
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
            ),
        };
        Self { r, g, b, a: 1.0 }
    }
}

/// Converts Color32 to Color by clamping f32 values to u8 range
#[cfg(feature = "float-colors")]
impl From<Color32> for Color {
    fn from(color: Color32) -> Self {
        let clamp = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        Color::Rgb(clamp(color.r), clamp(color.g), clamp(color.b))
    }
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

    #[cfg(feature = "float-colors")]
    #[test]
    fn test_color32_from_rgb() {
        let color = Color::Rgb(255, 128, 0);
        let color32 = Color32::from(color);
        assert!((color32.r - 1.0).abs() < 0.01);
        assert!((color32.g - 0.5).abs() < 0.01);
        assert!((color32.b - 0.0).abs() < 0.01);
        assert!((color32.a - 1.0).abs() < 0.01);
    }

    #[cfg(feature = "float-colors")]
    #[test]
    fn test_color_from_color32() {
        let color32 = Color32::new(1.0, 0.5, 0.0);
        let color = Color::from(color32);
        assert_eq!(color, Color::Rgb(255, 128, 0));
    }

    #[cfg(feature = "float-colors")]
    #[test]
    fn test_color32_roundtrip() {
        let original = Color::Rgb(100, 150, 200);
        let color32 = Color32::from(original);
        let back = Color::from(color32);
        assert_eq!(original, back);
    }
}
