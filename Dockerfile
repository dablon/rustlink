# Multi-stage build for RustLink
FROM rust:1.75 AS builder

WORKDIR /build

# Copy source
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build release
RUN cargo build --release

# Runtime image
FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create user
RUN useradd -m -s /bin/bash appuser

# Copy binary
COPY --from=builder /build/target/release/rustlink /usr/local/bin/rustlink

# Set working directory
WORKDIR /home/appuser

# Run as user
USER appuser

ENTRYPOINT ["/usr/local/bin/rustlink"]
