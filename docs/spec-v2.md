# Universal Component Protocol (UCP) – Vision & Strategic Alignment v2.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Vision & Strategic Alignment |
| Version | 2.0 |
| Date | 2026-04-21 |
| Author | System Architect (assisted by AI) |
| Status | Draft — Pending Review |
| Supersedes | Vision v1.0 |

**Change Summary (v1.0 → v2.0):** Incorporates findings from deep analysis of three real-world shadcn/ui ports (Dioxus, GPUI desktop, Leptos). Key revisions: multi-platform rendering target model, compound component groups, enriched type system for signal-based reactivity, desktop accessibility profiles, spec extraction tooling, compile-time conformance checking, modular crate-level specs, and integration with existing test infrastructure.

---

## Vision Statement

To establish a **Universal Component Protocol (UCP)**—a language-agnostic, machine-readable specification for component APIs and behaviors—that enables any framework or platform to implement a consistent set of UI components. UCP serves as the single source of truth for component contracts across **both web and native desktop rendering targets**, empowering tooling to generate idiomatic, framework-native implementations from React to Leptos to Dioxus to GPUI to SwiftUI to Jetpack Compose. UCP goes beyond surface-level API alignment to model compound component relationships, signal-based reactivity semantics, and platform-appropriate accessibility requirements.

## Elevator Pitch (Moore's Template)

**For** library maintainers, framework authors, and cross-platform tooling developers  
**who are frustrated** by the fragmented and inconsistent shadcn/ui ports across React, Vue, Svelte, Solid, Leptos, Dioxus, GPUI, SwiftUI, and Jetpack Compose—and by the impossibility of a single conformance model that works for both web DOM and native desktop renderers,  
**our initiative** defines a universal, machine-readable specification for component APIs, behaviors, compound structures, and platform-specific accessibility,  
**that provides** a clear, achievable target for any implementation to converge toward, with tiered conformance levels (per platform profile), a shared test suite, automatic spec extraction from existing code, and compile-time conformance verification.  
**Unlike** ad-hoc integration efforts, framework-specific ports, or web-only accessibility standards,  
**our approach** models the full reality of modern UI component libraries—including compound components, signal-based reactivity, builder-pattern APIs, and native desktop accessibility APIs—enabling tooling that generates idiomatic code for any target while respecting each platform's architectural constraints.

## Problem Statement & Business Context

**Current Reality:**
- shadcn/ui has become a de facto standard for component design, with thriving ports across web frameworks (React, Vue, Svelte, Solid) and Rust UI systems (Leptos, Dioxus, GPUI), plus native platforms (SwiftUI, Jetpack Compose, Flutter).
- Each port has diverged in component inventory, prop naming, event signatures, reactivity semantics, and accessibility completeness.
- **Web-only assumptions** pervade existing cross-framework efforts. Desktop renderers like GPUI have no DOM, no ARIA attributes, and use fundamentally different component models (builder patterns, entity-based state, `RenderOnce` elements).
- **Compound component patterns** (e.g., `Dialog` + `DialogTrigger` + `DialogContent`) are ubiquitous across all ports but are modeled as separate components sharing context—not as a parent with named slots. No existing specification model captures this.
- **Signal-based reactivity** (Leptos `Signal<T>`, `MaybeProp<T>`, Dioxus `Signal<T>`, GPUI `Model<T>`) is the dominant state management pattern in Rust UI frameworks, but no specification defines how "controlled" vs "uncontrolled" maps to signal semantics.
- **Type expressiveness gaps**: Real-world props include `Option<Callback<MouseEvent>>`, `Signal<CheckboxState>`, `ButtonVariant` (enums), and `Option<RenderFn>`—none of which map cleanly to a simplistic `string | boolean | function` type system.
- Developers who work across multiple frameworks or maintain cross-framework tools (e.g., Quoin UCP) must manage bespoke mappings for each target, with no way to automatically extract a spec from an existing library.

**Opportunity:** By defining a **Universal Component Protocol** that models the full complexity of real component libraries—across web and desktop, with rich type systems, compound component groups, and platform-appropriate accessibility—we can:
- Reduce integration effort from *O(n)* per-framework mappings to a single, spec-driven generator that understands each platform's constraints.
- Enable **automatic spec extraction** from existing libraries, eliminating the manual authoring bottleneck.
- Provide **compile-time conformance verification** via proc-macros for Rust implementations, catching drift before tests run.
- Improve component quality and accessibility across all ports through shared testing that understands platform differences.
- Create a virtuous cycle: libraries extract specs → conformance is measured → gaps are identified → libraries converge → tooling improves.

## Target Users & Customers

**Primary:**
- **Library Maintainers** (Rust: Leptos, Dioxus, GPUI; TypeScript: React, Vue, Svelte; Native: Swift, Kotlin) – Seek to align with community standards, automatically extract specs from existing code, improve component coverage, and benefit from compile-time and runtime conformance checking.
- **Cross-Platform Tooling Developers** (e.g., Quoin UCP) – Need a stable, predictable component contract across all frameworks *and rendering targets*.
- **Framework Authors** – Want to offer shadcn/ui compatibility out-of-the-box with minimal manual porting.

**Secondary:**
- **UI Developers** – Benefit from consistent component APIs and documentation across all frameworks they use.
- **Design System Teams** – Use UCP as the canonical source for component definitions across multi-framework, multi-platform stacks.
- **Accessibility Specialists** – Ensure accessibility requirements are specified per-platform (ARIA for web, native APIs for desktop/mobile).

## User Needs & Value Proposition

| Need | Description |
|------|-------------|
| **Universal Consistency** | Identical component names, prop names, and event names across all implementations. |
| **Multi-Platform Awareness** | The spec distinguishes between web (DOM-based) and native desktop rendering targets, with platform-appropriate conformance criteria. |
| **Compound Component Modeling** | Component groups (e.g., `Dialog` family) are modeled as related components with shared context, not as a parent with slots. |
| **Rich Type System** | Props can express signals (`Signal<T>`), callbacks with signatures (`Callback<MouseEvent, ()>`), enums with variants, and nullable wrappers (`MaybeProp<T>`). |
| **Clear Reactivity Semantics** | A definition of controlled/uncontrolled/static that maps cleanly to signal-based, entity-based, and hook-based state models across frameworks. |
| **Spec Extraction** | Tooling can parse existing source code (Rust `#[component]` functions, TypeScript React components) to produce a draft UCP spec automatically. |
| **Compile-Time Verification** | Rust implementations can use a proc-macro to verify conformance at compile time, before any test runs. |
| **Tiered Adoption Path** | Bronze (API names match), Silver (reactive behavior correct per platform), Gold (full accessibility per platform profile + keyboard interactions). |
| **Conformance Validation** | A shared test suite with platform-specific harness adapters validates implementations. Existing test infrastructure can be leveraged via adapter plugins. |
| **Modular Spec Support** | Per-crate/per-package spec files can be aggregated into a composite specification, matching real-world modular library structures. |
| **Tooling Integration** | The spec serves as a single source of truth for code generation, documentation, migration tools, and conformance badges. |

## Desired Outcomes & Success Metrics

| ID | Outcome | Key Results |
|----|---------|-------------|
| **G-1** | **API Convergence** | 100% of components in the UCP spec achieve Bronze conformance in at least 5 major framework ports (spanning web and desktop) within 12 months. |
| **G-2** | **Component Parity** | 100% of components in the UCP spec are implemented in at least 5 framework ports within 18 months. |
| **G-3** | **Tooling Simplification** | Quoin's `quoin_render!` macro reduces framework-specific code by at least 80% using UCP as its source of truth. |
| **G-4** | **Conformance Pass Rate** | All participating libraries achieve Gold conformance for the core 15 components within 12 months (per their platform profile). |
| **G-5** | **Community Adoption** | The UCP spec is referenced in the package metadata of at least 5 major ports within 12 months. |
| **G-6** | **Spec Extraction Adoption** | At least 3 major ports use `ucp extract` to generate their initial spec, reducing manual authoring effort by ≥70%. |
| **G-7** | **Compile-Time Verification** | At least 2 Rust implementations adopt the `#[ucp_conform]` proc-macro for compile-time conformance checking within 12 months. |

## Strategic Constraints

- **No Rewrites:** Existing libraries remain the canonical implementations; changes are incremental and follow a defined deprecation policy.
- **Multi-Platform, Not Web-Only:** The specification must define conformance criteria for both web (DOM/ARIA) and native desktop (platform accessibility APIs) rendering targets. Neither is subordinate.
- **Language-Agnostic Core, Platform-Aware Extensions:** The core specification defines logical props, types, and behaviors without dictating concrete types. Platform-specific concerns (ARIA vs native a11y, signal vs hook reactivity) are encapsulated in **Platform Profiles** and **Framework Mapping Documents**.
- **Respect Existing Architectures:** The spec must accommodate builder-pattern APIs (GPUI), function-component props (Leptos/Dioxus/React), and declarative markup (SwiftUI/Compose) without forcing any single paradigm.
- **Accessibility:** WCAG 2.1 AA compliance (via ARIA) is required for web targets. Native accessibility API compliance is required for desktop/mobile targets. Both are Gold-tier requirements within their respective platform profiles.
- **Backward Compatibility:** Within a major version, no breaking changes without a defined deprecation path.

## Goals and Non-Goals

**Goals:**
- Define a common, machine-readable API specification for all shadcn/ui components (50+), including reactivity categories, compound component groups, rich type signatures, and platform-specific accessibility requirements.
- Define a tiered conformance model (Bronze/Silver/Gold) with **platform profiles** (Web, Desktop, Mobile) allowing incremental alignment within each target environment.
- Model compound component patterns (component groups with shared context) as first-class spec citizens.
- Provide a **spec extraction tool** (`ucp extract`) that can parse existing Rust/TypeScript source to produce draft specs automatically.
- Provide a **compile-time conformance proc-macro** (`#[ucp_conform]`) for Rust implementations.
- Provide a shared conformance test suite with a `ComponentHarness` trait that supports both web DOM introspection and desktop/native introspection (via registration-time manifests).
- Enable harness adapters that **integrate with existing test infrastructure** (Playwright, axe-core, platform test frameworks) rather than replacing them.
- Support **modular spec files** (per-crate/per-package) that aggregate into a composite specification.
- Enable tooling to use the spec to generate idiomatic code, documentation, migration scripts, and conformance badges.

**Non-Goals:**
- Creating a new shadcn/ui implementation from scratch.
- Changing the visual design, CSS, or styling of existing components.
- Unifying the internal architecture of implementations.
- Defining design tokens or theming specifications (reserved for a future UCP Theming Extension).
- Visual regression testing (reserved for a future Gold+ tier or separate UCP Visual Extension).
- Runtime performance benchmarking as a conformance criterion.

## Operational Concept & High-Level Scenarios

**Scenario A: Spec Extraction from Existing Library**
1. A maintainer of `leptos-shadcn-ui` runs `ucp extract --source ./leptos-shadcn-button/src/lib.rs --format leptos`.
2. The CLI parses the `#[component]` function, extracts prop names, types (including `Signal<T>`, `MaybeProp<T>`, `Callback<T>`), defaults, and enum variants.
3. It outputs a draft `UcpSpec` JSON for the Button component, with all types correctly mapped to UCP's enriched type system.
4. The maintainer reviews, curates, and commits the spec alongside the component.

**Scenario B: Cross-Platform Code Generation (Web + Desktop)**
1. A developer runs `ucp generate button --target leptos` or `--target gpui`.
2. The UCP CLI reads the JSON schema for Button, consults the target's Framework Mapping Document, and selects the appropriate platform profile.
3. For Leptos: outputs a `#[component]` function with `Signal<T>` props and `Callback<T>` event handlers.
4. For GPUI: outputs a builder-pattern API with `Disableable`, `ButtonVariants` traits, and GPUI-typed event callbacks.
5. Both outputs are idiomatic to their framework.

**Scenario C: Compound Component Conformance Testing**
1. The conformance test for `Dialog` specifies a **component group**: `[Dialog, DialogTrigger, DialogContent, DialogOverlay, DialogTitle]`.
2. The harness renders the full compound structure (not just `Dialog` alone).
3. Gold-tier checks verify that `DialogContent` has `aria-labelledby` pointing to `DialogTitle`'s ID, and that keyboard interactions (Escape to close, focus trap) work across the group.

**Scenario D: Compile-Time Conformance**
1. A Leptos developer adds `#[ucp_conform(spec = "button.json")]` above their `Button` component function.
2. At compile time, the proc-macro reads `button.json`, compares the function signature against the spec, and emits a warning if a required prop is missing or a type doesn't match the FMD mapping.
3. No runtime test needed for API-level conformance.

**Scenario E: Desktop Accessibility Conformance (GPUI)**
1. The GPUI harness for `Button` does not check `get_aria_role()` (no DOM).
2. Instead, it uses GPUI's native accessibility API to verify that the button element exposes the correct role (`AXButton` on macOS) and label via the platform accessibility tree.
3. The Gold tier passes for the "Desktop" platform profile, independent of web ARIA checks.

**Scenario F: Leveraging Existing Tests**
1. The `cloud-shuttle-leptos-shadcn-ui` library already has Playwright E2E tests with axe-core accessibility checks.
2. Instead of reimplementing these, the maintainer configures a **test adapter** that maps existing axe-core results to UCP Gold-tier conformance results.
3. `ucp test --adapter playwright-axe` reads the existing test output and produces a UCP conformance report.

**Scenario G: Spec Evolution via SEP**
1. A maintainer from the GPUI port proposes adding a `hapticFeedback` prop to Button (desktop-specific).
2. They open a Spec Enhancement Proposal (SEP) with a JSON Patch and mark it as targeting the "Desktop" platform profile only.
3. The proposal is reviewed by Spec Editors from at least three different language ecosystems.
4. If approved, the spec is updated; web implementations are unaffected; desktop implementations update their FMD mappings.

## Stakeholders, Sponsorship, and Governance

| Role | Responsible |
|------|-------------|
| **Executive Sponsors** | Core maintainers of Leptos-shadcn/ui, Dioxus-shadcn/ui, gpui-component, and leading web ports (React, Vue) |
| **Spec Editors** | Designated community members (rotating, one per major language ecosystem: JS/TS, Rust-web, Rust-desktop, Swift/Kotlin) |
| **Platform Profile Maintainers** | Designated experts for Web (ARIA), Desktop (macOS/Windows/Linux accessibility), Mobile (iOS/Android accessibility) |
| **Quoin Maintainer** | Provides UCP integration requirements |
| **Decision Model** | Changes to the spec require approval via a **Spec Enhancement Proposal (SEP)** process, with at least one maintainer from three different language ecosystems approving. Platform-profile-specific changes require approval from the relevant profile maintainer. |

## Risks, Assumptions, and Open Questions

**Assumptions:**
- Library maintainers across languages and platforms are willing to align their APIs incrementally.
- Backward compatibility can be maintained during the transition using the defined deprecation policy.
- At least one reference harness per platform profile (Web: Leptos, Desktop: GPUI) can be built to validate the multi-platform model.
- Existing test infrastructure (Playwright, axe-core, platform test frameworks) can be adapter-plugged rather than replaced.

**Risks:**

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Multi-platform conformance model is too complex. | High | Medium | Platform profiles are opt-in extensions; web-only adoption is still valuable. |
| Rich type system creates mapping burden. | Medium | Medium | `ucp extract` automates the hardest part; FMDs provide per-framework mappings. |
| Desktop accessibility testing is platform-specific and hard to automate. | High | High | Gold tier for desktop starts with manual verification; automated checks added incrementally. |
| Compound component group modeling doesn't fit all frameworks. | Medium | Low | Group membership is advisory; components can also be tested individually. |
| Proc-macro conformance adds compile-time overhead. | Low | Low | Optional feature; can be disabled in CI for speed. |
| Spec extraction produces inaccurate drafts. | Medium | Medium | Extraction is explicitly "draft" quality; human curation is expected. |

**Open Questions:**
- Q1: Should visual regression testing be a future Gold+ tier or a separate UCP Visual Extension? (Proposed: separate extension.)
- Q2: What is the appropriate deprecation timeline for breaking API changes? (Proposal: 2 minor versions / 6 months.)
- Q3: How should the spec model frameworks that use CSS-in-JS or Tailwind class props? (Deferred to theming extension.)
- Q4: Can a single implementation target multiple platform profiles (e.g., a Leptos component that renders both web and desktop via different backends)? (Proposal: yes, with per-profile conformance reports.)

---

*End of Vision & Strategic Alignment v2.0*

---

# Business & Stakeholder Requirements Specification v2.0
**Universal Component Protocol (UCP)**

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Business & Stakeholder Requirements Specification (BRS) |
| Version | 2.0 |
| Date | 2026-04-21 |
| Author | System Architect |
| Status | Draft — Pending Review |
| References | UCP Vision & Strategic Alignment v2.0 |

---

## 1. Introduction

### 1.1 Purpose
This BRS defines the high-level business objectives, stakeholder needs, and solution-bounding requirements for the **Universal Component Protocol (UCP)** initiative, incorporating findings from analysis of real-world shadcn/ui ports across web (Leptos, React) and native desktop (GPUI) rendering targets.

### 1.2 Scope
UCP defines a **language-agnostic, machine-readable specification** for the API surface and behavioral contracts of UI components commonly found in shadcn/ui ports. The scope includes:

- A formal schema describing component names, props (with rich type signatures including signals and callbacks), events, slots, compound component groups, ARIA/native accessibility requirements, and reactivity categories.
- **Platform profiles** (Web, Desktop, Mobile) that define platform-specific conformance criteria.
- A **tiered conformance model** (Bronze/Silver/Gold) scoped per platform profile.
- A **spec extraction tool** (`ucp extract`) that parses existing source code to produce draft specs.
- A **compile-time conformance proc-macro** (`#[ucp_conform]`) for Rust implementations.
- A **shared conformance test suite** with a `ComponentHarness` trait supporting both DOM introspection (web) and manifest-based introspection (desktop).
- **Test adapter plugins** that integrate with existing test infrastructure (Playwright, axe-core).
- **Framework Mapping Documents** that translate abstract types and reactivity categories into concrete framework idioms (including signal wrappers, builder patterns, callback signatures).
- **Modular spec support** (per-crate/per-package files aggregating into a composite spec).
- Tooling to generate code stubs, documentation, migration scripts, and conformance badges.
- An **implementation registry** for discovery and automated conformance reporting.

**Excludes:**
- Visual styling, CSS, design tokens, or theming (domain of a future UCP Theming Extension).
- Visual regression testing (domain of a future UCP Visual Extension).
- Internal implementation details or component architecture.
- Runtime performance benchmarking.

### 1.3 Document Conventions
- Requirements: **BR-XXX** (Business), **SR-XXX** (Stakeholder).
- RFC 2119 key words apply.

---

## 2. Business Context

### 2.1 Problem Statement
Analysis of three real-world shadcn/ui ports revealed systemic gaps in any specification that does not account for:

1. **Multi-platform rendering**: GPUI renders to a native element tree with no DOM, no ARIA attributes, and no `data-*` attributes. Conformance checking via DOM introspection is impossible.
2. **Compound component architecture**: Dioxus, Leptos, and React ports use separate components linked by context (e.g., `Dialog` + `DialogTrigger` + `DialogContent`), not parent-with-slots patterns.
3. **Signal-based reactivity**: Leptos uses `Signal<T>`, `MaybeProp<T>`, `RwSignal<T>`; Dioxus uses `Signal<T>`; GPUI uses `Entity<T>`/`Model<T>`. The controlled/uncontrolled/static model must map to these.
4. **Rich prop types**: Real props include `Option<Callback<MouseEvent>>`, `Signal<CheckboxState>`, `ButtonVariant` (enum), `Option<RenderFn>`—none of which map to a simplistic type system.
5. **Builder-pattern APIs**: GPUI uses method chaining (`.primary().label("Go").on_click(...)`) rather than named props.
6. **Modular crate structures**: `cloud-shuttle-leptos-shadcn-ui` publishes 46+ individual crates; a monolithic spec is unnatural.
7. **Manual spec authoring bottleneck**: No tool exists to extract a spec from existing code.
8. **Existing test infrastructure**: Libraries already have Playwright E2E tests, axe-core accessibility checks, and visual regression tests. Reimplementing these in UCP is wasteful.

### 2.2 Business Objectives

| ID | Objective | Success Criteria |
|----|-----------|------------------|
| **BO-1** | **API Convergence** | 5+ framework ports achieve Bronze conformance within 12 months. |
| **BO-2** | **Component Parity** | 50+ components fully specified and implemented in 5+ ports within 18 months. |
| **BO-3** | **Tooling Efficiency** | Quoin UCP reduces adapter code by ≥80%. |
| **BO-4** | **Conformance Visibility** | Public dashboard displays per-platform-profile conformance. |
| **BO-5** | **Community Adoption** | UCP referenced in package metadata of 5+ ports within 12 months. |
| **BO-6** | **Spec Extraction** | 3+ ports use `ucp extract` for initial spec generation within 12 months. |
| **BO-7** | **Compile-Time Verification** | 2+ Rust ports adopt `#[ucp_conform]` within 12 months. |
| **BO-8** | **Multi-Platform Support** | At least 1 desktop port (GPUI) and 1 web port (Leptos) achieve Silver conformance within 12 months. |

---

## 3. Stakeholders

| Stakeholder Group | Role & Interest |
|-------------------|-----------------|
| **Library Maintainers (Web)** | Leptos, Dioxus, React, Vue, Svelte, Solid ports — clear spec; spec extraction; compile-time checks; minimal breaking changes. |
| **Library Maintainers (Desktop)** | GPUI, Tauri+React ports — spec that accommodates builder patterns, entity-based state, native accessibility APIs. |
| **Library Maintainers (Mobile)** | SwiftUI, Compose, Flutter ports — platform-appropriate accessibility; declarative API mapping. |
| **Cross-Platform Tooling Developers** | Quoin UCP — stable contracts across web AND desktop; deterministic code gen for both paradigms. |
| **Framework Authors** | New Rust/Kotlin/Swift frameworks — reference implementations; FMD templates for their paradigm. |
| **UI Developers** | Consistent APIs; predictable behavior; accessible components. |
| **Design System Teams** | Multi-framework, multi-platform source of truth. |
| **Accessibility Specialists** | Platform-appropriate a11y specifications (ARIA for web, native APIs for desktop). |
| **Platform Profile Maintainers** | Experts who define and maintain Web/Desktop/Mobile conformance criteria. |

---

## 4. Business Requirements

### BR-001: Component API Specification with Rich Types
UCP SHALL define a machine-readable specification including component names, props (with enriched type signatures covering signals, callbacks with signatures, enums, and nullable wrappers), events, slots, compound component groups, and platform-specific accessibility requirements.

**Traceability:** BO-1, BO-2, BO-3, BO-8

### BR-002: Platform Profiles
UCP SHALL define platform profiles (Web, Desktop, Mobile) that specify platform-specific conformance criteria, accessibility standards, and introspection strategies.

**Traceability:** BO-4, BO-8

### BR-003: Tiered Conformance Model (Per Profile)
UCP SHALL define Bronze/Silver/Gold tiers scoped per platform profile, allowing an implementation to report separate conformance for web vs desktop targets.

**Traceability:** BO-1, BO-4, BO-8

### BR-004: Compound Component Groups
UCP SHALL model compound component patterns (e.g., Dialog family) as component groups with defined inter-component relationships and shared context requirements.

**Traceability:** BO-2, BO-4

### BR-005: Spec Extraction Tooling
UCP SHALL provide a `ucp extract` tool that parses existing source code (Rust `#[component]` functions, TypeScript React components) to produce draft UCP spec entries automatically.

**Traceability:** BO-6

### BR-006: Compile-Time Conformance Verification
UCP SHALL provide a Rust proc-macro (`#[ucp_conform]`) that verifies component function signatures against a UCP spec at compile time.

**Traceability:** BO-7

### BR-007: Shared Conformance Test Suite
UCP SHALL provide a reusable test suite with a `ComponentHarness` trait that supports both DOM-based introspection (web) and manifest-based introspection (desktop), plus compound group rendering.

**Traceability:** BO-4

### BR-008: Test Adapter Plugins
UCP SHALL support adapter plugins that integrate with existing test infrastructure (Playwright, axe-core, platform test frameworks) to produce conformance reports without reimplementing tests.

**Traceability:** BO-4, BO-6

### BR-009: Modular Spec Support
UCP SHALL support per-crate/per-package spec files that can be aggregated into a composite specification for tooling consumption.

**Traceability:** BO-2, BO-5

### BR-010: Public Conformance Dashboard
UCP SHALL maintain a public dashboard displaying per-profile conformance status for all participating implementations.

**Traceability:** BO-4, BO-5

### BR-011: Tooling Ecosystem
UCP SHALL enable code generators, documentation generators, migration assistants, and badge generators to consume the spec and produce framework-specific outputs (including builder-pattern APIs for desktop frameworks).

**Traceability:** BO-3

### BR-012: Governance and Evolution
UCP SHALL operate under a documented SEP process with cross-ecosystem approval, including platform-profile-scoped changes.

**Traceability:** BO-5

### BR-013: Implementation Registry
UCP SHALL define a registry format and discovery mechanism allowing implementations to publish specs and conformance reports for automatic dashboard aggregation.

**Traceability:** BO-4, BO-5

---

## 5. Stakeholder Requirements

### 5.1 Specification Format and Structure

**SR-001: Machine-Readable Schema**
The specification SHALL be provided in JSON Schema format with stable identifiers. The schema SHALL support:
- Component definitions with enriched prop types (signals, callbacks, enums, nullable wrappers).
- Compound component group definitions.
- Platform profile scoping.
- Modular spec file references.

**Traceability:** BR-001, BR-004, BR-009

**SR-002: Enriched Prop Type System**
For each component prop, the spec SHALL define the type using an enriched type model that includes:
- Primitive types: `string`, `boolean`, `number`, `integer`
- `enum { values: [...] }` with typed variant names
- `signal { inner: PropType }` for reactive signal wrappers
- `callback { params: PropType[], return: PropType }` for typed event handlers
- `nullable { inner: PropType }` for `Option<T>` / `MaybeProp<T>` patterns
- `node` for children / rendered content
- `array { items: PropType }` and `object { properties: {...} }`
- `any` as escape hatch

**Traceability:** BR-001

**SR-003: Reactivity Categories (Extended)**
The spec SHALL define reactivity categories that account for signal-based frameworks:
- **Controlled**: Value managed externally via a writable signal or equivalent; updates propagate immediately.
- **Uncontrolled**: Initial value provided; component manages internal state; external updates are ignored or merged.
- **Static**: Value never changes after mount; no reactivity.
- **MaybeSignal**: Value can be either static or a signal; component must handle both cases.
- **EntityBacked**: Value backed by a framework entity/model (GPUI-specific); changes to the entity propagate.

**Traceability:** BR-001, BR-008

**SR-004: Compound Component Groups**
The spec SHALL define component groups as:
```json
{
  "componentGroups": {
    "dialog": {
      "components": ["dialog", "dialog-trigger", "dialog-content", "dialog-overlay", "dialog-title", "dialog-close"],
      "requiredParts": ["dialog", "dialog-content"],
      "contextShared": true,
      "interComponentConstraints": [
        { "from": "dialog-content", "attribute": "aria-labelledby", "references": "dialog-title" }
      ]
    }
  }
}
```

**Traceability:** BR-004

**SR-005: Platform Profiles**
The spec SHALL define platform profiles:
- **Web**: DOM-based rendering; ARIA roles/attributes; keyboard events via DOM; introspection via `data-*` attributes or DOM queries.
- **Desktop**: Native element tree rendering; platform accessibility APIs (NSAccessibility, UI Automation, ATK); introspection via component manifests or platform API queries.
- **Mobile**: Platform-native rendering; platform accessibility APIs (iOS Accessibility, Android Accessibility); introspection via platform UI testing frameworks.

Each profile defines:
- Applicable accessibility standard
- Introspection strategy for the harness
- Keyboard interaction simulation method
- Conformance tier criteria modifications

**Traceability:** BR-002, BR-003

**SR-006: Structured Keyboard Interactions**
The spec SHALL define keyboard interactions as structured data, not prose:
```json
{
  "keyboardInteractions": [
    { "key": "Escape", "action": "close", "condition": "when open", "result": "dialog closes, focus returns to trigger" },
    { "key": "Enter", "action": "activate", "condition": "when trigger focused", "result": "dialog opens" }
  ]
}
```

**Traceability:** BR-001 (Gold tier), BR-002

### 5.2 Spec Extraction

**SR-007: Rust Source Extraction**
UCP SHALL provide a `ucp extract` command that parses Rust source files containing `#[component]` functions and extracts:
- Component name
- Prop names, types (mapped to UCP enriched types), defaults, required/optional
- Event handler types (mapped to `callback` type)
- Enum variant definitions from referenced types
- Doc comments as descriptions

**Traceability:** BR-005

**SR-008: TypeScript Source Extraction**
UCP SHALL provide extraction support for TypeScript React components (interface props, event handlers, default values).

**Traceability:** BR-005

**SR-009: Existing Documentation Extraction**
UCP SHALL support extracting spec data from structured Markdown API documentation files (as used by `cloud-shuttle-leptos-shadcn-ui`).

**Traceability:** BR-005, BR-006

### 5.3 Compile-Time Conformance

**SR-010: Proc-Macro Conformance Check**
UCP SHALL provide a `#[ucp_conform(spec = "path/to/spec.json")]` proc-macro attribute that:
- Reads the spec file at compile time
- Compares the annotated function's parameter types against the spec (using FMD type mappings)
- Emits compiler warnings for missing props, type mismatches, or extra props
- Supports an `allow_extra` option for implementations that extend the spec

**Traceability:** BR-006

### 5.4 Conformance Testing

**SR-011: Dual-Mode Component Harness**
The `ComponentHarness` trait SHALL support two introspection modes:
- **DOM Mode** (web): Render component, inspect DOM attributes, simulate events via web APIs.
- **Manifest Mode** (desktop): Register component metadata at initialization, verify rendering succeeds, inspect via platform accessibility APIs.

A single implementation MAY implement both modes for frameworks that target multiple platforms.

**Traceability:** BR-007

**SR-012: Component Manifest**
For manifest-mode harnesses, the spec SHALL define a `ComponentManifest` that declares:
- Exported prop names and their types (at registration time)
- Exported event handler names and their signatures
- Accessibility role and label (as declared by the component)

This allows conformance checking without runtime DOM introspection.

**Traceability:** BR-007

**SR-013: Compound Group Test Scenarios**
The conformance test suite SHALL include scenarios that render entire component groups and verify inter-component constraints (e.g., `aria-labelledby` references).

**Traceability:** BR-004, BR-007

**SR-014: Test Adapter Interface**
UCP SHALL define a `TestAdapter` trait that allows external test results to be mapped to conformance reports:
```rust
trait TestAdapter {
    fn parse_results(&self, test_output: &str) -> Vec<ConformanceCheckResult>;
    fn map_to_tier(&self, checks: &[ConformanceCheckResult]) -> ConformanceTier;
}
```

Built-in adapters SHALL include Playwright+axe-core and JUnit XML.

**Traceability:** BR-008

### 5.5 Modular Specs and Registry

**SR-015: Modular Spec Files**
UCP SHALL support a `ucp-manifest.json` that references individual component spec files:
```json
{
  "ucpVersion": "1.0.0",
  "components": {
    "button": "specs/button.json",
    "dialog": "specs/dialog.json"
  },
  "componentGroups": { ... }
}
```

**Traceability:** BR-009

**SR-016: Implementation Registry**
UCP SHALL define a registry format where implementations declare:
- Implementation ID, name, version, framework, platform profiles supported
- Spec file URL (or embedded spec)
- Conformance report endpoint URL
- CI webhook for automatic report submission

**Traceability:** BR-013

### 5.6 Governance

**SR-017: Platform-Scoped SEPs**
SEPs MAY target a specific platform profile. A SEP scoped to "Desktop" does not require approval from web-only maintainers, but MUST be reviewed by at least one Desktop profile maintainer.

**Traceability:** BR-012

---

## 6. Non-Functional Requirements

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR-001 | Spec parsing performance | Full catalogue (50+ components with enriched types) parsable in ≤100ms. |
| NFR-002 | Test suite execution | Core 15 components tested in ≤5 minutes per platform profile. |
| NFR-003 | Proc-macro compile overhead | `#[ucp_conform]` adds ≤2s to clean build time for a 50-component library. |
| NFR-004 | Dashboard availability | 99% uptime. |
| NFR-005 | Dashboard accessibility | WCAG 2.1 AA. |
| NFR-006 | Spec stability | Backward-compatible within major version. |
| NFR-007 | Security | Signed releases; dashboard sanitizes inputs. |

---

## 7. Constraints and Assumptions

**Constraints:**
- C-001: No rewrites of existing libraries.
- C-002: Framework-neutral core spec; platform specifics in profiles and FMDs.
- C-003: Platform independence: components remain implementable on their existing targets.
- C-004: MIT licensing.
- C-005: Desktop accessibility testing may start with manual verification; automation is incremental.

**Assumptions:**
- A-001: Maintainers are willing to adopt incrementally.
- A-002: At least one web harness (Leptos) and one desktop harness (GPUI) can be built as references.
- A-003: Existing test infrastructure can be adapter-plugged.

---

## 8. Traceability Matrix

| SR | BR(s) | BO(s) |
|----|-------|-------|
| SR-001 (Schema) | BR-001, BR-004, BR-009 | BO-1, BO-2, BO-3 |
| SR-002 (Rich Types) | BR-001 | BO-1, BO-3 |
| SR-003 (Reactivity) | BR-001 | BO-1, BO-8 |
| SR-004 (Compound Groups) | BR-004 | BO-2, BO-4 |
| SR-005 (Platform Profiles) | BR-002, BR-003 | BO-4, BO-8 |
| SR-006 (Keyboard Interactions) | BR-001 | BO-4 |
| SR-007 (Rust Extraction) | BR-005 | BO-6 |
| SR-008 (TS Extraction) | BR-005 | BO-6 |
| SR-009 (Doc Extraction) | BR-005 | BO-6 |
| SR-010 (Proc-Macro) | BR-006 | BO-7 |
| SR-011 (Dual Harness) | BR-007 | BO-4, BO-8 |
| SR-012 (Manifest) | BR-007 | BO-4, BO-8 |
| SR-013 (Group Tests) | BR-004, BR-007 | BO-4 |
| SR-014 (Test Adapters) | BR-008 | BO-4, BO-6 |
| SR-015 (Modular Specs) | BR-009 | BO-2, BO-5 |
| SR-016 (Registry) | BR-013 | BO-4, BO-5 |
| SR-017 (Scoped SEPs) | BR-012 | BO-5 |

---

## 9. Approval

| Role | Name | Signature | Date |
|------|------|-----------|------|
| Executive Sponsor | [TBD] | | |
| Product Lead | [TBD] | | |
| Spec Editor (JS/TS) | [TBD] | | |
| Spec Editor (Rust-Web) | [TBD] | | |
| Spec Editor (Rust-Desktop) | [TBD] | | |
| Spec Editor (Swift/Kotlin) | [TBD] | | |
| Platform Profile Lead (Web) | [TBD] | | |
| Platform Profile Lead (Desktop) | [TBD] | | |

---

*End of Business & Stakeholder Requirements Specification v2.0*

---

# Software Requirements Specification v2.0
**Universal Component Protocol (UCP)**

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Software Requirements Specification (SRS) |
| Version | 2.0 |
| Date | 2026-04-21 |
| Author | System Architect |
| Status | Draft — Pending Review |
| References | UCP Vision v2.0, UCP BRS v2.0 |

---

## 1. Introduction

### 1.1 Purpose
This SRS defines the functional and non-functional requirements for the UCP system, incorporating findings from analysis of real-world shadcn/ui ports including Dioxus (web), GPUI (native desktop), and Leptos (web with signal-based reactivity and modular crate structure).

### 1.2 System Components
1. **UCP Specification Schema** – JSON Schema with enriched types, compound groups, platform profiles, and modular file support.
2. **UCP Core Library** (`ucp-core`) – Rust crate providing data structures, validation, conformance logic, and proc-macro support.
3. **UCP CLI** (`ucp-cli`) – Commands: `validate`, `schema`, `extract`, `generate`, `test`, `doc`, `diff`, `lint`, `badge`, `sep`.
4. **Conformance Test Suite** – Framework-agnostic with dual-mode harness (DOM + manifest).
5. **Harness Implementations** – `ucp-harness-leptos` (web/DOM), `ucp-harness-gpui` (desktop/manifest).
6. **Test Adapters** – `ucp-adapter-playwright-axe` (web), `ucp-adapter-junit` (general).
7. **Conformance Dashboard** – Static site generator with per-profile matrix display.
8. **UCP Spec Viewer** – Leptos web app for interactive spec browsing.
9. **Proc-Macro Crate** (`ucp-conform`) – Compile-time conformance verification.
10. **Implementation Registry** – Discovery and automated report aggregation.

---

## 4. Functional Capabilities

### 4.1 UCP Specification Schema (Revised)

**SR-SPEC-001: Enriched Prop Type System**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum PropType {
    String,
    Boolean,
    Number,
    Integer,
    Enum { values: Vec<EnumVariant> },
    Signal { inner: Box<PropType> },
    WritableSignal { inner: Box<PropType> },
    MaybeSignal { inner: Box<PropType> },
    Callback { params: Vec<PropType>, return_type: Box<PropType> },
    Nullable { inner: Box<PropType> },
    Node,
    Array { items: Box<PropType> },
    Object { properties: BTreeMap<String, PropType> },
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EnumVariant {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}
```

**Verification:** Schema validation test; FMD mapping test for each target framework.

**SR-SPEC-002: Extended Reactivity Categories**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Reactivity {
    Controlled,
    Uncontrolled,
    Static,
    MaybeSignal,
    EntityBacked,
}
```

**SR-SPEC-003: Compound Component Groups**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComponentGroupSpec {
    pub name: String,
    pub components: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_parts: Option<Vec<String>>,
    #[serde(default)]
    pub context_shared: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inter_component_constraints: Option<Vec<InterComponentConstraint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InterComponentConstraint {
    pub from: String,
    pub attribute: String,
    pub references: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}
```

**SR-SPEC-004: Platform Profiles**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum PlatformProfile {
    Web,
    Desktop,
    Mobile,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilitySpec {
    pub profile: PlatformProfile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_role: Option<String>, // e.g., "AXButton" for macOS
    pub required_attributes: Vec<AccessibilityAttribute>,
    pub keyboard_interactions: Vec<KeyboardInteraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityAttribute {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardInteraction {
    pub key: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub expected_result: String,
}
```

**SR-SPEC-005: Component Spec with Profile-Scoped Accessibility**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSpec {
    pub name: String,
    pub description: String,
    pub category: ComponentCategory,
    pub since: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<DeprecationInfo>,
    pub props: Vec<PropSpec>,
    pub events: Vec<EventSpec>,
    pub slots: Vec<SlotSpec>,
    /// Accessibility requirements keyed by platform profile
    pub accessibility: BTreeMap<String, AccessibilitySpec>,
    /// Component groups this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<String>>,
}
```

**SR-SPEC-006: Modular Spec Manifest**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UcpManifest {
    pub ucp_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<BTreeMap<String, String>>, // name -> relative path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_groups: Option<BTreeMap<String, ComponentGroupSpec>>,
}
```

### 4.2 Spec Extraction

**SR-EXT-001: Rust Component Extraction**

The `ucp extract` command SHALL:
- Parse `.rs` files for functions annotated with `#[component]` (Leptos) or `#[component]`/`#[props]` (Dioxus).
- Extract parameter names, types, and attributes (`#[prop(default)]`, `#[prop(optional)]`, `#[prop(into)]`).
- Map Rust types to UCP enriched types:
  - `Signal<T>` → `Signal { inner: map(T) }`
  - `RwSignal<T>` → `WritableSignal { inner: map(T) }`
  - `MaybeProp<T>` → `MaybeSignal { inner: map(T) }`
  - `Callback<T>` / `EventHandler<T>` → `Callback { params: [map(T)], return: Unit }`
  - `Option<T>` → `Nullable { inner: map(T) }`
  - `Children` / `Option<Children>` → `Node`
  - Named enum types → `Enum { values: [...variant names...] }`
- Extract default values from `#[prop(default = "...")]` attributes.
- Extract doc comments as descriptions.
- Output a `UcpSpec` JSON fragment for each component.

**Verification:** Test extraction against known components from `cloud-shuttle-leptos-shadcn-ui` and `mbeliou-shadcn-dioxus`.

**SR-EXT-002: TypeScript React Extraction**

Parse TypeScript interfaces and `React.FC<Props>` declarations to extract prop types, event handlers (`onXxx`), and default values. Map TypeScript types to UCP enriched types.

**SR-EXT-003: Markdown API Doc Extraction**

Parse structured Markdown files (as used by `cloud-shuttle-leptos-shadcn-ui` docs) that contain prop tables, event tables, and accessibility sections. Extract into `UcpSpec` format.

**SR-EXT-004: Aggregate Extraction**

`ucp extract --source ./crate-root --format leptos --recursive` SHALL walk a directory tree, find all component files, extract individually, and produce a composite `UcpManifest` referencing per-component spec files.

### 4.3 Compile-Time Conformance

**SR-COMPILER-001: Proc-Macro Definition**

The `ucp-conform` crate SHALL provide:
```rust
#[ucp_conform(spec = "specs/button.json", fmd = "fmd/leptos.json")]
#[component]
pub fn Button(...) -> impl IntoView { ... }
```

Behavior:
1. Read spec file at compile time.
2. Read FMD file (or use default for detected framework).
3. For each prop in the spec: verify the function has a parameter whose type matches the FMD-mapped type.
4. For each function parameter: verify it exists in the spec (or is explicitly allowed via `#[ucp_conform(allow_extra)]`).
5. Emit `warning!()` for mismatches (not errors, to avoid breaking builds during migration).

**Verification:** Unit test with mock component; integration test against real Leptos component.

**SR-COMPILER-002: FMD-Aware Type Comparison**

The proc-macro SHALL use the FMD to resolve type mappings. For example, if the spec says `Signal { inner: String }` and the FMD maps `Signal { inner: T }` → `leptos::Signal<T>`, then the proc-macro checks for `Signal<String>` in the function signature.

### 4.4 Conformance Harness (Revised)

**SR-HARNESS-001: Dual-Mode Trait**

```rust
pub trait ComponentHarness {
    type Component;

    /// Which platform profile this harness targets
    fn platform_profile(&self) -> PlatformProfile;

    /// Introspection mode
    fn introspection_mode(&self) -> IntrospectionMode;

    /// List all UCP component names provided
    fn provided_components(&self) -> Vec<String>;

    /// Declare metadata for a component (manifest mode)
    fn component_manifest(&self, name: &str) -> Option<ComponentManifest>;

    /// Render a single component
    fn render(&mut self, name: &str, props: HashMap<String, serde_json::Value>) -> Self::Component;

    /// Render a component group (for compound component testing)
    fn render_group(&mut self, group_name: &str, components: HashMap<String, HashMap<String, serde_json::Value>>) -> Self::ComponentGroup;

    /// Inspect prop names (DOM mode: from rendered element; manifest mode: from manifest)
    fn get_prop_names(&self, component: &Self::Component) -> Vec<String>;

    /// Inspect event names
    fn get_event_names(&self, component: &Self::Component) -> Vec<String>;

    /// Update a prop value (for Silver/reactivity testing)
    fn update_prop(&mut self, component: &mut Self::Component, name: &str, value: serde_json::Value);

    /// Read current prop value
    fn get_prop_value(&self, component: &Self::Component, name: &str) -> Option<serde_json::Value>;

    /// Trigger an event
    fn trigger_event(&mut self, component: &mut Self::Component, name: &str, payload: Option<serde_json::Value>);

    /// Check accessibility role (profile-aware)
    fn get_accessibility_role(&self, component: &Self::Component, profile: &PlatformProfile) -> Option<String>;

    /// Check accessibility attribute (profile-aware)
    fn get_accessibility_attribute(&self, component: &Self::Component, attr: &str, profile: &PlatformProfile) -> Option<String>;

    /// Simulate keyboard event (profile-aware)
    fn send_keyboard_event(&mut self, component: &mut Self::Component, key: &str, event_type: &str, profile: &PlatformProfile);

    /// Verify inter-component constraint (e.g., aria-labelledby reference)
    fn verify_constraint(&self, group: &Self::ComponentGroup, constraint: &InterComponentConstraint) -> bool;

    fn cleanup(&mut self, component: Self::Component);
    fn cleanup_group(&mut self, group: Self::ComponentGroup);
}

#[derive(Debug, Clone, Copy)]
pub enum IntrospectionMode {
    Dom,      // Web: inspect rendered DOM
    Manifest, // Desktop: use declared metadata
}

pub struct ComponentManifest {
    pub prop_names: Vec<String>,
    pub prop_types: HashMap<String, String>,
    pub event_names: Vec<String>,
    pub declared_role: Option<String>,
}
```

**SR-HARNESS-002: Web DOM Harness (Leptos Reference)**

The `ucp-harness-leptos` SHALL implement `ComponentHarness` with `IntrospectionMode::Dom` and `PlatformProfile::Web`. It SHALL:
- Mount real Leptos components in a WASM browser environment.
- Create external `Signal<T>` / `RwSignal<T>` for Silver reactivity tests.
- Inspect DOM for ARIA attributes, `data-*` attributes, and element hierarchy.
- Simulate keyboard events via `web-sys` `KeyboardEvent`.
- Verify inter-component constraints by querying the DOM tree.

**SR-HARNESS-003: Desktop Manifest Harness (GPUI Reference)**

The `ucp-harness-gpui` SHALL implement `ComponentHarness` with `IntrospectionMode::Manifest` and `PlatformProfile::Desktop`. It SHALL:
- Use GPUI's test utilities to render components in a test window.
- Require implementations to register `ComponentManifest` metadata at initialization (a one-time declaration per component listing props, events, and accessibility role).
- Verify accessibility via GPUI's native accessibility API inspection (not ARIA).
- Simulate keyboard events via GPUI's input simulation.
- Verify inter-component constraints by inspecting the accessibility tree (e.g., that a content element's label points to a title element).

### 4.5 Conformance Tiers (Revised)

**SR-TIER-001: Bronze (API Names Match)**
- [Same as v1.0 but profile-aware]: Component, prop, and event names match the spec per FMD naming rules.
- For builder-pattern APIs (GPUI): the manifest's prop names correspond to builder methods.

**SR-TIER-002: Silver (Reactive Behavior Correct)**
- Controlled props: external signal update → component reflects new value (via DOM inspection or state readback).
- Uncontrolled props: initial default matches spec; external updates ignored.
- Static props: value never changes.
- MaybeSignal props: component handles both static and signal inputs.
- EntityBacked props: entity mutation → component reflects new value.

**SR-TIER-003: Gold (Accessibility + Keyboard + Compound Constraints)**
Per platform profile:
- **Web**: ARIA role and attributes present and correct; structured keyboard interactions pass; compound component constraints verified via DOM.
- **Desktop**: Native accessibility role and label present; keyboard interactions pass via platform simulation; compound constraints verified via accessibility tree.
- **Mobile**: Platform accessibility role and label present; keyboard/gesture interactions pass.

### 4.6 Test Adapters

**SR-ADAPTER-001: TestAdapter Trait**

```rust
#[async_trait]
pub trait TestAdapter {
    fn name(&self) -> &str;
    async fn parse_and_map(&self, test_output_path: &str, spec: &UcpSpec) -> AdapterResult;
}

pub struct AdapterResult {
    pub implementation_id: String,
    pub profile: PlatformProfile,
    pub component_results: BTreeMap<String, ComponentConformance>,
}
```

**SR-ADAPTER-002: Playwright+axe Adapter**

Parse Playwright test output (JSON) and axe-core results to map accessibility violations to UCP Gold-tier failures. Map passing axe checks to Gold-tier passes.

**SR-ADAPTER-003: JUnit XML Adapter**

Parse JUnit XML test results, mapping test case names (e.g., `button::bronze::prop_names`) to conformance checks.

### 4.7 Code Generation (Revised)

**SR-GEN-001: Builder-Pattern FMD Template**

For frameworks using builder patterns (GPUI), the FMD SHALL define:
```json
{
  "componentPattern": "builder",
  "builderTraits": {
    "variant": { "trait": "ButtonVariants", "methods": ["primary", "secondary", "outline"] },
    "size": { "trait": "Sizable", "methods": ["small", "medium", "large"] },
    "disabled": { "trait": "Disableable", "methods": ["disabled(bool)"] }
  }
}
```

The code generator SHALL produce trait implementations and builder-method chains instead of prop-based component functions.

**SR-GEN-002: Signal-Aware Code Generation**

For signal-based frameworks (Leptos, Dioxus), the generator SHALL use FMD reactivity mappings to produce correct prop types:
- `Controlled` + `Signal<String>` → `value: RwSignal<String>`
- `MaybeSignal` + `String` → `value: MaybeProp<String>`
- `Static` + `String` → `value: String`

### 4.8 Implementation Registry

**SR-REG-001: Registry Entry Format**

```json
{
  "id": "leptos-shadcn",
  "name": "Leptos shadcn/ui",
  "version": "0.5.0",
  "framework": "Leptos",
  "profiles": ["web"],
  "specUrl": "https://raw.githubusercontent.com/.../ucp-manifest.json",
  "reportUrl": "https://ucp.dev/api/reports/leptos-shadcn/latest",
  "repository": "https://github.com/...",
  "maintainer": "..."
}
```

**SR-REG-002: Registry Discovery**

The `ucp-cli` SHALL support `ucp registry list` and `ucp registry add <url>` commands. The dashboard SHALL automatically fetch reports from registered implementations.

---

## 5. Non-Functional Requirements

(All v1.0 NFRs apply, plus:)

**NFR-PROC-001:** The `#[ucp_conform]` proc-macro SHALL add ≤2 seconds to incremental compilation for a 50-component library.

**NFR-EXTRACT-001:** `ucp extract` SHALL process a 50-component Rust crate in ≤10 seconds on typical developer hardware.

**NFR-HARNESS-001:** The manifest-mode harness SHALL not require a graphical display or browser for desktop conformance testing.

---

## 6. Data Contracts

### 6.1 Enriched Component Spec Example

```json
{
  "ucpVersion": "2.0.0",
  "componentGroups": {
    "dialog": {
      "name": "dialog",
      "components": ["dialog", "dialog-trigger", "dialog-content", "dialog-overlay", "dialog-title", "dialog-close"],
      "requiredParts": ["dialog", "dialog-content"],
      "contextShared": true,
      "interComponentConstraints": [
        {
          "from": "dialog-content",
          "attribute": "aria-labelledby",
          "references": "dialog-title",
          "condition": "when dialog-title is present"
        }
      ]
    }
  },
  "components": {
    "button": {
      "name": "button",
      "description": "A clickable button element.",
      "category": "form",
      "since": "1.0.0",
      "props": [
        {
          "name": "variant",
          "type": { "kind": "enum", "values": [
            { "name": "default" },
            { "name": "destructive" },
            { "name": "outline" },
            { "name": "secondary" },
            { "name": "ghost" },
            { "name": "link" }
          ]},
          "reactivity": "static",
          "required": false,
          "default": "default",
          "description": "Visual variant of the button."
        },
        {
          "name": "disabled",
          "type": { "kind": "boolean" },
          "reactivity": "static",
          "required": false,
          "default": false,
          "description": "Whether the button is disabled."
        },
        {
          "name": "onClick",
          "type": { "kind": "callback", "params": [], "returnType": { "kind": "any" } },
          "reactivity": "static",
          "required": false,
          "description": "Click event handler."
        }
      ],
      "events": [],
      "slots": [
        { "name": "default", "required": true, "description": "Button label content." }
      ],
      "accessibility": {
        "web": {
          "profile": "web",
          "implicitRole": "button",
          "nativeRole": null,
          "requiredAttributes": [],
          "keyboardInteractions": [
            { "key": "Enter", "action": "activate", "expectedResult": "onClick handler fires" },
            { "key": "Space", "action": "activate", "expectedResult": "onClick handler fires" }
          ]
        },
        "desktop": {
          "profile": "desktop",
          "implicitRole": null,
          "nativeRole": "AXButton",
          "requiredAttributes": [
            { "name": "AXDescription", "condition": "when no visible label" }
          ],
          "keyboardInteractions": [
            { "key": "Enter", "action": "activate", "expectedResult": "onClick handler fires" },
            { "key": "Space", "action": "activate", "expectedResult": "onClick handler fires" }
          ]
        }
      },
      "memberOfGroups": []
    }
  }
}
```

### 6.2 Conformance Report (Revised)

```json
{
  "implementation": { "id": "leptos-shadcn", "name": "...", "version": "0.5.0", "framework": "Leptos" },
  "ucpSpecVersion": "2.0.0",
  "testedAt": "2026-04-21T10:30:00Z",
  "profile": "web",
  "results": {
    "button": {
      "component": "button",
      "tier": "silver",
      "bronze": { "passed": true, "failures": [] },
      "silver": { "passed": true, "failures": [] },
      "gold": { "passed": false, "failures": ["keyboard: Space key did not fire onClick"] }
    },
    "dialog": {
      "component": "dialog",
      "tier": "none",
      "bronze": { "passed": true, "failures": [] },
      "silver": { "passed": false, "failures": ["controlled open prop did not update"] },
      "gold": { "passed": false, "failures": ["inter-component: dialog-content missing aria-labelledby referencing dialog-title"] }
    }
  }
}
```

---

## 7. Risks and Open Issues

| TBD ID | Description | Owner | Status |
|--------|-------------|-------|--------|
| TBD-001 | Finalize GPUI native accessibility API inspection strategy (GPUI's accessibility support is evolving). | Desktop Profile Lead | Open |
| TBD-002 | Determine whether `EntityBacked` reactivity is GPUI-specific or generalizable. | Spec Editors | Open |
| TBD-003 | Evaluate proc-macro approach for non-Rust languages (TypeScript decorator? Build plugin?). | Spec Editors | Deferred |
| TBD-004 | Define exact Playwright+axe adapter mapping rules for Gold-tier. | Web Profile Lead | Open |
| TBD-005 | Determine if SwiftUI/Compose harnesses are in scope for v2.0 or deferred. | Spec Editors | Deferred |
| TBD-006 | Finalize component list for UCP v2.0 (target: 50+). | Spec Editors | In Progress |
| TBD-007 | Evaluate whether `ucp extract` should use `syn` (AST) or `cargo doc` JSON output as primary source. | Tooling Lead | Open |

---

## 8. Appendices

### Appendix A: Type Mapping Reference Table

| Rust Type | UCP PropType | Leptos FMD Native Type | Dioxus FMD Native Type | GPUI FMD Native Type |
|-----------|-------------|------------------------|----------------------|---------------------|
| `String` | `String` | `String` | `String` | `SharedString` |
| `bool` | `Boolean` | `bool` | `bool` | `bool` |
| `Signal<T>` | `Signal { inner: T }` | `Signal<T>` | `Signal<T>` | N/A (use Entity) |
| `RwSignal<T>` | `WritableSignal { inner: T }` | `RwSignal<T>` | `Signal<T>` | N/A |
| `MaybeProp<T>` | `MaybeSignal { inner: T }` | `MaybeProp<T>` | N/A | N/A |
| `Callback<T>` | `Callback { params: [T], return: Any }` | `Callback<T>` | `EventHandler<T>` | `Box<dyn Fn(&mut Window, &mut App, &T)>` |
| `Option<T>` | `Nullable { inner: T }` | `Option<T>` | `Option<T>` | `Option<T>` |
| `Children` | `Node` | `Children` | `Element` | `impl IntoElement` |
| `ButtonVariant` (enum) | `Enum { values: [...] }` | `ButtonVariant` | `ButtonVariant` | (builder methods) |
| `f64` | `Number` | `f64` | `f64` | `f64` |

### Appendix B: Reactivity Mapping Reference

| UCP Reactivity | Leptos Pattern | Dioxus Pattern | GPUI Pattern |
|----------------|---------------|----------------|-------------|
| Controlled | `RwSignal<T>` prop | `Signal<T>` prop | `Model<T>` entity |
| Uncontrolled | `MaybeSignal<T>` with internal `Signal` | `Signal<T>` with internal state | Not applicable (use Static) |
| Static | Plain `T` prop | Plain `T` prop | Plain `T` in builder |
| MaybeSignal | `MaybeProp<T>` | N/A (use Controlled or Static) | N/A |
| EntityBacked | N/A | N/A | `Entity<T>` / `Model<T>` |

---

*End of Software Requirements Specification v2.0*

---

# Revised UCP Implementation Plan (17 Chunks)

> **For agentic workers:** Use `superpowers:subagent-driven-development` or `superpowers:executing-plans`. Steps use checkbox (`- [ ]`) syntax.

**Changes from v1.0 plan:** Chunks 1-3 revised for enriched type system. Chunk 4 split into separate Silver/Gold with profile awareness. Chunk 5 revised for DOM-mode harness with signal support. **New Chunk 5.5: GPUI Desktop Harness.** Chunk 7 revised for builder-pattern generation. **New Chunk 7.5: Spec Extraction Tool.** **New Chunk 8.5: Compile-Time Proc-Macro.** **New Chunk 9.5: Test Adapter Plugins.** Chunk 10 revised for per-profile badges. Chunks 11-13 updated for modular specs and registry.

---

## Chunk 1: Workspace and Enriched Core Types

**Goal:** Establish workspace with `ucp-core` containing the v2.0 data structures (enriched `PropType`, compound groups, platform profiles, structured keyboard interactions, modular manifest).

### Task 1: Initialize workspace (same as v1.0 Task 1)

- [ ] Create workspace `Cargo.toml`, `ucp-core/Cargo.toml`, `ucp-cli/Cargo.toml`
- [ ] Dependencies: `serde 1.0`, `serde_json 1.0`, `schemars 1.2`, `thiserror 2.0`
- [ ] Dev-deps: `rstest 0.24`, `quickcheck 1.0`
- [ ] Verify: `cargo build`

### Task 2: Error types (same as v1.0 Task 2)

- [ ] Write `ucp-core/src/error.rs` with `thiserror` enum
- [ ] Verify: `cargo build --package ucp-core`

### Task 3: Enriched specification structs

- [ ] Write `ucp-core/src/spec.rs` with:
  - `PropType` enum (all variants from SR-SPEC-001 including `Signal`, `WritableSignal`, `MaybeSignal`, `Callback`, `Nullable`, `Enum` with `EnumVariant`)
  - `Reactivity` enum (all 5 variants from SR-SPEC-002)
  - `ComponentGroupSpec`, `InterComponentConstraint` (SR-SPEC-003)
  - `PlatformProfile`, `AccessibilitySpec`, `AccessibilityAttribute`, `KeyboardInteraction` (SR-SPEC-004)
  - `ComponentSpec` with `accessibility: BTreeMap<String, AccessibilitySpec>` and `member_of_groups` (SR-SPEC-005)
  - `UcpManifest` with modular file references (SR-SPEC-006)
  - `UcpSpec` as before but with `component_groups: BTreeMap<String, ComponentGroupSpec>`
- [ ] Write test: `ucp-core/tests/spec_deser.rs` parsing a minimal spec with enriched types
- [ ] Verify: `cargo test --package ucp-core`

### Task 4: Validation logic (enhanced)

- [ ] Write `ucp-core/src/validation.rs` with:
  - Duplicate prop/event/slot detection (as v1.0)
  - Component group integrity: verify all referenced components exist in `components`
  - Inter-component constraint: verify `from` and `references` components exist
  - Accessibility spec: verify profile keys are valid `PlatformProfile` values
  - Keyboard interaction: verify `key` is non-empty
- [ ] Write tests for each validation rule
- [ ] Verify: `cargo test --package ucp-core`

### Task 5: JSON Schema generation

- [ ] Write `ucp-core/tests/schema_generation.rs` verifying `schemars::schema_for!(UcpSpec)` produces valid JSON Schema
- [ ] Commit all

---

## Chunk 2: CLI Validate and Schema (Enhanced)

**Goal:** Implement `validate` and `schema` commands that work with the enriched spec format.

### Task 1: Clap skeleton with all planned subcommands

- [ ] Define `Commands` enum with: `Validate`, `Schema`, `Extract`, `Generate`, `Test`, `Doc`, `Diff`, `Lint`, `Badge`, `Sep`
- [ ] Stub each with `todo!()` except `Validate` and `Schema`

### Task 2: Implement `validate`

- [ ] Load spec (try `UcpSpec`, fallback to `UcpManifest` and load modular files)
- [ ] Run validation
- [ ] Test with valid and invalid specs (including group integrity failures)

### Task 3: Implement `schema`

- [ ] Generate JSON Schema from enriched types
- [ ] Test output

### Task 4: Integration tests

- [ ] Add `assert_cmd` tests for both commands
- [ ] Commit

---

## Chunk 3: Harness Trait and Bronze Conformance (Revised)

**Goal:** Define the dual-mode `ComponentHarness` trait with profile awareness and implement Bronze conformance.

### Task 1: Define harness trait

- [ ] Write `ucp-core/src/harness.rs` with:
  - `IntrospectionMode` enum (`Dom`, `Manifest`)
  - `ComponentManifest` struct
  - `ComponentHarness` trait (full signature from SR-HARNESS-001)
  - Default no-op implementations for all methods
- [ ] Update `lib.rs`

### Task 2: Bronze conformance (profile-aware)

- [ ] Write `ucp-core/src/conformance/bronze.rs`:
  - `check_bronze` accepts `PlatformProfile` parameter
  - In `Dom` mode: render component, inspect DOM for prop/event names
  - In `Manifest` mode: use `component_manifest()` to check names
  - For builder-pattern frameworks: manifest mode maps prop names to builder methods
- [ ] Write tests with mock harness (both modes)

### Task 3: Report structures (revised)

- [ ] Write `ucp-core/src/report.rs` with `profile: PlatformProfile` field in `ConformanceReport`
- [ ] Test serialization
- [ ] Commit

---

## Chunk 4: Silver Conformance (Signal-Aware)

**Goal:** Implement Silver conformance that correctly tests reactivity across signal-based and entity-based frameworks.

### Task 1: Silver checks with reactivity mapping

- [ ] Write `ucp-core/src/conformance/silver.rs`:
  - `Controlled`: `update_prop` → verify `get_prop_value` reflects change
  - `Uncontrolled`: verify default; verify `update_prop` does NOT change value
  - `Static`: verify `update_prop` does NOT change value
  - `MaybeSignal`: test with both static value and signal input
  - `EntityBacked`: verify entity mutation propagates (harness-specific)
- [ ] Each check logs which reactivity category it tested and the result

### Task 2: Full conformance runner

- [ ] Write `ucp-core/src/conformance/mod.rs` with `run_conformance` that chains Bronze → Silver → Gold (skipping tiers based on previous results and profile)

### Task 3: Tests with mock harness

- [ ] Test each reactivity category with appropriate mock
- [ ] Commit

---

## Chunk 5: Gold Conformance (Profile-Aware Accessibility)

**Goal:** Implement Gold conformance with structured keyboard interactions and profile-aware accessibility checks.

### Task 1: Gold accessibility checks

- [ ] Write `ucp-core/src/conformance/gold.rs`:
  - Accept `PlatformProfile` parameter
  - `Web` profile: check `implicit_role` via `get_accessibility_role`, check `required_attributes` via `get_accessibility_attribute`
  - `Desktop` profile: check `native_role` via `get_accessibility_role`, check attributes via native API
  - Attribute checks support `condition` field (skip check if condition not met)

### Task 2: Structured keyboard interaction testing

- [ ] For each `KeyboardInteraction` in the spec:
  - Check `condition` (if present) — if not met, skip
  - Call `send_keyboard_event` with the specified `key`
  - Verify `expected_result` (best-effort: for now, verify the event was dispatched without error; full result verification requires framework-specific assertions)

### Task 3: Compound component constraint verification

- [ ] For each `InterComponentConstraint` in the component group:
  - Call `verify_constraint` on the rendered group
  - Report pass/fail

### Task 4: Tests

- [ ] Test with mock harness implementing both profiles
- [ ] Commit

---

## Chunk 5.5: Leptos Web Harness (Reference, Signal-Aware)

**Goal:** Build a real DOM-mode harness for Leptos that supports signal creation, external signal updates, and DOM introspection.

### Task 1: Scaffold `ucp-harness-leptos`

- [ ] Create crate with Leptos 0.7, wasm-bindgen, web-sys deps
- [ ] Implement `ComponentHarness` with `IntrospectionMode::Dom` and `PlatformProfile::Web`

### Task 2: Signal-aware rendering

- [ ] `render` method: accept `serde_json::Value` props, create Leptos `Signal<T>` / `RwSignal<T>` as needed based on FMD reactivity mapping
- [ ] Store signal handles for later `update_prop` / `get_prop_value` calls
- [ ] `update_prop`: update the stored signal's value
- [ ] `get_prop_value`: read the current signal value

### Task 3: DOM introspection

- [ ] `get_prop_names`: inspect rendered element's `data-ucp-props` attribute
- [ ] `get_accessibility_role`: inspect `role` attribute
- [ ] `get_accessibility_attribute`: inspect `aria-*` attributes
- [ ] `send_keyboard_event`: dispatch `KeyboardEvent` via `web-sys`

### Task 4: Compound group rendering

- [ ] `render_group`: mount full Leptos component tree (e.g., `Dialog` with children)
- [ ] `verify_constraint`: query DOM tree for attribute references

### Task 5: Tests

- [ ] WASM tests for Bronze/Silver/Gold with a real Button component
- [ ] Commit

---

## Chunk 5.7: GPUI Desktop Harness (Reference, Manifest-Mode)

**Goal:** Build a manifest-mode harness for GPUI that demonstrates desktop conformance without DOM.

### Task 1: Scaffold `ucp-harness-gpui`

- [ ] Create crate with gpui dev-dependencies
- [ ] Implement `ComponentHarness` with `IntrospectionMode::Manifest` and `PlatformProfile::Desktop`

### Task 2: Component manifest registration

- [ ] Define a `GpuiComponentRegistry` where implementations register `ComponentManifest` for each component
- [ ] `component_manifest`: look up from registry
- [ ] `provided_components`: list from registry

### Task 3: Rendering and entity-based reactivity

- [ ] `render`: use GPUI test utilities to open a test window and render the component
- [ ] For `EntityBacked` reactivity: create a GPUI `Model<T>`, pass to component, mutate model, observe render
- [ ] `update_prop`: for builder-pattern APIs, re-render with modified builder chain

### Task 4: Desktop accessibility inspection

- [ ] `get_accessibility_role`: use GPUI's accessibility API to query the native role
- [ ] `get_accessibility_attribute`: query native accessibility attributes
- [ ] `send_keyboard_event`: use GPUI's input simulation

### Task 5: Tests

- [ ] Native tests (not WASM) for Bronze with manifest
- [ ] Silver tests with entity-based reactivity
- [ ] Commit

---

## Chunk 6: Conformance Dashboard (Per-Profile)

**Goal:** Static site generator displaying separate matrices per platform profile.

### Task 1: Report aggregation (enhanced)

- [ ] Load reports tagged with `profile` field
- [ ] Group by profile; generate separate matrix per profile
- [ ] Show profile selector in UI

### Task 2: Templates (enhanced)

- [ ] Tabbed interface: "Web" | "Desktop" | "Mobile"
- [ ] Each tab shows the component × implementation matrix for that profile
- [ ] Compound group indicators (icon showing components that are group members)

### Task 3: Build and test

- [ ] Generate with sample reports for both profiles
- [ ] Commit

---

## Chunk 7: Code Generation (Builder + Signal Aware)

**Goal:** Generate idiomatic code for both prop-based frameworks (Leptos) and builder-pattern frameworks (GPUI), with correct signal types.

### Task 1: Enhanced FMD schema

- [ ] Add `componentPattern` field (`"props"` | `"builder"`)
- [ ] Add `builderTraits` mapping (for GPUI)
- [ ] Add signal type mappings (`signalWrapper`, `maybeSignalWrapper`, `writableSignalWrapper`)

### Task 2: Leptos template (signal-aware)

- [ ] Handlebars template that generates:
  - `Signal<T>` for Controlled props
  - `MaybeProp<T>` for MaybeSignal props
  - `RwSignal<T>` for WritableSignal props
  - `Callback<T>` for event handlers
  - Enum types as Rust enums
- [ ] Update `generator.rs` type mapping logic

### Task 3: GPUI template (builder-pattern)

- [ ] Handlebars template that generates:
  - Trait implementations (`Disableable`, `Sizable`, etc.)
  - Builder method chains
  - GPUI-typed event callbacks (`Box<dyn Fn(...)>`)
  - `impl IntoElement` for the component struct

### Task 4: Test both targets

- [ ] Generate Leptos Button → verify Rust compiles
- [ ] Generate GPUI Button → verify Rust compiles (structurally)
- [ ] Commit

---

## Chunk 7.5: Spec Extraction Tool

**Goal:** Implement `ucp extract` for Rust source code.

### Task 1: Rust AST parsing with `syn`

- [ ] Add `syn 2.0`, `quote 1.0` to `ucp-cli` dependencies
- [ ] Parse a `.rs` file, find `#[component]` functions
- [ ] Extract function name → component name
- [ ] Extract parameter names and types

### Task 2: Type mapping from Rust to UCP

- [ ] Implement `rust_type_to_ucp` function:
  - `Signal<T>` → `Signal { inner: map(inner_T) }`
  - `RwSignal<T>` → `WritableSignal { inner: map(inner_T) }`
  - `MaybeProp<T>` → `MaybeSignal { inner: map(inner_T) }`
  - `Callback<T>` → `Callback { params: [map(T)], return: Any }`
  - `Option<T>` → `Nullable { inner: map(T) }`
  - `Children` → `Node`
  - Named enum paths → look up enum definition, produce `Enum { values }`
- [ ] Handle nested generics recursively

### Task 3: Attribute extraction

- [ ] Parse `#[prop(default)]`, `#[prop(default = expr)]`, `#[prop(optional)]`, `#[prop(into)]`
- [ ] Map to UCP `required`, `default` fields

### Task 4: Doc comment extraction

- [ ] Extract `/// ...` doc comments as description

### Task 5: Aggregate extraction

- [ ] `--recursive` flag: walk directory, find all `#[component]` files
- [ ] Output composite `UcpManifest` with per-component spec files
- [ ] Write each component spec to `specs/<name>.json`

### Task 6: Test against real code

- [ ] Test extraction against `leptos-shadcn-button` source (synthetic or real)
- [ ] Verify output matches expected UCP spec structure
- [ ] Commit

---

## Chunk 8: CI Integration (Dual Profile)

**Goal:** CI workflows that run both web (WASM) and desktop (native) conformance tests.

### Task 1: Web conformance workflow

- [ ] Same as v1.0 Chunk 8 but outputs report with `"profile": "web"`
- [ ] Uses `ucp-harness-leptos`

### Task 2: Desktop conformance workflow

- [ ] New job: runs `ucp-harness-gpui` (native, no WASM)
- [ ] Outputs report with `"profile": "desktop"`
- [ ] Requires GPUI test environment

### Task 3: Dashboard update workflow

- [ ] Triggered by report uploads
- [ ] Generates dashboard with both profile tabs
- [ ] Commit

---

## Chunk 8.5: Compile-Time Conformance Proc-Macro

**Goal:** Implement `#[ucp_conform]` proc-macro crate.

### Task 1: Create `ucp-conform` crate

- [ ] `ucp-conform/Cargo.toml` with `proc-macro = true`, `syn 2.0`, `quote 1.0`, `serde_json 1.0`
- [ ] `ucp-conform/src/lib.rs` with attribute macro definition

### Task 2: Parse spec and FMD at compile time

- [ ] Read spec file from `include_str!` (resolved by proc-macro from relative path)
- [ ] Parse `UcpSpec` JSON
- [ ] Parse FMD JSON (optional, with framework detection fallback)

### Task 3: Compare function signature against spec

- [ ] For each prop in spec:
  - Resolve UCP type → FMD native type
  - Check if function has a parameter with matching name and compatible type
  - Type compatibility: compare `syn::Type` against expected type string (fuzzy matching for generics)
- [ ] For each function parameter:
  - Check if it exists in spec or is in allow-list
- [ ] Emit `proc_macro2::TokenStream` with `warning!()` calls for mismatches

### Task 4: Integration test

- [ ] Create a test component with deliberate mismatches
- [ ] Verify compiler warnings are emitted
- [ ] Commit

---

## Chunk 9: Documentation, Diff, Lint (Enhanced)

**Goal:** CLI commands for spec lifecycle, enhanced for enriched types and compound groups.

### Task 1: `doc` command (enhanced)

- [ ] Handlebars template showing:
  - Enriched prop types (signal wrappers, callback signatures)
  - Reactivity category per prop
  - Compound group membership
  - Per-profile accessibility tables
  - Structured keyboard interactions as tables

### Task 2: `diff` command (enhanced)

- [ ] Detect and report: new prop types (e.g., "prop 'value' changed from String to Signal<String>")
- [ ] Detect compound group additions/removals
- [ ] Detect accessibility spec changes per profile

### Task 3: `lint` command (enhanced)

- [ ] Check: components with accessibility specs for only some profiles (warning: "missing desktop accessibility spec")
- [ ] Check: compound group references non-existent components
- [ ] Check: keyboard interactions missing `expectedResult`
- [ ] Check: callback props missing parameter types

### Task 4: Tests and commit

---

## Chunk 9.5: Test Adapter Plugins

**Goal:** Implement `TestAdapter` trait and Playwright+axe adapter.

### Task 1: Define adapter trait in `ucp-core`

- [ ] `ucp-core/src/adapter.rs` with `TestAdapter` trait and `AdapterResult` struct

### Task 2: Playwright+axe adapter

- [ ] Create `ucp-adapter-playwright-axe` crate
- [ ] Parse Playwright JSON report
- [ ] Parse axe-core JSON results
- [ ] Map: axe violation → Gold failure; axe pass → Gold pass
- [ ] Map: Playwright test names → component/tier identification (convention: `button.gold.keyboard.escape`)

### Task 3: CLI integration

- [ ] `ucp test --adapter playwright-axe --results ./test-output/`
- [ ] Load adapter, parse results, produce `ConformanceReport`

### Task 4: JUnit XML adapter (simpler)

- [ ] Create `ucp-adapter-junit` crate
- [ ] Parse JUnit XML, map test cases to conformance checks

### Task 5: Tests and commit

---

## Chunk 10: Conformance Badges (Per-Profile)

**Goal:** Generate badges showing conformance tier per platform profile.

### Task 1: Badge generation (enhanced)

- [ ] Accept `--profile web` or `--profile desktop` flag
- [ ] Badge label: "UCP Web: Silver" or "UCP Desktop: Bronze"
- [ ] Color coding per tier

### Task 2: Composite badge (optional)

- [ ] Badge showing both profiles side by side: "Web: 🥈 | Desktop: 🥉"

### Task 3: CI integration and docs

- [ ] Commit

---

## Chunk 11: SEP Management (Profile-Scoped)

**Goal:** SEP process supporting platform-profile-scoped changes.

### Task 1: SEP init (enhanced)

- [ ] Add `--profile` flag to `sep init`: marks the SEP as targeting a specific profile
- [ ] Metadata includes `targetProfiles: ["desktop"]`

### Task 2: SEP validate (enhanced)

- [ ] If SEP targets a profile, validate only that profile's accessibility specs

### Task 3: SEP apply (enhanced)

- [ ] Apply JSON Patch only to the targeted profile's accessibility spec
- [ ] Version bump: profile-scoped changes → patch; cross-profile → minor

### Task 4: Tests and docs

- [ ] Commit

---

## Chunk 12: Spec Viewer Web App (Enhanced)

**Goal:** Leptos web app showing enriched spec with profile switching.

### Task 1: Profile selector

- [ ] Tabs: "Web" | "Desktop" | "Mobile"
- [ ] Switching tabs shows accessibility requirements for that profile

### Task 2: Enriched prop display

- [ ] Show signal wrappers, callback signatures in readable format
- [ ] Color-code reactivity categories

### Task 3: Compound group visualization

- [ ] Show group diagram (component relationships)
- [ ] Navigate from a component to its group

### Task 4: Structured keyboard interaction display

- [ ] Table showing key, action, condition, expected result

### Task 5: Deploy and commit

---

## Chunk 13: Implementation Registry

**Goal:** Define registry format, CLI commands, and dashboard auto-discovery.

### Task 1: Registry data model

- [ ] `ucp-core/src/registry.rs` with `RegistryEntry` struct

### Task 2: CLI commands

- [ ] `ucp registry list`: fetch and display registered implementations
- [ ] `ucp registry add <url>`: add entry to local config
- [ ] `ucp registry publish`: create/update entry for current implementation

### Task 3: Dashboard auto-fetch

- [ ] Dashboard reads registry, fetches reports from registered endpoints
- [ ] Shows "unregistered" implementations from report files alongside registered ones

### Task 4: Commit

---

## Chunk 14: Release Automation (Enhanced)

**Goal:** Multi-platform binary releases plus npm package and proc-macro publishing.

### Task 1: Cargo-release configuration (as v1.0)

### Task 2: Multi-platform binary builds (as v1.0)

### Task 3: Crates.io publishing

- [ ] Publish: `ucp-core`, `ucp-cli`, `ucp-conform`, `ucp-harness-leptos`, `ucp-adapter-playwright-axe`

### Task 4: npm package

- [ ] Publish enriched JSON Schema (v2.0 format) as `@ucp/spec`

### Task 5: Commit

---

## Chunk 15: End-to-End Validation

**Goal:** Run the complete pipeline against a real library.

### Task 1: Extract spec from `cloud-shuttle-leptos-shadcn-ui` Button

- [ ] `ucp extract --source ./leptos-shadcn-button/src/lib.rs --format leptos`
- [ ] Review and curate the output

### Task 2: Validate the extracted spec

- [ ] `ucp validate specs/button.json`

### Task 3: Generate documentation

- [ ] `ucp doc --spec specs/button.json --output docs/button.md`

### Task 4: Generate code stub for a new framework

- [ ] `ucp generate --spec specs/button.json --fmd fmd/gpui.json --component button --output gpui-button.rs`

### Task 5: Run conformance with Leptos harness

- [ ] `ucp test --spec specs/button.json --implementation leptos-shadcn --profile web --output report.json`

### Task 6: Generate badge and dashboard

- [ ] `ucp badge --report report.json --profile web --output badge.svg`
- [ ] `ucp-dashboard --reports-dir ./ --output-dir dist`

### Task 7: Verify compile-time conformance

- [ ] Add `#[ucp_conform(spec = "specs/button.json")]` to the real Button component
- [ ] Verify no warnings (or fix them)

### Task 8: Document results and commit

---

## Chunk 16: Documentation and Onboarding

**Goal:** Complete documentation for all new capabilities.

### Task 1: Enhanced README

- [ ] Overview of multi-platform support
- [ ] Quick start for each framework (Leptos, GPUI)
- [ ] Extraction tutorial
- [ ] Proc-macro tutorial

### Task 2: FMD authoring guide

- [ ] How to write an FMD for a new framework
- [ ] Examples: Leptos FMD, GPUI FMD, React FMD

### Task 3: Harness authoring guide

- [ ] How to implement `ComponentHarness` for a new framework
- [ ] DOM mode vs manifest mode decision tree

### Task 4: Adapter authoring guide

- [ ] How to write a `TestAdapter` for existing test infrastructure

### Task 5: Commit

---

## Chunk 17: Final Review and Launch

**Goal:** Final quality checks before public release.

### Task 1: Full test suite

- [ ] `cargo test --workspace` (all unit + integration tests pass)
- [ ] WASM tests pass for Leptos harness
- [ ] Native tests pass for GPUI harness (if environment available)

### Task 2: Documentation review

- [ ] All examples are accurate
- [ ] All CLI commands documented with working examples

### Task 3: Security scan

- [ ] `cargo audit` passes
- [ ] No high-severity dependencies

### Task 4: Performance benchmarks

- [ ] Spec parsing ≤100ms
- [ ] Proc-macro overhead ≤2s
- [ ] Extraction ≤10s for 50 components

### Task 5: Tag and release

- [ ] `cargo release patch` (or appropriate version)
- [ ] Verify GitHub Release with binaries
- [ ] Verify crates.io publications
- [ ] Verify npm publication
- [ ] Verify dashboard deployment
- [ ] Announce

---

## 📊 Chunk Summary

| Chunk | Focus | Key Addition vs v1.0 |
|-------|-------|---------------------|
| 1 | Core types | Enriched `PropType`, compound groups, platform profiles |
| 2 | CLI validate/schema | Modular spec loading |
| 3 | Harness + Bronze | Dual-mode trait, manifest mode |
| 4 | Silver | Signal-aware reactivity testing |
| 5 | Gold | Profile-aware accessibility, structured keyboard, compound constraints |
| **5.5** | **Leptos harness** | **Signal creation/updates in tests** |
| **5.7** | **GPUI harness** | **Manifest mode, native a11y, entity reactivity** |
| 6 | Dashboard | Per-profile tabs |
| 7 | Code generation | Builder-pattern templates, signal-aware types |
| **7.5** | **Spec extraction** | **`ucp extract` with `syn`, type mapping, aggregation** |
| 8 | CI | Dual profile (WASM + native) |
| **8.5** | **Proc-macro** | **`#[ucp_conform]` compile-time checking** |
| 9 | Doc/Diff/Lint | Enhanced for enriched types and profiles |
| **9.5** | **Test adapters** | **Playwright+axe, JUnit integration** |
| 10 | Badges | Per-profile labels |
| 11 | SEP | Profile-scoped proposals |
| 12 | Viewer | Profile switching, compound groups |
| **13** | **Registry** | **Discovery, auto-fetch, `ucp registry` commands** |
| 14 | Release | Multi-crate publishing |
| **15** | **E2E validation** | **Full pipeline against real library** |
| **16** | **Documentation** | **FMD, harness, adapter authoring guides** |
| 17 | Launch | Final QA and release |

**Total: 17 chunks (vs 13 in v1.0), with 8 new or substantially revised chunks addressing every gap identified in the codebase analyses.**
