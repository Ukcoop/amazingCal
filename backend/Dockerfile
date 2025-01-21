FROM rust:1.83 AS builder

WORKDIR /app

COPY backend/ .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /app/backend

EXPOSE 3080

CMD ["./backend"]
