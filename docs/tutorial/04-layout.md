# Tutorial 04: Layout System

Arrange components with flexible layouts.

## Goals

- Use Layout to split areas
- Apply constraints
- Create responsive layouts

## Layout Basics

```rust
use ctui_layout::{Layout, FlexDirection, Constraint};
use ctui_core::Rect;

let area = Rect::new(0, 0, 80, 24);

// Create a layout
let layout = Layout::flex()
    .direction(FlexDirection::Column);

// Split into regions
let rects = layout.split(area, &[
    Constraint::Length(3),   // Header: fixed 3 rows
    Constraint::Min(10),     // Content: flexible
    Constraint::Length(1),   // Footer: fixed 1 row
]);

// rects[0] = header area
// rects[1] = content area
// rects[2] = footer area
```

## FlexDirection

```rust
pub enum FlexDirection {
    Row,            // Horizontal, left to right
    RowReverse,     // Horizontal, right to left
    Column,         // Vertical, top to bottom
    ColumnReverse,  // Vertical, bottom to top
}
```

## Constraints

```rust
// Fixed size
Constraint::Length(20)      // Exactly 20 units

// Flexible
Constraint::Min(10)         // Minimum 10, can grow
Constraint::Max(50)         // Maximum 50, can shrink

// Percentage
Constraint::Percentage(25)  // 25% of available space

// Ratio
Constraint::Ratio(1, 3)     // 1/3 of space
Constraint::Ratio(2, 3)     // 2/3 of space

// Fill
Constraint::Fill(1)         // Fill with weight 1
Constraint::Fill(2)         // Fill with weight 2
```

## Example: Split Panel

```rust
use ctui_layout::{Layout, FlexDirection, Constraint};
use ctui_core::{Buffer, Component, Rect};

struct SplitPanel {
    sidebar_width: u16,
}

impl Component for SplitPanel {
    type Props = u16;
    type State = ();

    fn create(width: Self::Props) -> Self {
        Self { sidebar_width: width }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Horizontal split: sidebar | main
        let layout = Layout::flex()
            .direction(FlexDirection::Row);

        let rects = layout.split(area, &[
            Constraint::Length(self.sidebar_width),
            Constraint::Min(0),
        ]);

        // Render sidebar
        self.render_sidebar(rects[0], buf);

        // Render main content
        self.render_main(rects[1], buf);
    }
}
```

## Nested Layouts

Combine multiple layouts:

```rust
fn render(&self, area: Rect, buf: &mut Buffer) {
    // First split: header / body / footer
    let main_layout = Layout::flex()
        .direction(FlexDirection::Column);

    let main = main_layout.split(area, &[
        Constraint::Length(3),   // Header
        Constraint::Min(0),      // Body
        Constraint::Length(1),    // Footer
    ]);

    // Render header
    self.header.render(main[0], buf);

    // Split body: sidebar / content
    let body_layout = Layout::flex()
        .direction(FlexDirection::Row)
        .gap(1);

    let body = body_layout.split(main[1], &[
        Constraint::Length(20),  // Sidebar
        Constraint::Min(0),      // Content
    ]);

    // Render sidebar and content
    self.sidebar.render(body[0], buf);
    self.content.render(body[1], buf);

    // Render footer
    self.footer.render(main[2], buf);
}
```

Render Output:

```
┌────────────────────────────────────────────────────────────────┐
│                           Header                                │
├──────────────┬─────────────────────────────────────────────────┤
│              │                                                 │
│   Sidebar    │              Content                            │
│              │                                                 │
│              │                                                 │
│              │                                                 │
│              │                                                 │
├──────────────┴─────────────────────────────────────────────────┤
│ q: quit | h: help | ?: about                                   │
└────────────────────────────────────────────────────────────────┘
```

## Gap

Add spacing between items:

```rust
let layout = Layout::flex()
    .direction(FlexDirection::Row)
    .gap(2);  // 2 column gap

// Or separate horizontal/vertical
let layout = Layout::flex()
    .gap_row(1)     // Row gap
    .gap_column(2); // Column gap
```

## JustifyContent

Alignment along the main axis:

```rust
use ctui_layout::JustifyContent;

let layout = Layout::flex()
    .justify_content(JustifyContent::Center);
```

Options:

```rust
JustifyContent::Start         // Pack at start
JustifyContent::End           // Pack at end
JustifyContent::Center        // Center items
JustifyContent::SpaceBetween  // Equal space between
JustifyContent::SpaceAround   // Equal space around
JustifyContent::SpaceEvenly   // Equal space everywhere
```

## AlignItems

Alignment along the cross axis:

```rust
use ctui_layout::AlignItems;

let layout = Layout::flex()
    .align_items(AlignItems::Center);
```

Options:

```rust
AlignItems::Start     // Top/left
AlignItems::End       // Bottom/right
AlignItems::Center    // Center
AlignItems::Stretch   // Stretch to fill
```

## Grid Layout

For complex arrangements:

```rust
use ctui_layout::{Grid, GridTrack};

let grid = Grid::new()
    .columns(vec![
        GridTrack::length(20),
        GridTrack::flex(1),
        GridTrack::length(20),
    ])
    .rows(vec![
        GridTrack::auto(),    // Header
        GridTrack::flex(1),   // Content
        GridTrack::auto(),    // Footer
    ])
    .gap(1);

let cells = grid.layout(area);
// cells[row * num_columns + column]
```

## Responsive Layout

Adapt to terminal size:

```rust
fn render(&self, area: Rect, buf: &mut Buffer) {
    if area.width < 60 {
        // Narrow layout: vertical stack
        self.render_narrow(area, buf);
    } else {
        // Wide layout: horizontal split
        self.render_wide(area, buf);
    }
}

fn render_narrow(&self, area: Rect, buf: &mut Buffer) {
    let layout = Layout::flex()
        .direction(FlexDirection::Column);
    
    let rects = layout.split(area, &[
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ]);
    // ...
}

fn render_wide(&self, area: Rect, buf: &mut Buffer) {
    let layout = Layout::flex()
        .direction(FlexDirection::Row);
    
    let rects = layout.split(area, &[
        Constraint::Length(20),
        Constraint::Min(0),
    ]);
    // ...
}
```

## Exercise

Create a layout with:

1. Header (2 rows)
2. Two-column body (sidebar: 25%, main: 75%)
3. Footer (1 row)

## Solution

```rust
use ctui_layout::{Layout, FlexDirection, Constraint};

fn render(&self, area: Rect, buf: &mut Buffer) {
    // Main vertical layout
    let main = Layout::flex()
        .direction(FlexDirection::Column)
        .split(area, &[
            Constraint::Length(2),  // Header
            Constraint::Min(0),    // Body
            Constraint::Length(1), // Footer
        ]);

    // Header
    self.header.render(main[0], buf);

    // Body: horizontal split
    let body = Layout::flex()
        .direction(FlexDirection::Row)
        .split(main[1], &[
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ]);

    self.sidebar.render(body[0], buf);
    self.content.render(body[1], buf);

    // Footer
    self.footer.render(main[2], buf);
}
```

## Next Steps

Continue to [Tutorial 05: Styling](05-styling.md) to learn about colors and modifiers.

## See Also

- [Layout API](../api/layout.md)
- [Constraint Types](../api/layout.md#constraints)
