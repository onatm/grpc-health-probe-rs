VERSION 0.8
FROM rust:1
WORKDIR /grpc_health_probe

format:
    RUN rustup component add rustfmt
    COPY --dir src Cargo.lock Cargo.toml .
    RUN cargo fmt --all -- --check

security:
    RUN cargo install --locked cargo-deny
    COPY --dir src Cargo.lock Cargo.toml deny.toml .
    RUN cargo deny check advisories
    RUN cargo deny check bans

install-chef:
    RUN cargo install cargo-chef --locked

prepare-cache:
    FROM +install-chef
    RUN apt-get update
    RUN apt-get install lld protobuf-compiler -y
    COPY --dir src Cargo.lock Cargo.toml .
    RUN cargo chef prepare
    SAVE ARTIFACT recipe.json

build-cache:
    FROM +install-chef
    COPY +prepare-cache/recipe.json ./
    RUN apt-get update
    RUN apt-get install lld protobuf-compiler -y
    RUN cargo chef cook
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home

lint:
    FROM +build-cache
    RUN rustup component add clippy
    RUN cargo clippy -- -D warnings

build:
    COPY --dir src Cargo.lock Cargo.toml .
    COPY +build-cache/cargo_home $CARGO_HOME
    RUN apt-get update
    RUN apt-get install lld protobuf-compiler -y
    RUN cargo build --release
    SAVE ARTIFACT target/release/grpc_health_probe grpc_health_probe AS LOCAL build/grpc_health_probe

all:
    BUILD +format
    BUILD +security
    BUILD +lint
    BUILD +build
