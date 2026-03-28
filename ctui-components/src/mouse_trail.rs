//! Mouse trail effect component

use crate::Widget;
use ctui_core::geometry::Position;
use ctui_core::style::Color;
use ctui_core::{Buffer, Rect};
use std::collections::VecDeque;

/// A visual mouse trail effect component.
#[derive(Clone, Debug)]
pub struct MouseTrail {
    positions: VecDeque<Position>,
    max_length: usize,
    fade_chars: Vec<char>,
    color_start: Color,
    color_end: Color,
    enabled: bool,
}

impl Default for MouseTrail {
    fn default() -> Self {
        Self::new(20)
    }
}

impl MouseTrail {
    /// Default fade characters from solid to transparent.
    pub const DEFAULT_FADE_CHARS: [char; 5] = ['█', '▓', '▒', '░', ' '];

    /// Creates a new MouseTrail with the specified maximum length.
    pub fn new(max_length: usize) -> Self {
        Self {
            positions: VecDeque::with_capacity(max_length),
            max_length: max_length.clamp(5, 50),
            fade_chars: Self::DEFAULT_FADE_CHARS.to_vec(),
            color_start: Color::Rgb(100, 200, 255),
            color_end: Color::Rgb(30, 60, 100),
            enabled: true,
        }
    }

    /// Sets custom fade characters ordered from solid to transparent.
    pub fn with_fade_chars(mut self, chars: impl Into<Vec<char>>) -> Self {
        self.fade_chars = chars.into();
        if self.fade_chars.is_empty() {
            self.fade_chars = Self::DEFAULT_FADE_CHARS.to_vec();
        }
        self
    }

    /// Sets the color gradient for the trail.
    pub fn with_colors(mut self, start: Color, end: Color) -> Self {
        self.color_start = start;
        self.color_end = end;
        self
    }

    /// Sets whether the trail is enabled.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Enables the trail.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the trail and clears all positions.
    pub fn disable(&mut self) {
        self.enabled = false;
        self.positions.clear();
    }

    /// Returns whether the trail is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Updates the trail with a new mouse position.
    pub fn update(&mut self, new_pos: Position) {
        if !self.enabled {
            return;
        }

        if let Some(&last) = self.positions.front() {
            if last.x == new_pos.x && last.y == new_pos.y {
                return;
            }
        }

        self.positions.push_front(new_pos);

        if self.positions.len() > self.max_length {
            self.positions.pop_back();
        }
    }

    /// Clears all trail positions.
    pub fn clear(&mut self) {
        self.positions.clear();
    }

    /// Returns the current number of positions in the trail.
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// Returns true if the trail has no positions.
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    /// Returns the maximum configured length.
    pub fn max_length(&self) -> usize {
        self.max_length
    }

    /// Sets a new maximum length.
    pub fn set_max_length(&mut self, length: usize) {
        self.max_length = length.clamp(5, 50);
        while self.positions.len() > self.max_length {
            self.positions.pop_back();
        }
    }

    /// Returns the most recent position, if any.
    pub fn head(&self) -> Option<Position> {
        self.positions.front().copied()
    }

    /// Returns the oldest position, if any.
    pub fn tail(&self) -> Option<Position> {
        self.positions.back().copied()
    }

    fn interpolate_color(&self, index: usize, total: usize) -> Color {
        if total <= 1 {
            return self.color_start;
        }

        let t = index as f32 / (total - 1).max(1) as f32;

        let (r1, g1, b1) = extract_rgb(&self.color_start);
        let (r2, g2, b2) = extract_rgb(&self.color_end);

        let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
        let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
        let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;

        Color::Rgb(r, g, b)
    }

    fn get_fade_char(&self, index: usize, total: usize) -> char {
        if total <= 1 || self.fade_chars.is_empty() {
            return self.fade_chars.first().copied().unwrap_or('█');
        }

        let t = index as f32 / (total - 1).max(1) as f32;
        let char_index = (t * (self.fade_chars.len() - 1) as f32).round() as usize;
        self.fade_chars.get(char_index).copied().unwrap_or(' ')
    }
}

/// Extracts RGB values from a Color.
fn extract_rgb(color: &Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(r, g, b) => (*r, *g, *b),
        Color::Indexed(idx) => indexed_to_rgb(*idx),
        Color::Black => (0, 0, 0),
        Color::Red => (128, 0, 0),
        Color::Green => (0, 128, 0),
        Color::Yellow => (128, 128, 0),
        Color::Blue => (0, 0, 128),
        Color::Magenta => (128, 0, 128),
        Color::Cyan => (0, 128, 128),
        Color::White => (192, 192, 192),
        Color::DarkGray => (128, 128, 128),
        Color::LightRed => (255, 0, 0),
        Color::LightGreen => (0, 255, 0),
        Color::LightYellow => (255, 255, 0),
        Color::LightBlue => (0, 0, 255),
        Color::LightMagenta => (255, 0, 255),
        Color::LightCyan => (0, 255, 255),
        Color::Gray => (255, 255, 255),
        Color::Reset => (128, 128, 128),
    }
}

/// Converts 256-color index to RGB.
fn indexed_to_rgb(idx: u8) -> (u8, u8, u8) {
    if idx < 16 {
        match idx {
            0 => (0, 0, 0),
            1 => (128, 0, 0),
            2 => (0, 128, 0),
            3 => (128, 128, 0),
            4 => (0, 0, 128),
            5 => (128, 0, 128),
            6 => (0, 128, 128),
            7 => (192, 192, 192),
            8 => (128, 128, 128),
            9 => (255, 0, 0),
            10 => (0, 255, 0),
            11 => (255, 255, 0),
            12 => (0, 0, 255),
            13 => (255, 0, 255),
            14 => (0, 255, 255),
            15 => (255, 255, 255),
            _ => (0, 0, 0),
        }
    } else {
        let idx = idx - 16;
        if idx < 216 {
            let r = (idx / 36) * 51;
            let g = ((idx % 36) / 6) * 51;
            let b = (idx % 6) * 51;
            (r, g, b)
        } else {
            let gray = 8 + (idx - 216) * 10;
            (gray, gray, gray)
        }
    }
}

impl Widget for MouseTrail {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.enabled || self.positions.is_empty() || area.is_zero() {
            return;
        }

        let total = self.positions.len();

        for (index, pos) in self.positions.iter().enumerate() {
            let buf_x = area.x + pos.x;
            let buf_y = area.y + pos.y;

            if buf_x >= area.x + area.width || buf_y >= area.y + area.height {
                continue;
            }

            buf.modify_cell(buf_x, buf_y, |cell| {
                let fade_char = self.get_fade_char(index, total);
                let color = self.interpolate_color(index, total);
                if fade_char != ' ' {
                cell.symbol = fade_char.to_string();
                }
                cell.fg = color;
            });
        }
    }
}

/// Builder for creating customized MouseTrail instances.
#[derive(Debug, Clone)]
pub struct MouseTrailBuilder {
    max_length: usize,
    fade_chars: Vec<char>,
    color_start: Color,
    color_end: Color,
    enabled: bool,
}

impl Default for MouseTrailBuilder {
    fn default() -> Self {
        Self {
            max_length: 20,
            fade_chars: MouseTrail::DEFAULT_FADE_CHARS.to_vec(),
            color_start: Color::Rgb(100, 200, 255),
            color_end: Color::Rgb(30, 60, 100),
            enabled: true,
        }
    }
}

impl MouseTrailBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum trail length.
    pub fn max_length(mut self, length: usize) -> Self {
        self.max_length = length.clamp(5, 50);
        self
    }

    /// Sets custom fade characters.
    pub fn fade_chars(mut self, chars: impl Into<Vec<char>>) -> Self {
        let chars = chars.into();
        self.fade_chars = if chars.is_empty() {
            MouseTrail::DEFAULT_FADE_CHARS.to_vec()
        } else {
            chars
        };
        self
    }

    /// Sets the color gradient.
    pub fn colors(mut self, start: Color, end: Color) -> Self {
        self.color_start = start;
        self.color_end = end;
        self
    }

    /// Sets whether the trail starts enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Builds the MouseTrail.
    pub fn build(self) -> MouseTrail {
        MouseTrail {
            positions: VecDeque::with_capacity(self.max_length),
            max_length: self.max_length,
            fade_chars: self.fade_chars,
            color_start: self.color_start,
            color_end: self.color_end,
            enabled: self.enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WidgetExt;

    #[test]
    fn test_mouse_trail_new() {
        let trail = MouseTrail::new(25);
        assert_eq!(trail.max_length(), 25);
        assert!(trail.is_enabled());
        assert!(trail.is_empty());
    }

    #[test]
    fn test_mouse_trail_default() {
        let trail = MouseTrail::default();
        assert_eq!(trail.max_length(), 20);
        assert!(trail.is_enabled());
    }

    #[test]
    fn test_mouse_trail_clamp_length() {
        let trail1 = MouseTrail::new(3);
        assert_eq!(trail1.max_length(), 5);

        let trail2 = MouseTrail::new(100);
        assert_eq!(trail2.max_length(), 50);
    }

    #[test]
    fn test_mouse_trail_update() {
        let mut trail = MouseTrail::new(20);

        trail.update(Position::new(10, 10));
        assert_eq!(trail.len(), 1);
        assert_eq!(trail.head(), Some(Position::new(10, 10)));

        trail.update(Position::new(15, 15));
        assert_eq!(trail.len(), 2);
        assert_eq!(trail.head(), Some(Position::new(15, 15)));
    }

    #[test]
    fn test_mouse_trail_update_duplicate() {
        let mut trail = MouseTrail::new(20);

        trail.update(Position::new(10, 10));
        trail.update(Position::new(10, 10));
        assert_eq!(trail.len(), 1);
    }

    #[test]
    fn test_mouse_trail_max_length() {
        let mut trail = MouseTrail::new(5);

        for i in 0..10 {
            trail.update(Position::new(i, i));
        }

        assert_eq!(trail.len(), 5);
        assert_eq!(trail.head(), Some(Position::new(9, 9)));
        assert_eq!(trail.tail(), Some(Position::new(5, 5)));
    }

    #[test]
    fn test_mouse_trail_enable_disable() {
        let mut trail = MouseTrail::new(20);

        trail.update(Position::new(10, 10));
        assert_eq!(trail.len(), 1);

        trail.disable();
        assert!(!trail.is_enabled());
        assert!(trail.is_empty());

        trail.update(Position::new(20, 20));
        assert!(trail.is_empty());

        trail.enable();
        assert!(trail.is_enabled());
        trail.update(Position::new(30, 30));
        assert_eq!(trail.len(), 1);
    }

    #[test]
    fn test_mouse_trail_clear() {
        let mut trail = MouseTrail::new(20);

        for i in 0..10 {
            trail.update(Position::new(i, i));
        }
        assert_eq!(trail.len(), 10);

        trail.clear();
        assert!(trail.is_empty());
    }

    #[test]
    fn test_mouse_trail_with_colors() {
        let trail = MouseTrail::new(20).with_colors(Color::Rgb(255, 0, 0), Color::Rgb(0, 0, 255));

        assert!(trail.is_enabled());
    }

    #[test]
    fn test_mouse_trail_with_fade_chars() {
        let trail = MouseTrail::new(20).with_fade_chars(['A', 'B', 'C', 'D', 'E']);

        assert_eq!(trail.fade_chars, vec!['A', 'B', 'C', 'D', 'E']);
    }

    #[test]
    fn test_extract_rgb() {
        assert_eq!(extract_rgb(&Color::Rgb(100, 150, 200)), (100, 150, 200));
        assert_eq!(extract_rgb(&Color::Black), (0, 0, 0));
        assert_eq!(extract_rgb(&Color::White), (192, 192, 192));
        assert_eq!(extract_rgb(&Color::Red), (128, 0, 0));
    }

    #[test]
    fn test_interpolate_color() {
        let trail =
            MouseTrail::new(20).with_colors(Color::Rgb(100, 100, 100), Color::Rgb(200, 200, 200));

        let color_start = trail.interpolate_color(0, 10);
        if let Color::Rgb(r, _, _) = color_start {
            assert!(r < 110);
        }

        let color_end = trail.interpolate_color(9, 10);
        if let Color::Rgb(r, _, _) = color_end {
            assert!(r > 190);
        }
    }

    #[test]
    fn test_mouse_trail_get_fade_char() {
        let trail = MouseTrail::new(20);

        let first_char = trail.get_fade_char(0, 10);
        assert_eq!(first_char, '█');

        let last_char = trail.get_fade_char(9, 10);
        assert_eq!(last_char, ' ');
    }

    #[test]
    fn test_mouse_trail_render() {
        let mut trail = MouseTrail::new(20);
        trail.update(Position::new(5, 5));
        trail.update(Position::new(6, 6));

        let buf = trail.to_buffer(20, 20);
        assert!(!buf.content.is_empty());
    }

    #[test]
    fn test_mouse_trail_render_disabled() {
        let mut trail = MouseTrail::new(20);
        trail.disable();
        trail.update(Position::new(5, 5));

        let buf = trail.to_buffer(20, 20);
        for cell in buf.iter() {
            assert_eq!(cell.symbol, " ");
        }
    }

    #[test]
    fn test_builder() {
        let trail = MouseTrailBuilder::new()
            .max_length(30)
            .colors(Color::Rgb(255, 0, 0), Color::Rgb(0, 255, 0))
            .fade_chars(['X', 'Y', 'Z'])
            .enabled(false)
            .build();

        assert_eq!(trail.max_length(), 30);
        assert!(!trail.is_enabled());
    }

    #[test]
    fn test_set_max_length() {
        let mut trail = MouseTrail::new(20);

        for i in 0..20 {
            trail.update(Position::new(i, i));
        }
        assert_eq!(trail.len(), 20);

        trail.set_max_length(10);
        assert_eq!(trail.max_length(), 10);
        assert_eq!(trail.len(), 10);
    }

    #[test]
    fn test_head_tail() {
        let mut trail = MouseTrail::new(20);

        assert!(trail.head().is_none());
        assert!(trail.tail().is_none());

        trail.update(Position::new(0, 0));
        trail.update(Position::new(10, 10));

        assert_eq!(trail.head(), Some(Position::new(10, 10)));
        assert_eq!(trail.tail(), Some(Position::new(0, 0)));
    }
}
