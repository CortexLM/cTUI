//! Paragraph component for multi-line text rendering

use crate::text::{Alignment, Line, Text};
use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};
use unicode_width::UnicodeWidthStr;

/// A paragraph component that renders multi-line text.
///
/// Supports text alignment, line wrapping, and scrolling for long content.
#[derive(Clone, Debug)]
pub struct Paragraph {
    /// The text content
    text: Text,
    /// Text alignment
    alignment: Alignment,
    /// Whether to wrap text
    wrap: bool,
    /// Scroll offset (vertical, in lines)
    scroll: u16,
    /// Base style for the paragraph
    style: Style,
}

impl Paragraph {
    /// Creates a new paragraph from text
    pub fn new(text: impl Into<Text>) -> Self {
        Self {
            text: text.into(),
            alignment: Alignment::default(),
            wrap: true,
            scroll: 0,
            style: Style::default(),
        }
    }

    /// Sets text alignment
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Enables or disables text wrapping
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets the scroll offset (in lines)
    pub fn scroll(mut self, scroll: u16) -> Self {
        self.scroll = scroll;
        self
    }

    /// Sets the base style
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Returns the text content
    pub fn text(&self) -> &Text {
        &self.text
    }

    /// Returns the current scroll offset
    pub fn scroll_offset(&self) -> u16 {
        self.scroll
    }

    /// Scrolls by the given number of lines
    pub fn scroll_by(&mut self, lines: i16) {
        if lines >= 0 {
            self.scroll = self.scroll.saturating_add(lines as u16);
        } else {
            self.scroll = self.scroll.saturating_sub((-lines) as u16);
        }
    }

    /// Wraps a line to fit within the given width.
    ///
    /// Returns a vector of wrapped lines.
    fn wrap_line(line: &Line, width: usize) -> Vec<Line> {
        if width == 0 {
            return vec![Line::new()];
        }

        let content = line.content();
        let line_width = UnicodeWidthStr::width(content.as_str());

        if line_width <= width {
            return vec![line.clone()];
        }

        let mut wrapped_lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0;
        let mut chars = content.as_str().chars().peekable();

        while let Some(ch) = chars.next() {
            let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());

            if current_width + ch_width > width {
                // Try to break at word boundary if possible
                if ch == ' ' {
                    // Space at the boundary - just end current line
                    if !current_line.is_empty() {
                        wrapped_lines.push(Line::styled(current_line.clone(), *line.style_ref()));
                        current_line.clear();
                        current_width = 0;
                    }
                } else if !current_line.is_empty() {
                    // Find a good break point
                    if let Some(last_space_pos) = current_line.rfind(' ') {
                        // Break at last space
                        let before_space = &current_line[..last_space_pos];
                        let after_space = &current_line[last_space_pos + 1..];
                        wrapped_lines
                            .push(Line::styled(before_space.to_string(), *line.style_ref()));
                        current_line = after_space.to_string();
                        current_width = UnicodeWidthStr::width(current_line.as_str());
                    } else {
                        // No space found, force break
                        wrapped_lines.push(Line::styled(current_line.clone(), *line.style_ref()));
                        current_line.clear();
                        current_width = 0;
                    }
                }
                // Add current character to new line
                current_line.push(ch);
                current_width += ch_width;
            } else {
                current_line.push(ch);
                current_width += ch_width;
            }
        }

        if !current_line.is_empty() {
            wrapped_lines.push(Line::styled(current_line, *line.style_ref()));
        }

        if wrapped_lines.is_empty() {
            vec![Line::new()]
        } else {
            wrapped_lines
        }
    }

    /// Wraps all text lines to fit within the given width.
    fn wrap_text(&self, width: usize) -> Vec<Line> {
        if !self.wrap {
            return self.text.lines().to_vec();
        }

        self.text
            .lines()
            .iter()
            .flat_map(|line| Self::wrap_line(line, width))
            .collect()
    }

    /// Calculates the x offset for text alignment
    fn align_x(&self, text_width: usize, area_width: u16) -> u16 {
        match self.alignment {
            Alignment::Left => 0,
            Alignment::Center => {
                let area_w = area_width as usize;
                if text_width >= area_w {
                    0
                } else {
                    ((area_w - text_width) / 2) as u16
                }
            }
            Alignment::Right => {
                let area_w = area_width as usize;
                if text_width >= area_w {
                    0
                } else {
                    (area_w - text_width) as u16
                }
            }
        }
    }
}

/// Props for creating a Paragraph
pub struct ParagraphProps {
    pub text: Text,
    pub alignment: Alignment,
    pub wrap: bool,
    pub style: Style,
}

impl ParagraphProps {
    pub fn new(text: impl Into<Text>) -> Self {
        Self {
            text: text.into(),
            alignment: Alignment::default(),
            wrap: true,
            style: Style::default(),
        }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Component for Paragraph {
    type Props = ParagraphProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            text: props.text,
            alignment: props.alignment,
            wrap: props.wrap,
            scroll: 0,
            style: props.style,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        // Wrap text to fit area width
        let wrapped_lines = self.wrap_text(area.width as usize);

        // Apply scroll offset
        let scroll = self.scroll as usize;
        let start_line = scroll.min(wrapped_lines.len());

        // Render each line
        for (y_offset, line) in wrapped_lines
            .iter()
            .skip(start_line)
            .take(area.height as usize)
            .enumerate()
        {
            let y = area.y + y_offset as u16;
            if y >= area.y + area.height {
                break;
            }

            let line_width = line.width();
            let x_offset = self.align_x(line_width, area.width);

            // Start rendering at the aligned position
            let mut term_x = area.x + x_offset;
            let styled_chars = line.styled_chars();

            for (ch, char_style) in styled_chars {
                let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());

                // Check if we've reached the right edge
                if term_x >= area.x + area.width {
                    break;
                }

                // Write the character
                buf.modify_cell(term_x, y, |cell| {
                    cell.symbol = ch.to_string();
                    // Merge base style with line style and char style
                    let merged_style = Style {
                    fg: if char_style.fg != ctui_core::style::Color::Reset {
                    char_style.fg
                    } else if self.style.fg != ctui_core::style::Color::Reset {
                    self.style.fg
                    } else {
                    line.style_ref().fg
                    },
                    bg: if char_style.bg != ctui_core::style::Color::Reset {
                    char_style.bg
                    } else if self.style.bg != ctui_core::style::Color::Reset {
                    self.style.bg
                    } else {
                    line.style_ref().bg
                    },
                    modifier: self.style.modifier
                    | line.style_ref().modifier
                    | char_style.modifier,
                    };
                    cell.set_style(merged_style);
                });

                // Handle wide characters
                if ch_width > 1 {
                    for i in 1..ch_width {
                        let next_x = term_x + i as u16;
                        if next_x < area.x + area.width {
                            buf.modify_cell(next_x, y, |cell| { cell.skip = true; });
                        }
                    }
                }

                term_x += ch_width as u16;
            }
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctui_core::Buffer;
    use insta::assert_snapshot;

    /// Helper to render a Paragraph to string for snapshot testing.
    ///
    /// # Snapshot Testing Workflow
    ///
    /// To update snapshots after intentional changes:
    /// 1. Run `cargo insta test -p ctui-components` to see failures
    /// 2. Run `cargo insta review` to accept/reject changes
    /// 3. Accepted snapshots are saved to `src/snapshots/`
    fn render_to_string(paragraph: &Paragraph, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        paragraph.render(Rect::new(0, 0, width, height), &mut buf);

        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = buf.get(x, y) { output.push_str(&cell.symbol); }
            }
            if y < height - 1 {
                output.push('\n');
            }
        }
        output
    }

    // ==================== Snapshot Tests ====================
    // These tests use insta to capture expected rendering output.
    // Run `cargo insta test` and `cargo insta review` to manage snapshots.

    #[test]
    fn snapshot_paragraph_single_line() {
        let p = Paragraph::new("Hello, World!");
        let result = render_to_string(&p, 20, 1);
        assert_snapshot!("paragraph_single_line", result);
    }

    #[test]
    fn snapshot_paragraph_multiline() {
        let p = Paragraph::new("Line one\nLine two\nLine three");
        let result = render_to_string(&p, 15, 3);
        assert_snapshot!("paragraph_multiline", result);
    }

    #[test]
    fn snapshot_paragraph_wrap_enabled() {
        let p = Paragraph::new("This is a long line that should wrap to fit the width");
        let result = render_to_string(&p, 15, 5);
        assert_snapshot!("paragraph_wrap_enabled", result);
    }

    #[test]
    fn snapshot_paragraph_wrap_disabled() {
        let p = Paragraph::new("This is a long line that should be truncated").wrap(false);
        let result = render_to_string(&p, 15, 1);
        assert_snapshot!("paragraph_wrap_disabled", result);
    }

    #[test]
    fn snapshot_paragraph_align_left() {
        let p = Paragraph::new("Left").alignment(Alignment::Left);
        let result = render_to_string(&p, 15, 1);
        assert_snapshot!("paragraph_align_left", result);
    }

    #[test]
    fn snapshot_paragraph_align_center() {
        let p = Paragraph::new("Center").alignment(Alignment::Center);
        let result = render_to_string(&p, 15, 1);
        assert_snapshot!("paragraph_align_center", result);
    }

    #[test]
    fn snapshot_paragraph_align_right() {
        let p = Paragraph::new("Right").alignment(Alignment::Right);
        let result = render_to_string(&p, 15, 1);
        assert_snapshot!("paragraph_align_right", result);
    }

    #[test]
    fn snapshot_paragraph_scroll_offset() {
        let p = Paragraph::new("Line 1\nLine 2\nLine 3\nLine 4\nLine 5").scroll(2);
        let result = render_to_string(&p, 15, 3);
        assert_snapshot!("paragraph_scroll_offset", result);
    }

    #[test]
    fn snapshot_paragraph_empty() {
        let p = Paragraph::new("");
        let result = render_to_string(&p, 10, 2);
        assert_snapshot!("paragraph_empty", result);
    }

    #[test]
    fn snapshot_paragraph_single_char() {
        let p = Paragraph::new("X");
        let result = render_to_string(&p, 10, 1);
        assert_snapshot!("paragraph_single_char", result);
    }

    // ==================== Unit Tests ====================

    #[test]
    fn test_paragraph_new() {
        let p = Paragraph::new("Hello");
        assert_eq!(p.text().lines().len(), 1);
        assert_eq!(p.alignment, Alignment::Left);
        assert!(p.wrap);
    }

    #[test]
    fn test_paragraph_with_alignment() {
        let p = Paragraph::new("Test").alignment(Alignment::Center);
        assert_eq!(p.alignment, Alignment::Center);
    }

    #[test]
    fn test_paragraph_scroll() {
        let mut p = Paragraph::new("Line 1\nLine 2\nLine 3");
        assert_eq!(p.scroll_offset(), 0);

        p.scroll_by(1);
        assert_eq!(p.scroll_offset(), 1);

        p.scroll_by(-1);
        assert_eq!(p.scroll_offset(), 0);

        // Can't go negative
        p.scroll_by(-5);
        assert_eq!(p.scroll_offset(), 0);
    }

    #[test]
    fn test_wrap_line_short() {
        let line = Line::from("Short");
        let wrapped = Paragraph::wrap_line(&line, 10);
        assert_eq!(wrapped.len(), 1);
        assert_eq!(wrapped[0].content(), "Short");
    }

    #[test]
    fn test_wrap_line_exact() {
        let line = Line::from("12345");
        let wrapped = Paragraph::wrap_line(&line, 5);
        assert_eq!(wrapped.len(), 1);
        assert_eq!(wrapped[0].content(), "12345");
    }

    #[test]
    fn test_wrap_line_long() {
        let line = Line::from("1234567890");
        let wrapped = Paragraph::wrap_line(&line, 5);
        assert!(wrapped.len() >= 1);
    }

    #[test]
    fn test_wrap_line_word_boundary() {
        let line = Line::from("Hello World Test");
        let wrapped = Paragraph::wrap_line(&line, 7);
        // Should break at word boundaries when possible
        assert!(wrapped.len() >= 2);
    }

    #[test]
    fn test_wrap_line_preserves_style() {
        use ctui_core::style::Color;
        let line = Line::styled("Styled", Style::new().fg(Color::Red));
        let wrapped = Paragraph::wrap_line(&line, 3);
        for w in &wrapped {
            let chars = w.styled_chars();
            for (_, style) in chars {
                assert_eq!(style.fg, Color::Red);
            }
        }
    }

    #[test]
    fn test_align_x_left() {
        let p = Paragraph::new("Test").alignment(Alignment::Left);
        assert_eq!(p.align_x(5, 10), 0);
    }

    #[test]
    fn test_align_x_center() {
        let p = Paragraph::new("Test").alignment(Alignment::Center);
        assert_eq!(p.align_x(5, 15), 5); // (15-5)/2 = 5
        assert_eq!(p.align_x(6, 10), 2); // (10-6)/2 = 2
    }

    #[test]
    fn test_align_x_right() {
        let p = Paragraph::new("Test").alignment(Alignment::Right);
        assert_eq!(p.align_x(5, 10), 5); // 10-5 = 5
    }

    #[test]
    fn test_render_basic() {
        let p = Paragraph::new("Hello");
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
        p.render(Rect::new(0, 0, 20, 1), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().symbol, "H");
        assert_eq!(buf.get(1, 0).unwrap().symbol, "e");
        assert_eq!(buf.get(2, 0).unwrap().symbol, "l");
    }

    #[test]
    fn test_render_multiline() {
        let p = Paragraph::new("Line 1\nLine 2");
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 2));
        p.render(Rect::new(0, 0, 10, 2), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().symbol, "L");
        assert_eq!(buf.get(0, 1).unwrap().symbol, "L");
    }

    #[test]
    fn test_render_centered() {
        let p = Paragraph::new("Hi").alignment(Alignment::Center);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        p.render(Rect::new(0, 0, 10, 1), &mut buf);

        // "Hi" is 2 chars, should be centered in 10 (offset 4)
        assert_eq!(buf.get(0, 0).unwrap().symbol, " ");
        assert_eq!(buf.get(4, 0).unwrap().symbol, "H");
        assert_eq!(buf.get(5, 0).unwrap().symbol, "i");
    }

    #[test]
    fn test_render_right_aligned() {
        let p = Paragraph::new("Hi").alignment(Alignment::Right);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        p.render(Rect::new(0, 0, 10, 1), &mut buf);

        // "Hi" is 2 chars, right aligned in 10 means offset 8
        assert_eq!(buf.get(8, 0).unwrap().symbol, "H");
        assert_eq!(buf.get(9, 0).unwrap().symbol, "i");
    }

    #[test]
    fn test_render_with_scroll() {
        let p = Paragraph::new("Line 1\nLine 2\nLine 3").scroll(1);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 2));
        p.render(Rect::new(0, 0, 10, 2), &mut buf);

        // Should show Line 2 and Line 3 (skipped Line 1)
        assert!(buf.get(0, 0).unwrap().symbol.starts_with('L'));
    }

    #[test]
    fn test_render_nowrap() {
        let p = Paragraph::new("Very long text that exceeds width").wrap(false);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        p.render(Rect::new(0, 0, 10, 1), &mut buf);

        // Should render but not wrap
        assert_eq!(buf.get(0, 0).unwrap().symbol, "V");
    }
}
