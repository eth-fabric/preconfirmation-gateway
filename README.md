# Preconfirmation Gateway

A Rust-based preconfirmation gateway that enables Ethereum validators to issue commitments for transaction inclusion. The gateway implements the Commitments API specification and integrates with the Constraints API for relay communication, providing near-instant preconfirmation responses to users while ensuring compliance with Ethereum's block construction requirements.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo
- Taskfile

### Building

```bash
# Build the entire workspace
cargo build

# Build specific packages
cargo build --package preconfirmation-gateway
cargo build --package common
cargo build --package commitments
```

#### Building the URC Bindings

To refresh the Rust ABI bindings for the URC contracts, run the following steps:

Make sure Foundry is installed, if you don't have it yet:

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

Initialize the submodules:

```bash
git submodule update --init --recursive
```

Run forge to build the bindings:

```bash
cd ./urc
forge bind --crate-name urc --overwrite --bindings-path ../crates/common/bindings/urc
```

The new bindings will be in the `/crates/common/bindings/urc` module.

### Running

```bash
# Run the main commitments server
cargo run --package preconfirmation-gateway --bin commitments
```

This starts the RPC server on `127.0.0.1` with a random port and demonstrates client connectivity.

### Development

#### Code Formatting
```bash
task format
```

#### Linting
```bash
task lint
```

#### Testing
```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --package common --package commitments --package preconfirmation-gateway

# Run integration tests
cargo test --package integration-tests

# Run specific test packages
cargo test --package common
cargo test --package commitments

# Or use the task runner
task test
```

## Architecture

### Project Structure

The project is organized as a Cargo workspace with the following structure:

```
preconfirmation-gateway/
├── bin/                          # Binary executables
│   ├── src/lib.rs               # Re-exports from crates
│   └── commitments.rs           # Main RPC server binary
├── crates/                      # Shared libraries
│   ├── common/                 # Shared types, database, config
│   ├── commitments/            # Commitments API implementation
│   ├── constraints/            # Future: Constraints API
│   └── pricer/                 # Future: Pricing logic
├── tests/                      # Integration tests
│   ├── src/
│   │   ├── lib.rs             # Test package
│   │   └── test_common.rs     # Shared test utilities
│   └── tests/                  # Test files
└── Cargo.toml                  # Workspace configuration
```

### Crate Organization

#### `crates/common/`
Contains shared functionality used across all modules:
- **Types**: Core data structures (`CommitmentRequest`, `SignedCommitment`, etc.)
- **Database**: RocksDB operations and context management
- **Config**: Configuration structures and loading
- **Constants**: Application constants and parameters
- **Signing**: Cryptographic operations

#### `crates/commitments/`
Implements the Commitments API specification:
- **Handlers**: RPC method implementations (`commitmentRequest`, `commitmentResult`, `slots`, `fee`)
- **Methods**: RPC method registration and setup
- **Utils**: Business logic and validation
- **Server**: HTTP server setup and configuration

#### `crates/constraints/` (Future)
Will implement the Constraints API for relay communication.

#### `crates/pricer/` (Future)
Will contain dynamic pricing logic and gas oracle integration.

### Core Features
- **Commitments API**: Implements 4 JSON-RPC methods (`commitmentRequest`, `commitmentResult`, `slots`, `fee`)
- **Validator Integration**: Issues commitments on behalf of Ethereum proposers
- **Constraint Management**: Creates and disseminates constraints to builders via relay integration
- **Gas Pricing**: Dynamic pricing using rETH gas oracle with configurable pricing curves
- **First-Come-First-Serve**: Ensures fair commitment dispensing with near-instant response times

### Technical Implementation
- **RocksDB Integration**: Persistent storage for commitments and slot metadata
- **BLS/ECDSA Cryptography**: Secure signature verification and commitment signing
- **Slot Timing Management**: 8-second constraint submission windows with automated scheduling
- **Relay Communication**: Integration with Constraints API for builder coordination

## Current Status

### ✅ Completed Features
- **Workspace Organization**: Clean separation of concerns with modular crate structure
- **Commitments API**: Full implementation of all 4 RPC methods
- **Database Integration**: RocksDB persistence with health checks
- **Configuration Management**: Flexible config loading with commit-boost integration
- **Testing**: Comprehensive unit tests (26 tests) and integration test framework
- **Documentation**: Complete architectural documentation

### 🚧 In Progress
- **Integration Tests**: Framework complete, requires external services for full testing
- **Constraints API**: Placeholder crate ready for future implementation
- **Pricer Module**: Placeholder crate ready for future implementation

## Specifications

This project implements the following Ethereum preconfirmation specifications:
- [Commitments API](https://github.com/eth-fabric/commitments-specs/blob/main/specs/commitments-api.md)
- [Constraints API](https://github.com/eth-fabric/constraints-specs/blob/main/specs/constraints-api.md)
- [Gateway Specification](https://github.com/eth-fabric/constraints-specs/blob/main/specs/gateway.md)

## Docker (dev)

Prereqs: Docker Desktop for macOS, and `just` (`brew install just`).

### 1) Build images (commit-boost style)

Images are built per-binary using Docker Buildx and output tags like `preconfirmation-gateway/<bin>:dev`.

```bash
# Build all images with tag :dev
just build-all dev

# Or build a single image (e.g., relay)
just build-relay dev
```

### 2) Start containers with Docker Compose

Compose uses the prebuilt images; it no longer builds from source.

```bash
# Start all services
just up

# Start a single service (e.g., relay)
just up relay

# Stop everything
just down
```

### 3) Logs

```bash
just logs relay
```

### Notes
- Binaries handle SIGINT/SIGTERM and exit cleanly.
- Configs are mounted read-only:
  - gateway: `./gateway.config.toml` via `CB_CONFIG=/cb-config.toml`
  - proposer: `./proposer.config.toml` via `CB_CONFIG=/cb-config.toml`
  - relay: `./config/relay.toml`
- Healthcheck is enabled for `relay` on `http://localhost:8080/health`.
- Apple Silicon: images build for your local platform (typically `linux/arm64`). To build x64 images, set:
  ```bash
  DOCKER_DEFAULT_PLATFORM=linux/amd64 just build-relay dev
  ```