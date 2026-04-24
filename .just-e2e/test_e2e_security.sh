#!/usr/bin/env bash
set -euo pipefail
BIN="target/debug/ucp_cli"
DIR=$(mktemp -d)
cleanup() { rm -rf "$DIR" 2>/dev/null; }
trap cleanup EXIT

step "=== Setup: dangerous files ==="
mkdir -p "$DIR/src"
echo "---BEGIN PRIVATE KEY---" > "$DIR/src/credentials.pem"
echo "secret_key=abc" > "$DIR/src/secret.key"
echo "password=123" > "$DIR/src/secret.env"

step "=== Hidden files are rejected ==="
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1)
echo "$OUT" | grep -q "files_scanned: 0"

step "=== Dangerous extensions are rejected ==="
echo "data.txt" > "$DIR/src/data.txt"
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1)
echo "$OUT" | grep -q "files_scanned: 0"

step "=== Excluded dirs are skipped ==="
mkdir -p "$DIR/target/comp.rs"
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1)
echo "$OUT" | grep -q "files_sced: 0"

step "=== node_modules excluded ==="
mkdir -p "$DIR/node_modules/pkg/comp.rs"
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1)
echo "$OUT" | grep -q "files_scanned: 0"

step "=== Regular file in src/ IS scanned ==="
echo 'pub fn helper() {}' > "$DIR/src/helper.rs"
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1)
echo "$OUT" | grep -q "files_scanned: 1"
echo "$OUT" | grep -q "files_parsed: 0"
echo "$OUT" | grep -q "components_found: 0"

step "All security e2e tests passed."
cleanup
