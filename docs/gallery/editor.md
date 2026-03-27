# Editor Component

Full-featured text editor with editing capabilities.

## Variants

### Basic Editor

**Code:**

```rust
let editor = Editor::new()
    .content("Hello, World!");
```

**Render:**

```
Hello, World!_
```

### With Line Numbers

**Code:**

```rust
let editor = Editor::new()
    .content(multiline_text)
    .line_numbers(true);
```

**Render:**

```
 1 │ fn main() {
 2 │     println!("Hello");
 3 │ }_
 4 │ 
```

### With Syntax Highlighting

**Code:**

```rust
let editor = Editor::new()
    .content(rust_code)
    .language(Language::Rust)
    .highlight_current_line(true);
```

**Render:**

```
 1 │ fn main() {
 2 │ >   println!("Hello");  <-- highlighted
 3 │ }
```

### Textarea

Multi-line text area:

**Code:**

```rust
let textarea = Textarea::new()
    .content("Line 1\nLine 2\nLine 3")
    .max_lines(100);
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `content` | `String` | "" | Editor content |
| `cursor_position` | `usize` | 0 | Cursor offset |
| `line_numbers` | `bool` | false | Show line numbers |
| `language` | `Option<Language>` | None | Syntax highlighting |
| `highlight_current_line` | `bool` | true | Highlight cursor line |
| `read_only` | `bool` | false | Disable editing |
| `max_length` | `Option<usize>` | None | Max characters |
| `placeholder` | `String` | "" | Placeholder text |

## EditorState

```rust
pub struct EditorState {
    pub content: String,
    pub cursor: Cursor,
    pub selection: Option<Selection>,
    pub history: History,
}

pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

pub struct Selection {
    pub start: Cursor,
    pub end: Cursor,
}
```

## Events

| Event | Action |
|-------|--------|
| `Char(c)` | Insert character |
| `Backspace` | Delete before cursor |
| `Delete` | Delete after cursor |
| `Enter` | Insert newline |
| `Up` / `Down` | Move cursor |
| `Left` / `Right` | Move cursor |
| `Home` / `End` | Line start/end |
| `Ctrl+A` | Select all |
| `Ctrl+C` | Copy |
| `Ctrl+V` | Paste |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |

## Selection

```rust
impl Editor {
    // Selection methods
    pub fn select_all(&mut self);
    pub fn select_line(&mut self, line: usize);
    pub fn select_word(&mut self);
    pub fn clear_selection(&mut self);
    
    // Edit methods
    pub fn insert(&mut self, text: &str);
    pub fn delete_selection(&mut self);
    pub fn replace_selection(&mut self, text: &str);
    
    // History
    pub fn undo(&mut self) -> bool;
    pub fn redo(&mut self) -> bool;
}
```

## Example

```rust
use ctui_components::{Editor, EditorState, Language};

struct CodeEditor {
    editor: Editor,
}

impl CodeEditor {
    fn new() -> Self {
        let initial_code = r#"fn main() {
    println!("Hello, World!");
}"#;
        
        Self {
            editor: Editor::new()
                .content(initial_code)
                .language(Language::Rust)
                .line_numbers(true)
                .highlight_current_line(true),
        }
    }
}

impl Component for CodeEditor {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.editor.render(area, buf);
    }
    
    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char(c) => {
                        self.editor.insert(c);
                        Some(Box::new(ContentChanged))
                    }
                    KeyCode::Backspace => {
                        // Handle backspace
                        Some(Box::new(ContentChanged))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
```

## Scroll Management

```rust
impl Editor {
    pub fn scroll_to_cursor(&mut self);
    pub fn first_visible_line(&self) -> usize;
    pub fn visible_lines(&self, height: u16) -> usize;
}
```

## See Also

- [Input](input.md) - Single-line input
- [Code](code.md) - ReadOnly code display
- [Diff](diff.md) - Diff viewer
