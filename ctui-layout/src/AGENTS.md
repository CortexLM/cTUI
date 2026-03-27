# ctui-layout - AGENTS.md

## OVERVIEW

Flexbox-inspired layout engine. Supports flex direction, justify content, align items, gap, grid, absolute positioning with z-index.

## WHERE TO LOOK

| Need | File |
|------|------|
| Flex layout | `flex.rs` |
| Grid layout | `grid.rs` |
| Constraints | `constraint.rs` |
| Absolute positioning | `absolute.rs` |
| Validation | `validation.rs` |

## KEY TYPES

```rust
Layout, FlexLayout                 // Main flex container
FlexDirection                      // Row, Column
JustifyContent, AlignItems        // Alignment
Constraint                         // Length, Ratio, Percentage, Min, Max, Fill
Grid, GridTrack, GridPosition      // CSS Grid-like
AbsoluteLayout, ZIndex             // Overlays
LayoutValidator                     // No overlaps, bounds checking
```

## CONVENTIONS

- `Layout::flex()` for standard flex container
- Constraints describe child sizing
- `.split(area, &constraints)` returns Vec<Rect>
- Validation returns Result with helpful errors
