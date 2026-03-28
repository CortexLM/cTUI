//! Canvas backend for rendering to HTML5 Canvas
//!
//! This module provides a WebAssembly-compatible backend that renders
//! TUI content to an HTML5 Canvas element using `CanvasRenderingContext2d`.

use std::io::{self, Result};

use ctui_core::backend::Backend;
use ctui_core::cell::Cell;
use ctui_core::geometry::Rect;
use ctui_core::style::Color;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Font settings for canvas rendering
const FONT_FAMILY: &str = "monospace";
const FONT_SIZE_PX: u16 = 16;

/// Canvas-based backend for WebAssembly rendering.
///
/// This backend renders TUI content to an HTML5 Canvas element,
/// using web-sys's `CanvasRenderingContext2d` for drawing operations.
/// Each cell is rendered as a single character using `fillText()`.
pub struct CanvasBackend {
    /// The canvas 2D rendering context
    ctx: CanvasRenderingContext2d,
    /// The canvas element for size queries
    canvas: HtmlCanvasElement,
    /// Character width in pixels (calculated from font metrics)
    char_width: f64,
    /// Character height in pixels (line height)
    char_height: f64,
    /// Current cursor X position (for optimization)
    cursor_x: u16,
    /// Current cursor Y position (for optimization)
    cursor_y: u16,
}

impl CanvasBackend {
    /// Creates a new `CanvasBackend` from a canvas element.
    ///
    /// # Errors
    ///
    /// Returns an error if the canvas context cannot be obtained.
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self> {
        let ctx = canvas
            .get_context("2d")
            .map_err(|_| io::Error::other("Failed to get 2d context"))?
            .ok_or_else(|| io::Error::other("Context is null"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| io::Error::other("Failed to cast context"))?;

        // Calculate character dimensions
        ctx.set_font(&format!("{FONT_SIZE_PX}px {FONT_FAMILY}"));
        let metrics = ctx
            .measure_text("M")
            .map_err(|_| io::Error::other("Failed to measure text"))?;
        let char_width = metrics.width();
        let char_height = f64::from(FONT_SIZE_PX) * 1.2; // Line height with some padding

        Ok(Self {
            ctx,
            canvas,
            char_width,
            char_height,
            cursor_x: 0,
            cursor_y: 0,
        })
    }

    /// Creates a backend from an existing context with custom dimensions.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The canvas rendering context
    /// * `canvas` - The canvas element for size queries
    /// * `char_width` - Width of a single character in pixels
    /// * `char_height` - Height of a single character in pixels
    #[must_use]
    pub fn with_dimensions(
        ctx: CanvasRenderingContext2d,
        canvas: HtmlCanvasElement,
        char_width: f64,
        char_height: f64,
    ) -> Self {
        ctx.set_font(&format!("{FONT_SIZE_PX}px {FONT_FAMILY}"));
        Self {
            ctx,
            canvas,
            char_width,
            char_height,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// Returns the character width in pixels.
    #[must_use]
    pub const fn char_width(&self) -> f64 {
        self.char_width
    }

    /// Returns the character height in pixels.
    #[must_use]
    pub const fn char_height(&self) -> f64 {
        self.char_height
    }

    /// Converts a Color to a CSS color string.
    fn color_to_css(color: Color) -> String {
        match color {
            Color::Reset => "transparent".to_string(),
            Color::Black => "#000000".to_string(),
            Color::Red => "#800000".to_string(),
            Color::Green => "#008000".to_string(),
            Color::Yellow => "#808000".to_string(),
            Color::Blue => "#000080".to_string(),
            Color::Magenta => "#800080".to_string(),
            Color::Cyan => "#008080".to_string(),
            Color::White => "#c0c0c0".to_string(),
            Color::DarkGray => "#808080".to_string(),
            Color::LightRed => "#ff0000".to_string(),
            Color::LightGreen => "#00ff00".to_string(),
            Color::LightYellow => "#ffff00".to_string(),
            Color::LightBlue => "#0000ff".to_string(),
            Color::LightMagenta => "#ff00ff".to_string(),
            Color::LightCyan => "#00ffff".to_string(),
            Color::Gray => "#ffffff".to_string(),
            Color::Indexed(idx) => Self::indexed_color_to_css(idx),
            Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        }
    }

    /// Converts an indexed (256-color) to CSS.
    fn indexed_color_to_css(idx: u8) -> String {
        match idx {
            0..=15 => Self::standard_color(idx).to_string(),
            16..=231 => {
                let n = idx - 16;
                let r = (n / 36) * 51;
                let g = ((n % 36) / 6) * 51;
                let b = (n % 6) * 51;
                format!("#{r:02x}{g:02x}{b:02x}")
            }
            _ => {
                // Grayscale (232-255): gray = (idx - 232) * 10 + 8
                // Max value: (255 - 232) * 10 + 8 = 238, which is < 255
                let gray = (idx - 232) * 10 + 8;
                format!("#{gray:02x}{gray:02x}{gray:02x}")
            }
        }
    }

    /// Standard ANSI color palette (16 colors).
    #[allow(clippy::match_same_arms)]
    const fn standard_color(idx: u8) -> &'static str {
        match idx {
            0 => "#000000", // Black
            1 => "#800000", // Red
            2 => "#008000", // Green
            3 => "#808000", // Yellow
            4 => "#000080", // Blue
            5 => "#800080", // Magenta
            6 => "#008080", // Cyan
            7 => "#c0c0c0", // White
            8 => "#808080", // Bright Black (Dark Gray)
            9 => "#ff0000", // Bright Red
            10 => "#00ff00", // Bright Green
            11 => "#ffff00", // Bright Yellow
            12 => "#0000ff", // Bright Blue
            13 => "#ff00ff", // Bright Magenta
            14 => "#00ffff", // Bright Cyan
            15 => "#ffffff", // Bright White
            _ => "#000000", // Default to black for invalid indices
        }
    }

    /// Draws a single cell.
    fn draw_cell(&self, x: u16, y: u16, cell: &Cell) -> Result<()> {
        let px = f64::from(x) * self.char_width;
        let py = f64::from(y) * self.char_height;

        // Draw background first
        if cell.bg != Color::Reset {
            let bg_css = Self::color_to_css(cell.bg);
            self.ctx.set_fill_style_str(&bg_css);
            self.ctx.fill_rect(px, py, self.char_width, self.char_height);
        }

        // Draw text (skip empty or skipped cells)
        if !cell.skip && !cell.symbol.is_empty() && cell.symbol != " " {
            let fg_css = Self::color_to_css(cell.fg);
            self.ctx.set_fill_style_str(&fg_css);
            let baseline = self.char_height.mul_add(0.85, py);
            self.ctx
                .fill_text(&cell.symbol, px, baseline)
                .map_err(|_| io::Error::other("Failed to draw text"))?;
        }

        Ok(())
    }
}

impl Backend for CanvasBackend {
    fn draw<I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, Cell)>,
    {
        for (x, y, cell) in content {
            self.draw_cell(x, y, &cell)?;
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        let width = f64::from(self.canvas.width());
        let height = f64::from(self.canvas.height());
        self.ctx.set_fill_style_str("#000000");
        self.ctx.fill_rect(0.0, 0.0, width, height);
        self.cursor_x = 0;
        self.cursor_y = 0;
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn size(&self) -> Result<Rect> {
        let width = (f64::from(self.canvas.width()) / self.char_width).floor() as u16;
        let height = (f64::from(self.canvas.height()) / self.char_height).floor() as u16;
        Ok(Rect::new(0, 0, width.max(1), height.max(1)))
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    fn cursor_pos(&self) -> Result<(u16, u16)> {
        Ok((self.cursor_x, self.cursor_y))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        self.cursor_x = x;
        self.cursor_y = y;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_to_css() {
        assert_eq!(CanvasBackend::color_to_css(Color::Black), "#000000");
        assert_eq!(CanvasBackend::color_to_css(Color::Red), "#800000");
        assert_eq!(
            CanvasBackend::color_to_css(Color::Rgb(255, 128, 0)),
            "#ff8000"
        );
        assert_eq!(CanvasBackend::color_to_css(Color::Reset), "transparent");
    }

    #[test]
    fn test_standard_color() {
        assert_eq!(CanvasBackend::standard_color(0), "#000000");
        assert_eq!(CanvasBackend::standard_color(7), "#c0c0c0");
        assert_eq!(CanvasBackend::standard_color(15), "#ffffff");
    }

    #[test]
    fn test_indexed_color_to_css() {
        // First 16 colors should use standard mapping
        assert_eq!(CanvasBackend::indexed_color_to_css(0), "#000000");
        assert_eq!(CanvasBackend::indexed_color_to_css(15), "#ffffff");

        // 6x6x6 color cube (16-231)
        // Index 16 = r:0, g:0, b:0 = #000000
        assert_eq!(CanvasBackend::indexed_color_to_css(16), "#000000");
        // Index 196 = r:255, g:0, b:0 = #ff0000 (bright red)
        assert_eq!(CanvasBackend::indexed_color_to_css(196), "#ff0000");

        // Grayscale (232-255)
        // Index 232 = gray 8 = #080808
        assert_eq!(CanvasBackend::indexed_color_to_css(232), "#080808");
    }
}
