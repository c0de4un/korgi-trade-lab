# Multi-stage image for lob-server or cabinet-server.
# Usage:
#   docker build --build-arg BIN=lob-server -t korgi-lob .
#   docker build --build-arg BIN=cabinet-server -t korgi-cabinet .

ARG BIN=lob-server

FROM rust:1.85-bookworm AS builder
ARG BIN
WORKDIR /app

COPY Cargo.toml Cargo.lock rust-toolchain.toml rustfmt.toml ./
COPY crates ./crates
COPY lob ./lob
COPY cabinet ./cabinet

RUN cargo build --release -p ${BIN}

FROM debian:bookworm-slim AS runtime
ARG BIN
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/${BIN} /usr/local/bin/app

ENV CHUNK_ROOT=/data/chunks
VOLUME ["/data/chunks"]

EXPOSE 8080 8081
ENTRYPOINT ["/usr/local/bin/app"]
