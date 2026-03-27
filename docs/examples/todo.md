# Todo App Example

A full CRUD application demonstrating lists, forms, and state management.

## Overview

The todo app demonstrates:

- List with selection
- CRUD operations (Create, Read, Update, Delete)
- Form with input
- State management
- Event handling

## Render Output

```
╔══════════════════════════════════════╗
║           Todo App                   ║
╠══════════════════════════════════════╣
║ [ ] Learn cTUI framework             ║
║ [ ] Build awesome TUI apps           ║
║ [x] Contribute to open source        ║
╠══════════════════════════════════════╣
║ Add: [________________________]      ║
║         [Add]  [Clear Done]          ║
╚══════════════════════════════════════╝
```

## Source Code

```rust
//! Todo app example - full CRUD operations demonstration

use ctui_components::{Block, Borders, List, ListItem, Input, Paragraph};
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Style};

// Messages
struct AddTodo(String);
struct RemoveTodo(usize);
struct ToggleTodo(usize);
struct SetInput(String);

impl Msg for AddTodo {}
impl Msg for RemoveTodo {}
impl Msg for ToggleTodo {}
impl Msg for SetInput {}

// Todo item
#[derive(Clone, Debug)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

// State
struct TodoState {
    items: Vec<TodoItem>,
    next_id: usize,
    input: String,
}

impl TodoState {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 0,
            input: String::new(),
        }
    }

    fn add(&mut self, text: String) {
        if text.is_empty() { return; }
        let item = TodoItem {
            id: self.next_id,
            text,
            completed: false,
        };
        self.next_id += 1;
        self.items.push(item);
    }

    fn remove(&mut self, id: usize) {
        self.items.retain(|item| item.id != id);
    }

    fn toggle(&mut self, id: usize) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = !item.completed;
        }
    }
}

// Component
struct TodoApp {
    state: TodoState,
}

impl TodoApp {
    fn new() -> Self {
        Self { state: TodoState::new() }
    }
}

impl Component for TodoApp {
    type Props = ();
    type State = TodoState;

    fn create(_: Self::Props) -> Self {
        Self::new()
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Header
        let header = "╔══════════════════════════════════════╗";
        let title = "║           Todo App                   ║";
        let divider = "╠══════════════════════════════════════╣";
        let footer = "╚══════════════════════════════════════╝";

        // Render header
        for (row, line) in [header, title, divider].iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if let Some(cell) = buf.get_mut(area.x + col as u16, area.y + row as u16) {
                    cell.symbol = ch.to_string();
                }
            }
        }

        // Render items
        let item_start = 3;
        for (idx, item) in self.state.items.iter().enumerate() {
            let check = if item.completed { "[x]" } else { "[ ]" };
            let text = format!("║ {} {} {}", check, item.text);
            
            for (col, ch) in text.chars().enumerate() {
                if let Some(cell) = buf.get_mut(area.x + col as u16, area.y + item_start + idx as u16) {
                    cell.symbol = ch.to_string();
                }
            }
            
            // Pad rest of line
            for col in text.len()..38 {
                if let Some(cell) = buf.get_mut(area.x + col as u16, area.y + item_start + idx as u16) {
                    cell.symbol = " ".to_string();
                }
            }
            
            let border = if col < 38 { " " } else { "║" };
            if let Some(cell) = buf.get_mut(area.x + 38, area.y + item_start + idx as u16) {
                cell.symbol = "║".to_string();
            }
        }

        // Input area
        // ... (similar pattern)
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if let Some(add_msg) = msg.downcast_ref::<AddTodo>() {
            self.state.add(add_msg.0.clone());
            self.state.input.clear();
            Cmd::Render
        } else if let Some(remove_msg) = msg.downcast_ref::<RemoveTodo>() {
            self.state.remove(remove_msg.0);
            Cmd::Render
        } else if let Some(toggle_msg) = msg.downcast_ref::<ToggleTodo>() {
            self.state.toggle(toggle_msg.0);
            Cmd::Render
        } else if let Some(input_msg) = msg.downcast_ref::<SetInput>() {
            self.state.input = input_msg.0.clone();
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

fn main() {
    let mut app = TodoApp::new();
    app.on_mount();

    // Add initial todos
    app.update(Box::new(AddTodo("Learn cTUI framework".into())));
    app.update(Box::new(AddTodo("Build awesome TUI apps".into())));
    app.update(Box::new(AddTodo("Contribute to open source".into())));

    // Toggle first item
    app.update(Box::new(ToggleTodo(0)));

    println!("Todo App Example");
    println!("================\n");

    for (idx, item) in app.state.items.iter().enumerate() {
        let check = if item.completed { "[x]" } else { "[ ]" };
        println!("{}. {} {}", idx + 1, check, item.text);
    }

    app.on_unmount();
}
```

## Key Concepts

### State Management

The app maintains state in a struct:

```rust
struct TodoState {
    items: Vec<TodoItem>,
    next_id: usize,
    input: String,
}
```

### CRUD Operations

| Operation | Message | Action |
|-----------|---------|--------|
| Create | `AddTodo(text)` | Add new item |
| Read | Render | Display items |
| Update | `ToggleTodo(id)` | Toggle completed |
| Delete | `RemoveTodo(id)` | Remove item |

### Message Dispatch

```rust
fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
    if let Some(add) = msg.downcast_ref::<AddTodo>() {
        // Handle add
    }
    // ...
}
```

## Enhancements

### Add Filtering

```rust
enum Filter {
    All,
    Active,
    Completed,
}

impl TodoState {
    fn filtered_items(&self, filter: Filter) -> Vec<&TodoItem> {
        match filter {
            Filter::All => self.items.iter().collect(),
            Filter::Active => self.items.iter().filter(|i| !i.completed).collect(),
            Filter::Completed => self.items.iter().filter(|i| i.completed).collect(),
        }
    }
}
```

### Add Persistence

```rust
impl TodoState {
    fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string(&self.items)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load(&mut self, path: &Path) -> Result<()> {
        let json = fs::read_to_string(path)?;
        self.items = serde_json::from_str(&json)?;
        Ok(())
    }
}
```

### Add Keyboard Navigation

```rust
fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.selected = Some((self.selected.unwrap_or(0) + 1).min(self.items.len() - 1));
                Some(Box::new(SelectChanged))
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.selected = Some(self.selected.unwrap_or(0).saturating_sub(1));
                Some(Box::new(SelectChanged))
            }
            KeyCode::Char(' ') => {
                if let Some(idx) = self.selected {
                    Some(Box::new(ToggleTodo(self.items[idx].id)))
                } else {
                    None
                }
            }
            KeyCode::Char('d') => {
                if let Some(idx) = self.selected {
                    Some(Box::new(RemoveTodo(self.items[idx].id)))
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    }
}
```

## Run the Example

```bash
cargo run --example todo
```

## See Also

- [Counter](counter.md) - Simpler example
- [Dashboard](dashboard.md) - Data visualization
- [List Component](../gallery/list.md) - List documentation
