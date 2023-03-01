FROM rust:1.67.1-slim-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin auth
WORKDIR /auth

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/auth*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /auth/target/release/auth .

# set the startup command to run your binary
CMD ["./auth"]