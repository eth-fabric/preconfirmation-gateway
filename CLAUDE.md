# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based preconfirmation gateway that implements a JSON-RPC server using the `jsonrpsee` crate. The project is in early development and currently contains a simple RPC server example with a "say_hello" method.

## Development Commands

### Building
- `cargo build` - Build the project
- `task build` - Alternative build command using Taskfile

### Running
- `cargo run` - Run the main application (starts RPC server and client demo)

### Code Formatting
- `cargo fmt` - Format code using rustfmt with project-specific settings
- Uses hard tabs, 120 character line width, and Rust 2024 edition formatting rules

### Testing
- `cargo test` - Run tests (when tests are added)

### Linting
- `cargo clippy` - Run Clippy linter for code quality checks

## Architecture

### Core Components
- **main.rs**: Entry point containing both RPC server and client demonstration
  - Initializes tracing/logging with environment filter support
  - Starts an HTTP-based JSON-RPC server on `127.0.0.1:0` (random port)
  - Demonstrates client connection and method invocation
  - Currently implements a single "say_hello" RPC method

### Dependencies
- **jsonrpsee**: JSON-RPC implementation for both server and HTTP client
- **tokio**: Async runtime with multi-thread support and time features
- **tracing**: Structured logging and diagnostics
- **anyhow**: Error handling
- **futures**: Async utilities

### Project Structure
```
src/
├── main.rs          # Main application entry point
└── rpc/             # RPC-related modules (currently empty)
```

## Configuration Files

- **Cargo.toml**: Standard Rust project configuration using Rust 2024 edition
- **rustfmt.toml**: Code formatting configuration (hard tabs, 120 char width)
- **Taskfile.yml**: Task runner configuration with basic build task