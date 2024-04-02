FROM ubuntu:20.04 AS base

RUN apt update && \
    apt install -y cargo

RUN rustc --version
RUN cargo --version



FROM base AS builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

WORKDIR /usr/src/myapp/target/release

RUN mv ia-11-vorobei-ant myapp



FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install && \
    apt-get install -y libc6 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/myapp/target/release/myapp /usr/local/bin/myapp

CMD ["myapp"]
