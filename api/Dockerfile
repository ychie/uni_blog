FROM rust:latest

RUN cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 3001

VOLUME ["/usr/local/cargo"]

COPY ./ ./