FROM rust:1.96 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api /usr/local/bin/api
COPY --from=builder /app/migrations /migrations
CMD ["/usr/local/bin/api"]
