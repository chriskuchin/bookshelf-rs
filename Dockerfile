FROM rust:1.68.1 as builder
WORKDIR /usr/src/bookshelf-rs
COPY . .
RUN cargo install --path .
FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/bookshelf-rs /usr/local/bin/bookshelf-rs
CMD ["bookshelf-rs"]