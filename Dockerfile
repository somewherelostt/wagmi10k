FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/wagmi-9000 /usr/local/bin/wagmi-9000

# The port that your application listens to
ENV PORT=8080
EXPOSE ${PORT}

# Run the binary
CMD ["wagmi-9000"] 