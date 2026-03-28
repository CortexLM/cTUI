//! Input component for single-line text input
//!
//! This module provides a text input field with cursor tracking,
//! editing operations, and password masking support.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Style};

/// Password mask character
const PASSWORD_MASK: char = '•';

/// Messages for Input component
pub mod messages {
    use super::*;

    /// Emitted when the input value changes
    pub struct InputChanged {
        pub value: String,
    }
    impl Msg for InputChanged {}

    /// Emitted when Enter is pressed
    pub struct InputSubmitted {
        pub value: String,
    }
    impl Msg for InputSubmitted {}
}

/// State for the Input component
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputState {
    /// Current input value
    pub value: String,
    /// Current cursor position (byte index)
    pub cursor: usize,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
        }
    }
}

/// Props for creating an Input component
#[derive(Debug, Clone)]
pub struct InputProps {
    /// Initial value
    pub value: String,
    /// Placeholder text when empty
    pub placeholder: Option<String>,
    /// Whether to mask input (password field)
    pub password: bool,
    /// Visual style
    pub style: Style,
    /// Style for placeholder text
    pub placeholder_style: Style,
    /// Style for cursor
    pub cursor_style: Style,
}

impl Default for InputProps {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: None,
            password: false,
            style: Style::default(),
            placeholder_style: Style::default(),
            cursor_style: Style::default(),
        }
    }
}

impl InputProps {
    /// Create new InputProps with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set initial value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Enable password masking
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    /// Set style
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set placeholder style
    pub fn placeholder_style(mut self, style: Style) -> Self {
        self.placeholder_style = style;
        self
    }

    /// Set cursor style
    pub fn cursor_style(mut self, style: Style) -> Self {
        self.cursor_style = style;
        self
    }
}

/// Single-line text input component
#[derive(Debug, Clone)]
pub struct Input {
    /// Current value
    value: String,
    /// Cursor position (in characters, not bytes)
    cursor: usize,
    /// Placeholder text shown when empty
    placeholder: Option<String>,
    /// Whether to mask the input (password)
    password: bool,
    /// Base style for the input
    style: Style,
    /// Style for placeholder text
    placeholder_style: Style,
    /// Style for the cursor position
    cursor_style: Style,
    /// Insert mode (true = insert, false = replace)
    insert_mode: bool,
    /// Horizontal scroll offset for long text
    scroll: usize,
}

impl Input {
    /// Create a new Input with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the value (cursor positioned at end)
    pub fn value(mut self, value: impl Into<String>) -> Self {
        let value = value.into();
        let char_count = value.chars().count();
        self.cursor = char_count; // Cursor at end by default
        self.value = value;
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Enable password masking
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    /// Set base style
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set placeholder style
    pub fn placeholder_style(mut self, style: Style) -> Self {
        self.placeholder_style = style;
        self
    }

    /// Set cursor style
    pub fn cursor_style(mut self, style: Style) -> Self {
        self.cursor_style = style;
        self
    }

    /// Get current value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Get cursor position (in characters)
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Check if input is in insert mode
    pub fn is_insert_mode(&self) -> bool {
        self.insert_mode
    }

    /// Toggle insert/replace mode
    pub fn toggle_insert_mode(&mut self) {
        self.insert_mode = !self.insert_mode;
    }

    /// Set cursor position (in characters)
    pub fn set_cursor(&mut self, pos: usize) {
        let char_count = self.value.chars().count();
        self.cursor = pos.min(char_count);
    }

    /// Clear the input
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
        self.scroll = 0;
    }

    /// Handle a key event and return messages
    pub fn handle_key(&mut self, key: KeyEvent) -> Vec<Box<dyn Msg>> {
        let mut messages: Vec<Box<dyn Msg>> = Vec::new();
        let old_value = self.value.clone();

        match key.code {
            KeyCode::Char(c) => {
                // Handle Ctrl+key shortcuts
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match c {
                        'a' => self.move_cursor_to_start(),
                        'e' => self.move_cursor_to_end(),
                        'u' => self.clear_to_start(),
                        'k' => self.clear_to_end(),
                        'w' => self.delete_word_backwards(),
                        _ => {}
                    }
                } else if c.is_control() {
                    // Skip control characters
                } else {
                    self.insert_char(c);
                }
            }
            KeyCode::Backspace => {
                self.delete_char_before();
            }
            KeyCode::Delete => {
                self.delete_char_after();
            }
            KeyCode::Left => {
                if key.modifiers.contains(KeyModifiers::ALT) {
                    self.move_word_backwards();
                } else {
                    self.move_cursor_left();
                }
            }
            KeyCode::Right => {
                if key.modifiers.contains(KeyModifiers::ALT) {
                    self.move_word_forwards();
                } else {
                    self.move_cursor_right();
                }
            }
            KeyCode::Home => {
                self.move_cursor_to_start();
            }
            KeyCode::End => {
                self.move_cursor_to_end();
            }
            KeyCode::Insert => {
                self.toggle_insert_mode();
            }
            KeyCode::Enter => {
                messages.push(Box::new(messages::InputSubmitted {
                    value: self.value.clone(),
                }));
            }
            _ => {}
        }

        // Emit change message if value changed
        if self.value != old_value {
            messages.push(Box::new(messages::InputChanged {
                value: self.value.clone(),
            }));
        }

        messages
    }

    // Cursor movement operations

    fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.update_scroll();
        }
    }

    fn move_cursor_right(&mut self) {
        let char_count = self.value.chars().count();
        if self.cursor < char_count {
            self.cursor += 1;
            self.update_scroll();
        }
    }

    fn move_cursor_to_start(&mut self) {
        self.cursor = 0;
        self.scroll = 0;
    }

    fn move_cursor_to_end(&mut self) {
        self.cursor = self.value.chars().count();
        self.update_scroll();
    }

    fn move_word_backwards(&mut self) {
        // Skip trailing whitespace
        let chars: Vec<char> = self.value.chars().collect();
        let mut pos = self.cursor;

        while pos > 0 && chars.get(pos - 1).map_or(false, |c| c.is_whitespace()) {
            pos -= 1;
        }

        // Skip word characters
        while pos > 0 && chars.get(pos - 1).map_or(false, |c| !c.is_whitespace()) {
            pos -= 1;
        }

        self.cursor = pos;
        self.update_scroll();
    }

    fn move_word_forwards(&mut self) {
        let chars: Vec<char> = self.value.chars().collect();
        let len = chars.len();
        let mut pos = self.cursor;

        // Skip current word
        while pos < len && chars.get(pos).map_or(false, |c| !c.is_whitespace()) {
            pos += 1;
        }

        // Skip whitespace
        while pos < len && chars.get(pos).map_or(false, |c| c.is_whitespace()) {
            pos += 1;
        }

        self.cursor = pos;
        self.update_scroll();
    }

    // Editing operations

    fn insert_char(&mut self, c: char) {
        let byte_pos = self.char_index_to_byte(self.cursor);

        if self.insert_mode {
            self.value.insert(byte_pos, c);
        } else {
            let mut chars: Vec<char> = self.value.chars().collect();
            if self.cursor < chars.len() {
                chars[self.cursor] = c;
                self.value = chars.into_iter().collect();
            } else {
                self.value.insert(byte_pos, c);
            }
        }

        self.cursor += 1;
        self.update_scroll();
    }

    fn delete_char_before(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let byte_pos = self.char_index_to_byte(self.cursor - 1);
        self.value.remove(byte_pos);
        self.cursor -= 1;
        self.update_scroll();
    }

    fn delete_char_after(&mut self) {
        let char_count = self.value.chars().count();
        if self.cursor >= char_count {
            return;
        }

        let byte_pos = self.char_index_to_byte(self.cursor);
        self.value.remove(byte_pos);
        self.update_scroll();
    }

    fn clear_to_start(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let byte_pos = self.char_index_to_byte(self.cursor);
        self.value = self.value[byte_pos..].to_string();
        self.cursor = 0;
        self.scroll = 0;
    }

    fn clear_to_end(&mut self) {
        if self.cursor >= self.value.chars().count() {
            return;
        }

        let byte_pos = self.char_index_to_byte(self.cursor);
        self.value.truncate(byte_pos);
    }

    fn delete_word_backwards(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let old_cursor = self.cursor;
        self.move_word_backwards();

        let start = self.char_index_to_byte(self.cursor);
        let end = self.char_index_to_byte(old_cursor);
        self.value = format!("{}{}", &self.value[..start], &self.value[end..]);
    }

    // Helper functions

    fn char_index_to_byte(&self, char_idx: usize) -> usize {
        self.value
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.value.len())
    }

    fn update_scroll(&mut self) {
        // Keep cursor visible by adjusting scroll offset
        // This is a simple implementation - does not account for actual width
        // Real implementation would need to know the render width
        if self.cursor < self.scroll {
            self.scroll = self.cursor;
        }
    }

    /// Get display string (masked if password)
    fn display_string(&self) -> String {
        if self.password {
            self.value.chars().map(|_| PASSWORD_MASK).collect()
        } else {
            self.value.clone()
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            placeholder: None,
            password: false,
            style: Style::default(),
            placeholder_style: Style::default(),
            cursor_style: Style::default(),
            insert_mode: true,
            scroll: 0,
        }
    }
}

impl Component for Input {
    type Props = InputProps;
    type State = InputState;

    fn create(props: Self::Props) -> Self {
        let char_count = props.value.chars().count();
        Self {
            value: props.value,
            cursor: char_count, // Cursor at end by default
            placeholder: props.placeholder,
            password: props.password,
            style: props.style,
            placeholder_style: props.placeholder_style,
            cursor_style: props.cursor_style,
            insert_mode: true,
            scroll: 0,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        // Fill background with style
        for x in area.x..area.x + area.width {
            buf.modify_cell(x, area.y, |cell| { cell.set_style(self.style); });
        }

        // Render placeholder if empty
        if self.value.is_empty() {
            if let Some(ref placeholder) = self.placeholder {
                let placeholder_chars: Vec<char> = placeholder.chars().collect();
                let max_chars = (area.width as usize).min(placeholder_chars.len());

                for (i, ch) in placeholder_chars.iter().take(max_chars).enumerate() {
                    buf.modify_cell(area.x + i as u16, area.y, |cell| {
                        cell.symbol = ch.to_string();
                        cell.set_style(self.placeholder_style);
                    });
                }
            }
            return;
        }

        // Render actual value
        let display = self.display_string();
        let chars: Vec<char> = display.chars().collect();
        let available_width = area.width as usize;

        // Calculate visible range based on scroll and cursor
        let visible_start = self.scroll;
        let visible_end = (visible_start + available_width).min(chars.len());

        // Render visible characters
        for (i, ch_idx) in (visible_start..visible_end).enumerate() {
            if i >= available_width {
                break;
            }

            let x = area.x + i as u16;
            buf.modify_cell(x, area.y, |cell| {
                cell.symbol = chars.get(ch_idx).map(|c| c.to_string()).unwrap_or_default();
                // Highlight cursor position
                if ch_idx == self.cursor {
                cell.set_style(self.cursor_style);
                } else {
                cell.set_style(self.style);
                }
            });
        }

        // If cursor is at end, render cursor position
        if self.cursor >= visible_end && self.cursor < area.x as usize + available_width {
            let cursor_x = area.x + (self.cursor - visible_start) as u16;
            if cursor_x < area.x + area.width {
                buf.modify_cell(cursor_x, area.y, |cell| {
                    cell.symbol = " ".to_string();
                    cell.set_style(self.cursor_style);
                });
            }
        }

        // Show cursor indicator at end if cursor is at end of text
        if self.cursor == chars.len() && self.cursor >= visible_start {
            let cursor_screen_pos = (self.cursor - visible_start) as u16;
            if cursor_screen_pos < area.width {
                buf.modify_cell(area.x + cursor_screen_pos, area.y, |cell| {
                    // Cursor is at end - show block cursor or underline
                    cell.symbol = " ".to_string();
                    cell.set_style(self.cursor_style);
                });
            }
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

impl InputState {
    /// Create new InputState
    pub fn new() -> Self {
        Self::default()
    }

    /// Create InputState from an Input component
    pub fn from_input(input: &Input) -> Self {
        Self {
            value: input.value.clone(),
            cursor: input.cursor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyModifiers};
    use ctui_core::Buffer;
    use insta::assert_snapshot;

    fn make_key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::empty())
    }

    fn make_ctrl_key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::CONTROL)
    }

    fn render_to_string(input: &Input, width: u16, height: u16) -> String {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        input.render(Rect::new(0, 0, width, height), &mut buf);

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
    fn snapshot_input_empty() {
        let input = Input::new();
        let result = render_to_string(&input, 20, 1);
        assert_snapshot!("input_empty", result);
    }

    #[test]
    fn snapshot_input_with_placeholder() {
        let input = Input::new().placeholder("Enter your name...");
        let result = render_to_string(&input, 25, 1);
        assert_snapshot!("input_with_placeholder", result);
    }

    #[test]
    fn snapshot_input_with_text() {
        let input = Input::new().value("Hello, World!");
        let result = render_to_string(&input, 25, 1);
        assert_snapshot!("input_with_text", result);
    }

    #[test]
    fn snapshot_input_password_masked() {
        let input = Input::new().value("secret123").password(true);
        let result = render_to_string(&input, 20, 1);
        assert_snapshot!("input_password_masked", result);
    }

    #[test]
    fn snapshot_input_cursor_at_start() {
        let mut input = Input::new().value("test");
        input.set_cursor(0);
        let result = render_to_string(&input, 15, 1);
        assert_snapshot!("input_cursor_at_start", result);
    }

    #[test]
    fn snapshot_input_cursor_in_middle() {
        let mut input = Input::new().value("testing");
        input.set_cursor(3);
        let result = render_to_string(&input, 15, 1);
        assert_snapshot!("input_cursor_in_middle", result);
    }

    #[test]
    fn snapshot_input_cursor_at_end() {
        let input = Input::new().value("end");
        let result = render_to_string(&input, 15, 1);
        assert_snapshot!("input_cursor_at_end", result);
    }

    #[test]
    fn snapshot_input_truncated() {
        let input = Input::new().value("This is a very long text that should be truncated");
        let result = render_to_string(&input, 15, 1);
        assert_snapshot!("input_truncated", result);
    }

    #[test]
    fn test_new_input_empty() {
        let input = Input::new();
        assert_eq!(input.get_value(), "");
        assert_eq!(input.cursor(), 0);
        assert!(!input.password);
    }

    #[test]
    fn test_input_with_value() {
        let input = Input::new().value("hello");
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor(), 5); // Cursor at end
    }

    #[test]
    fn test_insert_char() {
        let mut input = Input::new();
        input.handle_key(make_key(KeyCode::Char('a')));
        input.handle_key(make_key(KeyCode::Char('b')));
        input.handle_key(make_key(KeyCode::Char('c')));

        assert_eq!(input.get_value(), "abc");
        assert_eq!(input.cursor(), 3);
    }

    #[test]
    fn test_insert_unicode() {
        let mut input = Input::new();
        input.handle_key(make_key(KeyCode::Char('日')));
        input.handle_key(make_key(KeyCode::Char('本')));
        input.handle_key(make_key(KeyCode::Char('語')));

        assert_eq!(input.get_value(), "日本語");
        assert_eq!(input.cursor(), 3);
    }

    #[test]
    fn test_backspace() {
        let mut input = Input::new().value("hello");
        input.set_cursor(5);

        input.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(input.get_value(), "hell");
        assert_eq!(input.cursor(), 4);

        input.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(input.get_value(), "hel");
        assert_eq!(input.cursor(), 3);
    }

    #[test]
    fn test_backspace_at_start() {
        let mut input = Input::new().value("hello");
        input.set_cursor(0);

        input.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_delete() {
        let mut input = Input::new().value("hello");
        input.set_cursor(0);

        input.handle_key(make_key(KeyCode::Delete));
        assert_eq!(input.get_value(), "ello");
        assert_eq!(input.cursor(), 0);

        input.handle_key(make_key(KeyCode::Delete));
        assert_eq!(input.get_value(), "llo");
    }

    #[test]
    fn test_delete_at_end() {
        let mut input = Input::new().value("hello");
        input.set_cursor(5);

        input.handle_key(make_key(KeyCode::Delete));
        assert_eq!(input.get_value(), "hello");
    }

    #[test]
    fn test_cursor_left_right() {
        let mut input = Input::new().value("hello");
        input.set_cursor(3);

        input.handle_key(make_key(KeyCode::Left));
        assert_eq!(input.cursor(), 2);

        input.handle_key(make_key(KeyCode::Left));
        assert_eq!(input.cursor(), 1);

        input.handle_key(make_key(KeyCode::Right));
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_cursor_bounds() {
        let mut input = Input::new().value("abc");
        input.set_cursor(0);

        // Can't go left of 0
        input.handle_key(make_key(KeyCode::Left));
        assert_eq!(input.cursor(), 0);

        // Can go right
        input.handle_key(make_key(KeyCode::Right));
        assert_eq!(input.cursor(), 1);

        // Go past end
        input.set_cursor(100);
        assert_eq!(input.cursor(), 3); // Clamped to length
    }

    #[test]
    fn test_home_end() {
        let mut input = Input::new().value("hello");
        input.set_cursor(3);

        input.handle_key(make_key(KeyCode::Home));
        assert_eq!(input.cursor(), 0);

        input.handle_key(make_key(KeyCode::End));
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_ctrl_a_e() {
        let mut input = Input::new().value("hello");
        input.set_cursor(2);

        input.handle_key(make_ctrl_key(KeyCode::Char('a')));
        assert_eq!(input.cursor(), 0);

        input.handle_key(make_ctrl_key(KeyCode::Char('e')));
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_ctrl_u() {
        let mut input = Input::new().value("hello world");
        input.set_cursor(6); // After "hello "

        input.handle_key(make_ctrl_key(KeyCode::Char('u')));
        assert_eq!(input.get_value(), "world");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_ctrl_k() {
        let mut input = Input::new().value("hello world");
        input.set_cursor(5); // After "hello"

        input.handle_key(make_ctrl_key(KeyCode::Char('k')));
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_ctrl_w() {
        let mut input = Input::new().value("hello world");
        input.set_cursor(11); // At end

        input.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(input.get_value(), "hello ");
        assert_eq!(input.cursor(), 6);

        input.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(input.get_value(), "");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_word_navigation() {
        let mut input = Input::new().value("hello world test");
        input.set_cursor(0);

        // Alt+Right to move forward by word
        input.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::ALT));
        assert_eq!(input.cursor(), 6); // At "world"

        input.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::ALT));
        assert_eq!(input.cursor(), 12); // At "test"

        // Alt+Left to move backward by word
        input.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::ALT));
        assert_eq!(input.cursor(), 6); // Back at "world"
    }

    #[test]
    fn test_insert_mode() {
        let mut input = Input::new().value("abc");
        input.set_cursor(1);
        input.insert_mode = false; // Replace mode

        input.handle_key(make_key(KeyCode::Insert)); // Toggle to insert
        input.handle_key(make_key(KeyCode::Char('X')));
        // In insert mode, 'X' is inserted at position 1
        assert_eq!(input.get_value(), "aXbc");

        input.insert_mode = false; // Replace mode
        input.handle_key(make_key(KeyCode::Char('Y')));
        assert_eq!(input.get_value(), "aXYc");
    }

    #[test]
    fn test_password_mask() {
        let mut input = Input::new().password(true);
        input.handle_key(make_key(KeyCode::Char('a')));
        input.handle_key(make_key(KeyCode::Char('b')));
        input.handle_key(make_key(KeyCode::Char('c')));

        assert_eq!(input.get_value(), "abc");
        assert_eq!(input.display_string(), "•••");
    }

    #[test]
    fn test_password_unicode() {
        let mut input = Input::new().password(true);
        input.handle_key(make_key(KeyCode::Char('日')));
        input.handle_key(make_key(KeyCode::Char('本')));

        assert_eq!(input.get_value(), "日本");
        assert_eq!(input.display_string(), "••");
    }

    #[test]
    fn test_clear() {
        let mut input = Input::new().value("hello");
        input.clear();
        assert_eq!(input.get_value(), "");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_change_notification() {
        let mut input = Input::new();
        let messages = input.handle_key(make_key(KeyCode::Char('a')));

        assert_eq!(messages.len(), 1);
        // Check that we got an InputChanged message
        let changed = messages[0].downcast_ref::<messages::InputChanged>();
        assert!(changed.is_some());
        assert_eq!(changed.unwrap().value, "a");
    }

    #[test]
    fn test_submit_notification() {
        let mut input = Input::new().value("test");
        input.set_cursor(4);

        let messages = input.handle_key(make_key(KeyCode::Enter));

        // Should have submitted message (no change since Enter doesn't modify)
        assert!(messages.iter().any(|m| m.is::<messages::InputSubmitted>()));
    }

    #[test]
    fn test_placeholder_when_empty() {
        let input = Input::new().placeholder("Enter text...");

        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
        input.render(Rect::new(0, 0, 20, 1), &mut buf);

        // Should show placeholder
        assert!(buf.get(0, 0).unwrap().symbol.starts_with('E'));
    }

    #[test]
    fn test_render_with_value() {
        let input = Input::new().value("test");

        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
        input.render(Rect::new(0, 0, 20, 1), &mut buf);

        assert_eq!(buf.get(0, 0).unwrap().symbol, "t");
        assert_eq!(buf.get(1, 0).unwrap().symbol, "e");
        assert_eq!(buf.get(2, 0).unwrap().symbol, "s");
        assert_eq!(buf.get(3, 0).unwrap().symbol, "t");
    }

    #[test]
    fn test_create_with_props() {
        let props = InputProps::new()
            .value("initial")
            .placeholder("type here")
            .password(true);

        let input = Input::create(props);
        assert_eq!(input.get_value(), "initial");
        assert!(input.password);
        assert!(input.placeholder.is_some());
    }

    #[test]
    fn test_set_cursor_clamping() {
        let mut input = Input::new().value("abc");
        input.set_cursor(100); // Way past end
        assert_eq!(input.cursor(), 3); // Clamped to length
    }

    #[test]
    fn test_insert_in_middle() {
        let mut input = Input::new().value("ac");
        input.set_cursor(1);

        input.handle_key(make_key(KeyCode::Char('b')));
        assert_eq!(input.get_value(), "abc");
        assert_eq!(input.cursor(), 2);
    }

    // ============================================
    // Additional event handler tests (T18)
    // ============================================

    #[test]
    fn test_arrow_up_no_op() {
        // Arrow Up should do nothing on single-line input
        let mut input = Input::new().value("hello");
        input.set_cursor(2);
        let old_cursor = input.cursor();

        input.handle_key(make_key(KeyCode::Up));
        assert_eq!(input.cursor(), old_cursor);
        assert_eq!(input.get_value(), "hello");
    }

    #[test]
    fn test_arrow_down_no_op() {
        // Arrow Down should do nothing on single-line input
        let mut input = Input::new().value("hello");
        input.set_cursor(2);
        let old_cursor = input.cursor();

        input.handle_key(make_key(KeyCode::Down));
        assert_eq!(input.cursor(), old_cursor);
        assert_eq!(input.get_value(), "hello");
    }

    #[test]
    fn test_ctrl_u_at_start_no_change() {
        // Ctrl+U at cursor position 0 should do nothing
        let mut input = Input::new().value("hello");
        input.set_cursor(0);

        input.handle_key(make_ctrl_key(KeyCode::Char('u')));
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_ctrl_k_at_end_no_change() {
        // Ctrl+K at end of text should do nothing
        let mut input = Input::new().value("hello");
        input.set_cursor(5); // At end

        input.handle_key(make_ctrl_key(KeyCode::Char('k')));
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_ctrl_a_already_at_start() {
        // Ctrl+A when already at start should be idempotent
        let mut input = Input::new().value("hello");
        input.set_cursor(0);

        input.handle_key(make_ctrl_key(KeyCode::Char('a')));
        assert_eq!(input.cursor(), 0);
        assert_eq!(input.get_value(), "hello");
    }

    #[test]
    fn test_ctrl_e_already_at_end() {
        // Ctrl+E when already at end should be idempotent
        let mut input = Input::new().value("hello");
        input.set_cursor(5);

        input.handle_key(make_ctrl_key(KeyCode::Char('e')));
        assert_eq!(input.cursor(), 5);
        assert_eq!(input.get_value(), "hello");
    }

    #[test]
    fn test_insert_key_toggle_multiple() {
        // Insert key should toggle mode multiple times
        let mut input = Input::new().value("abc");
        assert!(input.is_insert_mode()); // default is insert

        input.handle_key(make_key(KeyCode::Insert));
        assert!(!input.is_insert_mode()); // now replace

        input.handle_key(make_key(KeyCode::Insert));
        assert!(input.is_insert_mode()); // back to insert

        input.handle_key(make_key(KeyCode::Insert));
        assert!(!input.is_insert_mode()); // back to replace
    }

    #[test]
    fn test_insert_mode_replace_behavior() {
        // In replace mode, typing should replace the character at cursor
        let mut input = Input::new().value("xxxxx");
        input.set_cursor(0);
        input.insert_mode = false; // Replace mode

        input.handle_key(make_key(KeyCode::Char('a')));
        assert_eq!(input.get_value(), "axxxx");
        assert_eq!(input.cursor(), 1);

        input.handle_key(make_key(KeyCode::Char('b')));
        assert_eq!(input.get_value(), "abxxx");
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_insert_mode_replace_at_end() {
        // In replace mode at end, should append like insert mode
        let mut input = Input::new().value("abc");
        input.set_cursor(3); // At end
        input.insert_mode = false;

        input.handle_key(make_key(KeyCode::Char('d')));
        assert_eq!(input.get_value(), "abcd");
        assert_eq!(input.cursor(), 4);
    }

    #[test]
    fn test_alt_left_at_start() {
        // Alt+Left at position 0 should do nothing
        let mut input = Input::new().value("hello world");
        input.set_cursor(0);

        input.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::ALT));
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_alt_right_at_end() {
        // Alt+Right at end should do nothing
        let mut input = Input::new().value("hello");
        input.set_cursor(5); // At end

        input.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::ALT));
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_unicode_backspace() {
        // Backspace should correctly delete unicode characters
        let mut input = Input::new().value("日本語");
        input.set_cursor(3);

        input.handle_key(make_key(KeyCode::Backspace));
        assert_eq!(input.get_value(), "日本");
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_unicode_delete() {
        // Delete should correctly delete unicode characters
        let mut input = Input::new().value("日本語");
        input.set_cursor(0);

        input.handle_key(make_key(KeyCode::Delete));
        assert_eq!(input.get_value(), "本語");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_multiple_ctrl_u() {
        // Multiple Ctrl+U calls should eventually clear all
        let mut input = Input::new().value("hello world test");
        input.set_cursor(10);

        input.handle_key(make_ctrl_key(KeyCode::Char('u')));
        // Should clear to start
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_multiple_ctrl_k() {
        // Multiple Ctrl+K calls: first clears to end, subsequent no-ops
        let mut input = Input::new().value("hello world");
        input.set_cursor(5);

        input.handle_key(make_ctrl_key(KeyCode::Char('k')));
        assert_eq!(input.get_value(), "hello");

        // Second Ctrl+K should do nothing (already at end)
        input.handle_key(make_ctrl_key(KeyCode::Char('k')));
        assert_eq!(input.get_value(), "hello");
    }

    #[test]
    fn test_backspace_after_left_navigation() {
        // Navigate left then backspace
        let mut input = Input::new().value("abcdef");
        input.set_cursor(5); // At 'f'

        input.handle_key(make_key(KeyCode::Left)); // At 'e'
        input.handle_key(make_key(KeyCode::Left)); // At 'd'
        input.handle_key(make_key(KeyCode::Backspace));

        // cursor at 3, backspace deletes char at position 2 ('c')
        assert_eq!(input.get_value(), "abdef");
        assert_eq!(input.cursor(), 2);
    }

    #[test]
    fn test_delete_after_left_navigation() {
        // Navigate left then delete
        let mut input = Input::new().value("abcdef");
        input.set_cursor(5); // At 'f'

        input.handle_key(make_key(KeyCode::Left)); // At 'e'
        input.handle_key(make_key(KeyCode::Delete)); // Deletes 'e'

        assert_eq!(input.get_value(), "abcdf");
        assert_eq!(input.cursor(), 4);
    }

    #[test]
    fn test_home_after_typing() {
        // Type some characters then press Home
        let mut input = Input::new();
        input.handle_key(make_key(KeyCode::Char('a')));
        input.handle_key(make_key(KeyCode::Char('b')));
        input.handle_key(make_key(KeyCode::Char('c')));

        input.handle_key(make_key(KeyCode::Home));
        assert_eq!(input.cursor(), 0);
        assert_eq!(input.get_value(), "abc");
    }

    #[test]
    fn test_end_after_cursor_move() {
        // Move cursor then press End
        let mut input = Input::new().value("testing");
        input.set_cursor(0);

        input.handle_key(make_key(KeyCode::End));
        assert_eq!(input.cursor(), 7);
    }

    #[test]
    fn test_ctrl_w_multiple_words() {
        // Ctrl+W multiple times should delete words backward
        let mut input = Input::new().value("one two three");
        input.set_cursor(13); // At end

        input.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(input.get_value(), "one two ");
        assert_eq!(input.cursor(), 8);

        input.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(input.get_value(), "one ");
        assert_eq!(input.cursor(), 4);

        input.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(input.get_value(), "");
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_no_change_no_message() {
        // Keys that don't change value shouldn't emit InputChanged
        let mut input = Input::new().value("test");
        input.set_cursor(2);

        // Left doesn't change value
        let messages = input.handle_key(make_key(KeyCode::Left));
        let changed_msgs: Vec<_> = messages
            .iter()
            .filter(|m| m.is::<messages::InputChanged>())
            .collect();
        assert!(changed_msgs.is_empty());
    }

    #[test]
    fn test_change_message_after_ctrl_k() {
        // Ctrl+K should emit change message since it modifies value
        let mut input = Input::new().value("hello world");
        input.set_cursor(5);

        let messages = input.handle_key(make_ctrl_key(KeyCode::Char('k')));
        assert!(messages.iter().any(|m| m.is::<messages::InputChanged>()));
        // Verify the changed value
        let changed = messages
            .iter()
            .find_map(|m| m.downcast_ref::<messages::InputChanged>());
        assert!(changed.is_some());
        assert_eq!(changed.unwrap().value, "hello");
    }

    #[test]
    fn test_ctrl_w_at_start_no_change() {
        // Ctrl+W at cursor position 0 should do nothing
        let mut input = Input::new().value("hello");
        input.set_cursor(0);

        input.handle_key(make_ctrl_key(KeyCode::Char('w')));
        assert_eq!(input.get_value(), "hello");
        assert_eq!(input.cursor(), 0);
    }
}
