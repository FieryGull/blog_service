FROM rust:1.67.1-slim-buster as build

RUN cargo new app
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN apt-get update && apt-get install libpq-dev -y
RUN cargo build --release
RUN rm ./target/release/deps/auth_service*

COPY ./src ./src
RUN cargo build --release


FROM debian:buster-slim

RUN apt-get update && apt-get install libpq5 -y

COPY --from=build /app/target/release/auth_service .

CMD ["./auth_service"]