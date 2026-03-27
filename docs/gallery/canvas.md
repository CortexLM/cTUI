# Canvas Component

Custom drawing surface for arbitrary graphics.

## Variants

### Basic Canvas

**Code:**

```rust
let canvas = Canvas::new()
    .width(40)
    .height(10)
    .paint(|ctx| {
        // Draw to context
    });
```

**Render:**

```
┌─────────────────────────────────┐
│                                 │
│                                 │
│                                 │
│                                 │
│                                 │
│                                 │
│                                 │
│                                 │
│                                 │
└─────────────────────────────────┘
```

### Drawing Shapes

**Code:**

```rust
let canvas = Canvas::new()
    .width(40)
    .height(10)
    .paint(|ctx| {
        // Draw a line
        ctx.draw(&Shape::Line {
            start: Point::new(0, 5),
            end: Point::new(39, 5),
        });

        // Draw a rectangle
        ctx.draw(&Shape::Rectangle {
            x: 5, y: 2,
            width: 10, height: 5,
        });

        // Draw text
        ctx.print(10, 3, "Hello!");
    });
```

**Render:**

```
┌─────────────────────────────────┐
│                                 │
│     ┌──────────┐                │
│     │ Hello!   │----------------│
│     │          │                │
│     └──────────┘                │
│                                 │
│                                 │
│                                 │
│                                 │
└─────────────────────────────────┘
```

## Shapes

```rust
pub enum Shape {
    Line {
        start: Point,
        end: Point,
    },
    Rectangle {
        x: u16, y: u16,
        width: u16, height: u16,
    },
    Circle {
        center: Point,
        radius: u16,
    },
    Points(Vec<Point>),
    Text {
        x: u16, y: u16,
        content: String,
    },
}
```

## Point

```rust
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self;
}
```

## Drawing Context

```rust
// In paint closure
ctx.draw(&shape);          // Draw a shape
ctx.print(x, y, "text");  // Print text
ctx.set_fg(Color::Red);   // Set foreground
ctx.set_bg(Color::Black); // Set background
ctx.clear();              // Clear canvas
```

## Props

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `width` | `u16` | area width | Canvas width |
| `height` | `u16` | area height | Canvas height |
| `background` | `Color` | Reset | Background color |
| `paint` | `fn(&mut Context)` | - | Draw function |

## Example: Progress Bar

```rust
use ctui_components::{Canvas, Shape, Point};

fn render_progress(progress: f64, area: Rect, buf: &mut Buffer) {
    let canvas = Canvas::new()
        .width(area.width)
        .height(3)
        .paint(move |ctx| {
            let width = ((area.width - 2) as f64 * progress) as u16;
            
            // Bar background
            ctx.draw(&Shape::Rectangle {
                x: 1, y: 1,
                width: area.width - 2,
                height: 1,
            });
            
            // Filled portion
            ctx.set_fg(Color::Green);
            ctx.draw(&Shape::Rectangle {
                x: 1, y: 1,
                width: width,
                height: 1,
            });
        });
    
    canvas.render(area, buf);
}
```

## Example: Chart

```rust
fn render_chart(data: &[f64], area: Rect, buf: &mut Buffer) {
    let max = data.iter().cloned().fold(0.0, f64::max);
    
    let canvas = Canvas::new()
        .width(area.width)
        .height(area.height)
        .paint(|ctx| {
            for (i, &value) in data.iter().enumerate() {
                let height = ((value / max) * area.height as f64) as u16;
                let x = i as u16;
                
                ctx.draw(&Shape::Rectangle {
                    x, y: area.height - height,
                    width: 1, height,
                });
            }
        });
    
    canvas.render(area, buf);
}
```

## See Also

- [Chart](chart.md) - Built-in charts
- [Sparkline](sparkline.md) - Inline visualization
- [Gauge](gauge.md) - Gauge displays
