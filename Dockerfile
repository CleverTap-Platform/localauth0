FROM rust:1.96.0-slim-bookworm

WORKDIR /code

ENV CARGO_HOME=/home/app/.cargo

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates curl git \
    && rm -rf /var/lib/apt/lists/*

COPY entrypoint /code/entrypoint

RUN useradd -m app && chown -R app:app /code /home/app
USER app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --version ^0.17 trunk

ENTRYPOINT ["./entrypoint"]
