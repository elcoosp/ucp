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
    cargo run --release -- bootstrap {{ ARGS }}

validate *ARGS:
    cargo run --release -- validate {{ ARGS }}

merge *ARGS:
    cargo run --release -- merge {{ ARGS }}

generate *ARGS:
    cargo run --release -- generate {{ ARGS }}
components *ARGS:
    cargo run --release -- components {{ ARGS }}

wr:
    watchexec -w ./wr.sh --clear -r "sh ./wr.sh"

# ---------------------------------------------------------------------------
# Install tools for test hardening
# ---------------------------------------------------------------------------
install-tools:
    cargo install cargo-insta
    cargo install cargo-llvm-cov
    cargo install cargo-fuzz
    cargo install cargo-mutants

# ---------------------------------------------------------------------------
# Coverage reporting
# ---------------------------------------------------------------------------
coverage:
    cargo llvm-cov nextest --lcov --output-path lcov.info
    cargo llvm-cov report --html --output-dir coverage
    echo "Coverage report in coverage/index.html"

# =============================================================================
# v0.11 Spec Maintainer Toolkit targets
# =============================================================================
test-maintainer:
    cargo test -p ucp-maintainer

test-e2e-maintainer:
    cargo build && .just-e2e/test_e2e_curate.sh

# Run the interactive curation TUI on two sample specs
curate-demo *ARGS:
    cargo run --release -- curate {{ ARGS }}
