FROM rust:1.95-slim-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home --home-dir /data statusbot

COPY --from=builder /build/target/release/betterstack-status-bot /usr/local/bin/betterstack-status-bot

USER statusbot
WORKDIR /data
VOLUME ["/data"]

ENV DATABASE_URL=sqlite:/data/status.db
ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/betterstack-status-bot"]
