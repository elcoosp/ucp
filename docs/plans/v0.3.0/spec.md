Based on our earlier discussion, the natural candidates for v0.3.0 are:

- **shadcn Registry Export** — produce `registry-item.json` files from CAM
- **Leptos Code Generator** — mirror the Dioxus generator for Leptos
- **Full AST context detection** — replace string heuristics with proper syn visitor
- **W3C UI Specification Schema alignment** — research and mapping

I'll start with the **Vision & Strategic Alignment** document for v0.3.0, proposing this scope. You can adjust before we continue to the SRS, Architecture, and Test plans.

---

# Product Vision & Strategic Alignment – UCP v0.3.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.3.0‑vision‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

## 1. Vision Statement

> **UCP becomes the universal bridge between component libraries and the tools that consume them.**  
> v0.3.0 transforms the CAM from an internal model into a **distribution format** — generating shadcn‑compatible registry items, producing framework‑specific starter code for both Dioxus and Leptos, and aligning with emerging web component description standards.

## 2. Elevator Pitch (Moore Template)

> **For** component‑library maintainers and framework authors  
> **who need** to distribute their components across the shadcn ecosystem and multiple Rust frontend frameworks,  
> **our product** is a CLI pipeline that extracts, unifies, and republishes UI components  
> **that provides** automated registry export, multi‑framework code generation (Dioxus + Leptos), and W3C‑aligned component descriptions.  
> **Unlike** manual porting or single‑framework tools,  
> **our solution** preserves concrete types, detects cross‑framework conflicts, and outputs directly into the shadcn registry standard.

## 3. Problem Statement & Business Context

### 3.1 The Problem
v0.2.0 proved that UCP can extract Dioxus components and generate skeleton code. But the output is still "just files" — not integrated into any distribution channel. Specifically:

- **No registry integration**: The shadcn ecosystem uses a standard `registry-item.json` format for distributing components. Without it, generated components can't be installed via `npx shadcn add`.
- **Dioxus‑only generation**: Leptos is the other major Rust frontend framework, but UCP can't yet generate Leptos components from the same CAM.
- **Fragile context detection**: The string‑based `use_context_provider` detection works for simple cases but misses indirect usage, re‑exports, and macro‑generated code.
- **No standards alignment**: The W3C UI Specification Schema Community Group is defining a vendor‑neutral component description format. Mapping CAM to this emerging standard would position UCP as an early implementor.

### 3.2 Why Now?
- shadcn CLI usage is growing rapidly; registry compatibility is a force multiplier for adoption.
- Leptos adoption is accelerating; a Leptos code generator proves CAM's framework‑agnostic value.
- The W3C Community Group is actively seeking implementors and feedback.
- The string‑heuristic context detection has known gaps that will become more problematic as usage scales.

### 3.3 Business Objectives

| ID | Objective | Key Results |
|----|-----------|-------------|
| G‑01 | Become a first‑class shadcn registry producer | Generate valid `registry.json` from CAM; components installable via `npx shadcn add` |
| G‑02 | Prove framework‑agnostic code generation | CAM → Leptos stubs that compile with `cargo check` |
| G‑03 | Improve extraction accuracy | AST‑based context detection catches 95%+ of provider/consumer patterns |
| G‑04 | Align with web standards | CAM → W3C UI Spec Schema mapping documented; optional `--w3c` export flag |

## 4. Target Users

### 4.1 Primary: Component‑library Maintainers
- Already maintain a shadcn port (Dioxus, Leptos, etc.)
- Want one‑click publishing to the shadcn registry.
- Need automated detection of props drift from upstream.

### 4.2 Secondary: Framework Ecosystem Developers
- Building tooling around component libraries.
- Want machine‑readable component specs that conform to a web standard.

### 4.3 Secondary: AI/LLM Developers (unchanged from v0.2.0)
- Use CAM as structured training data; benefit from richer metadata.

## 5. User Needs & Value Proposition

| Need | v0.3.0 Solution |
|------|----------------|
| "I want to publish my Dioxus components to the shadcn registry." | `ucp generate --target shadcn-registry` produces ready‑to‑use `registry.json` |
| "I need to generate a Leptos port from the same spec." | `ucp generate --target leptos` produces Leptos stubs |
| "My component's context provider isn't being detected." | Full AST visitor replaces string heuristics; catches macros and re‑exports |
| "I need a vendor‑neutral component spec for my design system." | `ucp bootstrap --w3c` exports W3C‑aligned JSON |

## 6. Goals and Non‑Goals

### Goals (v0.3.0)
- [ ] shadcn registry JSON export (`--target shadcn-registry`)
- [ ] Leptos code generator (`--target leptos`)
- [ ] AST‑based context detection (replaces string heuristic)
- [ ] W3C UI Specification Schema mapping (research + implementation)
- [ ] Snake‑case file naming (carried forward from v0.2.0 fix)

### Non‑Goals
- **NG‑01:** No visual editor or dashboard UI.
- **NG‑02:** No code generation for React, Vue, Svelte (Rust frameworks only for now).
- **NG‑03:** No real‑time file watching during extraction.
- **NG‑04:** No formal W3C spec contribution (implementation only).

## 7. Success Metrics

| Metric | Target |
|--------|--------|
| Registry export produces valid `registry.json` | Passes shadcn CLI validation |
| Leptos generator output compiles | `cargo check` in generated project passes |
| Context detection accuracy | >95% on shadcn‑dioxus components |
| W3C mapping coverage | >80% of CAM fields mapped to W3C fields |
| Existing tests continue to pass | 100% pass rate maintained |

## 8. Out of Scope for v0.3.0

- Multi‑framework conflict detection using concrete types (deferred to v0.4.0).
- Code generation for non‑Rust frameworks.
- GUI or web dashboard for spec review.
- Performance optimization beyond current baseline.

---

Does this scope look right for v0.3.0? I can adjust the emphasis (e.g., drop W3C alignment if it's too research‑heavy, or add another feature). Once confirmed, I'll write the SRS, Architecture, and Test plans in the same session.
Now we'll write the full specification suite for v0.3.0 – SRS, Architecture, and Test Plan – all aligned with the Vision we just created. Let's produce each in sequence.

---

# Software Requirements Specification – UCP v0.3.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | SRS v0.3.0 |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

## 1. Introduction and Scope

### 1.1 Purpose
This SRS defines the new capabilities for v0.3.0 of UCP. Building on v0.2.0's Dioxus extraction, concrete type preservation, and code generation, v0.3.0 extends the pipeline to produce **distribution‑ready output** (shadcn registry format), adds a **Leptos code generator**, replaces the **string‑heuristic context detection** with full AST walking, and aligns CAM with the emerging **W3C UI Specification Schema**.

### 1.2 Scope
In scope:
- Shadcn registry JSON export
- Leptos code generator
- AST‑based context provider/consumer detection
- W3C UI Specification Schema mapping and export
- Snake‑case file naming (hardened from v0.2.0)

Out of scope:
- Code generation for React, Vue, Svelte
- GUI or dashboard
- Real‑time file watching
- Formal W3C standards contribution

## 2. System Context and Overview

UCP v0.3.0 operates as a CLI tool that:
1. Extracts components from source (Rust, TSX) – enhanced with AST‑based context detection.
2. Maps to CAM (unchanged data model, new fields if needed for W3C).
3. Optionally enriches via LLM.
4. Detects conflicts.
5. Outputs JSON spec.
6. **New** – exports shadcn `registry-item.json` files.
7. **New** – exports W3C‑aligned JSON.
8. **New** – generates Leptos project stubs.

## 3. Functional Requirements

### 3.1 Shadcn Registry Export

**REQ‑REG‑001 – Registry item generation**
> **When** the user runs `ucp generate --spec <spec.json> --target shadcn-registry --output <dir>`, the system shall produce one `registry-item.json` per component and an index `registry.json`.

*Acceptance criteria*:
- Each `registry-item.json` contains: `name`, `type` (always `"registry:ui"`), `registryDependencies`, `files` array with `{ path, content }`.
- The `files` array includes the generated source code for that component.
- The index `registry.json` lists all items.
- Output passes validation against the shadcn registry schema.

**REQ‑REG‑002 – Dependency resolution**
> **When** a component references other components (e.g., Button used by Dialog), the registry item shall list those dependencies in `registryDependencies`.

*Acceptance criteria*:
- Dependencies are detected from the CAM prop types and component references in the source.
- Circular dependencies are handled gracefully (logged as warning).

### 3.2 Leptos Code Generator

**REQ‑LEP‑001 – Generate Leptos source from PackageManifest**
> **When** given a `PackageManifest` and `--target leptos`, the generator shall create a Leptos project with stub components.

*Acceptance criteria*:
- Each component produces a valid Leptos `#[component]` function with `view!` macro.
- Props struct uses `#[derive(Props)]` or equivalent Leptos pattern.
- The generated project compiles with `cargo check`.
- The prop‑type mapping mirrors the Dioxus generator but uses Leptos‑specific types (`RwSignal`, `MaybeSignal`, etc.).

**REQ‑LEP‑002 – Prop mapping rules for Leptos**
> **The Leptos generator** shall use these mapping rules:
- `ControlFlag` with default → `#[prop(default)] bool`
- `SpreadAttributes` → `#[prop(attrs)] Vec<Attribute>`
- `Renderable` → `Children`
- `AsyncEventHandler` → `Option<Callback<MouseEvent>>`
- `ControlledValue` → `RwSignal<T>`
- `UncontrolledValue` → `MaybeSignal<T>`

### 3.3 AST‑Based Context Detection

**REQ‑CTX‑001 – Replace string heuristic with syn visitor**
> **The Dioxus extractor** shall use a `syn::visit::Visitor` to walk the function body AST and detect `use_context_provider` and `use_context` calls, rather than scanning the raw source text.

*Acceptance criteria*:
- All existing context detection tests continue to pass.
- New test: context provided via a helper function/macro is detected.
- New test: context consumed via a re‑exported function is detected.

**REQ‑CTX‑002 – Detection accuracy**
> **The AST‑based detector** shall correctly identify the provided/consumed context type in all `shadcn‑dioxus` components.

*Acceptance criteria*:
- At least 95% of context‑using components in `packages/ui` are correctly tagged.

### 3.4 W3C UI Specification Schema

**REQ‑W3C‑001 – CAM to W3C mapping**
> **The system** shall provide a documented mapping from `CanonicalAbstractComponent` fields to the W3C UI Specification Schema fields.

*Acceptance criteria*:
- A mapping document exists in `docs/w3c‑mapping.md`.
- All CAM fields are mapped, or explicitly noted as "no equivalent."

**REQ‑W3C‑002 – Export to W3C format**
> **When** the user runs `ucp bootstrap --w3c`, the output shall include a `ucp‑spec.w3c.json` conforming to the latest W3C UI Spec Schema draft.

*Acceptance criteria*:
- The W3C JSON is valid per the draft schema.
- All component props, events, and state machines are represented.

### 3.5 Snake‑Case File Naming (hardening)

**REQ‑NAM‑001 – Consistent snake_case in all generators**
> **All code generators** (Dioxus and Leptos) shall produce files named in `snake_case` (e.g., `accordion_content.rs` not `accordioncontent.rs`).

*Acceptance criteria*:
- The `to_snake_case` utility is shared between generators.
- Integration test verifies file names.

## 4. Quality and Non‑Functional Requirements

### 4.1 Performance
| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑PERF‑001 | Registry export for 200 components < 3s | `just test-perf` measurement |
| NFR‑PERF‑002 | AST context detection adds <10% overhead to extraction | Comparison benchmark |

### 4.2 Reliability
| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑REL‑001 | Generated Leptos project compiles | `cargo check` in CI |
| NFR‑REL‑002 | Registry JSON passes schema validation | Automated validation in CI |

### 4.3 Maintainability
| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑MNT‑001 | Test coverage ≥ 85% for new modules | Coverage report |
| NFR‑MNT‑002 | Zero clippy warnings | CI lint gate |

## 5. External Interfaces

### 5.1 CLI Interface
- `generate --target shadcn-registry` (new)
- `generate --target leptos` (new)
- `bootstrap --w3c` (new flag)

### 5.2 Output Formats
- `registry.json` / `registry-item.json` (shadcn format)
- `ucp-spec.w3c.json` (W3C format)

## 6. Traceability

| Goal | Capability | SRS Requirements |
|------|------------|------------------|
| G‑01 | Shadcn registry export | REQ‑REG‑001, REQ‑REG‑002 |
| G‑02 | Leptos code generator | REQ‑LEP‑001, REQ‑LEP‑002 |
| G‑03 | AST context detection | REQ‑CTX‑001, REQ‑CTX‑002 |
| G‑04 | W3C alignment | REQ‑W3C‑001, REQ‑W3C‑002 |

---

# Architecture & Design Specification – UCP v0.3.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Architecture & Design Specification |
| Version | 0.3.0‑arch‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

## 1. Context and Scope

### 1.1 Objective
UCP v0.3.0 adds four major capabilities: shadcn registry export, Leptos code generation, AST‑based context detection, and W3C UI Schema alignment. All build on the v0.2.0 CAM and pipeline without breaking changes.

### 1.2 Architecturally Significant Requirements (ASRs)

| ID | Requirement | Source |
|----|-------------|--------|
| ASR‑001 | Shadcn registry export format | REQ‑REG‑001, REQ‑REG‑002 |
| ASR‑002 | Leptos code generator | REQ‑LEP‑001, REQ‑LEP‑002 |
| ASR‑003 | AST‑based context detection | REQ‑CTX‑001, REQ‑CTX‑002 |
| ASR‑004 | W3C schema mapping and export | REQ‑W3C‑001, REQ‑W3C‑002 |

## 2. Goals and Non‑Goals

### Goals
1. Generate valid `registry-item.json` files from CAM.
2. Generate valid Leptos component stubs.
3. Replace string‑based context detection with AST walking.
4. Map CAM to W3C UI Spec Schema and export in that format.
5. Keep the existing CAM backward‑compatible (additive changes only).

### Non‑Goals
- No changes to the extraction pipeline for non‑Dioxus frameworks.
- No new extractors for additional frameworks.
- No GUI.

## 3. System Design

### 3.1 New Module: `generate::registry`

**Purpose:** Transform `PackageManifest` into shadcn registry format.

**Key data structures:**

```rust
pub struct RegistryItem {
    pub name: String,
    pub r#type: String,
    pub registry_dependencies: Vec<String>,
    pub files: Vec<RegistryFile>,
}

pub struct RegistryFile {
    pub path: String,
    pub content: String,
}
```

**Algorithm:**
1. For each component in the manifest:
   a. Generate the source code using the appropriate generator (currently Dioxus).
   b. Resolve dependencies by scanning prop types for references to other components.
   c. Create a `RegistryItem` with one `RegistryFile` containing the generated source.
2. Write `registry.json` (index) and individual `registry-item.json` files.

### 3.2 New Module: `generate::leptos`

**Purpose:** Transform `PackageManifest` into a Leptos project.

**Design:**
- Mirrors `generate::dioxus` but uses Leptos‑specific syntax and types.
- Shared prop‑mapping logic is extracted into `generate::common` (refactored from `dioxus.rs`).
- `to_snake_case` utility is shared.

### 3.3 Refactored: `extract::dioxus_ast` – Context Detection

**Current (v0.2.0):**
```rust
fn find_context_types(&self) -> (Option<String>, Vec<String>) {
    // String search on self.source
}
```

**New (v0.3.0):**
```rust
struct ContextVisitor {
    provided: Option<String>,
    consumed: Vec<String>,
}

impl Visit<'_> for ContextVisitor {
    fn visit_expr_call(&mut self, call: &syn::ExprCall) {
        // Match use_context_provider(...) and use_context::<T>()
    }
}
```

The `ContextVisitor` is run on the function body of each `#[component]` function, providing accurate detection that works with macros, helper functions, and re‑exports.

### 3.4 New Module: `export::w3c`

**Purpose:** Map CAM to W3C UI Specification Schema.

**Design:**
- Define a `W3cComponent` struct that mirrors the W3C draft schema.
- Implement `From<CanonicalAbstractComponent> for W3cComponent`.
- Serialize and write alongside the standard UCP spec when `--w3c` flag is used.

## 4. Key Design Decisions (ADRs)

### ADR‑006: Shadcn Registry as a Generator Target
**Context:** The shadcn ecosystem uses a specific JSON format for component distribution.  
**Decision:** Add `shadcn-registry` as a `--target` for the `generate` command, producing `registry.json` and per‑component files.  
**Drivers:** ASR‑001.  
**Consequences:** The generator now couples to a specific distribution format; future registry format changes require updates.

### ADR‑007: Shared Prop‑Mapping Between Generators
**Context:** Both Dioxus and Leptos generators need prop‑type mapping.  
**Decision:** Extract common mapping logic into `generate::common` module, with framework‑specific overrides in `dioxus.rs` and `leptos.rs`.  
**Drivers:** ASR‑002, DRY principle.  
**Consequences:** Cleaner code, easier to add future framework generators.

### ADR‑008: AST‑Based Context Detection
**Context:** String‑based detection misses indirect context usage.  
**Decision:** Implement `ContextVisitor` using `syn::visit` to walk function bodies.  
**Drivers:** ASR‑003.  
**Consequences:** More accurate detection; slightly more complex code; ~10% extraction overhead (acceptable per NFR‑PERF‑002).

### ADR‑009: W3C Export as Separate Output
**Context:** W3C schema is still a draft; binding CAM directly to it risks churn.  
**Decision:** Keep the native CAM format as the primary output. Add optional `--w3c` flag for W3C export. Maintain a mapping document.  
**Drivers:** ASR‑004.  
**Consequences:** Dual output adds maintenance burden; mitigated by clean separation in `export::w3c` module.

## 5. Traceability

| ASR | Design Elements | ADRs |
|-----|----------------|------|
| ASR‑001 | `generate::registry` module, CLI target | ADR‑006 |
| ASR‑002 | `generate::leptos`, `generate::common` | ADR‑007 |
| ASR‑003 | `ContextVisitor` in `dioxus_ast` | ADR‑008 |
| ASR‑004 | `export::w3c` module, `--w3c` flag | ADR‑009 |

---

# Behavioral Specification & Test Verification Plan – UCP v0.3.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Behavioral Spec & Test Verification |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

## 1. Behavioral Specifications

### Feature: Shadcn Registry Export

```gherkin
Scenario: Export a button component as registry item
  Given a PackageManifest with one Button component
  When the user runs `ucp generate --target shadcn-registry`
  Then a `registry.json` index file is created
  And a `registry-item-button.json` file is created
  And the item JSON contains:
    - name: "button"
    - type: "registry:ui"
    - registryDependencies: []
    - files: [ { path: "src/button.rs", content: "..." } ]

Scenario: Component with dependencies
  Given a PackageManifest with components "Dialog" and "Button"
  And Dialog depends on Button (references ButtonProps)
  When the registry export runs
  Then the Dialog registry item lists "button" in registryDependencies
```

### Feature: Leptos Code Generator

```gherkin
Scenario: Generate a Leptos button stub
  Given a PackageManifest with a Button component:
    - prop "disabled": concrete_type="bool", abstract_type=ControlFlag
    - prop "label": concrete_type="String", abstract_type=StaticValue(Any)
  When the Leptos generator runs
  Then a file "button.rs" is created
  And it contains:
    - #[component] pub fn Button(disabled: bool, label: String) -> impl IntoView
    - view! { <button>{label}</button> }
```

### Feature: AST Context Detection

```gherkin
Scenario: Detect context provided via a helper function
  Given a Rust file with:
    ```
    fn provide_dialog_ctx(open: bool) {
        use_context_provider(|| DialogContext { open: Signal::new(open) });
    }
    #[component] pub fn Dialog(props: DialogProps) -> Element {
        provide_dialog_ctx(props.open);
        rsx! { ... }
    }
    ```
  When the AST context detector runs
  Then Dialog has provided_context = Some("DialogContext")

Scenario: Detect context consumed via re‑export
  Given a file that calls `ctx!()` where `ctx!` is a macro wrapping `use_context::<DialogContext>()`
  When the AST context detector runs
  Then the component has consumed_contexts containing "DialogContext"
```

### Feature: W3C Export

```gherkin
Scenario: Export CAM as W3C UI Spec
  Given a SynthesisOutput with a Button component
  When the user runs `ucp bootstrap --w3c`
  Then a `ucp-spec.w3c.json` file is created
  And it conforms to the W3C UI Specification Schema draft
```

## 2. Test Strategy

| Layer | Goal | Tools |
|-------|------|-------|
| Unit tests | Individual functions: to_snake_case, prop mapping, context visitor | Rust #[test] |
| Integration tests | Full pipeline: extract → generate → validate | tempfile, tokio::test |
| Schema validation | Verify registry JSON and W3C JSON against schemas | jsonschema, cargo test |
| Compilation tests | Generated Leptos/Dioxus projects compile | cargo check in temp dir |
| Regression tests | All v0.2.0 tests continue to pass | cargo nextest |

## 3. NFR Verification

| NFR | Verification Method | Evidence |
|-----|---------------------|----------|
| PERF‑001 | `just test-perf` with 200 components | Timing log |
| PERF‑002 | Benchmark comparison script | Before/after timing report |
| REL‑001 | `cargo check` on generated Leptos project | CI log |
| REL‑002 | Schema validation in CI | Validation pass log |
| MNT‑001 | Coverage report | llvm-cov output |
| MNT‑002 | `just lint` | Zero warnings |

## 4. Requirements Traceability Matrix

| Requirement ID | Test Case | Verification |
|---------------|-----------|--------------|
| REQ‑REG‑001 | SCN‑REG‑export‑button | Test + Schema validation |
| REQ‑REG‑002 | SCN‑REG‑dependencies | Test |
| REQ‑LEP‑001 | SCN‑LEP‑button‑stub | Test + cargo check |
| REQ‑LEP‑002 | SCN‑LEP‑prop‑mapping | Unit test |
| REQ‑CTX‑001 | SCN‑CTX‑helper‑fn | Unit test |
| REQ‑CTX‑002 | SCN‑CTX‑accuracy | Integration test on shadcn‑dioxus |
| REQ‑W3C‑001 | SCN‑W3C‑mapping‑doc | Inspection |
| REQ‑W3C‑002 | SCN‑W3C‑export | Schema validation |
| REQ‑NAM‑001 | SCN‑NAM‑snake‑case | Unit test |
```

This completes the full specification suite for v0.3.0: Vision, SRS, Architecture, and Test Plan. Would you like me to save these to files in the `docs/superpowers/plans/` directory, or shall we proceed to writing the implementation plan using the writing‑plans skill?
