# Stage 1: Build
FROM rust:1.85-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/airust /usr/local/bin/airust
COPY --from=builder /app/knowledge/ ./knowledge/

EXPOSE 7070
CMD ["airust"]
