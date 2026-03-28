//! Memory benchmarks for Buffer and Cell types
//!
//! Measures per-cell memory size and total buffer allocation.
//! This baseline helps track memory efficiency improvements.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use ctui_core::buffer::Buffer;
use ctui_core::cell::Cell;
use ctui_core::geometry::Rect;
use std::mem::size_of;

/// Reports cell memory metrics (runs once, not iterated)
fn cell_size_info(c: &mut Criterion) {
    // This uses Criterion's throughput mechanism to report static info
    // We bench a no-op to display cell size in the output
    let mut group = c.benchmark_group("cell_size");

    group.throughput(Throughput::Bytes(size_of::<Cell>() as u64));
    group.bench_function("size_of_cell", |b| b.iter(|| size_of::<Cell>()));

    group.finish();
}

/// Benchmark buffer memory allocation for empty buffers
fn buffer_allocation_empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_allocation_empty");

    // Report total allocation size in throughput
    let cell_size = size_of::<Cell>();

    // 80x24 = 1920 cells (common terminal size)
    let size_80x24 = 80 * 24 * cell_size;
    group.throughput(Throughput::Bytes(size_80x24 as u64));
    group.bench_function("80x24", |b| {
        b.iter(|| Buffer::empty(Rect::new(0, 0, 80, 24)))
    });

    // 120x40 = 4800 cells (large terminal)
    let size_120x40 = 120 * 40 * cell_size;
    group.throughput(Throughput::Bytes(size_120x40 as u64));
    group.bench_function("120x40", |b| {
        b.iter(|| Buffer::empty(Rect::new(0, 0, 120, 40)))
    });

    // 200x50 = 10000 cells (extra large)
    let size_200x50 = 200 * 50 * cell_size;
    group.throughput(Throughput::Bytes(size_200x50 as u64));
    group.bench_function("200x50", |b| {
        b.iter(|| Buffer::empty(Rect::new(0, 0, 200, 50)))
    });

    group.finish();
}

/// Benchmark buffer memory allocation for filled buffers
fn buffer_allocation_filled(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_allocation_filled");

    let cell_size = size_of::<Cell>();
    let cell = Cell::default();

    // 80x24 = 1920 cells
    let size_80x24 = 80 * 24 * cell_size;
    group.throughput(Throughput::Bytes(size_80x24 as u64));
    group.bench_function("80x24", |b| {
        b.iter(|| Buffer::filled(Rect::new(0, 0, 80, 24), cell.clone()))
    });

    // 120x40 = 4800 cells
    let size_120x40 = 120 * 40 * cell_size;
    group.throughput(Throughput::Bytes(size_120x40 as u64));
    group.bench_function("120x40", |b| {
        b.iter(|| Buffer::filled(Rect::new(0, 0, 120, 40), cell.clone()))
    });

    // 200x50 = 10000 cells
    let size_200x50 = 200 * 50 * cell_size;
    group.throughput(Throughput::Bytes(size_200x50 as u64));
    group.bench_function("200x50", |b| {
        b.iter(|| Buffer::filled(Rect::new(0, 0, 200, 50), cell.clone()))
    });

    group.finish();
}

/// Benchmark total memory footprint verification
fn memory_footprint_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_footprint");

    // Verify cell size at runtime
    group.throughput(Throughput::Elements(1));
    group.bench_function("verify_cell_size", |b| {
        b.iter(|| {
            let cell_size = size_of::<Cell>();
            let buffer_overhead = size_of::<Buffer>();
            // Calculate expected total memory for a buffer
            let cells = 80 * 24;
            let total = buffer_overhead + cells * cell_size;
            total
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(5));
    targets = cell_size_info, buffer_allocation_empty, buffer_allocation_filled, memory_footprint_verification
);

criterion_main!(benches);
