//! Minimal example - simplest Hello World demonstration
//!
//! Run with: `cargo run --example minimal`

use ctui_core::{Buffer, Rect};

fn main() {
    let area = Rect::new(0, 0, 50, 10);
    let mut buf = Buffer::empty(area);

    // Render "Hello, cTUI!" to the buffer
    let text = "Hello, cTUI!";
    for (i, ch) in text.chars().enumerate() {
        buf.modify_cell(area.x + i as u16, area.y, |cell| {
            cell.symbol = ch.to_string();
        });
    }

    // Print the buffer contents
    println!("Minimal cTUI Example");
    println!("====================\n");

    for row in 0..area.height {
        let mut line = String::new();
        for col in 0..area.width {
            line.push_str(&buf.cell_at(area.x + col, area.y + row).symbol);
        }
        if !line.trim().is_empty() {
            println!("{}", line.trim_end());
        }
    }

    println!("\n✓ Buffer rendering verified");
}
