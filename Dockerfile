FROM debian:bookworm AS dectalk-builder
RUN apt-get update && apt-get --no-install-recommends -y install \
    build-essential libasound2-dev libpulse-dev libgtk2.0-dev unzip autoconf automake git \
    ca-certificates

RUN git clone https://github.com/dectalk/dectalk.git /dectalk
WORKDIR /dectalk/src

RUN autoreconf -i && ./configure && make


FROM rust:1.85 AS builder
RUN apt-get update && apt-get install -y libopus-dev libpulse-dev

WORKDIR /usr/src/dectalk
COPY . .

RUN cargo install --path .


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libopus0 libpulse0 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=dectalk-builder /dectalk/dist /dectalk
COPY --from=builder /usr/local/cargo/bin/dectalk /usr/local/bin/dectalk

CMD ["dectalk"]
