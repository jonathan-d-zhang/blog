FROM rust:1.54 as build

RUN USER=root cargo new --bin blog
WORKDIR /blog

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY src src

RUN rm target/release/deps/blog*
RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /blog/target/release/blog ./app

COPY static static
COPY styles /blog/styles
COPY templates templates
COPY articles articles
RUN mkdir articles/json
COPY Rocket.toml Rocket.toml
COPY fonts /blog/fonts

CMD ["./app"]

EXPOSE 80
