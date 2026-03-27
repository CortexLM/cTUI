use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ctui_core::style::Style;
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl Selection {
    pub fn new(start_row: usize, start_col: usize, end_row: usize, end_col: usize) -> Self {
        Self {
            start_row,
            start_col,
            end_row,
            end_col,
        }
    }

    pub fn normalized(&self) -> (usize, usize, usize, usize) {
        if self.start_row < self.end_row
            || (self.start_row == self.end_row && self.start_col <= self.end_col)
        {
            (self.start_row, self.start_col, self.end_row, self.end_col)
        } else {
            (self.end_row, self.end_col, self.start_row, self.start_col)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start_row == self.end_row && self.start_col == self.end_col
    }

    pub fn contains(&self, row: usize, col: usize) -> bool {
        let (start_row, start_col, end_row, end_col) = self.normalized();
        if row < start_row || row > end_row {
            return false;
        }
        if row == start_row && col < start_col {
            return false;
        }
        if row == end_row && col > end_col {
            return false;
        }
        true
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            start_row: 0,
            start_col: 0,
            end_row: 0,
            end_col: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Textarea {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    selection: Option<Selection>,
    style: Style,
    selection_style: Style,
    cursor_style: Style,
    scroll_row: usize,
    scroll_col: usize,
}

impl Default for Textarea {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_row: 0,
            cursor_col: 0,
            selection: None,
            style: Style::default(),
            selection_style: Style::default(),
            cursor_style: Style::default(),
            scroll_row: 0,
            scroll_col: 0,
        }
    }
}

impl Textarea {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(content: &str) -> Self {
        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(String::from).collect()
        };
        let cursor_row = lines.len().saturating_sub(1);
        let cursor_col = lines.last().map(|l| l.chars().count()).unwrap_or(0);
        Self {
            lines,
            cursor_row,
            cursor_col,
            ..Self::default()
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn selection_style(mut self, style: Style) -> Self {
        self.selection_style = style;
        self
    }

    pub fn cursor_style(mut self, style: Style) -> Self {
        self.cursor_style = style;
        self
    }

    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_row, self.cursor_col)
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) {
        self.cursor_row = row.min(self.lines.len().saturating_sub(1));
        let max_col = self
            .lines
            .get(self.cursor_row)
            .map(|l| l.chars().count())
            .unwrap_or(0);
        self.cursor_col = col.min(max_col);
    }

    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }

    pub fn selected_text(&self) -> Option<String> {
        self.selection.as_ref().map(|sel| {
            let (start_row, start_col, end_row, end_col) = sel.normalized();
            if start_row == end_row {
                self.lines
                    .get(start_row)
                    .map(|line| {
                        let chars: Vec<char> = line.chars().collect();
                        chars[start_col..end_col.min(chars.len())].iter().collect()
                    })
                    .unwrap_or_default()
            } else {
                let mut result = String::new();
                if let Some(first) = self.lines.get(start_row) {
                    let chars: Vec<char> = first.chars().collect();
                    result.push_str(&chars[start_col..].iter().collect::<String>());
                }
                for row in (start_row + 1)..end_row {
                    if let Some(line) = self.lines.get(row) {
                        result.push('\n');
                        result.push_str(line);
                    }
                }
                if let Some(last) = self.lines.get(end_row) {
                    result.push('\n');
                    let chars: Vec<char> = last.chars().collect();
                    result.push_str(&chars[..end_col.min(chars.len())].iter().collect::<String>());
                }
                result
            }
        })
    }

    pub fn has_selection(&self) -> bool {
        self.selection.as_ref().map_or(false, |s| !s.is_empty())
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn select_all(&mut self) {
        let last_row = self.lines.len().saturating_sub(1);
        let last_col = self.lines.last().map(|l| l.chars().count()).unwrap_or(0);
        self.selection = Some(Selection::new(0, 0, last_row, last_col));
        self.cursor_row = last_row;
        self.cursor_col = last_col;
    }

    pub fn delete_selection(&mut self) {
        if let Some(sel) = self.selection.take() {
            let (start_row, start_col, end_row, end_col) = sel.normalized();

            if start_row == end_row {
                if let Some(line) = self.lines.get_mut(start_row) {
                    let mut chars: Vec<char> = line.chars().collect();
                    chars.drain(start_col..end_col.min(chars.len()));
                    *line = chars.into_iter().collect();
                }
            } else {
                let first_line: String = if let Some(line) = self.lines.get(start_row) {
                    let chars: Vec<char> = line.chars().collect();
                    chars[..start_col].iter().collect()
                } else {
                    String::new()
                };
                let last_line: String = if let Some(line) = self.lines.get(end_row) {
                    let chars: Vec<char> = line.chars().collect();
                    chars[end_col.min(chars.len())..].iter().collect()
                } else {
                    String::new()
                };

                let new_line = format!("{}{}", first_line, last_line);
                self.lines.drain(start_row..=end_row);
                self.lines.insert(start_row, new_line);
            }

            self.cursor_row = start_row;
            self.cursor_col = start_col;
        }
    }

    pub fn insert(&mut self, text: &str) {
        if self.has_selection() {
            self.delete_selection();
        }

        for ch in text.chars() {
            if ch == '\n' {
                self.insert_newline();
            } else {
                self.insert_char(ch);
            }
        }
    }

    fn insert_char(&mut self, ch: char) {
        if let Some(line) = self.lines.get_mut(self.cursor_row) {
            let mut chars: Vec<char> = line.chars().collect();
            if self.cursor_col <= chars.len() {
                chars.insert(self.cursor_col, ch);
            } else {
                chars.push(ch);
            }
            *line = chars.into_iter().collect();
            self.cursor_col += 1;
        }
    }

    fn insert_newline(&mut self) {
        let current_line = self.lines.get(self.cursor_row).cloned().unwrap_or_default();
        let chars: Vec<char> = current_line.chars().collect();
        let before: String = chars[..self.cursor_col].iter().collect();
        let after: String = chars[self.cursor_col..].iter().collect();

        if let Some(line) = self.lines.get_mut(self.cursor_row) {
            *line = before;
        }
        self.lines.insert(self.cursor_row + 1, after);
        self.cursor_row += 1;
        self.cursor_col = 0;
    }

    pub fn delete_char_before(&mut self) {
        if self.has_selection() {
            self.delete_selection();
            return;
        }

        if self.cursor_col > 0 {
            if let Some(line) = self.lines.get_mut(self.cursor_row) {
                let mut chars: Vec<char> = line.chars().collect();
                if self.cursor_col <= chars.len() {
                    chars.remove(self.cursor_col - 1);
                    *line = chars.into_iter().collect();
                }
            }
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            let prev_col = self
                .lines
                .get(self.cursor_row)
                .map(|l| l.chars().count())
                .unwrap_or(0);
            if let Some(prev_line) = self.lines.get_mut(self.cursor_row) {
                prev_line.push_str(&current_line);
            }
            self.cursor_col = prev_col;
        }
    }

    pub fn delete_char_after(&mut self) {
        if self.has_selection() {
            self.delete_selection();
            return;
        }

        if let Some(line) = self.lines.get(self.cursor_row) {
            let chars: Vec<char> = line.chars().collect();
            if self.cursor_col < chars.len() {
                if let Some(line) = self.lines.get_mut(self.cursor_row) {
                    let mut chars: Vec<char> = line.chars().collect();
                    chars.remove(self.cursor_col);
                    *line = chars.into_iter().collect();
                }
            } else if self.cursor_row < self.lines.len() - 1 {
                let next_line = self.lines.remove(self.cursor_row + 1);
                if let Some(current) = self.lines.get_mut(self.cursor_row) {
                    current.push_str(&next_line);
                }
            }
        }
    }

    pub fn move_cursor_left(&mut self, extend_selection: bool) {
        let old_row = self.cursor_row;
        let old_col = self.cursor_col;

        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self
                .lines
                .get(self.cursor_row)
                .map(|l| l.chars().count())
                .unwrap_or(0);
        }

        self.update_selection(extend_selection, old_row, old_col);
    }

    pub fn move_cursor_right(&mut self, extend_selection: bool) {
        let old_row = self.cursor_row;
        let old_col = self.cursor_col;

        let current_len = self
            .lines
            .get(self.cursor_row)
            .map(|l| l.chars().count())
            .unwrap_or(0);
        if self.cursor_col < current_len {
            self.cursor_col += 1;
        } else if self.cursor_row < self.lines.len() - 1 {
            self.cursor_row += 1;
            self.cursor_col = 0;
        }

        self.update_selection(extend_selection, old_row, old_col);
    }

    pub fn move_cursor_up(&mut self, extend_selection: bool) {
        let old_row = self.cursor_row;
        let old_col = self.cursor_col;

        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            let max_col = self
                .lines
                .get(self.cursor_row)
                .map(|l| l.chars().count())
                .unwrap_or(0);
            self.cursor_col = self.cursor_col.min(max_col);
        }

        self.update_selection(extend_selection, old_row, old_col);
    }

    pub fn move_cursor_down(&mut self, extend_selection: bool) {
        let old_row = self.cursor_row;
        let old_col = self.cursor_col;

        if self.cursor_row < self.lines.len() - 1 {
            self.cursor_row += 1;
            let max_col = self
                .lines
                .get(self.cursor_row)
                .map(|l| l.chars().count())
                .unwrap_or(0);
            self.cursor_col = self.cursor_col.min(max_col);
        }

        self.update_selection(extend_selection, old_row, old_col);
    }

    pub fn move_cursor_to_start(&mut self, extend_selection: bool) {
        let old_row = self.cursor_row;
        let old_col = self.cursor_col;

        self.cursor_col = 0;

        self.update_selection(extend_selection, old_row, old_col);
    }

    pub fn move_cursor_to_end(&mut self, extend_selection: bool) {
        let old_row = self.cursor_row;
        let old_col = self.cursor_col;

        self.cursor_col = self
            .lines
            .get(self.cursor_row)
            .map(|l| l.chars().count())
            .unwrap_or(0);

        self.update_selection(extend_selection, old_row, old_col);
    }

    pub fn move_cursor_to_start_of_file(&mut self, extend_selection: bool) {
        let old_row = self.cursor_row;
        let old_col = self.cursor_col;

        self.cursor_row = 0;
        self.cursor_col = 0;
        self.scroll_row = 0;
        self.scroll_col = 0;

        self.update_selection(extend_selection, old_row, old_col);
    }

    pub fn move_cursor_to_end_of_file(&mut self, extend_selection: bool) {
        let old_row = self.cursor_row;
        let old_col = self.cursor_col;

        self.cursor_row = self.lines.len().saturating_sub(1);
        self.cursor_col = self
            .lines
            .get(self.cursor_row)
            .map(|l| l.chars().count())
            .unwrap_or(0);

        self.update_selection(extend_selection, old_row, old_col);
    }

    fn update_selection(&mut self, extend: bool, old_row: usize, old_col: usize) {
        if extend {
            if let Some(ref mut sel) = self.selection {
                sel.end_row = self.cursor_row;
                sel.end_col = self.cursor_col;
            } else {
                self.selection = Some(Selection::new(
                    old_row,
                    old_col,
                    self.cursor_row,
                    self.cursor_col,
                ));
            }
        } else {
            self.selection = None;
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Vec<Box<dyn Msg>> {
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);

        match key.code {
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.insert_char(c);
            }
            KeyCode::Enter => {
                self.insert_newline();
            }
            KeyCode::Backspace => {
                self.delete_char_before();
            }
            KeyCode::Delete => {
                self.delete_char_after();
            }
            KeyCode::Left => {
                self.move_cursor_left(shift);
            }
            KeyCode::Right => {
                self.move_cursor_right(shift);
            }
            KeyCode::Up => {
                self.move_cursor_up(shift);
            }
            KeyCode::Down => {
                self.move_cursor_down(shift);
            }
            KeyCode::Home => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.move_cursor_to_start_of_file(shift);
                } else {
                    self.move_cursor_to_end(shift);
                }
            }
            KeyCode::End => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.move_cursor_to_end_of_file(shift);
                } else {
                    self.move_cursor_to_end(shift);
                }
            }
            KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.select_all();
            }
            KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {}
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {}
            KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.delete_selection();
            }
            _ => {}
        }

        Vec::new()
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        let visible_height = area.height as usize;
        let visible_width = area.width as usize;

        for (row_idx, row_offset) in (self.scroll_row..).take(visible_height).enumerate() {
            if row_offset >= self.lines.len() {
                break;
            }

            let y = area.y + row_idx as u16;
            let line = &self.lines[row_offset];

            for (col_idx, col_offset) in (self.scroll_col..).take(visible_width).enumerate() {
                let x = area.x + col_idx as u16;
                let chars: Vec<char> = line.chars().collect();

                if let Some(cell) = buf.get_mut(x, y) {
                    if col_offset < chars.len() {
                        cell.symbol = chars[col_offset].to_string();

                        let in_selection = self
                            .selection
                            .as_ref()
                            .map_or(false, |s| s.contains(row_offset, col_offset));
                        let is_cursor =
                            row_offset == self.cursor_row && col_offset == self.cursor_col;

                        if is_cursor {
                            cell.set_style(self.cursor_style);
                        } else if in_selection {
                            cell.set_style(self.selection_style);
                        } else {
                            cell.set_style(self.style);
                        }
                    } else {
                        cell.symbol = " ".to_string();
                        cell.set_style(self.style);
                    }
                }
            }

            if let Some(cell) = buf.get_mut(area.x + line.chars().count() as u16, y) {
                if row_offset == self.cursor_row && self.cursor_col == line.chars().count() {
                    cell.set_style(self.cursor_style);
                }
            }
        }
    }

    pub fn update_scroll(&mut self, area: Rect) {
        let visible_height = area.height as usize;
        let visible_width = area.width as usize;

        if self.cursor_row < self.scroll_row {
            self.scroll_row = self.cursor_row;
        } else if self.cursor_row >= self.scroll_row + visible_height {
            self.scroll_row = self.cursor_row - visible_height + 1;
        }

        if self.cursor_col < self.scroll_col {
            self.scroll_col = self.cursor_col;
        } else if self.cursor_col >= self.scroll_col + visible_width {
            self.scroll_col = self.cursor_col - visible_width + 1;
        }
    }
}

pub type Editor = Textarea;

pub struct EditorState {
    pub content: String,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub selection: Option<Selection>,
}

impl From<&Textarea> for EditorState {
    fn from(textarea: &Textarea) -> Self {
        Self {
            content: textarea.content(),
            cursor_row: textarea.cursor_row,
            cursor_col: textarea.cursor_col,
            selection: textarea.selection.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EditorProps {
    pub content: String,
    pub style: Style,
    pub selection_style: Style,
    pub cursor_style: Style,
}

impl Default for EditorProps {
    fn default() -> Self {
        Self {
            content: String::new(),
            style: Style::default(),
            selection_style: Style::default(),
            cursor_style: Style::default(),
        }
    }
}

impl EditorProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn selection_style(mut self, style: Style) -> Self {
        self.selection_style = style;
        self
    }

    pub fn cursor_style(mut self, style: Style) -> Self {
        self.cursor_style = style;
        self
    }
}

impl Component for Textarea {
    type Props = EditorProps;
    type State = EditorState;

    fn create(props: Self::Props) -> Self {
        let mut textarea = Self::with_content(&props.content);
        textarea.style = props.style;
        textarea.selection_style = props.selection_style;
        textarea.cursor_style = props.cursor_style;
        textarea
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf);
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_textarea_new() {
        let textarea = Textarea::new();
        assert!(textarea.content().is_empty());
        assert_eq!(textarea.cursor_position(), (0, 0));
        assert!(!textarea.has_selection());
    }

    #[test]
    fn test_textarea_with_content() {
        let textarea = Textarea::with_content("Hello\nWorld");
        assert_eq!(textarea.content(), "Hello\nWorld");
        assert_eq!(textarea.line_count(), 2);
        assert_eq!(textarea.cursor_position(), (1, 5));
    }

    #[test]
    fn test_textarea_insert_char() {
        let mut textarea = Textarea::new();
        textarea.insert_char('a');
        textarea.insert_char('b');
        textarea.insert_char('c');
        assert_eq!(textarea.content(), "abc");
        assert_eq!(textarea.cursor_position(), (0, 3));
    }

    #[test]
    fn test_textarea_insert_newline() {
        let mut textarea = Textarea::new();
        textarea.insert_char('a');
        textarea.insert_char('b');
        textarea.insert_newline();
        textarea.insert_char('c');
        assert_eq!(textarea.content(), "ab\nc");
        assert_eq!(textarea.cursor_position(), (1, 1));
    }

    #[test]
    fn test_textarea_delete_char_before() {
        let mut textarea = Textarea::with_content("abc");
        textarea.set_cursor(0, 3);
        textarea.delete_char_before();
        assert_eq!(textarea.content(), "ab");
        assert_eq!(textarea.cursor_position(), (0, 2));
    }

    #[test]
    fn test_textarea_delete_char_after() {
        let mut textarea = Textarea::with_content("abc");
        textarea.set_cursor(0, 0);
        textarea.delete_char_after();
        assert_eq!(textarea.content(), "bc");
        assert_eq!(textarea.cursor_position(), (0, 0));
    }

    #[test]
    fn test_textarea_cursor_movement() {
        let mut textarea = Textarea::with_content("ab\ncd");
        textarea.set_cursor(1, 2);

        textarea.move_cursor_left(false);
        assert_eq!(textarea.cursor_position(), (1, 1));

        textarea.move_cursor_up(false);
        assert_eq!(textarea.cursor_position(), (0, 1));

        textarea.move_cursor_right(false);
        assert_eq!(textarea.cursor_position(), (0, 2));

        textarea.move_cursor_down(false);
        assert_eq!(textarea.cursor_position(), (1, 2));
    }

    #[test]
    fn test_textarea_selection() {
        let mut textarea = Textarea::with_content("Hello\nWorld");
        textarea.set_cursor(0, 0);
        textarea.move_cursor_right(true);
        textarea.move_cursor_right(true);
        textarea.move_cursor_right(true);

        assert!(textarea.has_selection());
        assert_eq!(textarea.selected_text(), Some("Hel".to_string()));
    }

    #[test]
    fn test_textarea_selection_vertical() {
        let mut textarea = Textarea::with_content("Hello\nWorld");
        textarea.set_cursor(0, 2);
        textarea.move_cursor_down(true);

        assert!(textarea.has_selection());
        assert_eq!(textarea.selected_text(), Some("llo\nWo".to_string()));
    }

    #[test]
    fn test_textarea_delete_selection() {
        let mut textarea = Textarea::with_content("Hello World");
        textarea.selection = Some(Selection::new(0, 6, 0, 11));
        textarea.delete_selection();
        assert_eq!(textarea.content(), "Hello ");
        assert_eq!(textarea.cursor_position(), (0, 6));
    }

    #[test]
    fn test_textarea_select_all() {
        let mut textarea = Textarea::with_content("Hello\nWorld");
        textarea.select_all();
        assert!(textarea.has_selection());
        assert_eq!(textarea.selected_text(), Some("Hello\nWorld".to_string()));
    }

    #[test]
    fn test_selection_contains() {
        let sel = Selection::new(0, 2, 2, 3);
        assert!(sel.contains(0, 2));
        assert!(sel.contains(1, 0));
        assert!(sel.contains(2, 2));
        assert!(!sel.contains(0, 1));
        assert!(!sel.contains(2, 4));
        assert!(!sel.contains(3, 0));
    }

    #[test]
    fn test_selection_normalized() {
        let sel = Selection::new(2, 3, 0, 1);
        let (s_row, s_col, e_row, e_col) = sel.normalized();
        assert_eq!((s_row, s_col, e_row, e_col), (0, 1, 2, 3));
    }

    #[test]
    fn test_textarea_insert_string() {
        let mut textarea = Textarea::new();
        textarea.insert("Hello\nWorld");
        assert_eq!(textarea.content(), "Hello\nWorld");
        assert_eq!(textarea.cursor_position(), (1, 5));
    }

    #[test]
    fn test_textarea_delete_line_merge() {
        let mut textarea = Textarea::with_content("Hello\nWorld");
        textarea.set_cursor(0, 5);
        textarea.delete_char_after();
        assert_eq!(textarea.content(), "HelloWorld");
        assert_eq!(textarea.line_count(), 1);
    }

    #[test]
    fn test_textarea_backspace_line_merge() {
        let mut textarea = Textarea::with_content("Hello\nWorld");
        textarea.set_cursor(1, 0);
        textarea.delete_char_before();
        assert_eq!(textarea.content(), "HelloWorld");
        assert_eq!(textarea.line_count(), 1);
    }

    #[test]
    fn test_editor_props() {
        let props = EditorProps::new().content("test").style(Style::default());

        let textarea = Textarea::create(props);
        assert_eq!(textarea.content(), "test");
    }
}
