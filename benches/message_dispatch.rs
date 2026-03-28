//! Message dispatch benchmarks comparing Box<dyn Msg> vs MessagePool
//!
//! This benchmark measures the allocation overhead difference between:
//! - Box<dyn Msg>: Traditional heap allocation for each message
//! - MessagePool<T>: Arena-based allocation using typed_arena
//!
//! The MessagePool approach uses bump allocation, avoiding the overhead
//! of individual heap allocations and providing better cache locality.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use ctui_core::Msg;

/// Test message types for benchmarking
#[derive(Debug, Clone)]
struct ClickMsg {
    x: u32,
    y: u32,
}
impl Msg for ClickMsg {}

#[derive(Debug, Clone)]
struct KeyMsg {
    key: u32,
    modifiers: u8,
}
impl Msg for KeyMsg {}

#[derive(Debug, Clone)]
struct TextMsg {
    buffer: [u8; 64],
    len: usize,
}
impl Msg for TextMsg {}

/// Benchmark: Box<dyn Msg> allocation and dispatch
///
/// Measures the cost of creating a new Box<dyn Msg> for each message,
/// which involves:
/// 1. Heap allocation
/// 2. VTable construction
/// 3. Pointer indirection
fn bench_boxed_message(c: &mut Criterion) {
    let mut group = c.benchmark_group("boxed_message");

    // Single allocation
    group.throughput(Throughput::Elements(1));
    group.bench_function("single_box_click", |b| {
        b.iter(|| {
            let msg: Box<dyn Msg> = Box::new(ClickMsg { x: 100, y: 200 });
            msg
        })
    });

    group.bench_function("single_box_key", |b| {
        b.iter(|| {
            let msg: Box<dyn Msg> = Box::new(KeyMsg { key: 65, modifiers: 0 });
            msg
        })
    });

    group.bench_function("single_box_text", |b| {
        b.iter(|| {
            let msg: Box<dyn Msg> = Box::new(TextMsg {
                buffer: [0; 64],
                len: 10,
            });
            msg
        })
    });

    // Batch allocations (simulating hot path)
    group.throughput(Throughput::Elements(100));
    group.bench_function("batch_100_box_click", |b| {
        b.iter(|| {
            let msgs: Vec<Box<dyn Msg>> = (0..100)
                .map(|i| Box::new(ClickMsg { x: i, y: i * 2 }) as Box<dyn Msg>)
                .collect();
            msgs
        })
    });

    group.bench_function("batch_100_box_key", |b| {
        b.iter(|| {
            let msgs: Vec<Box<dyn Msg>> = (0..100)
                .map(|i| Box::new(KeyMsg { key: i, modifiers: 0 }) as Box<dyn Msg>)
                .collect();
            msgs
        })
    });

    // High-frequency dispatch simulation (1000 messages)
    group.throughput(Throughput::Elements(1000));
    group.bench_function("dispatch_1000_box", |b| {
        b.iter(|| {
            let mut count = 0u32;
            for i in 0..1000u32 {
                let msg: Box<dyn Msg> = Box::new(ClickMsg { x: i, y: i });
                // Simulate type erasure cost by accessing through dyn trait
                let _: &dyn Msg = msg.as_ref();
                count += 1;
            }
            count
        })
    });

    group.finish();
}

/// Benchmark: MessagePool allocation and dispatch
///
/// Measures the cost of using MessagePool for message allocation,
/// which uses arena-based bump allocation.
#[cfg(feature = "component-pool")]
fn bench_pooled_message(c: &mut Criterion) {
    use ctui_core::MessagePool;

    let mut group = c.benchmark_group("pooled_message");

    // Single allocation
    group.throughput(Throughput::Elements(1));
    group.bench_function("single_pool_click", |b| {
        let pool: MessagePool<ClickMsg> = MessagePool::new();
        b.iter(|| {
            let msg = pool.acquire(ClickMsg { x: 100, y: 200 });
            msg.x
        })
    });

    group.bench_function("single_pool_key", |b| {
        let pool: MessagePool<KeyMsg> = MessagePool::new();
        b.iter(|| {
            let msg = pool.acquire(KeyMsg { key: 65, modifiers: 0 });
            msg.key
        })
    });

    group.bench_function("single_pool_text", |b| {
        let pool: MessagePool<TextMsg> = MessagePool::new();
        b.iter(|| {
            let msg = pool.acquire(TextMsg {
                buffer: [0; 64],
                len: 10,
            });
            msg.len
        })
    });

    // Batch allocations (simulating hot path)
    group.throughput(Throughput::Elements(100));
    group.bench_function("batch_100_pool_click", |b| {
        let pool: MessagePool<ClickMsg> = MessagePool::with_capacity(100);
        b.iter(|| {
            for i in 0..100u32 {
                let _ = pool.acquire(ClickMsg { x: i, y: i * 2 });
            }
            pool.len()
        })
    });

    group.bench_function("batch_100_pool_key", |b| {
        let pool: MessagePool<KeyMsg> = MessagePool::with_capacity(100);
        b.iter(|| {
            for i in 0..100u32 {
                let _ = pool.acquire(KeyMsg { key: i, modifiers: 0 });
            }
            pool.len()
        })
    });

    // High-frequency dispatch simulation (1000 messages)
    group.throughput(Throughput::Elements(1000));
    group.bench_function("dispatch_1000_pool", |b| {
        let pool: MessagePool<ClickMsg> = MessagePool::with_capacity(1000);
        b.iter(|| {
            let mut count = 0u32;
            for i in 0..1000u32 {
                let msg = pool.acquire(ClickMsg { x: i, y: i });
                count += msg.x;
            }
            count
        })
    });

    group.finish();
}

/// Compare Box vs Pool allocation overhead
#[cfg(feature = "component-pool")]
fn bench_comparison(c: &mut Criterion) {
    use ctui_core::MessagePool;

    let mut group = c.benchmark_group("comparison");

    // 1000 allocations comparison
    group.throughput(Throughput::Elements(1000));

    group.bench_function("box_vs_pool_1000", |b| {
        b.iter_batched(
            || {
                // Setup: create fresh pool each batch
                MessagePool::<ClickMsg>::with_capacity(1000)
            },
            |pool| {
                // Boxed version
                let boxed: Vec<Box<dyn Msg>> = (0..1000)
                    .map(|i| Box::new(ClickMsg { x: i, y: i }) as Box<dyn Msg>)
                    .collect();

                // Pooled version
                for i in 0..1000u32 {
                    let _ = pool.acquire(ClickMsg { x: i, y: i });
                }
                (boxed.len(), pool.len())
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

#[cfg(not(feature = "component-pool"))]
fn bench_pooled_message(_c: &mut Criterion) {
    // Placeholder when component-pool feature is not enabled
}

#[cfg(not(feature = "component-pool"))]
fn bench_comparison(_c: &mut Criterion) {
    // Placeholder when component-pool feature is not enabled
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(5));
    targets = bench_boxed_message, bench_pooled_message, bench_comparison
);

criterion_main!(benches);
