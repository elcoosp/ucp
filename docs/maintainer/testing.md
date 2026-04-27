# Testing Strategy

UCP uses a multi‑layered testing approach.

## Test Layers

| Layer | Tool | Location | What it covers |
|-------|------|----------|----------------|
| Unit tests | `cargo test` | `src/` inline `#[cfg(test)]` | Individual functions, parsing logic |
| Integration tests | `cargo test` | `tests/` directory | Pipeline, merge, export, CLI commands |
| Snapshot tests | `insta` | `tests/` + `tests/snapshots/` | Generated output, serialized formats |
| Property tests | `proptest` | `tests/proptest_*.rs` | Merge determinism, type unification |
| Doc tests | `cargo test --doc` | `///` comments in source | Code examples in documentation |
| E2E tests | shell scripts | `.just-e2e/` | Full CLI workflows |
| Fuzz tests | `cargo fuzz` | `fuzz/fuzz_targets/` | Parser robustness |

## Running Tests

``` bash
# All tests (unit + integration + doc + snapshot + proptest)
just test

# Specific crate
cargo test -p ucp-maintainer

# Specific test
cargo test -p ucp-synthesizer -- merge

# Doc tests only
cargo test --doc

# Fuzz (requires cargo-fuzz)
cargo fuzz run smdl_parser
```

## Coverage

``` bash
just coverage
# Opens coverage HTML in browser
```

## Writing New Tests

### Snapshot Tests

``` rust
use insta::assert_snapshot;

#[test]
fn my_snapshot() {
    let result = some_function();
    assert_snapshot!("my_snapshot", result);
}
```

Accept new snapshots: `INSTA_UPDATE=always just test`

### Property Tests

``` rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn merge_is_deterministic(s1 in any::<String>(), s2 in any::<String>()) {
        let result1 = merge(&s1, &s2);
        let result2 = merge(&s1, &s2);
        prop_assert_eq!(result1, result2);
    }
}
```

## CI Integration

Tests run on every push and PR via GitHub Actions (see [CI Setup](ci-setup.md)).
