FROM rust:latest AS builder

WORKDIR /usr/src/main_app

COPY ./services/main/Cargo.toml ./services/main/Cargo.lock ./

RUN cargo build --release || true

COPY ./nutt.conf.toml ./

COPY ./services/main .

RUN cargo build --release

FROM debian:bookworm

COPY --from=builder /usr/src/main_app/target/release/main /usr/local/bin/main

CMD ["main"]
