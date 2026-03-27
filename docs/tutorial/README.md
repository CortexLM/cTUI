# cTUI Tutorial

A step-by-step guide to building TUI applications with cTUI.

## Prerequisites

- Rust 1.75 or later
- Familiarity with basic Rust concepts
- A terminal application

## Tutorial Series

| Part | Title | Description |
|------|-------|-------------|
| [01](01-setup.md) | Project Setup | Create a new cTUI project |
| [02](02-first-component.md) | First Component | Build a simple component |
| [03](03-state-management.md) | State Management | Handle state and messages |
| [04](04-layout.md) | Layout System | Flexible layouts |
| [05](05-styling.md) | Styling | Colors and modifiers |
| [06](06-animations.md) | Animations | Smooth transitions |
| [07](07-events.md) | Events | Handle user input |

## What You'll Build

By the end of this tutorial, you'll have built a complete todo application:

```
╭ Todo App ───────────────────────────────────────────╮
│ [ ] Learn cTUI basics                              │
│ [ ] Build your first component                     │
│ [x] Complete the tutorial                          │
╠────────────────────────────────────────────────────╣
│ Add: [_____________________________________]       │
│                          [Add Item]  [Clear Done]   │
╰────────────────────────────────────────────────────╯
```

## Learning Path

```
Setup → Component → State → Layout → Styling → Animation → Events
```

## Time Estimate

- Total: ~2 hours
- Per part: ~15-20 minutes

## Quick Start

If you want to skip ahead:

```bash
# Generate a new project
cargo new my-tui-app
cd my-tui-app

# Add dependencies
cargo add ctui tokio --features full

# Copy the minimal example
cat > src/main.rs << 'EOF'
use ctui_core::{Buffer, Rect};

fn main() {
    let area = Rect::new(0, 0, 40, 10);
    let mut buf = Buffer::empty(area);
    
    let msg = "Hello, cTUI!";
    for (i, c) in msg.chars().enumerate() {
        if let Some(cell) = buf.get_mut(i as u16, 0) {
            cell.symbol = c.to_string();
        }
    }
    
    println!("{}", buf_to_string(&buf, area));
}

fn buf_to_string(buf: &Buffer, area: Rect) -> String {
    let mut s = String::new();
    for y in 0..area.height {
        for x in 0..area.width {
            s.push_str(&buf[(x, y)].symbol);
        }
        s.push('\n');
    }
    s
}
EOF

cargo run
```

## Resources

- [API Reference](../api/README.md) - Detailed API docs
- [Component Gallery](../gallery/README.md) - Visual component reference
- [Examples](../examples/README.md) - Complete examples

## Getting Help

- [GitHub Issues](https://github.com/CortexLM/cTUI/issues)
- [Discord](https://discord.gg/ctui)
- [Documentation](https://docs.rs/ctui)
