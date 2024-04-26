FROM rust:1-bookworm

RUN apt update && apt -y install libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres
