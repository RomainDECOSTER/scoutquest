FROM rust:1.75 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release --example axum_service

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/examples/axum_service .

EXPOSE 4000

ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=4000
ENV DISCOVERY_URL=http://scoutquest-server:8080

CMD ["./axum_service"]
