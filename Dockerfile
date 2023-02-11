FROM rustlang/nightly-bullseye-slim as build

# create a new empty shell project
RUN USER=root cargo new --bin tutor
WORKDIR /tutor

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/tutor*
RUN cargo build --release

# our final base
FROM rust:1.67

# copy the build artifact from the build stage
COPY --from=build /holodeck/target/release/tutor .

# set the startup command to run your binary
CMD ["./tutor"]