#!/usr/bin/env bash
set -euo pipefail

step() { echo ">>> $*"; }

BIN="target/debug/ucp-cli"
TMPBASE=/tmp/ucp-e2e-conflict
cleanup() { rm -rf "$TMPBASE" 2>/dev/null; }
trap cleanup EXIT

step "Setup: two Button components with conflicting types"
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

step "Bootstrap dir a"
LOG_A="/tmp/e2e-conflict-a.txt"
if $BIN bootstrap --source-dir "$TMPBASE/a" --output-dir "$TMPBASE/out-a" 2>&1 | tee "$LOG_A"; then
    : # ok
else
    echo "FAIL: bootstrap a returned non-zero"
    exit 1
fi
if grep -q "Components:     1" "$LOG_A"; then
    echo "OK: 1 component"
else
    echo "FAIL: expected 1 component in a"
    cat "$LOG_A"
    exit 1
fi

step "Bootstrap dir b"
LOG_B="/tmp/e2e-conflict-b.txt"
if $BIN bootstrap --source-dir "$TMPBASE/b" --output-dir "$TMPBASE/out-b" 2>&1 | tee "$LOG_B"; then
    : # ok
else
    echo "FAIL: bootstrap b returned non-zero"
    exit 1
fi
if grep -q "Components:     1" "$LOG_B"; then
    echo "OK: 1 component"
else
    echo "FAIL: expected 1 component in b"
    cat "$LOG_B"
    exit 1
fi

step "Merge and check for conflicts"
LOG_MERGE="/tmp/e2e-merge-conflict.txt"
if $BIN merge \
    --input "$TMPBASE/out-a/ucp-spec.json" \
    --input "$TMPBASE/out-b/ucp-spec.json" \
    -o "$TMPBASE/merged.json" 2>&1 | tee "$LOG_MERGE"; then
    : # ok
else
    echo "FAIL: merge returned non-zero"
    exit 1
fi

step "Validate merged spec"
LOG_VAL="/tmp/e2e-validate-conflict.txt"
if $BIN validate "$TMPBASE/merged.json" 2>&1 | tee "$LOG_VAL"; then
    : # ok
else
    echo "FAIL: validate returned non-zero"
    exit 1
fi
if grep -q "unresolved conflict" "$LOG_VAL"; then
    echo "OK: conflict detected"
else
    echo "FAIL: expected unresolved conflict message"
    cat "$LOG_VAL"
    exit 1
fi

step "Verify conflict markers in components output"
LOG_COMP="/tmp/e2e-comp-conflict.txt"
$BIN components --verbose "$TMPBASE/merged.json" > "$LOG_COMP" 2>&1 || true
if grep -q "\[1 conflict(s)\]" "$LOG_COMP"; then
    echo "OK: conflict marker present"
else
    echo "FAIL: expected [1 conflict(s)]"
    cat "$LOG_COMP"
    exit 1
fi

echo "All conflict e2e tests passed."
cleanup
