#![allow(missing_docs)]

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use ctui_core::{terminal::LayoutCache, Rect};
use ctui_layout::{AlignItems, Constraint, JustifyContent, Layout};

fn bench_split(c: &mut Criterion) {
    let area = Rect::new(0, 0, 80, 24);
    let constraints = vec![
        Constraint::Length(20),
        Constraint::Min(10),
        Constraint::Percentage(25),
        Constraint::Fill,
    ];

    c.bench_function("flex_split_row_4_children", |b| {
        b.iter_batched(
            || Layout::row(),
            |layout| black_box(layout.split(area, &constraints)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("flex_split_column_4_children", |b| {
        b.iter_batched(
            || Layout::column(),
            |layout| black_box(layout.split(area, &constraints)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_justify_content(c: &mut Criterion) {
    let area = Rect::new(0, 0, 80, 24);
    let constraints = vec![
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
    ];

    let mut group = c.benchmark_group("justify_content");
    group.throughput(Throughput::Elements(3));

    for (name, justify) in [
        ("start", JustifyContent::Start),
        ("center", JustifyContent::Center),
        ("end", JustifyContent::End),
        ("space_between", JustifyContent::SpaceBetween),
        ("space_around", JustifyContent::SpaceAround),
    ] {
        group.bench_function(name, |b| {
            b.iter_batched(
                || Layout::row().justify_content(justify),
                |layout| black_box(layout.split(area, &constraints)),
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn bench_align_items(c: &mut Criterion) {
    let area = Rect::new(0, 0, 80, 24);
    let constraints = vec![Constraint::Length(20)];

    let mut group = c.benchmark_group("align_items");
    group.throughput(Throughput::Elements(1));

    for (name, align) in [
        ("start", AlignItems::Start),
        ("center", AlignItems::Center),
        ("end", AlignItems::End),
        ("stretch", AlignItems::Stretch),
    ] {
        group.bench_function(name, |b| {
            b.iter_batched(
                || Layout::row().align_items(align),
                |layout| black_box(layout.split(area, &constraints)),
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    group.throughput(Throughput::Elements(10));

    for n in [2, 5, 10, 20].iter() {
        let constraints: Vec<_> = (0..*n).map(|_| Constraint::Length(5)).collect();
        group.throughput(Throughput::Elements(*n as u64));

        group.bench_function(format!("{}_children", n), |b| {
            b.iter_batched(
                || Layout::row().gap(1),
                |layout| {
                    let area = Rect::new(0, 0, 80, 24);
                    black_box(layout.split(area, &constraints))
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn bench_mixed_constraints(c: &mut Criterion) {
    let area = Rect::new(0, 0, 100, 24);
    let mixed = vec![
        Constraint::Length(20),
        Constraint::Percentage(10),
        Constraint::Min(5),
        Constraint::Max(30),
        Constraint::Fill,
        Constraint::Ratio(1, 2),
    ];

    c.bench_function("mixed_constraints_6_children", |b| {
        b.iter_batched(
            || Layout::row().gap(2),
            |layout| black_box(layout.split(area, &mixed)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_layout_cache(c: &mut Criterion) {
    let area = Rect::new(0, 0, 80, 24);
    let constraints: Vec<Constraint> = vec![
        Constraint::Length(20),
        Constraint::Min(10),
        Constraint::Percentage(25),
        Constraint::Fill,
    ];
    let layout = Layout::row();

    let mut group = c.benchmark_group("layout_cache");

    group.bench_function("uncached_split", |b| {
        b.iter(|| {
            black_box(layout.split(area, &constraints));
        })
    });

    group.bench_function("cached_hit", |b| {
        let mut cache = LayoutCache::new();
        cache.store(area, &constraints, layout.split(area, &constraints));
        b.iter(|| {
            black_box(cache.get(area, &constraints));
        })
    });

    group.bench_function("cached_miss", |b| {
        let mut cache = LayoutCache::new();
        let different_area = Rect::new(0, 0, 100, 30);
        b.iter(|| {
            cache.get(different_area, &constraints);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_split,
    bench_justify_content,
    bench_align_items,
    bench_scaling,
    bench_mixed_constraints,
    bench_layout_cache,
);
criterion_main!(benches);
