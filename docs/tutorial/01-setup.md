# Tutorial 01: Project Setup

Set up a new cTUI project from scratch.

## Goals

- Create a new Rust project
- Add cTUI dependencies
- Run your first cTUI app

## Create Project

```bash
# Create new project
cargo new my-tui-app
cd my-tui-app
```

## Add Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "my-tui-app"
version = "0.1.0"
edition = "2021"

[dependencies]
ctui = "0.1"
tokio = { version = "1", features = ["full"] }
```

Or use cargo:

```bash
cargo add ctui
cargo add tokio --features full
```

## Project Structure

```
my-tui-app/
├── Cargo.toml
├── src/
│   └── main.rs
└── target/
```

## Basic Application

Replace `src/main.rs`:

```rust
use ctui_core::{Buffer, Rect};

fn main() {
    // Create a buffer (40 columns x 10 rows)
    let area = Rect::new(0, 0, 40, 10);
    let mut buf = Buffer::empty(area);
    
    // Write a message
    let message = "Hello, cTUI!";
    for (i, ch) in message.chars().enumerate() {
        if let Some(cell) = buf.get_mut(i as u16, 0) {
            cell.symbol = ch.to_string();
        }
    }
    
    // Print the buffer
    println!("╭{:─^38}╮", "");
    for y in 0..area.height {
        print!("│");
        for x in 0..area.width {
            print!("{}", buf[(x, y)].symbol);
        }
        println!("│");
    }
    println!("╰{:─^38}╯", "");
}
```

## Run It

```bash
cargo run
```

Output:

```
╭──────────────────────────────────────╮
│Hello, cTUI!                          │
│                                      │
│                                      │
│                                      │
│                                      │
│                                      │
│                                      │
│                                      │
│                                      │
│                                      │
╰──────────────────────────────────────╯
```

## Understanding the Code

### Buffer

The `Buffer` is the screen representation:

```rust
// Create empty buffer
let buf = Buffer::empty(Rect::new(0, 0, 80, 24));

// Access cells
let cell = buf[(0, 0)];
let cell = buf.get(5, 3).unwrap();

// Modify cells
buf[(0, 0)].symbol = "A".to_string();
```

### Rect

A `Rect` defines a rectangular area:

```rust
let rect = Rect::new(x, y, width, height);

// x: horizontal position
// y: vertical position
// width: number of columns
// height: number of rows
```

### Cell

Each cell has:

```rust
pub struct Cell {
    pub symbol: String,   // Character to display
    pub fg: Color,        // Foreground color
    pub bg: Color,        // Background color
    pub modifier: Modifier, // Bold, italic, etc.
}
```

## Exercise

1. Change the message to "Welcome to cTUI!"
2. Make the message appear in the center (row 5)
3. Add a second line with your name

## Solution

```rust
use ctui_core::{Buffer, Rect};

fn main() {
    let area = Rect::new(0, 0, 40, 10);
    let mut buf = Buffer::empty(area);
    
    // Line 1: centered
    let msg1 = "Welcome to cTUI!";
    let start1 = (40 - msg1.len() as u16) / 2;
    for (i, ch) in msg1.chars().enumerate() {
        if let Some(cell) = buf.get_mut(start1 + i as u16, 4) {
            cell.symbol = ch.to_string();
        }
    }
    
    // Line 2: your name
    let msg2 = "by Your Name";
    let start2 = (40 - msg2.len() as u16) / 2;
    for (i, ch) in msg2.chars().enumerate() {
        if let Some(cell) = buf.get_mut(start2 + i as u16, 5) {
            cell.symbol = ch.to_string();
        }
    }
    
    // Print
    for y in 0..area.height {
        for x in 0..area.width {
            print!("{}", buf[(x, y)].symbol);
        }
        println!();
    }
}
```

## Next Steps

Continue to [Tutorial 02: First Component](02-first-component.md) to learn about the Component trait.

## See Also

- [API: Buffer](../api/core.md#buffer)
- [Getting Started](../getting-started.md)
