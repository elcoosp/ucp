#!/usr/bin/env bash
set -euo pipefail
BIN="target/debug/ucp_cli"
TMPBASE=/tmp/ucp-e2e-conflict
cleanup() { rm -rf "$TMPBASE" 2>/dev/null; }
trap cleanup EXIT

step "=== Setup: two Button components with conflicting types ==="
mkdir -p "$TMPBASE/a/src" "$TMPBASE/b/src"

cat > "$TMPBASE/a/src/button.rs" << 'SRC'
#[component]
pub fn Button(
    disabled: bool,
) -> () { loop {} }
SRC

cat > "$TMPBASE/b/src/button.rs" << 'SRC'
#[component]
pub fn Button(
    disabled: String,
) -> () { loop {} }
SRC

step "=== Bootstrap both dirs ==="
 $BIN bootstrap --source-dir "$TMPBASE/a" --output-dir "$TMPBASE/out-a" 2>&1 | grep -q "components_found: 1"
 $BIN bootstrap --source-dir "$TMPBASE/b" --output-dir "$TMPBASE/out-b" 2>&1 | grep -q "components_found: 1"

step "=== Merge and check for conflicts ==="
 $BIN merge \
    --input "$TMPBASE/out-a/ucp-spec.json" \
    --input "$TMPBASE/out-b/ucp-spec.json" \
    -o "$TMPBASE/merged.json" 2>&1 | tee /tmp/e2e-merge.txt

 $BIN validate "$TMPBASE/merged.json" 2>&1 | grep -q "unresolved conflict"

step "=== Verify conflict markers in components output ==="
 $BIN components "$TMPBASE/merged.json" 2>&1 | grep -q "\[1 conflict(s)\]"

step "All conflict e2e tests passed."
cleanup
