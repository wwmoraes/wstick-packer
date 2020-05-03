### builder stage
FROM rust:alpine as builder

WORKDIR /usr/src/wstick-packer/

RUN apk add --no-cache build-base

# cache dependencies
COPY Cargo.toml Cargo.lock .cargo/ vendor/ ./
RUN cargo vendor

# build
COPY . .
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release
RUN cargo install --path .

### runner stage
FROM alpine:latest

RUN apk add --no-cache libgcc

COPY --from=builder /usr/local/cargo/bin/wstick-packer /usr/local/bin/wstick-packer

ENTRYPOINT [ "wstick-packer" ]

CMD [ "wstick-packer" ]
