# Examples

Complete example applications demonstrating cTUI features.

## Available Examples

| Example | Description |
|---------|-------------|
| [Counter](counter.md) | Minimal stateful app |
| [Todo App](todo.md) | Full CRUD application |
| [Dashboard](dashboard.md) | Real-time data visualization |
| [Editor](editor.md) | Text editor with syntax highlighting |
| [Game](game.md) | Simple terminal game |

## Running Examples

```bash
# Run a specific example
cargo run --example counter
cargo run --example todo
cargo run --example dashboard

# Run with features
cargo run --example dashboard --features dynamic-assets

# Run with release optimizations
cargo run --release --example game
```

## Example Structure

All examples follow a similar structure:

```
examples/
├── counter.rs      # Single file example
├── todo.rs         # Medium complexity
├── dashboard.rs    # Multiple components
├── editor.rs       # Full-featured app
└── game.rs         # Game loop pattern
```

## Example Patterns

### Basic Component

```rust
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

struct MyComponent {
    // Component state
}

impl Component for MyComponent {
    type Props = ();
    type State = ();

    fn create(_: Self::Props) -> Self {
        Self { /* initial state */ }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render to buffer
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        // Handle messages
        Cmd::Render
    }
}
```

### Application Pattern

```rust
struct App {
    // Application state
}

impl App {
    fn run(&mut self) -> Result<()> {
        // Initialize terminal
        // Enter main loop
        // Handle events
        // Render
        // Cleanup
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut app = App::new();
    app.run()
}
```

## Common Imports

Most examples use these imports:

```rust
// Core types
use ctui_core::{
    Buffer, Cell, Rect, Position, Size,
    Component, Msg, Cmd, State, Store,
    Event, KeyEvent, KeyCode, MouseEvent,
    Style, Color, Modifier,
};

// Components
use ctui_components::{
    Block, Paragraph, Text, Line, Span,
    List, ListItem, Input, Table, Tree,
    ProgressBar, Spinner, Gauge,
    Tabs, Modal, Scrollable,
};

// Layout
use ctui_layout::{
    Layout, FlexDirection, JustifyContent, Constraint,
    Grid, GridTrack,
};

// Animation
use ctui_animate::{
    EasingFunction, Keyframe, KeyframeAnimation,
};

// Theme
use ctui_theme::{Theme, Style, Color};
```

## Project Templates

Use the CLI to generate a new project:

```bash
ctui new my-app
cd my-app
cargo run
```

This generates:

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── app.rs
│   └── components/
│       └── mod.rs
└── .github/
    └── workflows/
        └── ci.yml
```

## See Also

- [Getting Started](../getting-started.md)
- [Tutorial](../tutorial/README.md)
- [API Reference](../api/README.md)
