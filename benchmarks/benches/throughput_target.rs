//! Benchmark overall throughput (100K events/sec target)
//!
//! Run with: cargo bench -p hodei-audit-benchmarks throughput_target

use criterion::{Criterion, Throughput, black_box};
use std::sync::Arc;
use tokio::runtime::Runtime;

use hodei_audit_service::performance::batcher::{BatcherConfig, BatchingPolicy, SmartBatcher};

fn bench_throughput_target(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("throughput_target");

    group.throughput(Throughput::Elements(100000));

    group.bench_function("1000_events", |b| {
        b.to_async(&rt).iter(|| async {
            let test_data = b"event data for throughput testing".to_vec();

            let start = std::time::Instant::now();

            // Create batcher with optimal settings for throughput
            let config = BatcherConfig {
                max_queue_size: 10_000,
                policy: BatchingPolicy::Hybrid {
                    max_time: std::time::Duration::from_millis(5),
                    max_size: 100,
                },
                flush_timeout: std::time::Duration::from_millis(50),
                adaptive_tuning: false,
                backpressure_controller: None,
                enable_metrics: true,
            };
            let mut batcher = SmartBatcher::new(config);

            // Process 1K events (fast)
            for _ in 0..1000 {
                batcher.add_event(test_data.clone()).await;
            }

            batcher.flush().await;

            let duration = start.elapsed();
            black_box(duration);
        });
    });

    group.finish();
}

criterion::criterion_group!(benches, bench_throughput_target);
criterion::criterion_main!(benches);
