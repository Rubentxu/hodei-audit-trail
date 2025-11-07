//! Benchmark zero-copy batching
//!
//! Run with: cargo bench -p hodei-audit-benchmarks zero_copy_batching

use criterion::{Criterion, Throughput, black_box};
use std::sync::Arc;
use tokio::runtime::Runtime;

use hodei_audit_service::zero_copy_batching::{BatcherConfig, ZeroCopyBatcher};

fn bench_zero_copy_batching(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("zero_copy_batching");

    group.throughput(Throughput::Bytes(1024 * 10)); // 10KB throughput

    let test_data = vec![0u8; 100]; // 100 bytes chunks

    // Single buffer operations
    group.bench_function("add_data_single", |b| {
        b.to_async(&rt).iter(|| async {
            let mut batcher = ZeroCopyBatcher::new(BatcherConfig {
                max_batch_size: 1024, // 1KB
                flush_timeout: std::time::Duration::from_millis(10),
            });

            for _ in 0..10 {
                black_box(batcher.add_data(black_box(&test_data)).await);
            }
            black_box(batcher.flush().await);
        });
    });

    // Multiple buffers
    group.bench_function("add_data_multiple_buffers", |b| {
        b.to_async(&rt).iter(|| async {
            let mut batcher = ZeroCopyBatcher::new(BatcherConfig {
                max_batch_size: 200, // 200 bytes
                flush_timeout: std::time::Duration::from_millis(10),
            });

            for _ in 0..20 {
                black_box(batcher.add_data(black_box(&test_data)).await);
            }
            black_box(batcher.flush().await);
        });
    });

    group.finish();
}

criterion::criterion_group!(benches, bench_zero_copy_batching);
criterion::criterion_main!(benches);
