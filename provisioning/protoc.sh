#!/usr/bin/env bash
set -euo pipefail

ARCH=$(uname -m)
PROTOC_ARCH="linux-x86_64"
if [[ "$ARCH" == "aarch64" ]] || [[ "$ARCH" == "arm64" ]]; then
  PROTOC_ARCH="linux-aarch_64"
fi
PROTOC_VERSION="25.1"

apt-get update && apt-get install -y --no-install-recommends curl unzip ca-certificates && rm -rf /var/lib/apt/lists/*

curl -sSL -o /tmp/protoc.zip "https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-${PROTOC_ARCH}.zip"
unzip -o /tmp/protoc.zip -d /usr/local
rm -f /tmp/protoc.zip

# Google APIs common protos (annotations)
apt-get update && apt-get install -y --no-install-recommends git && rm -rf /var/lib/apt/lists/*
rm -rf /tmp/googleapis || true
GIT_TERMINAL_PROMPT=0 git clone --depth 1 https://github.com/googleapis/googleapis.git /tmp/googleapis
mkdir -p /usr/local/include/google
cp -r /tmp/googleapis/google/* /usr/local/include/google/
rm -rf /tmp/googleapis
