FROM node:21-alpine AS webpack
ADD . /public
WORKDIR /public
RUN npm install && npx webpack

FROM rust:1.74-bullseye as builder
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

# # Use a Rust image as the base image
# FROM rust:latest as builder

# # Set the working directory to /app
# WORKDIR /app

# # Copy the Cargo.toml and Cargo.lock files to the container
# COPY Cargo.toml Cargo.lock ./

# # Download and compile the dependencies
# RUN mkdir src && \
#     echo "fn main() {}" > src/main.rs && \
#     cargo build --release && \
#     rm -rf src

# # Copy the rest of the application source code to the container
# COPY . .

# # Build the application
# RUN cargo build --release

# # Create a new image from the Alpine Linux base image
# FROM alpine:latest

# # Set the working directory to /app
# WORKDIR /app

# # Copy the binary from the builder stage to the container
# COPY --from=builder /app/target/release/bookshelf-rs .

# # Set the application to run on port 8080
# EXPOSE 8080

# # Run the application
# CMD ["./bookshelf-rs"]
