//! Benchmark concurrent operations
//!
//! Run with: cargo bench -p hodei-audit-benchmarks concurrent_operations

use criterion::{Criterion, Throughput, black_box};
use std::sync::Arc;
use tokio::runtime::Runtime;

use hodei_audit_service::performance::batcher::{BatcherConfig, BatchingPolicy, SmartBatcher};

fn bench_concurrent_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("concurrent_operations");

    group.throughput(Throughput::Elements(50000));

    group.bench_function("concurrent_batchers", |b| {
        b.to_async(&rt).iter(|| async {
            let test_data = b"concurrent test data".to_vec();

            let mut handles = Vec::new();
            for _ in 0..3 {
                let data = test_data.clone();
                handles.push(tokio::spawn(async move {
                    let config = BatcherConfig {
                        max_queue_size: 1_000,
                        policy: BatchingPolicy::SizeBased(10),
                        flush_timeout: std::time::Duration::from_millis(10),
                        adaptive_tuning: false,
                        backpressure_controller: None,
                        enable_metrics: true,
                    };
                    let mut batcher = SmartBatcher::new(config);

                    for _ in 0..100 {
                        batcher.add_event(data.clone()).await;
                    }
                    batcher.flush().await;
                }));
            }

            for handle in handles {
                black_box(handle.await);
            }
        });
    });

    group.finish();
}

criterion::criterion_group!(benches, bench_concurrent_operations);
criterion::criterion_main!(benches);
