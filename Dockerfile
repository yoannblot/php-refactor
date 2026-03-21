FROM rust:1.94.0-slim

RUN rustup toolchain install && \
    rustup component add rustfmt clippy

WORKDIR /app
