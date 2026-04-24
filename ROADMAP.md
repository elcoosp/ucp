# 🗺️ UCP Roadmap

> **Last Updated:** 2026-04-21
> **Philosophy:** Compute, don't dictate. This roadmap tracks our journey from manual specification to a fully AI-synthesized, behaviorally strict, multi-platform component standard.

---

## 🏗️ Phase 1: The AI Bootstrap (v0.1 - Foundation)
*Goal: Prove the multi-agent synthesis pipeline works by automatically generating a draft spec from 2 web frameworks.*

- [ ] **Core Schema v1:** Define `ucp-core` data structures (Enriched `PropType`, `CanonicalAbstractModel`, `StateMachines`, `Component Parts`).
- [ ] **Discovery Agent:** Implement GitHub/npm registry scraping and framework detection (React, Leptos).
- [ ] **AST Extraction Agent:** Parse `#[component]` (Leptos) and React TSX using `syn`/`swc`.
- [ ] **LLM Extraction Agent:** Infer behavioral semantics (focus traps, portaling) via targeted LLM prompts.
- [ ] **Unification Agent (MVP):** Implement Semantic Fingerprinting to cluster `Button` across React and Leptos. Translate to the `CanonicalAbstractModel`.
- [ ] **Conflict Flagging:** Implement majority-rules logic and output the first `UnificationGraph`.
- [ ] **`ucp bootstrap` CLI:** Orchestrate the pipeline end-to-end.

*Milestone:* Running `ucp bootstrap` against `shadcn/ui` and `leptos-shadcn` outputs a conflict-flagged JSON spec for 20 core components.

---

## 🧪 Phase 2: Conformance & Code Gen (v0.5 - Usability)
*Goal: Make the synthesized spec actionable by adding automated testing and code generation for web targets.*

- [ ] **Spec Generation Agent:** Convert the `CanonicalAbstractModel` into the final UCP JSON Schema format.
- [ ] **Auto-FMD Generation:** Generate Framework Mapping Documents for discovered repos automatically.
- [ ] **LLM Annotation Engine:** Synthesize `behaviorDescription` and `antiPatterns` from clustered docstrings.
- [ ] **Web Harness (Leptos):** Build `ucp-harness-leptos` (DOM-mode) supporting `Signal<T>` creation and external state updates.
- [ ] **Bronze/Silver Tiers:** Implement name-matching and signal-reactivity conformance tests.
- [ ] **`ucp generate`:** Implement Handlebars templates for Leptos (signal-aware) and React (hook-aware).
- [ ] **Stories as Fixtures:** Render canonical stories (e.g., "Disabled Button") to ensure they don't crash.

*Milestone:* A developer can run `ucp bootstrap`, resolve conflicts, and then run `ucp generate button --target leptos` to get a fully typed, signal-ready component stub that passes Bronze/Silver tests.

---

## 🖥️ Phase 3: Deep Conformance & Desktop (v0.9 - Completeness)
*Goal: Expand beyond web APIs to support native desktop paradigms and strict behavioral state machine verification.*

- [ ] **Desktop Extraction:** Expand AST/LLM extraction to support Dioxus and GPUI (builder-pattern parsing, entity-based state).
- [ ] **Desktop Harness (GPUI):** Build `ucp-harness-gpui` (Manifest-mode) using native accessibility APIs instead of DOM.
- [ ] **Gold Tier (Web):** Implement structured keyboard interaction testing and `aria-labelledby` compound constraint verification.
- [ ] **Gold Tier (Desktop):** Implement native role checking and focus-trap verification via platform APIs.
- [ ] **State Machine Testing:** Drive components through exact transitions (e.g., `closed → open → closing → closed`) and verify side-effects (scroll lock, portal unmount).
- [ ] **Curation CLI:** Implement `ucp resolve` to step through AI-flagged conflicts interactively. Implement `ucp graph` (DOT format) for auditability.

*Milestone:* UCP accurately models and tests both a web signal framework (Leptos) and a native desktop builder framework (GPUI) from a single unified spec.

---

## 🌍 Phase 4: The Living Standard (v1.0 - Ecosystem)
*Goal: Launch UCP as the canonical, continuously synced standard for UI components and AI generators.*

- [ ] **`ucp-conform` Proc-Macro:** Compile-time verification (`#[ucp_conform(spec = "button.json")]`) for Rust implementations.
- [ ] **Test Adapters:** Integrate with Playwright+axe-core so existing test suites count toward UCP Gold conformance.
- [ ] **Per-Profile Dashboard:** Launch the public dashboard showing separate Web/Desktop conformance matrices.
- [ ] **Implementation Registry:** Launch `ucp registry` for auto-discovery and continuous report aggregation.
- [ ] **Full E2E Bootstrap:** Run pipeline against 5+ frameworks (React, Vue, Leptos, Dioxus, GPUI).
- [ ] **Continuous Sync Workflow:** Automated weekly re-synthesis of the spec as upstream repos update.

*Milestone:* v1.0 Release. The UCP spec is strictly versioned, fully auditable, and supports both human maintainers and AI coding tools (Cursor, v0, Bolt).

---

## 🔮 Future Extensions (Post-v1.0)

These features are explicitly reserved in the v3.0/v4.0 schema via the `_extension` namespace to prevent breaking changes.

*   **🎨 UCP Theming Extension (`ucp/theming-v1`):** Token binding per component part. Define how `button::label` maps to `--button-text-color`, including density scaling (compact/default/spacious) and dark mode state variants.
*   **🎞️ UCP Motion Extension (`ucp/motion-v1`):** Structured animation definitions for state machine transitions. Define durations, easings, and CSS/property targets, with strict `prefers-reduced-motion` fallbacks to `0ms`.
*   **📊 UCP Telemetry Extension (`ucp/telemetry-v1`):** Standardized lifecycle events (`mount`, `unmount`, `stateChange`) and performance markers (e.g., "time to interactive" for a Select dropdown) for design system observability.
*   **📦 UCP Size Extension (`ucp/size-v1`):** Bundle budget declarations per component (e.g., Button max 5kB gzip JS), allowing CI to fail if a conformance change bloated the component.
*   **👁️ UCP Visual Extension (`ucp/visual-v1`):** Link canonical stories to reference images. Enable automated visual regression testing (Chromatic/Percy) tied directly to UCP conformance fixtures.

---

## 🤝 How to Get Involved

UCP is an ambitious synthesis of static analysis, AI reasoning, and UI architecture. We need help across multiple domains:

*   **Rustaceans:** Help build the `syn`/`swc` extraction parsers and the `ucp-harness-gpui` native accessibility checker.
*   **AI/ML Engineers:** Help refine the LLM prompts for behavioral extraction and the semantic equivalence algorithms for the Unification Agent.
*   **Design System Authors:** If you maintain a shadcn/ui port, run `ucp bootstrap` against your repo and tell us where the AI got it wrong.
*   **Accessibility Specialists:** Help us define the strict side-effect rules for Gold-tier state machine transitions.

---

## 📐 Roadmap Principles

1.  **Extraction over Imagination:** We never invent props. If the AI hallucinates a behavior, it is a bug in the pipeline, not a feature of the spec.
2.  **Audibility over Magic:** Every generated prop must trace back to a specific line in a specific GitHub repo via the Unification Graph.
3.  **Profiles over Forks:** We do not maintain separate specs for Web and Desktop. We maintain one spec with scoped profiles.
4.  **AI-First, Human-Vetoed:** The LLM does the heavy lifting of cross-framework translation, but human Spec Editors hold the final merge key.

---

<div align="center">
  <sub>The future of UI components is computed, not written.</sub>
</div>
