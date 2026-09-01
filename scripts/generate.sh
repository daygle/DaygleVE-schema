#!/usr/bin/env bash
#
# Generate the TypeScript bindings for DaygleVE-schema from the annotated
# Rust source. The output in generated/typescript is committed so that the
# frontend can consume it without a Rust toolchain.
#
# Requires: typeshare-cli  (cargo install typeshare-cli)
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$ROOT/generated/typescript"

mkdir -p "$OUT"

echo "Generating TypeScript bindings -> $OUT/index.ts"
typeshare "$ROOT" \
  --lang=typescript \
  --output-file="$OUT/index.ts"

echo "Done. Remember to commit the regenerated bindings."
