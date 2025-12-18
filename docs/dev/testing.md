# Testing Guide

This guide explains how to write and run tests for mcp-guard.

## Test Organization

```
mcp-guard/
├── src/
│   └── */mod.rs          # Unit tests (in-module #[cfg(test)])
├── tests/
│   └── integration_tests.rs  # Integration tests
├── benches/
│   └── performance.rs    # Criterion benchmarks
└── examples/
    └── *.rs              # Example code (tested via cargo test)
```

## Running Tests

### All Tests

```bash
# Run all tests
cargo test

# With output
cargo test -- --nocapture

# Run tests in parallel (default)
cargo test -- --test-threads=4
```

### Specific Test Sets

```bash
# Unit tests only (in src/)
cargo test --lib

# Integration tests only
cargo test --test integration_tests

# A specific test
cargo test test_rate_limit_enabled

# Tests matching a pattern
cargo test rate_limit

# Doc tests only
cargo test --doc
```

### With Features

```bash
# Test with specific features
cargo test --features "feature_name"

# Test all features
cargo test --all-features
```

## Unit Tests

Unit tests live alongside the code they test in `#[cfg(test)]` modules:

```rust
// src/rate_limit/mod.rs

pub struct RateLimitService { /* ... */ }

impl RateLimitService {
    pub fn check(&self, identity_id: &str, custom_limit: Option<u32>) -> RateLimitResult {
        // Implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_disabled() {
        let config = RateLimitConfig {
            enabled: false,
            requests_per_second: 1,
            burst_size: 1,
        };
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
        }
    }
}
```

### Async Unit Tests

Use `#[tokio::test]` for async tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_authenticate() {
        let provider = ApiKeyProvider::new(vec![/* config */]);
        let result = provider.authenticate("test-key").await;
        assert!(result.is_ok());
    }
}
```

## Integration Tests

Integration tests in `tests/` test the public API and cross-module behavior:

```rust
// tests/integration_tests.rs

use mcp_guard::{
    auth::{ApiKeyProvider, AuthProvider},
    config::Config,
    server::AppState,
};

#[tokio::test]
async fn test_full_request_flow() {
    // Set up test config
    let config = create_test_config();

    // Bootstrap the application
    let state = bootstrap_test(config).await;

    // Make a test request
    let response = make_request(&state, "/mcp", "Bearer test-key").await;

    assert_eq!(response.status(), 200);
}
```

### Test Fixtures

Create helper functions for common test setup:

```rust
// tests/common/mod.rs (or in test file)

pub fn create_test_config() -> Config {
    Config {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
        },
        upstream: UpstreamConfig {
            transport: TransportType::Http,
            url: Some("http://localhost:8080".to_string()),
            ..Default::default()
        },
        auth: AuthConfig::default(),
        rate_limit: RateLimitConfig {
            enabled: false,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn create_test_identity() -> Identity {
    Identity {
        id: "test-user".to_string(),
        name: Some("Test User".to_string()),
        allowed_tools: None,
        rate_limit: None,
        claims: HashMap::new(),
    }
}
```

### Mock Servers

Use `wiremock` for HTTP mock servers:

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, body_json};

#[tokio::test]
async fn test_http_transport() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Define expected behavior
    Mock::given(method("POST"))
        .and(body_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "ping"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": "pong"
        })))
        .mount(&mock_server)
        .await;

    // Test transport
    let transport = HttpTransport::new_unchecked(mock_server.uri());
    // ... test logic
}
```

## Mock Providers

The crate provides mock implementations for testing:

```rust
// src/mocks.rs

pub struct MockAuthProvider {
    pub should_succeed: bool,
    pub identity: Identity,
}

#[async_trait]
impl AuthProvider for MockAuthProvider {
    async fn authenticate(&self, _token: &str) -> Result<Identity, AuthError> {
        if self.should_succeed {
            Ok(self.identity.clone())
        } else {
            Err(AuthError::InvalidApiKey)
        }
    }

    fn name(&self) -> &str {
        "mock"
    }
}

pub struct MockTransport {
    pub messages_to_receive: Mutex<VecDeque<Message>>,
    pub sent_messages: Mutex<Vec<Message>>,
}
```

Usage:

```rust
#[tokio::test]
async fn test_with_mock_provider() {
    let mock = MockAuthProvider {
        should_succeed: true,
        identity: create_test_identity(),
    };

    let result = mock.authenticate("any-token").await;
    assert!(result.is_ok());
}
```

## Temporary Files

Use `tempfile` for tests that need files:

```rust
use tempfile::NamedTempFile;

#[test]
fn test_config_from_file() {
    let config_content = r#"
        [server]
        host = "127.0.0.1"
        port = 3000
    "#;

    let temp_file = NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), config_content).unwrap();

    let config = Config::from_file(&temp_file.path().to_path_buf());
    assert!(config.is_ok());
}
```

## Test Coverage

Generate coverage reports with `cargo-tarpaulin`:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --out Html

# View report
open tarpaulin-report.html
```

### Coverage Options

```bash
# Ignore test functions themselves
cargo tarpaulin --ignore-tests

# Include integration tests
cargo tarpaulin --test-threads 1

# Generate multiple formats
cargo tarpaulin --out Html --out Xml --out Json
```

## Benchmarks

Performance benchmarks use Criterion:

```rust
// benches/performance.rs

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use mcp_guard::auth::ApiKeyProvider;

fn benchmark_api_key_auth(c: &mut Criterion) {
    let provider = setup_provider();

    c.bench_function("api_key_valid", |b| {
        b.iter(|| {
            let _ = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(provider.authenticate("valid-key"));
        });
    });
}

criterion_group!(benches, benchmark_api_key_auth);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench -- api_key

# Compare with baseline
cargo bench -- --save-baseline main
cargo bench -- --baseline main
```

### Benchmark Output

```
api_key_valid           time:   [45.123 µs 45.456 µs 45.789 µs]
                        change: [-2.5% -1.2% +0.1%] (p = 0.05 > 0.05)
                        No change in performance detected.
```

## Testing Patterns

### Table-Driven Tests

```rust
#[test]
fn test_authorization() {
    let test_cases = vec![
        // (allowed_tools, tool_name, expected)
        (None, "any_tool", true),
        (Some(vec!["read".to_string()]), "read", true),
        (Some(vec!["read".to_string()]), "write", false),
        (Some(vec!["*".to_string()]), "anything", true),
        (Some(vec![]), "nothing", false),
    ];

    for (allowed_tools, tool, expected) in test_cases {
        let identity = Identity {
            allowed_tools,
            ..Default::default()
        };

        let result = authorize_tool_call(&identity, tool);
        assert_eq!(
            result, expected,
            "Failed for tools={:?}, tool={}",
            identity.allowed_tools, tool
        );
    }
}
```

### Property-Based Testing

Use `proptest` for property-based testing:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_hash_key_deterministic(key: String) {
        let hash1 = ApiKeyProvider::hash_key(&key);
        let hash2 = ApiKeyProvider::hash_key(&key);
        prop_assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_rate_limit_result_fields(
        limit in 1u32..1000,
        remaining in 0u32..1000,
        reset_at in 0u64..u64::MAX,
    ) {
        let result = RateLimitResult::allowed(limit, remaining, reset_at);
        prop_assert!(result.allowed);
        prop_assert_eq!(result.limit, limit);
    }
}
```

### Error Case Testing

```rust
#[test]
fn test_error_cases() {
    // Test specific error variants
    let err = AuthError::TokenExpired;
    assert!(matches!(err, AuthError::TokenExpired));

    // Test error messages
    let err = AuthError::InvalidJwt("bad token".to_string());
    let message = format!("{:?}", err);
    assert!(message.contains("bad token"));
}

#[tokio::test]
async fn test_auth_failure_logged() {
    let (logger, _handle) = AuditLogger::with_tasks(&AuditConfig {
        enabled: true,
        ..Default::default()
    }).unwrap();

    let provider = ApiKeyProvider::new(vec![]);
    let result = provider.authenticate("invalid-key").await;

    assert!(result.is_err());
    // Verify audit log was written
}
```

### Timeout Testing

```rust
#[tokio::test]
async fn test_timeout_handling() {
    let result = tokio::time::timeout(
        Duration::from_millis(100),
        slow_operation(),
    ).await;

    assert!(result.is_err(), "Expected timeout");
}
```

## CI Integration

Tests run in GitHub Actions on every PR:

```yaml
# .github/workflows/ci.yml

- name: Run tests
  run: cargo test --all-features

- name: Run clippy
  run: cargo clippy --all-features -- -D warnings

- name: Check formatting
  run: cargo fmt --check
```

## Common Issues

### Test Isolation

Tests may interfere when they share global state (e.g., metrics recorders):

```rust
// Bad: Uses global recorder
#[test]
fn test_metrics() {
    init_metrics();  // Fails if already initialized
}

// Good: Uses local recorder
#[test]
fn test_metrics() {
    let handle = create_metrics_handle();  // Local, no global state
}
```

### Async Runtime Conflicts

Each `#[tokio::test]` creates its own runtime:

```rust
// Each test gets isolated runtime
#[tokio::test]
async fn test_one() { /* ... */ }

#[tokio::test]
async fn test_two() { /* ... */ }
```

### File Path Issues

Use relative paths or test fixtures:

```rust
// Bad: Hardcoded path
let config = Config::from_file(&PathBuf::from("/home/user/config.toml"));

// Good: Use tempfile
let temp = NamedTempFile::new().unwrap();
let config = Config::from_file(&temp.path().to_path_buf());
```

## Test Documentation

Document test intentions:

```rust
/// Verify that disabled rate limiter always allows requests.
///
/// This ensures that when rate limiting is disabled in config,
/// no requests are ever rejected, regardless of traffic volume.
#[test]
fn test_rate_limit_disabled() {
    // ...
}
```
