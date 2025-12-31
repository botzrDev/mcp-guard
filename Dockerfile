# =============================================================================
# MCP-Guard Production Dockerfile
# Multi-stage build for minimal image size and security
# =============================================================================

# -----------------------------------------------------------------------------
# Stage 1: Build the application
# -----------------------------------------------------------------------------
FROM rust:1.75-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates/mcp-guard-core/Cargo.toml ./crates/mcp-guard-core/
COPY crates/mcp-guard-cli/Cargo.toml ./crates/mcp-guard-cli/
COPY crates/mcp-guard-pro/Cargo.toml ./crates/mcp-guard-pro/
COPY crates/mcp-guard-enterprise/Cargo.toml ./crates/mcp-guard-enterprise/

# Create dummy sources for all workspace members to build dependencies
RUN mkdir -p crates/mcp-guard-core/src && \
    echo "pub fn dummy() {}" > crates/mcp-guard-core/src/lib.rs && \
    mkdir -p crates/mcp-guard-cli/src && \
    echo "fn main() {}" > crates/mcp-guard-cli/src/main.rs && \
    mkdir -p crates/mcp-guard-pro/src && \
    echo "pub fn dummy() {}" > crates/mcp-guard-pro/src/lib.rs && \
    mkdir -p crates/mcp-guard-enterprise/src && \
    echo "pub fn dummy() {}" > crates/mcp-guard-enterprise/src/lib.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release --locked

# Remove dummy sources
RUN rm -rf crates/*/src

# Copy actual source code
COPY crates ./crates
COPY templates ./templates

# Build the actual application
RUN cargo build --release --locked --package mcp-guard

# -----------------------------------------------------------------------------
# Stage 2: Create minimal runtime image
# -----------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies and create non-root user
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false -u 1000 mcp-guard

# Copy the binary from builder
COPY --from=builder /app/target/release/mcp-guard /usr/local/bin/mcp-guard

# Copy default config template
COPY --from=builder /app/templates/mcp-guard.toml /etc/mcp-guard/mcp-guard.toml.template

# Create directories for config and data
RUN mkdir -p /etc/mcp-guard /var/lib/mcp-guard /var/log/mcp-guard \
    && chown -R mcp-guard:mcp-guard /etc/mcp-guard /var/lib/mcp-guard /var/log/mcp-guard

# Set proper permissions
RUN chmod 755 /usr/local/bin/mcp-guard

# Switch to non-root user
USER mcp-guard

# Set working directory
WORKDIR /var/lib/mcp-guard

# Default environment variables
ENV MCP_GUARD_HOST=0.0.0.0
ENV MCP_GUARD_PORT=3000
ENV RUST_LOG=info

# Expose the default port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/mcp-guard", "health-check", "--url", "http://localhost:3000/health"] || exit 1

# Default command
ENTRYPOINT ["/usr/local/bin/mcp-guard"]
CMD ["run", "--config", "/etc/mcp-guard/mcp-guard.toml"]
