//! Markdown component for rendering formatted text

use crate::text::{Line, Span};
use crate::Widget;
use ctui_core::style::{Color, Modifier, Style};
use ctui_core::{Buffer, Rect};
use unicode_width::UnicodeWidthStr;

/// Theme for Markdown rendering
#[derive(Clone, Debug)]
pub struct MarkdownTheme {
    pub heading_color: Color,
    pub bold_style: Style,
    pub italic_style: Style,
    pub code_bg: Color,
    pub code_fg: Color,
    pub link_color: Color,
    pub quote_color: Color,
    pub list_bullet: char,
    pub code_block_bg: Color,
    pub code_block_fg: Color,
}

impl Default for MarkdownTheme {
    fn default() -> Self {
        Self {
            heading_color: Color::Cyan,
            bold_style: Style::new().add_modifier(Modifier::BOLD),
            italic_style: Style::new().add_modifier(Modifier::ITALIC),
            code_bg: Color::DarkGray,
            code_fg: Color::White,
            link_color: Color::Blue,
            quote_color: Color::Yellow,
            list_bullet: '•',
            code_block_bg: Color::Indexed(235),
            code_block_fg: Color::Gray,
        }
    }
}

impl MarkdownTheme {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heading_color(mut self, color: Color) -> Self {
        self.heading_color = color;
        self
    }

    pub fn bold_style(mut self, style: Style) -> Self {
        self.bold_style = style;
        self
    }

    pub fn italic_style(mut self, style: Style) -> Self {
        self.italic_style = style;
        self
    }

    pub fn code_colors(mut self, fg: Color, bg: Color) -> Self {
        self.code_fg = fg;
        self.code_bg = bg;
        self
    }

    pub fn link_color(mut self, color: Color) -> Self {
        self.link_color = color;
        self
    }

    pub fn quote_color(mut self, color: Color) -> Self {
        self.quote_color = color;
        self
    }

    pub fn list_bullet(mut self, bullet: char) -> Self {
        self.list_bullet = bullet;
        self
    }
}

/// A parsed inline element
#[derive(Clone, Debug, PartialEq)]
pub enum Inline {
    Text(String),
    Bold(String),
    Italic(String),
    Code(String),
    Link { text: String, url: String },
}

/// A parsed block-level markdown element
#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownNode {
    Heading {
        level: u8,
        text: String,
    },
    Paragraph(Vec<Inline>),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    List {
        ordered: bool,
        items: Vec<Vec<Inline>>,
    },
    Blockquote(Vec<Inline>),
    Hr,
}

/// A Markdown renderer component
#[derive(Clone, Debug)]
pub struct Markdown {
    content: String,
    wrap: bool,
    theme: MarkdownTheme,
    nodes: Vec<MarkdownNode>,
}

impl Markdown {
    pub fn new(content: impl Into<String>) -> Self {
        let content = content.into();
        let nodes = parse_markdown(&content);
        Self {
            content,
            wrap: true,
            theme: MarkdownTheme::default(),
            nodes,
        }
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn theme(mut self, theme: MarkdownTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn nodes(&self) -> &[MarkdownNode] {
        &self.nodes
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn parse(&mut self) {
        self.nodes = parse_markdown(&self.content);
    }

    fn render_inline(&self, inline: &Inline) -> Vec<Span> {
        match inline {
            Inline::Text(text) => vec![Span::new(text.clone())],
            Inline::Bold(text) => vec![Span::styled(text.clone(), self.theme.bold_style)],
            Inline::Italic(text) => vec![Span::styled(text.clone(), self.theme.italic_style)],
            Inline::Code(text) => vec![Span::styled(
                format!(" {} ", text),
                Style::new().fg(self.theme.code_fg).bg(self.theme.code_bg),
            )],
            Inline::Link { text, url: _ } => {
                vec![Span::styled(
                    text.clone(),
                    Style::new()
                        .fg(self.theme.link_color)
                        .add_modifier(Modifier::UNDERLINED),
                )]
            }
        }
    }

    fn nodes_to_lines(&self) -> Vec<Line> {
        let mut lines = Vec::new();

        for node in &self.nodes {
            match node {
                MarkdownNode::Heading { level, text } => {
                    let prefix = "#".repeat(*level as usize);
                    let style = Style::new()
                        .fg(self.theme.heading_color)
                        .add_modifier(Modifier::BOLD);
                    lines.push(Line::styled(format!("{} {}", prefix, text), style));
                    lines.push(Line::new());
                }
                MarkdownNode::Paragraph(inlines) => {
                    let mut current_line_spans: Vec<Span> = Vec::new();
                    for inline in inlines {
                        match inline {
                            Inline::Text(t) => {
                                for (i, part) in t.split('\n').enumerate() {
                                    if i > 0 {
                                        lines.push(Line::from_spans(std::mem::take(
                                            &mut current_line_spans,
                                        )));
                                    }
                                    current_line_spans.push(Span::new(part.to_string()));
                                }
                            }
                            _ => {
                                current_line_spans.extend(self.render_inline(inline));
                            }
                        }
                    }
                    if !current_line_spans.is_empty() {
                        lines.push(Line::from_spans(current_line_spans));
                    }
                    lines.push(Line::new());
                }
                MarkdownNode::CodeBlock { language, code } => {
                    if let Some(lang) = language {
                        if !lang.is_empty() {
                            lines.push(Line::styled(
                                format!("```{}", lang),
                                Style::new().fg(Color::DarkGray),
                            ));
                        }
                    } else {
                        lines.push(Line::styled("```", Style::new().fg(Color::DarkGray)));
                    }

                    for line in code.split('\n') {
                        lines.push(Line::styled(
                            line.to_string(),
                            Style::new()
                                .fg(self.theme.code_block_fg)
                                .bg(self.theme.code_block_bg),
                        ));
                    }

                    lines.push(Line::styled("```", Style::new().fg(Color::DarkGray)));
                    lines.push(Line::new());
                }
                MarkdownNode::List { ordered, items } => {
                    for (i, item_inlines) in items.iter().enumerate() {
                        let prefix = if *ordered {
                            format!("{}. ", i + 1)
                        } else {
                            format!("{} ", self.theme.list_bullet)
                        };

                        let mut spans = vec![Span::styled(
                            prefix,
                            Style::new().fg(self.theme.heading_color),
                        )];

                        for inline in item_inlines {
                            if matches!(inline, Inline::Text(_)) {
                                if let Inline::Text(t) = inline {
                                    for (j, part) in t.split('\n').enumerate() {
                                        if j > 0 {
                                            lines
                                                .push(Line::from_spans(std::mem::take(&mut spans)));
                                            spans =
                                                vec![Span::new("  "), Span::new(part.to_string())];
                                        } else {
                                            spans.push(Span::new(part.to_string()));
                                        }
                                    }
                                }
                            } else {
                                spans.extend(self.render_inline(inline));
                            }
                        }

                        if !spans.is_empty() {
                            lines.push(Line::from_spans(spans));
                        }
                    }
                    lines.push(Line::new());
                }
                MarkdownNode::Blockquote(inlines) => {
                    let mut quote_spans =
                        vec![Span::styled("> ", Style::new().fg(self.theme.quote_color))];

                    for inline in inlines {
                        match inline {
                            Inline::Text(t) => {
                                for (j, part) in t.split('\n').enumerate() {
                                    if j > 0 {
                                        lines.push(Line::from_spans(std::mem::take(
                                            &mut quote_spans,
                                        )));
                                        quote_spans = vec![Span::styled(
                                            "> ",
                                            Style::new().fg(self.theme.quote_color),
                                        )];
                                    }
                                    quote_spans.push(Span::styled(
                                        part.to_string(),
                                        Style::new().fg(self.theme.quote_color),
                                    ));
                                }
                            }
                            _ => {
                                let rendered = self.render_inline(inline);
                                for span in rendered {
                                    let new_span = if span.style_ref().fg == Color::Reset {
                                        Span::styled(
                                            span.content().to_string(),
                                            Style::new()
                                                .fg(self.theme.quote_color)
                                                .bg(span.style_ref().bg)
                                                .modifier(span.style_ref().modifier),
                                        )
                                    } else {
                                        span
                                    };
                                    quote_spans.push(new_span);
                                }
                            }
                        }
                    }

                    if quote_spans.len() > 1 {
                        lines.push(Line::from_spans(quote_spans));
                    }
                    lines.push(Line::new());
                }
                MarkdownNode::Hr => {
                    lines.push(Line::styled(
                        "─".repeat(40),
                        Style::new().fg(Color::DarkGray),
                    ));
                    lines.push(Line::new());
                }
            }
        }

        if lines.last().map_or(false, |l| l.is_empty()) {
            lines.pop();
        }

        lines
    }

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
        let mut current_line_spans: Vec<Span> = Vec::new();
        let mut current_width = 0;

        for span in line.spans() {
            let span_content = span.content();
            let span_style = *span.style_ref();

            for ch in span_content.chars() {
                let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());

                if current_width + ch_width > width {
                    if ch == ' ' {
                        if !current_line_spans.is_empty() {
                            wrapped_lines
                                .push(Line::from_spans(std::mem::take(&mut current_line_spans)));
                            current_width = 0;
                        }
                    } else if !current_line_spans.is_empty() {
                        let last_span = current_line_spans.last_mut();
                        if let Some(last) = last_span {
                            let content = last.content().to_string();
                            if let Some(pos) = content.rfind(' ') {
                                let before = &content[..pos];
                                let after = &content[pos + 1..];
                                *last = Span::styled(before.to_string(), last.style_ref().clone());
                                wrapped_lines.push(Line::from_spans(std::mem::take(
                                    &mut current_line_spans,
                                )));
                                current_line_spans
                                    .push(Span::styled(after.to_string(), span_style));
                                current_width = UnicodeWidthStr::width(after);
                            } else {
                                wrapped_lines.push(Line::from_spans(std::mem::take(
                                    &mut current_line_spans,
                                )));
                                current_line_spans.push(Span::styled(ch.to_string(), span_style));
                                current_width = ch_width;
                            }
                        }
                    } else {
                        current_line_spans.push(Span::styled(ch.to_string(), span_style));
                        current_width = ch_width;
                    }
                } else {
                    if let Some(last) = current_line_spans.last_mut() {
                        if last.style_ref() == &span_style {
                            let new_content = format!("{}{}", last.content(), ch);
                            *last = Span::styled(new_content, span_style);
                        } else {
                            current_line_spans.push(Span::styled(ch.to_string(), span_style));
                        }
                    } else {
                        current_line_spans.push(Span::styled(ch.to_string(), span_style));
                    }
                    current_width += ch_width;
                }
            }
        }

        if !current_line_spans.is_empty() {
            wrapped_lines.push(Line::from_spans(current_line_spans));
        }

        if wrapped_lines.is_empty() {
            vec![Line::new()]
        } else {
            wrapped_lines
        }
    }
}

impl Widget for Markdown {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let lines = self.nodes_to_lines();
        let width = area.width as usize;

        let rendered_lines: Vec<Line> = if self.wrap {
            lines
                .iter()
                .flat_map(|line| Self::wrap_line(line, width))
                .collect()
        } else {
            lines
        };

        for (y_offset, line) in rendered_lines.iter().enumerate().take(area.height as usize) {
            let y = area.y + y_offset as u16;
            if y >= area.y + area.height {
                break;
            }

            let styled_chars = line.styled_chars();
            let mut term_x = area.x;

            for (ch, char_style) in styled_chars {
                let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());

                if term_x >= area.x + area.width {
                    break;
                }

                buf.modify_cell(term_x, y, |cell| {
                    cell.symbol = ch.to_string();
                    cell.set_style(char_style);
                });

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
}

/// Parses markdown content into a vector of nodes
pub fn parse_markdown(content: &str) -> Vec<MarkdownNode> {
    let mut nodes = Vec::new();
    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with('#') {
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count() as u8;
            if hash_count >= 1 && hash_count <= 6 {
                let text = trimmed[hash_count as usize..].trim().to_string();
                nodes.push(MarkdownNode::Heading {
                    level: hash_count,
                    text,
                });
                continue;
            }
        }

        if trimmed.starts_with("```") {
            let language = trimmed[3..].trim().to_string();
            let language = if language.is_empty() {
                None
            } else {
                Some(language)
            };

            let mut code_lines = Vec::new();
            while let Some(code_line) = lines.next() {
                if code_line.trim().starts_with("```") {
                    break;
                }
                code_lines.push(code_line);
            }

            let code = code_lines.join("\n");
            nodes.push(MarkdownNode::CodeBlock { language, code });
            continue;
        }

        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            nodes.push(MarkdownNode::Hr);
            continue;
        }

        if trimmed.starts_with('>') {
            let text = trimmed[1..].trim();
            let inlines = parse_inline(text);
            nodes.push(MarkdownNode::Blockquote(inlines));
            continue;
        }

        let unordered_prefix =
            trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ");

        let ordered_prefix = trimmed
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
            && trimmed.contains('.');

        if unordered_prefix || ordered_prefix {
            let mut items = Vec::new();
            let is_ordered = ordered_prefix;

            let item_text = if is_ordered {
                let dot_pos = trimmed.find('.').unwrap_or(0);
                trimmed[dot_pos + 1..].trim()
            } else {
                trimmed[2..].trim()
            };
            items.push(parse_inline(item_text));

            while let Some(next_line) = lines.peek() {
                let next_trimmed = next_line.trim();

                if next_trimmed.is_empty() {
                    lines.next();
                    continue;
                }

                let is_next_unordered = next_trimmed.starts_with("- ")
                    || next_trimmed.starts_with("* ")
                    || next_trimmed.starts_with("+ ");

                let is_next_ordered = next_trimmed
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
                    && next_trimmed.contains('.');

                if (is_ordered && !is_next_ordered) || (!is_ordered && !is_next_unordered) {
                    break;
                }

                lines.next();
                let next_text = if is_ordered {
                    let dot_pos = next_trimmed.find('.').unwrap_or(0);
                    next_trimmed[dot_pos + 1..].trim()
                } else {
                    next_trimmed[2..].trim()
                };
                items.push(parse_inline(next_text));
            }

            nodes.push(MarkdownNode::List {
                ordered: is_ordered,
                items,
            });
            continue;
        }

        let mut paragraph_lines = vec![trimmed];
        while let Some(next_line) = lines.peek() {
            let next_trimmed = next_line.trim();
            if next_trimmed.is_empty()
                || next_trimmed.starts_with('#')
                || next_trimmed.starts_with("```")
                || next_trimmed.starts_with('>')
                || next_trimmed.starts_with("- ")
                || next_trimmed.starts_with("* ")
                || next_trimmed.starts_with("+ ")
                || next_trimmed == "---"
                || next_trimmed == "***"
                || next_trimmed == "___"
            {
                break;
            }
            if next_trimmed
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
                && next_trimmed.contains('.')
            {
                break;
            }
            lines.next();
            paragraph_lines.push(next_trimmed);
        }

        let paragraph_text = paragraph_lines.join(" ");
        nodes.push(MarkdownNode::Paragraph(parse_inline(&paragraph_text)));
    }

    nodes
}

fn parse_inline(text: &str) -> Vec<Inline> {
    let mut inlines = Vec::new();
    let mut chars = text.chars().peekable();
    let mut current_text = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            '*' | '_' if chars.peek() == Some(&ch) => {
                chars.next();
                let delimiter = ch;

                let mut bold_text = String::new();
                let mut found_close = false;

                while let Some(c) = chars.next() {
                    if c == delimiter && chars.peek() == Some(&delimiter) {
                        chars.next();
                        found_close = true;
                        break;
                    }
                    bold_text.push(c);
                }

                if found_close {
                    if !current_text.is_empty() {
                        inlines.push(Inline::Text(current_text.clone()));
                        current_text.clear();
                    }
                    inlines.push(Inline::Bold(bold_text));
                } else {
                    current_text.push(ch);
                    current_text.push(ch);
                    current_text.push_str(&bold_text);
                }
            }
            '*' | '_' => {
                let delimiter = ch;
                let mut italic_text = String::new();
                let mut found_close = false;

                while let Some(c) = chars.next() {
                    if c == delimiter {
                        found_close = true;
                        break;
                    }
                    italic_text.push(c);
                }

                if found_close && !italic_text.is_empty() {
                    if !current_text.is_empty() {
                        inlines.push(Inline::Text(current_text.clone()));
                        current_text.clear();
                    }
                    inlines.push(Inline::Italic(italic_text));
                } else {
                    current_text.push(ch);
                    current_text.push_str(&italic_text);
                }
            }
            '`' => {
                let mut code_text = String::new();
                let mut found_close = false;

                while let Some(c) = chars.next() {
                    if c == '`' {
                        found_close = true;
                        break;
                    }
                    code_text.push(c);
                }

                if found_close {
                    if !current_text.is_empty() {
                        inlines.push(Inline::Text(current_text.clone()));
                        current_text.clear();
                    }
                    inlines.push(Inline::Code(code_text));
                } else {
                    current_text.push('`');
                    current_text.push_str(&code_text);
                }
            }
            '[' => {
                let mut link_text = String::new();
                let mut found_url_start = false;

                while let Some(c) = chars.next() {
                    if c == ']' {
                        found_url_start = true;
                        break;
                    }
                    link_text.push(c);
                }

                if found_url_start && chars.peek() == Some(&'(') {
                    chars.next();
                    let mut url = String::new();
                    let mut found_url_close = false;

                    while let Some(c) = chars.next() {
                        if c == ')' {
                            found_url_close = true;
                            break;
                        }
                        url.push(c);
                    }

                    if found_url_close {
                        if !current_text.is_empty() {
                            inlines.push(Inline::Text(current_text.clone()));
                            current_text.clear();
                        }
                        inlines.push(Inline::Link {
                            text: link_text,
                            url,
                        });
                        continue;
                    }
                }

                current_text.push('[');
                current_text.push_str(&link_text);
                current_text.push(']');
            }
            _ => {
                current_text.push(ch);
            }
        }
    }

    if !current_text.is_empty() {
        inlines.push(Inline::Text(current_text));
    }

    inlines
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctui_core::Buffer;

    fn render_to_string(markdown: &Markdown, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        markdown.render(Rect::new(0, 0, width, height), &mut buf);

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

    #[test]
    fn test_parse_heading() {
        let nodes = parse_markdown("# Heading 1");
        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0],
            MarkdownNode::Heading {
                level: 1,
                text: "Heading 1".to_string()
            }
        );

        let nodes = parse_markdown("## Heading 2");
        assert_eq!(
            nodes[0],
            MarkdownNode::Heading {
                level: 2,
                text: "Heading 2".to_string()
            }
        );

        let nodes = parse_markdown("###### Heading 6");
        assert_eq!(
            nodes[0],
            MarkdownNode::Heading {
                level: 6,
                text: "Heading 6".to_string()
            }
        );
    }

    #[test]
    fn test_parse_paragraph() {
        let nodes = parse_markdown("Hello world");
        assert_eq!(nodes.len(), 1);

        if let MarkdownNode::Paragraph(inlines) = &nodes[0] {
            assert_eq!(inlines.len(), 1);
            assert_eq!(inlines[0], Inline::Text("Hello world".to_string()));
        } else {
            panic!("Expected Paragraph");
        }
    }

    #[test]
    fn test_parse_bold() {
        let inlines = parse_inline("**bold text**");
        assert_eq!(inlines.len(), 1);
        assert_eq!(inlines[0], Inline::Bold("bold text".to_string()));

        let inlines = parse_inline("__bold text__");
        assert_eq!(inlines.len(), 1);
        assert_eq!(inlines[0], Inline::Bold("bold text".to_string()));
    }

    #[test]
    fn test_parse_italic() {
        let inlines = parse_inline("*italic text*");
        assert_eq!(inlines.len(), 1);
        assert_eq!(inlines[0], Inline::Italic("italic text".to_string()));

        let inlines = parse_inline("_italic text_");
        assert_eq!(inlines.len(), 1);
        assert_eq!(inlines[0], Inline::Italic("italic text".to_string()));
    }

    #[test]
    fn test_parse_code() {
        let inlines = parse_inline("`code`");
        assert_eq!(inlines.len(), 1);
        assert_eq!(inlines[0], Inline::Code("code".to_string()));
    }

    #[test]
    fn test_parse_link() {
        let inlines = parse_inline("[text](url)");
        assert_eq!(inlines.len(), 1);
        assert_eq!(
            inlines[0],
            Inline::Link {
                text: "text".to_string(),
                url: "url".to_string()
            }
        );
    }

    #[test]
    fn test_parse_mixed_inline() {
        let inlines = parse_inline("Hello **world** and `code`");
        assert_eq!(inlines.len(), 4);
        assert_eq!(inlines[0], Inline::Text("Hello ".to_string()));
        assert_eq!(inlines[1], Inline::Bold("world".to_string()));
        assert_eq!(inlines[2], Inline::Text(" and ".to_string()));
        assert_eq!(inlines[3], Inline::Code("code".to_string()));
    }

    #[test]
    fn test_parse_blockquote() {
        let nodes = parse_markdown("> This is a quote");
        assert_eq!(nodes.len(), 1);

        if let MarkdownNode::Blockquote(inlines) = &nodes[0] {
            assert_eq!(inlines.len(), 1);
            assert_eq!(inlines[0], Inline::Text("This is a quote".to_string()));
        } else {
            panic!("Expected Blockquote");
        }
    }

    #[test]
    fn test_parse_unordered_list() {
        let nodes = parse_markdown("- Item 1\n- Item 2");
        assert_eq!(nodes.len(), 1);

        if let MarkdownNode::List { ordered, items } = &nodes[0] {
            assert!(!ordered);
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected List");
        }
    }

    #[test]
    fn test_parse_ordered_list() {
        let nodes = parse_markdown("1. First\n2. Second");
        assert_eq!(nodes.len(), 1);

        if let MarkdownNode::List { ordered, items } = &nodes[0] {
            assert!(ordered);
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected List");
        }
    }

    #[test]
    fn test_parse_code_block() {
        let nodes = parse_markdown("```rust\nfn main() {}\n```");
        assert_eq!(nodes.len(), 1);

        if let MarkdownNode::CodeBlock { language, code } = &nodes[0] {
            assert_eq!(language, &Some("rust".to_string()));
            assert_eq!(code, "fn main() {}");
        } else {
            panic!("Expected CodeBlock");
        }
    }

    #[test]
    fn test_parse_code_block_no_language() {
        let nodes = parse_markdown("```\nplain code\n```");
        assert_eq!(nodes.len(), 1);

        if let MarkdownNode::CodeBlock { language, code } = &nodes[0] {
            assert_eq!(language, &None);
            assert_eq!(code, "plain code");
        } else {
            panic!("Expected CodeBlock");
        }
    }

    #[test]
    fn test_parse_horizontal_rule() {
        let nodes = parse_markdown("---");
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0], MarkdownNode::Hr);

        let nodes = parse_markdown("***");
        assert_eq!(nodes[0], MarkdownNode::Hr);

        let nodes = parse_markdown("___");
        assert_eq!(nodes[0], MarkdownNode::Hr);
    }

    #[test]
    fn test_parse_complex_document() {
        let doc = r#"# Main Title

This is a paragraph with **bold** and *italic*.

## Section

- List item 1
- List item 2

> A blockquote

```
code here
```
"#;
        let nodes = parse_markdown(doc);
        assert!(nodes.len() >= 5);

        assert!(matches!(&nodes[0], MarkdownNode::Heading { level: 1, .. }));
    }

    #[test]
    fn test_render_heading() {
        let md = Markdown::new("# Heading");
        let result = render_to_string(&md, 20, 3);

        assert!(result.contains("# Heading"));
    }

    #[test]
    fn test_render_paragraph() {
        let md = Markdown::new("Hello world");
        let result = render_to_string(&md, 20, 2);

        assert!(result.contains("Hello world"));
    }

    #[test]
    fn test_render_bold() {
        let md = Markdown::new("**bold**");
        let result = render_to_string(&md, 10, 2);

        assert!(result.contains("bold"));
    }

    #[test]
    fn test_render_code_inline() {
        let md = Markdown::new("`code`");
        let result = render_to_string(&md, 10, 2);

        assert!(result.contains("code"));
    }

    #[test]
    fn test_render_list() {
        let md = Markdown::new("- One\n- Two");
        let result = render_to_string(&md, 20, 5);

        assert!(result.contains("One") || result.contains("\u{2022}"));
    }

    #[test]
    fn test_render_blockquote() {
        let md = Markdown::new("> Quote");
        let result = render_to_string(&md, 20, 2);

        assert!(result.contains(">"));
        assert!(result.contains("Quote"));
    }

    #[test]
    fn test_render_code_block() {
        let md = Markdown::new("```\nhello\n```");
        let result = render_to_string(&md, 20, 5);

        assert!(result.contains("hello"));
        assert!(result.contains("```"));
    }

    #[test]
    fn test_theme_customization() {
        let theme = MarkdownTheme::new()
            .heading_color(Color::Red)
            .link_color(Color::Green)
            .list_bullet('>');

        assert_eq!(theme.heading_color, Color::Red);
        assert_eq!(theme.link_color, Color::Green);
        assert_eq!(theme.list_bullet, '>');
    }

    #[test]
    fn test_default_theme() {
        let theme = MarkdownTheme::default();

        assert_eq!(theme.heading_color, Color::Cyan);
        assert_eq!(theme.link_color, Color::Blue);
        assert_eq!(theme.list_bullet, '•');
    }

    #[test]
    fn test_empty_content() {
        let md = Markdown::new("");
        let nodes = md.nodes();
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let md = Markdown::new("   \n   \n");
        let nodes = md.nodes();
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_no_wrap() {
        let md = Markdown::new(
            "This is a very long line that would normally wrap but wrapping is disabled",
        )
        .wrap(false);
        let result = render_to_string(&md, 20, 3);

        assert!(!result.contains("\nThis"));
    }

    #[test]
    fn test_nested_formatting() {
        let md = Markdown::new("**bold *italic***");
        let _ = render_to_string(&md, 30, 3);
    }

    #[test]
    fn test_unclosed_formatting() {
        let md = Markdown::new("**unclosed bold");
        let _ = render_to_string(&md, 20, 2);

        let md = Markdown::new("*unclosed italic");
        let _ = render_to_string(&md, 20, 2);
    }

    #[test]
    fn test_widget_render_to_buffer() {
        let md = Markdown::new("# Test");
        let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
        md.render(Rect::new(0, 0, 15, 3), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().symbol, "#");
        assert_eq!(buf.get(1, 0).unwrap().symbol, " ");
        assert_eq!(buf.get(2, 0).unwrap().symbol, "T");
    }

    #[test]
    fn test_widget_empty_area() {
        let md = Markdown::new("Test");
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 2));

        md.render(Rect::new(0, 0, 0, 0), &mut buf);
        md.render(Rect::new(0, 0, 10, 0), &mut buf);
    }
}
