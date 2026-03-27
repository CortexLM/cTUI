# Scrollable Component

Wrap any widget in a scrollable region.

## Variants

### Basic Scrollable

**Code:**

```rust
let scrollable = Scrollable::new(content)
    .scroll_y(offset);
```

**Render:**

```
┌─────────────────┐
│ Line 1          │
│ Line 2          │
│ Line 3          │
│ Line 4          │
│ Line 5      ▼   │
└─────────────────┘
```

### Horizontal Scroll

**Code:**

```rust
let scrollable = Scrollable::new(content)
    .scroll_x(offset)
    .scroll_y(0);
```

**Render:**

```
This is a very long line of t...>
```

### With Scrollbars

**Code:**

```rust
let scrollable = Scrollable::new(content)
    .scrollbar_visibility(ScrollbarVisibility::Always);
```

**Render:**

```
┌─────────────│───┐
│ Content     │ ▲ │
│ More        │ █ │
│ Content     │ █ │
│ Here        │ ▼ │
└─────────────│───┘
```

### Nested Scrollables

**Code:**

```rust
let outer = Scrollable::new(
    Scrollable::new(inner_content)
        .scroll_y(inner_offset)
).scroll_y(outer_offset);
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `scroll_x` | `u16` | 0 | Horizontal offset |
| `scroll_y` | `u16` | 0 | Vertical offset |
| `scrollbar_visibility` | `ScrollbarVisibility` | Auto | When to show |
| `scrollbar_style` | `Style` | default | Scrollbar style |

## ScrollbarVisibility

```rust
pub enum ScrollbarVisibility {
    Auto,     // Show when needed
    Always,   // Always visible
    Never,    // Never visible
}
```

## Events

| Event | Action |
|-------|--------|
| `Up` / `k` | Scroll up one line |
| `Down` / `j` | Scroll down one line |
| `PageUp` | Scroll up one page |
| `PageDown` | Scroll down one page |
| `Home` | Scroll to top |
| `End` | Scroll to bottom |
| `MouseScroll` | Scroll with mouse |

## Example

```rust
use ctui_components::{Scrollable, Paragraph, Text};

fn render(&self, area: Rect, buf: &mut Buffer) {
    let long_text = Text::from("Line 1\nLine 2\n...\nLine 100");
    let paragraph = Paragraph::new(long_text);
    
    let scrollable = Scrollable::new(paragraph)
        .scroll_y(self.scroll_offset)
        .scrollbar_visibility(ScrollbarVisibility::Auto);
    
    scrollable.render(area, buf);
}

fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
    if msg.is::<ScrollDown>() {
        self.scroll_offset += 1;
        Cmd::Render
    } else {
        Cmd::Noop
    }
}
```

## See Also

- [List](list.md) - Built-in scrolling list
- [Table](table.md) - Tables with scrolling
- [Editor](editor.md) - Text editor with scrolling
