//! Benchmark circuit breaker
//!
//! Run with: cargo bench -p hodei-audit-benchmarks circuit_breaker

use criterion::{Criterion, Throughput, black_box};
use tokio::runtime::Runtime;

use hodei_audit_service::performance::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};

fn bench_circuit_breaker(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("circuit_breaker");

    group.throughput(Throughput::Elements(10000));

    // Benchmark state transitions
    group.bench_function("state_transitions", |b| {
        b.to_async(&rt).iter(|| async {
            let mut breaker = CircuitBreaker::new(CircuitBreakerConfig {
                failure_threshold: 5,
                success_threshold: 3,
                timeout: std::time::Duration::from_secs(60),
                error_rate_threshold: 0.5,
                min_request_threshold: 10,
                rolling_window: std::time::Duration::from_secs(60),
                auto_recovery: true,
            });

            for _ in 0..5 {
                // Simulate successful call
                black_box(
                    breaker
                        .record_success(std::time::Duration::from_millis(1))
                        .await,
                );
                // Simulate failed call
                black_box(breaker.record_failure().await);
            }
        });
    });

    // Benchmark in closed state
    group.bench_function("closed_state", |b| {
        b.to_async(&rt).iter(|| async {
            let breaker = CircuitBreaker::new(CircuitBreakerConfig {
                failure_threshold: 5,
                success_threshold: 3,
                timeout: std::time::Duration::from_secs(60),
                error_rate_threshold: 0.5,
                min_request_threshold: 10,
                rolling_window: std::time::Duration::from_secs(60),
                auto_recovery: true,
            });

            for _ in 0..100 {
                black_box(
                    breaker
                        .record_success(std::time::Duration::from_millis(1))
                        .await,
                );
            }
        });
    });

    group.finish();
}

criterion::criterion_group!(benches, bench_circuit_breaker);
criterion::criterion_main!(benches);
