FROM rust:1.84-slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

LABEL org.opencontainers.image.title="HyperBin"
LABEL org.opencontainers.image.description="Lightweight httpbin alternative written in Rust"
LABEL org.opencontainers.image.version="0.1.0"
LABEL org.opencontainers.image.source="https://github.com/vishnugt/hyperbin"
LABEL org.opencontainers.image.licenses="MIT"

COPY --from=builder /app/target/release/hyperbin /usr/local/bin/hyperbin

EXPOSE 80

CMD ["hyperbin"]
