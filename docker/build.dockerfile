FROM rust:bullseye AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --bin shared_moments


FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y openssl ca-certificates curl jq \
    && rm -rf /var/lib/apt/lists/*

RUN update-ca-certificates

COPY ./scripts/*.sh /
RUN chmod +x /*.sh

WORKDIR /app

COPY --from=builder /app/target/release/shared_moments /usr/local/bin
ENTRYPOINT ["/start.sh"]
