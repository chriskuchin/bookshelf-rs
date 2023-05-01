FROM node:18-alpine AS webpack
ADD public /public
WORKDIR /public
RUN npm install && npx webpack

FROM rust:1.68.1 as builder
WORKDIR /usr/src/bookshelf-rs
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies glibc & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/bookshelf-rs /usr/local/bin/bookshelf-rs
COPY --from=webpack 
CMD ["bookshelf-rs"]