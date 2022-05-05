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

COPY --from=build /blog/target/release/blog .

COPY static static
COPY templates templates
COPY articles articles
COPY Rocket.toml Rocket.toml

CMD ["./blog"]

EXPOSE 80
