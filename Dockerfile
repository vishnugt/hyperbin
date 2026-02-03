FROM rust:1.84-slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/hyper-status /usr/local/bin/hyper-status

EXPOSE 3002

CMD ["hyper-status"]
