FROM rust:1.60.0-alpine
WORKDIR /usr/src/hoshiyomi
COPY . .
RUN apk add --no-cache protobuf-dev protoc musl-dev
RUN cargo build --release
CMD ["/usr/src/hoshiyomi/target/release/hoshiyomi"]