# Multi-stage build for Pet Clinic Application

# ============================================================================
# BUILD STAGE
# ============================================================================
FROM rust:1.75-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY hodei-audit-sdk ./hodei-audit-sdk
COPY hodei-audit-types ./hodei-audit-types
COPY petclinic-app ./petclinic-app

# Build dependencies first (for better caching)
RUN cd hodei-audit-sdk && cargo build --release
RUN cd hodei-audit-types && cargo build --release

# Build application
RUN cd petclinic-app && cargo build --release

# ============================================================================
# RUNTIME STAGE
# ============================================================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r petclinic && useradd -r -g petclinic petclinic

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/petclinic-app/target/release/petclinic-app /usr/local/bin/petclinic-app

# Set ownership
RUN chown -R petclinic:petclinic /usr/local/bin/petclinic-app

# Switch to app user
USER petclinic

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["/usr/local/bin/petclinic-app"]
