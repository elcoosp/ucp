# UCP v0.7.0 — Full Specification Suite

## Product Vision & Strategic Alignment — UCP v0.7.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.7.0‑vision‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Vision Statement

> **UCP becomes the universal bridge between design systems and AI agents.**
> v0.7.0 makes every extracted component library immediately consumable by AI agents via DESIGN.md, MCP Apps, A2UI, and AG-UI — while expanding extraction coverage to Svelte 5 and Vue 4, generating framework‑agnostic Web Components, and aligning with Open UI standards.

### 2. Elevator Pitch

> **For** design‑system teams and AI‑application developers
> **who need** their component libraries to be natively understood by AI agents and framework‑agnostic tools,
> **our product** is a CLI pipeline that extracts, unifies, and exports UI component metadata
> **that provides** DESIGN.md files (AI‑native design spec), MCP Apps dashboards (interactive in‑conversation UI), A2UI/AG‑UI catalogs, Web Components generation, and extraction for Svelte 5 and Vue 4.
> **Unlike** manual documentation or framework‑specific tools,
> **our solution** makes any component library instantly AI‑readable, cross‑framework, and aligned with emerging Open UI standards.

### 3. Problem Statement

Google just open‑sourced DESIGN.md — an AI‑native design specification format consumed by coding agents. Anthropic released MCP Apps — interactive dashboards embedded directly in AI conversations. The web platform is standardising around Web Components (58% adoption). Meanwhile, Svelte 5 (runes‑first) and Vue 4 (Composition API) represent two of the largest non‑React ecosystems, with no unified extraction tool spanning them all. UCP v0.6.0 closed framework parity for generators. v0.7.0 extends that parity to the AI‑native ecosystem and the broader web platform.

### 4. Business Objectives

| ID | Objective | Key Results |
|----|-----------|-------------|
| G‑01 | Make any component library AI‑readable | DESIGN.md export from CAM; MCP Apps dashboard serving component intelligence inside AI conversations |
| G‑02 | Achieve framework‑agnostic output | Web Components code generator (Lit‑based), usable in React/Vue/Svelte/Angular/vanilla HTML |
| G‑03 | Expand extraction coverage | Svelte 5 (runes) and Vue 4 (Composition API) extractors |
| G‑04 | Align with W3C standards | Open UI formal alignment; CAM → Open UI spec mapping |

### 5. Goals and Non‑Goals

**Goals (v0.7.0):**
- DESIGN.md export (`ucp export --target design-md`)
- MCP Apps dashboard (interactive in‑conversation component browser)
- Web Components code generator (`--target web-components`)
- Svelte 5 extractor (runes‑based components)
- Vue 4 extractor (Composition API, `<script setup>`)
- Open UI formal alignment (anatomy, states, behaviours mapping)

**Non‑Goals:**
- No bidirectional DESIGN.md → CAM (deferred to v0.8.0)
- No shadcn MCP server compatibility (deferred)
- No additional Rust framework generators (Dioxus, Leptos, GPUI already have parity)

---

## Software Requirements Specification — UCP v0.7.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | SRS v0.7.0 |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Introduction

This SRS defines v0.7.0 capabilities: DESIGN.md export, MCP Apps dashboard, Web Components generator, Svelte 5 extractor, Vue 4 extractor, and Open UI alignment.

### 2. Functional Requirements

#### 2.1 DESIGN.md Export

**REQ‑DSN‑001 – Generate DESIGN.md from CAM**
> **When** the user runs `ucp export --target design-md`, **the system** shall produce a `DESIGN.md` file containing YAML front matter (colors, typography, spacing tokens) and natural‑language design rules derived from CAM metadata.

*Acceptance criteria:*
- YAML front matter includes `colors`, `typography`, `spacing` extracted from DTCG tokens or CAM.
- Free‑text sections describe component usage, variants, and state behaviours.
- Output passes DESIGN.md schema validation (when available).

**REQ‑DSN‑002 – Per‑component design documentation**
> **The DESIGN.md export** shall include a section per component describing its purpose, props (typed), variants, state machine, and dependencies.

#### 2.2 MCP Apps Dashboard

**REQ‑MCP‑010 – Serve dashboard as MCP App**
> **When** the user runs `ucp mcp --spec <spec.json>`, **the MCP server** shall expose a `dashboard` tool that returns an interactive HTML page (MCP App) rendering the component browser directly in the AI conversation.

*Acceptance criteria:*
- The response is valid MCP App HTML.
- The dashboard includes: component list, search, detail pages, conflict viewer, state machine diagrams, registry preview.
- All interactions work inside Claude/Cursor without external network requests.

**REQ‑MCP‑011 – Component API lookup via MCP**
> **The MCP server** shall expose `get_component` and `list_components` tools that return structured JSON conforming to shadcn/skills expectations for AI agent consumption.

#### 2.3 Web Components Code Generator

**REQ‑WC‑001 – Generate Lit‑based Web Components**
> **When** given a `PackageManifest` with `--target web-components`, **the system** shall produce a Lit‑based project with one `.js` file per component using `LitElement`, reactive properties, and scoped styles.

*Acceptance criteria:*
- Each component is a `customElements.define('ucp-{name}', ...)` call.
- Props are declared as `@property()` reactive properties with types.
- Events are emitted via `CustomEvent`.
- The generated project loads in any modern browser without build tools (or with a minimal build setup).

**REQ‑WC‑002 – Prop mapping for Web Components**
> **The Web Components generator** shall map CAM types:
> - `ControlFlag` → `{ type: Boolean }`
> - `StaticValue(Any)` → `{ type: String }`
> - `AsyncEventHandler` → `this.dispatchEvent(new CustomEvent('...'))`
> - `Renderable` → `<slot></slot>`
> - `SpreadAttributes` → attribute passthrough or `...rest` pattern

#### 2.4 Svelte 5 Extractor

**REQ‑SVE‑001 – Extract Svelte 5 runes components**
> **The Svelte extractor** shall parse `.svelte` files and extract components defined with `<script lang="ts">`, `let { prop1, prop2 } = $props()`, and `$state()` runes.

*Acceptance criteria:*
- Props are extracted from `$props()` destructuring.
- Reactive state is detected via `$state()` and `$derived()`.
- Events are detected from `onclick` and other `on:` handlers or callback props.

#### 2.5 Vue 4 Extractor

**REQ‑VUE‑001 – Extract Vue 4 Composition API components**
> **The Vue extractor** shall parse `.vue` SFC files and extract components defined with `<script setup lang="ts">` and `defineProps<{ ... }>()`.

*Acceptance criteria:*
- Props are extracted from `defineProps<T>()` generic type.
- Emits are extracted from `defineEmits<T>()` generic type.
- `<slot>` usage is detected and mapped to `ExtractedPart`.

#### 2.6 Open UI Alignment

**REQ‑OUI‑001 – Map CAM to Open UI spec**
> **The system** shall provide a documented mapping from CAM fields to Open UI's component spec template (anatomy, states, behaviours, properties).

*Acceptance criteria:*
- `ExtractedPart` → Open UI anatomy.
- `StateMachine` → Open UI states.
- `CanonicalAbstractEvent` → Open UI behaviours.
- Mapping document exists in `docs/open-ui-mapping.md`.

**REQ‑OUI‑002 – Open UI spec export**
> **When** the user runs `ucp export --target open-ui`, **the system** shall produce Open UI‑compliant component specification JSON.

### 3. Quality Requirements

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑REL‑001 | Generated Web Components work in Chrome, Firefox, Safari | Manual browser test |
| NFR‑REL‑002 | DESIGN.md output passes schema validation | CI validation step |
| NFR‑MNT‑001 | Test coverage ≥ 85% for new modules | Coverage report |
| NFR‑MNT‑002 | Zero clippy warnings | CI lint gate |

### 4. Traceability

| Goal | Capability | SRS Requirements |
|------|------------|------------------|
| G‑01 | DESIGN.md export | REQ‑DSN‑001, REQ‑DSN‑002 |
| G‑01 | MCP Apps dashboard | REQ‑MCP‑010, REQ‑MCP‑011 |
| G‑02 | Web Components generator | REQ‑WC‑001, REQ‑WC‑002 |
| G‑03 | Svelte 5 extractor | REQ‑SVE‑001 |
| G‑03 | Vue 4 extractor | REQ‑VUE‑001 |
| G‑04 | Open UI alignment | REQ‑OUI‑001, REQ‑OUI‑002 |

---

## Architecture & Design Specification — UCP v0.7.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Architecture & Design Specification |
| Version | 0.7.0‑arch‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Context and Scope

v0.7.0 adds AI‑native design export (DESIGN.md), interactive MCP Apps dashboard, Web Components code generation, Svelte 5 and Vue 4 extractors, and Open UI alignment. All new modules are additive.

### 2. Architecturally Significant Requirements

| ID | Requirement | Source |
|----|-------------|--------|
| ASR‑001 | DESIGN.md export from CAM | REQ‑DSN‑001, REQ‑DSN‑002 |
| ASR‑002 | MCP Apps dashboard | REQ‑MCP‑010, REQ‑MCP‑011 |
| ASR‑003 | Web Components generator | REQ‑WC‑001, REQ‑WC‑002 |
| ASR‑004 | Svelte 5 extractor | REQ‑SVE‑001 |
| ASR‑005 | Vue 4 extractor | REQ‑VUE‑001 |
| ASR‑006 | Open UI alignment | REQ‑OUI‑001, REQ‑OUI‑002 |

### 3. System Design

#### 3.1 New Module: `export::design_md`

**Purpose:** Transform CAM into DESIGN.md format (YAML front matter + Markdown rules).

**Design:** Uses `serde_yaml` for front matter generation. Template‑driven: reads component metadata and produces structured documentation. Includes design tokens from DTCG extractor when available.

#### 3.2 Enhanced: `contract::mcp_server` → MCP Apps

**Purpose:** Upgrade the existing minimal MCP server to serve interactive MCP Apps.

**Design:** The `dashboard` tool returns HTML conforming to MCP Apps protocol (embedded iframe content). The `get_component` and `list_components` tools return structured JSON compatible with shadcn/skills patterns.

#### 3.3 New Module: `generate::web_components`

**Purpose:** Generate Lit‑based Web Components from CAM.

**Design:** Mirrors existing generator structure. Uses Lit's `@property()` decorators for reactive props, `CustomEvent` for event dispatching, `<slot>` for children, and scoped CSS via `static styles`.

#### 3.4 New Module: `extract::svelte_ast`

**Purpose:** Parse `.svelte` files and extract Svelte 5 runes components.

**Design:** Uses `swc` or regex‑based parsing for `$props()`, `$state()`, `$derived()`, and event handlers. May use `svelte/compiler` for AST access if available.

#### 3.5 New Module: `extract::vue_ast`

**Purpose:** Parse `.vue` SFC files and extract Vue 4 Composition API components.

**Design:** Parses `<script setup lang="ts">` blocks. Extracts `defineProps<T>()` and `defineEmits<T>()` using TypeScript AST or regex patterns.

#### 3.6 New Module: `export::open_ui`

**Purpose:** Map CAM to Open UI component spec format.

**Design:** Produces JSON conforming to Open UI's component spec template. Maps `ExtractedPart` → anatomy, `StateMachine` → states, `CanonicalAbstractEvent` → behaviours.

### 4. Key Design Decisions (ADRs)

| ADR | Decision | Drivers |
|-----|----------|---------|
| ADR‑022 | DESIGN.md uses `serde_yaml` for front matter | ASR‑001, YAML is standard for AI‑native design specs |
| ADR‑023 | MCP Apps dashboard serves existing dashboard HTML | ASR‑002, reuse v0.4.0 dashboard with MCP‑compliant wrapper |
| ADR‑024 | Web Components use Lit (not vanilla) | ASR‑003, Lit provides reactive props, scoped styles, small bundle |
| ADR‑025 | Svelte 5 extractor uses regex + `svelte/compiler` | ASR‑004, `svelte/compiler` provides reliable AST if available |
| ADR‑026 | Vue 4 extractor uses TypeScript AST parsing | ASR‑005, `defineProps<T>()` has well‑defined TS structure |
| ADR‑027 | Open UI export is JSON‑based | ASR‑006, matches Open UI spec template format |

### 5. Traceability

| ASR | Design Elements | ADRs |
|-----|----------------|------|
| ASR‑001 | `export::design_md` | ADR‑022 |
| ASR‑002 | Enhanced `contract::mcp_server` | ADR‑023 |
| ASR‑003 | `generate::web_components` | ADR‑024 |
| ASR‑004 | `extract::svelte_ast` | ADR‑025 |
| ASR‑005 | `extract::vue_ast` | ADR‑026 |
| ASR‑006 | `export::open_ui` | ADR‑027 |

---

## Behavioral Specification & Test Verification Plan — UCP v0.7.0

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
Feature: DESIGN.md Export
  Scenario: Export a component library as DESIGN.md
    Given a SynthesisOutput with 2 components: Button, Dialog
    And DTCG tokens with colors: { primary: "#ff0000" }
    When `ucp export --target design-md` runs
    Then a `DESIGN.md` file is created containing:
      - YAML front matter with `colors.primary: "#ff0000"`
      - A section "## Components"
      - A sub-section "### Button" with props table
      - A sub-section "### Dialog" with variant documentation

Feature: MCP Apps Dashboard
  Scenario: MCP dashboard returns interactive HTML
    Given an MCP server loaded with a spec containing 3 components
    When the `dashboard` tool is invoked
    Then the response contains valid MCP App HTML
    And the HTML includes a searchable component list
    And clicking a component shows its props table

Feature: Web Components Generator
  Scenario: Generate a Lit button component
    Given a PackageManifest with a Button component
    When `ucp generate --target web-components` runs
    Then a `button.js` file is created containing:
      - `@customElement('ucp-button')`
      - `@property({ type: Boolean }) disabled = false`
      - `@property({ type: String }) label = ''`
      - `<slot></slot>` for children
    And the component dispatches CustomEvent on click

Feature: Svelte 5 Extractor
  Scenario: Extract a Svelte 5 button component
    Given a `.svelte` file with:
      ```
      <script lang="ts">
        let { disabled = false, label }: { disabled: boolean; label: string } = $props();
      </script>
      <button {disabled}>{label}</button>
      ```
    When the Svelte extractor runs
    Then a RawComponentExtraction is produced with:
      - name = "Button"
      - props[0].name = "disabled", raw_type = "boolean", has_default = true
      - props[1].name = "label", raw_type = "string"

Feature: Vue 4 Extractor
  Scenario: Extract a Vue 4 button component
    Given a `.vue` SFC file with:
      ```
      <script setup lang="ts">
        defineProps<{ disabled?: boolean; label: string }>();
        defineEmits<{ click: [] }>();
      </script>
      <template><button :disabled="disabled" @click="$emit('click')">{{ label }}</button></template>
      ```
    When the Vue extractor runs
    Then a RawComponentExtraction is produced with:
      - props: disabled (optional, boolean), label (required, string)
      - events: click
```

### 2. Test Strategy

| Layer | Goal | Tools |
|-------|------|-------|
| Unit tests | DESIGN.md generation, MCP Apps protocol, Web Components generation, Svelte/Vue extraction | Rust #[test] |
| Integration tests | End‑to‑end: extract → export DESIGN.md, generate WC, export Open UI | tempfile, tokio |
| Manual verification | MCP Apps dashboard renders in Claude/Cursor; Web Components work in browsers | Browser, MCP Inspector |
| Regression tests | All v0.6.0 tests continue to pass | cargo nextest |

### 3. Requirements Traceability Matrix

| Requirement | Test Case | Verification |
|-------------|-----------|--------------|
| REQ‑DSN‑001 | design‑md‑full‑export | Test + schema validation |
| REQ‑MCP‑010 | mcp‑dashboard‑html | Manual in Claude/Cursor |
| REQ‑WC‑001 | web‑components‑button | Test + browser |
| REQ‑SVE‑001 | svelte‑extract‑button | Test |
| REQ‑VUE‑001 | vue‑extract‑button | Test |
| REQ‑OUI‑001 | open‑ui‑mapping‑doc | Inspection |
| REQ‑OUI‑002 | open‑ui‑export | Test + schema validation |

---

This completes the specification suite for v0.7.0. Ready for implementation planning whenever you are.
