//! Crossterm backend implementation
//!
//! This module provides a CrosstermBackend that implements the Backend trait
//! using the crossterm library for terminal operations.

use std::io::{self, Result, Write};

use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::{
        Attribute, Attributes, Color as CrosstermColor, Print, SetAttributes, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};

use crate::cell::Cell;
use crate::geometry::Rect;
use crate::style::{Color, Modifier};

use super::{Backend, CursorConfig, CursorStyle};

/// Position for tracking adjacent cell optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Position {
    x: u16,
    y: u16,
}

impl Position {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn is_adjacent_to(&self, other: Option<Position>) -> bool {
        match other {
            Some(pos) => pos.y == self.y && pos.x + 1 == self.x,
            None => false,
        }
    }
}

/// Crossterm-based terminal backend with style caching and adjacent cell optimization.
pub struct CrosstermBackend<W: Write> {
    writer: W,
    fg: Color,
    bg: Color,
    modifier: Modifier,
    last_pos: Option<Position>,
    supports_sync: bool,
    in_alternate_screen: bool,
    supports_kitty: bool,
    kitty_keyboard_enabled: bool,
}

impl<W: Write> CrosstermBackend<W> {
    /// Creates a new CrosstermBackend with the given writer
    pub fn new(writer: W) -> Self {
        let supports_kitty = detect_kitty_terminal();
        Self {
            writer,
            fg: Color::default(),
            bg: Color::default(),
            modifier: Modifier::default(),
            last_pos: None,
            supports_sync: false,
            in_alternate_screen: false,
            supports_kitty,
            kitty_keyboard_enabled: false,
        }
    }

    /// Creates a new CrosstermBackend with synchronized output support detection
    pub fn with_sync_detection(writer: W) -> Self {
        let supports_sync = detect_sync_support();
        let supports_kitty = detect_kitty_terminal();
        Self {
            writer,
            fg: Color::default(),
            bg: Color::default(),
            modifier: Modifier::default(),
            last_pos: None,
            supports_sync,
            in_alternate_screen: false,
            supports_kitty,
            kitty_keyboard_enabled: false,
        }
    }

    /// Creates a new CrosstermBackend with all capability detection
    pub fn with_capabilities(writer: W) -> Self {
        let (supports_sync, supports_kitty) = detect_capabilities();
        Self {
            writer,
            fg: Color::default(),
            bg: Color::default(),
            modifier: Modifier::default(),
            last_pos: None,
            supports_sync,
            in_alternate_screen: false,
            supports_kitty,
            kitty_keyboard_enabled: false,
        }
    }

    fn convert_color(color: Color) -> CrosstermColor {
        match color {
            Color::Reset => CrosstermColor::Reset,
            Color::Black => CrosstermColor::Black,
            Color::Red => CrosstermColor::DarkRed,
            Color::Green => CrosstermColor::DarkGreen,
            Color::Yellow => CrosstermColor::DarkYellow,
            Color::Blue => CrosstermColor::DarkBlue,
            Color::Magenta => CrosstermColor::DarkMagenta,
            Color::Cyan => CrosstermColor::DarkCyan,
            Color::White => CrosstermColor::Grey,
            Color::DarkGray => CrosstermColor::DarkGrey,
            Color::LightRed => CrosstermColor::Red,
            Color::LightGreen => CrosstermColor::Green,
            Color::LightYellow => CrosstermColor::Yellow,
            Color::LightBlue => CrosstermColor::Blue,
            Color::LightMagenta => CrosstermColor::Magenta,
            Color::LightCyan => CrosstermColor::Cyan,
            Color::Gray => CrosstermColor::White,
            Color::Indexed(idx) => CrosstermColor::AnsiValue(idx),
            Color::Rgb(r, g, b) => CrosstermColor::Rgb { r, g, b },
        }
    }

    fn update_style(&mut self, cell: &Cell) -> Result<()> {
        if self.fg != cell.fg {
            self.fg = cell.fg;
            queue!(
                self.writer,
                SetForegroundColor(Self::convert_color(cell.fg))
            )?;
        }

        if self.bg != cell.bg {
            self.bg = cell.bg;
            queue!(
                self.writer,
                SetBackgroundColor(Self::convert_color(cell.bg))
            )?;
        }

        if self.modifier != cell.modifier {
            if !self.modifier.is_empty() {
                queue!(self.writer, SetAttributes(Attributes::default()))?;
            }
            self.modifier = cell.modifier;
            self.apply_modifiers(cell.modifier)?;
        }

        Ok(())
    }

    fn apply_modifiers(&mut self, modifier: Modifier) -> Result<()> {
        let mut attrs = Attributes::default();
        if modifier.contains(Modifier::BOLD) {
            attrs = attrs | Attribute::Bold;
        }
        if modifier.contains(Modifier::DIM) {
            attrs = attrs | Attribute::Dim;
        }
        if modifier.contains(Modifier::ITALIC) {
            attrs = attrs | Attribute::Italic;
        }
        if modifier.contains(Modifier::UNDERLINED) {
            attrs = attrs | Attribute::Underlined;
        }
        if modifier.contains(Modifier::SLOW_BLINK) {
            attrs = attrs | Attribute::SlowBlink;
        }
        if modifier.contains(Modifier::RAPID_BLINK) {
            attrs = attrs | Attribute::RapidBlink;
        }
        if modifier.contains(Modifier::REVERSED) {
            attrs = attrs | Attribute::Reverse;
        }
        if modifier.contains(Modifier::HIDDEN) {
            attrs = attrs | Attribute::Hidden;
        }
        if modifier.contains(Modifier::CROSSED_OUT) {
            attrs = attrs | Attribute::CrossedOut;
        }
        if !attrs.is_empty() {
            queue!(self.writer, SetAttributes(attrs))?;
        }
        Ok(())
    }

    fn move_cursor_if_needed(&mut self, x: u16, y: u16) -> Result<()> {
        let pos = Position::new(x, y);
        if !pos.is_adjacent_to(self.last_pos) {
            queue!(self.writer, MoveTo(x, y))?;
        }
        self.last_pos = Some(pos);
        Ok(())
    }
}

impl<W: Write> Backend for CrosstermBackend<W> {
    fn draw<I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, Cell)>,
    {
        for (x, y, cell) in content {
            if cell.skip {
                continue;
            }

            self.move_cursor_if_needed(x, y)?;
            self.update_style(&cell)?;
            queue!(self.writer, Print(&cell.symbol))?;
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::empty();
        self.last_pos = None;
        execute!(self.writer, Clear(ClearType::All))
    }

    fn size(&self) -> Result<Rect> {
        let (cols, rows) = crossterm::terminal::size()?;
        Ok(Rect::new(0, 0, cols, rows))
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

    fn supports_synchronized_output(&self) -> bool {
        self.supports_sync
    }

    fn cursor_pos(&self) -> Result<(u16, u16)> {
        let (x, y) = crossterm::cursor::position()?;
        Ok((x, y))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        self.last_pos = Some(Position::new(x, y));
        queue!(self.writer, MoveTo(x, y))?;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        execute!(self.writer, Show)?;
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        execute!(self.writer, Hide)?;
        Ok(())
    }

    fn scroll_up(&mut self, n: u16) -> Result<()> {
        // Use CSI n S sequence to scroll up n lines
        write!(self.writer, "\x1b[{}S", n)?;
        self.flush()
    }

    fn scroll_down(&mut self, n: u16) -> Result<()> {
        // Use CSI n T sequence to scroll down n lines
        write!(self.writer, "\x1b[{}T", n)?;
        self.flush()
    }

    fn set_title(&mut self, title: &str) -> Result<()> {
        execute!(self.writer, SetTitle(title))?;
        Ok(())
    }

    fn enter_alternate_screen(&mut self) -> Result<()> {
        execute!(self.writer, EnterAlternateScreen)?;
        self.in_alternate_screen = true;
        Ok(())
    }

    fn leave_alternate_screen(&mut self) -> Result<()> {
        execute!(self.writer, LeaveAlternateScreen)?;
        self.in_alternate_screen = false;
        Ok(())
    }

    fn is_alternate_screen(&self) -> bool {
        self.in_alternate_screen
    }

    fn clear_region(&mut self, area: Rect) -> Result<()> {
        for y in area.y..area.y.saturating_add(area.height) {
            queue!(self.writer, MoveTo(area.x, y))?;
            for x in area.x..area.x.saturating_add(area.width) {
                queue!(self.writer, MoveTo(x, y), Print(' '))?;
            }
        }
        self.flush()
    }

    fn set_cursor_style(&mut self, config: CursorConfig) -> Result<()> {
        let style = match config.style {
            CursorStyle::Default => SetCursorStyle::DefaultUserShape,
            CursorStyle::Block => {
                if config.blinking {
                    SetCursorStyle::BlinkingBlock
                } else {
                    SetCursorStyle::SteadyBlock
                }
            }
            CursorStyle::Underline => {
                if config.blinking {
                    SetCursorStyle::BlinkingUnderScore
                } else {
                    SetCursorStyle::SteadyUnderScore
                }
            }
            CursorStyle::Bar => {
                if config.blinking {
                    SetCursorStyle::BlinkingBar
                } else {
                    SetCursorStyle::SteadyBar
                }
            }
        };
        execute!(self.writer, style)?;
        Ok(())
    }

    fn set_background_color(&mut self, color: Color) -> Result<()> {
        self.bg = color;
        execute!(self.writer, SetBackgroundColor(Self::convert_color(color)))
    }

    fn enable_mouse_capture(&mut self) -> Result<()> {
        execute!(self.writer, EnableMouseCapture)?;
        Ok(())
    }

    fn disable_mouse_capture(&mut self) -> Result<()> {
        execute!(self.writer, DisableMouseCapture)?;
        Ok(())
    }

    fn supports_kitty_keyboard(&self) -> bool {
        self.supports_kitty
    }

    fn enable_kitty_keyboard(&mut self) -> Result<()> {
        if self.supports_kitty {
            // Kitty keyboard protocol: CSI > 1 u enables full protocol
            write!(self.writer, "\x1b[>1u")?;
            self.flush()?;
            self.kitty_keyboard_enabled = true;
        }
        // Non-Kitty terminals: silently succeed (use standard crossterm events)
        Ok(())
    }

    fn disable_kitty_keyboard(&mut self) -> Result<()> {
        if self.kitty_keyboard_enabled {
            // Kitty keyboard protocol: CSI < 1 u disables the protocol
            write!(self.writer, "\x1b[<1u")?;
            self.flush()?;
            self.kitty_keyboard_enabled = false;
        }
        Ok(())
    }
}

impl<W: Write> Drop for CrosstermBackend<W> {
    fn drop(&mut self) {
        // Ensure Kitty keyboard is disabled on drop
        if self.kitty_keyboard_enabled {
            let _ = write!(self.writer, "\x1b[<1u");
            let _ = self.flush();
        }
    }
}

impl<W: Write> CrosstermBackend<W> {
    /// Returns a mutable reference to the underlying writer
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Returns true if Kitty keyboard is currently enabled
    #[must_use]
    pub const fn is_kitty_keyboard_enabled(&self) -> bool {
        self.kitty_keyboard_enabled
    }
}

/// Detect if the terminal is Kitty
fn detect_kitty_terminal() -> bool {
    std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("KITTY_PID").is_ok()
        || std::env::var("TERM")
            .map(|t| t == "xterm-kitty")
            .unwrap_or(false)
}

/// Detect sync support
fn detect_sync_support() -> bool {
    std::env::var("TERM")
        .map(|term| {
            matches!(
                term.as_str(),
                "xterm-256color"
                    | "screen-256color"
                    | "tmux-256color"
                    | "alacritty"
                    | "kitty"
                    | "wezterm"
                    | "foot"
            )
        })
        .unwrap_or(false)
}

/// Detect all capabilities
fn detect_capabilities() -> (bool, bool) {
    let supports_kitty = detect_kitty_terminal();
    let supports_sync = if supports_kitty {
        true
    } else {
        detect_sync_support()
    };
    (supports_sync, supports_kitty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_adjacent() {
        let pos = Position::new(5, 3);
        assert!(pos.is_adjacent_to(Some(Position::new(4, 3))));
        assert!(!pos.is_adjacent_to(Some(Position::new(3, 3))));
        assert!(!pos.is_adjacent_to(Some(Position::new(5, 3))));
        assert!(!pos.is_adjacent_to(Some(Position::new(4, 2))));
    }

    #[test]
    fn test_color_conversion() {
        assert_eq!(
            CrosstermBackend::<std::io::Stdout>::convert_color(Color::Red),
            CrosstermColor::DarkRed
        );
        assert_eq!(
            CrosstermBackend::<std::io::Stdout>::convert_color(Color::LightRed),
            CrosstermColor::Red
        );
        assert_eq!(
            CrosstermBackend::<std::io::Stdout>::convert_color(Color::Rgb(255, 128, 64)),
            CrosstermColor::Rgb {
                r: 255,
                g: 128,
                b: 64
            }
        );
    }

    #[test]
    fn test_backend_creation() {
        let backend = CrosstermBackend::new(Vec::new());
        assert!(!backend.supports_synchronized_output());
        // Kitty detection depends on environment
        let _ = backend.supports_kitty_keyboard();
    }

    #[test]
    fn test_backend_with_capabilities() {
        let backend = CrosstermBackend::with_capabilities(Vec::new());
        // Capability detection depends on environment
        let _ = backend.supports_synchronized_output();
        let _ = backend.supports_kitty_keyboard();
    }

    #[test]
    fn test_kitty_keyboard_enable_disable() {
        let mut backend = CrosstermBackend::new(Vec::new());
        
        // Should succeed even on non-Kitty terminals
        assert!(backend.enable_kitty_keyboard().is_ok());
        
        // Should track state
        if backend.supports_kitty_keyboard() {
            assert!(backend.is_kitty_keyboard_enabled());
        }
        
        // Disable should work
        assert!(backend.disable_kitty_keyboard().is_ok());
        assert!(!backend.is_kitty_keyboard_enabled());
    }

    #[test]
    fn test_detect_kitty_terminal() {
        // Just ensure it doesn't panic
        let _ = detect_kitty_terminal();
    }

    #[test]
    fn test_detect_capabilities() {
        let (sync, kitty) = detect_capabilities();
        // If Kitty terminal, both should be true
        if kitty {
            assert!(sync);
        }
    }
}
