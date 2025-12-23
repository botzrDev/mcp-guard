# Contributing to mcp-guard

Thank you for your interest in contributing to mcp-guard! This document provides guidelines for contributing.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/botzrdev/mcp-guard.git
cd mcp-guard

# Build and test
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Development Setup

### Prerequisites

- **Rust 1.75+**: Install via [rustup](https://rustup.rs/)
- **Git**: For version control

### IDE Setup (VS Code)

```json
{
  "rust-analyzer.check.command": "clippy",
  "[rust]": { "editor.formatOnSave": true }
}
```

## Making Changes

### Branch Naming

| Type | Pattern | Example |
|------|---------|---------|
| Feature | `feat/<description>` | `feat/websocket-transport` |
| Bug fix | `fix/<description>` | `fix/jwt-expiry-check` |
| Docs | `docs/<description>` | `docs/api-reference` |

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(auth): add LDAP authentication provider
fix(rate-limit): prevent overflow in token calculation
docs(api): add HTTP endpoint reference
```

### Pull Request Process

1. Create a branch from `main`
2. Make your changes following the code style
3. Write tests for new functionality
4. Run the full test suite:
   ```bash
   cargo build && cargo test && cargo clippy -- -D warnings && cargo fmt --check
   ```
5. Open a PR with a clear description

### Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use descriptive names (not abbreviations)
- Document public APIs with doc comments
- All code must pass `cargo clippy -- -D warnings`

## Feature Checklist

For new features, ensure:

- [ ] Implementation complete and working
- [ ] Unit tests and integration tests added
- [ ] Documentation updated
- [ ] Config template updated (if applicable)
- [ ] CHANGELOG entry added

## Getting Help

- Check existing [documentation](./docs/)
- Search [issues](https://github.com/botzrdev/mcp-guard/issues)
- Open a new issue for questions

## Code of Conduct

Please read our [Code of Conduct](./CODE_OF_CONDUCT.md) before contributing.

## License

By contributing, you agree to license your work under the [AGPL-3.0](./LICENSE) license.
