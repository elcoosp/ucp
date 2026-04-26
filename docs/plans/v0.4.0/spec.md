# UCP v0.4.0 — Full Specification Suite

---

## Product Vision & Strategic Alignment – UCP v0.4.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.4.0‑vision‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Vision Statement

> **UCP becomes the universal bridge between component libraries and the tools that consume them — with full W3C alignment, shadcn registry compliance, and an interactive visual dashboard.**
> v0.4.0 transforms UCP from a CLI extraction/generation tool into a **component intelligence platform** that produces standards‑compliant, browsable, and AI‑readable component specifications.

### 2. Elevator Pitch (Moore Template)

> **For** component‑library maintainers, framework authors, and design‑system teams
> **who need** to distribute, browse, and validate their UI components across the shadcn ecosystem and emerging web standards,
> **our product** is a CLI pipeline that extracts, unifies, and republishes UI components
> **that provides** W3C‑aligned component descriptions, fully shadcn CLI 3.0‑compatible registry exports, and an interactive dashboard for visual browsing, conflict inspection, and state‑machine exploration.
> **Unlike** manual porting, single‑framework tools, or static spec documents,
> **our solution** preserves concrete types, detects cross‑framework conflicts, generates code for multiple frameworks, exports to emerging web standards, and visualises everything in a shareable, self‑contained dashboard.

### 3. Problem Statement & Business Context

#### 3.1 The Problem

v0.3.0 proved that UCP can extract components, generate code, and produce registry files. However, the output is still "data files" — not integrated into a visual workflow. Additionally:

- **No formal standards alignment:** The W3C UI Specification Schema Community Group is actively defining a vendor‑neutral component description format. UCP’s output is close but not yet conformant, missing first‑mover advantage.
- **Registry compliance gaps:** shadcn CLI 3.0 introduced namespaced registries, private registries, and an updated schema. v0.3.0 registry output is incomplete (missing `title`, `description`, `$schema`, `dependencies`, proper `registry.json` wrapping).
- **No visual interface:** Reviewing extracted specs, conflicts, and state machines requires reading raw JSON or a static HTML page. Teams want a browsable, searchable dashboard that makes component intelligence accessible to non‑developers.
- **Missing AI‑readable contract format:** LLMs need structured, typed component descriptions for code generation. The CAM is perfect for this but lacks a dedicated export format optimised for AI consumption.

#### 3.2 Why Now?

- **W3C window is open:** The UI Spec Community Group has no published draft yet and only 12 members. UCP can join now, shape the standard, and ship the first working implementation.
- **shadcn CLI 3.0 is the new baseline:** Private registries, MCP integration, and cross‑registry dependencies mean the registry format is the de‑facto distribution channel for UI components. UCP must be fully compliant to be useful.
- **AI‑native UI development is accelerating:** Tools like `@alvandal/ai-ui` and `contextify‑ai` show that AI models need structured component contracts. CAM is perfectly positioned to be that contract.
- **Competitive landscape is still unoccupied:** No tool currently does all of: multi‑framework extraction, W3C‑aligned export, shadcn registry compliance, and visual browsing. This is UCP’s unique position.

#### 3.3 Business Objectives

| ID | Objective | Key Results |
|----|-----------|-------------|
| G‑01 | Achieve full W3C UI Specification Schema conformance | Join Community Group; `ucp‑spec.w3c.json` passes official validation |
| G‑02 | Achieve full shadcn CLI 3.0+ registry compliance | `shadcn build` succeeds on UCP‑generated registries; `shadcn add @ucp/button` works end‑to‑end |
| G‑03 | Ship an interactive visual dashboard | `ucp dashboard` produces a self‑contained browsable catalog with component props, state machines, conflicts, and registry preview |
| G‑04 | Define AI‑contract format | `ucp‑contract.json` format optimised for LLM structured output; MCP server integration |

### 4. Target Users

| User | Need | v0.4.0 Solution |
|------|------|----------------|
| Component‑library maintainer | Publish to shadcn registry with full compliance | `ucp generate --target shadcn-registry` produces validated, namespaced, CLI 3.0‑ready output |
| Design‑system architect | Share component specs with stakeholders | `ucp dashboard` produces a browser‑ready catalog |
| Framework author | Validate against emerging W3C standard | `ucp bootstrap --w3c` produces conformant W3C JSON |
| AI developer | Feed structured component data to LLMs | `ucp contract` exports an AI‑optimised JSON contract |

### 5. Goals and Non‑Goals

#### Goals (v0.4.0)

- [ ] Full W3C UI Spec Schema conformance (join Community Group, ship reference implementation)
- [ ] Full shadcn CLI 3.0 registry compliance (updated schema, namespaces, dependencies)
- [ ] Interactive visual dashboard (component browser, conflict viewer, state machine diagrams, registry preview)
- [ ] AI‑contract export format (`ucp‑contract.json`)
- [ ] MCP server integration for AI agent access to CAM data
- [ ] Design token extraction and integration (extract colors, spacing, typography from source and include in CAM/W3C/registry output)

#### Non‑Goals

- **NG‑01:** No real‑time collaboration or cloud sync (filesystem‑based tool).
- **NG‑02:** No framework‑specific UI rendering in the dashboard (it shows specs, not live components).
- **NG‑03:** No visual design tool or Figma integration (design tokens are extracted, not edited).
- **NG‑04:** No code generation for non‑Rust frameworks beyond what already exists (Dioxus + Leptos).

### 6. Success Metrics

| Metric | Target |
|--------|--------|
| W3C spec passes community group validation | 100% |
| `shadcn build` on UCP‑generated registry succeeds | No errors |
| Dashboard loads in under 2 seconds for a spec with 200 components | P95 < 2s |
| AI contract JSON is parseable by standard LLM tool‑use frameworks | Valid JSON Schema |
| Existing tests continue to pass | 100% |

### 7. Out of Scope for v0.4.0

- Real‑time collaboration features.
- Cloud‑hosted registry service.
- Visual design tooling.
- Non‑Rust framework code generators beyond Dioxus + Leptos.

---

## Software Requirements Specification – UCP v0.4.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | SRS v0.4.0 |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Introduction and Scope

This SRS defines the new capabilities for v0.4.0 of UCP. Building on v0.3.0’s registry export, Leptos generator, and AST‑based context detection, v0.4.0 delivers full W3C UI Specification Schema compliance, complete shadcn CLI 3.0+ registry support, an interactive visual dashboard, and an AI‑contract export format.

### 2. Functional Requirements

#### 2.1 W3C UI Specification Schema — Full Compliance

**REQ‑W3C‑010 – Join W3C Community Group**
> **The UCP project** shall join the W3C UI Specification Schema Community Group.  
*Acceptance:* Mailing list membership confirmed; group participation visible.

**REQ‑W3C‑011 – W3C Schema Alignment**
> **When** exporting with `--w3c`, **the system** shall produce a JSON document that conforms to the latest published W3C UI Specification Schema draft.  
*Acceptance:* `ucp‑spec.w3c.json` passes official W3C validator (when available) or community‑reviewed schema check.

**REQ‑W3C‑012 – Full Field Coverage**
> **The W3C export** shall include all fields defined by the W3C UI Spec Schema: `id`, `name`, `category`, `description`, `anatomy`, `states`, `variants`, `properties`, `events`, `accessibility`, `responsive`, `internationalisation`, `dependencies`, `designTokens`.  
*Acceptance:* Mapping document shows 100% coverage of required fields; optional fields are populated where CAM data exists.

**REQ‑W3C‑013 – Anatomy from CAM Parts**
> **When** a CAM component has `extracted_parts`, **the W3C export** shall map each part to the `anatomy` section with name, description, and selectability.

**REQ‑W3C‑014 – States from CAM State Machines**
> **When** a CAM component has an `extracted_state_machine`, **the W3C export** shall map it to the `states` section, including initial state, transitions, and side effects.

**REQ‑W3C‑015 – Variants from Concrete Types**
> **When** a CAM prop has a concrete type matching an enum pattern (e.g., `enum: Default, Destructive`), **the W3C export** shall create a `variants` entry with the available values.

**REQ‑W3C‑016 – Design Tokens Integration**
> **The pipeline** shall extract design tokens (colors, spacing, typography) from source code when available and include them in the W3C export under `designTokens`, referencing DTCG‑format token files.

#### 2.2 shadcn CLI 3.0+ Registry Compliance

**REQ‑REG‑010 – Full Registry Schema Compliance**
> **The registry exporter** shall produce `registry.json` and `registry‑item.json` files that conform to the shadcn CLI 3.0+ schema, including all required fields (`$schema`, `name`, `type`, `files`) and optional fields where data is available (`title`, `description`, `dependencies`, `devDependencies`, `registryDependencies`, `cssVars`, `meta`).

**REQ‑REG‑011 – Registry Index Wrapper**
> **The `registry.json` index** shall be an object containing `$schema`, `name`, `homepage`, and `items` (not a bare array).

**REQ‑REG‑012 – Namespaced Registry Support**
> **When** a manifest specifies a namespace (e.g., `@acme`), **the registry exporter** shall emit `registryDependencies` using the namespaced format (`@acme/button`).

**REQ‑REG‑013 – CSS Variables Extraction**
> **When** source components define CSS custom properties or theme variables, **the registry exporter** shall populate the `cssVars` field with `light` and `dark` variants.

**REQ‑REG‑014 – File Target Paths**
> **When** a registry item is of type `registry:page` or `registry:file`, **the file entry** shall include a `target` field specifying the installation path.

**REQ‑REG‑015 – npm Dependencies**
> **The registry exporter** shall populate the `dependencies` and `devDependencies` fields based on the framework and manifest metadata (e.g., `dioxus = "0.7"` for Dioxus components).

**REQ‑REG‑016 – Registry Validation**
> **The pipeline** shall include a `ucp validate --target registry` subcommand that checks generated registry files against the official shadcn schema.

#### 2.3 Interactive Visual Dashboard

**REQ‑DASH‑001 – Dashboard Generation**
> **When** the user runs `ucp dashboard --spec <spec.json> --output <dir>`, **the system** shall produce a self‑contained static HTML/CSS/JS dashboard directory that can be opened in any modern browser without a server.

**REQ‑DASH‑002 – Component List View**
> **The dashboard** shall display a searchable, filterable list of all components in the spec. Filters shall include: framework, confidence, prop count, and text search.

**REQ‑DASH‑003 – Component Detail Page**
> **For each component**, the dashboard shall show:
> - Props table: canonical name, abstract type, concrete type, reactivity, default, required/optional.
> - Events table: name, payload.
> - State machine diagram (rendered from SMDL using Mermaid.js or similar).
> - Parts/anatomy view.

**REQ‑DASH‑004 – Conflict View**
> **The dashboard** shall display unresolved conflicts per component and as a global summary, including present‑in / absent‑in attribution.

**REQ‑DASH‑005 – Dependency Graph**
> **The dashboard** shall render an interactive force‑directed graph showing component dependencies (which components reference each other).

**REQ‑DASH‑006 – Registry Preview**
> **When** the spec has components that could be exported as a registry, **the dashboard** shall show a preview of the registry output, including generated source code with syntax highlighting.

**REQ‑DASH‑007 – Export Integration**
> **The dashboard** shall include one‑click download buttons for:
> - Generated Dioxus project (as .zip)
> - Generated Leptos project (as .zip)
> - Registry JSON files
> - W3C‑compliant spec
> - AI contract JSON

**REQ‑DASH‑008 – Offline Operation**
> **The dashboard** shall be fully functional offline — no CDN dependencies, all assets self‑contained.

**REQ‑DASH‑009 – Dashboard CLI Integration**
> **The `ucp dashboard` command** shall also be accessible as a post‑bootstrap step: after `ucp bootstrap`, the dashboard is optionally generated automatically.

#### 2.4 AI‑Contract Export Format

**REQ‑AI‑001 – AI‑Contract Generation**
> **When** the user runs `ucp contract --spec <spec.json>`, **the system** shall produce a `ucp‑contract.json` file containing a Zod‑validatable, typed JSON schema describing all components in a format optimised for LLM structured output.

**REQ‑AI‑002 – Contract Schema**
> **The AI contract** shall include per‑component: `id`, `name`, `description`, `props` (with type, required, default, enum values), `events` (with payload types), `variants`, `stateMachine` (as SMDL string).

**REQ‑AI‑003 – MCP Server Integration**
> **The pipeline** shall include an MCP server mode (`ucp mcp`) that exposes CAM data to AI agents via the Model Context Protocol, providing structured access to component definitions, prop schemas, and guidelines.

### 3. Quality and Non‑Functional Requirements

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑PERF‑001 | Dashboard generation for 200 components < 3s | `just test-perf` measurement |
| NFR‑PERF‑002 | Dashboard loads in browser < 2s (P95) | Lighthouse / manual measurement |
| NFR‑REL‑001 | Generated registry passes `shadcn build` validation | CI step with shadcn CLI |
| NFR‑REL‑002 | W3C export passes official schema validation | CI step when validator available |
| NFR‑MNT‑001 | Test coverage ≥ 85% for new modules | Coverage report |
| NFR‑MNT‑002 | Zero clippy warnings | CI lint gate |
| NFR‑SEC‑001 | Dashboard has no server‑side dependencies (static only) | Inspection |
| NFR‑SEC‑002 | MCP server requires explicit opt‑in and authentication | Inspection |

### 4. External Interfaces

#### 4.1 CLI Interface (New Commands)

- `ucp dashboard --spec <spec.json> --output <dir>` — generate the dashboard.
- `ucp contract --spec <spec.json> --output <file>` — generate AI‑contract.
- `ucp mcp` — start MCP server.
- `ucp validate --target registry` — validate registry output against shadcn schema.

#### 4.2 Output Formats

- W3C‑compliant `ucp‑spec.w3c.json`.
- shadcn CLI 3.0+‑compliant `registry.json` and `registry‑item.json`.
- AI‑contract `ucp‑contract.json`.
- Dashboard static site (`index.html` + assets).

### 5. Traceability

| Goal | Capability | SRS Requirements |
|------|------------|------------------|
| G‑01 | W3C compliance | REQ‑W3C‑010 … REQ‑W3C‑016 |
| G‑02 | shadcn registry compliance | REQ‑REG‑010 … REQ‑REG‑016 |
| G‑03 | Visual dashboard | REQ‑DASH‑001 … REQ‑DASH‑009 |
| G‑04 | AI‑contract format | REQ‑AI‑001 … REQ‑AI‑003 |

---

## Architecture & Design Specification – UCP v0.4.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Architecture & Design Specification |
| Version | 0.4.0‑arch‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Context and Scope

v0.4.0 adds four major capability areas: W3C UI Spec export, shadcn CLI 3.0+ registry export, visual dashboard, and AI‑contract format. These are implemented as new modules and enhancements to existing ones, with no breaking changes to the v0.3.0 CAM or pipeline.

### 2. Architecturally Significant Requirements (ASRs)

| ID | Requirement | Source |
|----|-------------|--------|
| ASR‑001 | W3C UI Spec export with full field coverage | REQ‑W3C‑011 … REQ‑W3C‑016 |
| ASR‑002 | shadcn CLI 3.0+ registry compliance | REQ‑REG‑010 … REQ‑REG‑016 |
| ASR‑003 | Static dashboard generation | REQ‑DASH‑001 … REQ‑DASH‑009 |
| ASR‑004 | AI‑contract format and MCP server | REQ‑AI‑001 … REQ‑AI‑003 |

### 3. System Design

#### 3.1 Refactored: `export::w3c` → Full Schema Compliance

The v0.3.0 W3C exporter used a simplified `W3cComponent` struct. v0.4.0 expands it to cover all fields defined by the W3C UI Spec Schema draft:

```rust
struct W3cComponent {
    id: String,
    name: String,
    category: Option<String>,        // new: input, layout, overlay, etc.
    description: Option<String>,     // new: from LLM or CAM
    anatomy: Vec<W3cPart>,           // new: from ExtractedPart
    states: Option<W3cStateMachine>, // enhanced: from StateMachine
    variants: Vec<W3cVariant>,       // new: from concrete enum types
    properties: Vec<W3cProp>,        // enhanced: added constraints
    events: Vec<W3cEvent>,
    accessibility: Option<W3cAccessibility>, // new: ARIA, keyboard
    responsive: Option<W3cResponsive>,       // new: breakpoints
    dependencies: Vec<String>,               // new: registry dependencies
    design_tokens: Option<String>,           // new: ref to DTCG file
}
```

#### 3.2 Refactored: `generate::registry` → CLI 3.0+ Compliance

The registry module is updated to emit the full shadcn CLI 3.0 schema:

```rust
struct RegistryIndex {
    schema: String,
    name: String,
    homepage: String,
    items: Vec<RegistryItem>,
}

struct RegistryItem {
    schema: Option<String>,
    name: String,
    item_type: String,
    title: Option<String>,
    description: Option<String>,
    author: Option<String>,
    dependencies: Vec<String>,          // npm deps
    dev_dependencies: Vec<String>,       // npm dev deps
    registry_dependencies: Vec<String>,  // namespaced format
    files: Vec<RegistryFile>,
    css_vars: Option<CssVars>,
    meta: Option<serde_json::Value>,
}

struct CssVars {
    light: serde_json::Value,
    dark: serde_json::Value,
}
```

Namespaced dependency resolution: `@registry/component` format is supported, with fallback to bare names.

#### 3.3 New Module: `dashboard`

**Purpose:** Produce a self‑contained static site from a `SynthesisOutput`.

**Design decisions:**
- **No build step:** The dashboard is pure HTML/CSS/JS, written directly by the generator. No NPM, no bundler.
- **Tailwind CSS via CDN or inline:** Use Tailwind Play CDN for styling in the generated HTML.
- **Mermaid.js for diagrams:** Embedded via `<script>` tag; renders state machines client‑side.
- **Fuse.js for search:** Lightweight fuzzy search in the component list.
- **Prism.js for syntax highlighting:** Source code display in the registry preview.
- **Vanilla JS for interactivity:** No framework dependency; all DOM manipulation is imperative.

**File structure produced by `ucp dashboard`:**

```
dashboard/
├── index.html          # main entry point
├── assets/
│   ├── style.css       # generated Tailwind CSS
│   ├── app.js          # core dashboard logic
│   ├── search.js       # Fuse.js integration
│   └── data.json       # the spec data, embedded
└── favicon.ico
```

**Architecture:**
```
SynthesisOutput
       │
       ▼
┌─────────────────────┐
│ dashboard::generate  │
│ - render list view   │
│ - render detail pages│
│ - render conflicts   │
│ - render graph       │
│ - embed spec JSON    │
└──────────────────────┘
       │
       ▼
   dashboard/ directory (static files)
```

All dynamic behavior (search, filtering, tab switching) is handled by client‑side JavaScript. The `data.json` file is the same `ucp‑spec.json` content, embedded directly in the HTML or as a separate file loaded via `fetch`.

#### 3.4 New Module: `contract`

**Purpose:** Export an AI‑optimised JSON contract from a `SynthesisOutput`.

```rust
#[derive(Serialize)]
struct AiContract {
    schema: String,                     // "ucp-contract/v1"
    generated_at: String,
    components: Vec<AiComponent>,
}

#[derive(Serialize)]
struct AiComponent {
    id: String,
    name: String,
    description: String,
    props: Vec<AiProp>,
    events: Vec<AiEvent>,
    variants: Vec<AiVariant>,
    state_machine: Option<String>,      // SMDL string
    dependencies: Vec<String>,
}

#[derive(Serialize)]
struct AiProp {
    name: String,
    #[serde(rename = "type")]
    prop_type: String,                  // concrete type preferred, fallback abstract
    required: bool,
    default: Option<String>,
    #[serde(rename = "enumValues", skip_serializing_if = "Vec::is_empty")]
    enum_values: Vec<String>,
    description: Option<String>,
}
```

#### 3.5 New Feature: MCP Server

**Purpose:** Expose CAM data to AI agents via the Model Context Protocol.

The MCP server is started with `ucp mcp --spec <spec.json>` and listens on stdio (standard MCP transport). It responds to:
- `list_components` — returns component IDs and names.
- `get_component` — returns full component details.
- `get_props` — returns typed props for a component.
- `search` — fuzzy search across components.

Implementation uses the `mcp-server` Rust crate (or a minimal custom implementation of the MCP JSON‑RPC protocol over stdio).

### 4. Key Design Decisions (ADRs)

#### ADR‑010: Static Site Dashboard (No Framework)
**Context:** The dashboard must be self‑contained and openable in any browser without a server or build step.
**Decision:** Generate pure HTML/CSS/JS using embedded Tailwind CDN, Mermaid.js, Fuse.js, and Vanilla JS.
**Drivers:** ASR‑003, offline operation requirement.
**Consequences:** No build step required; all dependencies are loaded from CDN or inlined.

#### ADR‑011: W3C Schema as Primary Export Format
**Context:** The W3C UI Spec Schema is still draft but will become a formal standard.
**Decision:** Make the W3C export the most complete format; keep the native CAM format for compatibility.
**Drivers:** ASR‑001, first‑mover advantage.
**Consequences:** Dual maintenance of export formats; mitigated by clean mapping layer.

#### ADR‑012: shadcn Registry as Index Object
**Context:** shadcn CLI 3.0 requires `registry.json` to be an object with `items`, not a bare array.
**Decision:** Update the registry generator to produce a proper index object and namespaced dependencies.
**Drivers:** ASR‑002.
**Consequences:** Breaking change from v0.3.0 format; acceptable since v0.3.0 was not widely adopted.

#### ADR‑013: Design Tokens Integration
**Context:** The W3C schema and shadcn registry both support design tokens.
**Decision:** Extract design tokens (colors, spacing, typography) from source CSS/Tailwind config and include them in all export formats.
**Drivers:** REQ‑W3C‑016, REQ‑REG‑013.
**Consequences:** New `extract::tokens` module; optional feature gated behind `--tokens` flag initially.

### 5. Traceability

| ASR | Design Elements | ADRs |
|-----|----------------|------|
| ASR‑001 | `export::w3c` full schema, `extract::tokens` | ADR‑011, ADR‑013 |
| ASR‑002 | `generate::registry` CLI 3.0+, `CssVars`, namespaced deps | ADR‑012 |
| ASR‑003 | `dashboard` module, static site generation | ADR‑010 |
| ASR‑004 | `contract` module, MCP server | – |

---

## Behavioral Specification & Test Verification Plan – UCP v0.4.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Behavioral Spec & Test Verification |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Behavioral Specifications

#### Feature: W3C UI Spec Export

```gherkin
Scenario: Export a button component as W3C UI Spec
  Given a SynthesisOutput with a Button component
    - props: disabled (ControlFlag, bool), label (StaticValue, String)
    - events: click (AsyncEventHandler)
    - parts: trigger, content
    - state machine: Closed ↔ Open
  When the W3C exporter runs
  Then `ucp‑spec.w3c.json` is created containing:
    - component.id = "...:Button"
    - component.properties[0].name = "disabled", type = "controlflag", required = false
    - component.properties[1].name = "label", type = "staticvalue"
    - component.events[0].name = "click"
    - component.anatomy[0].name = "trigger"
    - component.states.initial = "Closed"
    - component.states.nodes count = 2

Scenario: Export component with variants
  Given a component with prop "variant" of concrete_type "enum: Default, Destructive, Outline"
  When the W3C exporter runs
  Then the component has a "variants" array containing a variant "variant" with values ["Default", "Destructive", "Outline"]
```

#### Feature: shadcn CLI 3.0+ Registry Export

```gherkin
Scenario: Export registry with full CLI 3.0 schema
  Given a PackageManifest with name "acme", homepage "https://acme.com"
    And a Button component
  When the registry exporter runs
  Then `registry.json` is an object (not array) with:
    - $schema = "https://ui.shadcn.com/schema/registry.json"
    - name = "acme"
    - homepage = "https://acme.com"
    - items[0].$schema = "https://ui.shadcn.com/schema/registry-item.json"
    - items[0].name = "button"
    - items[0].type = "registry:ui"
    - items[0].title = "Button" (humanised)
    - items[0].files[0].path = "src/button.rs"
    - items[0].files[0].content = (source code)

Scenario: Namespaced dependencies in registry
  Given a manifest with namespace "@acme"
    And a Dialog component that depends on Button
  When the registry exporter runs
  Then the Dialog item's registryDependencies includes "@acme/button"

Scenario: CSS variables in registry item
  Given a component that defines --primary: oklch(0.645 0.246 16.439) in light mode
  When the registry exporter runs with CSS variable extraction enabled
  Then the item contains:
    cssVars.light["--primary"] = "oklch(0.645 0.246 16.439)"
```

#### Feature: Visual Dashboard

```gherkin
Scenario: Generate dashboard from spec
  Given a SynthesisOutput with 3 components: Button, Dialog, Accordion
  When `ucp dashboard` runs
  Then a `dashboard/index.html` file is created
    And it contains a searchable component list with all 3 components
    And clicking a component opens a detail page with props table, events, and state machine diagram

Scenario: Conflict display in dashboard
  Given a spec with a conflict on "disabled" prop (bool vs String)
  When the dashboard is generated
  Then the conflict view shows:
    - prop name: "disabled"
    - present in: ["button.rs"]
    - absent in: ["button.tsx"]
    - confidence: 0.7

Scenario: Dependency graph in dashboard
  Given a spec where Dialog references Button
  When the dashboard is generated
  Then the dependency graph shows a directed edge from Dialog to Button

Scenario: Registry preview in dashboard
  Given a spec with generated source code
  When the dashboard is generated
  Then each component detail page shows a "Registry Preview" tab
    And it displays the source code with syntax highlighting
    And a download button is present
```

#### Feature: AI‑Contract Export

```gherkin
Scenario: Export AI contract from spec
  Given a SynthesisOutput with a Button component
    - props: disabled (ControlFlag, bool, required=false)
    - events: click
  When `ucp contract` runs
  Then `ucp‑contract.json` is created containing:
    - components[0].id = "...:Button"
    - components[0].props[0].name = "disabled"
    - components[0].props[0].type = "bool"
    - components[0].props[0].required = false
    - components[0].events[0].name = "click"

Scenario: AI contract includes enum values
  Given a component with prop "variant" of concrete_type "enum: Default, Destructive"
  When the AI contract is generated
  Then the prop entry includes enumValues: ["Default", "Destructive"]
```

### 2. Test Strategy

| Layer | Goal | Tools |
|-------|------|-------|
| Unit tests | W3C mapping, registry schema, dashboard rendering, contract format | Rust #[test] |
| Integration tests | Full pipeline: extract → export → validate | tempfile, tokio::test |
| Schema validation | W3C JSON Schema, shadcn registry Schema | jsonschema‑rs or external CLI |
| Manual verification | Dashboard opens in browser, MCP server connects to AI tool | Browser, MCP Inspector |
| Regression tests | All v0.3.0 tests continue to pass | cargo nextest |

### 3. NFR Verification

| NFR | Verification | Evidence |
|-----|-------------|----------|
| PERF‑001 | `just test-perf` with 200 components | Timing log |
| PERF‑002 | Lighthouse or manual measurement | Performance report |
| REL‑001 | `shadcn build` in CI | Build log |
| REL‑002 | W3C schema validation in CI | Validation pass log |
| MNT‑001 | Coverage report | llvm‑cov output |
| MNT‑002 | `just lint` | Zero warnings |
| SEC‑001 | Inspection of generated dashboard | No server code |
| SEC‑002 | MCP server audit | Authentication check |

### 4. Requirements Traceability Matrix

| Requirement ID | Test Case / Scenario | Verification |
|---------------|---------------------|--------------|
| REQ‑W3C‑011 | SCN‑W3C‑export‑button | Schema validation |
| REQ‑W3C‑012 | SCN‑W3C‑full‑fields | Inspection |
| REQ‑W3C‑013 | SCN‑W3C‑anatomy | Test |
| REQ‑W3C‑014 | SCN‑W3C‑states | Test |
| REQ‑W3C‑015 | SCN‑W3C‑variants | Test |
| REQ‑W3C‑016 | SCN‑W3C‑tokens | Test |
| REQ‑REG‑010 | SCN‑REG‑full‑schema | Schema validation |
| REQ‑REG‑011 | SCN‑REG‑index‑object | Test |
| REQ‑REG‑012 | SCN‑REG‑namespaced | Test |
| REQ‑REG‑013 | SCN‑REG‑css‑vars | Test |
| REQ‑REG‑014 | SCN‑REG‑target‑path | Test |
| REQ‑REG‑015 | SCN‑REG‑npm‑deps | Test |
| REQ‑DASH‑001 | SCN‑DASH‑generation | Test + Manual |
| REQ‑DASH‑002 | SCN‑DASH‑list‑view | Manual |
| REQ‑DASH‑003 | SCN‑DASH‑detail‑page | Manual |
| REQ‑DASH‑004 | SCN‑DASH‑conflicts | Manual |
| REQ‑DASH‑005 | SCN‑DASH‑graph | Manual |
| REQ‑DASH‑006 | SCN‑DASH‑registry‑preview | Manual |
| REQ‑AI‑001 | SCN‑AI‑contract‑export | Test |
| REQ‑AI‑002 | SCN‑AI‑contract‑schema | Test |
| REQ‑AI‑003 | SCN‑AI‑mcp | Test + Manual |

---

This completes the full specification suite for UCP v0.4.0: Vision, SRS, Architecture, and Test Plan. The suite defines a complete, testable scope for the release, grounded in the W3C research, shadcn CLI 3.0 schema details, and dashboard requirements. Ready for implementation planning whenever you are.
