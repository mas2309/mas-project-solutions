# Build stage
FROM rust:1.96-slim AS build
WORKDIR /app

# Install dependencies for building (OpenSSL, pkg-config needed by some crates)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy templates (needed at compile time by Askama)
COPY templates/ templates/

# Copy actual source code and rebuild
COPY src/ src/
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy binary from build stage
COPY --from=build /app/target/release/mas-project-advanced .

# Copy templates and migrations for runtime
COPY templates/ templates/
COPY migrations/ migrations/

EXPOSE 8080

ENTRYPOINT ["./mas-project-advanced"]
