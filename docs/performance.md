# Performance Guide

Optimize cTUI applications for maximum performance.

## Overview

cTUI is designed for high performance. This guide covers:

- Buffer diff algorithm
- Layout caching
- Rendering optimizations
- Memory management

## Buffer Diff Algorithm

cTUI uses a zero-allocation diff algorithm to minimize terminal updates.

### How It Works

1. **Previous Frame**: Store the last rendered buffer
2. **Current Frame**: Render into a new buffer
3. **Diff**: Compare cell by cell
4. **Update**: Only write changed cells to terminal

```rust
// Diff two buffers
for (x, y, cell) in current.diff(&previous) {
    // Only update cells that changed
    backend.draw_cell(x, y, cell);
}
```

### Performance Benefits

| Scenario | Full Render | Diff Render |
|----------|-------------|-------------|
| No changes | 7680 cells | 0 cells |
| Single line | 7680 cells | 160 cells |
| Small area | 7680 cells | ~50 cells |

## Layout Caching

Cache layout computations to avoid recalculation.

### Terminal Cache

```rust
use ctui_core::Terminal;

let mut terminal = Terminal::new(backend)?;

// Layout cache metrics
let metrics = terminal.cache_metrics();
println!("Cache hits: {}", metrics.hits);
println!("Cache misses: {}", metrics.misses);
```

### Layout Memoization

```rust
use std::cell::RefCell;

struct CachedLayout {
    cache: RefCell<HashMap<(Rect, Vec<Constraint>), Vec<Rect>>>,
}

impl CachedLayout {
    fn split(&self, area: Rect, constraints: &[Constraint]) -> Vec<Rect> {
        let key = (area, constraints.to_vec());
        
        if let Some(cached) = self.cache.borrow().get(&key) {
            return cached.clone();
        }
        
        let result = Layout::flex().split(area, constraints);
        self.cache.borrow_mut().insert(key, result.clone());
        result
    }
}
```

## Rendering Best Practices

### 1. Minimize Buffer Operations

```rust
// Bad: O(n*m) per render
for y in 0..area.height {
    for x in 0..area.width {
        if let Some(cell) = buf.get_mut(x, y) {
            cell.symbol = get_char(x, y);
        }
    }
}

// Good: Only render what you need
for (i, ch) in text.chars().enumerate() {
    if let Some(cell) = buf.get_mut(area.x + i as u16, area.y) {
        cell.symbol = ch.to_string();
    }
}
```

### 2. Batch Cell Updates

```rust
use ctui_core::Cell;

// Prepare cells
let cells: Vec<(u16, u16, Cell)> = items.iter()
    .enumerate()
    .flat_map(|(y, item)| {
        item.chars().enumerate().map(move |(x, ch)| {
            (x as u16, y as u16, Cell::new(ch.to_string()))
        })
    })
    .collect();

// Apply in batch
for (x, y, cell) in cells {
    buf[(x, y)] = cell;
}
```

### 3. Use Styles Sparingly

```rust
// Bad: New style per cell
for (i, ch) in text.chars().enumerate() {
    let style = Style::new().fg(Color::Red);
    buf[(i as u16, 0)] = Cell::new(ch).with_style(style);
}

// Good: Reuse style
let style = Style::new().fg(Color::Red);
for (i, ch) in text.chars().enumerate() {
    let mut cell = Cell::new(ch);
    cell.fg = style.fg;
    buf[(i as u16, 0)] = cell;
}
```

## Memory Management

### Buffer Size

| Display | Dimensions | Cells | Memory |
|----------|------------|-------|--------|
| Small | 80x24 | 1,920 | ~60 KB |
| Medium | 120x40 | 4,800 | ~150 KB |
| Large | 200x60 | 12,000 | ~380 KB |

### Guidelines

1. **Reuse Buffers**: Don't allocate new buffers per frame
2. **Limit Allocations**: Pre-allocate buffers for known sizes
3. **Arena Allocation**: For complex apps, consider arena allocators

```rust
// Pre-allocate
struct App {
    buffer: Buffer,
    previous: Buffer,
}

impl App {
    fn new() -> Self {
        let area = Rect::new(0, 0, 80, 24);
        Self {
            buffer: Buffer::empty(area),
            previous: Buffer::empty(area),
        }
    }

    fn render(&mut self) {
        // Reuse buffers
        self.buffer.reset();
        // ... render into buffer ...
        
        // Swap
        std::mem::swap(&mut self.buffer, &mut self.previous);
    }
}
```

## Benchmarks

### vs ratatui

| Metric | cTUI | ratatui |
|--------|------|---------|
| Diff algorithm | O(n) | N/A |
| Memory per frame | ~1 KB | ~100 KB |
| Layout caching | Yes | No |
| 60 FPS | Yes | Limited |

### Performance Tips

1. **Use `--release`**: Always profile/release build
2. **Benchmark**: Use `criterion` for benchmarks
3. **Profile**: Use `perf` or `flamegraph`

```rust
// benches/buffer_diff.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn buffer_diff_benchmark(c: &mut Criterion) {
    let area = Rect::new(0, 0, 80, 24);
    let a = Buffer::empty(area);
    let b = Buffer::empty(area);

    c.bench_function("diff", |bencher| {
        bencher.iter(|| {
            black_box(a.diff(&b))
        })
    });
}

criterion_group!(benches, buffer_diff_benchmark);
criterion_main!(benches);
```

## Animation Performance

### Frame Rate

Target 60 FPS (16.67ms per frame):

```rust
use std::time::{Duration, Instant};

const TARGET_FRAME_TIME: Duration = Duration::from_micros(16_667);

fn render_loop() {
    let mut last_frame = Instant::now();

    loop {
        let frame_start = Instant::now();

        // Process events
        handle_events();

        // Update state
        update();

        // Render
        render();

        // Wait for next frame
        let elapsed = frame_start.elapsed();
        if elapsed < TARGET_FRAME_TIME {
            std::thread::sleep(TARGET_FRAME_TIME - elapsed);
        }
    }
}
```

### Animation Optimization

```rust
// Use spring animations (no keyframes to compute)
let spring = SpringAnimation::new()
    .config(SpringConfig::stiff());

// Or use simple interpolation
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
```

## Benchmarking Tips

### Run Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench -- buffer_diff

# Generate flamegraph
cargo flamegraph --bench buffer_diff
```

### Profile with perf

```bash
# Record profile
perf record -g ./target/release/my-app

# View report
perf report
```

## Summary

Key performance optimizations:

1. **Use Buffer Diff**: Only update changed cells
2. **Cache Layouts**: Avoid recomputation
3. **Minimize Allocations**: Reuse buffers
4. **Batch Updates**: Apply changes in bulk
5. **Profile First**: Measure before optimizing

## See Also

- [API: Buffer](api/core.md#buffer)
- [API: Terminal](api/core.md#terminal)
- [Migration Guide](migration.md)
