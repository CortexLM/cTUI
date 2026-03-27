# cTUI Component Gallery

Visual reference for all cTUI components with ASCII renders.

## All Components

### Container Components

| Component | Description |
|-----------|-------------|
| [Block](block.md) | Container with borders and titles |
| [Scrollable](scrollable.md) | Scrollable region with scrollbars |
| [Modal](modal.md) | Modal/dialog overlay |
| [Tabs](tabs.md) | Tabbed navigation |

### Text Components

| Component | Description |
|-----------|-------------|
| [Paragraph](paragraph.md) | Multi-line text with alignment |
| [Text](text.md) | Rich text content |
| [Markdown](markdown.md) | Markdown rendering |
| [Code](code.md) | Syntax-highlighted code |
| [Diff](diff.md) | Side-by-side diff viewer |

### Input Components

| Component | Description |
|-----------|-------------|
| [Input](input.md) | Single-line text input |
| [Select](select.md) | Dropdown selection |
| [Checkbox](checkbox.md) | Checkbox input |
| [Radio](radio.md) | Radio button group |
| [Slider](slider.md) | Value slider |
| [Editor](editor.md) | Text editor |
| [Form](form.md) | Form with validation |

### Data Components

| Component | Description |
|-----------|-------------|
| [List](list.md) | Scrollable list |
| [Table](table.md) | Tabular data |
| [Tree](tree.md) | Hierarchical tree |

### Visualization Components

| Component | Description |
|-----------|-------------|
| [Chart](chart.md) | ASCII charts |
| [Sparkline](sparkline.md) | Inline visualization |
| [Gauge](gauge.md) | Semi-circular gauge |
| [Progress](progress.md) | Progress bars and spinners |
| [Canvas](canvas.md) | Custom drawing surface |

### Navigation Components

| Component | Description |
|-----------|-------------|
| [Link](link.md) | Clickable link |

## Quick Example

```rust
use ctui_components::{Block, Paragraph, Borders, BorderType};
use ctui_core::{Buffer, Rect};

let block = Block::new()
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .title("Example");

let paragraph = Paragraph::new("Hello, World!");
```

## Visual Preview

Here's a showcase of all components:

```
┌─────────────────────────────────────────────────────────────────┐
│                        cTUI Gallery                              │
├─────────────────────────────────────────────────────────────────┤
│  Block:      ╭ Panel ──╮    Input:      Type here_|             │
│               │        │    Select:     Option ▼                │
│               ╰────────╯                                         │
│                                                                  │
│  List:       > Item 1      Checkbox:   [x] Checked              │
│               Item 2       Radio:      (•) Selected             │
│               Item 3                   ( ) Option B             │
│                                                                  │
│  Progress:   ████████████░░░░░░░░ 60%                            │
│  Spinner:    ⠋ Loading...                                         │
│  Gauge:     [████████████████░░░░░░] 75%                         │
│  Sparkline:      ▁▃▅▆█▅▃▁                                        │
│                                                                  │
│  Table:      ID │ Name        │ Status                           │
│             ────┼─────────────┼────────                          │
│              1  │ Alice       │ Active                           │
│              2  │ Bob         │ Inactive                         │
│                                                                  │
│  Tree:       ▼ Root                                             │
│                ▶ Child 1                                         │
│                ▼ Child 2                                         │
│                  Grandchild                                     │
│                                                                  │
│  Tabs:       [Home]  Settings  About                            │
│  Slider:     ████████●░░░░░░░░░ 50                              │
├─────────────────────────────────────────────────────────────────┤
│  q: quit  Tab: next  Enter: select                              │
└─────────────────────────────────────────────────────────────────┘
```

## Component Properties

### Common Properties

Most components share these base properties:

| Property | Type | Description |
|----------|------|-------------|
| `style` | `Style` | Foreground, background, modifiers |
| `block` | `Block` | Container with borders |
| `area` | `Rect` | Rendering area |

### Styling Example

```rust
use ctui_core::{Style, Color, Modifier};

let style = Style::new()
    .fg(Color::Cyan)
    .bg(Color::Rgb(30, 30, 40))
    .add_modifier(Modifier::BOLD);

let paragraph = Paragraph::new("Styled text")
    .style(style);
```

## See Also

- [API Reference: Components](../api/components.md) - Detailed API documentation
- [Examples](../examples/README.md) - Complete example applications
- [Tutorial](../tutorial/README.md) - Build components step by step
