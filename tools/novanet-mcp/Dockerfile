# syntax=docker/dockerfile:1.6
# =============================================================================
# NovaNet MCP Server — Static Multi-Platform Docker Image
# =============================================================================
# Build: docker buildx build --platform linux/amd64,linux/arm64 -t novanet-mcp .
# Run:   docker run --rm -e NEO4J_URI=bolt://host.docker.internal:7687 \
#                   ghcr.io/supernovae-st/novanet-mcp:latest
#
# Environment variables:
#   NEO4J_URI      - Neo4j connection URI (required)
#   NEO4J_USER     - Neo4j username (default: neo4j)
#   NEO4J_PASSWORD - Neo4j password (required)
#   RUST_LOG       - Log level (default: info)
# =============================================================================

# Stage 1: Builder
# Using Rust 1.86 for novanet-mcp (edition = "2024")
FROM --platform=$BUILDPLATFORM rust:1.86-alpine AS builder

# Build arguments for cross-compilation
ARG TARGETARCH
ARG VERSION=dev

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig perl make

# Determine target triple based on architecture
RUN case "${TARGETARCH}" in \
        amd64) echo "x86_64-unknown-linux-musl" > /tmp/target ;; \
        arm64) echo "aarch64-unknown-linux-musl" > /tmp/target ;; \
        *) echo "unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac

# Install cross-compilation toolchain for ARM64
RUN if [ "${TARGETARCH}" = "arm64" ]; then \
        apk add --no-cache gcc-aarch64-none-elf; \
    fi

# Add musl target
RUN rustup target add $(cat /tmp/target)

WORKDIR /app

# Copy source
COPY . .

# Build application with static linking
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    RUSTFLAGS="-C target-feature=+crt-static" \
    cargo build --release --target $(cat /tmp/target) --bin novanet-mcp \
    && mkdir -p /release \
    && cp target/$(cat /tmp/target)/release/novanet-mcp /release/

# Stage 2: Minimal runtime (scratch = ~8MB)
FROM scratch AS runtime

# OCI Labels (https://github.com/opencontainers/image-spec/blob/main/annotations.md)
LABEL org.opencontainers.image.source="https://github.com/supernovae-st/novanet-hq"
LABEL org.opencontainers.image.description="NovaNet MCP Server — Knowledge Graph for AI Agents"
LABEL org.opencontainers.image.licenses="LicenseRef-Proprietary"
LABEL org.opencontainers.image.vendor="SuperNovae Studio"
LABEL org.opencontainers.image.title="novanet-mcp"
LABEL org.opencontainers.image.url="https://supernovae.studio"
LABEL org.opencontainers.image.documentation="https://github.com/supernovae-st/novanet-hq#readme"

ARG VERSION=dev
LABEL org.opencontainers.image.version="${VERSION}"

# Copy static binary
COPY --from=builder /release/novanet-mcp /novanet-mcp

# MCP servers use stdio transport (stdin/stdout)
ENTRYPOINT ["/novanet-mcp"]
