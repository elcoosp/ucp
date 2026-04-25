# Default: run all unit + integration tests
test:
    cargo nextest run

# Quality gates
check:
    cargo check --all-targets

lint:
    cargo clippy --all-targets

fmt-check:
    cargo fmt --all -- --check

fmt:
    cargo fmt --all

# =============================================================================
# End-to-end CLI tests (requires cargo build)
# =============================================================================

# Multi-file extraction, CLI commands, error handling, regex filter
test-e2e:
    cargo build && .just-e2e/test_e2e.sh

# Conflict detection across managed specs
test-e2e-conflict:
    cargo build && .just-e2e/test_e2e_conflict.sh

# Hidden files, dangerous extensions, excluded directories
test-e2e-security:
    cargo build && .just-e2e/test_e2e_security.sh

# 200-component performance baseline
test-perf:
    cargo build && .just-e2e/test_perf.sh

# =============================================================================
# Domain Commands
# =============================================================================

bootstrap *ARGS:
    cargo run --release -- bootstrap "{{ ARGS }}"

validate *ARGS:
    cargo run --release -- validate "{{ ARGS }}"

merge *ARGS:
    cargo run --release -- merge "{{ ARGS }}"

components *ARGS:
    cargo run --release -- components "{{ ARGS }}"
wr:
    watchexec -w ./wr.sh --clear -r "sh ./wr.sh"
