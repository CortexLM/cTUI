//! Backend trait for terminal abstraction
//!
//! This module defines the Backend trait that abstracts over different
//! terminal backends (crossterm, termion, etc.)

mod caps;
mod crossterm;

pub use caps::Capabilities;

use std::io::{self, Result};

use crate::{cell::Cell, geometry::Rect, style::Color};

pub use crossterm::CrosstermBackend;

/// Cursor style for terminal display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorStyle {
    /// Default terminal cursor (usually block)
    #[default]
    Default,
    /// Block cursor (filled rectangle)
    Block,
    /// Underline cursor (underscore at bottom)
    Underline,
    /// Bar cursor (thin vertical line)
    Bar,
}

/// Cursor configuration including style and blink
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorConfig {
    /// Cursor style
    pub style: CursorStyle,
    /// Whether the cursor should blink
    pub blinking: bool,
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            style: CursorStyle::Default,
            blinking: false,
        }
    }
}

impl CursorConfig {
    /// Create a new cursor config
    #[must_use]
    pub const fn new(style: CursorStyle, blinking: bool) -> Self {
        Self { style, blinking }
    }

    /// Create a block cursor
    #[must_use]
    pub const fn block() -> Self {
        Self {
            style: CursorStyle::Block,
            blinking: false,
        }
    }

    /// Create a blinking block cursor
    #[must_use]
    pub const fn blinking_block() -> Self {
        Self {
            style: CursorStyle::Block,
            blinking: true,
        }
    }

    /// Create an underline cursor
    #[must_use]
    pub const fn underline() -> Self {
        Self {
            style: CursorStyle::Underline,
            blinking: false,
        }
    }

    /// Create a blinking underline cursor
    #[must_use]
    pub const fn blinking_underline() -> Self {
        Self {
            style: CursorStyle::Underline,
            blinking: true,
        }
    }

    /// Create a bar cursor
    #[must_use]
    pub const fn bar() -> Self {
        Self {
            style: CursorStyle::Bar,
            blinking: false,
        }
    }

    /// Create a blinking bar cursor
    #[must_use]
    pub const fn blinking_bar() -> Self {
        Self {
            style: CursorStyle::Bar,
            blinking: true,
        }
    }
}

/// Trait for terminal backends
///
/// This trait abstracts the terminal operations needed for TUI rendering.
/// Implementations should optimize for minimal escape sequence emissions
/// through style caching and adjacent cell optimization.
pub trait Backend {
    /// Draws the given cells at their specified positions
    ///
    /// The iterator yields (x, y, cell) tuples representing the position
    /// and content of each cell to be drawn.
    ///
    /// Implementations should:
    /// - Skip cursor movement when drawing sequential adjacent cells
    /// - Only emit style changes when the style actually differs
    /// - Use batched operations (queue!) for efficiency
    ///
    /// # Errors
    ///
    /// Returns an error if drawing to the terminal fails.
    fn draw<I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, Cell)>;

    /// Clears the terminal screen
    ///
    /// # Errors
    ///
    /// Returns an error if clearing the terminal fails.
    fn clear(&mut self) -> Result<()>;

    /// Returns the terminal size as a Rect
    ///
    /// # Errors
    ///
    /// Returns an error if querying the terminal size fails.
    fn size(&self) -> Result<Rect>;

    /// Flushes any pending output
    ///
    /// # Errors
    ///
    /// Returns an error if flushing pending output fails.
    fn flush(&mut self) -> Result<()>;

    /// Returns true if the terminal supports synchronized output
    ///
    /// Synchronized output (a.k.a. "sync update" or "bypass mode") allows
    /// the application to write to the alternate screen buffer without
    /// causing flickering during updates.
    fn supports_synchronized_output(&self) -> bool {
        false
    }

    /// Begins a synchronized output session
    ///
    /// All subsequent writes should be buffered until [`end_synchronized_output`]
    /// is called. This is used to prevent flickering during updates.
    ///
    /// # Errors
    ///
    /// Returns an error if beginning synchronized output fails.
    fn begin_synchronized_output(&mut self) -> Result<()> {
        Ok(())
    }

    /// Ends a synchronized output session
    ///
    /// Flushes any buffered output and returns to normal mode.
    ///
    /// # Errors
    ///
    /// Returns an error if ending synchronized output fails.
    fn end_synchronized_output(&mut self) -> Result<()> {
        Ok(())
    }

    /// Returns the current cursor position (x, y)
    ///
    /// # Errors
    ///
    /// Returns an error if querying the cursor position fails.
    fn cursor_pos(&self) -> Result<(u16, u16)>;

    /// Sets the cursor position to (x, y)
    ///
    /// # Errors
    ///
    /// Returns an error if setting the cursor position fails.
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()>;

    /// Shows the cursor
    ///
    /// # Errors
    ///
    /// Returns an error if showing the cursor fails.
    fn show_cursor(&mut self) -> Result<()>;

    /// Hides the cursor
    ///
    /// # Errors
    ///
    /// Returns an error if hiding the cursor fails.
    fn hide_cursor(&mut self) -> Result<()>;

    /// Scrolls the terminal content up by n lines
    ///
    /// New lines are inserted at the bottom. This is useful for
    /// implementing scrolling UIs like logs or chats.
    ///
    /// # Errors
    ///
    /// Returns an error if scrolling fails.
    fn scroll_up(&mut self, n: u16) -> Result<()> {
        // Default: no-op (backends can override for better performance)
        let _ = n;
        Ok(())
    }

    /// Scrolls the terminal content down by n lines
    ///
    /// New lines are inserted at the top. This is useful for
    /// implementing scrolling UIs.
    ///
    /// # Errors
    ///
    /// Returns an error if scrolling fails.
    fn scroll_down(&mut self, n: u16) -> Result<()> {
        // Default: no-op (backends can override for better performance)
        let _ = n;
        Ok(())
    }

    /// Sets the terminal window title
    ///
    /// This sets the title of the terminal window or tab. Useful for
    /// displaying application state or context.
    ///
    /// # Errors
    ///
    /// Returns an error if setting the title fails.
    fn set_title(&mut self, title: &str) -> Result<()> {
        let _ = title;
        Ok(())
    }

    /// Enters alternate screen mode
    ///
    /// Alternate screen mode creates a separate screen buffer that doesn't
    /// affect the scrollback. When the application exits, the original screen
    /// is restored. This is the default mode for most TUI applications.
    ///
    /// # Errors
    ///
    /// Returns an error if entering alternate screen fails.
    fn enter_alternate_screen(&mut self) -> Result<()> {
        Ok(())
    }

    /// Leaves alternate screen mode
    ///
    /// Restores the main screen buffer. Should be called before exiting
    /// to restore the terminal to its previous state.
    ///
    /// # Errors
    ///
    /// Returns an error if leaving alternate screen fails.
    fn leave_alternate_screen(&mut self) -> Result<()> {
        Ok(())
    }

    /// Returns true if currently in alternate screen mode
    fn is_alternate_screen(&self) -> bool {
        false
    }

    /// Clears a specific region of the screen
    ///
    /// The region is defined by a Rect (x, y, width, height).
    /// Only cells within this region are cleared.
    ///
    /// # Errors
    ///
    /// Returns an error if clearing the region fails.
    fn clear_region(&mut self, area: Rect) -> Result<()> {
        for y in area.y..area.y.saturating_add(area.height) {
            self.set_cursor(area.x, y)?;
        }
        Ok(())
    }

    /// Sets the cursor style (block, underline, bar) with optional blinking
    ///
    /// # Errors
    ///
    /// Returns an error if setting the cursor style fails.
    fn set_cursor_style(&mut self, config: CursorConfig) -> Result<()> {
        let _ = config;
        Ok(())
    }

    /// Sets the default background color for the terminal
    ///
    /// # Errors
    ///
    /// Returns an error if setting the background color fails.
    fn set_background_color(&mut self, color: Color) -> Result<()> {
        let _ = color;
        Ok(())
    }

    /// Enables mouse event tracking
    ///
    /// # Errors
    ///
    /// Returns an error if enabling mouse capture fails.
    fn enable_mouse_capture(&mut self) -> Result<()> {
        Ok(())
    }

    /// Disables mouse event tracking
    ///
    /// # Errors
    ///
    /// Returns an error if disabling mouse capture fails.
    fn disable_mouse_capture(&mut self) -> Result<()> {
        Ok(())
    }

    /// Returns true if the terminal supports Kitty keyboard protocol
    ///
    /// Kitty keyboard protocol provides enhanced keyboard input including:
    /// - Key event kinds (Press, Repeat, Release)
    /// - Extended modifiers (Hyper, Meta, CapsLock, NumLock)
    /// - Distinguishing key-up vs key-down events
    fn supports_kitty_keyboard(&self) -> bool {
        false
    }

    /// Enables Kitty keyboard protocol
    ///
    /// Sends the CSI sequence to enable enhanced keyboard reporting.
    /// After enabling, key events will include:
    /// - Key event kinds (Press, Repeat, Release)
    /// - Extended modifiers
    ///
    /// # Errors
    ///
    /// Returns an error if enabling fails.
    fn enable_kitty_keyboard(&mut self) -> Result<()> {
        Ok(())
    }

    /// Disables Kitty keyboard protocol
    ///
    /// Sends the CSI sequence to disable enhanced keyboard reporting.
    ///
    /// # Errors
    ///
    /// Returns an error if disabling fails.
    fn disable_kitty_keyboard(&mut self) -> Result<()> {
        Ok(())
    }
}

pub mod test;

#[cfg(test)]
mod tests {
    use super::*;

    // Test stub to verify module compiles
    #[test]
    fn test_backend_trait_exists() {
        // This test verifies the trait is properly defined
        fn assert_backend<B: Backend>() {}
        assert_backend::<CrosstermBackend<std::io::Stdout>>();
    }
}
