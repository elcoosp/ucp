# Architecture & Design Specification

| Field | Value |
|-------|-------|
| Project | Shadcn UI API Alignment for Quoin UCP |
| Document | Architecture & Design Specification |
| Version | 0.1 (Draft) |
| Date | 2026-04-21 |
| Author | System Architect (assisted by AI) |
| Status | Draft — Pending Review |

## 1. Context and Scope

This document describes the architectural approach for aligning the existing Leptos‑shadcn/ui and Dioxus‑shadcn/ui libraries under a common API specification, without rewriting them.

**Goals:**
- Define a machine‑readable common spec that serves as the source of truth for component APIs.
- Provide a conformance test suite that validates the existing libraries against the spec.
- Enable Quoin's UCP to use a unified mapping to both libraries.

**Non‑goals:**
- Changing the internal architecture of the libraries.
- Creating a new shared codebase.

## 2. Architecturally Significant Requirements (ASRs)

| ID | Requirement | Impact |
|----|-------------|--------|
| **ASR‑001** | Common spec must be machine‑readable. | JSON Schema chosen. |
| **ASR‑002** | Conformance tests must work against both libraries without modification. | WebDriver/DOM‑level testing. |
| **ASR‑003** | Libraries must be able to evolve independently while tracking the spec. | Spec versioned; libraries reference spec version. |

## 3. System Overview

 ```
┌─────────────────────────┐
│  Common API Spec (JSON) │
└───────────┬─────────────┘
            │ (references)
            ▼
┌─────────────────────────┐      ┌─────────────────────────┐
│ Leptos‑shadcn/ui        │      │ Dioxus‑shadcn/ui        │
│ (existing, updated)     │      │ (existing, updated)     │
└───────────┬─────────────┘      └───────────┬─────────────┘
            │                                │
            └──────────────┬─────────────────┘
                           │ (test against)
                           ▼
                  ┌─────────────────┐
                  │ Conformance     │
                  │ Test Suite      │
                  └─────────────────┘
 ```

## 4. Common API Specification Repository

**Structure:**
 ```
shadcn-rs-spec/
├── schemas/
│   ├── button.json
│   ├── input.json
│   └── ...
├── docs/
│   └── (generated markdown)
└── tests/
    └── conformance/
        ├── scenarios/       # YAML test scenarios
        └── runner/          # Rust WebDriver runner
 ```

**Versioning:** The spec uses semantic versioning. Libraries declare which spec version they conform to (e.g., in `Cargo.toml` metadata or documentation).

## 5. Library Updates (Incremental)

Both libraries continue to live in their existing repositories. Changes are made via normal PR processes:

- **Renaming props/events** to match the spec (with deprecation cycle if breaking).
- **Adding missing components** following the spec.
- **Running conformance tests** in CI to verify compliance.

**Deprecation Strategy:**
1. Introduce new API (matching spec) alongside old API.
2. Mark old API as `#[deprecated]`.
3. Remove old API in next major version.

## 6. Conformance Test Suite

Uses `fantoccini` (Rust WebDriver) to drive a headless browser. For each component:
1. Mount the component with a predefined set of props (using a minimal test harness for each library).
2. Query the DOM for expected elements, classes, and ARIA attributes.
3. Simulate interactions and verify callbacks.
4. Compare against expected DOM snapshot.

Test scenarios are defined in YAML, framework‑agnostic.

## 7. Quoin UCP Integration

Quoin's `quoin_render!` macro uses the common spec as its source of truth. The macro's emitters generate code that calls the appropriate library functions. For example:

**Leptos emitter:**
 ```rust
leptos_shadcn_button::Button(
    variant: MaybeProp::from(button_variant),
    size: MaybeProp::from(button_size),
    on_click: Some(Callback::new(move |_| ...)),
    children: ...
)
 ```

**Dioxus emitter:**
 ```rust
dioxus_shadcn_button::Button {
    variant: Some(button_variant),
    size: Some(button_size),
    onclick: Some(EventHandler::new(move |_| ...)),
    children: ...
}
 ```

Because prop names and event names are identical, the emitter logic is unified.

## 8. Architecture Decision Records (ADRs)

### ADR‑001: JSON Schema for Common Spec
**Decision:** Use JSON Schema to define component APIs.
**Rationale:** Machine‑readable, supports validation and documentation generation.

### ADR‑002: WebDriver‑Based Conformance Testing
**Decision:** Use `fantoccini` to test actual DOM output.
**Rationale:** Framework‑agnostic; validates real browser behavior.

### ADR‑003: Incremental Alignment with Deprecation
**Decision:** Allow breaking changes only with a deprecation cycle.
**Rationale:** Protects existing users while enabling convergence.

## 9. Traceability

- **ASR‑001** → ADR‑001 (JSON Schema)
- **ASR‑002** → ADR‑002 (WebDriver)
- **ASR‑003** → ADR‑003 (Deprecation cycle)
