# Universal Component Protocol (UCP) – Vision & Strategic Alignment v4.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Vision & Strategic Alignment |
| Version | 4.0 |
| Date | 2026-04-21 |
| Author | System Architect (assisted by AI) |
| Status | Draft — Pending Review |
| Supersedes | Vision v3.0 |

**Change Summary (v3.0 → v4.0):** Introduces **Ecosystem Synthesis** as the primary bootstrap mechanism. UCP shifts from a manually authored or single-source-extracted standard to an **AI-computed consensus**. The protocol defines an automated multi-agent workflow for discovering existing shadcn/ui ports, deeply extracting their implementations via AST and LLM, and intelligently unifying them into a Canonical Abstract Model (CAM). The spec is no longer written; it is *synthesized*, audited via strict security/sandboxing rules, and curated through a static 3-column conflict resolution UI.

---

## Vision Statement

To establish a **Universal Component Protocol (UCP)** that is automatically computed from the existing shadcn/ui ecosystem. Using an intelligent, secure multi-agent workflow, UCP discovers disparate framework ports (React, Leptos, Dioxus, GPUI), extracts their APIs and behavioral semantics, resolves semantic equivalences across fundamentally different paradigms (signals vs. hooks vs. builders), and synthesizes a single, canonical, machine-readable specification. This living consensus serves as the absolute source of truth for human engineers, cross-platform tooling, and AI component generators.

## Elevator Pitch (Moore's Template)

**For** framework authors, library maintainers, and AI tooling developers  
**who are frustrated** by the impossibility of manually writing and maintaining a universal spec that accurately reflects the rapidly evolving realities of 10+ divergent shadcn/ui ports across web and desktop paradigms,  
**our initiative** defines an AI-driven synthesis engine that automatically discovers, extracts, and intelligently unifies existing component codebases into a single canonical protocol,  
**that provides** a mathematically resolved, conflict-flagged, and human-curatable specification covering APIs, state machines, anatomical parts, and accessibility—generated entirely from real code,  
**unlike** traditional standards that require manual authoring, endless pull requests, or simple regex extractions that miss behavioral nuances,  
**our approach** treats the existing ecosystem as the source of truth, using local-first AI to compute the consensus so that implementations no longer conform to a document—they *are* the document.

## Problem Statement & Business Context

**The Core Fallacy of v3.0:** Previous iterations assumed a human (or simple AST CLI tool) would author or extract the UCP spec. This is fundamentally flawed:
1.  **The ecosystem is too vast:** 10+ frameworks, 50+ components each, with unique reactivity models.
2.  **Semantic divergence is hidden:** React's `isOpen` + `onOpenChange` is functionally identical to Leptos's `open: RwSignal<bool>`, but no simple regex can know this.
3.  **Behavioral opacity:** Two components can have identical props, but one traps focus and the other doesn't. AST parsing cannot detect focus trapping.
4.  **Manual curation doesn't scale:** By the time humans agree on a spec, three libraries have added new props.

**The v4.0 Paradigm Shift:** We stop writing the spec. Instead, we build an **AI Unification Engine** that:
1.  **Discovers** known shadcn/ui ports via registries and GitHub (respecting strict license boundaries).
2.  **Extracts** deep semantics using AST parsing (for structure) and local LLMs (for behavior).
3.  **Fingerprints** components to find exact matches across frameworks.
4.  **Unifies** them by translating framework-specific types into a Canonical Abstract Model (CAM).
5.  **Generates** the UCP JSON Schema with built-in confidence scores, conflict flags, and a 3-column HTML UI for human review.

## Target Users & Customers

| User | Role in v4.0 |
|------|--------------|
| **Spec Editors** | No longer write specs; they review AI-generated unification diffs in a local HTML UI and resolve conflicts. |
| **Library Maintainers** | Their existing code automatically becomes part of the UCP consensus. They only act if the AI misinterprets their API. |
| **AI Component Generators** | Consume the synthesized UCP spec, knowing it is mathematically derived from real implementations and includes explicit LLM-ready annotations. |
| **Cross-Platform Toolers** | Use the unified spec and auto-generated Framework Mapping Documents (FMDs) to generate code for both prop-based and builder-pattern targets. |

## Desired Outcomes & Success Metrics

| ID | Outcome | Key Results |
|----|---------|-------------|
| **G-1** | **Zero-Touch Bootstrapping** | A runnable UCP v1.0 spec for 50 components is synthesized automatically from 3 existing ports within 24 hours. |
| **G-2** | **Semantic Resolution Accuracy** | The unification engine correctly maps signal-based reactivity to hook-based reactivity with ≥95% accuracy against a manually curated "Golden Dataset". |
| **G-3** | **Continuous Sync** | The UCP spec is automatically re-synthesized weekly, with Spec Editors only reviewing delta diffs in a secure, local HTML UI. |
| **G-4** | **AI Tooling Adoption** | At least 2 major AI coding tools reference UCP specs for component generation within 18 months. |

## Strategic Constraints

-   **Extraction over Imagination:** The AI must *only* synthesize what exists in the codebases. It cannot invent props or behaviors.
-   **Local-First & Secure:** Source code extraction and LLM inference MUST execute locally by default. No source code leaves the user's machine unless explicitly overridden.
-   **License Respect:** The pipeline MUST strictly reject non-permissive (copyleft) licenses.
-   **Conflict Visibility:** When ports disagree, the spec MUST expose this as a flagged conflict, not silently pick a winner.
-   **Human Veto:** The AI proposes the spec; human Spec Editors must approve it via a 3-column curation UI before it becomes canonical.

## Goals and Non-Goals

**Goals:**
-   Define the multi-agent AI pipeline (Discovery, Extraction, Unification, Generation).
-   Define the Canonical Abstract Model (CAM) for framework-agnostic type representation.
-   Define the XState v5 subset for behavioral state machines.
-   Define component anatomy (`data-ucp-part`) and state attributes.
-   Define security boundaries (SPDX license filtering, sandboxing, local LLM).
-   Provide the `ucp bootstrap` CLI and static HTML conflict resolution UI.

**Non-Goals:**
-   Defining visual design, CSS, or design tokens (reserved for UCP Theming Extension).
-   Running cloud-based LLMs by default (local Ollama integration is the default).

---

*End of Vision v4.0*

---

# Business & Stakeholder Requirements Specification v4.0
**Universal Component Protocol (UCP)**

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Business & Stakeholder Requirements Specification (BRS) |
| Version | 4.0 |
| Date | 2026-04-21 |
| Status | Draft — Pending Review |

---

## 4. Business Requirements

### BR-001: Automated Multi-Source Discovery
UCP SHALL provide an agent that automatically discovers shadcn/ui implementations across public repositories using heuristics and registry metadata, subject to strict license gating.

### BR-002: Deep Semantic Extraction
UCP SHALL extract behavioral semantics (state machines, focus management, portaling) using a combination of static AST analysis and targeted local LLM queries against sandboxed source code.

### BR-003: Intelligent Cross-Framework Unification
UCP SHALL define a Unification Engine that resolves semantic equivalences across different paradigms (e.g., mapping React's `onChange` + `value` to Leptos's `RwSignal<T>`, and GPUI's `Model<T>`) via a strict Cross-Framework Type Ontology.

### BR-004: Conflict Flagging & Confidence Scoring
UCP SHALL assign mathematical confidence scores (0.0-1.0) to unified properties and explicitly flag conflicts where implementations diverge.

### BR-005: Human-in-the-Loop Curation Workflow
UCP SHALL present unification results as a static, local 3-column HTML UI (Source A vs Source B vs Proposed UCP Canon) that Spec Editors can review without exposing source code to external servers.

### BR-006: Canonical Spec Generation
UCP SHALL generate the final UCP JSON Schema (incorporating state machines, parts, LLM annotations) entirely from the unified Canonical Abstract Model.

### BR-007: Strict Security & Privacy
UCP SHALL enforce permissive license filtering, file-system sandboxing, and local-first LLM inference to ensure proprietary source code is never leaked.

---

## 5. Stakeholder Requirements

### 5.1 Security & Privacy (Critical for Bootstrap)

**SR-SEC-001: License Gate**
The Discovery Agent MUST parse `Cargo.toml` `[package.license]` or `package.json` `"license"`. If the license is NOT in the SPDX permissive list (`MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `0BSD`, `Unicode-DFS-2016`), the agent SHALL reject the repository.

**SR-SEC-002: Extraction Sandboxing**
The Extraction Agent MUST only parse files matching strict globs (`**/src/**/*.rs`, `**/src/**/*.tsx`, `**/components/**`). It MUST explicitly ignore `.env`, `credentials*`, `*.pem`, `.github/`, `tests/`.

**SR-SEC-003: Local-First LLM Inference**
By default, the `ucp bootstrap` pipeline MUST use a local LLM via the `Ollama` REST API (`http://localhost:11434`). If the user explicitly passes `--llm-provider openai`, it MUST require an environment variable `UCP_OPENAI_KEY` and MUST print a warning: `WARNING: Sending extracted source code to external API.`

**SR-SEC-004: Local Conflict Review UI**
The Curation UI output (`ucp resolve --format html`) SHALL generate a static HTML file running entirely in the browser with zero backend dependencies, ensuring extracted source code never leaves the user's machine during curation.

### 5.2 The Unification Engine

**SR-UNI-001: Cross-Framework Type Ontology**
The engine SHALL utilize a strict mapping table translating AST types to Canonical Abstract Model (CAM) types:
-   `ControlledValue<T>`: React `[T, (val: T) => void]`; Leptos `RwSignal<T>`; GPUI `Model<T>`.
-   `AsyncEventHandler<T>`: React `(event: T) => void`; Leptos `Callback<T>`; GPUI `Box<dyn Fn(&mut Window, &mut App, &T)>`.
-   `Renderable`: React `ReactNode`; Leptos `Children`; GPUI `impl IntoElement`.

**SR-UNI-002: LLM Equivalence Fallback**
If AST mapping confidence falls below 0.8, the engine SHALL query the local LLM with isolated function signatures and docstrings to determine the exact CAM type, requesting a structured JSON response.

**SR-UNI-003: Conflict Resolution Strategies**
The engine SHALL apply strict mathematical strategies:
-   *Majority Rules:* If 4/5 ports have a prop, it is included with `confidence: 0.8`.
-   *Profile Scoping:* If only desktop ports have a prop, it is scoped to the `desktop` profile.
-   *Flag for Review:* If behavior differs (e.g., focus management present in 2/4 ports), it is flagged as a `conflict` requiring human SEP review.

### 5.3 State Machines & Anatomy

**SR-SM-001: XState v5 Subset**
State machines SHALL be defined using a strict subset of the XState v5 JSON schema, augmented with a custom `sideEffects` array.

**SR-SM-002: State Machine DSL (SMDL)**
To improve LLM generation accuracy, the pipeline SHALL accept an intermediate DSL (parsed by `pest`/`nom`) that compiles to the XState JSON format.

**SR-ANAT-001: Web Part Standardization**
Web frameworks SHALL use `data-ucp-part="label"` attributes to expose component anatomy, avoiding Shadow DOM `part=""` to prevent SSR hydration mismatches and allow global CSS utility styling.

**SR-ANAT-002: Desktop Native A11y Tree**
Desktop harnesses SHALL verify component anatomy and state via platform APIs (macOS `osascript` AX queries, Windows `UIAutomation`), mapping UCP parts to native accessibility node roles.

---

*End of BRS v4.0*

---

# Software Requirements Specification v4.0
**Universal Component Protocol (UCP)**

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Software Requirements Specification (SRS) |
| Version | 4.0 |
| Date | 2026-04-21 |
| Status | Draft — Pending Review |

---

## 4. Functional Capabilities and Behavior

### 4.1 The Multi-Agent Pipeline Architecture

The UCP system is restructured around a pipeline of specialized AI agents, implemented in a new core crate: `ucp-synthesizer`.

**SR-ARCH-001: Agent Data Structures**

```rust
/// Represents a discovered repository
pub struct DiscoveredRepo {
    pub url: String,
    pub framework: Framework,
    pub platform: PlatformProfile,
    pub spdx_license: Option<String>,
    pub local_path: PathBuf,
}

/// The abstract, framework-agnostic representation of a component
pub struct CanonicalAbstractComponent {
    pub id: String,
    pub semantic_fingerprint: SemanticFingerprint,
    pub props: Vec<CanonicalAbstractProp>,
    pub events: Vec<CanonicalAbstractEvent>,
    pub extracted_state_machine: Option<StateMachine>,
    pub extracted_parts: Vec<ExtractedPart>,
    pub source_repos: Vec<SourceAttribution>,
}

pub struct CanonicalAbstractProp {
    pub canonical_name: String, // e.g., "disabled"
    pub abstract_type: AbstractPropType, // e.g., ControlFlag, ControlledValue(String)
    pub reactivity: AbstractReactivity,
    pub sources: Vec<PropSourceMapping>, // Maps back to React, Leptos, GPUI
    pub confidence: f32, // 0.0 to 1.0
    pub conflicts: Vec<Conflict>,
}

pub enum AbstractPropType {
    ControlFlag,
    ControlledValue(Box<AbstractPropType>),
    UncontrolledValue(Box<AbstractPropType>),
    StaticValue(Box<AbstractPropType>),
    AsyncEventHandler(Vec<AbstractPropType>),
    Renderable,
    Any,
}
```

### 4.2 Phase 1: Discovery Agent

**SR-DISC-001: Registry Scraping & License Gating**
The agent SHALL query GitHub search API and package registries. Before cloning, it SHALL fetch the `LICENSE` file or parse `Cargo.toml`/`package.json` remotely. If the license is not in the allowed SPDX list, it SHALL skip the repo and log `Rejected {repo}: Non-permissive license`.

**SR-DISC-002: Framework Detection**
The agent SHALL detect the framework based on dependencies: `leptos` (Leptos), `dioxus` (Dioxus), `gpui` (GPUI), `react` (React).

### 4.3 Phase 2: Extraction Agent (Dual-Mode)

**SR-EXT-001: AST Extraction (Sandboxed)**
The agent SHALL parse files matching `**/src/**/*.rs` and `**/src/**/*.tsx`. For Rust, it uses `syn` to find `#[component]` functions. For TS, it uses `swc_ecma_ast`. It outputs a `RawComponentExtraction` containing purely structural data (names, raw types, docstrings).

**SR-EXT-002: LLM Behavioral Extraction (Local)**
The agent SHALL format targeted prompts for a local Ollama instance to infer behavior.
*Example Prompt:*
> "Analyze this Leptos Dialog component code. Does it implement focus trapping when opened? Does it portal to the body? Does it lock scroll? Answer strictly in JSON: `{ "focus_trap": true, "portal": true, "scroll_lock": true }`."

**SR-EXT-003: State Machine DSL Generation**
To maximize LLM accuracy, the agent SHALL prompt the LLM to output the UCP State Machine DSL (SMDL), rather than raw JSON.
*SMDL Syntax:*
```text
component Dialog {
  state Closed {
    on OPEN -> Open {
      + focus: move_to [part=content]
      + scroll: lock_body
      + portal: mount [part=content] to body
    }
  }
  state Open {
    on CLOSE -> Closing { + animation: play [part=content] exit }
    on ESCAPE -> Closing { + animation: play [part=content] exit }
  }
  state Closing {
    on ANIMATION_END -> Closed {
      + focus: return_to trigger
      + scroll: unlock_body
      + portal: unmount [part=content]
    }
  }
}
```
The pipeline SHALL include a `pest` parser to compile this DSL into the required XState v5 JSON schema format.

### 4.4 Phase 3: Unification Agent

**SR-UNI-001: Semantic Fingerprinting & Clustering**
The agent SHALL generate a `SemanticFingerprint` (a vector of purpose embeddings + normalized prop names) for each `RawComponentExtraction`. It SHALL use DBSCAN clustering to group identical components across frameworks.

**SR-UNI-002: CAM Translation via Ontology**
For each cluster, the agent SHALL translate framework-specific extractions into `CanonicalAbstractComponent` using the strict Cross-Framework Type Ontology.

*Mapping Logic Example:*
| React Extraction | Leptos Extraction | GPUI Extraction | CAM Translation |
| :--- | :--- | :--- | :--- |
| `disabled: boolean` | `disabled: bool` | `.disabled(bool)` | `disabled: ControlFlag` |
| `value: string`, `onChange: (e) => void` | `value: RwSignal<String>` | `value: Model<String>` | `value: ControlledValue<String>` |

**SR-UNI-003: Conflict Scoring Algorithm**
If CAM translation reveals a discrepancy, a `Conflict` is generated with a strict score:
```rust
pub struct Conflict {
    pub id: String,
    pub field: String,
    pub present_in: Vec<String>, // e.g., ["react", "leptos", "vue"]
    pub absent_in: Vec<String>,  // e.g., ["gpui"]
    pub confidence: f32,        // e.g., 0.75 (3/4 ports)
    pub resolution_suggestion: ResolutionStrategy,
}

pub enum ResolutionStrategy {
    IncludeMajority,        // Add to spec
    ScopeToProfile(String), // Add only to "web" or "desktop"
    FlagForHumanReview,     // Block until Spec Editor decides
}
```

### 4.5 Phase 4: Spec Generation Agent

**SR-GEN-001: CAM to UCP JSON Translation**
The agent SHALL convert the finalized `CanonicalAbstractComponent` into the UCP JSON Schema, translating CAM types back into UCP enriched types (e.g., `ControlledValue<String>` becomes `Signal { inner: String }` in the final output, scoped by FMD).

**SR-GEN-002: Automatic LLM Annotation Generation**
Using the aggregated source code and docstrings, the agent SHALL generate the `llm` block:
- `behaviorDescription`: Synthesized from all repo READMEs.
- `antiPatterns`: Extracted from "Don't" comments or bug reports.
- `fewShotExamples`: Generated from existing Storybook stories.

**SR-GEN-003: Auto-FMD Generation**
For every source repo, the agent SHALL automatically output the Framework Mapping Document based on the `PropSourceMapping` data.

### 4.6 Phase 5: Curation & Governance

**SR-CUR-001: 3-Column Conflict Review UI**
The `ucp resolve` command SHALL generate a single, static `index.html` file. It MUST NOT require a backend server.
*Structure:*
- **Left Column:** Source Code A (e.g., React `Button` interface).
- **Middle Column:** Source Code B (e.g., Leptos `Button` props struct).
- **Right Column:** Proposed UCP Canon JSON.
- **Footer:** Buttons to `Accept Majority`, `Scope to Desktop`, or `Flag for Review`.

### 4.7 UCP Specification Schema (Final Output Format)

The JSON Schema generated by the pipeline SHALL conform to this exact structure:

**SR-SPEC-001: Complete Component Schema**
```json
{
  "ucpVersion": "4.0.0",
  "components": {
    "button": {
      "name": "button",
      "description": "A clickable button element.",
      "category": "form",
      "since": "4.0.0",
      "dependencies": { "internal": ["slot"], "external": [] },
      "props": [
        {
          "name": "variant",
          "type": { "kind": "enum", "values": [ {"name": "default"}, {"name": "destructive"} ] },
          "reactivity": "static",
          "required": false,
          "default": "default"
        },
        {
          "name": "disabled",
          "type": { "kind": "boolean" },
          "reactivity": "static",
          "required": false,
          "default": false
        },
        {
          "name": "asChild",
          "type": { "kind": "boolean" },
          "slotBehavior": "instance-swap",
          "excludedWith": ["href"]
        }
      ],
      "variantAxes": [
        { "prop": "variant", "values": ["default", "destructive"], "default": "default" }
      ],
      "parts": [
        { "name": "root", "selectable": true },
        { "name": "label", "selectable": true }
      ],
      "stateAttributes": [
        { "name": "disabled", "type": "boolean", "reflectedWhen": "disabled prop is true" }
      ],
      "stateMachine": null,
      "accessibility": {
        "web": {
          "profile": "web",
          "implicitRole": "button",
          "requiredAttributes": [],
          "keyboardInteractions": [
            { "key": "Enter", "action": "activate", "expectedResult": "onClick fires" }
          ]
        },
        "desktop": {
          "profile": "desktop",
          "nativeRole": "AXButton"
        }
      },
      "stories": [
        { "name": "Disabled", "props": { "disabled": true }, "expectedState": { "stateAttributes": { "disabled": true } } }
      ],
      "llm": {
        "behaviorDescription": "A button triggers an action on click or keyboard.",
        "antiPatterns": ["Do NOT use a <div> for a button."]
      }
    }
  }
}
```

**SR-SPEC-002: XState v5 State Machine Subset**
For interactive components (e.g., Dialog), the `stateMachine` field SHALL follow this format:
```json
{
  "stateMachine": {
    "id": "ucp-dialog",
    "initial": "closed",
    "states": {
      "closed": {
        "on": { "OPEN": { "target": "open", "sideEffects": ["focus:move_to_first_focusable_in_content", "scroll:lock"] } }
      },
      "open": {
        "on": {
          "CLOSE": { "target": "closing", "sideEffects": ["animation:play_close"] }
        }
      },
      "closing": {
        "on": {
          "ANIMATION_END": { "target": "closed", "sideEffects": ["focus:return_to_trigger", "scroll:unlock", "portal:unmount"] }
        }
      }
    }
  }
}
```

### 4.8 Conformance Harness Architecture (Revised)

**SR-HARNESS-001: Web DOM Assertions (Headless CI)**
The Web harness SHALL run in Playwright (headless Chromium) and execute explicit JS assertions for state machine side effects:
- **Scroll Lock:** `await page.evaluate(() => getComputedStyle(document.body).overflow === 'hidden')`
- **Portal:** `await page.evaluate(() => document.body.contains(dialogElement))`
- **Focus Trap:** `await page.keyboard.press('Tab'); await page.evaluate(() => contentElement.contains(document.activeElement))`

**SR-HARNESS-002: Desktop Native A11y Tree Assertions**
The Desktop harness SHALL define a `NativeA11yInspector` trait:
```rust
pub trait NativeA11yInspector {
    fn dump_a11y_tree(&self, component: &Self::Component) -> Result<A11yNode>;
    fn get_focused_node(&self) -> Result<A11yNode>;
}

pub struct A11yNode {
    pub role: String, // e.g., "AXDialog"
    pub label: String,
    pub children: Vec<A11yNode>,
    pub is_focused: bool,
}
```
*macOS Implementation:* Shell out to `/usr/bin/osascript` running AppleScript to dump accessibility hierarchy and verify `AXRole == "AXDialog"` and `AXFocused == true`.
*Windows Implementation:* Shell out to PowerShell `Get-UIAControl` to verify `ControlType == "Dialog"`.

**SR-HARNESS-003: Web Part Attribute Standard**
The Web harness SHALL verify parts using `data-ucp-part` attributes:
```javascript
const labelPart = await page.$('[data-ucp-part="label"]');
expect(labelPart).not.toBeNull();
```
Shadow DOM `part=""` SHALL NOT be used by default to prevent SSR hydration mismatches.

## 5. Quality and Non-Functional Requirements

**NFR-SYN-001: Determinism**
Given the same set of source repositories and local LLM temperature=0, the Unification Engine SHALL produce byte-for-byte identical output.

**NFR-SYN-002: Auditability**
Every prop in the final UCP spec SHALL be traceable back to the exact source file, line number, and AST node via the `source_repos` manifest and `UnificationGraph`.

**NFR-SEC-001: Zero External Leakage**
In default configuration (`--llm-provider ollama`), the `ucp bootstrap` pipeline SHALL make zero network requests except to clone public git repositories. All LLM inference MUST occur via `localhost:11434`.

---

## 6. Data Contracts

### 6.1 The Unification Graph Format
To ensure humans can audit *how* the AI built the spec, the engine outputs a graph:
```json
{
  "unificationGraph": {
    "nodes": [
      { "id": "cam_button", "type": "CanonicalAbstractComponent" },
      { "id": "react_button", "type": "SourceComponent", "repo": "shadcn-ui", "file": "button.tsx" }
    ],
    "edges": [
      {
        "from": "react_button", "to": "cam_button",
        "mappings": [
          { "source": "disabled: boolean", "target": "disabled: ControlFlag", "confidence": 1.0 },
          { "source": "value: string + onChange", "target": "value: ControlledValue<String>", "confidence": 0.95 }
        ]
      }
    ],
    "conflicts": [
      { "id": "conf_001", "node": "cam_button", "field": "props.loading", "present_in": ["react"], "absent_in": ["leptos", "gpui"] }
    ]
  }
}
```

### 6.2 Auto-Generated FMD Example (Output of Synthesis)
```json
{
  "framework": "gpui",
  "autoGeneratedFrom": "unification_graph",
  "mappingRules": [
    { "canonicalProp": "disabled", "abstractType": "ControlFlag", "targetImplementation": { "trait": "Disableable", "method": "disabled(bool)" } },
    { "canonicalProp": "value", "abstractType": "ControlledValue<String>", "targetImplementation": { "type": "Model<String>" } }
  ]
}
```

---

## 8. Risks and Open Issues (Resolution Status)

| ID | Description | Resolution Status |
|----|-------------|-------------------|
| TBD-001 | How to strictly test "scroll lock" and "portal" in headless CI? | **RESOLVED:** Playwright executes real layout; harness uses explicit JS evaluations for `overflow` and `document.body.contains()`. |
| TBD-002 | Should state machines use SCXML? | **RESOLVED:** No. Using strict subset of XState v5 JSON, with custom `sideEffects` array. LLMs generate intermediate SMDL DSL (parsed by `pest`) which compiles to XState. |
| TBD-003 | How to map FocusTarget to GPUI native a11y? | **RESOLVED:** Defined `NativeA11yInspector` trait. macOS uses `osascript` AX queries; Windows uses `UIAutomation` PowerShell cmdlets to verify native roles and focus. |
| TBD-004 | Standardize `data-part` vs Shadow DOM `part`? | **RESOLVED:** Mandate `data-ucp-part="label"`. Shadow DOM `part=""` rejected due to SSR hydration issues and utility CSS conflicts in modern meta-frameworks. |
| TBD-005 | How to bootstrap without a ground truth spec? | **RESOLVED:** Execute "Pre-Implementation Sprint" to manually curate a Golden Dataset of 3 components across 2 frameworks to serve as the test suite for the Unification Agent. |

---

*End of Software Requirements Specification v4.0*
