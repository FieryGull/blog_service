FROM rust:1.67.1-slim-buster as build

RUN cargo new app
WORKDIR /app

COPY ./post_service/Cargo.lock ./Cargo.lock
COPY ./post_service/Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN apt-get update && apt-get install libpq-dev -y
RUN cargo build --release
RUN rm ./target/release/deps/post_service*

COPY ./post_service/src ./src
COPY ./common_lib ./src/common_lib
RUN cargo build --release


FROM debian:buster-slim

RUN apt-get update && apt-get install libpq5 -y

COPY --from=build /app/target/release/post_service .

CMD ["./post_service"]