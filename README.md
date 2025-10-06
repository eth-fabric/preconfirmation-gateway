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

### Running

```bash
# Run the main commitments server
cargo run --package preconfirmation-gateway --bin commitments

# Or use the task runner
task run
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

### 📊 Test Coverage
- **Unit Tests**: 26 tests passing
  - Common crate: 19 tests (types, database, config)
  - Commitments crate: 7 tests (validation, business logic)
- **Integration Tests**: Framework complete, requires external signer services
- **Compilation**: All packages compile successfully

## Dependencies

### Core Runtime
- **jsonrpsee**: JSON-RPC 2.0 server implementation
- **tokio**: Async runtime with multi-threading support
- **rocksdb**: Embedded key-value database for persistence
- **tracing**: Structured logging and diagnostics
- **alloy**: Ethereum library for blockchain interactions
- **commit-boost**: Commit-boost client integration

### Development
- **eyre**: Error handling and reporting
- **serde**: Serialization framework
- **config**: Configuration management
- **tempfile**: Temporary file handling for tests
- **reqwest**: HTTP client for integration tests

## Specifications

This project implements the following Ethereum preconfirmation specifications:
- [Commitments API](https://github.com/eth-fabric/commitments-specs/blob/main/specs/commitments-api.md)
- [Constraints API](https://github.com/eth-fabric/constraints-specs/blob/main/specs/constraints-api.md)
- [Gateway Specification](https://github.com/eth-fabric/constraints-specs/blob/main/specs/gateway.md)