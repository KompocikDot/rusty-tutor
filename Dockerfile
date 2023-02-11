FROM rust:alpine3.17

COPY ./ ./

RUN rustup toolchain install nightly
RUN cargo +nightly install -Z sparse-registry --debug cargo-ament-build 
RUN cargo run
