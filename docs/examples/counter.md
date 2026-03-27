# Counter Example

A minimal stateful application demonstrating the component pattern.

## Overview

The counter example shows:

- Basic component structure
- State management
- Message handling
- Simple rendering

## Source Code

```rust
//! Counter example - minimal stateful component demonstration

use ctui_core::{Buffer, Cmd, Component, Msg, Rect};

// Define messages for state updates
struct Increment;
struct Decrement;
struct Reset;

impl Msg for Increment {}
impl Msg for Decrement {}
impl Msg for Reset {}

// Component state
struct Counter {
    count: i32,
}

impl Counter {
    fn new(initial: i32) -> Self {
        Self { count: initial }
    }
}

impl Component for Counter {
    type Props = i32;    // Initial count
    type State = ();

    fn create(props: Self::Props) -> Self {
        Self::new(props)
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("Counter: {}", self.count);
        for (i, ch) in text.chars().take(area.width as usize).enumerate() {
            if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }

    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
        if msg.is::<Increment>() {
            self.count += 1;
            Cmd::Render
        } else if msg.is::<Decrement>() {
            self.count -= 1;
            Cmd::Render
        } else if msg.is::<Reset>() {
            self.count = 0;
            Cmd::Render
        } else {
            Cmd::Noop
        }
    }
}

fn main() {
    let mut counter = Counter::new(0);
    counter.on_mount();

    println!("Counter Example");
    println!("===============\n");

    println!("Initial count: {}", counter.count);

    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    counter.update(Box::new(Increment));
    println!("After 3 increments: {}", counter.count);

    counter.update(Box::new(Decrement));
    println!("After 1 decrement: {}", counter.count);

    counter.update(Box::new(Reset));
    println!("After reset: {}", counter.count);

    counter.on_unmount();
}
```

## Render Output

```
Counter: 3
```

## Key Concepts

### Message Pattern

Messages are how components communicate:

```rust
// Define a message
struct Increment;
impl Msg for Increment {}

// Send a message
component.update(Box::new(Increment));

// Handle a message
fn update(&mut self, msg: Box<dyn Msg>) -> Cmd {
    if msg.is::<Increment>() {
        self.count += 1;
        Cmd::Render
    } else {
        Cmd::Noop
    }
}
```

### Cmd Return Type

The `Cmd` enum controls what happens after an update:

```rust
pub enum Cmd {
    Noop,    // Do nothing
    Render,  // Request a render
    Async,   // Start async operation
    Batch,   // Multiple commands
}
```

### Lifecycle Hooks

```rust
impl Component for Counter {
    fn on_mount(&mut self) {
        // Called when component is mounted
    }

    fn on_unmount(&mut self) {
        // Called when component is unmounted
    }
}
```

## Enhancing the Example

### Add Input Handling

```rust
fn handle_event(&mut self, event: &Event) -> Option<Box<dyn Msg>> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Char('+') => Some(Box::new(Increment)),
            KeyCode::Char('-') => Some(Box::new(Decrement)),
            KeyCode::Char('r') => Some(Box::new(Reset)),
            _ => None,
        },
        _ => None,
    }
}
```

### Add Animation

```rust
use ctui_animate::{KeyframeAnimation, EasingFunction};

struct AnimatedCounter {
    count: i32,
    displayed_count: f64,
    animation: KeyframeAnimation,
}

impl AnimatedCounter {
    fn new() -> Self {
        Self {
            count: 0,
            displayed_count: 0.0,
            animation: KeyframeAnimation::new()
                .duration_ms(200)
                .easing(EasingFunction::EaseOutCubic),
        }
    }

    fn set_count(&mut self, new_count: i32) {
        self.animation = KeyframeAnimation::new()
            .keyframe(Keyframe::new(0.0, self.displayed_count))
            .keyframe(Keyframe::new(1.0, new_count as f64));
        self.count = new_count;
    }
}
```

### Add Styling

```rust
use ctui_core::{Style, Color, Modifier};

fn render(&self, area: Rect, buf: &mut Buffer) {
    let style = Style::new()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let text = format!("Counter: {}", self.count);
    for (i, ch) in text.chars().enumerate() {
        if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
            cell.symbol = ch.to_string();
            cell.fg = style.fg;
            cell.modifier = style.modifier;
        }
    }
}
```

## Run the Example

```bash
cargo run --example counter
```

## See Also

- [Todo App](todo.md) - More complex example
- [Component API](../api/components.md) - API documentation
- [Tutorial](../tutorial/README.md) - Step by step guide
