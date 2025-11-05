# syntax=docker/dockerfile:1.7-labs

FROM --platform=${BUILDPLATFORM} rust:1.89-slim-bookworm AS chef
ARG TARGETOS TARGETARCH BUILDPLATFORM BIN_NAME
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    CARGO_NET_GIT_FETCH_WITH_CLI=true
WORKDIR /app
RUN cargo install cargo-chef --locked && rm -rf $CARGO_HOME/registry/

FROM --platform=${BUILDPLATFORM} chef AS planner
ARG TARGETOS TARGETARCH BUILDPLATFORM BIN_NAME
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=${BUILDPLATFORM} chef AS builder
ARG TARGETOS TARGETARCH BUILDPLATFORM BIN_NAME
COPY --from=planner /app/recipe.json recipe.json

# Cross compilation setup (mirrors commit-boost approach) + deps
RUN set -eux; \
    BUILD_VAR_SCRIPT=/tmp/env.sh; \
    if [ "$BUILDPLATFORM" = "linux/amd64" -a "$TARGETARCH" = "arm64" ]; then \
      rustup target add aarch64-unknown-linux-gnu; \
      dpkg --add-architecture arm64; apt-get update; \
      apt-get install -y gcc-aarch64-linux-gnu; \
      printf "#!/bin/sh\nexport TARGET=aarch64-unknown-linux-gnu\nexport TARGET_FLAG=--target=aarch64-unknown-linux-gnu\nexport CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc\nexport RUSTFLAGS=\"-L /usr/aarch64-linux-gnu/lib -L $(dirname $(aarch64-linux-gnu-gcc -print-libgcc-file-name))\"\nexport PKG_CONFIG_ALLOW_CROSS=true\nexport PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig\nexport OPENSSL_INCLUDE_DIR=/usr/include/aarch64-linux-gnu\nexport OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu\n" > ${BUILD_VAR_SCRIPT}; \
    elif [ "$BUILDPLATFORM" = "linux/arm64" -a "$TARGETARCH" = "amd64" ]; then \
      rustup target add x86_64-unknown-linux-gnu; \
      dpkg --add-architecture amd64; apt-get update; \
      apt-get install -y gcc-x86-64-linux-gnu; \
      printf "#!/bin/sh\nexport TARGET=x86_64-unknown-linux-gnu\nexport TARGET_FLAG=--target=x86_64-unknown-linux-gnu\nexport CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/x86_64-linux-gnu-gcc\nexport RUSTFLAGS=\"-L /usr/x86_64-linux-gnu/lib -L $(dirname $(x86_64-linux-gnu-gcc -print-libgcc-file-name))\"\nexport PKG_CONFIG_ALLOW_CROSS=true\nexport PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig\nexport OPENSSL_INCLUDE_DIR=/usr/include/x86_64-linux-gnu\nexport OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu\n" > ${BUILD_VAR_SCRIPT}; \
    fi; \
    if [ -f ${BUILD_VAR_SCRIPT} ]; then . ${BUILD_VAR_SCRIPT}; fi; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        git curl unzip pkg-config ca-certificates \
        build-essential cmake clang llvm-dev libclang-dev \
        libssl-dev zlib1g-dev protobuf-compiler; \
    # Install recent protoc and googleapis includes for prost/tonic builds
    ARCH=$(uname -m); \
    case "$ARCH" in \
      aarch64) PROTOC_ARCH=linux-aarch_64 ;; \
      x86_64) PROTOC_ARCH=linux-x86_64 ;; \
      *) PROTOC_ARCH=linux-x86_64 ;; \
    esac; \
    PROTOC_VERSION=25.1; \
    curl -sSL -o /tmp/protoc.zip https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-${PROTOC_ARCH}.zip; \
    unzip -o /tmp/protoc.zip -d /usr/local; \
    rm -f /tmp/protoc.zip; \
    git clone --depth 1 https://github.com/googleapis/googleapis.git /tmp/googleapis; \
    cp -r /tmp/googleapis/google /usr/local/include/; \
    rm -rf /tmp/googleapis; \
    rm -rf /var/lib/apt/lists/*; \
    if [ -f ${BUILD_VAR_SCRIPT} ]; then . ${BUILD_VAR_SCRIPT}; fi; \
    cargo chef cook ${TARGET_FLAG:-} --release --recipe-path recipe.json

COPY . .
RUN set -eux; if [ -f /tmp/env.sh ]; then . /tmp/env.sh; fi; \
    cargo build ${TARGET_FLAG:-} --release --bin ${BIN_NAME}; \
    if [ -n "${TARGET:-}" ]; then mv target/${TARGET}/release/${BIN_NAME} target/release/${BIN_NAME}; fi

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
ARG BIN_NAME
COPY --from=builder /app/target/release/${BIN_NAME} /usr/local/bin/app
ENV RUST_LOG=info
USER 10001:10001
ENTRYPOINT ["/usr/local/bin/app"]


