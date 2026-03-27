# ctui-core - AGENTS.md

## OVERVIEW

Low-level rendering primitives. Foundation for all other crates. Contains Buffer, Cell, Component trait, Terminal, geometry, and event system.

## WHERE TO LOOK

| Need | File |
|------|------|
| Component trait | `component.rs` |
| Buffer/Cell | `buffer/mod.rs`, `cell.rs` |
| Events | `event.rs` |
| Terminal loop | `terminal.rs`, `render_loop.rs` |
| Style/Color | `style.rs` |
| Rect/Position/Size | `geometry.rs` |
| State management | `state.rs` |
| Backend abstraction | `backend/mod.rs` |

## KEY TYPES (Public API)

```rust
Buffer, Cell                           // Screen representation
Rect, Position, Size                  // Geometry
Style, Color, Modifier                // Styling
Component, Cmd, Msg                   // Component system
Event, KeyEvent, MouseEvent           // Input handling
Terminal, Frame, Widget               // Rendering
Props, DefaultProps                   // Props system
```

## CONVENTIONS

- Props structs use builder pattern: `MyProps::new().field(val)`
- Components return `Cmd` from update (Render, Quit, Navigate, Batch)
- State = unit `()` for stateless widgets
