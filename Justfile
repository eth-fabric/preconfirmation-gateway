# Simple Docker Compose workflow

up SERVICE="":
	@if [ -z "{{SERVICE}}" ]; then \
	  docker compose up -d; \
	else \
	  docker compose up -d {{SERVICE}}; \
	fi

build SERVICE="":
	@if [ -z "{{SERVICE}}" ]; then \
	  docker compose build; \
	else \
	  docker compose build {{SERVICE}}; \
	fi

down:
	docker compose down

logs SERVICE:
	docker compose logs -f {{SERVICE}}

# ===============================
# Commit-boost style builders
# ===============================

_create-docker-builder:
	docker buildx create --name multiarch-builder --driver docker-container --use > /dev/null 2>&1 || true

# Detect the first supported platform shorthand like linux_amd64 or linux_arm64
_platform:
	@docker buildx inspect --bootstrap | sed -n 's/^ *Platforms: *//p' | cut -d',' -f1 | tr -d ' ' | tr '/' '_'

# Build binary artifact for a workspace bin into ./build/<version>/<platform>

_docker-build-binary version bin: _create-docker-builder
	PLATFORM=`just _platform`; \
	docker buildx build --rm --platform=local \
	  -f provisioning/build.Dockerfile \
	  --output "build/{{version}}/$PLATFORM" \
	  --target output \
	  --build-arg TARGET_CRATE=preconfirmation-gateway \
	  --build-arg BINARY_NAME={{bin}} .

# Build runtime image for a bin using prebuilt artifacts
_docker-build-image version bin: _create-docker-builder
	PLATFORM=`just _platform`; \
	docker buildx build --rm --load \
	  -f provisioning/{{bin}}.Dockerfile \
	  --build-arg BINARIES_PATH=build/{{version}} \
	  --build-arg PLATFORM=$PLATFORM \
	  -t preconfirmation-gateway/{{bin}}:{{version}} .

# Public tasks per service (gateway, relay, proposer, spammer)

build-gateway version: (_docker-build-binary version "gateway") (_docker-build-image version "gateway")
build-relay version:   (_docker-build-binary version "relay")   (_docker-build-image version "relay")
build-proposer version:(_docker-build-binary version "proposer")(_docker-build-image version "proposer")
build-spammer version: (_docker-build-binary version "spammer") (_docker-build-image version "spammer")

build-all version:
	just build-gateway {{version}} && \
	just build-relay {{version}} && \
	just build-proposer {{version}} && \
	just build-spammer {{version}}

# ===============================
# Local binary execution (without Docker)
# ===============================

# Generate .simulation.env with JWTs and configuration
setup-simulation:
	cargo run --bin simulation-setup

# Run local signer module
run-local-signer:
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export RUST_LOG=info
	cargo run --bin local-signer-module

# Run local gateway module
run-local-gateway:
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/gateway.config.toml
	export CB_MODULE_ID=gateway-module
	export CB_SIGNER_JWT=$GATEWAY_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	cargo run --bin gateway

# Run local proposer daemon
run-local-proposer:
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_SIGNER_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=debug
	cargo run --bin proposer

# Run local proposer daemon with debug logging
run-local-proposer-debug:
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=debug
	echo "=== Environment Check ==="
	echo "CB_CONFIG: $CB_CONFIG"
	echo "CB_MODULE_ID: $CB_MODULE_ID"
	echo "CB_SIGNER_URL: $CB_SIGNER_URL"
	echo "CB_MODULE_JWT: ${CB_MODULE_JWT:0:10}..."
	echo "========================="
	cargo run --bin proposer -- run

# Register proposer BLS keys with URC contract
proposer-register urc_address keystore password="": 
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	if [ -z "{{password}}" ]; then \
		cargo run --bin proposer -- register --urc-address {{urc_address}} --keystore {{keystore}}; \
	else \
		cargo run --bin proposer -- register --urc-address {{urc_address}} --keystore {{keystore}} --password {{password}}; \
	fi

# Dry run registration (sign but don't send transaction)
proposer-register-dry-run urc_address keystore password="":
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	if [ -z "{{password}}" ]; then \
		cargo run --bin proposer -- register --urc-address {{urc_address}} --keystore {{keystore}} --dry-run; \
	else \
		cargo run --bin proposer -- register --urc-address {{urc_address}} --keystore {{keystore}} --password {{password}} --dry-run; \
	fi

# Opt-in to a slasher with a specific committer
proposer-opt-in urc_address registration_root slasher committer keystore password="":
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	if [ -z "{{password}}" ]; then \
		cargo run --bin proposer -- opt-in-to-slasher --urc-address {{urc_address}} --registration-root {{registration_root}} --slasher {{slasher}} --committer {{committer}} --keystore {{keystore}}; \
	else \
		cargo run --bin proposer -- opt-in-to-slasher --urc-address {{urc_address}} --registration-root {{registration_root}} --slasher {{slasher}} --committer {{committer}} --keystore {{keystore}} --password {{password}}; \
	fi

# Opt-out of a slasher
proposer-opt-out urc_address registration_root slasher keystore password="":
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	if [ -z "{{password}}" ]; then \
		cargo run --bin proposer -- opt-out-of-slasher --urc-address {{urc_address}} --registration-root {{registration_root}} --slasher {{slasher}} --keystore {{keystore}}; \
	else \
		cargo run --bin proposer -- opt-out-of-slasher --urc-address {{urc_address}} --registration-root {{registration_root}} --slasher {{slasher}} --keystore {{keystore}} --password {{password}}; \
	fi

# Get slasher commitment info
proposer-get-commitment urc_address registration_root slasher:
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	cargo run --bin proposer -- get-slasher-commitment --urc-address {{urc_address}} --registration-root {{registration_root}} --slasher {{slasher}}

# Add collateral to existing registration
proposer-add-collateral urc_address registration_root amount_wei keystore password="":
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	if [ -z "{{password}}" ]; then \
		cargo run --bin proposer -- add-collateral --urc-address {{urc_address}} --registration-root {{registration_root}} --amount-wei {{amount_wei}} --keystore {{keystore}}; \
	else \
		cargo run --bin proposer -- add-collateral --urc-address {{urc_address}} --registration-root {{registration_root}} --amount-wei {{amount_wei}} --keystore {{keystore}} --password {{password}}; \
	fi

# Unregister from URC
proposer-unregister urc_address registration_root keystore password="":
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	if [ -z "{{password}}" ]; then \
		cargo run --bin proposer -- unregister --urc-address {{urc_address}} --registration-root {{registration_root}} --keystore {{keystore}}; \
	else \
		cargo run --bin proposer -- unregister --urc-address {{urc_address}} --registration-root {{registration_root}} --keystore {{keystore}} --password {{password}}; \
	fi

# Claim collateral after unregistration delay
proposer-claim-collateral urc_address registration_root keystore password="":
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	if [ -z "{{password}}" ]; then \
		cargo run --bin proposer -- claim-collateral --urc-address {{urc_address}} --registration-root {{registration_root}} --keystore {{keystore}}; \
	else \
		cargo run --bin proposer -- claim-collateral --urc-address {{urc_address}} --registration-root {{registration_root}} --keystore {{keystore}} --password {{password}}; \
	fi

# Claim slashed collateral after slash window
proposer-claim-slashed urc_address registration_root keystore password="":
	#!/usr/bin/env bash
	set -a
	source .simulation.env
	set +a
	export CB_CONFIG=config/proposer.config.toml
	export CB_MODULE_ID=proposer-module
	export CB_MODULE_JWT=$PROPOSER_JWT
	export CB_SIGNER_URL=$CB_SIGNER_URL
	export RUST_LOG=info
	if [ -z "{{password}}" ]; then \
		cargo run --bin proposer -- claim-slashed-collateral --urc-address {{urc_address}} --registration-root {{registration_root}} --keystore {{keystore}}; \
	else \
		cargo run --bin proposer -- claim-slashed-collateral --urc-address {{urc_address}} --registration-root {{registration_root}} --keystore {{keystore}} --password {{password}}; \
	fi

# Run local relay
run-local-relay:
	export RUST_LOG=info
	cargo run --bin relay config/relay.config.toml

# Run local spammer
run-local-spammer:
	export RUST_LOG=info
	cargo run --bin spammer config/spammer.config.toml

# Run local mock beacon node (reads config from simulation.config.toml)
run-local-beacon-mock:
	#!/usr/bin/env bash
	export RUST_LOG=info
	# Extract beacon_api_url and proposer_bls_key from simulation.config.toml
	BEACON_URL=$(grep '^beacon_api_url' config/simulation.config.toml | cut -d'"' -f2)
	PROPOSER_KEY=$(grep '^proposer_bls_key' config/simulation.config.toml | cut -d'"' -f2)
	echo "Starting beacon mock with URL: $BEACON_URL and Proposer Key: $PROPOSER_KEY"
	cargo run --bin beacon-mock -- --url "$BEACON_URL" "$PROPOSER_KEY"

# Generate ECDSA proxy key for gateway
generate-proxy-key-gateway-ecdsa:
	#!/usr/bin/env bash
	set -a
	source .simulation.env 2>/dev/null || true
	set +a
	echo "Generating ECDSA proxy key for gateway..."
	RESPONSE=$(curl -s -w "\n%{http_code}" -X POST $CB_SIGNER_URL/signer/v1/generate_proxy_key \
		-H "Content-Type: application/json" \
		-H "Authorization: Bearer $GATEWAY_PROXY_ECDSA_JWT" \
		-d "{\"pubkey\":\"$GATEWAY_DEFAULT_BLS_KEY\",\"scheme\":\"ecdsa\"}")
	HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
	BODY=$(echo "$RESPONSE" | sed '$d')
	if [ "$HTTP_CODE" -eq 200 ]; then \
		echo "$BODY" | jq; \
	else \
		echo "Error: HTTP $HTTP_CODE"; \
		echo "$BODY"; \
		exit 1; \
	fi

# Generate BLS proxy key for gateway
generate-proxy-key-gateway-bls:
	#!/usr/bin/env bash
	set -a
	source .simulation.env 2>/dev/null || true
	set +a
	echo "Generating BLS proxy key for gateway..."
	RESPONSE=$(curl -s -w "\n%{http_code}" -X POST $CB_SIGNER_URL/signer/v1/generate_proxy_key \
		-H "Content-Type: application/json" \
		-H "Authorization: Bearer $GATEWAY_PROXY_BLS_JWT" \
		-d "{\"pubkey\":\"$GATEWAY_DEFAULT_BLS_KEY\",\"scheme\":\"bls\"}")
	HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
	BODY=$(echo "$RESPONSE" | sed '$d')
	if [ "$HTTP_CODE" -eq 200 ]; then \
		echo "$BODY" | jq; \
	else \
		echo "Error: HTTP $HTTP_CODE"; \
		echo "$BODY"; \
		exit 1; \
	fi

# Generate ECDSA proxy key for proposer
generate-proxy-key-proposer-ecdsa:
	#!/usr/bin/env bash
	set -a
	source .simulation.env 2>/dev/null || true
	set +a
	echo "Generating ECDSA proxy key for proposer..."
	RESPONSE=$(curl -s -w "\n%{http_code}" -X POST $CB_SIGNER_URL/signer/v1/generate_proxy_key \
		-H "Content-Type: application/json" \
		-H "Authorization: Bearer $PROPOSER_PROXY_ECDSA_JWT" \
		-d "{\"pubkey\":\"$PROPOSER_DEFAULT_BLS_KEY\",\"scheme\":\"ecdsa\"}")
	HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
	BODY=$(echo "$RESPONSE" | sed '$d')
	if [ "$HTTP_CODE" -eq 200 ]; then \
		echo "$BODY" | jq; \
	else \
		echo "Error: HTTP $HTTP_CODE"; \
		echo "$BODY"; \
		exit 1; \
	fi

# Generate BLS proxy key for proposer
generate-proxy-key-proposer-bls:
	#!/usr/bin/env bash
	set -a
	source .simulation.env 2>/dev/null || true
	set +a
	echo "Generating BLS proxy key for proposer..."
	RESPONSE=$(curl -s -w "\n%{http_code}" -X POST $CB_SIGNER_URL/signer/v1/generate_proxy_key \
		-H "Content-Type: application/json" \
		-H "Authorization: Bearer $PROPOSER_PROXY_BLS_JWT" \
		-d "{\"pubkey\":\"$PROPOSER_DEFAULT_BLS_KEY\",\"scheme\":\"bls\"}")
	HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
	BODY=$(echo "$RESPONSE" | sed '$d')
	if [ "$HTTP_CODE" -eq 200 ]; then \
		echo "$BODY" | jq; \
	else \
		echo "Error: HTTP $HTTP_CODE"; \
		echo "$BODY"; \
		exit 1; \
	fi
