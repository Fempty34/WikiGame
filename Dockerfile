FROM rust:1.70-slim-bullseye as builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf target/release/deps/wikigame*

COPY src ./src
COPY config.toml ./

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/wikigame /app/wikigame
COPY --from=builder /app/config.toml /app/config.toml

EXPOSE 8081

CMD ["/app/wikigame"]