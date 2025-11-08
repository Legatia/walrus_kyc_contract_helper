# Multi-stage build for Rust service
FROM rust:1.75 as builder

WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY core/Cargo.toml ./core/
COPY walrus-client/Cargo.toml ./walrus-client/
COPY sui-client/Cargo.toml ./sui-client/
COPY service/Cargo.toml ./service/

# Copy source code
COPY core ./core
COPY walrus-client ./walrus-client
COPY sui-client ./sui-client
COPY service ./service

# Build release binary
RUN cargo build --release --bin service

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/service /usr/local/bin/service

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /usr/local/bin/service

USER appuser

# Expose API port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Run service
CMD ["service"]
