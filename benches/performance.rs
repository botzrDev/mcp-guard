//! Performance benchmarks for mcp-guard
//!
//! Run with: cargo bench
//!
//! Performance targets:
//! - Latency overhead: <2ms p99
//! - Throughput: >5,000 RPS
//! - Memory: <50MB RSS (not measured here)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mcp_guard::{
    auth::{ApiKeyProvider, AuthProvider, Identity},
    authz::{authorize_tool_call, filter_tools_list_response},
    cli::{generate_api_key, hash_api_key},
    config::{ApiKeyConfig, RateLimitConfig},
    rate_limit::RateLimitService,
    transport::Message,
};
use std::collections::HashMap;

/// Create an identity with optional tools restriction
fn make_identity(id: &str, allowed_tools: Option<Vec<String>>) -> Identity {
    Identity {
        id: id.to_string(),
        name: None,
        allowed_tools,
        rate_limit: None,
        claims: HashMap::new(),
    }
}

/// Benchmark API key authentication
fn bench_api_key_auth(c: &mut Criterion) {
    let mut group = c.benchmark_group("auth/api_key");
    group.throughput(Throughput::Elements(1));

    // Create a provider with varying numbers of keys
    for key_count in [1, 10, 100, 1000] {
        let keys: Vec<ApiKeyConfig> = (0..key_count)
            .map(|i| {
                let key = generate_api_key();
                ApiKeyConfig {
                    id: format!("user_{}", i),
                    key_hash: hash_api_key(&key),
                    allowed_tools: vec!["read".to_string(), "write".to_string()],
                    rate_limit: None,
                }
            })
            .collect();

        // Generate a valid key for testing
        let valid_key = generate_api_key();
        let valid_hash = hash_api_key(&valid_key);
        let mut all_keys = keys;
        all_keys.push(ApiKeyConfig {
            id: "test_user".to_string(),
            key_hash: valid_hash,
            allowed_tools: vec!["read".to_string()],
            rate_limit: Some(100),
        });

        let provider = ApiKeyProvider::new(all_keys);

        group.bench_with_input(
            BenchmarkId::new("authenticate", key_count),
            &valid_key,
            |b, key| {
                b.iter(|| {
                    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
                    let _ = black_box(rt.block_on(provider.authenticate(black_box(key))));
                });
            },
        );

        // Also benchmark invalid key
        group.bench_with_input(
            BenchmarkId::new("authenticate_invalid", key_count),
            &"invalid_key",
            |b, key| {
                b.iter(|| {
                    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
                    let _ = black_box(rt.block_on(provider.authenticate(black_box(key))));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark rate limiting
fn bench_rate_limiting(c: &mut Criterion) {
    let mut group = c.benchmark_group("rate_limit");
    group.throughput(Throughput::Elements(1));

    let config = RateLimitConfig {
        enabled: true,
        requests_per_second: 1000,
        burst_size: 100,
        tool_limits: vec![],
    };
    let rate_limiter = RateLimitService::new(&config);

    // Benchmark single identity check
    group.bench_function("check/single", |b| {
        b.iter(|| {
            let result = rate_limiter.check(black_box("user_1"), None);
            black_box(result);
        });
    });

    // Benchmark with many different identities (tests DashMap scalability)
    for identity_count in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("check/many_identities", identity_count),
            &identity_count,
            |b, &count| {
                // Pre-populate identities
                for i in 0..count {
                    rate_limiter.check(&format!("preload_{}", i), None);
                }

                let mut idx = 0u64;
                b.iter(|| {
                    let identity = format!("user_{}", idx % (count as u64));
                    idx += 1;
                    let result = rate_limiter.check(black_box(&identity), None);
                    black_box(result);
                });
            },
        );
    }

    // Benchmark with custom rate limits
    group.bench_function("check/custom_limit", |b| {
        b.iter(|| {
            let result = rate_limiter.check(black_box("custom_user"), Some(500));
            black_box(result);
        });
    });

    group.finish();
}

/// Benchmark authorization checks
fn bench_authorization(c: &mut Criterion) {
    let mut group = c.benchmark_group("authz");
    group.throughput(Throughput::Elements(1));

    // Unrestricted identity
    let unrestricted = make_identity("admin", None);

    // Restricted identity with varying tool counts
    for tool_count in [5, 25, 100] {
        let tools: Vec<String> = (0..tool_count).map(|i| format!("tool_{}", i)).collect();
        let restricted = make_identity("user", Some(tools));

        group.bench_with_input(
            BenchmarkId::new("authorize_tool/restricted", tool_count),
            &restricted,
            |b, identity| {
                b.iter(|| {
                    let result = authorize_tool_call(black_box(identity), black_box("tool_0"));
                    black_box(result);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("authorize_tool/restricted_denied", tool_count),
            &restricted,
            |b, identity| {
                b.iter(|| {
                    let result = authorize_tool_call(black_box(identity), black_box("nonexistent_tool"));
                    black_box(result);
                });
            },
        );
    }

    group.bench_function("authorize_tool/unrestricted", |b| {
        b.iter(|| {
            let result = authorize_tool_call(black_box(&unrestricted), black_box("any_tool"));
            black_box(result);
        });
    });

    group.finish();
}

/// Benchmark tools/list filtering
fn bench_tools_filtering(c: &mut Criterion) {
    let mut group = c.benchmark_group("authz/filter_tools");
    group.throughput(Throughput::Elements(1));

    // Create tools/list responses with varying tool counts
    for tool_count in [10, 50, 200] {
        let tools: Vec<serde_json::Value> = (0..tool_count)
            .map(|i| {
                serde_json::json!({
                    "name": format!("tool_{}", i),
                    "description": format!("Tool number {}", i),
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                })
            })
            .collect();

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": tools
            })),
            error: None,
        };

        // Identity with half the tools allowed
        let allowed: Vec<String> = (0..tool_count / 2).map(|i| format!("tool_{}", i)).collect();
        let identity = make_identity("user", Some(allowed));

        group.bench_with_input(
            BenchmarkId::new("filter", tool_count),
            &(response.clone(), identity.clone()),
            |b, (resp, id)| {
                b.iter(|| {
                    let result = filter_tools_list_response(black_box(resp.clone()), black_box(id));
                    black_box(result);
                });
            },
        );

        // Unrestricted identity (should pass through quickly)
        let unrestricted = make_identity("admin", None);

        group.bench_with_input(
            BenchmarkId::new("filter_unrestricted", tool_count),
            &(response.clone(), unrestricted),
            |b, (resp, id)| {
                b.iter(|| {
                    let result = filter_tools_list_response(black_box(resp.clone()), black_box(id));
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark API key hashing (crypto operation)
fn bench_crypto(c: &mut Criterion) {
    let mut group = c.benchmark_group("crypto");
    group.throughput(Throughput::Elements(1));

    let key = generate_api_key();

    group.bench_function("hash_api_key", |b| {
        b.iter(|| {
            let hash = hash_api_key(black_box(&key));
            black_box(hash);
        });
    });

    group.bench_function("generate_api_key", |b| {
        b.iter(|| {
            let key = generate_api_key();
            black_box(key);
        });
    });

    group.finish();
}

/// Benchmark JSON-RPC message parsing
fn bench_json_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("json");
    group.throughput(Throughput::Elements(1));

    // Small request
    let small_request = r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"read_file","arguments":{"path":"/tmp/test.txt"}}}"#;

    // Large request (with many arguments)
    let large_args: serde_json::Value = serde_json::json!({
        "files": (0..100).map(|i| format!("/tmp/file_{}.txt", i)).collect::<Vec<_>>(),
        "options": {
            "recursive": true,
            "max_depth": 10,
            "follow_symlinks": false,
            "include_hidden": true
        }
    });
    let large_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "batch_read",
            "arguments": large_args
        }
    });
    let large_request_str = serde_json::to_string(&large_request).unwrap();

    group.bench_function("parse/small", |b| {
        b.iter(|| {
            let parsed: serde_json::Value = serde_json::from_str(black_box(small_request)).unwrap();
            black_box(parsed);
        });
    });

    group.bench_function("parse/large", |b| {
        b.iter(|| {
            let parsed: serde_json::Value =
                serde_json::from_str(black_box(&large_request_str)).unwrap();
            black_box(parsed);
        });
    });

    // Serialize back
    group.bench_function("serialize/small", |b| {
        let parsed: serde_json::Value = serde_json::from_str(small_request).unwrap();
        b.iter(|| {
            let json = serde_json::to_string(black_box(&parsed)).unwrap();
            black_box(json);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_api_key_auth,
    bench_rate_limiting,
    bench_authorization,
    bench_tools_filtering,
    bench_crypto,
    bench_json_parsing,
);

criterion_main!(benches);
