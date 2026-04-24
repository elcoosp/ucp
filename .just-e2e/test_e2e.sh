#!/usr/bin/env bash
set -euo pipefail
BIN="target/debug/ucp_cli"
TMPBASE=/tmp/ucp-e2e

cleanup() { rm -rf "$TMPBASE" 2>/dev/null; }
trap cleanup EXIT

step "=== Setup temp directories ==="
mkdir -p "$TMPBASE/rust-a/src" "$TMPBASE/rust-b/src" \
         "$TMPBASE/tsx-a/src/components" "$TMPBASE/tsx-b/src/components" \
         "$TMPBASE/no-src/comp.rs" "$TMPBASE/empty-dir"

step "=== Write multi-file Rust source (2 components) ==="
cat > "$TMPBASE/rust-a/src/button.rs" << 'SRC'
#[component]
pub fn Button(
    #[prop(default)] disabled: bool,
    label: String,
) -> () { loop {} }
SRC
cat > "$TMPBASE/rust-a/src/dialog.rs" << 'SRC'
#[component]
pub fn Dialog(
    open: bool,
    title: String,
) -> () { loop {} }
SRC

step "=== Write multi-file TSX source (React.FC + default function) ==="
cat > "$TMPBASE/tsx-a/src/components/Card.tsx" << 'TSX'
export interface CardProps {
  title: string;
  children?: React.ReactNode;
}
const Card: React.FC<CardProps> = ({ title, children }) => <div>{title}{children}</div>;
TSX
cat > "$TMPBASE/tsx-a/src/components/Badge.tsx" << 'TSX'
export interface BadgeProps {
  label: string;
}
export default function Badge({ label }: BadgeProps) {
  return <span>{label}</span>;
}
TSX

step "=== Run bootstrap on each dir ==="
 $BIN bootstrap --source-dir "$TMPBASE/rust-a" --output-dir "$TMPBASE/out-rust" 2>&1 | tee /tmp/e2e-rust.txt
grep -q "components_found: 2" /tmp/e2e-rust.txt || { echo "FAIL: expected 2 Rust components"; exit 1; }
grep -q "files_parsed: 2" /tmp/e2e-rust.txt || { echo "FAIL: expected files_parsed: 2"; exit 1; }

 $BIN bootstrap --source-dir "$TMPBASE/tsx-a" --output-dir "$TMPBASE/out-tsx" 2>&1 | tee /tmp/e2e-tsx.txt
grep -q "components_found: 2" /tmp/e2e-tsx.txt || { echo "FAIL: expected 2 TSX components"; exit 1; }
grep -q "files_parsed: 2" /tmp/e2e-tsx.txt || { echo "FAIL: expected files_parsed: 2"; exit 1; }

step "=== Validate each output ==="
 $BIN validate "$TMPBASE/out-rust/ucp-spec.json" 2>&1 | grep -q "valid with no conflicts"
 $BIN validate "$TMPBASE/out-tsx/ucp-spec.json" 2>&1 | grep -q "valid with no conflicts"

step "=== Components text output ==="
 $BIN components "$TMPBASE/out-rust/ucp-spec.json" 2>&1 | grep -q "component(s)"

step "=== Components JSON output ==="
 $BIN components --format json "$TMPBASE/out-rust/ucp-spec.json" 2>&1 | python3 -m json.tool >/dev/null \
    || { echo "FAIL: invalid JSON output"; exit 1; }

step "=== Components regex filter ==="
FILTERED=$($BIN components --format json "$TMPBASE/out-rust/ucp-spec.json" 2>&1)
echo "$FILTERED" | python3 -c \
    "import json, sys; data=json.load(sys.stdin); \
     names=[c['id'].rsplit(':')[-1] for c in data]; \
     assert all('Button' in n for n in names), \
     f'FAIL: expected only Button, got: {names}'" \
    || { echo "FAIL: regex filter"; exit 1; }

step "=== Error: nonexistent source dir ==="
 $BIN bootstrap --source-dir /nonexistent/path 2>&1 | grep -q "files_scanned: 0"

step "=== Error: nonexistent spec file ==="
 $BIN validate /nonexistent.json 2>&1 | grep -q "Failed to load"

step "=== Error: empty directory ==="
 $BIN bootstrap --source-dir "$TMPBASE/empty-dir" 2>&1 | grep -q "files_scanned: 0"

step "=== Error: no src/ dir ==="
 $BIN bootstrap --source-dir "$TMPBASE/no-src" 2>&1 | grep -q "files_scanned: 0"

step "All e2e tests passed."
cleanup
