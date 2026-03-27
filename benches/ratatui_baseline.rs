//! Baseline benchmarks for ratatui primitives
//!
//! These benchmarks establish performance baselines for core ratatui operations.
//! We target beating these by >=10% with our ctui implementation.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Paragraph};

/// Benchmark Buffer::empty() creation
/// Measures allocation overhead for typical terminal size
fn buffer_empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_empty");

    // 80x24 is the most common terminal size
    group.throughput(Throughput::Elements(80 * 24));
    group.bench_function("80x24", |b| {
        b.iter(|| Buffer::empty(Rect::new(0, 0, 80, 24)))
    });

    // Also test larger sizes to understand scaling
    group.throughput(Throughput::Elements(120 * 40));
    group.bench_function("120x40", |b| {
        b.iter(|| Buffer::empty(Rect::new(0, 0, 120, 40)))
    });

    group.throughput(Throughput::Elements(200 * 50));
    group.bench_function("200x50", |b| {
        b.iter(|| Buffer::empty(Rect::new(0, 0, 200, 50)))
    });

    group.finish();
}

/// Benchmark Buffer::filled() creation
/// Measures allocation + initialization overhead
fn buffer_filled(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_filled");

    group.throughput(Throughput::Elements(80 * 24));
    group.bench_function("80x24", |b| {
        b.iter(|| Buffer::filled(Rect::new(0, 0, 80, 24), ratatui::buffer::Cell::default()))
    });

    group.throughput(Throughput::Elements(120 * 40));
    group.bench_function("120x40", |b| {
        b.iter(|| Buffer::filled(Rect::new(0, 0, 120, 40), ratatui::buffer::Cell::default()))
    });

    group.throughput(Throughput::Elements(200 * 50));
    group.bench_function("200x50", |b| {
        b.iter(|| Buffer::filled(Rect::new(0, 0, 200, 50), ratatui::buffer::Cell::default()))
    });

    group.finish();
}

/// Benchmark simple Paragraph widget rendering
/// This is a common operation in TUI apps
fn paragraph_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("paragraph_render");

    // Simple single-line paragraph
    let paragraph = Paragraph::new("Hello, World!");
    let area = Rect::new(0, 0, 80, 1);

    group.throughput(Throughput::Elements(80));
    group.bench_function("single_line_80x1", |b| {
        let mut buffer = Buffer::empty(area);
        b.iter(|| {
            paragraph.clone().render(area, &mut buffer);
        })
    });

    // Multi-line paragraph
    let lines: Vec<Line<'static>> = (0..24)
        .map(|i| Line::from(format!("Line {i}: Some content here for testing")))
        .collect();
    let multi_para = Paragraph::new(lines);
    let area_24 = Rect::new(0, 0, 80, 24);

    group.throughput(Throughput::Elements(80 * 24));
    group.bench_function("multi_line_80x24", |b| {
        let mut buffer = Buffer::empty(area_24);
        b.iter(|| {
            multi_para.clone().render(area_24, &mut buffer);
        })
    });

    // Large paragraph (stress test)
    let large_lines: Vec<Line<'static>> = (0..50)
        .map(|i| {
            Line::from(format!(
                "Line {i}: More content for a larger area test with more text"
            ))
        })
        .collect();
    let large_para = Paragraph::new(large_lines);
    let area_50 = Rect::new(0, 0, 200, 50);

    group.throughput(Throughput::Elements(200 * 50));
    group.bench_function("large_200x50", |b| {
        let mut buffer = Buffer::empty(area_50);
        b.iter(|| {
            large_para.clone().render(area_50, &mut buffer);
        })
    });

    group.finish();
}

/// Benchmark buffer diff operation (critical for performance)
/// This is the operation we most need to optimize
fn buffer_diff(c: &mut Criterion) {
    use ratatui::buffer::Buffer;

    let mut group = c.benchmark_group("buffer_diff");

    group.throughput(Throughput::Elements(80 * 24));
    group.bench_function("80x24_full_diff", |b| {
        let prev = Buffer::empty(Rect::new(0, 0, 80, 24));
        let next = {
            let mut buf = Buffer::empty(Rect::new(0, 0, 80, 24));
            // Fill with different content
            for y in 0..24 {
                for x in 0..80 {
                    buf[(x, y)].set_char('X');
                }
            }
            buf
        };
        b.iter(|| {
            let diff = next.diff(&prev);
            diff
        })
    });

    // Partial diff (50% changed)
    group.bench_function("80x24_partial_diff", |b| {
        let prev = Buffer::empty(Rect::new(0, 0, 80, 24));
        let next = {
            let mut buf = Buffer::empty(Rect::new(0, 0, 80, 24));
            // Only change first 12 lines
            for y in 0..12 {
                for x in 0..80 {
                    buf[(x, y)].set_char('Y');
                }
            }
            buf
        };
        b.iter(|| {
            let diff = next.diff(&prev);
            diff
        })
    });

    group.finish();
}

use ratatui::widgets::Widget;

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(5));
    targets = buffer_empty, buffer_filled, paragraph_render, buffer_diff
);

criterion_main!(benches);
