# Getting Started

This guide will help you get up and running with cTUI in minutes.

## Prerequisites

- Rust 1.75 or later
- A terminal that supports ANSI escape codes

## Installation

Add cTUI to your project:

```bash
cargo add ctui
```

Or manually add to your `Cargo.toml`:

```toml
[dependencies]
ctui = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Project Structure

A typical cTUI project looks like this:

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Main app component
│   ├── components/      # Custom components
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   └── sidebar.rs
│   └── state.rs         # State management
```

## Your First App

### Minimal Example

Create a simple "Hello, World" app:

```rust
// src/main.rs
use ctui_core::{Buffer, Cmd, Component, Msg, Rect, Terminal};

struct HelloWorld;

impl Component for HelloWorld {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        Self
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = "Hello, cTUI!";
        for (i, ch) in text.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }

    fn update(&mut self, _msg: Box<dyn Msg>) -> Cmd {
        Cmd::Noop
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For testing: print and exit
    let mut buf = Buffer::empty(Rect::new(0, 0, 80, 24));
    let app = HelloWorld;
    app.render(buf.area, &mut buf);
    
    // Print the buffer content
    for y in 0..24 {
        let line: String = (0..80)
            .map(|x| buf[(x, y)].symbol.clone())
            .collect();
        println!("{}", line.trim_end());
    }
    
    Ok(())
}
```

Run with:

```bash
cargo run
```

### Counter App

A more complete example with state:

```rust
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

// Define messages for state updates
struct Increment;
struct Decrement;

impl Msg for Increment {}
impl Msg for Decrement {}

struct Counter {
    count: i32,
}

impl Counter {
    fn new(initial: i32) -> Self {
        Self { count: initial }
    }
}

impl Component for Counter {
    type Props = i32;
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self::new(props)
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("Counter: {}  [+] [-]", self.count);
        for (i, ch) in text.chars().take(area.width as usize).enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if msg.is::<Increment>() {
            self.count += 1;
            Cmd::Render
        } else if msg.is::<Decrement>() {
            self.count -= 1;
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

fn main() {
    let mut counter = Counter::new(0);
    counter.on_mount();

    println!("Counter Example");
    println!("==============\n");

    // Initial state
    println!("Initial: {}", counter.count);

    // Increment
    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    println!("After +2: {}", counter.count);

    // Decrement
    counter.update(Box::new(Decrement));
    println!("After -1: {}", counter.count);

    counter.on_unmount();
}
```

## Using Components

cTUI comes with many built-in components. Here is how to use them:

### Block - Container Widget

```rust
use ctui_components::{Block, Borders, BorderType};
use ctui_core::Rect;

let block = Block::new()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .title("My Panel");

// Renders as:
// ╭ My Panel ──────────╮
// │                    │
// │                    │
// ╰────────────────────╯
```

### Paragraph - Text Display

```rust
use ctui_components::{Paragraph, Text, Alignment};

let paragraph = Paragraph::new(Text::from("Hello,\nWorld!"))
    .alignment(Alignment::Center);

// Renders as:
//    
//   Hello,
//   World!
//    
```

### List - Selectable Items

```rust
use ctui_components::{List, ListItem, SelectionMode};

let list = List::new(vec![
    ListItem::new("Option 1"),
    ListItem::new("Option 2"),
    ListItem::new("Option 3"),
]).selection_mode(SelectionMode::Single);

// Renders as:
// > Option 1
//   Option 2
//   Option 3
```

### Input - Text Entry

```rust
use ctui_components::{Input, InputProps};

let input = Input::new()
    .value("Type here...")
    .placeholder("Enter text...");

// Renders as:
// Type here...|_
```

## Layout System

cTUI provides flexbox-inspired layouts:

### Flex Layout

```rust
use ctui_layout::{Layout, FlexDirection, JustifyContent, Constraint};
use ctui_core::Rect;

let area = Rect::new(0, 0, 80, 24);

let layout = Layout::flex()
    .direction(FlexDirection::Column)
    .gap(1);

let constraints = vec![
    Constraint::Length(3),   // Fixed height header
    Constraint::Min(10),     // Flexible content
    Constraint::Length(3),   // Fixed height footer
];

let rects = layout.split(area, &constraints);

// rects[0] - Header (height: 3)
// rects[1] - Content (height: remaining)
// rects[2] - Footer (height: 3)
```

### Grid Layout

```rust
use ctui_layout::{Grid, GridTrack};

let grid = Grid::new()
    .columns(vec![
        GridTrack::auto(),    // Column 1: auto-width
        GridTrack::flex(1),   // Column 2: flexible
        GridTrack::length(20), // Column 3: fixed
    ])
    .rows(vec![
        GridTrack::auto(),
        GridTrack::flex(1),
    ]);

let cells = grid.layout(area);
```

## Events and Input

Handle keyboard and mouse input:

```rust
use ctui_core::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseButton};

impl Component for MyComponent {
    fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
        match event {
            Event::Key(key) => self.handle_key(key),
            Event::Mouse(mouse) => self.handle_mouse(mouse),
            Event::Resize(size) => self.handle_resize(size),
            _ => None,
        }
    }
}

fn handle_key(&mut self, key: &KeyEvent) -> Option<Box<dyn Msg>> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) => Some(Box::new(Quit)),
        (KeyCode::Up, _) => Some(Box::new(NavigateUp)),
        (KeyCode::Enter, _) => Some(Box::new(Select)),
        _ => None,
    }
}
```

## Styling

Apply colors and modifiers:

```rust
use ctui_core::{Color, Modifier, Style};
use ctui_components::{Block, Borders};

let style = Style::new()
    .fg(Color::Cyan)
    .bg(Color::Black)
    .add_modifier(Modifier::BOLD);

let block = Block::new()
    .borders(Borders::ALL)
    .style(style);
```

### Color Options

```rust
// Named colors
Color::Red
Color::Green
Color::Blue
Color::Cyan
Color::Magenta
Color::Yellow
Color::White
Color::Black
Color::Gray

// 256-color palette
Color::Indexed(196)  // Bright red

// RGB colors (true color)
Color::Rgb(255, 100, 50)

// Transparent
Color::Reset
```

## Animations

Add smooth animations:

```rust
use ctui_animate::{EasingFunction, Keyframe, KeyframeAnimation, PlaybackMode};

let mut animation = KeyframeAnimation::new()
    .keyframe(Keyframe::new(0.0, 0.0))
    .keyframe(Keyframe::new(0.5, 50.0))
    .keyframe(Keyframe::new(1.0, 100.0))
    .duration_ms(1000)
    .easing(EasingFunction::EaseOutCubic)
    .playback_mode(PlaybackMode::Once);

// In your update loop:
animation.tick(delta_ms);
let current_value = animation.current_value();
```

### Easing Functions

```rust
// Available easing functions
EasingFunction::Linear
EasingFunction::EaseIn
EasingFunction::EaseOut
EasingFunction::EaseInOut
EasingFunction::EaseInCubic
EasingFunction::EaseOutCubic
EasingFunction::EaseInOutCubic
EasingFunction::EaseInElastic
EasingFunction::EaseOutElastic
EasingFunction::EaseOutBounce
```

## Testing

Use `TestBackend` for unit testing:

```rust
use ctui_core::backend::test::TestBackend;
use ctui_core::{Buffer, Cell, Rect};

#[test]
fn test_my_widget() {
    let mut backend = TestBackend::new(20, 10);
    
    // Render your widget
    backend.draw(vec![
        (0, 0, &Cell::new("H")),
        (1, 0, &Cell::new("i")),
    ].into_iter()).unwrap();
    
    // Assert on the output
    assert_eq!(backend[(0, 0)].symbol, "H");
    assert_eq!(backend[(1, 0)].symbol, "i");
    
    // Or use snapshot-style assertions
    backend.assert_buffer_lines(["Hi                  ", "                    "]);
}
```

## Next Steps

1. **[Component Gallery](gallery/README.md)** - Explore all available components
2. **[API Reference](api/README.md)** - Deep dive into the API
3. **[Examples](examples/README.md)** - Learn from complete applications
4. **[Tutorial](tutorial/README.md)** - Build a real app step by step

## Common Patterns

### Stateful Component

```rust
struct MyComponent {
    data: Vec<String>,
    selected: Option<usize>,
}

impl Component for MyComponent {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        Self {
            data: Vec::new(),
            selected: None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render your component
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        // Handle messages and update state
        Cmd::Render
    }
}
```

### Async Data Loading

```rust
use tokio::sync::mpsc;

struct DataLoader {
    data: Option<Vec<String>>,
    loading: bool,
}

impl DataLoader {
    async fn load_data(tx: mpsc::Sender<Box<dyn Msg>>) {
        let data = fetch_from_api().await;
        let _ = tx.send(Box::new(DataLoaded(data))).await;
    }
}
```

### Component Composition

```rust
impl Component for Dashboard {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::flex()
            .direction(FlexDirection::Column)
            .split(area, &[
                Constraint::Length(3),
                Constraint::Min(10),
            ]);

        // Render header in top area
        self.header.render(layout[0], buf);
        
        // Render body in remaining area
        self.body.render(layout[1], buf);
    }
}
```

## Troubleshooting

### Terminal Not Displaying Correctly

Ensure your terminal supports ANSI escape codes. Most modern terminals work fine.

### Performance Issues

Use `#[inline]` on hot paths and avoid allocations in render loops. See [Performance Guide](performance.md).

### Component Not Rendering

Check that your component returns the correct area and that you're calling `render` with valid coordinates.
