FROM node:21-alpine AS webpack
ADD . /public
WORKDIR /public
RUN npm install && npx webpack

FROM rust:1.78-bullseye as builder
WORKDIR /usr/src/bookshelf-rs

RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
ENV BOOKSHELF_FRONTEND_LOCATION=/opt/bookshelf/frontend

RUN apt-get update && apt-get install -y ca-certificates libsqlite3-dev
COPY --from=builder /usr/src/bookshelf-rs/target/release/bookshelf-rs /usr/local/bin/bookshelf-rs
COPY --from=webpack /public/dist /opt/bookshelf/frontend

CMD ["bookshelf-rs"]