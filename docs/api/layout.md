# API Reference - Layout

Flexbox-inspired layout system for terminal UIs.

## Overview

cTUI's layout system provides flexible, declarative layouts similar to CSS Flexbox and Grid.

```rust
use ctui_layout::{Layout, FlexDirection, JustifyContent, AlignItems, Constraint, Grid};
use ctui_core::Rect;

let area = Rect::new(0, 0, 80, 24);

// Create a flex layout
let layout = Layout::flex()
    .direction(FlexDirection::Column)
    .gap(1);

// Split area into constraints
let rects = layout.split(area, &[
    Constraint::Length(3),
    Constraint::Min(10),
    Constraint::Length(3),
]);
```

---

## Flex Layout

Create flexible layouts with the Flexbox model.

### Layout

```rust
use ctui_layout::{Layout, FlexDirection, JustifyContent, AlignItems};

let layout = Layout::flex()
    .direction(FlexDirection::Row)      // Horizontal
    .justify_content(JustifyContent::SpaceBetween)
    .align_items(AlignItems::Center)
    .gap(1);                             // Gap between items
```

### FlexDirection

```rust
pub enum FlexDirection {
    Row,            // Left to right
    RowReverse,     // Right to left
    Column,         // Top to bottom
    ColumnReverse,  // Bottom to top
}
```

### JustifyContent

Controls alignment along the main axis.

```rust
pub enum JustifyContent {
    Start,          // Pack items at start
    End,            // Pack items at end
    Center,         // Pack items at center
    SpaceBetween,   // Equal space between items
    SpaceAround,    // Equal space around items
    SpaceEvenly,    // Equal space everywhere
}
```

Examples:

```
Start:      [item] [item] [item]                   _
End:        _                   [item] [item] [item]
Center:     _        [item] [item] [item]         _
SpaceBetween: [item]       [item]       [item]
SpaceAround:   [item]     [item]     [item]
```

### AlignItems

Controls alignment along the cross axis.

```rust
pub enum AlignItems {
    Start,      // Align to top (column) or left (row)
    End,        // Align to bottom (column) or right (row)
    Center,     // Center items
    Stretch,     // Stretch to fill
    Baseline,   // Align text baselines
}
```

### Gap

```rust
let layout = Layout::flex()
    .gap(1)           // Single value
    .gap_row(2)       // Row gap only
    .gap_column(1);   // Column gap only
```

### Split Areas

```rust
let rects = layout.split(area, &constraints);

// rects[0] - First child
// rects[1] - Second child
// etc.
```

### Examples

#### Header / Content / Footer

```rust
let layout = Layout::flex()
    .direction(FlexDirection::Column);
    .gap(0);

let rects = layout.split(area, &[
    Constraint::Length(3),    // Header: 3 rows
    Constraint::Min(0),        // Content: remaining
    Constraint::Length(3),    // Footer: 3 rows
]);

// Render
header.render(rects[0], buf);
content.render(rects[1], buf);
footer.render(rects[2], buf);
```

#### Sidebar / Main

```rust
let layout = Layout::flex()
    .direction(FlexDirection::Row);

let rects = layout.split(area, &[
    Constraint::Length(20),   // Sidebar: 20 columns
    Constraint::Min(0),        // Main: remaining
]);
```

---

## Constraints

Control how space is allocated.

### Constraint Types

```rust
pub enum Constraint {
    Length(u16),      // Fixed size
    Min(u16),         // Minimum size, can grow
    Max(u16),         // Maximum size, can shrink
    Percentage(u16),  // Percentage of available space
    Ratio(u32, u32),  // Ratio of available space
    Fill(u16),        // Fill remaining space (weight)
}
```

### Examples

```rust
// Fixed width/height
Constraint::Length(20)      // Exactly 20 units

// Minimum size
Constraint::Min(10)          // At least 10, can grow

// Maximum size
Constraint::Max(50)          // At most 50, can shrink

// Percentage
Constraint::Percentage(30)   // 30% of available space

// Ratio
Constraint::Ratio(1, 3)      // 1/3 of available space
Constraint::Ratio(2, 3)      // 2/3 of available space

// Fill with weight
Constraint::Fill(1)          // Fill space with weight 1
Constraint::Fill(2)          // Fill space with weight 2
```

### Common Patterns

#### Equal Columns

```rust
let rects = layout.split(area, &[
    Constraint::Percentage(33),
    Constraint::Percentage(33),
    Constraint::Percentage(34),
]);
```

#### Proportional Layout

```rust
let rects = layout.split(area, &[
    Constraint::Ratio(1, 4),  // 25%
    Constraint::Ratio(3, 4),  // 75%
]);
```

#### Dynamic Sidebar

```rust
let sidebar_width = if sidebar_expanded { 30 } else { 0 };

let rects = layout.split(area, &[
    Constraint::Length(sidebar_width),
    Constraint::Min(0),
]);
```

---

## Grid Layout

CSS Grid-like layout for complex arrangements.

### Grid

```rust
use ctui_layout::{Grid, GridTrack, GridPosition};

let grid = Grid::new()
    .columns(vec![
        GridTrack::auto(),       // Auto-sized
        GridTrack::flex(1),      // Flexible
        GridTrack::length(20),   // Fixed
    ])
    .rows(vec![
        GridTrack::auto(),
        GridTrack::flex(1),
        GridTrack::auto(),
    ])
    .gap(1);

let cells = grid.layout(area);
```

### GridTrack

```rust
pub enum GridTrack {
    Auto,               // Size to content
    Length(u16),        // Fixed size
    Percentage(u16),    // Percentage
    Fractional(u16),    // Fractional unit (fr)
    MinMax(u16, u16),   // Min/Max range
}
```

### GridPosition

```rust
pub struct GridPosition {
    pub row: usize,
    pub column: usize,
    pub row_span: usize,
    pub column_span: usize,
}

// Position a widget
let position = GridPosition::new(0, 0)           // Row 0, Col 0
    .row_span(2)                                 // Span 2 rows
    .column_span(1);                             // Span 1 col
```

### GridAlignment

```rust
pub enum GridAlignment {
    Start,
    End,
    Center,
    Stretch,
}
```

### Example

```rust
let grid = Grid::new()
    .columns(vec![
        GridTrack::length(20),
        GridTrack::flex(1),
        GridTrack::length(20),
    ])
    .rows(vec![
        GridTrack::auto(),        // Header
        GridTrack::flex(1),       // Content
        GridTrack::auto(),        // Footer
    ])
    .column_gap(1)
    .row_gap(0);

let cells = grid.layout(area);

// cells[0] -> Grid cell at row 0, column 0
// cells[1] -> Grid cell at row 0, column 1
// etc.
```

---

## Absolute Layout

Position elements absolutely with z-index.

### AbsoluteLayout

```rust
use ctui_layout::{AbsoluteLayout, AbsoluteItem, ZIndex};

let layout = AbsoluteLayout::new()
    .item(AbsoluteItem::new(widget)
        .position(10, 5)
        .size(30, 10)
        .z_index(ZIndex::Overlay))
    .item(AbsoluteItem::new(another_widget)
        .position(0, 0)
        .z_index(ZIndex::Base));
```

### ZIndex

```rust
pub struct ZIndex(u32);

impl ZIndex {
    pub const BASE: ZIndex = ZIndex(0);
    pub const NORMAL: ZIndex = ZIndex(1);
    pub const ELEVATED: ZIndex = ZIndex(10);
    pub const OVERLAY: ZIndex = ZIndex(100);
    pub const MODAL: ZIndex = ZIndex(1000);
    pub const TOOLTIP: ZIndex = ZIndex(2000);
}
```

### StackingContext

```rust
let context = StackingContext::new()
    .layer(z_index, item)
    .render(area, buf);
```

---

## Margin

Add margins to elements.

```rust
use ctui_layout::Margin;

let margin = Margin::all(2);         // All sides
let margin = Margin::horizontal(2);  // Left and right
let margin = Margin::vertical(1);    // Top and bottom
let margin = Margin {
    left: 1,
    right: 2,
    top: 1,
    bottom: 2,
};
```

### Applying Margins

```rust
let inner_area = margin.apply(area);
widget.render(inner_area, buf);
```

---

## Layout Validation

Validate layout correctness.

```rust
use ctui_layout::{LayoutValidator, ValidationResult};

let validator = LayoutValidator::new()
    .check_overlap(true)
    .check_bounds(true);

let result = validator.validate(area, &rects);

match result {
    ValidationResult::Valid => println!("Layout OK"),
    ValidationResult::Invalid(errors) => {
        for error in errors {
            eprintln!("Error: {:?}", error);
        }
    }
}
```

### Validation Functions

```rust
// Check rects don't overlap
pub fn no_overlapping_rects(rects: &[Rect]) -> Result<(), LayoutValidationError>;

// Check rect fits in container
pub fn rect_fits_in_container(rect: Rect, container: Rect) -> Result<(), LayoutValidationError>;
```

---

## FlexChild

Builder for flex children with alignment overrides.

```rust
use ctui_layout::FlexChild;

let child = FlexChild::new(widget)
    .flex_grow(1)                    // Grow factor
    .flex_shrink(0)                  // Shrink factor
    .flex_basis(Constraint::Min(0)) // Base size
    .align_self(AlignItems::Center); // Override alignment
```

---

## Examples

### Dashboard Layout

```rust
fn render(&self, area: Rect, buf: &mut Buffer) {
    // Main layout: Header / Body / Footer
    let main_layout = Layout::flex()
        .direction(FlexDirection::Column);

    let main = main_layout.split(area, &[
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);

    // Body: Sidebar / Content
    let body_layout = Layout::flex()
        .direction(FlexDirection::Row)
        .gap(1);

    let body = body_layout.split(main[1], &[
        Constraint::Length(20),
        Constraint::Min(0),
    ]);

    // Render sections
    self.header.render(main[0], buf);
    self.sidebar.render(body[0], buf);
    self.content.render(body[1], buf);
    self.footer.render(main[2], buf);
}
```

### Split Panel

```rust
// Resizable split panel
let left_width = self.split_position;
let layout = Layout::flex()
    .direction(FlexDirection::Row);

let rects = layout.split(area, &[
    Constraint::Length(left_width),
    Constraint::Min(0),
]);
```

### Responsive Layout

```rust
// Change layout based on terminal width
let layout = if area.width < 60 {
    Layout::flex()
        .direction(FlexDirection::Column)
} else {
    Layout::flex()
        .direction(FlexDirection::Row)
};

let rects = layout.split(area, &[
    Constraint::Percentage(50),
    Constraint::Percentage(50),
]);
```
