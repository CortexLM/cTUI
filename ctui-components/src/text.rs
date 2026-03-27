//! Text types for terminal rendering
//!
//! This module provides `Span`, `Line`, and `Text` types for representing
//! multi-line text content with optional styling.

use ctui_core::style::Style;
use unicode_width::UnicodeWidthStr;

/// A styled span of text.
///
/// A `Span` represents a contiguous string with a single style.
/// Multiple spans can be combined in a `Line` for mixed styling.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Span {
    content: String,
    style: Style,
}

impl Span {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn styled(content: impl Into<String>, style: Style) -> Self {
        Self {
            content: content.into(),
            style,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn style_ref(&self) -> &Style {
        &self.style
    }

    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self.content.as_str())
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn chars(&self) -> std::str::Chars<'_> {
        self.content.chars()
    }
}

impl From<&str> for Span {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Span {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// A single line of text with optional styling.
///
/// A `Line` represents one line of text that can be rendered to the terminal.
/// It contains a vector of spans, each potentially having different styles.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Line {
    spans: Vec<Span>,
    style: Style,
}

impl Line {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(content: impl Into<String>) -> Self {
        Self {
            spans: vec![Span::new(content)],
            style: Style::default(),
        }
    }

    pub fn styled(content: impl Into<String>, style: Style) -> Self {
        Self {
            spans: vec![Span::styled(content, style)],
            style,
        }
    }

    pub fn from_spans(spans: Vec<Span>) -> Self {
        Self {
            spans,
            style: Style::default(),
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }

    pub fn span_styled(mut self, content: impl Into<String>, style: Style) -> Self {
        self.spans.push(Span::styled(content, style));
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn content(&self) -> String {
        self.spans.iter().map(|s| s.content.as_str()).collect()
    }

    pub fn style_ref(&self) -> &Style {
        &self.style
    }

    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    pub fn width(&self) -> usize {
        self.spans.iter().map(|s| s.width()).sum()
    }

    pub fn len(&self) -> usize {
        self.spans.iter().map(|s| s.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.spans.iter().all(|s| s.is_empty())
    }

    pub fn styled_chars(&self) -> Vec<(char, Style)> {
        let base_style = self.style;
        let mut result = Vec::new();
        for span in &self.spans {
            let merged = Style {
                fg: if span.style.fg != ctui_core::style::Color::Reset {
                    span.style.fg
                } else {
                    base_style.fg
                },
                bg: if span.style.bg != ctui_core::style::Color::Reset {
                    span.style.bg
                } else {
                    base_style.bg
                },
                modifier: base_style.modifier | span.style.modifier,
            };
            for ch in span.content.chars() {
                result.push((ch, merged));
            }
        }
        result
    }
}

impl From<String> for Line {
    fn from(s: String) -> Self {
        Self::from(s)
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for span in &self.spans {
            write!(f, "{}", span.content)?;
        }
        Ok(())
    }
}

/// Multi-line text content.
///
/// `Text` represents multiple lines of text that can be rendered together.
/// It's the primary type used by the `Paragraph` component.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Text {
    /// The lines that make up this text
    lines: Vec<Line>,
}

impl Text {
    /// Creates new empty text
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates text from a single line
    pub fn from(line: impl Into<Line>) -> Self {
        Self {
            lines: vec![line.into()],
        }
    }

    /// Creates text from a raw string (lines separated by `\n`)
    pub fn raw(content: impl Into<String>) -> Self {
        let content = content.into();
        let lines: Vec<Line> = content.split('\n').map(Line::from).collect();
        Self { lines }
    }

    /// Adds a line to this text
    pub fn push_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    /// Adds a line and returns self for chaining
    pub fn line(mut self, line: impl Into<Line>) -> Self {
        self.lines.push(line.into());
        self
    }

    /// Returns the lines in this text
    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    /// Returns the number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Returns the maximum width across all lines
    pub fn width(&self) -> usize {
        self.lines.iter().map(|l| l.width()).max().unwrap_or(0)
    }

    /// Returns true if this text has no lines
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Returns the total number of characters across all lines
    pub fn len(&self) -> usize {
        self.lines.iter().map(|l| l.len()).sum()
    }

    /// Sets the style for all lines
    pub fn style(mut self, style: Style) -> Self {
        for line in &mut self.lines {
            line.style = style;
        }
        self
    }

    /// Creates an iterator over the lines
    pub fn iter(&self) -> std::slice::Iter<'_, Line> {
        self.lines.iter()
    }

    /// Return the lines as a vector
    pub fn into_lines(self) -> Vec<Line> {
        self.lines
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::raw(s)
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::raw(s)
    }
}

impl From<Line> for Text {
    fn from(line: Line) -> Self {
        Self::from(line)
    }
}

impl From<Vec<Line>> for Text {
    fn from(lines: Vec<Line>) -> Self {
        Self { lines }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, line) in self.lines.iter().enumerate() {
            if i > 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}

/// Text alignment options
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Alignment {
    /// Left aligned (default)
    #[default]
    Left,
    /// Center aligned
    Center,
    /// Right aligned
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctui_core::style::Color;

    #[test]
    fn test_span_new() {
        let span = Span::new("Hello");
        assert_eq!(span.content(), "Hello");
        assert_eq!(span.width(), 5);
        assert!(span.style_ref().fg == Color::Reset);
    }

    #[test]
    fn test_span_styled() {
        let span = Span::styled("Test", Style::new().fg(Color::Red));
        assert_eq!(span.content(), "Test");
        assert_eq!(span.style_ref().fg, Color::Red);
    }

    #[test]
    fn test_span_width() {
        let span = Span::new("你好");
        assert_eq!(span.width(), 4);
    }

    #[test]
    fn test_line_new() {
        let line = Line::new();
        assert!(line.is_empty());
        assert_eq!(line.width(), 0);
    }

    #[test]
    fn test_line_from_string() {
        let line = Line::from("Hello, World!");
        assert_eq!(line.content(), "Hello, World!");
        assert_eq!(line.width(), 13);
    }

    #[test]
    fn test_line_from_spans() {
        let line = Line::from_spans(vec![
            Span::styled("Hello ", Style::new().fg(Color::Red)),
            Span::styled("World", Style::new().fg(Color::Blue)),
        ]);
        assert_eq!(line.content(), "Hello World");
        assert_eq!(line.spans().len(), 2);
    }

    #[test]
    fn test_line_span_chaining() {
        let line = Line::new()
            .span(Span::new("Hello "))
            .span_styled("World", Style::new().fg(Color::Green));
        assert_eq!(line.content(), "Hello World");
        assert_eq!(line.spans().len(), 2);
    }

    #[test]
    fn test_line_width_cjk() {
        let line = Line::from("你好");
        assert_eq!(line.width(), 4);
    }

    #[test]
    fn test_line_width_emoji() {
        let line = Line::from("😀");
        assert_eq!(line.width(), 2);
    }

    #[test]
    fn test_line_styled() {
        let line = Line::styled("Styled", Style::new().fg(Color::Red));
        assert!(line.spans().first().is_some());
        assert_eq!(line.spans().first().unwrap().style_ref().fg, Color::Red);
    }

    #[test]
    fn test_line_styled_chars() {
        let line = Line::from_spans(vec![
            Span::styled("AB", Style::new().fg(Color::Red)),
            Span::styled("CD", Style::new().fg(Color::Blue)),
        ]);
        let chars = line.styled_chars();
        assert_eq!(chars.len(), 4);
        assert_eq!(chars[0].0, 'A');
        assert_eq!(chars[0].1.fg, Color::Red);
        assert_eq!(chars[2].0, 'C');
        assert_eq!(chars[2].1.fg, Color::Blue);
    }

    #[test]
    fn test_text_from_raw() {
        let text = Text::raw("Line 1\nLine 2\nLine 3");
        assert_eq!(text.line_count(), 3);
        assert_eq!(text.lines()[0].content(), "Line 1");
        assert_eq!(text.lines()[1].content(), "Line 2");
        assert_eq!(text.lines()[2].content(), "Line 3");
    }

    #[test]
    fn test_text_width() {
        let text = Text::raw("Short\nVery Long Line\nMed");
        assert_eq!(text.width(), 14);
    }

    #[test]
    fn test_text_empty() {
        let text = Text::new();
        assert!(text.is_empty());
        assert_eq!(text.line_count(), 0);
    }

    #[test]
    fn test_text_chaining() {
        let text = Text::new()
            .line(Line::from("First"))
            .line(Line::from("Second"))
            .line(Line::from("Third"));
        assert_eq!(text.line_count(), 3);
    }

    #[test]
    fn test_alignment_default() {
        let align = Alignment::default();
        assert_eq!(align, Alignment::Left);
    }
}
