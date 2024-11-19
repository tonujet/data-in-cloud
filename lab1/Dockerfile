FROM rust:1.70 as builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

WORKDIR /usr/local/cargo/bin

RUN mv lab1 myapp


FROM debian:bullseye-slim

RUN apt-get update && apt-get install && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

CMD ["myapp"]
