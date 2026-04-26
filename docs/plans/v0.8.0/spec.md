# UCP v0.8.0 – Full Specification Suite

## Product Vision & Strategic Alignment – UCP v0.8.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.8.0‑vision‑1 (Draft) |
| Date | 2026-04-28 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Vision Statement

> **UCP becomes a library that developers trust to build upon.**
> v0.8.0 is the "quality" release – no new frameworks, no new export targets. The entire codebase is restructured for maintainability, consistency, and developer experience. Duplicated code is eliminated. The public API is documented. Dependencies are optional. The test suite is refactored for speed and reliability. This release transforms UCP from a prototype that works into a codebase that scales.

### 2. Elevator Pitch

> **For** open‑source contributors and toolchain developers
> **who want** to extend UCP with new extractors, generators, or export targets,
> **our product** is a Rust workspace
> **that now provides** a clean, documented public API, a reusable Generator trait, feature‑gated dependencies, properly structured modules, and a reliable test suite.
> **Unlike** the prototype codebase of v0.7.0,
> **our solution** is built for extension and maintenance, not just feature velocity.

### 3. Problem Statement

v0.7.0 shipped seven releases of continuous feature additions. The codebase accumulated technical debt: `pipeline.rs` grew to ~900 lines, the same `make_test_component` helper was copy‑pasted into seven test files, every code generator duplicated the same scaffolding logic, the MCP server used `String::contains` for JSON‑RPC dispatch, optional dependencies pulled in heavy crates unconditionally, and the public API had no documentation. This makes onboarding contributors difficult, increases the risk of regressions, and slows down future feature work.

### 4. Business Objectives

| ID | Objective | Key Results |
|----|-----------|-------------|
| G‑01 | Achieve a clean, modular architecture | Pipeline split into 5 sub‑modules; Generator trait implemented; no file over 400 lines |
| G‑02 | Eliminate test fragility | Shared test helpers in `tests/common/`; all test files updated to use them |
| G‑03 | Reduce compilation footprint | Optional dependencies behind feature flags; `cargo check --no-default-features` compiles in <5s |
| G‑04 | Improve developer experience | Public API documented; CLI dispatch type‑safe; error types structured |
| G‑05 | Maintain 100% test pass rate | All existing 145+ tests continue to pass; no regressions |

### 5. Goals and Non‑Goals

**Goals (v0.8.0):**
- Split `pipeline.rs` into focused sub‑modules.
- Extract shared test helpers into `tests/common/mod.rs`.
- Implement a `Generator` trait to eliminate code duplication.
- Clean up CLI dispatch with type‑safe enums.
- Proper JSON‑RPC handling in MCP server.
- Feature‑gate optional dependencies (`octocrab`, `spdx`, `reqwest`).
- Move dashboard HTML/JS to external template files.
- Document Svelte/Vue extractors as experimental.
- Add doc comments to all public items in `ucp‑core` and `ucp‑synthesizer`.
- Enhance `UcpError` with structured variants.

**Non‑Goals:**
- No new framework extractors or code generators.
- No new export targets.
- No new CLI commands.
- No performance optimizations beyond the structural improvements.
- No behavior changes visible to end users (the CLI interface remains identical to v0.7.0).

---

## Software Requirements Specification – UCP v0.8.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | SRS v0.8.0 |
| Version | 0.1 (Draft) |
| Date | 2026-04-28 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Introduction

This SRS defines the internal improvements for v0.8.0: module restructuring, code deduplication, dependency management, test infrastructure, API documentation, and error handling enhancements. No external behavior changes; all existing CLI commands and output formats remain identical.

### 2. Functional Requirements

#### 2.1 Pipeline Module Restructuring

**REQ‑REF‑001 – Split `pipeline.rs`**
> **The `pipeline.rs` file** shall be split into five sub‑modules: `extraction`, `unification`, `enrichment`, `conflicts`, and `output`. The existing `pipeline/mod.rs` shall re‑export all public items to preserve the current public API.

*Acceptance criteria:*
- No single file exceeds 400 lines.
- All existing tests pass without modification.
- `use ucp_synthesizer::pipeline::SynthesisOutput` continues to work.

**REQ‑REF‑002 – No circular dependencies**
> **The sub‑modules** shall be organized so that `extraction` depends on nothing else in the pipeline, `unification` depends only on `extraction` types, `enrichment` depends on `output`, and `conflicts` depends on `output`. No circular imports.

#### 2.2 Generator Trait

**REQ‑REF‑003 – Define `CodeGenerator` trait**
> **The `generate` module** shall define a `CodeGenerator` trait with methods: `file_extension()`, `map_prop_type()`, `generate_component_code()`, `write_project_files()`.

*Acceptance criteria:*
- All five generators (Dioxus, Leptos, React, GPUI, Web Components) implement this trait.
- The shared scaffolding logic (directory creation, iteration, file writing) lives in a single `generate_with` function.
- Adding a new generator requires implementing the trait only, not duplicating scaffold code.

**REQ‑REF‑004 – Registry generator uses trait where applicable**
> **The registry generator** shall reuse the trait’s `generate_component_code` method via delegation, even though it doesn't implement the full trait.

#### 2.3 CLI Dispatch Cleanup

**REQ‑REF‑005 – Type‑safe target enums**
> **The CLI** shall define `GeneratorTarget` and `ExportTarget` enums with `clap::ValueEnum` derive, replacing raw string matching for the `--target` argument.

*Acceptance criteria:*
- `ucp generate --target dioxus` continues to work.
- Invalid targets produce clap‑generated error messages, not runtime `anyhow::bail!`.
- The `cmd_generate`, `cmd_export`, and `cmd_registry_build` functions all use `load_manifest()` to avoid duplicated deserialization logic.

**REQ‑REF‑006 – Shared manifest loader**
> **The `load_manifest` function** shall accept a `Path` and return a `PackageManifest`, automatically detecting whether the input is a `PackageManifest` or a `SynthesisOutput`.

#### 2.4 Test Infrastructure

**REQ‑REF‑007 – Shared test helpers**
> **The `tests/common/mod.rs`** shall provide: `make_minimal_component()`, `make_button_component()`, `make_component_with_props()`, `make_empty_spec()`, `make_package_manifest()`.

*Acceptance criteria:*
- All test files that previously defined their own helpers are updated to import from `common`.
- No duplicate `make_test_component` or `make_button_component` definitions exist.

#### 2.5 MCP Server JSON‑RPC

**REQ‑REF‑008 – Proper JSON‑RPC parsing**
> **The MCP server** shall use `serde_json::Value` for request/response handling, dispatching on the `method` field rather than `String::contains`.

*Acceptance criteria:*
- `{"jsonrpc":"2.0","method":"tool/invalid","id":1}` returns a proper error response with `-32601` code.
- The `dashboard` tool response is valid MCP App HTML.
- The server does not panic on malformed JSON.

#### 2.6 Feature‑Gated Dependencies

**REQ‑REF‑009 – Optional dependency features**
> **The `ucp‑synthesizer` crate** shall define features: `github-discovery` (gates `octocrab`), `license-check` (gates `spdx`), `llm` (gates `reqwest`). All are enabled by default to preserve backward compatibility.

*Acceptance criteria:*
- `cargo check --no-default-features` compiles without `octocrab`, `spdx`, or `reqwest`.
- The `discovery` module and `check_spdx_compliance` function are conditionally compiled with `#[cfg(feature = "...")]`.
- The LLM enrichment path is gated; pipeline runs without it when feature is disabled.

#### 2.7 Dashboard Template Extraction

**REQ‑REF‑010 – External HTML template**
> **The dashboard HTML** shall be read from a static file (`src/dashboard/template.html`) using `include_str!` rather than being embedded as a `format!` string in Rust code.

*Acceptance criteria:*
- The dashboard generation output is identical to v0.7.0.
- Editing the template does not require recompilation of Rust code (it does, but it's a separate file).

#### 2.8 Svelte/Vue Extractor Documentation

**REQ‑REF‑011 – Document experimental status**
> **The `svelte_ast.rs` and `vue_ast.rs` modules** shall carry doc comments stating they are experimental, best‑effort extractors that use string matching and may miss patterns.

*Acceptance criteria:*
- Each module starts with `/// Experimental extractor. Uses simple string matching and may miss components.`.
- The CLI help text for `.svelte` and `.vue` files noted as experimental.

#### 2.9 Public API Documentation

**REQ‑REF‑012 – Document all public items**
> **Every `pub` item** in `ucp‑core` and `ucp‑synthesizer` shall have a `///` doc comment. Modules may use `#![deny(missing_docs)]` with appropriate `#[allow]` on internal items.

*Acceptance criteria:*
- `cargo doc --no-deps` produces docs without warnings.
- Key entry points (`extract_rust_components`, `map_raw_type_with_concrete`, `run_pipeline`, etc.) have usage examples.

#### 2.10 Error Type Enhancements

**REQ‑REF‑013 – Structured error payloads**
> **The `UcpError` enum** shall add structured variants for `Conflict` (with component/prop fields) and `LlmInference` (with optional model field). Existing string variants remain but are supplemented.

*Acceptance criteria:*
- `Conflict` errors can be matched on component and prop name.
- Existing code that formats errors via `Display` continues to work.

#### 2.11 Minor Cleanups

**REQ‑REF‑014 – Remove `map_raw_type_to_cam` wrapper**
> **The `map_raw_type_to_cam` function** shall be removed; its two call sites updated to use `map_raw_type_with_concrete(...).0`.

**REQ‑REF‑015 – SmdlComponent ID parameter**
> **`parse_smdl`** shall accept a `component_id: &str` parameter rather than hardcoding `"ucp-smdl"`.

**REQ‑REF‑016 – Unused import cleanup**
> **All `#[allow(unused_imports)]` and `#[allow(dead_code)]` annotations** shall be resolved (either used or removed). Warnings count reduced to zero under `cargo clippy --all-targets`.

### 3. Quality Requirements

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑MNT‑001 | No file over 400 lines (except generated/templates) | `find . -name '*.rs' -exec wc -l {} + \| awk '$1>400'` returns empty |
| NFR‑MNT‑002 | Zero clippy warnings | `cargo clippy --all-targets` exits 0 with no warnings |
| NFR‑MNT‑003 | All 145+ tests pass | `just test` exits 0 |
| NFR‑MNT‑004 | `cargo doc` produces no warnings | `cargo doc --no-deps` 2>&1 has no warning lines |
| NFR‑PERF‑001 | `cargo check --no-default-features` completes in <5s | Timed on CI reference hardware |

### 4. Traceability

| Goal | Capability | SRS Requirements |
|------|------------|------------------|
| G‑01 | Pipeline sub‑modules | REQ‑REF‑001, REQ‑REF‑002 |
| G‑01 | Generator trait | REQ‑REF‑003, REQ‑REF‑004 |
| G‑02 | Shared test helpers | REQ‑REF‑007 |
| G‑03 | Feature‑gated deps | REQ‑REF‑009 |
| G‑04 | Public API docs | REQ‑REF‑012 |
| G‑04 | CLI dispatch cleanup | REQ‑REF‑005, REQ‑REF‑006 |
| G‑04 | Error type enhancements | REQ‑REF‑013 |
| G‑05 | Test pass rate | NFR‑MNT‑003 |

---

## Architecture & Design Specification – UCP v0.8.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Architecture & Design Specification |
| Version | 0.8.0‑arch‑1 (Draft) |
| Date | 2026-04-28 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Context and Scope

v0.8.0 is a pure refactoring release. No new features. The architecture changes are all internal: restructuring modules, introducing traits, gating dependencies, and cleaning up tests.

### 2. Architecturally Significant Requirements

| ID | Requirement | Source |
|----|-------------|--------|
| ASR‑001 | Pipeline sub‑module split | REQ‑REF‑001, REQ‑REF‑002 |
| ASR‑002 | Generator trait | REQ‑REF‑003, REQ‑REF‑004 |
| ASR‑003 | Feature‑gated dependencies | REQ‑REF‑009 |

### 3. System Design

#### 3.1 Pipeline Restructuring

**Before:**
```
pipeline.rs (~900 lines)
```

**After:**
```
pipeline/
├── mod.rs        (~80 lines — public facade, re‑exports)
├── extraction.rs (~250 lines — run_pipeline_with_options, walk_source_dir)
├── unification.rs (~200 lines — unify_rust_component, unify_tsx, unify_struct)
├── enrichment.rs (~150 lines — enrich_components_with_llm, smdl_to_state_machine)
├── conflicts.rs  (~150 lines — detect_conflicts, detect_cross_spec_conflicts)
└── output.rs     (~150 lines — SynthesisOutput, PipelineStats, serialization, to_package_manifest)
```

Each sub‑module is `pub(crate)`; `mod.rs` re‑exports the public types:
```rust
pub use extraction::*;
pub use unification::*;
// etc.
pub use output::{SynthesisOutput, PipelineStats};
```

#### 3.2 Generator Trait Design

```rust
pub trait CodeGenerator {
    /// File extension for generated output (e.g., "rs", "tsx", "js").
    fn file_extension(&self) -> &str;

    /// Map a CAM prop to the framework‑specific type string.
    fn map_prop_type(&self, prop: &CanonicalAbstractProp) -> String;

    /// Generate the full source code for a single component.
    fn generate_component_code(&self, comp: &CanonicalAbstractComponent) -> String;

    /// Write the project configuration files (Cargo.toml, package.json, etc.).
    fn write_project_files(&self, manifest: &PackageManifest, dir: &Path) -> Result<()>;
}
```

A free function provides the shared scaffold:
```rust
pub fn generate_with<G: CodeGenerator>(
    manifest: &PackageManifest,
    output_dir: &str,
    gen: &G,
) -> Result<()> {
    let dir = Path::new(output_dir);
    fs::create_dir_all(dir.join("src"))?;
    for comp in &manifest.components {
        let name = comp.id.rsplit(':').next().unwrap_or(&comp.id);
        let file_name = to_snake_case(name);
        let path = dir.join("src").join(format!("{}.{}", file_name, gen.file_extension()));
        fs::write(&path, gen.generate_component_code(comp))?;
    }
    gen.write_project_files(manifest, dir)
}
```

Each generator then becomes a struct implementing the trait:
```rust
pub struct DioxusGenerator;
impl CodeGenerator for DioxusGenerator { ... }
```

#### 3.3 Feature Flags

```toml
[features]
default = ["github-discovery", "license-check", "llm"]
github-discovery = ["octocrab"]
license-check = ["spdx"]
llm = ["reqwest"]
```

Conditional compilation:
- `discovery.rs` — `#[cfg(feature = "github-discovery")]` on the entire module.
- `security::check_spdx_compliance` — `#[cfg(feature = "license-check")]`.
- `llm.rs` and the enrichment path in `pipeline/enrichment.rs` — `#[cfg(feature = "llm")]`.

The `#[cfg(not(feature = "..."))]` stubs provide graceful degradation: the `discovery` module is empty, `check_spdx_compliance` always returns `Ok(())`, and LLM enrichment is skipped with a log message.

#### 3.4 MCP Server JSON‑RPC

Replace the `String::contains` approach with:
```rust
#[derive(Deserialize)]
struct McpRequest {
    jsonrpc: String,
    method: String,
    #[serde(default)]
    params: Option<serde_json::Value>,
    id: serde_json::Value,
}

fn handle_request(raw: &str, spec: &SynthesisOutput) -> String {
    let req: McpRequest = match serde_json::from_str(raw) {
        Ok(r) => r,
        Err(e) => return error_response(None, -32700, &e.to_string()),
    };
    match req.method.as_str() {
        "tools/list" => tools_list_response(req.id),
        "tools/call" => tools_call_response(req.id, req.params, spec),
        _ => error_response(Some(req.id), -32601, "Method not found"),
    }
}
```

#### 3.5 Dashboard Template

Move the HTML template to `ucp-synthesizer/src/dashboard/template.html`. The `build_html` function becomes:
```rust
fn build_html(spec_json: &str) -> String {
    let template = include_str!("template.html");
    template.replace("{spec_json}", spec_json)
}
```

The JavaScript remains inline but is now in a dedicated file that can be edited independently.

### 4. Key Design Decisions (ADRs)

| ADR | Decision | Drivers |
|-----|----------|---------|
| ADR‑028 | Split pipeline into sub‑modules by concern | ASR‑001, maintainability |
| ADR‑029 | Generator trait with shared scaffold | ASR‑002, eliminate duplication |
| ADR‑030 | Feature flags for optional deps, default on | ASR‑003, backward compatibility |
| ADR‑031 | MCP server uses serde_json for JSON‑RPC | Correctness over simplicity |
| ADR‑032 | Dashboard template as external file | Maintainability, edit without touching Rust |

### 5. Traceability

| ASR | Design Elements | ADRs |
|-----|----------------|------|
| ASR‑001 | Pipeline sub‑module structure | ADR‑028 |
| ASR‑002 | CodeGenerator trait, `generate_with` | ADR‑029 |
| ASR‑003 | Feature flags in Cargo.toml | ADR‑030 |

---

## Behavioral Specification & Test Verification Plan – UCP v0.8.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Behavioral Spec & Test Verification |
| Version | 0.1 (Draft) |
| Date | 2026-04-28 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Behavioral Specifications

Since v0.8.0 is a pure refactoring release, behavioral specs verify that **no observable behavior changes**. All existing v0.7.0 tests must pass identically.

```gherkin
Feature: Backward Compatibility
  As a user of UCP v0.7.0
  I want to upgrade to v0.8.0 without any changes to my workflow
  So that I can benefit from the improved codebase without disruption

  Scenario: All CLI commands produce identical output
    Given a v0.7.0 spec file
    When I run `ucp generate --target dioxus` with v0.8.0
    Then the generated files are identical to v0.7.0 output

  Scenario: All 145+ tests pass
    Given the v0.8.0 codebase
    When I run `just test`
    Then all tests pass with zero failures

  Scenario: Feature flags work correctly
    Given I compile with `--no-default-features`
    Then the binary is smaller and LLM/discovery features are unavailable
    And `ucp bootstrap` still works (without Ollama integration)
```

#### Specific Refactoring Verification

```gherkin
Scenario: Pipeline sub-module re-exports preserve API
  Given I import `ucp_synthesizer::pipeline::SynthesisOutput`
  When I compile
  Then it resolves to the type from `pipeline/output.rs` via `pipeline/mod.rs`

Scenario: Generator trait produces identical output
  Given a PackageManifest with a Button component
  When I generate Dioxus code using the new trait‑based generator
  Then the output is byte‑for‑byte identical to the old `generate_dioxus` function

Scenario: MCP server handles malformed JSON gracefully
  Given I send `{"invalid json` to the MCP server
  Then it returns a JSON‑RPC error with code -32700 (Parse error)
  And does not panic or crash

Scenario: Dashboard output unchanged
  Given a SynthesisOutput with components
  When I generate a dashboard with the external template
  Then the produced HTML is identical to v0.7.0 (except for the template loading mechanism)
```

### 2. Test Strategy

| Layer | Goal | Tools |
|-------|------|-------|
| Unit tests | Verify Generator trait implementations produce correct output | Rust #[test] |
| Unit tests | Verify MCP server JSON‑RPC parsing handles valid/invalid input | Rust #[test] |
| Integration tests | Verify pipeline sub‑modules produce identical output to monolith | Compare JSON output |
| Regression tests | All 145+ existing tests pass without modification | cargo nextest |
| Compilation tests | Feature flag combinations compile | `cargo check --no-default-features`, `cargo check --all-features` |
| Doc tests | `cargo doc --no-deps` produces no warnings | CI step |
| Lint | `cargo clippy --all-targets` reports zero warnings | CI step |

### 3. NFR Verification

| NFR | Verification Method | Evidence |
|-----|---------------------|----------|
| MNT‑001 | `find . -name '*.rs' -exec wc -l {} +` script | No file >400 lines report |
| MNT‑002 | `cargo clippy --all-targets` | Zero warnings |
| MNT‑003 | `cargo nextest run` | 145+ tests pass |
| MNT‑004 | `cargo doc --no-deps` | Zero warnings |
| PERF‑001 | Timed `cargo check --no-default-features` | <5s |

### 4. Requirements Traceability Matrix

| Requirement | Test Case | Verification |
|-------------|-----------|--------------|
| REQ‑REF‑001 | Pipeline public API unchanged | Compilation + all tests pass |
| REQ‑REF‑003 | Generator trait output matches old output | Byte‑for‑byte comparison tests |
| REQ‑REF‑005 | Invalid target produces clap error | Manual CLI test |
| REQ‑REF‑008 | MCP server handles malformed JSON | Unit test with invalid input |
| REQ‑REF‑009 | `--no-default-features` compiles | `cargo check --no-default-features` |
| REQ‑REF‑010 | Dashboard output unchanged | Snapshot test |
| REQ‑REF‑012 | `cargo doc` zero warnings | `cargo doc --no-deps` |
| REQ‑REF‑013 | Error payload accessible | Unit test matching on structured variant |

---

This completes the specification suite for v0.8.0. Would you like me to produce the implementation plan next?
