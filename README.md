# WAGMI-9000

A high-performance Rust server for the WAGMI-9000 challenge.

## Features

- Single POST /wagmi endpoint
- Handles ping requests and addition operations
- Optimized for high concurrency
- Built with Actix-web for maximum performance

## Local Development

1. Install Rust (if not already installed):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Run the server:

   ```bash
   cargo run --release
   ```

The server will start on `http://localhost:8080`

## Testing

### Ping Test

```bash
curl -X POST http://localhost:8080/wagmi -H "Content-Type: application/json" -d "{}"
```

### Addition Test

```bash
curl -X POST http://localhost:8080/wagmi -H "Content-Type: application/json" -d '{"a": 40, "b": 55}'
```

## Deployment

This project is configured for deployment on Railway.app. Simply connect your GitHub repository to Railway and it will automatically build and deploy using the provided Dockerfile.

## Performance Optimizations

- Uses Actix-web for high-performance async I/O
- Optimized worker count based on CPU cores
- Minimal memory footprint
- Efficient JSON serialization/deserialization
