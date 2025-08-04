use criterion::{criterion_group, criterion_main, Criterion};
use scoutquest_rust::*;

fn benchmark_load_balancing(c: &mut Criterion) {

    let instances = (0..100).map(|i| ServiceInstance {
        id: format!("instance-{}", i),
        service_name: "benchmark-service".to_string(),
        host: format!("host-{}", i),
        port: 3000 + i as u16,
        secure: false,
        status: InstanceStatus::Up,
        metadata: std::collections::HashMap::new(),
        tags: Vec::new(),
        registered_at: chrono::Utc::now(),
        last_heartbeat: chrono::Utc::now(),
        last_status_change: chrono::Utc::now(),
    }).collect::<Vec<_>>();

    let load_balancer = LoadBalancer::new();

    c.bench_function("load_balance_random", |b| {
        b.iter(|| {
            load_balancer.select_instance(
                std::hint::black_box(&instances),
                std::hint::black_box(&LoadBalancingStrategy::Random)
            ).unwrap()
        })
    });

    c.bench_function("load_balance_round_robin", |b| {
        b.iter(|| {
            load_balancer.select_instance(
                std::hint::black_box(&instances),
                std::hint::black_box(&LoadBalancingStrategy::RoundRobin)
            ).unwrap()
        })
    });

    c.bench_function("load_balance_healthy_only", |b| {
        b.iter(|| {
            load_balancer.select_instance(
                std::hint::black_box(&instances),
                std::hint::black_box(&LoadBalancingStrategy::HealthyOnly)
            ).unwrap()
        })
    });
}

criterion_group!(benches, benchmark_load_balancing);
criterion_main!(benches);