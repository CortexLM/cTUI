
//! Buffer diff benchmarks comparing ctui-core vs ratatui
//!
//! This benchmark compares the zero-allocation BufferDiff iterator
//! in ctui-core against ratatui's implementation.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn buffer_diff_no_changes(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_diff_no_changes");

    group.throughput(Throughput::Elements(80 * 24));

    group.bench_function("ctui_80x24", |b| {
        use ctui_core::{Buffer, Rect};
        let area = Rect::new(0, 0, 80, 24);
        let prev = Buffer::empty(area);
        let next = Buffer::empty(area);

        b.iter(|| prev.diff(&next).count())
    });

    group.bench_function("ratatui_80x24", |b| {
        use ratatui::{buffer::Buffer, layout::Rect};
        let area = Rect::new(0, 0, 80, 24);
        let prev = Buffer::empty(area);
        let next = Buffer::empty(area);

        b.iter(|| next.diff(&prev).len())
    });

    group.finish();
}

fn buffer_diff_full_change(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_diff_full_change");

    group.throughput(Throughput::Elements(80 * 24));

    group.bench_function("ctui_80x24", |b| {
        use ctui_core::{Buffer, Cell, Rect};
        let area = Rect::new(0, 0, 80, 24);
        let prev = Buffer::empty(area);
        let mut next = Buffer::empty(area);
        for y in 0..area.height {
            for x in 0..area.width {
                next.set(x, y, Cell::new("X"));
            }
        }

        b.iter(|| prev.diff(&next).count())
    });

    group.bench_function("ratatui_80x24", |b| {
        use ratatui::{buffer::Buffer, layout::Rect};
        let area = Rect::new(0, 0, 80, 24);
        let prev = Buffer::empty(area);
        let mut next = Buffer::empty(area);
        for cell in next.content.iter_mut() {
            cell.set_char('X');
        }

        b.iter(|| next.diff(&prev).len())
    });

    group.finish();
}

fn buffer_diff_partial_change(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_diff_partial_change");

    group.throughput(Throughput::Elements(80 * 24));

    group.bench_function("ctui_80x24_50percent", |b| {
        use ctui_core::{Buffer, Cell, Rect};
        let area = Rect::new(0, 0, 80, 24);
        let prev = Buffer::empty(area);
        let mut next = Buffer::empty(area);
        for y in 0..12 {
            for x in 0..area.width {
                next.set(x, y, Cell::new("Y"));
            }
        }

        b.iter(|| prev.diff(&next).count())
    });

    group.bench_function("ratatui_80x24_50percent", |b| {
        use ratatui::{buffer::Buffer, layout::Rect};
        let area = Rect::new(0, 0, 80, 24);
        let prev = Buffer::empty(area);
        let mut next = Buffer::empty(area);
        for i in 0..(12 * 80) {
            next.content[i].set_char('Y');
        }

        b.iter(|| next.diff(&prev).len())
    });

    group.finish();
}

fn buffer_diff_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_diff_large");

    group.throughput(Throughput::Elements(200 * 50));

    group.bench_function("ctui_200x50", |b| {
        use ctui_core::{Buffer, Cell, Rect};
        let area = Rect::new(0, 0, 200, 50);
        let prev = Buffer::empty(area);
        let mut next = Buffer::empty(area);
        for y in 0..25 {
            for x in (0..area.width).step_by(2) {
                next.set(x, y, Cell::new("X"));
            }
        }

        b.iter(|| prev.diff(&next).count())
    });

    group.bench_function("ratatui_200x50", |b| {
        use ratatui::{buffer::Buffer, layout::Rect};
        let area = Rect::new(0, 0, 200, 50);
        let prev = Buffer::empty(area);
        let mut next = Buffer::empty(area);
        for i in (0..(25 * 200)).step_by(2) {
            next.content[i].set_char('X');
        }

        b.iter(|| next.diff(&prev).len())
    });

    group.finish();
}

/// Real-world benchmark: buffers share the same symbol table (clone case)
/// This tests the fast path optimization where Arc::ptr_eq returns true.
fn buffer_diff_shared_table(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_diff_shared_table");

    group.throughput(Throughput::Elements(80 * 24));

    // Real-world case: clone buffer, then diff (shares symbol table)
    group.bench_function("ctui_80x24_clone_no_change", |b| {
        use ctui_core::{Buffer, Rect};
        let area = Rect::new(0, 0, 80, 24);
        let prev = Buffer::empty(area);
        let next = prev.clone(); // Same symbol table!

        b.iter(|| prev.diff(&next).count())
    });

    group.bench_function("ctui_80x24_clone_50percent_change", |b| {
        use ctui_core::{Buffer, Cell, Rect};
        let area = Rect::new(0, 0, 80, 24);
        let prev = Buffer::empty(area);
        let mut next = prev.clone(); // Same symbol table!
        for y in 0..12 {
            for x in 0..area.width {
                next.set(x, y, Cell::new("X"));
            }
        }

        b.iter(|| prev.diff(&next).count())
    });

    group.bench_function("ctui_80x24_clone_full_change", |b| {
        use ctui_core::{Buffer, Cell, Rect};
        let area = Rect::new(0, 0, 80, 24);
        let prev = Buffer::empty(area);
        let mut next = prev.clone(); // Same symbol table!
        for y in 0..area.height {
            for x in 0..area.width {
                next.set(x, y, Cell::new("X"));
            }
        }

        b.iter(|| prev.diff(&next).count())
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(5));
    targets = buffer_diff_no_changes, buffer_diff_full_change, buffer_diff_partial_change, buffer_diff_large, buffer_diff_shared_table
);

criterion_main!(benches);
