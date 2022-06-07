FROM rust:1.61 as base

RUN USER=root cargo new --bin blog
WORKDIR /blog

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY src src

RUN rm target/release/deps/blog*
RUN cargo build --release

FROM debian:bullseye-slim as prod

COPY --from=base /blog/target/release/blog ./app

COPY Rocket.toml Rocket.toml
COPY images /blog/images
COPY fonts /blog/fonts
COPY styles /blog/styles
COPY templates templates
COPY articles articles

CMD ["./app"]

EXPOSE 80
