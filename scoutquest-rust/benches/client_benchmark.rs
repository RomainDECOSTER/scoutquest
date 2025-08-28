use criterion::{criterion_group, criterion_main, Criterion};
use scoutquest_rust::*;
use std::collections::HashMap;

fn benchmark_service_discovery(c: &mut Criterion) {
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

    // Benchmark service instance filtering
    c.bench_function("filter_healthy_instances", |b| {
        b.iter(|| {
            instances
                .iter()
                .filter(|instance| instance.is_healthy())
                .count()
        })
    });

    // Benchmark getting service URL
    c.bench_function("get_service_url", |b| {
        b.iter(|| {
            instances[0].get_url("/api/test")
        })
    });
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
        b.iter(|| std::hint::black_box(&instance).get_url("/api/users"))
    });

    c.bench_function("get_secure_url", |b| {
        let secure_instance = ServiceInstance {
            secure: true,
            ..instance.clone()
        };
        b.iter(|| std::hint::black_box(&secure_instance).get_url("/api/users"))
    });
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

fn benchmark_service_discovery_options(c: &mut Criterion) {
    c.bench_function("create_default_options", |b| {
        b.iter(|| ServiceDiscoveryOptions::default())
    });

    c.bench_function("create_complex_options", |b| {
        b.iter(|| {
            ServiceDiscoveryOptions::new()
                .with_healthy_only(true)
                .with_tags(vec!["api".to_string(), "production".to_string()])
                .with_limit(50)
        })
    });
}

criterion_group!(
    benches,
    benchmark_service_discovery,
    benchmark_service_instance_operations,
    benchmark_serialization,
    benchmark_service_discovery_options
);
criterion_main!(benches);
