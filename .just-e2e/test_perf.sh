#!/usr/bin/env bash
set -euo pipefail
BIN="target/debug/ucp_cli"
DIR=$(mktemp -d)
cleanup() { rm -rf "$DIR" 2>/dev/null; }
trap cleanup EXIT

step "=== Generating 200 component files ==="
START=$(date +%s%N)
for i in $(seq 1 200); do
    printf '#[component]\npub fn Comp%s(disabled: bool) -> () { () }\n' "$i" > "$DIR/src/comp_${i}.rs"
done
END=$(date +%s%N)
echo "  Generated in $((END - START)) seconds"

step "=== Running pipeline on 200 components ==="
START=$(date +%s%N)
 $BIN bootstrap --source-dir "$DIR" --output-dir /tmp/ucp-perf 2>&1 | tee /tmp/perf-out.txt
END=$(date +%s%N)
echo "  Pipeline ran in $((END - START)) seconds"
grep -q "components_found: 200" /tmp/perf-out.txt || { echo "FAIL: expected 200 components"; exit 1; }
grep -q "files_parsed: 200" /tmp/perf-out.txt || { echo "FAIL: expected files_parsed: 200"; exit 1; }

step "Performance test passed."
cleanup
