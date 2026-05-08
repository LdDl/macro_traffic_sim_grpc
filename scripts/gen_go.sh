#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROTO_DIR="$ROOT_DIR/protos"

# Args/env
OUT_DIR="${1:-$ROOT_DIR/gen/go}"
REPO_MODULE="${PKG_MODULE:-github.com/LdDl/macro_traffic_sim_grpc}"

# Compute import path: repo module + OUT_DIR relative to repo root
REL_OUT="${OUT_DIR#$ROOT_DIR/}"
if [[ "$REL_OUT" == "$OUT_DIR" ]]; then
  # OUT_DIR is outside repo; fall back to default subpath
  REL_OUT="gen/go"
fi
PKG_IMPORT_PATH="${PKG_IMPORT_PATH:-$REPO_MODULE/$REL_OUT}"

mkdir -p "$OUT_DIR"

# Check tools
need() { command -v "$1" >/dev/null 2>&1 || { echo "$1 not found" >&2; exit 1; }; }
need protoc
need protoc-gen-go
need protoc-gen-go-grpc

# Generate (map every proto to the same package)
protoc \
  -I"$PROTO_DIR" \
  --go_out="$OUT_DIR" --go_opt=paths=source_relative,\
Mprotos/service.proto=$PKG_IMPORT_PATH,\
Mprotos/config.proto=$PKG_IMPORT_PATH,\
Mprotos/network.proto=$PKG_IMPORT_PATH,\
Mprotos/results.proto=$PKG_IMPORT_PATH,\
Mprotos/run.proto=$PKG_IMPORT_PATH,\
Mprotos/session.proto=$PKG_IMPORT_PATH,\
Mprotos/uuid.proto=$PKG_IMPORT_PATH \
  --go-grpc_out="$OUT_DIR" --go-grpc_opt=paths=source_relative \
  "$PROTO_DIR/service.proto" \
  "$PROTO_DIR/config.proto" \
  "$PROTO_DIR/network.proto" \
  "$PROTO_DIR/results.proto" \
  "$PROTO_DIR/run.proto" \
  "$PROTO_DIR/session.proto" \
  "$PROTO_DIR/uuid.proto"

echo "Go client generated:"
echo "  OUT_DIR:          $OUT_DIR"
echo "  PKG_IMPORT_PATH:  $PKG_IMPORT_PATH"
echo "  REPO_MODULE:      $REPO_MODULE"
