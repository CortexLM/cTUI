# cTUI - AGENTS.md

## OVERVIEW

High-performance Rust TUI framework with React-style declarative components. Pure Rust, zero unsafe code. Async-first with Tokio runtime.

## STRUCTURE

```
cTUI/
├── ctui-core/       # Rendering primitives, Buffer, Cell, Component trait
├── ctui-components/ # 27 pre-built widgets (Block, Input, List, Table...)
├── ctui-layout/     # Flexbox-inspired layout engine
├── ctui-animate/    # Easing, keyframes, spring physics
├── ctui-theme/      # Built-in themes + theming system
├── ctui-macros/     # #[component] proc-macro
├── ctui-cli/        # Project scaffolding CLI
├── ctui-tests/      # Integration test harness
├── benches/         # Criterion benchmarks
└── examples/        # counter, todo, dashboard, file_explorer, animation
```

## WHERE TO LOOK

| Need | Location |
|------|----------|
| Component trait / lifecycle | `ctui-core/src/component.rs` |
| Buffer rendering | `ctui-core/src/buffer/` |
| Input handling | `ctui-core/src/event.rs` |
| Widget implementations | `ctui-components/src/{widget}.rs` |
| Layout algorithms | `ctui-layout/src/flex.rs`, `grid.rs` |
| Animation easing | `ctui-animate/src/easing.rs` |
| Theme presets | `ctui-theme/src/theme.rs` |
| Benchmarks | `benches/ratatui_baseline.rs` |

## KEY ABSTRACTIONS

### Component Trait (Elm Architecture)
```rust
trait Component {
    type Props;  // Configuration (builder pattern)
    type State;  // Internal state
    fn create(props: Self::Props) -> Self;
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn update(&mut self, msg: Box<dyn Msg>) -> Cmd;
    fn on_mount(&mut self) {}
    fn on_unmount(&mut self) {}
}
```

### Widget Trait (Stateless)
```rust
trait Widget {
    fn render(&self, area: Rect, buf: &mut Buffer);
}
```

### Layout Snippet
```rust
Layout::flex()
    .direction(FlexDirection::Row)
    .justify_content(JustifyContent::SpaceBetween)
    .gap(1)
    .split(area, &constraints)
```

### Animation Snippet
```rust
KeyframeAnimation::new()
    .keyframe(Keyframe::new(0.0, 0.0))
    .keyframe(Keyframe::new(1.0, 100.0))
    .duration_ms(1000)
    .playback_mode(PlaybackMode::Loop)
```

## COMMANDS

```bash
cargo test --workspace --all-features    # Run all 1079 tests
cargo clippy --workspace -- -D warnings  # Lint check
cargo bench                                # Criterion benchmarks
cargo run --example counter                # Run example
cargo doc --workspace --open               # View docs
ctui new my-app --template counter        # Scaffold project
```

## BENCHMARKS

Target: >=10% faster than ratatui baseline
- Buffer operations: 64-136% improvement
- Buffer diff: 35% faster
- Paragraph render: 20% faster

## CRATE DEPENDENCY GRAPH

```
ctui-core      (foundation, no deps)
├── ctui-components -> ctui-core
├── ctui-layout     -> ctui-core
├── ctui-animate    -> ctui-core
├── ctui-theme      -> ctui-core
└── ctui-macros     (proc-macro, standalone)
```

## NOTES

- ~43k lines Rust, 1079 tests across workspace
- CI: blacksmith-4cpu-ubuntu-2204 runner
- Lints: `#![deny(unsafe_code)]`, clippy pedantic + nursery
- Release profile: LTO + opt-level 3
- Snapshot testing via insta crate
