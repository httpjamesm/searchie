# Build stage
FROM rust:1.83-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev g++

# Create a new empty shell project
WORKDIR /usr/src/searchie

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY templates ./templates
COPY .sqlx ./.sqlx

# Build for release
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/searchie/target/release/searchie /usr/local/bin/

# Copy required runtime files
COPY templates ./templates
COPY migrations ./migrations

# Set the startup command
EXPOSE 3030
CMD ["searchie"]