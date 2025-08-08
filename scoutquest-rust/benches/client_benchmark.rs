use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use scoutquest_rust::*;
use std::collections::HashMap;

fn benchmark_load_balancing(c: &mut Criterion) {
    let instances = (0..100)
        .map(|i| ServiceInstance {
            id: format!("instance-{}", i),
            service_name: "benchmark-service".to_string(),
            host: format!("host-{}", i),
            port: 3000 + i as u16,
            secure: false,
            status: if i % 10 == 0 {
                InstanceStatus::Down
            } else {
                InstanceStatus::Up
            },
            metadata: HashMap::new(),
            tags: vec!["benchmark".to_string()],
            registered_at: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
            last_status_change: chrono::Utc::now(),
        })
        .collect::<Vec<_>>();

    let load_balancer = LoadBalancer::new();

    // Benchmark different load balancing strategies
    let strategies = vec![
        ("Random", LoadBalancingStrategy::Random),
        ("RoundRobin", LoadBalancingStrategy::RoundRobin),
        ("HealthyOnly", LoadBalancingStrategy::HealthyOnly),
        ("LeastConnections", LoadBalancingStrategy::LeastConnections),
        ("WeightedRandom", LoadBalancingStrategy::WeightedRandom),
    ];

    for (name, strategy) in strategies {
        c.bench_with_input(
            BenchmarkId::new("load_balance", name),
            &strategy,
            |b, strategy| {
                b.iter(|| {
                    load_balancer
                        .select_instance(
                            std::hint::black_box(&instances),
                            std::hint::black_box(strategy),
                        )
                        .unwrap()
                })
            },
        );
    }
}

fn benchmark_service_instance_operations(c: &mut Criterion) {
    let instance = ServiceInstance {
        id: "benchmark-instance".to_string(),
        service_name: "benchmark-service".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        secure: false,
        status: InstanceStatus::Up,
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("version".to_string(), "1.0.0".to_string());
            metadata.insert("region".to_string(), "us-west-2".to_string());
            metadata
        },
        tags: vec!["web".to_string(), "api".to_string()],
        registered_at: chrono::Utc::now(),
        last_heartbeat: chrono::Utc::now(),
        last_status_change: chrono::Utc::now(),
    };

    c.bench_function("is_healthy", |b| {
        b.iter(|| std::hint::black_box(&instance).is_healthy())
    });

    c.bench_function("get_url", |b| {
        b.iter(|| std::hint::black_box(&instance).get_url("/api/v1/users"))
    });
}

fn benchmark_instance_pool_sizes(c: &mut Criterion) {
    let load_balancer = LoadBalancer::new();

    for pool_size in [1, 5, 10, 50, 100, 500, 1000].iter() {
        let instances = (0..*pool_size)
            .map(|i| ServiceInstance {
                id: format!("instance-{}", i),
                service_name: "test-service".to_string(),
                host: format!("host-{}", i),
                port: 3000 + i as u16,
                secure: false,
                status: InstanceStatus::Up,
                metadata: HashMap::new(),
                tags: Vec::new(),
                registered_at: chrono::Utc::now(),
                last_heartbeat: chrono::Utc::now(),
                last_status_change: chrono::Utc::now(),
            })
            .collect::<Vec<_>>();

        c.bench_with_input(
            BenchmarkId::new("random_selection_pool_size", pool_size),
            &instances,
            |b, instances| {
                b.iter(|| {
                    load_balancer
                        .select_instance(
                            std::hint::black_box(instances),
                            std::hint::black_box(&LoadBalancingStrategy::Random),
                        )
                        .unwrap()
                })
            },
        );

        c.bench_with_input(
            BenchmarkId::new("round_robin_selection_pool_size", pool_size),
            &instances,
            |b, instances| {
                b.iter(|| {
                    load_balancer
                        .select_instance(
                            std::hint::black_box(instances),
                            std::hint::black_box(&LoadBalancingStrategy::RoundRobin),
                        )
                        .unwrap()
                })
            },
        );

        c.bench_with_input(
            BenchmarkId::new("healthy_only_selection_pool_size", pool_size),
            &instances,
            |b, instances| {
                b.iter(|| {
                    load_balancer
                        .select_instance(
                            std::hint::black_box(instances),
                            std::hint::black_box(&LoadBalancingStrategy::HealthyOnly),
                        )
                        .unwrap()
                })
            },
        );
    }
}

fn benchmark_serialization(c: &mut Criterion) {
    let instance = ServiceInstance {
        id: "benchmark-instance".to_string(),
        service_name: "benchmark-service".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        secure: false,
        status: InstanceStatus::Up,
        metadata: {
            let mut metadata = HashMap::new();
            for i in 0..50 {
                metadata.insert(format!("key_{}", i), format!("value_{}", i));
            }
            metadata
        },
        tags: (0..20).map(|i| format!("tag_{}", i)).collect(),
        registered_at: chrono::Utc::now(),
        last_heartbeat: chrono::Utc::now(),
        last_status_change: chrono::Utc::now(),
    };

    c.bench_function("serialize_service_instance", |b| {
        b.iter(|| serde_json::to_string(std::hint::black_box(&instance)).unwrap())
    });

    let serialized = serde_json::to_string(&instance).unwrap();
    c.bench_function("deserialize_service_instance", |b| {
        b.iter(|| {
            serde_json::from_str::<ServiceInstance>(std::hint::black_box(&serialized)).unwrap()
        })
    });

    let instances_list: Vec<ServiceInstance> = (0..100)
        .map(|i| ServiceInstance {
            id: format!("instance-{}", i),
            service_name: format!("service-{}", i % 10),
            host: format!("host-{}.example.com", i),
            port: 3000 + i as u16,
            secure: i % 2 == 0,
            status: if i % 5 == 0 {
                InstanceStatus::Down
            } else {
                InstanceStatus::Up
            },
            metadata: HashMap::new(),
            tags: vec!["benchmark".to_string()],
            registered_at: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
            last_status_change: chrono::Utc::now(),
        })
        .collect();

    c.bench_function("serialize_instances_list", |b| {
        b.iter(|| serde_json::to_string(std::hint::black_box(&instances_list)).unwrap())
    });

    let serialized_list = serde_json::to_string(&instances_list).unwrap();
    c.bench_function("deserialize_instances_list", |b| {
        b.iter(|| {
            serde_json::from_str::<Vec<ServiceInstance>>(std::hint::black_box(&serialized_list))
                .unwrap()
        })
    });
}

criterion_group!(
    benches,
    benchmark_load_balancing,
    benchmark_service_instance_operations,
    benchmark_instance_pool_sizes,
    benchmark_serialization
);
criterion_main!(benches);
