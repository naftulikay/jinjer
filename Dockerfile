# BUILD IMAGE
FROM rust:latest as build
MAINTAINER Naftuli Kay <naftuli.kay@hulu.com>

RUN DEBIAN_FRONTEND=noninteractive apt-get update >/dev/null && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y musl musl-dev musl-tools >/dev/null && \
  rm -fr /var/lib/apt/lists/* && \
  DEBIAN_FRONTEND=noninteractive

RUN rustup target add x86_64-unknown-linux-musl && \
  cargo install --force cargo-audit

COPY ./ /usr/src/jinjer/

WORKDIR /usr/src/jinjer/

RUN cargo audit
RUN cargo test
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/jinjer

# RUNTIME IMAGE
FROM scratch
COPY --from=build /usr/src/jinjer/target/x86_64-unknown-linux-musl/release/jinjer /

ENTRYPOINT ["/jinjer"]
