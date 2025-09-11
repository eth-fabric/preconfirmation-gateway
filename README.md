# Preconfirmation Gateway

A Rust-based preconfirmation gateway that implements a JSON-RPC server using the `jsonrpsee` crate.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Building

```bash
cargo build
```

Or using the task runner:

```bash
task build
```

### Running

```bash
cargo run
```

This starts the RPC server on `127.0.0.1` with a random port and demonstrates client connectivity.

### Development

#### Code Formatting
```bash
cargo fmt
```

#### Linting
```bash
cargo clippy
```

#### Testing
```bash
cargo test
```

## Architecture

The project currently includes:

- **JSON-RPC Server**: HTTP-based server with example "say_hello" method
- **Client Demo**: Demonstrates connecting to and calling RPC methods
- **Structured Logging**: Using tracing with environment filter support

## Dependencies

- **jsonrpsee**: JSON-RPC implementation
- **tokio**: Async runtime
- **tracing**: Structured logging
- **anyhow**: Error handling
- **futures**: Async utilities