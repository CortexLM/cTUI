//! Code component for syntax highlighting in terminal UIs.
//!
//! This module provides a `Code` component that renders source code with
//! syntax highlighting using a simple pattern-based tokenizer. Supports
//! multiple languages and customizable themes.
//!
//! # Features
//!
//! - Syntax highlighting for multiple languages
//! - Line numbers with gutter
//! - Current line highlight
//! - Diff markers (+/-)
//! - Built-in themes (One Dark, Monokai, Solarized, GitHub)
//!
//! # Example
//!
//! ```rust
//! use ctui_components::code::{Code, CodeTheme, Language};
//!
//! let code = Code::new("fn main() { println!(\"Hello\"); }", Language::Rust)
//!     .theme(CodeTheme::one_dark())
//!     .line_numbers(true);
//! ```

use ctui_core::style::{Color, Modifier, Style};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};
use unicode_width::UnicodeWidthStr;

/// The kind of token for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Language keywords (fn, let, if, etc.)
    Keyword,
    /// String literals ("...", '...')
    String,
    /// Numeric literals (123, 0xFF, 1.0)
    Number,
    /// Comments (//, /* */, #)
    Comment,
    /// Function names
    Function,
    /// Type names
    Type,
    /// Operators (+, -, *, /, etc.)
    Operator,
    /// Punctuation (brackets, commas, etc.)
    Punctuation,
    /// Identifiers (variable names, etc.)
    Identifier,
    /// Whitespace
    Whitespace,
    /// Diff-added lines (+)
    DiffAdd,
    /// Diff-removed lines (-)
    DiffRemove,
}

impl Default for TokenKind {
    fn default() -> Self {
        Self::Identifier
    }
}

/// A token from source code tokenization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// The text content of the token
    pub text: String,
    /// The span (start, end) in the source
    pub span: (usize, usize),
}

impl Token {
    /// Creates a new token.
    pub fn new(kind: TokenKind, text: impl Into<String>, span: (usize, usize)) -> Self {
        Self {
            kind,
            text: text.into(),
            span,
        }
    }
}

/// Supported programming languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    /// Rust
    Rust,
    /// TypeScript
    TypeScript,
    /// JavaScript
    JavaScript,
    /// Python
    Python,
    /// JSON
    Json,
    /// Markdown
    Markdown,
    /// Unknown/unsupported language
    #[default]
    Unknown,
}

impl Language {
    /// Detects the language from a file extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Language::Rust,
            "ts" | "tsx" => Language::TypeScript,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "py" => Language::Python,
            "json" => Language::Json,
            "md" | "markdown" => Language::Markdown,
            _ => Language::Unknown,
        }
    }

    /// Returns the file extension for this language.
    pub fn extension(&self) -> &str {
        match self {
            Language::Rust => "rs",
            Language::TypeScript => "ts",
            Language::JavaScript => "js",
            Language::Python => "py",
            Language::Json => "json",
            Language::Markdown => "md",
            Language::Unknown => "txt",
        }
    }
}

/// Color theme for code syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeTheme {
    /// Keyword color
    pub keyword: Color,
    /// String literal color
    pub string: Color,
    /// Numeric literal color
    pub number: Color,
    /// Comment color
    pub comment: Color,
    /// Function name color
    pub function: Color,
    /// Type name color
    pub r#type: Color,
    /// Operator color
    pub operator: Color,
    /// Punctuation color
    pub punctuation: Color,
    /// Background color (optional)
    pub background: Option<Color>,
    /// Gutter background color
    pub gutter_bg: Option<Color>,
    /// Gutter text color
    pub gutter_fg: Color,
    /// Line highlight background
    pub line_highlight_bg: Option<Color>,
    /// Diff add background
    pub diff_add_bg: Option<Color>,
    /// Diff remove background
    pub diff_remove_bg: Option<Color>,
}

impl CodeTheme {
    /// Creates a new custom theme.
    pub fn new() -> Self {
        Self::default()
    }

    /// One Dark theme (default).
    pub fn one_dark() -> Self {
        Self {
            keyword: Color::Rgb(198, 120, 221),
            string: Color::Rgb(152, 195, 121),
            number: Color::Rgb(209, 154, 102),
            comment: Color::Rgb(92, 99, 112),
            function: Color::Rgb(97, 175, 239),
            r#type: Color::Rgb(229, 192, 123),
            operator: Color::Rgb(86, 182, 194),
            punctuation: Color::Rgb(171, 178, 191),
            background: Some(Color::Rgb(40, 44, 52)),
            gutter_bg: Some(Color::Rgb(33, 37, 43)),
            gutter_fg: Color::Rgb(92, 99, 112),
            line_highlight_bg: Some(Color::Rgb(55, 60, 70)),
            diff_add_bg: Some(Color::Rgb(40, 60, 40)),
            diff_remove_bg: Some(Color::Rgb(60, 40, 40)),
        }
    }

    /// Monokai theme.
    pub fn monokai() -> Self {
        Self {
            keyword: Color::Rgb(249, 38, 114),
            string: Color::Rgb(230, 219, 116),
            number: Color::Rgb(174, 129, 255),
            comment: Color::Rgb(117, 113, 94),
            function: Color::Rgb(166, 226, 46),
            r#type: Color::Rgb(102, 217, 239),
            operator: Color::Rgb(249, 38, 114),
            punctuation: Color::Rgb(248, 248, 242),
            background: Some(Color::Rgb(39, 40, 34)),
            gutter_bg: Some(Color::Rgb(32, 33, 28)),
            gutter_fg: Color::Rgb(117, 113, 94),
            line_highlight_bg: Some(Color::Rgb(55, 56, 50)),
            diff_add_bg: Some(Color::Rgb(50, 60, 40)),
            diff_remove_bg: Some(Color::Rgb(60, 40, 50)),
        }
    }

    /// Solarized Dark theme.
    pub fn solarized() -> Self {
        Self {
            keyword: Color::Rgb(203, 75, 91),
            string: Color::Rgb(42, 161, 152),
            number: Color::Rgb(211, 54, 130),
            comment: Color::Rgb(88, 110, 117),
            function: Color::Rgb(38, 139, 210),
            r#type: Color::Rgb(181, 137, 0),
            operator: Color::Rgb(133, 153, 0),
            punctuation: Color::Rgb(253, 246, 227),
            background: Some(Color::Rgb(7, 54, 66)),
            gutter_bg: Some(Color::Rgb(0, 43, 54)),
            gutter_fg: Color::Rgb(88, 110, 117),
            line_highlight_bg: Some(Color::Rgb(14, 64, 77)),
            diff_add_bg: Some(Color::Rgb(14, 64, 54)),
            diff_remove_bg: Some(Color::Rgb(64, 24, 34)),
        }
    }

    /// GitHub theme (light).
    pub fn github() -> Self {
        Self {
            keyword: Color::Rgb(215, 58, 150),
            string: Color::Rgb(3, 121, 67),
            number: Color::Rgb(2, 107, 170),
            comment: Color::Rgb(106, 115, 125),
            function: Color::Rgb(111, 66, 193),
            r#type: Color::Rgb(2, 107, 170),
            operator: Color::Rgb(215, 58, 150),
            punctuation: Color::Rgb(36, 41, 46),
            background: Some(Color::Rgb(255, 255, 255)),
            gutter_bg: Some(Color::Rgb(246, 248, 250)),
            gutter_fg: Color::Rgb(106, 115, 125),
            line_highlight_bg: Some(Color::Rgb(255, 248, 220)),
            diff_add_bg: Some(Color::Rgb(228, 255, 228)),
            diff_remove_bg: Some(Color::Rgb(255, 228, 228)),
        }
    }

    /// Sets the keyword color.
    pub fn keyword(mut self, color: Color) -> Self {
        self.keyword = color;
        self
    }

    /// Sets the string color.
    pub fn string(mut self, color: Color) -> Self {
        self.string = color;
        self
    }

    /// Sets the number color.
    pub fn number(mut self, color: Color) -> Self {
        self.number = color;
        self
    }

    /// Sets the comment color.
    pub fn comment(mut self, color: Color) -> Self {
        self.comment = color;
        self
    }

    /// Sets the function color.
    pub fn function(mut self, color: Color) -> Self {
        self.function = color;
        self
    }

    /// Sets the type color.
    pub fn r#type(mut self, color: Color) -> Self {
        self.r#type = color;
        self
    }

    /// Sets the operator color.
    pub fn operator(mut self, color: Color) -> Self {
        self.operator = color;
        self
    }

    /// Sets the punctuation color.
    pub fn punctuation(mut self, color: Color) -> Self {
        self.punctuation = color;
        self
    }

    /// Sets the background color.
    pub fn background(mut self, color: Option<Color>) -> Self {
        self.background = color;
        self
    }

    /// Returns the color for a token kind.
    pub fn color_for(&self, kind: TokenKind) -> Color {
        match kind {
            TokenKind::Keyword => self.keyword,
            TokenKind::String => self.string,
            TokenKind::Number => self.number,
            TokenKind::Comment => self.comment,
            TokenKind::Function => self.function,
            TokenKind::Type => self.r#type,
            TokenKind::Operator => self.operator,
            TokenKind::Punctuation => self.punctuation,
            TokenKind::Identifier => Color::Reset,
            TokenKind::Whitespace => Color::Reset,
            TokenKind::DiffAdd => self.string,
            TokenKind::DiffRemove => self.keyword,
        }
    }
}

impl Default for CodeTheme {
    fn default() -> Self {
        Self::one_dark()
    }
}

/// Diff marker for lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffMarker {
    /// Added line (+)
    Add,
    /// Removed line (-)
    Remove,
    /// No diff marker
    None,
}

impl Default for DiffMarker {
    fn default() -> Self {
        Self::None
    }
}

/// A code component for rendering syntax-highlighted source code.
#[derive(Debug, Clone)]
pub struct Code {
    content: String,
    language: Language,
    theme: CodeTheme,
    line_numbers: bool,
    highlight_line: Option<usize>,
    scroll: u16,
    style: Style,
    diff_markers: Vec<DiffMarker>,
    copy_button: bool,
}

impl Code {
    /// Creates a new code component.
    pub fn new(content: impl Into<String>, language: Language) -> Self {
        Self {
            content: content.into(),
            language,
            theme: CodeTheme::default(),
            line_numbers: false,
            highlight_line: None,
            scroll: 0,
            style: Style::default(),
            diff_markers: Vec::new(),
            copy_button: false,
        }
    }

    /// Sets the color theme.
    pub fn theme(mut self, theme: CodeTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Enables or disables line numbers.
    pub fn line_numbers(mut self, enable: bool) -> Self {
        self.line_numbers = enable;
        self
    }

    /// Sets the line to highlight (1-indexed).
    pub fn highlight_line(mut self, line: Option<usize>) -> Self {
        self.highlight_line = line;
        self
    }

    /// Sets the scroll offset (in lines).
    pub fn scroll(mut self, lines: u16) -> Self {
        self.scroll = lines;
        self
    }

    /// Sets the base style.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Enables the copy button.
    pub fn copy_button(mut self, enable: bool) -> Self {
        self.copy_button = enable;
        self
    }

    /// Sets diff markers for each line.
    pub fn diff_markers(mut self, markers: Vec<DiffMarker>) -> Self {
        self.diff_markers = markers;
        self
    }

    /// Returns the content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Returns the language.
    pub fn language(&self) -> Language {
        self.language
    }

    /// Returns the current scroll offset.
    pub fn scroll_offset(&self) -> u16 {
        self.scroll
    }

    /// Scrolls by the given number of lines.
    pub fn scroll_by(&mut self, lines: i16) {
        if lines >= 0 {
            self.scroll = self.scroll.saturating_add(lines as u16);
        } else {
            self.scroll = self.scroll.saturating_sub((-lines) as u16);
        }
    }

    fn lines(&self) -> Vec<&str> {
        self.content.lines().collect()
    }

    fn tokenize_line(&self, line: &str, line_idx: usize) -> Vec<Token> {
        let diff_marker = self
            .diff_markers
            .get(line_idx)
            .copied()
            .unwrap_or(DiffMarker::None);

        if diff_marker != DiffMarker::None {
            return self.tokenize_diff_line(line, diff_marker);
        }

        match self.language {
            Language::Rust => self.tokenize_rust(line),
            Language::TypeScript | Language::JavaScript => self.tokenize_typescript(line),
            Language::Python => self.tokenize_python(line),
            Language::Json => self.tokenize_json(line),
            Language::Markdown => self.tokenize_markdown(line),
            Language::Unknown => self.tokenize_plain(line),
        }
    }

    fn tokenize_diff_line(&self, line: &str, marker: DiffMarker) -> Vec<Token> {
        let kind = match marker {
            DiffMarker::Add => TokenKind::DiffAdd,
            DiffMarker::Remove => TokenKind::DiffRemove,
            DiffMarker::None => TokenKind::Identifier,
        };

        let prefix = match marker {
            DiffMarker::Add => "+",
            DiffMarker::Remove => "-",
            DiffMarker::None => " ",
        };

        vec![
            Token::new(kind, prefix, (0, 1)),
            Token::new(
                TokenKind::String,
                &line[1.min(line.len())..],
                (1, line.len()),
            ),
        ]
    }

    fn tokenize_rust(&self, line: &str) -> Vec<Token> {
        self.tokenize_with_keywords(line, &RUST_KEYWORDS, &RUST_TYPES)
    }

    fn tokenize_typescript(&self, line: &str) -> Vec<Token> {
        self.tokenize_with_keywords(line, &TS_KEYWORDS, &TS_TYPES)
    }

    fn tokenize_python(&self, line: &str) -> Vec<Token> {
        self.tokenize_with_keywords(line, &PYTHON_KEYWORDS, &PYTHON_TYPES)
    }

    fn tokenize_json(&self, line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut pos = 0;
        let chars: Vec<char> = line.chars().collect();

        while pos < chars.len() {
            let ch = chars[pos];

            if ch == '"' {
                let start = pos;
                pos += 1;
                while pos < chars.len() {
                    if chars[pos] == '\\' && pos + 1 < chars.len() {
                        pos += 2;
                        continue;
                    }
                    if chars[pos] == '"' {
                        pos += 1;
                        break;
                    }
                    pos += 1;
                }
                tokens.push(Token::new(
                    TokenKind::String,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            if ch.is_ascii_digit() {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_ascii_digit() || chars[pos] == '.') {
                    pos += 1;
                }
                tokens.push(Token::new(
                    TokenKind::Number,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            if ch == ':' || ch == ',' {
                tokens.push(Token::new(
                    TokenKind::Punctuation,
                    ch.to_string(),
                    (pos, pos + 1),
                ));
                pos += 1;
                continue;
            }

            if ch.is_whitespace() {
                let start = pos;
                while pos < chars.len() && chars[pos].is_whitespace() {
                    pos += 1;
                }
                tokens.push(Token::new(
                    TokenKind::Whitespace,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            if ch.is_alphabetic() || ch == '_' {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                    pos += 1;
                }
                let text = &line[start..pos];
                let kind = if tokens
                    .last()
                    .map_or(true, |t| t.kind != TokenKind::Punctuation || t.text != ":")
                {
                    TokenKind::Function
                } else {
                    TokenKind::Identifier
                };
                tokens.push(Token::new(kind, text.to_string(), (start, pos)));
                continue;
            }

            if ch == '{' || ch == '}' || ch == '[' || ch == ']' {
                tokens.push(Token::new(
                    TokenKind::Punctuation,
                    ch.to_string(),
                    (pos, pos + 1),
                ));
                pos += 1;
                continue;
            }

            tokens.push(Token::new(
                TokenKind::Identifier,
                ch.to_string(),
                (pos, pos + 1),
            ));
            pos += 1;
        }

        tokens
    }

    fn tokenize_markdown(&self, line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = line.chars().collect();

        if line.starts_with("### ") {
            tokens.push(Token::new(TokenKind::Keyword, "### ", (0, 4)));
            if line.len() > 4 {
                tokens.push(Token::new(
                    TokenKind::String,
                    line[4..].to_string(),
                    (4, line.len()),
                ));
            }
            return tokens;
        }
        if line.starts_with("## ") {
            tokens.push(Token::new(TokenKind::Keyword, "## ", (0, 3)));
            if line.len() > 3 {
                tokens.push(Token::new(
                    TokenKind::String,
                    line[3..].to_string(),
                    (3, line.len()),
                ));
            }
            return tokens;
        }
        if line.starts_with("# ") {
            tokens.push(Token::new(TokenKind::Keyword, "# ", (0, 2)));
            if line.len() > 2 {
                tokens.push(Token::new(
                    TokenKind::String,
                    line[2..].to_string(),
                    (2, line.len()),
                ));
            }
            return tokens;
        }

        if line.starts_with("```") {
            tokens.push(Token::new(
                TokenKind::Comment,
                line.to_string(),
                (0, line.len()),
            ));
            return tokens;
        }

        let mut pos = 0;
        while pos < chars.len() {
            if chars[pos] == '`' {
                let start = pos;
                pos += 1;
                while pos < chars.len() && chars[pos] != '`' {
                    pos += 1;
                }
                if pos < chars.len() {
                    pos += 1;
                }
                tokens.push(Token::new(
                    TokenKind::String,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            if chars[pos] == '*' || chars[pos] == '_' {
                let start = pos;
                let marker = chars[pos];
                let count = if pos + 1 < chars.len() && chars[pos + 1] == marker {
                    if pos + 2 < chars.len() && chars[pos + 2] == marker {
                        3
                    } else {
                        2
                    }
                } else {
                    1
                };
                pos += count;
                tokens.push(Token::new(
                    TokenKind::Operator,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            if chars[pos] == '[' {
                let start = pos;
                while pos < chars.len() && chars[pos] != ']' {
                    pos += 1;
                }
                if pos < chars.len() && chars[pos] == ']' {
                    pos += 1;
                    if pos < chars.len() && chars[pos] == '(' {
                        while pos < chars.len() && chars[pos] != ')' {
                            pos += 1;
                        }
                        if pos < chars.len() {
                            pos += 1;
                        }
                    }
                }
                tokens.push(Token::new(
                    TokenKind::Function,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            tokens.push(Token::new(
                TokenKind::Identifier,
                chars[pos].to_string(),
                (pos, pos + 1),
            ));
            pos += 1;
        }

        tokens
    }

    fn tokenize_plain(&self, line: &str) -> Vec<Token> {
        vec![Token::new(
            TokenKind::Identifier,
            line.to_string(),
            (0, line.len()),
        )]
    }

    fn tokenize_with_keywords(&self, line: &str, keywords: &[&str], types: &[&str]) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut pos = 0;
        let chars: Vec<char> = line.chars().collect();

        while pos < chars.len() {
            let ch = chars[pos];

            if ch == '/' && pos + 1 < chars.len() {
                if chars[pos + 1] == '/' {
                    let start = pos;
                    pos = chars.len();
                    tokens.push(Token::new(
                        TokenKind::Comment,
                        line[start..].to_string(),
                        (start, pos),
                    ));
                    continue;
                } else if chars[pos + 1] == '*' {
                    let start = pos;
                    pos += 2;
                    while pos + 1 < chars.len() && !(chars[pos] == '*' && chars[pos + 1] == '/') {
                        pos += 1;
                    }
                    if pos + 1 < chars.len() {
                        pos += 2;
                    }
                    tokens.push(Token::new(
                        TokenKind::Comment,
                        line[start..pos].to_string(),
                        (start, pos),
                    ));
                    continue;
                }
            }

            if self.language == Language::Python && ch == '#' {
                let start = pos;
                pos = chars.len();
                tokens.push(Token::new(
                    TokenKind::Comment,
                    line[start..].to_string(),
                    (start, pos),
                ));
                continue;
            }

            if ch == '"' || ch == '\'' {
                let quote = ch;
                let start = pos;
                pos += 1;
                while pos < chars.len() {
                    if chars[pos] == '\\' && pos + 1 < chars.len() {
                        pos += 2;
                        continue;
                    }
                    if chars[pos] == quote {
                        pos += 1;
                        break;
                    }
                    pos += 1;
                }
                tokens.push(Token::new(
                    TokenKind::String,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            if ch == '`' {
                let start = pos;
                pos += 1;
                while pos < chars.len() {
                    if chars[pos] == '\\' && pos + 1 < chars.len() {
                        pos += 2;
                        continue;
                    }
                    if chars[pos] == '`' {
                        pos += 1;
                        break;
                    }
                    pos += 1;
                }
                tokens.push(Token::new(
                    TokenKind::String,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            if ch.is_ascii_digit()
                || (ch == '-' && pos + 1 < chars.len() && chars[pos + 1].is_ascii_digit())
            {
                let start = pos;
                if ch == '-' {
                    pos += 1;
                }
                if pos + 1 < chars.len()
                    && chars[pos] == '0'
                    && (chars[pos + 1] == 'x' || chars[pos + 1] == 'X')
                {
                    pos += 2;
                    while pos < chars.len() && (chars[pos].is_ascii_hexdigit()) {
                        pos += 1;
                    }
                } else {
                    while pos < chars.len() && (chars[pos].is_ascii_digit()) {
                        pos += 1;
                    }
                    if pos < chars.len() && chars[pos] == '.' {
                        pos += 1;
                        while pos < chars.len() && chars[pos].is_ascii_digit() {
                            pos += 1;
                        }
                    }
                    if pos < chars.len() && (chars[pos] == 'e' || chars[pos] == 'E') {
                        pos += 1;
                        if pos < chars.len() && (chars[pos] == '+' || chars[pos] == '-') {
                            pos += 1;
                        }
                        while pos < chars.len() && chars[pos].is_ascii_digit() {
                            pos += 1;
                        }
                    }
                }
                tokens.push(Token::new(
                    TokenKind::Number,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            if ch.is_alphabetic() || ch == '_' {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                    pos += 1;
                }
                let text = &line[start..pos];

                // Check for function call: either `(` directly or `!(` for macros
                let is_function = (pos < chars.len() && chars[pos] == '(')
                    || (pos + 1 < chars.len() && chars[pos] == '!' && chars[pos + 1] == '(');

                let kind = if keywords.contains(&text) {
                    TokenKind::Keyword
                } else if types.contains(&text) {
                    TokenKind::Type
                } else if is_function {
                    TokenKind::Function
                } else {
                    TokenKind::Identifier
                };

                tokens.push(Token::new(kind, text.to_string(), (start, pos)));
                continue;
            }

            if is_operator_char(ch) {
                let start = pos;
                while pos < chars.len() && is_operator_char(chars[pos]) {
                    pos += 1;
                }
                let op_text = &line[start..pos];
                for (i, op_char) in op_text.chars().enumerate() {
                    tokens.push(Token::new(
                        TokenKind::Operator,
                        op_char.to_string(),
                        (start + i, start + i + 1),
                    ));
                }
                continue;
            }

            if is_punctuation_char(ch) {
                tokens.push(Token::new(
                    TokenKind::Punctuation,
                    ch.to_string(),
                    (pos, pos + 1),
                ));
                pos += 1;
                continue;
            }

            if ch.is_whitespace() {
                let start = pos;
                while pos < chars.len() && chars[pos].is_whitespace() {
                    pos += 1;
                }
                tokens.push(Token::new(
                    TokenKind::Whitespace,
                    line[start..pos].to_string(),
                    (start, pos),
                ));
                continue;
            }

            tokens.push(Token::new(
                TokenKind::Identifier,
                ch.to_string(),
                (pos, pos + 1),
            ));
            pos += 1;
        }

        tokens
    }

    fn gutter_width(&self, line_count: usize) -> usize {
        if !self.line_numbers {
            return 0;
        }
        let digits = line_count.to_string().len();
        digits + 1
    }
}

impl Default for Code {
    fn default() -> Self {
        Self::new("", Language::Unknown)
    }
}

/// Props for creating a Code component.
#[derive(Debug, Clone)]
pub struct CodeProps {
    pub content: String,
    pub language: Language,
    pub theme: CodeTheme,
    pub line_numbers: bool,
    pub highlight_line: Option<usize>,
    pub style: Style,
}

impl CodeProps {
    pub fn new(content: impl Into<String>, language: Language) -> Self {
        Self {
            content: content.into(),
            language,
            theme: CodeTheme::default(),
            line_numbers: false,
            highlight_line: None,
            style: Style::default(),
        }
    }

    pub fn theme(mut self, theme: CodeTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn line_numbers(mut self, enable: bool) -> Self {
        self.line_numbers = enable;
        self
    }

    pub fn highlight_line(mut self, line: Option<usize>) -> Self {
        self.highlight_line = line;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Component for Code {
    type Props = CodeProps;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self {
            content: props.content,
            language: props.language,
            theme: props.theme,
            line_numbers: props.line_numbers,
            highlight_line: props.highlight_line,
            scroll: 0,
            style: props.style,
            diff_markers: Vec::new(),
            copy_button: false,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.is_zero() {
            return;
        }

        let lines = self.lines();
        let gutter_width = self.gutter_width(lines.len());
        let code_area = Rect::new(
            area.x + gutter_width as u16,
            area.y,
            area.width.saturating_sub(gutter_width as u16),
            area.height,
        );

        if let Some(bg) = self.theme.background {
            for y in area.y..(area.y + area.height) {
                for x in area.x..(area.x + area.width) {
                    if let Some(cell) = buf.get_mut(x, y) {
                        cell.set_style(Style::new().bg(bg));
                    }
                }
            }
        }

        if self.line_numbers {
            self.render_gutter(area, buf, lines.len());
        }

        let scroll = self.scroll as usize;
        let start_line = scroll.min(lines.len());

        for (y_offset, (line_idx, line)) in lines
            .iter()
            .enumerate()
            .skip(start_line)
            .take(area.height as usize)
            .enumerate()
        {
            let y = code_area.y + y_offset as u16;
            if y >= code_area.y + code_area.height {
                break;
            }

            if let Some(hl) = self.highlight_line {
                if line_idx + 1 == hl {
                    if let Some(hl_bg) = self.theme.line_highlight_bg {
                        for x in code_area.x..(code_area.x + code_area.width) {
                            if let Some(cell) = buf.get_mut(x, y) {
                                cell.set_style(Style::new().bg(hl_bg));
                            }
                        }
                    }
                }
            }

            let diff_marker = self
                .diff_markers
                .get(line_idx)
                .copied()
                .unwrap_or(DiffMarker::None);
            if diff_marker != DiffMarker::None {
                let bg = match diff_marker {
                    DiffMarker::Add => self.theme.diff_add_bg,
                    DiffMarker::Remove => self.theme.diff_remove_bg,
                    DiffMarker::None => None,
                };
                if let Some(bg) = bg {
                    for x in code_area.x..(code_area.x + code_area.width) {
                        if let Some(cell) = buf.get_mut(x, y) {
                            cell.set_style(Style::new().bg(bg));
                        }
                    }
                }
            }

            let tokens = self.tokenize_line(line, line_idx);
            let mut term_x = code_area.x;

            for token in tokens {
                let token_style = Style::new()
                    .fg(self.theme.color_for(token.kind))
                    .bg(self.theme.background.unwrap_or(Color::Reset));

                for ch in token.text.chars() {
                    if term_x >= code_area.x + code_area.width {
                        break;
                    }

                    let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());

                    if let Some(cell) = buf.get_mut(term_x, y) {
                        cell.symbol = ch.to_string();

                        let final_style = if diff_marker != DiffMarker::None {
                            let bg = match diff_marker {
                                DiffMarker::Add => self.theme.diff_add_bg,
                                DiffMarker::Remove => self.theme.diff_remove_bg,
                                DiffMarker::None => self.theme.background,
                            };
                            Style {
                                fg: token_style.fg,
                                bg: bg.unwrap_or(Color::Reset),
                                modifier: token_style.modifier,
                            }
                        } else if self.highlight_line == Some(line_idx + 1) {
                            Style {
                                fg: token_style.fg,
                                bg: self.theme.line_highlight_bg.unwrap_or(Color::Reset),
                                modifier: token_style.modifier,
                            }
                        } else {
                            token_style
                        };

                        cell.set_style(final_style);
                    }

                    if ch_width > 1 {
                        for i in 1..ch_width {
                            let next_x = term_x + i as u16;
                            if next_x < code_area.x + code_area.width {
                                if let Some(cell) = buf.get_mut(next_x, y) {
                                    cell.skip = true;
                                }
                            }
                        }
                    }

                    term_x += ch_width as u16;
                }
            }
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

impl Code {
    fn render_gutter(&self, area: Rect, buf: &mut Buffer, total_lines: usize) {
        let gutter_width = self.gutter_width(total_lines);
        let scroll = self.scroll as usize;

        if let Some(bg) = self.theme.gutter_bg {
            for y in area.y..(area.y + area.height) {
                for x in area.x..(area.x + gutter_width as u16) {
                    if let Some(cell) = buf.get_mut(x, y) {
                        cell.set_style(Style::new().bg(bg).fg(self.theme.gutter_fg));
                    }
                }
            }
        }

        let start_line = scroll.min(total_lines);
        for (y_offset, line_idx) in (start_line..).take(area.height as usize).enumerate() {
            if line_idx >= total_lines {
                break;
            }

            let y = area.y + y_offset as u16;
            let line_num = (line_idx + 1).to_string();

            let num_width = line_num.len();
            let x_offset = gutter_width.saturating_sub(num_width);

            let num_style = if self.highlight_line == Some(line_idx + 1) {
                Style::new()
                    .fg(self.theme.gutter_fg)
                    .bg(self.theme.line_highlight_bg.unwrap_or(Color::Reset))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::new()
                    .fg(self.theme.gutter_fg)
                    .bg(self.theme.gutter_bg.unwrap_or(Color::Reset))
            };

            for (i, ch) in line_num.chars().enumerate() {
                let x = area.x + (x_offset + i) as u16;
                if x < area.x + gutter_width as u16 {
                    if let Some(cell) = buf.get_mut(x, y) {
                        cell.symbol = ch.to_string();
                        cell.set_style(num_style);
                    }
                }
            }

            if let Some(marker) = self.diff_markers.get(line_idx) {
                let marker_char = match marker {
                    DiffMarker::Add => '+',
                    DiffMarker::Remove => '-',
                    DiffMarker::None => ' ',
                };
                let marker_x = area.x + gutter_width as u16;
                if marker_x < area.x + area.width {
                    if let Some(cell) = buf.get_mut(marker_x, y) {
                        cell.symbol = marker_char.to_string();
                        let marker_style = match marker {
                            DiffMarker::Add => Style::new().fg(self.theme.string),
                            DiffMarker::Remove => Style::new().fg(self.theme.keyword),
                            DiffMarker::None => Style::new().fg(self.theme.gutter_fg),
                        };
                        cell.set_style(marker_style);
                    }
                }
            }
        }
    }
}

fn is_operator_char(ch: char) -> bool {
    matches!(
        ch,
        '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '^' | '~' | '?' | ':'
    )
}

fn is_punctuation_char(ch: char) -> bool {
    matches!(
        ch,
        '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' | '@' | '#' | '$'
    )
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield",
];

const RUST_TYPES: &[&str] = &[
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "usize",
    "f32",
    "f64",
    "bool",
    "char",
    "str",
    "String",
    "Vec",
    "Option",
    "Result",
    "Box",
    "Rc",
    "Arc",
    "Cell",
    "RefCell",
    "Mutex",
    "RwLock",
    "HashMap",
    "HashSet",
    "BTreeMap",
    "BTreeSet",
    "Cow",
    "Path",
    "PathBuf",
    "OsStr",
    "OsString",
    "Duration",
    "Instant",
    "SystemTime",
    "Error",
    "Result",
    "Ok",
    "Err",
    "Some",
    "None",
];

const TS_KEYWORDS: &[&str] = &[
    "async",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "let",
    "new",
    "null",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "undefined",
    "var",
    "void",
    "while",
    "with",
    "yield",
    "as",
    "from",
    "of",
    "type",
    "namespace",
    "module",
    "declare",
    "abstract",
    "readonly",
    "static",
    "get",
    "set",
];

const TS_TYPES: &[&str] = &[
    "string",
    "number",
    "boolean",
    "void",
    "any",
    "unknown",
    "never",
    "object",
    "symbol",
    "bigint",
    "null",
    "undefined",
    "Array",
    "Map",
    "Set",
    "WeakMap",
    "WeakSet",
    "Promise",
    "Date",
    "RegExp",
    "Error",
    "Function",
    "Object",
    "String",
    "Number",
    "Boolean",
    "Symbol",
    "BigInt",
    "Partial",
    "Required",
    "Readonly",
    "Record",
    "Pick",
    "Omit",
    "Exclude",
    "Extract",
    "NonNullable",
    "Parameters",
    "ConstructorParameters",
    "ReturnType",
];

const PYTHON_KEYWORDS: &[&str] = &[
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

const PYTHON_TYPES: &[&str] = &[
    "int",
    "float",
    "complex",
    "str",
    "bytes",
    "bytearray",
    "list",
    "tuple",
    "set",
    "frozenset",
    "dict",
    "bool",
    "None",
    "Ellipsis",
    "object",
    "type",
    "Exception",
    "BaseException",
    "ValueError",
    "TypeError",
    "KeyError",
    "IndexError",
    "StopIteration",
    "Generator",
    "Iterator",
    "Iterable",
    "Callable",
    "Any",
    "Union",
    "Optional",
    "List",
    "Dict",
    "Set",
    "Tuple",
    "Sequence",
    "Mapping",
    "MutableMapping",
    "MutableSequence",
    "MutableSet",
];

#[cfg(test)]
mod tests {
    use super::*;

    fn render_to_string(code: &Code, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        code.render(Rect::new(0, 0, width, height), &mut buf);

        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                output.push_str(&buf[(x, y)].symbol);
            }
            if y < height - 1 {
                output.push('\n');
            }
        }
        output
    }

    #[test]
    fn test_token_new() {
        let token = Token::new(TokenKind::Keyword, "fn", (0, 2));
        assert_eq!(token.kind, TokenKind::Keyword);
        assert_eq!(token.text, "fn");
        assert_eq!(token.span, (0, 2));
    }

    #[test]
    fn test_language_from_extension() {
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("ts"), Language::TypeScript);
        assert_eq!(Language::from_extension("js"), Language::JavaScript);
        assert_eq!(Language::from_extension("py"), Language::Python);
        assert_eq!(Language::from_extension("json"), Language::Json);
        assert_eq!(Language::from_extension("md"), Language::Markdown);
        assert_eq!(Language::from_extension("unknown"), Language::Unknown);
    }

    #[test]
    fn test_language_extension() {
        assert_eq!(Language::Rust.extension(), "rs");
        assert_eq!(Language::TypeScript.extension(), "ts");
        assert_eq!(Language::JavaScript.extension(), "js");
        assert_eq!(Language::Python.extension(), "py");
        assert_eq!(Language::Json.extension(), "json");
        assert_eq!(Language::Markdown.extension(), "md");
        assert_eq!(Language::Unknown.extension(), "txt");
    }

    #[test]
    fn test_code_theme_default() {
        let theme = CodeTheme::default();
        assert_eq!(theme, CodeTheme::one_dark());
    }

    #[test]
    fn test_code_theme_color_for() {
        let theme = CodeTheme::one_dark();
        assert_eq!(theme.color_for(TokenKind::Keyword), theme.keyword);
        assert_eq!(theme.color_for(TokenKind::String), theme.string);
        assert_eq!(theme.color_for(TokenKind::Number), theme.number);
        assert_eq!(theme.color_for(TokenKind::Comment), theme.comment);
    }

    #[test]
    fn test_code_new() {
        let code = Code::new("fn main() {}", Language::Rust);
        assert_eq!(code.content(), "fn main() {}");
        assert_eq!(code.language(), Language::Rust);
        assert!(!code.line_numbers);
        assert!(code.highlight_line.is_none());
    }

    #[test]
    fn test_code_with_line_numbers() {
        let code = Code::new("line 1\nline 2", Language::Unknown).line_numbers(true);
        assert!(code.line_numbers);
    }

    #[test]
    fn test_code_with_highlight() {
        let code = Code::new("line 1\nline 2", Language::Unknown).highlight_line(Some(2));
        assert_eq!(code.highlight_line, Some(2));
    }

    #[test]
    fn test_code_scroll() {
        let mut code = Code::new("line 1\nline 2\nline 3", Language::Unknown);
        assert_eq!(code.scroll_offset(), 0);

        code.scroll_by(1);
        assert_eq!(code.scroll_offset(), 1);

        code.scroll_by(-1);
        assert_eq!(code.scroll_offset(), 0);

        code.scroll_by(-5);
        assert_eq!(code.scroll_offset(), 0);
    }

    #[test]
    fn test_render_basic() {
        let code = Code::new("hello", Language::Unknown);
        let result = render_to_string(&code, 10, 1);
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_render_with_line_numbers() {
        let code = Code::new("line 1\nline 2", Language::Unknown).line_numbers(true);
        let result = render_to_string(&code, 15, 2);
        assert!(result.contains("1"));
        assert!(result.contains("2"));
    }

    #[test]
    fn test_rust_tokenizer() {
        let code = Code::new("fn main() { let x = 42; }", Language::Rust);
        let tokens = code.tokenize_rust("fn main() { let x = 42; }");

        assert!(!tokens.is_empty());

        let first = &tokens[0];
        assert_eq!(first.kind, TokenKind::Keyword);
        assert_eq!(first.text, "fn");
    }

    #[test]
    fn test_rust_string_tokenizer() {
        let code = Code::new("", Language::Rust);
        let tokens = code.tokenize_rust(r#"let s = "hello";"#);

        let string_token = tokens.iter().find(|t| t.kind == TokenKind::String);
        assert!(string_token.is_some());
        assert_eq!(string_token.unwrap().text, r#""hello""#);
    }

    #[test]
    fn test_rust_comment_tokenizer() {
        let code = Code::new("", Language::Rust);
        let tokens = code.tokenize_rust("// this is a comment");

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Comment);
        assert!(tokens[0].text.starts_with("//"));
    }

    #[test]
    fn test_python_comment_tokenizer() {
        let code = Code::new("", Language::Python);
        let tokens = code.tokenize_python("# this is a comment");

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Comment);
        assert!(tokens[0].text.starts_with('#'));
    }

    #[test]
    fn test_json_tokenizer() {
        let code = Code::new("", Language::Json);
        let tokens = code.tokenize_json(r#"{"key": "value"}"#);

        assert!(!tokens.is_empty());

        let strings: Vec<_> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::String)
            .collect();
        assert!(!strings.is_empty());
    }

    #[test]
    fn test_typescript_tokenizer() {
        let code = Code::new("", Language::TypeScript);
        let tokens = code.tokenize_typescript("const x: number = 42;");

        assert!(!tokens.is_empty());

        let keywords: Vec<_> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::Keyword)
            .collect();
        assert!(!keywords.is_empty());
    }

    #[test]
    fn test_diff_marker() {
        let code = Code::new("+ added line\n- removed line", Language::Unknown)
            .diff_markers(vec![DiffMarker::Add, DiffMarker::Remove]);

        let tokens_add = code.tokenize_diff_line("+ added line", DiffMarker::Add);
        assert_eq!(tokens_add[0].kind, TokenKind::DiffAdd);

        let tokens_remove = code.tokenize_diff_line("- removed line", DiffMarker::Remove);
        assert_eq!(tokens_remove[0].kind, TokenKind::DiffRemove);
    }

    #[test]
    fn test_markdown_header() {
        let code = Code::new("", Language::Markdown);
        let tokens = code.tokenize_markdown("# Header");

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
    }

    #[test]
    fn test_markdown_code_block() {
        let code = Code::new("", Language::Markdown);
        let tokens = code.tokenize_markdown("```rust");

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Comment);
    }

    #[test]
    fn test_builtin_themes() {
        let one_dark = CodeTheme::one_dark();
        let monokai = CodeTheme::monokai();
        let solarized = CodeTheme::solarized();
        let github = CodeTheme::github();

        assert_ne!(one_dark.keyword, monokai.keyword);
        assert_ne!(monokai.keyword, solarized.keyword);
        assert_ne!(solarized.keyword, github.keyword);
    }

    #[test]
    fn test_render_rust_code() {
        let code = Code::new("fn main() {\n    println!(\"Hello\");\n}", Language::Rust)
            .theme(CodeTheme::one_dark());
        let result = render_to_string(&code, 30, 3);

        assert!(result.contains("fn"));
        assert!(result.contains("main"));
    }

    #[test]
    fn test_render_json() {
        let code = Code::new(r#"{"name": "test", "value": 42}"#, Language::Json)
            .theme(CodeTheme::one_dark());
        let result = render_to_string(&code, 40, 1);

        assert!(result.contains("name") || result.contains("test"));
    }

    #[test]
    fn test_code_props() {
        let props = CodeProps::new("fn main() {}", Language::Rust)
            .line_numbers(true)
            .highlight_line(Some(1));

        let code = Code::create(props);
        assert!(code.line_numbers);
        assert_eq!(code.highlight_line, Some(1));
    }

    #[test]
    fn test_operator_detection() {
        assert!(is_operator_char('+'));
        assert!(is_operator_char('-'));
        assert!(is_operator_char('='));
        assert!(is_operator_char('<'));
        assert!(!is_operator_char('a'));
        assert!(!is_operator_char('('));
    }

    #[test]
    fn test_punctuation_detection() {
        assert!(is_punctuation_char('('));
        assert!(is_punctuation_char(')'));
        assert!(is_punctuation_char('{'));
        assert!(is_punctuation_char('}'));
        assert!(is_punctuation_char(';'));
        assert!(!is_punctuation_char('a'));
        assert!(!is_punctuation_char('+'));
    }

    #[test]
    fn test_number_tokenizer_hex() {
        let code = Code::new("", Language::Rust);
        let tokens = code.tokenize_rust("let x = 0xFF;");

        let numbers: Vec<_> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::Number)
            .collect();
        assert!(!numbers.is_empty());
        assert!(numbers[0].text.contains("0x"));
    }

    #[test]
    fn test_number_tokenizer_float() {
        let code = Code::new("", Language::Rust);
        let tokens = code.tokenize_rust("let x = 3.14;");

        let numbers: Vec<_> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::Number)
            .collect();
        assert!(!numbers.is_empty());
        assert!(numbers[0].text.contains('.'));
    }

    #[test]
    fn test_function_call_detection() {
        let code = Code::new("", Language::Rust);
        let tokens = code.tokenize_rust("println!(\"Hello\");");

        let functions: Vec<_> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::Function)
            .collect();
        assert!(!functions.is_empty());
        assert!(functions[0].text.contains("println"));
    }
}
