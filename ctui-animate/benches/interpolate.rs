use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ctui_animate::{
    interpolate_color, interpolate_position, interpolate_size, lerp, AnimationScheduler,
    EasingFunction, Interpolator, TransitionBuilder, TransitionContext,
};
use ctui_core::geometry::{Position, Size};
use ctui_core::style::Color;

// ============================================================================
// Trait-based interpolation benchmarks
// ============================================================================

fn bench_lerp_f32(c: &mut Criterion) {
    let a = 0.0_f32;
    let b = 100.0_f32;

    c.bench_function("lerp_f32", |bencher| {
        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(lerp(a, b, t));
            }
        });
    });
}

fn bench_interpolate_numeric(c: &mut Criterion) {
    let mut group = c.benchmark_group("numeric_trait");

    group.bench_function("f32_interpolate", |bencher| {
        let a = 0.0_f32;
        let b = 100.0_f32;
        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(a.interpolate(&b, t));
            }
        });
    });

    group.bench_function("f64_interpolate", |bencher| {
        let a = 0.0_f64;
        let b = 100.0_f64;
        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(a.interpolate(&b, t));
            }
        });
    });

    group.bench_function("u16_interpolate", |bencher| {
        let a: u16 = 0;
        let b: u16 = 1000;
        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(a.interpolate(&b, t));
            }
        });
    });

    group.bench_function("i32_interpolate", |bencher| {
        let a: i32 = -500;
        let b: i32 = 500;
        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(a.interpolate(&b, t));
            }
        });
    });

    group.finish();
}

fn bench_interpolate_color_trait(c: &mut Criterion) {
    let mut group = c.benchmark_group("color_trait");

    group.bench_function("rgb_to_rgb", |bencher| {
        let red = Color::Rgb(255, 0, 0);
        let blue = Color::Rgb(0, 0, 255);

        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(red.interpolate(&blue, t));
            }
        });
    });

    group.bench_function("named_to_named", |bencher| {
        let red = Color::Red;
        let blue = Color::Blue;

        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(red.interpolate(&blue, t));
            }
        });
    });

    group.finish();
}

fn bench_interpolate_geometry_trait(c: &mut Criterion) {
    let mut group = c.benchmark_group("geometry_trait");

    group.bench_function("position", |bencher| {
        let start = Position::new(0, 0);
        let end = Position::new(1000, 1000);

        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(start.interpolate(&end, t));
            }
        });
    });

    group.bench_function("size", |bencher| {
        let small = Size::new(10, 20);
        let large = Size::new(1000, 2000);

        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(small.interpolate(&large, t));
            }
        });
    });

    group.finish();
}

fn bench_interpolate_arrays(c: &mut Criterion) {
    let mut group = c.benchmark_group("arrays_trait");

    group.bench_function("array_2", |bencher| {
        let a = [0.0_f32, 100.0];
        let b = [100.0_f32, 0.0];

        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(a.interpolate(&b, t));
            }
        });
    });

    group.bench_function("array_4", |bencher| {
        let a = [0.0_f32, 100.0, 200.0, 300.0];
        let b = [300.0_f32, 200.0, 100.0, 0.0];

        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                black_box(a.interpolate(&b, t));
            }
        });
    });

    group.finish();
}

fn bench_real_world_frame(c: &mut Criterion) {
    c.bench_function("frame_interpolation_100_cells", |bencher| {
        let fg_start = Color::Rgb(255, 0, 0);
        let fg_end = Color::Rgb(0, 255, 0);
        let bg_start = Color::Rgb(0, 0, 0);
        let bg_end = Color::Rgb(50, 50, 50);
        let pos_start = Position::new(0, 0);
        let pos_end = Position::new(80, 24);

        bencher.iter(|| {
            for i in 0..100 {
                let t = i as f32 / 100.0;
                let fg = fg_start.interpolate(&fg_end, t);
                let bg = bg_start.interpolate(&bg_end, t);
                let pos = pos_start.interpolate(&pos_end, t);
                black_box((fg, bg, pos));
            }
        });
    });
}

// ============================================================================
// Legacy function benchmarks (from transition module)
// ============================================================================

fn bench_color_interpolation(c: &mut Criterion) {
    let from = Color::Rgb(0, 0, 0);
    let to = Color::Rgb(255, 255, 255);

    c.bench_function("interpolate_color", |b| {
        b.iter(|| interpolate_color(&from, &to, 0.5))
    });
}

fn bench_position_interpolation(c: &mut Criterion) {
    let from = Position::new(0, 0);
    let to = Position::new(1000, 1000);

    c.bench_function("interpolate_position", |b| {
        b.iter(|| interpolate_position(&from, &to, 0.5))
    });
}

fn bench_size_interpolation(c: &mut Criterion) {
    let from = Size::new(10, 10);
    let to = Size::new(100, 100);

    c.bench_function("interpolate_size", |b| {
        b.iter(|| interpolate_size(&from, &to, 0.5))
    });
}

fn bench_transition_build(c: &mut Criterion) {
    c.bench_function("transition_build", |b| {
        b.iter(|| {
            TransitionBuilder::new()
                .color(Color::Red, Color::Blue)
                .duration_ms(500)
                .easing(EasingFunction::CubicOut)
                .build()
        })
    });
}

// ============================================================================
// Easing function evaluation benchmarks
// ============================================================================

fn bench_easing_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("easing");

    // All easing functions at various progress values
    let easings = [
        EasingFunction::Linear,
        EasingFunction::QuadIn,
        EasingFunction::QuadOut,
        EasingFunction::QuadInOut,
        EasingFunction::CubicIn,
        EasingFunction::CubicOut,
        EasingFunction::CubicInOut,
        EasingFunction::SineIn,
        EasingFunction::SineOut,
        EasingFunction::SineInOut,
        EasingFunction::ElasticOut,
        EasingFunction::BounceOut,
    ];

    for easing in easings {
        let name = format!("{:?}", easing);
        group.bench_function(name, |bencher| {
            bencher.iter(|| {
                for i in 0..100 {
                    let t = i as f64 / 100.0;
                    black_box(easing.eval(t));
                }
            });
        });
    }

    group.finish();
}

fn bench_easing_eval_single(c: &mut Criterion) {
    c.bench_function("easing_eval_single", |bencher| {
        bencher.iter(|| black_box(EasingFunction::CubicOut.eval(0.5)));
    });
}

// ============================================================================
// Scheduler tick overhead benchmarks
// ============================================================================

fn bench_scheduler_tick_empty(c: &mut Criterion) {
    c.bench_function("scheduler_tick_empty", |bencher| {
        let mut scheduler = AnimationScheduler::new();
        bencher.iter(|| black_box(scheduler.tick(16)));
    });
}

fn bench_scheduler_tick_single(c: &mut Criterion) {
    c.bench_function("scheduler_tick_1_animation", |bencher| {
        let mut scheduler = AnimationScheduler::new();
        let _ = scheduler.spawn(1000, EasingFunction::Linear);
        bencher.iter(|| {
            black_box(scheduler.tick(16));
        });
    });
}

fn bench_scheduler_tick_multiple(c: &mut Criterion) {
    let mut group = c.benchmark_group("scheduler_tick");

    for count in [5, 10, 50, 100] {
        group.bench_function(format!("{}_animations", count), |bencher| {
            let mut scheduler = AnimationScheduler::new();
            for _ in 0..count {
                let _ = scheduler.spawn(1000, EasingFunction::CubicOut);
            }
            bencher.iter(|| {
                black_box(scheduler.tick(16));
            });
        });
    }

    group.finish();
}

// ============================================================================
// TransitionContext tick overhead benchmarks
// ============================================================================

fn bench_transition_context_tick_empty(c: &mut Criterion) {
    c.bench_function("transition_context_tick_empty", |bencher| {
        let mut ctx = TransitionContext::new();
        bencher.iter(|| black_box(ctx.tick(16)));
    });
}

fn bench_transition_context_tick_single(c: &mut Criterion) {
    c.bench_function("transition_context_tick_1_transition", |bencher| {
        let mut ctx = TransitionContext::new();
        ctx.start(
            TransitionBuilder::new()
                .color(Color::Red, Color::Blue)
                .duration_ms(1000)
                .build(),
        );
        bencher.iter(|| {
            black_box(ctx.tick(16));
        });
    });
}

fn bench_transition_context_tick_multiple(c: &mut Criterion) {
    let mut group = c.benchmark_group("transition_context_tick");

    for count in [5, 10, 50] {
        group.bench_function(format!("{}_transitions", count), |bencher| {
            let mut ctx = TransitionContext::new();
            for i in 0..count {
                let color_from = Color::Rgb(i as u8, 0, 0);
                let color_to = Color::Rgb(0, 0, i as u8);
                ctx.start(
                    TransitionBuilder::new()
                        .color(color_from, color_to)
                        .duration_ms(1000)
                        .build(),
                );
            }
            bencher.iter(|| {
                black_box(ctx.tick(16));
            });
        });
    }

    group.finish();
}

// ============================================================================
// Frame overhead simulation (realistic 16ms frame budget)
// ============================================================================

fn bench_frame_overhead(c: &mut Criterion) {
    c.bench_function("frame_overhead_simulation", |bencher| {
        let mut scheduler = AnimationScheduler::new();
        let mut ctx = TransitionContext::new();

        for _ in 0..5 {
            let _ = scheduler.spawn(1000, EasingFunction::CubicOut);
        }
        ctx.start(
            TransitionBuilder::new()
                .color(Color::Red, Color::Blue)
                .duration_ms(500)
                .build(),
        );
        ctx.start(
            TransitionBuilder::new()
                .position(Position::new(0, 0), Position::new(100, 100))
                .duration_ms(500)
                .build(),
        );
        ctx.start(
            TransitionBuilder::new()
                .opacity(0.0, 1.0)
                .duration_ms(500)
                .build(),
        );

        bencher.iter(|| {
            let _ = black_box(scheduler.tick(16));
            let _ = black_box(ctx.tick(16));
        });
    });
}

criterion_group!(
    benches,
    bench_lerp_f32,
    bench_interpolate_numeric,
    bench_interpolate_color_trait,
    bench_interpolate_geometry_trait,
    bench_interpolate_arrays,
    bench_real_world_frame,
    bench_color_interpolation,
    bench_position_interpolation,
    bench_size_interpolation,
    bench_transition_build,
    bench_easing_functions,
    bench_easing_eval_single,
    bench_scheduler_tick_empty,
    bench_scheduler_tick_single,
    bench_scheduler_tick_multiple,
    bench_transition_context_tick_empty,
    bench_transition_context_tick_single,
    bench_transition_context_tick_multiple,
    bench_frame_overhead,
);

criterion_main!(benches);
