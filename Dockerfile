FROM rust:1.54 as base

RUN USER=root cargo new --bin blog
WORKDIR /blog

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

FROM base as dev-build

RUN rustup component add clippy

RUN cargo build
RUN rm src/*.rs

COPY src src

RUN rm target/debug/deps/blog*

RUN cargo build
RUN cargo clippy -p blog -- --no-deps -D clippy::all

FROM debian:buster-slim as dev

COPY --from=dev-build /blog/target/debug/blog ./app

COPY images /blog/images
COPY styles /blog/styles
COPY fonts /blog/fonts
COPY Rocket.toml Rocket.toml

CMD ["./app"]

EXPOSE 8000

FROM base as prod-build

RUN cargo build --release
RUN rm src/*.rs

COPY src src

RUN rm target/release/deps/blog*
RUN cargo build --release

FROM debian:buster-slim as prod

COPY --from=prod-build /blog/target/release/blog ./app

COPY images /blog/images
COPY styles /blog/styles
COPY fonts /blog/fonts
COPY templates templates
COPY articles articles
COPY Rocket.toml Rocket.toml

CMD ["./app"]

EXPOSE 80
