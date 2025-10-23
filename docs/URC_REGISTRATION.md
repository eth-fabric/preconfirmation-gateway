# URC Registration Guide

## Overview

This guide explains how to register your BLS validator keys with the Universal Registry Contract (URC) using the proposer service.

## What is URC Registration?

The Universal Registry Contract (URC) is a smart contract that maintains a registry of BLS validator public keys and their owners. Registration is required before your validator can participate in the preconfirmation system.

## Prerequisites

1. **Proposer Service Configured**: The proposer service must be properly configured with commit-boost and connected to your BLS signer
2. **BLS Keys Loaded**: Your BLS validator keys must be loaded in the signer
3. **Ethereum Account with Funds**: You need an Ethereum account (keystore file) with:
   - Enough ETH to cover transaction gas fees
   - The required collateral amount (typically 1 ETH)

## Configuration

Add the following fields to your `proposer.config.toml`:

```toml
# URC Registration Configuration
urc_owner = "0xYourOwnerAddress"  # The Ethereum address that will own the registered keys
execution_rpc_url = "http://localhost:8545"  # Ethereum execution layer RPC endpoint
registration_collateral_wei = "1000000000000000000"  # Collateral amount in wei (1 ETH = 10^18 wei)
```

### Configuration Fields

- **`urc_owner`**: The Ethereum address that will own the registered BLS keys. This address will be able to manage the registration later.
  - Example: `"0xBB771250f7CA20123AA4c4175a49fdD8774838F3"`

- **`execution_rpc_url`**: The RPC endpoint for the Ethereum execution layer. This is where the registration transaction will be sent.
  - Local: `"http://localhost:8545"` (Anvil, Hardhat, Geth)
  - Testnet: `"https://ethereum-holesky-rpc.publicnode.com"`
  - Mainnet: `"https://eth.llamarpc.com"` or your preferred RPC provider

- **`registration_collateral_wei`**: The amount of collateral to send with the registration in wei. Check with the URC deployment for the minimum required amount.
  - 1 ETH = `"1000000000000000000"`
  - 0.1 ETH = `"100000000000000000"`

## Usage

### 1. Dry Run (Recommended First Step)

Test the registration process without sending a transaction:

```bash
./proposer register \
  --urc-address 0xYourURCContractAddress \
  --keystore /path/to/keystore.json \
  --dry-run
```

This will:
- Load your BLS keys from the signer
- Sign registration messages for each key
- Display what would be registered
- **Not send any transaction**

### 2. Actual Registration

Register your keys with the URC:

```bash
./proposer register \
  --urc-address 0xYourURCContractAddress \
  --keystore /path/to/keystore.json
```

The command will:
1. Prompt you for the keystore password
2. Load and sign registrations for all BLS keys
3. Send the registration transaction to the URC
4. Wait for confirmation
5. Display the transaction hash

**Alternative with password flag** (not recommended for production):

```bash
./proposer register \
  --urc-address 0xYourURCContractAddress \
  --keystore /path/to/keystore.json \
  --password "your-password"
```

### 3. Run Daemon (Normal Operation)

To run the proposer service normally (without registration):

```bash
./proposer run
```

or simply:

```bash
./proposer
```

## What Happens During Registration?

1. **Key Loading**: The proposer queries all BLS public keys from your configured signer
2. **Message Signing**: For each BLS key, it creates and signs a `Registration` message containing:
   - The owner address from your config
   - A nonce (from the signer)
3. **Transaction Creation**: All signed registrations are bundled into a single transaction calling `URC.register()`
4. **Transaction Signing**: Your Ethereum keystore is decrypted and used to sign the transaction
5. **Submission**: The transaction is submitted to the execution layer RPC
6. **Confirmation**: The command waits for the transaction to be mined and confirmed
7. **Registration Root**: The URC generates a unique registration root (a merkle root) that identifies your set of registered keys

## Example Output

```
Starting URC registration process
URC Registration Parameters:
  URC Contract: 0x1234...5678
  Owner: 0xBB77...38F3
  Execution RPC: http://localhost:8545
  Collateral: 1000000000000000000 wei
Signing registrations for all consensus keys...
Signing registrations for 3 consensus keys
  [1/3] Signing for pubkey: 0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4
  [2/3] Signing for pubkey: 0x8bb50a3c7e76df0e7e4489c7f86d54e9e4b6c8c1e3a8c96c4f8e7c0e5d9f7f7d3c6c5c4c3c2c1c0b
  [3/3] Signing for pubkey: 0x9cc60b4d8e87e0f8f5598a0e8g97e65f0f5c7d9d2f4b9d87d5f9e8d1f7e6f6e8e4d7d6d5d4d3d2d1d
Successfully signed 3 registrations
  [1] Pubkey: 0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4
       Nonce:  12345
  [2] Pubkey: 0x8bb50a3c7e76df0e7e4489c7f86d54e9e4b6c8c1e3a8c96c4f8e7c0e5d9f7f7d3c6c5c4c3c2c1c0b
       Nonce:  12346
  [3] Pubkey: 0x9cc60b4d8e87e0f8f5598a0e8g97e65f0f5c7d9d2f4b9d87d5f9e8d1f7e6f6e8e4d7d6d5d4d3d2d1d
       Nonce:  12347
Enter keystore password:
Loaded wallet from keystore: 0xBB771250f7CA20123AA4c4175a49fdD8774838F3
Sending URC registration transaction...
  URC contract: 0x1234...5678
  Collateral: 1000000000000000000 wei
  Number of keys: 3
Transaction sent, waiting for confirmation...
URC registration transaction confirmed: 0xabcd...ef01
✅ Registration successful!
Transaction hash: 0xabcd...ef01
Owner: 0xBB771250f7CA20123AA4c4175a49fdD8774838F3
Number of keys registered: 3
```

## Troubleshooting

### Error: "Failed to read keystore file"

- **Cause**: The keystore file path is incorrect or the file doesn't exist
- **Solution**: Check that the `--keystore` path is correct and the file exists

### Error: "Failed to decrypt keystore"

- **Cause**: Incorrect password
- **Solution**: Make sure you're entering the correct keystore password

### Error: "Failed to get public keys from signer"

- **Cause**: The signer service is not running or not configured correctly
- **Solution**: 
  - Check that commit-boost signer is running
  - Verify your BLS keys are properly loaded
  - Check the signer port configuration matches your setup

### Error: "Invalid URC owner address"

- **Cause**: The `urc_owner` in your config is not a valid Ethereum address
- **Solution**: Ensure `urc_owner` is a valid hex address starting with `0x`

### Error: "Invalid collateral amount"

- **Cause**: The `registration_collateral_wei` is not a valid number
- **Solution**: Ensure the value is a valid integer in wei (no decimals, no quotes except around the whole string)

### Error: "Transaction reverted" or "Insufficient funds"

- **Cause**: Your keystore account doesn't have enough ETH for gas + collateral
- **Solution**: Fund your keystore address with sufficient ETH

### Error: "Keys already registered"

- **Cause**: Your BLS keys are already registered in the URC
- **Solution**: You don't need to register again. If you need to change registration, follow the URC update process

## Security Notes

### Keystore Security

- **Never commit keystore files to version control**
- Store keystores in a secure location with appropriate file permissions
- Use strong passwords for keystores
- Consider using hardware wallets for production (future feature)

### Testing

- **Always test on testnet first** before registering on mainnet
- Use the `--dry-run` flag to verify everything works before sending real transactions
- Verify the URC contract address carefully - sending to the wrong address will result in loss of funds

### Collateral

- The collateral sent with registration is held by the URC contract
- You can withdraw the collateral later by unregistering (see URC documentation)
- Ensure you have additional ETH beyond the collateral to cover gas fees

## Creating a Keystore File

If you don't have a keystore file, you can create one using `cast` (part of Foundry):

```bash
# Create a new keystore
cast wallet new --password ./my-keystore
```

Or decrypt an existing private key into a keystore:

```bash
# Import a private key into a keystore
cast wallet import my-key --private-key 0xYourPrivateKey
```

**Note**: The test keystore at `./tests/data/keystores/keys/proposer-owner-key` with password `password` is for testing only. Never use it in production.

## Next Steps

After successfully registering:

1. **Verify Registration**: Check that your registration appears on-chain (use a block explorer or URC view functions)
2. **Start the Proposer**: Run the proposer daemon with `./proposer run`
3. **Monitor**: Watch the logs to ensure delegations are being signed and posted correctly

## Additional Resources

- [URC Contract Documentation](../urc/README.md)
- [Proposer Service Overview](./PROPOSER.md)
- [Commit-Boost Signer Setup](https://commit-boost.github.io/commit-boost-client/)

## Support

If you encounter issues not covered in this guide:

1. Check the proposer service logs for detailed error messages
2. Verify all configuration fields are correct
3. Ensure your signer is running and accessible
4. Test with a dry run first

For further assistance, please consult the project repository or community channels.


