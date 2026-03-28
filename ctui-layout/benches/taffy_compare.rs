//! Benchmark comparing cTUI FlexLayout vs Taffy layout engine
#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use ctui_core::Rect;
use ctui_layout::{Constraint, FlexDirection, JustifyContent, Layout};

#[cfg(feature = "taffy-layout")]
use ctui_layout::TaffyLayoutEngine;

fn bench_flex_native(c: &mut Criterion) {
    let area = Rect::new(0, 0, 80, 24);
    let constraints = vec![
        Constraint::Length(20),
        Constraint::Min(10),
        Constraint::Percentage(25),
        Constraint::Fill,
    ];

    c.bench_function("native_flex_split_row_4_children", |b| {
        b.iter_batched(
            || Layout::row(),
            |layout| black_box(layout.split(area, &constraints)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("native_flex_split_column_4_children", |b| {
        b.iter_batched(
            || Layout::column(),
            |layout| black_box(layout.split(area, &constraints)),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "taffy-layout")]
fn bench_taffy_engine(c: &mut Criterion) {
    let area = Rect::new(0, 0, 80, 24);
    let constraints = vec![
        Constraint::Length(20),
        Constraint::Min(10),
        Constraint::Percentage(25),
        Constraint::Fill,
    ];

    c.bench_function("taffy_flex_split_row_4_children", |b| {
        b.iter_batched(
            || TaffyLayoutEngine::new().direction(FlexDirection::Row),
            |engine| black_box(engine.split(area, &constraints)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("taffy_flex_split_column_4_children", |b| {
        b.iter_batched(
            || TaffyLayoutEngine::new().direction(FlexDirection::Column),
            |engine| black_box(engine.split(area, &constraints)),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "taffy-layout")]
fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("native_vs_taffy");
    group.throughput(Throughput::Elements(10));

    let area = Rect::new(0, 0, 100, 24);
    let constraints: Vec<_> = (0..10).map(|i| {
        if i % 3 == 0 { Constraint::Length(10) }
        else if i % 3 == 1 { Constraint::Percentage(10) }
        else { Constraint::Fill }
    }).collect();

    group.bench_function("native_10_children", |b| {
        b.iter_batched(
            || Layout::row().gap(1),
            |layout| black_box(layout.split(area, &constraints)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("taffy_10_children", |b| {
        b.iter_batched(
            || TaffyLayoutEngine::new().direction(FlexDirection::Row).gap(1),
            |engine| black_box(engine.split(area, &constraints)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

#[cfg(feature = "taffy-layout")]
criterion_group!(
    benches,
    bench_flex_native,
    bench_taffy_engine,
    bench_comparison,
);

#[cfg(not(feature = "taffy-layout"))]
criterion_group!(benches, bench_flex_native);

criterion_main!(benches);
