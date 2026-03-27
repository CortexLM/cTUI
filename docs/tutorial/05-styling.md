# Tutorial 05: Styling

Apply colors, modifiers, and visual effects.

## Goals

- Use the Style struct
- Apply colors and modifiers
- Style components

## Style Basics

```rust
use ctui_core::{Style, Color, Modifier};

let style = Style::new()
    .fg(Color::Cyan)           // Foreground color
    .bg(Color::Rgb(30, 30, 40)) // Background color
    .add_modifier(Modifier::BOLD); // Text modifier
```

## Colors

### Named Colors

```rust
Color::Black
Color::Red
Color::Green
Color::Yellow
Color::Blue
Color::Magenta
Color::Cyan
Color::White
Color::Gray
Color::DarkGray
Color::LightRed
Color::LightGreen
Color::LightYellow
Color::LightBlue
Color::LightMagenta
Color::LightCyan
```

### Indexed Colors (256 palette)

```rust
Color::Indexed(196)  // Bright red
Color::Indexed(34)   // Green
Color::Indexed(214)  // Orange
```

### RGB Colors

```rust
Color::Rgb(255, 100, 50)
Color::Rgb(0, 200, 150)
```

### Helper Methods

```rust
Color::rgb(255, 100, 50)      // Same as Rgb(...)
Color::from_hex("#FF6432")?    // Parse hex code
```

## Modifiers

```rust
use ctui_core::Modifier;

Modifier::BOLD
Modifier::DIM
Modifier::ITALIC
Modifier::UNDERLINED
Modifier::SLOW_BLINK
Modifier::RAPID_BLINK
Modifier::REVERSED
Modifier::HIDDEN
Modifier::CROSSED_OUT
```

### Combining Modifiers

```rust
let style = Style::new()
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);

// Or use bitflags
let mods = Modifier::BOLD | Modifier::ITALIC;
let style = Style::new().add_modifier(mods);
```

## Applying Styles

### To Cells

```rust
let mut buf = Buffer::empty(area);

// Apply to single cell
buf[(0, 0)].fg = Color::Red;
buf[(0, 0)].bg = Color::Black;
buf[(0, 0)].modifier = Modifier::BOLD;

// Use Style directly
buf[(0, 0)].set_style(Style::new()
    .fg(Color::Cyan)
    .bg(Color::Rgb(30, 30, 40)));
```

### To Components

Many components accept a style:

```rust
use ctui_components::{Block, Paragraph, Borders};

let block = Block::new()
    .borders(Borders::ALL)
    .style(Style::new()
        .fg(Color::White)
        .bg(Color::Rgb(40, 40, 50)));

let paragraph = Paragraph::new(text)
    .style(Style::new().fg(Color::Cyan));
```

### Border Styling

```rust
let block = Block::new()
    .borders(Borders::ALL)
    .border_style(Style::new()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD));
```

### Title Styling

```rust
let block = Block::new()
    .title("Panel")
    .title_style(Style::new()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD));
```

## Theme Integration

Use theme colors:

```rust
use ctui_theme::Theme;

let theme = Theme::dracula();

let style = Style::new()
    .fg(theme.colors.primary)
    .bg(theme.colors.background);

let block = Block::new()
    .style(style)
    .border_style(Style::new().fg(theme.colors.border));
```

## Styled Example

```rust
use ctui_core::{Buffer, Color, Modifier, Rect, Style};
use ctui_components::{Block, Borders, Paragraph, Text};

fn render_styled(area: Rect, buf: &mut Buffer) {
    // Create themed styles
    let header_style = Style::new()
        .fg(Color::Cyan)
        .bg(Color::Rgb(30, 30, 40))
        .add_modifier(Modifier::BOLD);

    let body_style = Style::new()
        .fg(Color::White)
        .bg(Color::Rgb(20, 20, 30));

    let accent_style = Style::new()
        .fg(Color::Yellow)
        .add_modifier(Modifier::ITALIC);

    // Header
    let header = Block::new()
        .borders(Borders::BOTTOM)
        .style(header_style);
    
    let header_inner = header.inner(area);
    header.render(area, buf);

    // Header text
    let title = "My Application";
    for (i, ch) in title.chars().enumerate() {
        if let Some(cell) = buf.get_mut(header_inner.x + i as u16, header_inner.y) {
            cell.symbol = ch.to_string();
            cell.fg = Color::Cyan;
            cell.bg = Color::Rgb(30, 30, 40);
            cell.modifier = Modifier::BOLD;
        }
    }

    // Accent text
    let accent_text = "★";
    if let Some(cell) = buf.get_mut(area.x, area.y) {
        cell.symbol = accent_text.to_string();
        cell.fg = Color::Yellow;
        cell.modifier = Modifier::BOLD;
    }
}
```

## Gradient Effect

Simulate gradients with color:

```rust
fn render_gradient(area: Rect, buf: &mut Buffer) {
    for x in 0..area.width {
        // Calculate color based on position
        let ratio = x as f64 / area.width as f64;
        let r = (50.0 + ratio * 150.0) as u8;
        let g = (100.0 + ratio * 100.0) as u8;
        let b = (200.0 - ratio * 100.0) as u8;
        
        for y in 0..area.height {
            if let Some(cell) = buf.get_mut(area.x + x, area.y + y) {
                cell.symbol = " ".to_string();
                cell.bg = Color::Rgb(r, g, b);
            }
        }
    }
}
```

## Focus Styling

Change style based on focus:

```rust
struct StyledButton {
    label: String,
    focused: bool,
}

impl StyledButton {
    fn style(&self) -> Style {
        if self.focused {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new()
                .fg(Color::Gray)
                .bg(Color::Reset)
        }
    }
}
```

## Selection Styling

Highlight selected items:

```rust
fn render_list(items: &[String], selected: usize, area: Rect, buf: &mut Buffer) {
    for (i, item) in items.iter().enumerate() {
        let y = area.y + i as u16;
        let style = if i == selected {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::White)
        };
        
        // Render item with style
        for (j, ch) in item.chars().enumerate() {
            if let Some(cell) = buf.get_mut(area.x + j as u16, y) {
                cell.symbol = ch.to_string();
                cell.fg = style.fg;
                cell.bg = style.bg;
                cell.modifier = style.modifier;
            }
        }
    }
}
```

## Exercise

1. Create a styled header with a gradient border
2. Add a highlighted state for buttons
3. Create a dark theme style set

## Solution

```rust
use ctui_core::{Buffer, Color, Modifier, Rect, Style};
use ctui_components::{Block, Borders};

fn render_styled_header(area: Rect, buf: &mut Buffer) {
    // Dark theme colors
    let bg_dark = Color::Rgb(20, 20, 30);
    let bg_light = Color::Rgb(35, 35, 50);
    let accent = Color::Rgb(100, 200, 255);
    let text = Color::Rgb(240, 240, 250);

    // Header block
    let block = Block::new()
        .borders(Borders::ALL)
        .style(Style::new().bg(bg_dark))
        .border_style(Style::new().fg(accent));
    
    let inner = block.inner(area);
    block.render(area, buf);

    // Title
    let title = "  My Application  ";
    for (i, ch) in title.chars().enumerate() {
        if let Some(cell) = buf.get_mut(inner.x + i as u16, inner.y) {
            cell.symbol = ch.to_string();
            cell.fg = accent;
            cell.bg = bg_dark;
            cell.modifier = Modifier::BOLD;
        }
    }

    // Button component
    let button_text = "[ Submit ]";
    let button_x = inner.x + inner.width - button_text.len() as u16 - 2;
    
    for (i, ch) in button_text.chars().enumerate() {
        if let Some(cell) = buf.get_mut(button_x + i as u16, inner.y) {
            cell.symbol = ch.to_string();
            cell.fg = Color::Rgb(0, 0, 0);
            cell.bg = accent;
            cell.modifier = Modifier::BOLD;
        }
    }
}
```

## Next Steps

Continue to [Tutorial 06: Animations](06-animations.md) to learn about smooth transitions.

## See Also

- [Style API](../api/core.md#style)
- [Theme API](../api/theme.md)
