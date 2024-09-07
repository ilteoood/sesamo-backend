FROM rust:1 AS builder
WORKDIR /builder
RUN apt-get update && apt-get install -y musl-tools openssl libssl-dev pkg-config libudev-dev
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder --chmod=755 /builder/target/x86_64-unknown-linux-musl/release/sesamo-backend .
ENTRYPOINT ["./sesamo-backend"]