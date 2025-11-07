//! Benchmark backpressure controller
//!
//! Run with: cargo bench -p hodei-audit-benchmarks backpressure_controller

use criterion::{Criterion, Throughput, black_box};
use std::sync::Arc;
use tokio::runtime::Runtime;

use hodei_audit_service::performance::backpressure::BackpressureController;

fn bench_backpressure_controller(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("backpressure_controller");

    group.throughput(Throughput::Elements(10000));

    group.bench_function("evaluate", |b| {
        b.to_async(&rt).iter(|| async {
            let controller = BackpressureController::new(
                hodei_audit_service::performance::backpressure::BackpressureConfig {
                    queue_size_warnings: (5_000, 8_000, 9_500),
                    rate_warnings: (5_000, 8_000, 10_000),
                    rate_window: std::time::Duration::from_millis(100),
                    auto_recovery: true,
                    recovery_delay: std::time::Duration::from_millis(500),
                    enable_metrics: true,
                },
            );
            for i in 0..100 {
                black_box(controller.evaluate().await);
            }
        });
    });

    group.bench_function("get_metrics", |b| {
        b.to_async(&rt).iter(|| async {
            let controller = BackpressureController::new(
                hodei_audit_service::performance::backpressure::BackpressureConfig {
                    queue_size_warnings: (5_000, 8_000, 9_500),
                    rate_warnings: (5_000, 8_000, 10_000),
                    rate_window: std::time::Duration::from_millis(100),
                    auto_recovery: true,
                    recovery_delay: std::time::Duration::from_millis(500),
                    enable_metrics: true,
                },
            );
            for i in 0..100 {
                let _ = black_box(controller.get_metrics().await);
            }
        });
    });

    group.finish();
}

criterion::criterion_group!(benches, bench_backpressure_controller);
criterion::criterion_main!(benches);
