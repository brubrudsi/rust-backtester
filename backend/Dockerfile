# NOTE: The API depends on crates (e.g. ICU via `url`/`reqwest`) that require
# a newer compiler than 1.78. Pin to a modern, widely-available Rust release
# so Docker builds are reproducible.
FROM rust:1.85-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config libssl-dev ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.toml
COPY crates crates
COPY config config

RUN cargo build --release -p api

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api /app/api
COPY --from=builder /app/config /app/config

ENV API_BIND=0.0.0.0:8080
EXPOSE 8080

CMD ["/app/api"]
