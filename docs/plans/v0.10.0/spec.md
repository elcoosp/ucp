# UCP v0.10.0 — Full Specification Suite

## Product Vision & Strategic Alignment – UCP v0.10.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.10.0‑vision‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Vision Statement

> **UCP's test suite becomes a reference architecture for testing Rust CLI tools.**
> v0.10.0 is the "test hardening" release — no new features, no new extractors, no new generators. The entire test suite is restructured for speed, reliability, and coverage. `insta` snapshot testing captures every output format. `proptest` validates type unification and generator output properties. `cargo-llvm-cov` enforces ≥85% line coverage. `cargo-mutants` validates test quality. And `cargo-fuzz` stress‑tests every parser for crash safety.

### 2. Elevator Pitch

> **For** contributors and downstream consumers who rely on UCP's stability,
> **our product** is a CLI toolchain that extracts, unifies, and exports UI components
> **that now ships** with a battle‑tested test suite featuring snapshot testing, property‑based testing, mutation testing, fuzz testing, and CI‑enforced coverage thresholds.
> **Unlike** the prototype‑quality tests of v0.9.0,
> **our solution** catches regressions before they ship and gives contributors confidence that their changes don't break anything.

### 3. Problem Statement

v0.9.0 shipped seven releases of continuous feature additions. The test suite grew organically but accumulated technical debt: test helpers are duplicated across files, there are no property‑based or fuzz tests, snapshot coverage is sparse (one gpui JSON file), coverage is not measured, and mutation testing is absent. This makes it difficult to detect regressions, slows down code review, and creates friction for new contributors.

### 4. Business Objectives

| ID | Objective | Key Results |
|----|-----------|-------------|
| G‑01 | Achieve comprehensive snapshot coverage | `insta` snapshots for all export formats, generator outputs, and CAM serialization |
| G‑02 | Enforce measurable quality gates | ≥85% line coverage via `cargo-llvm-cov`; 100% test pass rate maintained |
| G‑03 | Harden parsing robustness | `cargo-fuzz` targets for DESIGN.md import, SMDL, TSX, Svelte, Vue parsers |
| G‑04 | Validate test quality | `cargo-mutants` baseline; no untested code paths in critical modules |

### 5. Goals and Non‑Goals

**Goals (v0.10.0):**
- Integrate `insta` for snapshot testing across all crate outputs.
- Integrate `proptest` for property‑based testing of `unify.rs`, generators, and conflict detection.
- Integrate `cargo-llvm-cov` for CI‑enforced coverage thresholds.
- Create `cargo-fuzz` targets for all input parsers.
- Consolidate test helpers into a single `tests/common/mod.rs`.
- Move e2e shell tests to `assert_cmd`‑based Rust integration tests.

**Non‑Goals:**
- No new features, extractors, or exporters.
- No behavior changes to the library or CLI.
- No changes to the public API.

---

## Software Requirements Specification – UCP v0.10.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | SRS v0.10.0 |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Introduction

This SRS defines the test infrastructure hardening for v0.10.0: `insta` snapshot testing, `proptest` property‑based testing, `cargo-fuzz` fuzzing, `cargo-llvm-cov` coverage thresholds, and test organization improvements. No external behavior changes.

### 2. Functional Requirements

#### 2.1 Snapshot Testing with `insta`

**REQ‑TST‑001 – Install `insta` and `cargo-insta`**
> **The workspace** shall include `insta` as a dev‑dependency with `json` and `yaml` features enabled. `cargo-insta` shall be available for snapshot review.

*Acceptance:* `cargo add --dev insta --features json,yaml` succeeds; `cargo insta review` runs.

**REQ‑TST‑002 – CAM Serialization Snapshots**
> **Every CAM type** (`CanonicalAbstractComponent`, `CanonicalAbstractProp`, `StateMachine`, `PackageManifest`, etc.) shall have snapshot tests covering JSON serialization and deserialization round‑trips.

*Acceptance:* `assert_json_snapshot!` is used in `cam_serialization.rs` for all structs.

**REQ‑TST‑003 – Generator Output Snapshots**
> **Every code generator** (Dioxus, Leptos, React, GPUI, Web Components) shall have snapshot tests verifying the generated source code for a standard component.

*Acceptance:* Each generator has a snapshot test that captures the full generated .rs/.tsx/.js file contents.

**REQ‑TST‑004 – Export Format Snapshots**
> **Every export format** (A2UI catalog, AG‑UI schema, DESIGN.md, W3C spec, DTCG tokens, AI contract) shall have snapshot tests.

*Acceptance:* Each exporter has a snapshot test verifying the output JSON/Markdown.

**REQ‑TST‑005 – Snapshot Review Workflow**
> **The CI pipeline** shall reject unexpected snapshot changes (`INSTA_UPDATE=never`). Local development shall support `cargo insta review` for interactive review.

#### 2.2 Property‑Based Testing with `proptest`

**REQ‑TST‑006 – Type Unification Properties**
> **The `unify.rs` module** shall have property‑based tests verifying: (a) `map_raw_type_to_cam` is idempotent for all input strings, (b) concrete types round‑trip through `map_raw_type_with_concrete`.

*Acceptance:* `proptest` strategies generate valid Rust type strings; properties hold for 10,000 cases.

**REQ‑TST‑007 – Generator Output Properties**
> **The code generator trait** shall have a property that for any valid `PackageManifest`, the generator output is non‑empty and contains the component name.

**REQ‑TST‑008 – Conflict Detection Properties**
> **The `detect_conflicts` function** shall be tested with the property: merging two specs with identical components produces zero conflicts; merging specs with type‑mismatched props produces ≥1 conflict.

#### 2.3 Fuzz Testing with `cargo-fuzz`

**REQ‑TST‑009 – Parser Fuzz Targets**
> **Fuzz targets** shall be created for: `DESIGN.md` import, SMDL parser, TSX extractor, Svelte extractor, Vue extractor, and `map_raw_type_with_concrete`.

*Acceptance:* Each fuzz target compiles and runs for at least 1 minute without crashing.

#### 2.4 Coverage Thresholds

**REQ‑TST‑010 – CI Coverage Gate**
> **The CI pipeline** shall run `cargo llvm-cov nextest` and fail if line coverage is below 85% for the workspace.

*Acceptance:* Coverage report shows ≥85% line coverage; CI build fails below threshold.

#### 2.5 Test Organization

**REQ‑TST‑011 – Single Source for Test Helpers**
> **All test factories** (`make_minimal_component`, `make_button_component`, `make_package_manifest`, `make_empty_spec`) shall be defined in `tests/common/mod.rs` and imported by all test files.

**REQ‑TST‑012 – E2E Tests as Rust Integration Tests**
> **The shell‑based e2e tests** (`.just-e2e/`) shall be converted to Rust integration tests using `assert_cmd` or `duct`, making them cross‑platform and debuggable.

### 3. Quality Requirements

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑MNT‑001 | All 141+ existing tests pass | `just test` exits 0 |
| NFR‑MNT‑002 | ≥85% line coverage | `cargo llvm-cov` report |
| NFR‑MNT‑003 | Zero clippy warnings | `cargo clippy --all-targets` |
| NFR‑MNT‑004 | Fuzz targets run 60s without crash | `cargo fuzz run` timeout |
| NFR‑PERF‑001 | Test suite completes in <15s (debug) | `just test` timing |

### 4. Traceability

| Goal | Capability | SRS Requirements |
|------|------------|------------------|
| G‑01 | Snapshot coverage | REQ‑TST‑001 … REQ‑TST‑005 |
| G‑02 | Quality gates | REQ‑TST‑010, NFR‑MNT‑002 |
| G‑03 | Parser robustness | REQ‑TST‑009 |
| G‑04 | Test quality | REQ‑TST‑006 … REQ‑TST‑008 |

---

## Architecture & Design Specification – UCP v0.10.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Architecture & Design Specification |
| Version | 0.10.0‑arch‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Context and Scope

v0.10.0 is a pure test infrastructure release. No library code changes. The architecture focuses on test organization, tool integration, and CI pipeline design.

### 2. Test Architecture

#### 2.1 Directory Structure

```
ucp-synthesizer/tests/
├── common/
│   └── mod.rs              # All shared test factories
├── snapshots/              # insta snapshot files (*.snap)
├── proptest/               # Property-based tests
│   ├── unify_proptest.rs
│   ├── generator_proptest.rs
│   └── conflicts_proptest.rs
├── fuzz/                   # cargo-fuzz targets
│   └── fuzz_targets/
│       ├── design_md_import.rs
│       ├── smdl_parser.rs
│       ├── tsx_extractor.rs
│       └── type_mapping.rs
├── integration/            # Existing integration tests (reorganized)
│   ├── pipeline.rs
│   ├── generators.rs
│   └── exports.rs
└── e2e/                    # assert_cmd-based CLI tests
    └── cli_tests.rs
```

#### 2.2 Snapshot Testing Strategy

`insta` is used with:
- `assert_json_snapshot!` for CAM types and export formats (A2UI, AG‑UI, W3C, AI contract).
- `assert_snapshot!` for code generator output (Dioxus, Leptos, React, GPUI, WC).
- `assert_yaml_snapshot!` for DESIGN.md front matter.
- `INSTA_UPDATE=never` in CI; `cargo insta review` locally.

A `nextest.toml` configuration enables `insta` integration:

```toml
[profile.default]
# insta integration
retries = 0
```

And `.cargo/config.toml` aliases:

```toml
[alias]
snapshot = "insta test --test-runner nextest"
snapshot-review = "insta review"
```

#### 2.3 Property‑Based Testing Strategy

`proptest` is used to validate:
1. **Type unification idempotency**: For any Rust type string, `map_raw_type_to_cam(map_raw_type_to_cam(s))` should be consistent.
2. **Generator output invariants**: For any valid manifest, the generated output is non‑empty, contains the component name, and is syntactically valid.
3. **Conflict detection determinism**: Merging the same specs in any order produces the same conflicts.

#### 2.4 CI Pipeline

```yaml
test:
  steps:
    - cargo fmt --check
    - cargo clippy --all-targets -- -D warnings
    - cargo nextest run --profile ci
    - cargo llvm-cov nextest --profile ci -- --lcov --output-path lcov.info
    - bash <(curl -s https://codecov.io/bash) -f lcov.info
  coverage_threshold: 85%
```

#### 2.5 Fuzzing

Fuzz targets use `cargo-fuzz` with libfuzzer. Each target feeds the parser random byte sequences and verifies that the parser doesn't panic.

### 3. Key Design Decisions (ADRs)

| ADR | Decision | Drivers |
|-----|----------|---------|
| ADR‑036 | Use `insta` for snapshot testing over `expect-test` | Better JSON/YAML support, review workflow, redactions |
| ADR‑037 | Use `proptest` for property testing over `quickcheck` | Better shrinking, composable strategies, mature ecosystem |
| ADR‑038 | Use `cargo-llvm-cov` over `tarpaulin` for coverage | Faster, more accurate, nextest integration |
| ADR‑039 | Use `cargo-fuzz` for parser fuzzing | Rust standard, well‑documented, CI‑integration |

### 4. Traceability

| ASR | Design Elements | ADRs |
|-----|----------------|------|
| ASR‑001 | `insta` macros, snapshot directory structure | ADR‑036 |
| ASR‑002 | `proptest` strategies, property functions | ADR‑037 |
| ASR‑003 | `cargo-fuzz` targets, fuzz directory | ADR‑039 |
| ASR‑004 | `cargo-llvm-cov` CI step | ADR‑038 |

---

## Behavioral Specification & Test Verification Plan – UCP v0.10.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Behavioral Spec & Test Verification |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Behavioral Specifications

```gherkin
Feature: Snapshot Testing
  Scenario: CAM component JSON snapshot matches
    Given a standard Button CAM component
    When serialized to JSON
    Then the output matches the stored insta snapshot

  Scenario: Dioxus generator output matches snapshot
    Given a PackageManifest with a Button component
    When the Dioxus generator runs
    Then the generated button.rs matches the stored snapshot

  Scenario: CI rejects unexpected snapshot changes
    Given a modified CAM output
    And INSTA_UPDATE=never
    When tests run in CI
    Then the test fails with a snapshot mismatch error

Feature: Property-Based Testing
  Scenario: Type unification is idempotent
    Given any random Rust type string
    When mapped through map_raw_type_to_cam
    Then mapping the result again produces the same abstract type

  Scenario: Generator output is non-empty
    Given any valid PackageManifest
    When a generator produces output
    Then the output contains the component name and is non-empty

Feature: Fuzz Testing
  Scenario: DESIGN.md parser doesn't crash
    Given random byte sequences
    When fed to the DESIGN.md import parser
    Then the parser returns an error or a valid manifest, never panics

  Scenario: SMDL parser doesn't crash
    Given random string inputs
    When fed to the SMDL parser
    Then the parser returns an error or a valid SmdlComponent, never panics
```

### 2. Test Strategy

| Layer | Goal | Tools |
|-------|------|-------|
| Snapshot tests | Capture output regressions | `insta` macros, `cargo insta review` |
| Property tests | Validate invariants | `proptest` strategies |
| Fuzz tests | Crash safety for parsers | `cargo-fuzz` |
| Coverage | Enforce quality threshold | `cargo-llvm-cov`, 85% minimum |
| CI | Automated quality gates | GitHub Actions, `just test` |

### 3. Requirements Traceability Matrix

| Requirement | Test Case | Verification |
|-------------|-----------|--------------|
| REQ‑TST‑001 | `cargo install cargo-insta` | Tool availability |
| REQ‑TST‑002 | CAM JSON snapshot test | `cargo insta test` |
| REQ‑TST‑003 | Generator output snapshot test | `cargo insta test` |
| REQ‑TST‑006 | Unify idempotency property | `proptest` test |
| REQ‑TST‑009 | Parser fuzz targets | `cargo fuzz run` |
| REQ‑TST‑010 | Coverage threshold | `cargo llvm-cov` report |
