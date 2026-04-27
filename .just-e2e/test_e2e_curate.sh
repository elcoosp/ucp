#!/usr/bin/env bash
set -euo pipefail

step() { echo ">>> $*"; }

BIN="target/debug/ucp-cli"
DIR=$(mktemp -d)
cleanup() { rm -rf "$DIR" 2>/dev/null; }
trap cleanup EXIT

step "Setup: two minimal specs with a conflict"
mkdir -p "$DIR/a/src" "$DIR/b/src"

cat > "$DIR/a/src/button.rs" << 'SRC'
#[component]
pub fn Button(disabled: bool) -> () { () }
SRC

cat > "$DIR/b/src/button.rs" << 'SRC'
#[component]
pub fn Button(disabled: String) -> () { () }
SRC

step "Bootstrap both specs"
"$BIN" bootstrap --source-dir "$DIR/a" --output-dir "$DIR/out-a"
"$BIN" bootstrap --source-dir "$DIR/b" --output-dir "$DIR/out-b"

step "Merge the two specs"
"$BIN" merge --input "$DIR/out-a/ucp-spec.json" --input "$DIR/out-b/ucp-spec.json" -o "$DIR/merged.json"

step "Verify merged spec has conflicts"
if "$BIN" validate "$DIR/merged.json" | grep -q "unresolved conflict"; then
    echo "OK: Conflict detected as expected"
else
    echo "FAIL: No conflict found"
    exit 1
fi

echo "E2E curation test passed."
