# Contributing Guide

Thank you for your interest in contributing to mcp-guard! This guide will help you get started.

## Development Setup

### Prerequisites

- **Rust 1.75+**: Install via [rustup](https://rustup.rs/)
- **Git**: For version control
- **cargo-tarpaulin** (optional): For code coverage
- **cargo-criterion** (optional): For benchmarks

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/botzr/mcp-guard.git
cd mcp-guard

# Build the project
cargo build

# Run tests
cargo test

# Run lints
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check
```

### IDE Setup

**VS Code** (recommended):
```json
// .vscode/settings.json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.allTargets": true,
  "[rust]": {
    "editor.formatOnSave": true
  }
}
```

**Other IDEs**: Ensure your IDE uses `rustfmt` for formatting and `clippy` for linting.

## Project Structure

```
mcp-guard/
├── src/                # Main source code
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Library root
│   ├── auth/           # Authentication providers
│   ├── authz/          # Authorization logic
│   ├── rate_limit/     # Rate limiting
│   ├── transport/      # MCP transports
│   ├── router/         # Multi-server routing
│   ├── server/         # HTTP server
│   ├── audit/          # Audit logging
│   └── observability/  # Metrics and tracing
├── tests/              # Integration tests
├── benches/            # Performance benchmarks
├── templates/          # Configuration templates
├── docs/               # Documentation
└── examples/           # Example code
```

## Branch Naming

Use descriptive branch names:

| Type | Pattern | Example |
|------|---------|---------|
| Feature | `feat/<description>` | `feat/websocket-transport` |
| Bug fix | `fix/<description>` | `fix/jwt-expiry-check` |
| Docs | `docs/<description>` | `docs/api-reference` |
| Refactor | `refactor/<description>` | `refactor/auth-middleware` |
| Test | `test/<description>` | `test/rate-limit-edge-cases` |

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation changes |
| `style` | Formatting, no code change |
| `refactor` | Code refactoring |
| `test` | Adding/updating tests |
| `chore` | Maintenance tasks |
| `perf` | Performance improvements |

### Examples

```bash
# Feature
git commit -m "feat(auth): add LDAP authentication provider"

# Bug fix
git commit -m "fix(rate-limit): prevent overflow in token calculation"

# Documentation
git commit -m "docs(api): add HTTP endpoint reference"

# With body
git commit -m "feat(transport): add WebSocket transport

Implements WebSocket transport for bidirectional MCP communication.
Includes SSRF validation and automatic reconnection.

Closes #123"
```

## Pull Request Process

### Before Opening a PR

1. **Create a branch** from `main`
2. **Make your changes** following the code style guide
3. **Write tests** for new functionality
4. **Update documentation** if needed
5. **Run the full test suite**:
   ```bash
   cargo build && cargo test && cargo clippy -- -D warnings && cargo fmt --check
   ```

### PR Template

When opening a PR, include:

```markdown
## Summary
Brief description of what this PR does.

## Changes
- List of specific changes
- Another change

## Test Plan
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Related Issues
Closes #123
```

### Review Process

1. **CI must pass**: All tests, clippy, and format checks
2. **Code review**: At least one maintainer approval
3. **Documentation**: Update docs if behavior changes
4. **Changelog**: Add entry for user-facing changes

## Code Style

### Rust Style

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
// Use descriptive names
fn validate_jwt_token(token: &str) -> Result<Claims, AuthError>  // Good
fn vjt(t: &str) -> Result<Claims, AuthError>                      // Bad

// Document public APIs
/// Validates a JWT token and extracts claims.
///
/// # Arguments
/// * `token` - The JWT token string
///
/// # Returns
/// The decoded claims or an authentication error.
pub fn validate_jwt_token(token: &str) -> Result<Claims, AuthError>

// Handle errors explicitly
fn do_something() -> Result<T, Error> {
    let result = fallible_operation()?;  // Good: propagate with ?
    Ok(result)
}

// Use constants for magic numbers
const JWKS_CACHE_TTL_SECS: u64 = 3600;  // Good
let ttl = Duration::from_secs(3600);     // Bad: magic number
```

### Clippy Lints

All code must pass `cargo clippy -- -D warnings`. Common issues:

```rust
// Bad: unused variable
let _unused = compute_something();

// Good: explicitly ignore
let _ = compute_something();

// Bad: using clone unnecessarily
let s = string.clone();
use_ref(&s);

// Good: borrow instead
use_ref(&string);
```

### Formatting

Run `cargo fmt` before committing:

```bash
# Format all files
cargo fmt

# Check without modifying
cargo fmt --check
```

## Feature Checklist

For any new feature, ensure:

- [ ] **Code**: Implementation complete and working
- [ ] **Tests**: Unit tests and integration tests
- [ ] **Documentation**: Inline docs and guides updated
- [ ] **Config template**: `templates/mcp-guard.toml` updated if needed
- [ ] **Changelog**: Entry added for user-facing changes
- [ ] **CI**: All checks passing

## Adding a New Feature

### Example: Adding a New Auth Provider

1. **Create the provider** in `src/auth/`:
   ```rust
   // src/auth/ldap.rs
   pub struct LdapProvider { /* ... */ }

   #[async_trait]
   impl AuthProvider for LdapProvider { /* ... */ }
   ```

2. **Add configuration types** in `src/config/mod.rs`:
   ```rust
   #[derive(Debug, Clone, Deserialize)]
   pub struct LdapConfig {
       pub server_url: String,
       pub base_dn: String,
   }
   ```

3. **Wire into bootstrap** in `src/main.rs`:
   ```rust
   if let Some(ldap_config) = &config.auth.ldap {
       providers.push(Arc::new(LdapProvider::new(ldap_config)?));
   }
   ```

4. **Write tests**:
   ```rust
   #[tokio::test]
   async fn test_ldap_provider_valid_creds() { /* ... */ }

   #[tokio::test]
   async fn test_ldap_provider_invalid_creds() { /* ... */ }
   ```

5. **Update documentation**:
   - `docs/authentication.md`
   - `docs/dev/auth-provider.md`

6. **Update config template**:
   ```toml
   # templates/mcp-guard.toml
   [auth.ldap]
   server_url = "ldap://localhost:389"
   base_dn = "dc=example,dc=com"
   ```

## Release Process

### Version Bumping

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features, backwards compatible
- **PATCH**: Bug fixes, backwards compatible

### Release Steps

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release commit: `chore: release v1.2.3`
4. Tag the release: `git tag v1.2.3`
5. Push: `git push origin main --tags`
6. CI publishes to crates.io

## Getting Help

- **Documentation**: Check existing docs first
- **Issues**: Search for existing issues
- **Discussions**: Ask questions in GitHub Discussions
- **Discord**: Join our community (link in README)

## Code of Conduct

Be respectful, inclusive, and constructive. We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## License

Contributions are licensed under the project's AGPL-3.0 license. By contributing, you agree to license your work under this license.
