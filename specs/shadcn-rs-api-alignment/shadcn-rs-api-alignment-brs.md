# Business & Stakeholder Requirements Specification (BRS)

| Field | Value |
|-------|-------|
| Project | Shadcn UI API Alignment for Quoin UCP |
| Document | Business & Stakeholder Requirements |
| Version | 0.1 (Draft) |
| Date | 2026-04-21 |
| Author | System Architect (assisted by AI) |
| Status | Draft — Pending Review |

## 1. Business Context

### 1.1 Purpose
This BRS defines the business and stakeholder requirements for aligning the public APIs of **Leptos‑shadcn/ui** and **Dioxus‑shadcn/ui** under a common specification. It captures the needs of Quoin UCP developers and the library maintainers, establishing a shared understanding of the desired outcome.

### 1.2 Business Problem/Opportunity
The two leading Rust shadcn/ui implementations have diverged in component inventory, prop naming, and event signatures. This forces Quoin’s UCP to maintain two separate renderer backends and creates confusion for developers working across frameworks. Aligning the APIs reduces maintenance cost, improves component coverage, and strengthens the Rust UI ecosystem.

### 1.3 Scope Boundaries
**In‑Scope:**
- A common API specification (component names, prop names, event names, ARIA roles) for all 50+ shadcn/ui components.
- A shared conformance test suite that validates both existing libraries against the spec.
- Incremental updates to the existing libraries to achieve conformance.

**Out‑of‑Scope:**
- Creating a new shadcn/ui implementation.
- Changing the visual styling or CSS architecture of the libraries.
- Unifying the internal codebases of the two libraries.

## 2. Business Goals, Objectives & Success Metrics

| ID | Goal | Fit Criterion | Priority |
|----|------|---------------|----------|
| **BG‑1** | API Convergence | 100% of components in the common spec have identical prop names and event signatures in both libraries. | High |
| **BG‑2** | Component Parity | 100% of components in the common spec are implemented in both libraries. | High |
| **BG‑3** | Quoin UCP Simplification | Quoin’s `quoin_render!` macro reduces framework‑specific code by at least 80% (measured by lines of emitter code). | High |
| **BG‑4** | Conformance Validation | Both libraries pass the shared conformance test suite with 100% success. | High |

## 3. Business Model and Processes

### 3.1 Value Propositions
- **For Quoin:** Single, predictable component contract; eliminates per‑framework conditional logic.
- **For Library Maintainers:** Clear roadmap for component parity; shared testing infrastructure reduces QA burden.
- **For Developers:** Consistent shadcn/ui experience across Leptos and Dioxus.

### 3.2 Core Business Processes
1. **Specification Maintenance:** The common spec is hosted in a dedicated repository. Changes are proposed via PR and reviewed by maintainers from both libraries.
2. **Incremental Alignment:** Each library maintainer prioritizes work to align existing components and add missing ones, following the spec.
3. **Conformance Testing:** A shared test suite runs in CI for both libraries. A dashboard displays compliance status.
4. **Quoin Integration:** Quoin’s UCP renderer uses the spec as its source of truth for component APIs.

## 4. Business Rules and Policies

| ID | Rule | Description |
|----|------|-------------|
| **BR‑001** | Backward Compatibility | Breaking API changes must follow a deprecation cycle (minimum one minor version) with clear migration guidance. |
| **BR‑002** | Accessibility Mandate | All components must meet WCAG 2.1 AA standards, as verified by the conformance suite. |
| **BR‑003** | Spec‑First for New Components | Any new component added to one library must be added to the common spec first, then implemented in both libraries. |

## 5. Stakeholders and User Classes

### 5.1 Stakeholder Map
| Stakeholder | Primary Concerns |
|-------------|------------------|
| Quoin UCP Maintainers | Consistent component contract; minimal framework‑specific code. |
| Leptos‑shadcn/ui Maintainers | Preserving quality; managing breaking changes; community expectations. |
| Dioxus‑shadcn/ui Maintainer | Closing feature gap; aligning with community standards. |
| Rust Frontend Developers | Consistent, well‑documented components across frameworks. |

### 5.2 User Classes and Personas
**Primary: Quoin UCP Developer ("Quinn")**
- Maintains Quoin’s renderer backends. Quinn wants the two shadcn/ui libraries to expose identical prop names and event signatures so that `quoin_render!` can generate code without framework‑specific conditionals.

**Secondary: Library Maintainer ("Lee")**
- Maintains one of the shadcn/ui ports. Lee wants clear guidance on what components to add and how to name their props, plus a test suite to validate correctness.

### 5.3 Jobs to Be Done (JTBD)
- **When** implementing a Quoin renderer, **I want to** map a logical component (e.g., "Button") to a concrete library function with predictable prop names **so that** I don't write framework‑specific adapters.
- **When** adding a new shadcn/ui component, **I want to** follow a standard API definition **so that** my implementation is consistent with the other framework's port.

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| **Common Spec** | Framework‑agnostic definition of a component's public API (props, events, ARIA roles). |
| **Conformance Test** | Automated test verifying that a library's component produces the expected DOM structure and behavior. |
| **Logical Prop** | A prop defined in the spec without a concrete Rust type (e.g., `disabled`). Each library maps it to its native reactive type. |

## 7. Conceptual Domain Model

**Entities:**
- **ComponentSpec:** Name, props (with reactivity flag), events, ARIA roles.
- **LibraryImpl:** Leptos‑shadcn/ui or Dioxus‑shadcn/ui.
- **ConformanceTest:** Validates a LibraryImpl against a ComponentSpec.

**Relationships:**
- A `ComponentSpec` is implemented by up to two `LibraryImpl`s.
- A `LibraryImpl` is validated by 1..* `ConformanceTest`s.

## 8. Stakeholder Needs and User Requirements

| ID | Need | User Class | Priority |
|----|------|------------|----------|
| **SN‑001** | Identical component names, prop names, and event names across both libraries. | Quinn, Lee | High |
| **SN‑002** | All shadcn/ui components available in both libraries. | Quinn, Lee | High |
| **SN‑003** | A conformance test suite to validate implementations. | Lee | High |
| **SN‑004** | Clear documentation of the common API for each component. | Quinn, Lee | High |

## 9. System‑in‑Context and Operational Concept

**System‑of‑Interest:** Common API Specification + Conformance Test Suite + Updated Leptos/Dioxus Libraries.

**Operational Concept:**
1. Quoin's `quoin_render!` macro consults the common spec to generate calls to the appropriate library.
2. A library maintainer implements a new component following the spec; they run the conformance tests locally to verify.
3. CI runs the conformance suite against both libraries on every PR, ensuring ongoing alignment.

## 10. Stakeholder‑Level Constraints and Quality Expectations

| ID | Constraint / Quality Expectation | Fit Criterion |
|----|----------------------------------|---------------|
| **CON‑001** | No rewrites of existing libraries | Changes are incremental, via normal PR processes. |
| **CON‑002** | Backward compatibility | Breaking changes follow deprecation cycle. |
| **QUAL‑001** | Conformance test pass rate | 100% for both libraries. |

## 11. Risks, Assumptions, and Open Issues

*(See Vision document.)*

## 12. Traceability Mapping to Vision

| Vision Goal | Stakeholder Need | High‑Level Feature |
|-------------|------------------|-------------------|
| G‑1 (API Convergence) | SN‑001 | Common API spec |
| G‑2 (Component Parity) | SN‑002 | Component catalog alignment |
| G‑3 (Quoin Integration) | SN‑004 | Unified UCP mapping |
| G‑4 (Conformance) | SN‑003 | Shared test suite |
