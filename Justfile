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
