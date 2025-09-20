FROM rust:1.89 AS builder

WORKDIR /app

# Copy Cargo files first for better layer caching
COPY Cargo.toml ./
COPY src ./src
COPY config ./config

# Build the server
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/scoutquest-server .
COPY --from=builder /app/config/default.toml ./config.toml


ENV RUST_LOG=info
ENV SCOUTQUEST_SERVER_HOST=0.0.0.0
ENV SCOUTQUEST_SERVER_PORT=8080

EXPOSE $SCOUTQUEST_SERVER_PORT

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:$SCOUTQUEST_SERVER_PORT/health || exit 1

CMD ["./scoutquest-server"]
