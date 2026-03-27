# cTUI Architecture

This document explains the crate organization, design decisions, and how the different parts of cTUI work together.

## Overview

cTUI is organized as a Cargo workspace with multiple specialized crates. This modular architecture provides:

- Clear separation of concerns
- Faster compilation for dependent projects
- Flexible dependency management
- Better API stability for the core types

## Crate Organization

### Core Crates

#### `ctui-core`

The foundation of cTUI. Contains low-level primitives for terminal rendering.

**Contents:**
- Buffer and Cell types for screen representation
- Backend trait for terminal abstraction
- Terminal struct for managing terminal state
- Geometry primitives (`Rect`, `Position`, `Size`)
- Style types (`Color`, `Modifier`, `Style`)
- Component trait for declarative UI elements
- Props system for component configuration
- State management with dispatch pattern
- Event system for input handling

**When to use:**
- Building custom widget libraries
- Creating minimal applications
- Extending cTUI's capabilities

```rust
use ctui_core::{
    Buffer, Cell, Rect, Style, Color,
    Component, Event, Terminal,
};
```

#### `ctui-components`

Built-in widget library with a comprehensive set of pre-built components.

**Contents:**
- Block, Paragraph, Text, Line
- Input, Form, Editor
- List, Table, Tree
- Tabs, Modal, Scrollable
- ProgressBar, Spinner, Gauge
- Chart, Sparkline, Canvas
- Checkbox, Radio, Select, Slider
- Markdown, Code, Diff

**When to use:**
- Building standard TUI applications
- Need ready-to-use widgets
- Prototyping quickly

```rust
use ctui_components::{
    Block, Paragraph, List, Table,
    Button, Input, ProgressBar,
};
```

#### `ctui-layout`

Flexbox-inspired layout engine for terminal UIs.

**Contents:**
- Flex direction (row, column)
- Justify content (main axis alignment)
- Align items (cross axis alignment)
- Gap between children
- Grid layout with rows/columns
- Absolute positioning with z-index
- Layout validation

**When to use:**
- Complex layouts
- Responsive designs
- Custom layout algorithms

```rust
use ctui_layout::{
    Layout, FlexDirection, JustifyContent,
    AlignItems, Constraint, Grid,
};
```

#### `ctui-animate`

Animation primitives and easing functions.

**Contents:**
- Easing functions (linear, ease-in, ease-out, elastic, bounce)
- Interpolation primitives for value blending
- Keyframe animations with playback modes
- Spring physics for natural motion
- Animation sequences and groups
- Global animation management

**When to use:**
- Creating animated UIs
- Smooth transitions
- Interactive feedback

```rust
use ctui_animate::{
    EasingFunction, KeyframeAnimation,
    PlaybackMode, SpringAnimation,
};
```

#### `ctui-theme`

Theming system with built-in presets.

**Contents:**
- Color types (Named, Indexed, RGB)
- Style properties
- Theme tokens
- Component themes
- Elevation and shadow system
- Built-in themes (Dark, Light, Tokyo Night, Dracula, Catppuccin, Nord, Gruvbox)
- Theme validation and accessibility checking

**When to use:**
- Customizing app appearance
- Supporting dark/light modes
- Ensuring accessibility

```rust
use ctui_theme::{
    Theme, Style, Color, Modifier,
    ThemeLoader, AccessibilityAudit,
};

let dark = Theme::dark();
let dracula = Theme::dracula();
let tokyo = Theme::tokyo_night();
```

#### `ctui-macros`

Procedural macros for reducing boilerplate.

**Contents:**
- `#[component]` attribute macro

**What it generates:**
- Original struct (unchanged)
- Props struct with same fields
- Component implementation with defaults

```rust
use ctui_macros::component;

#[component]
struct Button {
    label: String,
    #[prop(default = false)]
    disabled: bool,
}
// Generates ButtonProps and Component impl
```

### Utility Crates

#### `ctui-cli`

CLI tool and project generator.

**Contents:**
- Project scaffolding
- Template system (basic, counter, todo-app)
- Cargo integration

```bash
cargo install ctui-cli
ctui new my-app --template counter
```

#### `ctui-tests`

Integration tests and test utilities.

**Contents:**
- Test harness
- Snapshot testing helpers
- Mock backends

## Dependency Graph

```
ctui (main crate - re-exports everything)
├── ctui-core (foundation)
├── ctui-components → ctui-core
├── ctui-layout → ctui-core
├── ctui-animate → ctui-core
├── ctui-theme → ctui-core
├── ctui-macros (proc-macro crate)
└── ctui-cli
```

### Key Dependencies

- **ctui-core**: Foundation for all other crates
- **ctui-components**: Depends on core for widget traits
- **ctui-layout**: Depends on core for geometry types
- **ctui-animate**: Depends on core for timing
- **ctui-theme**: Depends on core for style types

## Design Decisions

### Declarative Component Model

cTUI uses a React-style component model:

- Components are functions that return UI descriptions
- State is managed through hooks (`use_state`, `use_effect`, `use_async`)
- Props flow down, events flow up
- Diff algorithm updates only what changed

### Zero-Allocation Diff

The diff algorithm compares virtual DOM trees directly:

- No intermediate allocations during diffing
- Cell-based updates only for changed cells
- Batched event processing
- Lazy component re-rendering

### Async-First Design

Built on Tokio for async operations:

- Non-blocking event handling
- Seamless async integration
- Thread-safe state management
- Works with any async runtime

### Type Safety

Leverage Rust's type system:

- Props are strongly typed
- Component state is type-checked
- Events are typed messages
- Compile-time error detection

## Public API Surface

### Core Types (Stable)

These types are guaranteed to remain stable:

- `Buffer`, `Cell`
- `Rect`, `Position`, `Size`
- `Style`, `Color`, `Modifier`
- `Event`, `KeyEvent`, `MouseEvent`
- `Component` trait

### Extended API (Evolving)

These may change between minor versions:

- Individual widget implementations
- Animation easing functions
- Theme format
- CLI commands

### Experimental API (Unstable)

Behind feature flags:

- `#[component]` macro extensions
- WebAssembly support
- Plugin system

## Contributing to Architecture

When adding new features:

1. **Core primitives** → `ctui-core`
2. **New widgets** → `ctui-components`
3. **Layout features** → `ctui-layout`
4. **Animations** → `ctui-animate`
5. **Theme support** → `ctui-theme`
6. **Developer tools** → `ctui-cli`

## Migration Guide

### From Imperative TUI Libraries

If you're coming from ratatui or similar:

```rust
// Ratatui (imperative)
fn ui(f: &mut Frame, app: &App) {
    let block = Block::default().title("Counter");
    f.render_widget(block, f.size());
}

// cTUI (declarative)
#[component]
fn Counter() -> impl Component {
    let (count, set_count) = use_state(0);
    Column::new()
        .child(Text::new(&format!("Count: {}", count)))
        .child(Button::new("+").on_click(move || set_count(count + 1)))
}
```

### From Other Declarative Libraries

If you're familiar with React patterns:

- `use_state` → React's `useState`
- `use_effect` → React's `useEffect`
- `use_async` → Custom hook for async
- `Component` trait → React component
- Props → React props

## Future Considerations

- Independent crate versioning
- WebAssembly support
- Plugin system
- Additional themes
- More animation primitives
