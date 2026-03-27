# API Reference

Complete API documentation for the cTUI framework.

## Modules

| Module | Description |
|--------|-------------|
| [Core](core.md) | Buffer, Backend, Terminal, Component, State, Events |
| [Layout](layout.md) | Flex, Grid, Constraints |
| [Components](components.md) | All built-in widgets |
| [Animation](animation.md) | Easing, Keyframes, Springs |
| [Theme](theme.md) | Colors, Styles, Presets |

## Quick Reference

### Core Types

```rust
use ctui_core::{
    Buffer, Cell, Rect, Size, Position,
    Component, Msg, Cmd, State, Store,
    Event, KeyEvent, MouseEvent,
    Style, Color, Modifier,
    Terminal, Backend,
};
```

### Layout Types

```rust
use ctui_layout::{
    Layout, FlexDirection, JustifyContent, AlignItems,
    Constraint, Grid, GridTrack,
    AbsoluteLayout, ZIndex, Margin,
};
```

### Component Types

```rust
use ctui_components::{
    Widget,
    Block, Paragraph, Text, Line, Span,
    List, ListItem, Input, Table, Tree,
    ProgressBar, Spinner, Gauge, Chart,
    Tabs, Modal, Scrollable, Form,
};
```

### Animation Types

```rust
use ctui_animate::{
    EasingFunction, Keyframe, KeyframeAnimation, PlaybackMode,
    SpringAnimation, SpringConfig,
    Transition, TransitionBuilder,
    AnimationScheduler, AnimationManager,
};
```

### Theme Types

```rust
use ctui_theme::{
    Theme, ColorPalette, Style, Spacing, BorderStyle,
    Color, Modifier, Typography, FontWeight,
    ThemeLoader, ThemeValidator,
};
```

## Re-exports

The `ctui` crate re-exports common types:

```rust
// ctui = { path = "." } in Cargo.toml

pub use ctui_core::*;
pub use ctui_layout::*;
pub use ctui_components::*;
pub use ctui_animate::*;
pub use ctui_theme::*;
```
