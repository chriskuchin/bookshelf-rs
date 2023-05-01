FROM node:18-alpine AS webpack
ADD . /public
WORKDIR /public
RUN npm install && npx webpack

FROM rust:1.69-buster as builder
WORKDIR /usr/src/bookshelf-rs
COPY . .
RUN cargo build --release

FROM debian:buster-slim
ENV BOOKSHELF_FRONTEND_LOCATION=/opt/bookshelf/frontend

RUN apt-get update & apt-get install -y --no-install-recommends ca-certificates extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*


COPY --from=builder /usr/src/bookshelf-rs/target/release/bookshelf-rs /usr/local/bin/bookshelf-rs
COPY --from=webpack /public/dist /opt/bookshelf/frontend

CMD ["bookshelf-rs"]