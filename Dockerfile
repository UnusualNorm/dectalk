FROM rust:1.85 AS builder
RUN apt-get update && apt-get install -y libopus-dev libpulse-dev

WORKDIR /usr/src/dectalk
COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libopus0 libpulse0 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/dectalk /usr/local/bin/dectalk

CMD ["dectalk"]
