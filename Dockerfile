FROM rust:1.75-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sps_embryo /app/sps
COPY config.toml /app/config.toml
EXPOSE 9101 9201 9103
CMD ["/app/sps"]
