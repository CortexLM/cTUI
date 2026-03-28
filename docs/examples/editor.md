# Editor Example

Text editor with syntax highlighting and editing capabilities.

## Overview

The editor demonstrates:

- Text buffer management
- Cursor movement and editing
- Syntax highlighting
- File operations
- Keyboard shortcuts

## Render Output

```
╭ editor.rs ───────────────────────────────────────────────────────╮
│  1 │ use ctui_core::{Buffer, Component, Rect};                   │
│  2 │ use ctui_components::{Editor, EditorProps};                 │
│  3 │                                                             │
│  4 │ fn main() {                                                 │
│  5 │     let mut editor = Editor::new()                          │
│  6 │         .content(initial_code)                              │
│  7 │         .language(Language::Rust)__;                        │
│  8 │ }                                                           │
│  9 │                                                             │
├─────┴ Code Editor ───────────────────────────────────────────────┤
│ i = INSERT | v = VISUAL | :q = quit | Ctrl-s = save              │
╰───────────────────────────────────────────────────────────────────╯
```

## Source Code

```rust
//! Editor example - text editor with syntax highlighting

use ctui_components::{Editor, EditorState, Language, Code};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Event, KeyCode, KeyEvent};

struct CodeEditor {
    editor: Editor,
    filename: String,
    modified: bool,
    mode: EditorMode,
}

enum EditorMode {
    Normal,
    Insert,
    Visual,
    Command,
}

impl Component for CodeEditor {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        let initial = r#"// Welcome to cTUI Editor
// Press 'i' for insert mode
// Press 'Esc' for normal mode

fn main() {
    println!("Hello, World!");
}
"#;
        
        Self {
            editor: Editor::new()
                .content(initial)
                .language(Language::Rust)
                .line_numbers(true)
                .highlight_current_line(true),
            filename: "untitled.rs".into(),
            modified: false,
            mode: EditorMode::Normal,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Calculate layout
        let header_height = 1;
        let status_height = 1;
        
        let header_area = Rect::new(area.x, area.y, area.width, header_height);
        let editor_area = Rect::new(
            area.x, 
            area.y + header_height, 
            area.width, 
            area.height - header_height - status_height
        );
        let status_area = Rect::new(
            area.x, 
            area.y + area.height - status_height, 
            area.width, 
            status_height
        );
        
        // Render header
        self.render_header(header_area, buf);
        
        // Render editor
        self.editor.render(editor_area, buf);
        
        // Render status bar
        self.render_status(status_area, buf);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        match (&self.mode, event) {
            // Normal mode
            (EditorMode::Normal, Event::Key(key)) => {
                match key.code {
                    KeyCode::Char('i') => {
                        self.mode = EditorMode::Insert;
                        Some(Box::new(ModeChanged))
                    }
                    KeyCode::Char('v') => {
                        self.mode = EditorMode::Visual;
                        Some(Box::new(ModeChanged))
                    }
                    KeyCode::Char(':') => {
                        self.mode = EditorMode::Command;
                        Some(Box::new(ModeChanged))
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        // Move cursor down
                        Some(Box::new(MoveDown))
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        // Move cursor up
                        Some(Box::new(MoveUp))
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        // Move cursor left
                        Some(Box::new(MoveLeft))
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        // Move cursor right
                        Some(Box::new(MoveRight))
                    }
                    _ => None,
                }
            }
            
            // Insert mode
            (EditorMode::Insert, Event::Key(key)) => {
                match key.code {
                    KeyCode::Esc => {
                        self.mode = EditorMode::Normal;
                        Some(Box::new(ModeChanged))
                    }
                    KeyCode::Char(c) => {
                        self.editor.insert(c);
                        self.modified = true;
                        Some(Box::new(ContentChanged))
                    }
                    KeyCode::Enter => {
                        self.editor.insert('\n');
                        self.modified = true;
                        Some(Box::new(ContentChanged))
                    }
                    KeyCode::Backspace => {
                        self.editor.backspace();
                        self.modified = true;
                        Some(Box::new(ContentChanged))
                    }
                    _ => None,
                }
            }
            
            // Visual mode
            (EditorMode::Visual, Event::Key(key)) => {
                match key.code {
                    KeyCode::Esc => {
                        self.mode = EditorMode::Normal;
                        Some(Box::new(ModeChanged))
                    }
                    KeyCode::Char('y') => {
                        // Yank selection
                        self.mode = EditorMode::Normal;
                        Some(Box::new(YankSelection))
                    }
                    KeyCode::Char('d') => {
                        // Delete selection
                        self.modified = true;
                        self.mode = EditorMode::Normal;
                        Some(Box::new(DeleteSelection))
                    }
                    _ => None,
                }
            }
            
            // Command mode
            (EditorMode::Command, Event::Key(key)) => {
                match key.code {
                    KeyCode::Esc => {
                        self.mode = EditorMode::Normal;
                        Some(Box::new(ModeChanged))
                    }
                    KeyCode::Enter => {
                        // Execute command
                        self.mode = EditorMode::Normal;
                        Some(Box::new(ExecuteCommand))
                    }
                    _ => None,
                }
            }
            
            _ => None,
        }
    }
}

impl CodeEditor {
    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let modified_marker = if self.modified { " [modified]" } else { "" };
        let title = format!(" {}{} ", self.filename, modified_marker);
        
        // Render title
        for (i, ch) in title.chars().take(area.width as usize).enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }
    
    fn render_status(&self, area: Rect, buf: &mut Buffer) {
        let mode_str = match self.mode {
            EditorMode::Normal => "NORMAL",
            EditorMode::Insert => "INSERT",
            EditorMode::Visual => "VISUAL",
            EditorMode::Command => "COMMAND",
        };
        
        let status = format!(
            "{} | {:?} | i=insert | v=visual | :q=quit",
            mode_str,
            self.editor.cursor_position()
        );
        
        for (i, ch) in status.chars().take(area.width as usize).enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }
    
    fn save(&mut self) -> Result<(), std::io::Error> {
        std::fs::write(&self.filename, self.editor.content())?;
        self.modified = false;
        Ok(())
    }
    
    fn open(&mut self, path: &str) -> Result<(), std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        self.editor = Editor::new()
            .content(&content)
            .line_numbers(true);
        self.filename = path.into();
        self.modified = false;
        Ok(())
    }
}
```

## Keyboard Shortcuts

| Mode | Key | Action |
|------|-----|--------|
| Normal | `i` | Insert mode |
| Normal | `v` | Visual mode |
| Normal | `:` | Command mode |
| Normal | `h/j/k/l` | Move cursor |
| Insert | `Esc` | Normal mode |
| Insert | Any char | Insert |
| Visual | `y` | Yank (copy) |
| Visual | `d` | Delete |
| Visual | `Esc` | Normal mode |
| Command | `:w` | Save |
| Command | `:q` | Quit |
| Command | `:wq` | Save and quit |

## Enhancements

### Add Undo/Redo

```rust
impl CodeEditor {
    fn undo(&mut self) {
        self.editor.undo();
    }
    
    fn redo(&mut self) {
        self.editor.redo();
    }
}
```

### Add Find/Replace

```rust
impl CodeEditor {
    fn find(&self, query: &str) -> Vec<Cursor> {
        // Return all positions matching query
    }
    
    fn replace(&mut self, find: &str, replace: &str) {
        // Replace all occurrences
    }
}
```

## Run the Example

```bash
cargo run -p ctui-benches --example editor
```

## See Also

- [Todo App](todo.md) - CRUD application
- [Editor Component](../gallery/editor.md) - Editor documentation
- [Code Component](../gallery/code.md) - Syntax highlighting
