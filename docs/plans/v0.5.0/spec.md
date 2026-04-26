We'll write the full specification suite for v0.5.0, focusing on shadcn CLI v4 compliance, AI protocol catalog export, design token integration, and W3C community engagement — without adding new code generators for additional frameworks.

---

# Product Vision & Strategic Alignment – UCP v0.5.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.5.0‑vision‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

## 1. Vision Statement

> **UCP becomes the standard bridge between component libraries and AI agents.**
> v0.5.0 aligns UCP with the AI-native UI revolution — generating A2UI catalogs for Google's agent framework, AG-UI event streams for frontend-backend agent communication, fully shadcn CLI v4‑compliant registries with presets and design tokens, and production‑grade DTCG token integration. UCP joins the W3C UI Specification Schema Community Group as a reference implementor.

## 2. Elevator Pitch

> **For** AI developers and component‑library maintainers
> **who need** to make their UI components consumable by AI coding agents and the shadcn ecosystem,
> **our product** is a CLI pipeline that extracts, unifies, and exports UI component metadata
> **that provides** A2UI‑compatible catalogs, AG‑UI event schemas, shadcn CLI v4 registries with presets, DTCG design tokens, and W3C‑aligned component descriptions.
> **Unlike** manual specification or framework‑specific tools,
> **our solution** preserves concrete types, detects conflicts, generates framework‑specific code (Dioxus + Leptos), and now directly feeds AI agent frameworks and the latest shadcn registry ecosystem.

## 3. Problem Statement

The AI‑native UI landscape has exploded. Google's A2UI framework requires structured component catalogs. AG‑UI requires typed event streams. shadcn CLI v4 fundamentally reworked the registry format. Without support for these, UCP's output is increasingly incompatible with the tools developers actually use. Additionally, the W3C UI Spec Community Group remains open to reference implementations — a window for first‑mover advantage.

## 4. Business Objectives

| ID | Objective | Key Results |
|----|-----------|-------------|
| G‑01 | Achieve full shadcn CLI v4 registry compliance | `registry:base` and `registry:font` support; preset generation; all v4 fields populated |
| G‑02 | Become the standard A2UI catalog generator | Produce valid A2UI component catalogs from any extracted source |
| G‑03 | Integrate AG‑UI event schemas | Export AG‑UI‑compatible event definitions alongside CAM |
| G‑04 | Production‑grade DTCG design token support | Integrate `mezzanine`; produce valid DTCG `tokens.json` from extracted source |
| G‑05 | Join and contribute to W3C UI Spec Community Group | Membership confirmed; reference implementation submitted |

## 5. Key Differentiators

- **AI‑first:** No other tool generates A2UI catalogs and AG‑UI schemas from existing component libraries.
- **Registry v4 native:** UCP will be one of the first tools to fully support shadcn CLI v4 presets and base/font types.
- **DTCG via mezzanine:** Production‑grade Rust token handling, not a proof‑of‑concept.
- **W3C participation:** UCP shapes the emerging standard rather than following it.

## 6. Goals and Non‑Goals

### Goals (v0.5.0)

- [ ] shadcn CLI v4 registry: `registry:base`, `registry:font`, presets, monorepo paths
- [ ] A2UI catalog export (`--target a2ui`)
- [ ] AG‑UI event schema export (`--target ag-ui`)
- [ ] Full DTCG design token integration (via `mezzanine`)
- [ ] W3C Community Group membership and reference implementation submission

### Non‑Goals

- **NG‑01:** No new code generators for additional frameworks (Vue, Svelte, Solid, etc.)
- **NG‑02:** No real‑time collaboration or cloud sync
- **NG‑03:** No visual design tool or Figma plugin
- **NG‑04:** No performance optimization beyond current baseline

---

# Software Requirements Specification – UCP v0.5.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | SRS v0.5.0 |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

## 1. Introduction

This SRS defines v0.5.0 capabilities: shadcn CLI v4 registry compliance, A2UI catalog export, AG‑UI event schema export, DTCG design token integration, and W3C Community Group participation. No new framework generators are added.

## 2. Functional Requirements

### 2.1 shadcn CLI v4 Registry Compliance

**REQ‑REG‑020 – `registry:base` support**
> **The registry exporter** shall support the `registry:base` item type, allowing distribution of an entire design system as a single payload including components, CSS, fonts, and configuration.

*Acceptance:* Generated `registry.json` contains at least one item of type `registry:base` with all associated files bundled.

**REQ‑REG‑021 – `registry:font` support**
> **The registry exporter** shall support the `registry:font` item type for distributing font files.

*Acceptance:* Font items include appropriate `files[].target` paths and font declarations.

**REQ‑REG‑022 – Preset generation**
> **When** design tokens are extracted from source, **the registry exporter** shall generate a `--preset` compatible configuration encoding colors, theme, icons, fonts, and radius.

*Acceptance:* Generated preset is accepted by `shadcn init --preset <preset‑name>`.

**REQ‑REG‑023 – Monorepo path support**
> **The registry exporter** shall emit `files[].target` paths compatible with monorepo structures when a workspace root is specified.

### 2.2 A2UI Catalog Export

**REQ‑A2UI‑001 – Generate A2UI component catalog**
> **When** the user runs `ucp export --target a2ui`, **the system** shall produce an A2UI‑compatible component catalog JSON file.

*Acceptance:* The catalog is loadable by A2UI renderers (React, Flutter, Lit, Angular) without errors.

**REQ‑A2UI‑002 – Map CAM to A2UI catalog format**
> **The A2UI exporter** shall map CAM fields to A2UI fields: `id`, `name`, `description`, `props` (with types), `events`, `variants`, `state_machine`.

### 2.3 AG‑UI Event Schema Export

**REQ‑AGUI‑001 – Generate AG‑UI event definitions**
> **When** the user runs `ucp export --target ag-ui`, **the system** shall produce an AG‑UI‑compatible event schema JSON file.

*Acceptance:* Schema defines all component events with correct AG‑UI event types.

**REQ‑AGUI‑002 – Map CAM events to AG‑UI event types**
> **The AG‑UI exporter** shall map `CanonicalAbstractEvent` to AG‑UI's typed event format (e.g., `component.click`, `component.change`).

### 2.4 DTCG Design Token Integration

**REQ‑TOK‑001 – Replace proof‑of‑concept token extractor with `mezzanine`**
> **The token extractor** shall use the `mezzanine` crate for fully DTCG‑compliant token parsing and serialization.

*Acceptance:* Output `tokens.json` passes DTCG schema validation.

**REQ‑TOK‑002 – Round‑trip token extraction**
> **The pipeline** shall extract design tokens from source CSS/Tailwind config, store them in CAM, and export them as valid DTCG `tokens.json`.

**REQ‑TOK‑003 – Feed tokens into registry cssVars**
> **When** tokens are available, **the registry exporter** shall populate `cssVars` with `light`/`dark` variants from DTCG tokens.

### 2.5 W3C Community Group Participation

**REQ‑W3C‑020 – Join Community Group**
> **The UCP project** shall join the W3C UI Specification Schema Community Group.  
*Acceptance:* Membership confirmed via public‑uispec mailing list.

**REQ‑W3C‑021 – Submit reference implementation**
> **The UCP project** shall submit the v0.5.0 W3C export as a reference implementation to the Community Group for feedback.

## 3. Quality and Non‑Functional Requirements

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑PERF‑001 | A2UI catalog export for 200 components < 2s | `just test‑perf` measurement |
| NFR‑REL‑001 | Generated preset passes `shadcn init --preset` | CI step with shadcn CLI v4 |
| NFR‑REL‑002 | DTCG output passes schema validation | CI validation step |
| NFR‑MNT‑001 | Test coverage ≥ 85% for new modules | Coverage report |
| NFR‑MNT‑002 | Zero clippy warnings | CI lint gate |

## 4. Traceability

| Goal | Capability | SRS Requirements |
|------|------------|------------------|
| G‑01 | shadcn CLI v4 registry | REQ‑REG‑020 … REQ‑REG‑023 |
| G‑02 | A2UI catalog export | REQ‑A2UI‑001, REQ‑A2UI‑002 |
| G‑03 | AG‑UI event schema | REQ‑AGUI‑001, REQ‑AGUI‑002 |
| G‑04 | DTCG token integration | REQ‑TOK‑001 … REQ‑TOK‑003 |
| G‑05 | W3C participation | REQ‑W3C‑020, REQ‑W3C‑021 |

---

# Architecture & Design Specification – UCP v0.5.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Architecture & Design Specification |
| Version | 0.5.0‑arch‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

## 1. Context and Scope

v0.5.0 adds AI‑protocol exports (A2UI, AG‑UI), shadcn CLI v4 registry enhancements, production‑grade DTCG token support, and W3C participation. No new framework generators. All new modules are additive.

## 2. Architecturally Significant Requirements

| ID | Requirement | Source |
|----|-------------|--------|
| ASR‑001 | shadcn CLI v4 compliance | REQ‑REG‑020 … REQ‑REG‑023 |
| ASR‑002 | A2UI catalog export | REQ‑A2UI‑001, REQ‑A2UI‑002 |
| ASR‑003 | AG‑UI event schema export | REQ‑AGUI‑001, REQ‑AGUI‑002 |
| ASR‑004 | DTCG token integration | REQ‑TOK‑001 … REQ‑TOK‑003 |

## 3. System Design

### 3.1 New Module: `export::a2ui`

**Purpose:** Transform CAM components into A2UI catalog format.

**Key structure:**
```rust
struct A2uiCatalog {
    schema: String,       // "https://a2ui.dev/schema/v0.9"
    library: String,      // library name
    version: String,
    components: Vec<A2uiComponent>,
}

struct A2uiComponent {
    id: String,
    name: String,
    description: String,
    props: Vec<A2uiProp>,
    events: Vec<A2uiEvent>,
    variants: Vec<A2uiVariant>,
    state_machine: Option<String>, // SMDL
}
```

**Algorithm:** Map each `CanonicalAbstractComponent` to `A2uiComponent`, using concrete types for prop types and converting events to A2UI format.

### 3.2 New Module: `export::ag_ui`

**Purpose:** Export AG‑UI event schemas from CAM events.

**Key structure:**
```rust
struct AgUiSchema {
    protocol: String,     // "ag-ui/v1"
    events: Vec<AgUiEvent>,
}

struct AgUiEvent {
    component: String,
    event: String,
    event_type: String,   // "component.click", "component.change"
    payload: Option<String>,
}
```

### 3.3 Enhanced: `generate::registry` — CLI v4

Add support for `RegistryIndexBase` (bundles all components + config into a single item), font items, preset generation from extracted DTCG tokens, and monorepo paths.

### 3.4 Replaced: `extract::tokens` — DTCG via `mezzanine`

Replace the proof‑of‑concept string parser with `mezzanine::DesignTokens`, parsing full DTCG 2025.10 format. Store parsed tokens in CAM (new optional field `design_tokens`). Export as DTCG `tokens.json`.

### 3.5 CLI Enhancements

- `ucp export --target a2ui` — A2UI catalog export
- `ucp export --target ag-ui` — AG‑UI event schema export
- `ucp export --target dtcg` — DTCG token export
- `ucp registry build` — build a complete shadcn registry (replaces `ucp generate --target shadcn-registry`)

## 4. Key Design Decisions (ADRs)

| ADR | Decision | Drivers |
|-----|----------|---------|
| ADR‑014 | Use `mezzanine` for DTCG token handling | ASR‑004, production‑grade compliance |
| ADR‑015 | A2UI and AG‑UI as export targets under `ucp export` | ASR‑002, ASR‑003, unified export interface |
| ADR‑016 | Registry v4 as an evolution of existing registry module | ASR‑001, backward compatibility with v0.4.0 |
| ADR‑017 | Presets generated from DTCG tokens | ASR‑004 → ASR‑001 |

## 5. Traceability

| ASR | Design Elements | ADRs |
|-----|----------------|------|
| ASR‑001 | `generate::registry` v4, `RegistryIndexBase`, presets | ADR‑016 |
| ASR‑002 | `export::a2ui` module, `ucp export --target a2ui` | ADR‑015 |
| ASR‑003 | `export::ag_ui` module, `ucp export --target ag-ui` | ADR‑015 |
| ASR‑004 | `extract::tokens` via `mezzanine`, `ucp export --target dtcg` | ADR‑014, ADR‑017 |

---

# Behavioral Specification & Test Verification Plan – UCP v0.5.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Behavioral Spec & Test Verification |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

## 1. Behavioral Specifications

### Feature: A2UI Catalog Export

```gherkin
Scenario: Export a button component as A2UI catalog
  Given a SynthesisOutput with a Button component
    - props: disabled (ControlFlag, bool), label (StaticValue, String)
    - events: click (AsyncEventHandler)
    - variants: "variant" = [Default, Destructive]
  When `ucp export --target a2ui` runs
  Then an A2UI catalog JSON file is created
  And it contains:
    - schema: "https://a2ui.dev/schema/v0.9"
    - components[0].id = "...:Button"
    - components[0].props[0].name = "disabled", type = "bool"
    - components[0].variants[0].values = ["Default", "Destructive"]
```

### Feature: AG‑UI Event Schema Export

```gherkin
Scenario: Export component events as AG‑UI schema
  Given a SynthesisOutput with a Button (click event) and a Dialog (open, close events)
  When `ucp export --target ag-ui` runs
  Then an AG‑UI event schema JSON file is created
  And it contains:
    - events[0] = { component: "Button", event: "click", event_type: "component.click" }
    - events[1] = { component: "Dialog", event: "open", event_type: "component.open" }
```

### Feature: DTCG Token Export

```gherkin
Scenario: Extract and export design tokens as DTCG
  Given a source file with CSS custom properties:
    --primary: oklch(0.645 0.246 16.439);
    --radius: 0.65rem;
  When the pipeline extracts tokens and `ucp export --target dtcg` runs
  Then a `tokens.json` file is created conforming to DTCG 2025.10
  And it contains a color token "primary" with value in oklch
```

## 2. Test Strategy

| Layer | Goal | Tools |
|-------|------|-------|
| Unit tests | A2UI mapping, AG‑UI mapping, DTCG parsing, registry v4 types | Rust #[test] |
| Integration tests | Full pipeline: extract → export A2UI, AG‑UI, DTCG | tempfile, tokio |
| Schema validation | A2UI catalog format, DTCG schema | Schema validator |
| Regression tests | All v0.4.0 tests continue to pass | cargo nextest |

## 3. Requirements Traceability Matrix

| Requirement ID | Test Case | Verification |
|---------------|-----------|--------------|
| REQ‑REG‑020 | registry‑base‑test | Test + shadcn build |
| REQ‑REG‑022 | preset‑generation‑test | Test + shadcn init |
| REQ‑A2UI‑001 | a2ui‑export‑test | Schema validation |
| REQ‑AGUI‑001 | ag-ui-export-test | Schema validation |
| REQ‑TOK‑001 | dtcg‑export‑test | DTCG validator |
| REQ‑W3C‑020 | Community Group membership | Manual confirmation |
```

---

This completes the full specification suite for v0.5.0: Vision, SRS, Architecture, and Test Plan. Would you like me to now write the implementation plan or save these documents to files?
