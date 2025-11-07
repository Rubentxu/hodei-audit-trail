//! Benchmark SmartBatcher with different policies
//!
//! Run with: cargo bench -p hodei-audit-benchmarks smart_batcher_policies

use criterion::{Criterion, Throughput, black_box};
use std::sync::Arc;
use tokio::runtime::Runtime;

// Import the modules to benchmark
use hodei_audit_service::performance::batcher::{BatcherConfig, BatchingPolicy, SmartBatcher};

fn bench_smart_batcher_policies(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("smart_batcher_policies");

    // Setup test data
    let test_data = b"test event data with some content to process".to_vec();

    // TimeBased policy
    group.bench_function("time_based", |b| {
        b.to_async(&rt).iter(|| async {
            let config = BatcherConfig {
                max_queue_size: 1_000,
                policy: BatchingPolicy::TimeBased(std::time::Duration::from_millis(1)),
                flush_timeout: std::time::Duration::from_millis(10),
                adaptive_tuning: false,
                backpressure_controller: None,
                enable_metrics: true,
            };
            let mut batcher = SmartBatcher::new(config);
            black_box(batcher.add_event(black_box(test_data.clone())).await);
            black_box(batcher.flush().await);
        });
    });

    // SizeBased policy
    group.bench_function("size_based", |b| {
        b.to_async(&rt).iter(|| async {
            let config = BatcherConfig {
                max_queue_size: 1_000,
                policy: BatchingPolicy::SizeBased(5),
                flush_timeout: std::time::Duration::from_millis(10),
                adaptive_tuning: false,
                backpressure_controller: None,
                enable_metrics: true,
            };
            let mut batcher = SmartBatcher::new(config);
            for _ in 0..5 {
                black_box(batcher.add_event(black_box(test_data.clone())).await);
            }
            black_box(batcher.flush().await);
        });
    });

    // Hybrid policy
    group.bench_function("hybrid", |b| {
        b.to_async(&rt).iter(|| async {
            let config = BatcherConfig {
                max_queue_size: 1_000,
                policy: BatchingPolicy::Hybrid {
                    max_time: std::time::Duration::from_millis(1),
                    max_size: 5,
                },
                flush_timeout: std::time::Duration::from_millis(10),
                adaptive_tuning: false,
                backpressure_controller: None,
                enable_metrics: true,
            };
            let mut batcher = SmartBatcher::new(config);
            for _ in 0..5 {
                black_box(batcher.add_event(black_box(test_data.clone())).await);
            }
            black_box(batcher.flush().await);
        });
    });

    // Adaptive policy
    group.bench_function("adaptive", |b| {
        b.to_async(&rt).iter(|| async {
            let config = BatcherConfig {
                max_queue_size: 1_000,
                policy: BatchingPolicy::Adaptive {
                    target_throughput: 1000,
                    min_batch_size: 2,
                    max_batch_size: 10,
                    min_time: std::time::Duration::from_millis(1),
                    max_time: std::time::Duration::from_millis(10),
                },
                flush_timeout: std::time::Duration::from_millis(10),
                adaptive_tuning: false,
                backpressure_controller: None,
                enable_metrics: true,
            };
            let mut batcher = SmartBatcher::new(config);
            for _ in 0..5 {
                black_box(batcher.add_event(black_box(test_data.clone())).await);
            }
            black_box(batcher.flush().await);
        });
    });

    group.finish();
}

criterion::criterion_group!(benches, bench_smart_batcher_policies);
criterion::criterion_main!(benches);
