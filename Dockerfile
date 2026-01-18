FROM rust:1.91 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/log_scanner /usr/local/bin/

EXPOSE 8080
ENTRYPOINT [ "log_scanner", "server" ]