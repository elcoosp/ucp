# Shadcn UI for Rust – API Alignment Vision

| Field | Value |
|-------|-------|
| Project | Shadcn UI API Alignment for Quoin UCP |
| Document | Vision & Strategic Alignment |
| Version | 0.1 (Draft) |
| Date | 2026-04-21 |
| Author | System Architect (assisted by AI) |
| Status | Draft — Pending Review |

## Vision Statement

To align the public APIs of the existing **Leptos‑shadcn/ui** and **Dioxus‑shadcn/ui** component libraries under a common, framework‑agnostic specification, enabling Quoin’s Universal Component Protocol (UCP) to target both as consistent renderer backends without maintaining bespoke adapters.

## Elevator Pitch (Moore’s Template)

**For** Quoin UCP developers and maintainers of Rust shadcn/ui ports
**who are frustrated** by the fragmented APIs and uneven component coverage between Leptos‑shadcn/ui and Dioxus‑shadcn/ui,
**our initiative** defines a common API specification and conformance test suite
**that provides** a clear, achievable target for both libraries to converge toward.
**Unlike** ad‑hoc integration efforts,
**our approach** treats the existing libraries as first‑class implementations and focuses on incremental alignment, reducing Quoin’s renderer complexity and benefiting the entire Rust UI ecosystem.

## Problem Statement & Business Context

**Current Reality:**
- **Leptos‑shadcn/ui** (~50 components) and **Dioxus‑shadcn/ui** (~26 components) are valuable, production‑ready libraries.
- Their APIs differ significantly: component names, prop names, event handler signatures, and feature depth (validation, accessibility, complex components).
- Quoin’s UCP must currently maintain two divergent backend mappings to support both frameworks, increasing maintenance cost and limiting component coverage.

**Opportunity:** By defining a **common API specification** that both libraries agree to target, Quoin can reduce its renderer logic to a single mapping. The libraries themselves benefit from clear guidance on feature parity and a shared conformance test suite.

## Target Users & Customers

**Primary:**
- **Quoin UCP Contributors** – Need a stable, predictable component contract across Leptos and Dioxus.
- **Leptos‑shadcn/ui Maintainers** – Seek to expand component coverage and improve consistency with community standards.
- **Dioxus‑shadcn/ui Maintainer** – Aims to close the feature gap with the Leptos port.

**Secondary:**
- **Rust Frontend Developers** – Benefit indirectly from improved consistency and component availability.

## User Needs & Value Proposition

| Need | Description |
|------|-------------|
| **API Consistency** | Identical component names, prop names, and event handler signatures across both libraries. |
| **Component Parity** | All shadcn/ui components available in both implementations. |
| **Clear Alignment Path** | A specification that defines the target API for each component, allowing incremental convergence. |
| **Conformance Validation** | A shared test suite that verifies both implementations against the spec. |

## Desired Outcomes & Success Metrics

| ID | Outcome | Key Results |
|----|---------|-------------|
| **G‑1** | **API Convergence** | 100% of components in the common spec have identical prop names and event signatures in both libraries. |
| **G‑2** | **Component Parity** | 100% of components in the common spec are implemented in both libraries. |
| **G‑3** | **Quoin Integration** | Quoin’s `quoin_render!` macro uses a single mapping to target both libraries, with zero framework‑specific conditionals per component. |
| **G‑4** | **Conformance Pass Rate** | Both libraries pass the shared conformance test suite with 100% success. |

## Strategic Constraints

- **No Rewrites:** The existing libraries remain the canonical implementations; changes are incremental and backward‑compatible where possible.
- **Framework‑Agnostic Spec:** The specification defines logical props (e.g., `disabled`) without dictating concrete Rust types. Each library adapts to its native reactivity model (Leptos `MaybeProp<T>` or `(ReadSignal, WriteSignal)`, Dioxus `Signal<T>`).
- **WASM Compatibility:** All components must continue to work in WebAssembly.
- **Accessibility:** WCAG 2.1 AA compliance is a non‑negotiable quality requirement.

## Goals and Non‑goals

**Goals:**
- Define a common API specification (component names, props, events, ARIA roles) for all 50+ shadcn/ui components.
- Provide a conformance test suite that validates both existing libraries against the spec.
- Enable Quoin’s UCP to use a unified mapping to both libraries.

**Non‑goals:**
- Creating a new shadcn/ui implementation from scratch.
- Changing the visual design or CSS class names of existing components.
- Unifying the internal architecture of the two libraries (they remain independent).

## Operational Concept & High‑Level Scenarios

**Scenario A: Quoin UCP Rendering**
1. A developer writes a Quoin component using `quoin_render!`.
2. The macro consults the common API spec to generate framework‑specific code.
3. With `features = ["leptos"]`, it emits `leptos_shadcn_button::Button` with appropriate reactive bindings. With `features = ["dioxus"]`, it emits `dioxus_shadcn_button::Button` with the same logical props.

**Scenario B: Library Maintainer Adding a Component**
1. The maintainer consults the common spec to see the required props and events for a new component (e.g., `Combobox`).
2. They implement the component in their library, following the spec’s naming and behavior.
3. They run the conformance test suite locally to validate their implementation.

## Stakeholders, Sponsorship, and Governance

| Role | Responsible |
|------|-------------|
| **Executive Sponsors** | Core maintainers of Leptos‑shadcn/ui and Dioxus‑shadcn/ui |
| **Spec Editor** | Designated community member (rotating) |
| **Quoin Maintainer** | Provides UCP integration requirements |
| **Decision Model** | Changes to the spec require approval from at least one maintainer from each library. |

## Traceability & Alignment Notes

- All components assigned unique ID (e.g., `SPEC‑BUTTON‑001`).
- The conformance test suite maps spec requirements to automated DOM checks.

## Risks, Assumptions, and Open Questions

**Assumptions:**
- Both library maintainers are willing to align their APIs incrementally.
- Backward compatibility can be maintained during the transition (e.g., deprecation cycles).

**Risks:**
- **Divergent Reactivity Models:** Prop types will necessarily differ (Leptos `MaybeProp<T>`, Dioxus `Signal<T>`). The spec handles this by defining logical prop semantics; the conformance suite validates behavior, not types.
- **Maintainer Bandwidth:** Alignment work may be slow due to limited volunteer time.

**Open Questions:**
- Q1: How to handle components that exist in only one library today (e.g., `InputGroup` in Dioxus, `Select` in Leptos)? The spec should include them, and the missing library should implement them.
- Q2: What is the appropriate deprecation and migration timeline for breaking API changes?
