# API Reference - Components

Built-in UI components for the cTUI framework.

## Overview

cTUI provides a rich set of pre-built components. All components implement the `Widget` trait for simple rendering.

```rust
use ctui_components::Widget;

pub trait Widget {
    fn render(&self, area: Rect, buf: &mut Buffer);
}
```

---

## Container Components

### Block

Container widget with borders, padding, and optional title.

```rust
use ctui_components::{Block, Borders, BorderType, Title, TitlePosition, Alignment};

let block = Block::new()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .title(Title::from("My Panel"))
    .title_position(TitlePosition::Top)
    .title_alignment(Alignment::Left)
    .padding(Padding::new(1, 1, 1, 1));
```

**Render:**

```
╭ My Panel ─────────────────╮
│                           │
│                           │
│                           │
╰───────────────────────────╯
```

#### Borders

```rust
Borders::NONE       // No borders
Borders::TOP        // Top only
Borders::RIGHT      // Right only
Borders::BOTTOM    // Bottom only
Borders::LEFT       // Left only
Borders::ALL        // All borders
```

#### BorderType

```rust
BorderType::Plain      // ┌─┐│└┘
BorderType::Rounded    // ╭─╮│╰╯
BorderType::Double     // ╔═╗║╚╝
BorderType::Thick      // ┏━┓┃┗┛
```

---

### Scrollable

Wrap any widget in a scrollable region.

```rust
use ctui_components::{Scrollable, ScrollableProps, ScrollbarVisibility};

let scrollable = Scrollable::new(content)
    .scroll_x(0)
    .scroll_y(offset)
    .scrollbar_visibility(ScrollbarVisibility::Always);
```

**Props:**

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `scroll_x` | `u16` | 0 | Horizontal scroll offset |
| `scroll_y` | `u16` | 0 | Vertical scroll offset |
| `scrollbar_visibility` | `ScrollbarVisibility` | Auto | When to show scrollbars |

---

### Tabs

Tabbed navigation component.

```rust
use ctui_components::{Tabs, Tab, TabAlignment};

let tabs = Tabs::new(vec![
    Tab::new("Home"),
    Tab::new("Settings"),
    Tab::new("About"),
])
.selected(0)
.alignment(TabAlignment::Left);
```

**Render:**

```
[Home]  Settings  About
─────                    
```

---

## Text Components

### Paragraph

Multi-line text rendering with alignment and wrapping.

```rust
use ctui_components::{Paragraph, Text, Alignment, Line};

let paragraph = Paragraph::new(Text::from("Hello, World!\nLine two."))
    .alignment(Alignment::Center)
    .wrap(true);

// With styled text
let text = Text::from(vec![
    Line::from(vec![
        Span::styled("Hello", Style::default().fg(Color::Red)),
        Span::raw(", "),
        Span::styled("World!", Style::default().fg(Color::Blue)),
    ]),
]);

let paragraph = Paragraph::new(text);
```

**Props:**

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `text` | `Text` | - | Content |
| `alignment` | `Alignment` | Left | Text alignment |
| `wrap` | `bool` | true | Word wrap |

---

### Text, Line, Span

Rich text types.

```rust
use ctui_components::{Text, Line, Span};

// Text (multiple lines)
let text = Text::from("Line 1\nLine 2");
let text = Text::from(vec![
    Line::from("First line"),
    Line::from("Second line"),
]);

// Line (single line with spans)
let line = Line::from(vec![
    Span::raw("Normal "),
    Span::styled("Styled", Style::default().fg(Color::Cyan)),
]);

// Span (styled fragment)
let span = Span::styled("Text", style);
let span = Span::raw("Plain text");
```

---

### Markdown

Render Markdown content.

```rust
use ctui_components::Markdown;

let markdown = Markdown::new(r#"
# Heading

This is **bold** and *italic*.

- List item 1
- List item 2

```rust
let x = 42;
```
"#);
```

---

### Code

Syntax-highlighted code blocks.

```rust
use ctui_components::{Code, Language, CodeTheme};

let code = Code::new(r#"fn main() {
    println!("Hello, World!");
}"#)
.language(Language::Rust)
.theme(CodeTheme::Dark);
```

**Languages:**

```rust
Language::Rust
Language::JavaScript
Language::Python
Language::Go
Language::C
Language::Cpp
Language::Java
Language::Json
Language::Toml
Language::Yaml
Language::Markdown
Language::Html
Language::Css
Language::Shell
```

---

### Diff

Side-by-side diff viewer.

```rust
use ctui_components::{DiffViewer, DiffAlgorithm};

let diff = DiffViewer::new()
    .old_content(original)
    .new_content(modified)
    .algorithm(DiffAlgorithm::Myers);
```

---

## Input Components

### Input

Single-line text input.

```rust
use ctui_components::{Input, InputProps, InputState};

let input = Input::new()
    .value("Type here...")
    .placeholder("Enter text...")
    .cursor_position(10);
```

**Props:**

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `value` | `String` | "" | Input value |
| `placeholder` | `String` | "" | Placeholder text |
| `cursor_position` | `usize` | end | Cursor position |
| `password` | `bool` | false | Password mode |
| `max_length` | `Option<usize>` | None | Max characters |

**Render:**

```
Type here...|_
```

---

### Select

Dropdown selection widget.

```rust
use ctui_components::{Select, SelectItem};

let select = Select::new()
    .items(vec![
        SelectItem::new("Option A").value("a"),
        SelectItem::new("Option B").value("b"),
    ])
    .selected(Some(0));
```

**Render:**

```
Option A     ▼
```

---

### Checkbox

Checkbox input.

```rust
use ctui_components::{Checkbox, CheckboxGroup};

let checkbox = Checkbox::new("Enable feature")
    .checked(true);

// Checkbox group
let group = CheckboxGroup::new()
    .item("Option 1", true)
    .item("Option 2", false)
    .item("Option 3", true);
```

**Render:**

```
[x] Enable feature
```

---

### Radio

Radio button group.

```rust
use ctui_components::{RadioGroup, RadioItem};

let radio = RadioGroup::new()
    .items(vec![
        RadioItem::new("Small").value("s"),
        RadioItem::new("Medium").value("m"),
        RadioItem::new("Large").value("l"),
    ])
    .selected(1);
```

**Render:**

```
( ) Small
(•) Medium
( ) Large
```

---

### Slider

Value slider input.

```rust
use ctui_components::{Slider, Orientation};

let slider = Slider::new()
    .value(50.0)
    .min(0.0)
    .max(100.0)
    .step(5.0)
    .orientation(Orientation::Horizontal);
```

**Render:**

```
██████████●░░░░░░░░░ 50
```

---

## Data Components

### List

Scrollable list with selection.

```rust
use ctui_components::{List, ListItem, SelectionMode};

let list = List::new(vec![
    ListItem::new("Item 1"),
    ListItem::new("Item 2").style(highlight_style),
    ListItem::new("Item 3"),
])
.selected(Some(1))
.selection_mode(SelectionMode::Single);
```

**Render:**

```
  Item 1
> Item 2
  Item 3
```

**Selection Modes:**

```rust
SelectionMode::None       // No selection
SelectionMode::Single     // Single item
SelectionMode::Multiple   // Multiple items
```

---

### Table

Tabular data with rows and columns.

```rust
use ctui_components::{Table, Column, Row, Cell};

let table = Table::new()
    .columns(vec![
        Column::new("ID").width(Constraint::Length(5)),
        Column::new("Name").width(Constraint::Min(10)),
        Column::new("Status").width(Constraint::Length(10)),
    ])
    .rows(vec![
        Row::from_cells(vec![
            Cell::from("1"),
            Cell::from("Alice"),
            Cell::from("Active"),
        ]),
        Row::from_cells(vec![
            Cell::from("2"),
            Cell::from("Bob"),
            Cell::from("Inactive"),
        ]),
    ])
    .header_style(Style::default().fg(Color::Cyan))
    .selected(Some(0));
```

**Render:**

```
ID    Name        Status
─────────────────────────
1     Alice       Active
2     Bob         Inactive
```

---

### Tree

Hierarchical tree view.

```rust
use ctui_components::{Tree, TreeNode};

let tree = Tree::new(
    TreeNode::new("Root")
        .expanded(true)
        .children(vec![
            TreeNode::new("Child 1")
                .children(vec![
                    TreeNode::new("Grandchild 1"),
                ]),
            TreeNode::new("Child 2"),
        ])
);
```

**Render:**

```
▼ Root
  ▶ Child 1
  ▼ Child 2
    Grandchild 1
```

---

## Visualization Components

### Chart

ASCII bar and line charts.

```rust
use ctui_components::{Chart, DataPoint, ChartType, BarOrientation};

let chart = Chart::new()
    .data(vec![
        DataPoint::new("Mon", 40.0),
        DataPoint::new("Tue", 60.0),
        DataPoint::new("Wed", 80.0),
        DataPoint::new("Thu", 50.0),
        DataPoint::new("Fri", 70.0),
    ])
    .chart_type(ChartType::Bar)
    .orientation(BarOrientation::Vertical);
```

**Render:**

```
            █████   
            █████   
            █████   
      █████ █████   
      █████ █████ █████
      █████ █████ █████
Mon   Tue   Wed   Thu   Fri
```

---

### Gauge

Semi-circular or linear gauge.

```rust
use ctui_components::{Gauge, LinearGauge};

// Linear gauge
let gauge = LinearGauge::new()
    .value(75.0)
    .max(100.0)
    .label("Progress");

// Semi-circular gauge
let gauge = Gauge::new()
    .value(75.0)
    .max(100.0);
```

**Render (Linear):**

```
[████████████████████░░░░░] 75%
```

---

### Sparkline

Compact inline visualization.

```rust
use ctui_components::{Sparkline, BarSparkline};

let sparkline = Sparkline::new()
    .data(vec![1.0, 2.0, 3.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
```

**Render:**

```
 ▁▃▅▇▆▄▂▁
```

---

### ProgressBar

Progress indicator.

```rust
use ctui_components::{ProgressBar, ProgressBarProps};

let progress = ProgressBar::new()
    .ratio(0.65)
    .show_percent(true)
    .label("Loading...");
```

**Render:**

```
Loading... ████████████████░░░░░░░░░░ 65%
```

---

### Spinner

Animated loading indicator.

```rust
use ctui_components::{Spinner, SpinnerStyle};

let spinner = Spinner::new()
    .style(SpinnerStyle::Dots);
```

**Spinner Styles:**

```rust
SpinnerStyle::Dots       // ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
SpinnerStyle::Line       // |/-\
SpinnerStyle::Block      // ▉▊▋▌▍▎▏
SpinnerStyle::Simple    // .oOo
```

---

## Overlay Components

### Modal

Modal dialog overlay.

```rust
use ctui_components::{Modal, ModalProps, ModalSize, ModalAlignment, ModalButton};

let modal = Modal::new()
    .title("Confirm")
    .body("Are you sure you want to delete this item?")
    .buttons(vec![
        ModalButton::new("Cancel").action(ModalAction::Close),
        ModalButton::new("Delete").action(ModalAction::Submit),
    ])
    .size(ModalSize::Small)
    .alignment(ModalAlignment::Center);
```

---

## Editor Components

### Editor

Full-featured text editor.

```rust
use ctui_components::{Editor, EditorProps, EditorState};

let editor = Editor::new()
    .content("Hello, World!")
    .cursor_position(5)
    .line_numbers(true)
    .highlight_current_line(true);
```

---

### Canvas

Custom drawing surface.

```rust
use ctui_components::{Canvas, Shape, Point};

let canvas = Canvas::new()
    .width(50)
    .height(20)
    .paint(|ctx| {
        ctx.draw(&Shape::Line {
            start: Point::new(0, 10),
            end: Point::new(49, 10),
        });
        ctx.draw(&Shape::Rectangle {
            x: 10,
            y: 5,
            width: 30,
            height: 10,
        });
    });
```

---

## Navigation Components

### Link

Clickable link.

```rust
use ctui_components::Link;

let link = Link::new("Click here")
    .url("https://example.com")
    .on_click(|| {
        // Handle click
    });
```

---

## Form Components

### Form

Form with validation.

```rust
use ctui_components::{Form, FormField, FieldType};

let form = Form::new()
    .field(FormField::new("name")
        .label("Name")
        .field_type(FieldType::Text)
        .required(true))
    .field(FormField::new("email")
        .label("Email")
        .field_type(FieldType::Email)
        .required(true)
        .validator(|v| v.contains('@')));
```

---

## Widget Trait

All components implement `Widget`:

```rust
pub trait Widget {
    /// Render to buffer
    fn render(&self, area: Rect, buf: &mut Buffer);
}

pub trait WidgetExt: Widget + Sized {
    /// Render to new buffer
    fn to_buffer(&self, width: u16, height: u16) -> Buffer;

    /// Render to string
    fn render_to_string(&self, width: u16, height: u16) -> String;
}
```

## Creating Custom Components

```rust
use ctui_components::Widget;
use ctui_core::{Buffer, Rect};

struct MyWidget {
    value: String,
}

impl Widget for MyWidget {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        for (i, ch) in self.value.chars().take(area.width as usize).enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }
}
```
