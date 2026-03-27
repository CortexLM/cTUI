# Migration Guide

Migrate your applications from other TUI frameworks to cTUI.

## From ratatui

ratatui is the most popular Rust TUI framework. Here's how to migrate.

### Conceptual Differences

| ratatui | cTUI |
|---------|------|
| Imperative | Declarative |
| `Frame::render_widget` | `Component::render` |
| Manual state | Message-driven updates |
| No diff | Automatic diff |

### Project Structure

**ratatui:**

```rust
// main.rs
use ratatui::{Terminal, Frame};
use ratatui::widgets::{Paragraph, Block, Borders};

fn main() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(...))?;
    
    loop {
        terminal.draw(|f| {
            let p = Paragraph::new("Hello")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(p, f.size());
        })?;

        // Handle events
        if let Event::Key(key) = read()? {
            // ...
        }
    }
}
```

**cTUI:**

```rust
// main.rs
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};
use ctui_components::{Block, Paragraph, Borders};

struct App;

impl Component for App {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self { Self }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let p = Paragraph::new("Hello")
            .block(Block::new().borders(Borders::ALL));
        p.render(area, buf);
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}
```

### Widget Migration

#### Block

```rust
// ratatui
Block::default()
    .borders(Borders::ALL)
    .title("Title")

// cTUI
Block::new()
    .borders(Borders::ALL)
    .title("Title")
```

#### Paragraph

```rust
// ratatui
Paragraph::new("Text")
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center)

// cTUI
Paragraph::new("Text")
    .block(Block::new().borders(Borders::ALL))
    .alignment(Alignment::Center)
```

#### List

```rust
// ratatui
List::new(vec![
    ListItem::new("Item 1"),
    ListItem::new("Item 2"),
])
.highlight_symbol(">")

// cTUI
List::new(vec![
    ListItem::new("Item 1"),
    ListItem::new("Item 2"),
]).selected(Some(0))
```

#### Table

```rust
// ratatui
Table::new(vec![
    Row::new(vec!["A", "B"]),
    Row::new(vec!["C", "D"]),
].rows)
.header(Row::new(vec!["Col1", "Col2"]))

// cTUI
Table::new()
    .add_column(Column::new("Col1"))
    .add_column(Column::new("Col2"))
    .add_row(Row::from_strings(vec!["A", "B"]))
    .add_row(Row::from_strings(vec!["C", "D"]))
```

### Layout Migration

```rust
// ratatui
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Min(0)])
    .split(f.size());

// cTUI
use ctui_layout::{Layout, FlexDirection, Constraint};

let layout = Layout::flex()
    .direction(FlexDirection::Column);

let rects = layout.split(area, &[
    Constraint::Length(3),
    Constraint::Min(0),
]);
```

### Event Handling

```rust
// ratatui
match read()? {
    Event::Key(key) => match key.code {
        KeyCode::Char('q') => break,
        KeyCode::Up => /* handle */,
        _ => {}
    },
    _ => {}
}

// cTUI
fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Char('q') => Some(Box::new(Quit)),
            KeyCode::Up => Some(Box::new(NavigateUp)),
            _ => None,
        },
        _ => None,
    }
}
```

## From cursive

### Conceptual Differences

| cursive | cTUI |
|---------|------|
| Callbacks | Messages |
| Global state | Component state |
| Views | Components |
| ncurses | Crossterm |

### View Migration

**cursive:**

```rust
let mut siv = Cursive::new();
siv.add_layer(TextView::new("Hello"));
siv.run();
```

**cTUI:**

```rust
struct App;

impl Component for App {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self { Self }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = "Hello";
        for (i, ch) in text.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }
}
```

### Callback Migration

**cursive:**

```rust
siv.add_layer(Button::new("Click", |s| {
    s.add_layer(Dialog::info("Clicked!"));
}));
```

**cTUI:**

```rust
struct Clicked;
impl Msg for Clicked {}

fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
    match event {
        Event::Key(k) if k.code == KeyCode::Enter => {
            Some(Box::new(Clicked))
        }
        _ => None,
    }
}

fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
    if msg.is::<Clicked>() {
        // Show dialog
        Cmd::Render
    } else {
        Cmd::Noop
    }
}
```

## Migration Checklist

- [ ] Update dependencies in `Cargo.toml`
- [ ] Replace widget imports
- [ ] Convert widgets to Component trait
- [ ] Update layout code
- [ ] Convert event handlers to message pattern
- [ ] Test all functionality

## Getting Help

If you encounter issues during migration:

1. Check the [API Reference](api/README.md)
2. Browse the [Component Gallery](gallery/README.md)
3. Look at [Examples](examples/README.md)
4. Open an issue on [GitHub](https://github.com/CortexLM/cTUI/issues)

## See Also

- [Getting Started](getting-started.md)
- [Tutorial](tutorial/README.md)
- [Performance Guide](performance.md)
