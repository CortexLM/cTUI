# ctui-core

Core rendering primitives for cTUI, providing the foundational types for terminal UI rendering.

## Overview

This crate provides:
- **Buffer** - The render surface holding cell data
- **Cell** - Individual terminal cell with style information
- **Component trait** - The Elm-inspired component lifecycle
- **Event handling** - Keyboard and mouse input abstractions
- **Color types** - Standard terminal colors and optional high-precision float colors

## Features

### `float-colors` (Optional)

Enables high-precision color representation with `Color32` for advanced rendering use cases.

```toml
[dependencies]
ctui-core = { version = "0.1", features = ["float-colors"] }
```

#### What is Color32?

`Color32` is an RGBA color type using `f32` components in the 0.0-1.0 range:

```rust
#[cfg(feature = "float-colors")]
pub struct Color32 {
    pub r: f32,  // Red (0.0-1.0)
    pub g: f32,  // Green (0.0-1.0)
    pub b: f32,  // Blue (0.0-1.0)
    pub a: f32,  // Alpha (0.0-1.0)
}
```

#### Memory Trade-offs

| Type | Size Per Color | Precision |
|------|---------------|-----------|
| `Color::Rgb(u8, u8, u8)` | 3-4 bytes | 256 levels per channel |
| `Color32` | 16 bytes (4 × f32) | ~7 decimal digits precision |

**Why use Color32?**

- **Alpha blending** - Full alpha channel support for layering compositing
- **Gradients** - Smooth interpolation between colors without banding
- **Color math** - Safe multiplication, blending, and interpolation operations
- **High dynamic range** - Can represent values outside 0.0-1.0 for intermediate calculations

**When to avoid:**

For standard terminal use, `Color::Rgb` is more efficient. Enable `float-colors` only when you need alpha blending or gradient calculations. The 16-byte per-color overhead becomes significant in large buffers.

#### Usage Example

```rust
use ctui_core::style::{Color, Color32};

// Create from RGB (fully opaque)
let red = Color32::new(1.0, 0.0, 0.0);

// Create with alpha transparency
let semi_transparent = Color32::new_with_alpha(0.5, 0.5, 0.5, 0.7);

// Convert from standard Color
let from_rgb: Color32 = Color::Rgb(128, 64, 255).into();

// Convert back to Color (clamps to u8 range)
let back_to_rgb: Color = semi_transparent.into();
```

## License

MIT OR Apache-2.0
