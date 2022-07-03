FROM rust:1.60.0-alpine
WORKDIR /usr/src/koyomi
COPY . .
RUN apk add --no-cache protobuf-dev protoc musl-dev
RUN cargo build --release
CMD ["/usr/src/koyomi/target/release/koyomi"]