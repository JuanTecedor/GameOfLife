FROM rust:1.66.0

COPY ./src /rust_app/src
COPY ./data /rust_app/data

COPY ./Cargo.toml /rust_app
COPY ./Cargo.lock /rust_app

RUN apt update
RUN apt install -y libsdl2-dev
WORKDIR /rust_app
RUN cargo check

