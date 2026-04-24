# Default: run all tests
test:
    cargo nextest run

# Quick compile check (no tests)
check:
    cargo check --all-targets

# Bootstrap a source directory
bootstrap *ARGS:
    cargo run --release -- bootstrap "{{ARGS}}"

# Validate a spec file
validate *ARGS:
    cargo run --release -- validate "{{ARGS}}"

# Merge spec files
merge *ARGS:
    cargo run --release -- merge "{{ARGS}}"

# List components in a spec
components *ARGS:
    cargo run --release -- components "{{ARGS}}"

# Lint
lint:
    cargo clippy --all-targets -- -D warnings

# Format
fmt:
    cargo fmt --all

# Watch and run tests on change
wr:
    watchexec -w ./wr.sh --clear -r "sh ./wr.sh test"
