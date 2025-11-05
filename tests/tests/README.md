# Integration Tests

This directory contains integration tests for the preconfirmation gateway.

## URC Registration Tests

The `urc_registration_spec.rs` file contains comprehensive integration tests for URC (Universal Registry Contract) registration functionality.

### Tests

#### 1. `test_urc_registration_end_to_end` (requires `forge`)

Full end-to-end test that:
- Starts a local Anvil instance
- Deploys the URC contract using the existing Forge deployment script
- Sets up a proposer test harness with a signer
- Signs registrations for all BLS keys
- Sends the registration transaction to the URC contract
- Verifies the transaction was successfully mined

**Usage:**
```bash
cargo test --test urc_registration_spec test_urc_registration_end_to_end -- --ignored --nocapture
```

#### 2. `test_sign_single_registration` (requires `forge`)

Tests the registration signing process:
- Sets up a proposer test harness
- Signs registrations for all available BLS keys
- Verifies the registration structure is correct
- Validates owner and signing ID match expectations

**Usage:**
```bash
cargo test --test urc_registration_spec test_sign_single_registration -- --ignored --nocapture
```

#### 3. `test_urc_deployment` (requires `forge`)

Tests URC contract deployment:
- Starts a local Anvil instance
- Deploys the URC contract using Forge script
- Verifies the deployed address is valid

**Usage:**
```bash
cargo test --test urc_registration_spec test_urc_deployment -- --ignored --nocapture
```

#### 4. `test_abi_encode_placeholder`

Tests the placeholder ABI encoding implementation:
- Creates a `URCRegisterInputs` struct
- Calls `abi_encode()` method
- Verifies it returns empty bytes (placeholder implementation)

**Usage:**
```bash
cargo test --test urc_registration_spec test_abi_encode_placeholder
```

### Requirements

Tests marked with `#[ignore]` require:
- `forge` installed and in PATH
- The URC Solidity contracts in `./urc/`
- Access to the deployment script at `urc/script/Deploy.s.sol`

### Running Tests

To run all non-ignored tests:
```bash
cargo test --test urc_registration_spec
```

To run all tests including ignored ones (requires `forge`):
```bash
cargo test --test urc_registration_spec -- --ignored --nocapture
```

To run a specific test:
```bash
cargo test --test urc_registration_spec test_name -- --nocapture
```

### Test Structure

The tests use:
- `TestHarnessBuilder` from `integration_tests::test_common` to set up test environments
- `deploy_urc_to_anvil()` from `integration_tests::urc_helpers` to deploy contracts
- `sign_registrations()` from `proposer` crate to create signed registrations
- `send_registration_transaction()` from `proposer` crate to submit transactions

### Notes

- All tests that deploy contracts are marked with `#[ignore]` to avoid requiring Forge for regular test runs
- The end-to-end test uses Anvil's pre-funded accounts for transaction signing
- Tests stream Forge output for visibility during deployment
- The address parser looks for the specific pattern: `"Registry deployed to: 0x..."`


