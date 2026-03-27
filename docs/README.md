# cTUI Documentation

Welcome to the cTUI documentation. cTUI is a high-performance TUI framework in Rust with React-style declarative components.

## Table of Contents

- [Getting Started](getting-started.md) - Installation, quick start, and first app tutorial
- [Component Gallery](gallery/README.md) - Visual reference for all components
- [API Reference](api/README.md) - Detailed API documentation
- [Examples](examples/README.md) - Complete example applications
- [Tutorial](tutorial/README.md) - Step-by-step learning path
- [Performance Guide](performance.md) - Optimization techniques and benchmarks
- [Migration Guide](migration.md) - Migrating from other frameworks

## Quick Links

### Core Concepts

| Topic | Description |
|-------|-------------|
| [Component Model](api/core.md#component) | Declarative UI components |
| [State Management](api/core.md#state) | Managing application state |
| [Buffer & Rendering](api/core.md#buffer) | Low-level rendering primitives |
| [Events](api/core.md#events) | Input handling and event system |

### Layout

| Topic | Description |
|-------|-------------|
| [Flex Layout](api/layout.md#flex) | Flexbox-inspired layout system |
| [Grid Layout](api/layout.md#grid) | CSS Grid-like layout |
| [Constraints](api/layout.md#constraints) | Sizing constraints |

### Components

| Component | Description |
|-----------|-------------|
| [Block](gallery/block.md) | Container with borders and titles |
| [Paragraph](gallery/paragraph.md) | Multi-line text |
| [List](gallery/list.md) | Scrollable list with selection |
| [Table](gallery/table.md) | Tabular data display |
| [Input](gallery/input.md) | Text input field |
| [See all...](gallery/README.md) | Full component list |

### Advanced Topics

| Topic | Description |
|-------|-------------|
| [Animation](api/animation.md) | Easing, transitions, keyframes |
| [Theming](api/theme.md) | Colors, styles, presets |
| [Performance](performance.md) | Optimization techniques |

## Installation

Add cTUI to your `Cargo.toml`:

```toml
[dependencies]
ctui = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Hello World

```rust
use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

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
```

Run with:

```bash
cargo run --example minimal
```

## Architecture

```
cTUI Workspace
├── ctui-core/        # Core rendering engine
│   ├── buffer/       # Screen buffer and diff algorithm
│   ├── backend/      # Terminal backends (Crossterm, Test)
│   ├── component.rs  # Component trait definition
│   ├── state.rs      # State management
│   └── event.rs      # Input event handling
│
├── ctui-components/  # Built-in widget library
│   ├── block.rs      # Container widgets
│   ├── input.rs      # Text input
│   ├── list.rs       # List widgets
│   ├── table.rs      # Table widgets
│   └── ...           # Many more components
│
├── ctui-layout/      # Layout engine
│   ├── flex.rs       # Flexbox layout
│   ├── grid.rs       # Grid layout
│   └── constraint.rs # Sizing constraints
│
├── ctui-animate/     # Animation system
│   ├── easing.rs     # Easing functions
│   ├── keyframe.rs   # Keyframe animations
│   └── spring.rs     # Spring physics
│
├── ctui-theme/       # Theming system
│   ├── theme.rs      # Theme definition
│   ├── color.rs      # Color types
│   └── style.rs      # Style properties
│
└── ctui-cli/         # CLI tooling
```

## Feature Highlights

### Declarative Components

Write UI components like React, not imperative drawing code:

```rust
impl Component for Counter {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("Count: {}", self.count);
        // Render text to buffer
    }
}
```

### Zero-Allocation Diff Rendering

Only update cells that changed:

```
Previous frame:  ████████████████████
Current frame:    ████████████████████
Diff:             (no changes needed)
```

### Built-in Animation

Smooth 60 FPS animations:

```rust
use ctui_animate::{EasingFunction, KeyframeAnimation};

let animation = KeyframeAnimation::new()
    .keyframe(Keyframe::new(0.0, 0.0))
    .keyframe(Keyframe::new(1.0, 100.0))
    .easing(EasingFunction::EaseOutCubic);
```

### Rich Component Library

Pre-built widgets for common use cases:

- Text display: Paragraph, Block, Markdown, Code
- Input: Input, Select, Checkbox, Radio, Slider
- Data: Table, List, Tree, Chart, Sparkline
- Feedback: ProgressBar, Spinner, Gauge
- Layout: Scrollable, Tabs, Modal, Form

## Comparison

| Feature | cTUI | ratatui | cursive |
|---------|------|---------|---------|
| Paradigm | Declarative | Imperative | Declarative |
| Diff rendering | Yes | No | Partial |
| Async support | Yes | Manual | Manual |
| Animation | Built-in | Manual | Basic |
| Layout | Flexbox/Grid | Manual | Stack |
| Rust version | 1.75+ | 1.63+ | 1.60+ |

## Contributing

We welcome contributions! See the main [README](../README.md) for development setup instructions.

## License

Licensed under MIT or Apache-2.0 at your option.
