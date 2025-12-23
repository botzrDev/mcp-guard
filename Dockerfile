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

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY src ./src
COPY templates ./templates

# Touch main.rs to ensure rebuild
RUN touch src/main.rs

# Build the actual application
RUN cargo build --release --locked

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
