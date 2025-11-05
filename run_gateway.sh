#!/bin/bash

# Run the gateway
export CB_CONFIG=config.toml
export CB_MODULE_ID=inclusion-preconf-module
export CB_MODULE_JWT=todo
export CB_SIGNER_URL=http://localhost:20000
export CB_SIGNER_JWT=http://localhost:20000

cargo run