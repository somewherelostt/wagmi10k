FROM rust:1.75-slim as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /usr/src/app/target/release/wagmi-9000 /usr/local/bin/wagmi-9000

EXPOSE 8080

CMD ["wagmi-9000"] 