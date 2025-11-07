//! Benchmark connection pool performance
//!
//! Run with: cargo bench -p hodei-audit-benchmarks connection_pool

use criterion::{Criterion, Throughput, black_box};
use std::sync::Arc;
use tokio::runtime::Runtime;

use hodei_audit_service::performance::connection_pool::{ConnectionPool, PoolConfig};

fn bench_connection_pool(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("connection_pool");

    group.throughput(Throughput::Elements(1000));

    // Benchmark connection acquisition
    group.bench_function("get_3_connections", |b| {
        b.to_async(&rt).iter(|| async {
            let pool = ConnectionPool::new(PoolConfig {
                min_connections: 3,
                max_connections: 10,
                connection_timeout: std::time::Duration::from_millis(100),
                health_check_interval: std::time::Duration::from_secs(3),
                idle_timeout: std::time::Duration::from_secs(30),
                max_retries: 1,
                retry_delay: std::time::Duration::from_millis(10),
            });

            let mut connections = Vec::new();
            for _ in 0..3 {
                connections.push(black_box(pool.get().await));
            }
            black_box(connections);
        });
    });

    // Benchmark concurrent connection usage
    group.bench_function("concurrent_10_users", |b| {
        b.to_async(&rt).iter(|| async {
            let pool = Arc::new(ConnectionPool::new(PoolConfig {
                min_connections: 3,
                max_connections: 10,
                connection_timeout: std::time::Duration::from_millis(100),
                health_check_interval: std::time::Duration::from_secs(3),
                idle_timeout: std::time::Duration::from_secs(30),
                max_retries: 1,
                retry_delay: std::time::Duration::from_millis(10),
            }));

            let mut handles = Vec::new();
            for _ in 0..10 {
                let pool = pool.clone();
                handles.push(tokio::spawn(async move {
                    let conn = pool.get().await;
                    black_box(conn);
                }));
            }

            for handle in handles {
                black_box(handle.await);
            }
        });
    });

    group.finish();
}

criterion::criterion_group!(benches, bench_connection_pool);
criterion::criterion_main!(benches);
