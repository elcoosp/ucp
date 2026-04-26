#!/usr/bin/env bash
set -euo pipefail

step() { echo ">>> $*"; }

BIN="target/debug/ucp-cli"
DIR=$(mktemp -d)
cleanup() { rm -rf "$DIR" 2>/dev/null; }
trap cleanup EXIT

check_output() {
    local label="$1"
    local pattern="$2"
    local output="$3"
    if echo "$output" | grep -qi "$pattern"; then
        echo "OK: $label"
    else
        echo "FAIL: $label (expected '$pattern')"
        echo "Actual output: $output"
        exit 1
    fi
}

step "Setup: dangerous files"
mkdir -p "$DIR/src"
echo "---BEGIN PRIVATE KEY---" > "$DIR/src/credentials.pem"
echo "secret_key=abc" > "$DIR/src/secret.key"
echo "password=123" > "$DIR/src/secret.env"

step "Hidden files are rejected"
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1) || true
check_output "hidden files rejected" "files scanned.*0" "$OUT"

step "Dangerous extensions are rejected"
echo "data.txt" > "$DIR/src/data.txt"
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1) || true
check_output "dangerous extensions rejected" "files scanned.*0" "$OUT"

step "Excluded dirs are skipped"
mkdir -p "$DIR/target/comp.rs"
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1) || true
check_output "target dir skipped" "files scanned.*0" "$OUT"

step "node_modules excluded"
mkdir -p "$DIR/node_modules/pkg/comp.rs"
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1) || true
check_output "node_modules excluded" "files scanned.*0" "$OUT"

step "Regular file in src/ IS scanned"
echo 'pub fn helper() {}' > "$DIR/src/helper.rs"
OUT=$($BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-sec 2>&1) || true
check_output "regular file scanned" "files scanned.*1" "$OUT"
check_output "no component found" "components.*0" "$OUT"

echo "All security e2e tests passed."
cleanup
