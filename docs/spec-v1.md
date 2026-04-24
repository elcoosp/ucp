I'll now write the fully-fledged specification documents using the `spec-writer` skill framework, incorporating all the deep research we've covered: cross-language scope, the Universal Component Protocol (UCP), tiered conformance, adoption funnel, and the technical architecture we've designed.

Let's start with the Vision document.

---

# Universal Component Protocol (UCP) – Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Vision & Strategic Alignment |
| Version | 1.0 |
| Date | 2026-04-21 |
| Author | System Architect (assisted by AI) |
| Status | Draft — Pending Review |

## Vision Statement

To establish a **Universal Component Protocol (UCP)**—a language-agnostic, machine-readable specification for component APIs—that enables any framework or platform to implement a consistent set of UI components. UCP serves as the single source of truth for component contracts, empowering tooling to generate idiomatic, framework-native implementations across the entire ecosystem, from Rust to TypeScript to SwiftUI to Jetpack Compose.

## Elevator Pitch (Moore's Template)

**For** library maintainers, framework authors, and cross-platform tooling developers  
**who are frustrated** by the fragmented and inconsistent shadcn/ui ports across React, Vue, Svelte, Solid, Leptos, Dioxus, GPUI, SwiftUI, and Jetpack Compose,  
**our initiative** defines a universal, machine‑readable specification for component APIs and behaviors,  
**that provides** a clear, achievable target for any implementation to converge toward, with tiered conformance levels and a shared test suite.  
**Unlike** ad‑hoc integration efforts or framework‑specific ports,  
**our approach** treats all implementations as first‑class citizens, defines a common language for components, and enables tooling that generates idiomatic code for any target, reducing fragmentation and strengthening the entire UI ecosystem.

## Problem Statement & Business Context

**Current Reality:**
- shadcn/ui has become a de facto standard for component design, with thriving ports across many languages and frameworks (React, Vue, Svelte, Solid, Leptos, Dioxus, GPUI, SwiftUI, Jetpack Compose, Flutter, etc.).
- Each port has diverged in component inventory, prop naming, event signatures, reactivity semantics, and accessibility completeness.
- Developers who work across multiple frameworks (or maintain tools like Quoin UCP) must currently manage bespoke mappings for each target, increasing maintenance cost and limiting component coverage.

**Opportunity:** By defining a **Universal Component Protocol** that all implementations agree to target, we can reduce integration effort to a single logical mapping. The libraries themselves benefit from clear guidance on feature parity, a shared conformance test suite, and a tiered compliance model that allows incremental progress. Tooling can use the spec to generate framework‑native code, documentation, and tests, creating a virtuous cycle of adoption and improvement.

## Target Users & Customers

**Primary:**
- **Library Maintainers** (Rust, TypeScript, Kotlin, Swift, etc.) – Seek to align with community standards, improve component coverage, and benefit from shared testing infrastructure.
- **Cross‑Platform Tooling Developers** (e.g., Quoin UCP) – Need a stable, predictable component contract across all frameworks.
- **Framework Authors** – Want to offer a shadcn/ui compatible component set to their users with minimal manual effort.

**Secondary:**
- **UI Developers** – Benefit from consistent component APIs and documentation across all the frameworks they use.
- **Design System Teams** – Can use UCP as the canonical source for component definitions across their organization's multi‑framework stack.

## User Needs & Value Proposition

| Need | Description |
|------|-------------|
| **Universal Consistency** | Identical component names, prop names, and event names across all implementations. |
| **Clear Reactivity Semantics** | A definition of controlled/uncontrolled/static props that maps cleanly to each framework's state model. |
| **Tiered Adoption Path** | Bronze (API names match), Silver (reactive behavior correct), Gold (full accessibility & edge cases). |
| **Conformance Validation** | A shared test suite that verifies all implementations against the spec, with a public compliance dashboard. |
| **Tooling Integration** | The spec serves as a single source of truth for code generation, documentation, and migration tools. |

## Desired Outcomes & Success Metrics

| ID | Outcome | Key Results |
|----|---------|-------------|
| **G‑1** | **API Convergence** | 100% of components in the UCP spec achieve Bronze conformance in at least 5 major framework ports within 12 months. |
| **G‑2** | **Component Parity** | 100% of components in the UCP spec are implemented in at least 5 framework ports within 18 months. |
| **G‑3** | **Tooling Simplification** | Quoin's `quoin_render!` macro reduces framework‑specific code by at least 80% using UCP as its source of truth. |
| **G‑4** | **Conformance Pass Rate** | All participating libraries achieve Gold conformance for the core 15 components within 12 months. |
| **G‑5** | **Community Adoption** | The UCP spec is referenced in the `Cargo.toml` / `package.json` / `build.gradle` metadata of at least 5 major ports within 12 months. |

## Strategic Constraints

- **No Rewrites:** Existing libraries remain the canonical implementations; changes are incremental and follow a defined deprecation policy.
- **Language‑Agnostic Spec:** The specification defines logical props and behaviors without dictating concrete types. Each implementation adapts to its native reactivity model via a **Framework Mapping Document**.
- **Multi‑Platform Compatibility:** Components must continue to work on their target platforms (Web, native desktop, mobile).
- **Accessibility:** WCAG 2.1 AA compliance is a non‑negotiable quality requirement (Gold tier).

## Goals and Non‑goals

**Goals:**
- Define a common, machine‑readable API specification for all shadcn/ui components (50+), including reactivity categories and ARIA roles.
- Define a tiered conformance model (Bronze, Silver, Gold) to allow incremental alignment.
- Provide a shared conformance test suite that validates any implementation against the spec via a framework‑agnostic test harness trait.
- Enable tooling to use the spec to generate idiomatic code, documentation, and migration scripts.

**Non‑goals:**
- Creating a new shadcn/ui implementation from scratch.
- Changing the visual design or CSS class names of existing components.
- Unifying the internal architecture of the implementations.

## Operational Concept & High‑Level Scenarios

**Scenario A: Cross‑Framework Code Generation**
1. A developer runs `ucp generate button --target react` or `--target leptos` or `--target swiftui`.
2. The UCP CLI reads the JSON schema for Button and consults the target's **Framework Mapping Document**.
3. The CLI outputs a fully‑typed, spec‑compliant component skeleton in idiomatic framework code, ready for implementation.

**Scenario B: Library Maintainer Adding a Component**
1. The maintainer consults the UCP spec for the required props, events, and ARIA roles for a new component (e.g., `Combobox`).
2. They implement the component in their library, using the spec as a checklist.
3. They run the conformance test suite locally to validate Bronze/Silver/Gold tier.
4. Their CI dashboard updates to reflect the new conformance status.

**Scenario C: Spec Evolution via SEP**
1. A maintainer from the SwiftUI port proposes adding a new prop `hapticFeedback` to Button.
2. They open a Spec Enhancement Proposal (SEP) in the UCP repository.
3. The proposal is reviewed by Spec Editors from at least three different language ecosystems.
4. If approved, the spec is updated, and generators are updated to include the new prop.

## Stakeholders, Sponsorship, and Governance

| Role | Responsible |
|------|-------------|
| **Executive Sponsors** | Core maintainers of Leptos‑shadcn/ui, Dioxus‑shadcn/ui, gpui‑component, and leading web ports (React, Vue) |
| **Spec Editors** | Designated community members (rotating, one per major language ecosystem) |
| **Quoin Maintainer** | Provides UCP integration requirements |
| **Decision Model** | Changes to the spec require approval via a **Spec Enhancement Proposal (SEP)** process, with at least one maintainer from three different language ecosystems approving. |

## Traceability & Alignment Notes

- All components assigned unique ID (e.g., `UCP‑BUTTON‑001`).
- The conformance test suite maps spec requirements to automated checks.

## Risks, Assumptions, and Open Questions

**Assumptions:**
- Library maintainers across languages are willing to align their APIs incrementally.
- Backward compatibility can be maintained during the transition using the defined deprecation policy.

**Risks:**
- **Divergent Reactivity Models:** Mitigated by defining logical prop categories (controlled/uncontrolled/static) and framework‑specific mapping documents.
- **Maintainer Bandwidth:** Tiered conformance (Bronze/Silver/Gold) allows libraries to make progress without being blocked on full compliance.
- **Cross‑Language Testing:** Conformance suite requires a harness for each target platform (WebDriver for web, custom harness for native). Mitigated by defining a common `ComponentHarness` trait.

**Open Questions:**
- Q1: Should visual regression testing be included in the Gold conformance tier? (Deferred to future Gold+ tier.)
- Q2: What is the appropriate deprecation timeline for breaking API changes? (Proposal: 2 minor versions / 6 months)

---

Now, the Business & Stakeholder Requirements Specification (BRS).
# Business & Stakeholder Requirements Specification  
**Universal Component Protocol (UCP)**

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Business & Stakeholder Requirements Specification (BRS) |
| Version | 1.0 |
| Date | 2026-04-21 |
| Author | System Architect |
| Status | Draft — Pending Review |
| References | UCP Vision & Strategic Alignment v1.0 |

---

## 1. Introduction

### 1.1 Purpose
This Business & Stakeholder Requirements Specification (BRS) defines the high‑level business objectives, stakeholder needs, and solution‑bounding requirements for the **Universal Component Protocol (UCP)** initiative. It serves as the foundational requirements artefact from which more detailed stakeholder requirements, system requirements, and design specifications will be derived.

The document is intended for:
- Executive sponsors and decision‑makers approving the initiative.
- Product and engineering leadership guiding implementation.
- Library maintainers and framework authors who will adopt UCP.
- Tooling developers who will consume the specification.

### 1.2 Scope
UCP defines a **language‑agnostic, machine‑readable specification** for the API surface of UI components commonly found in shadcn/ui ports. The scope includes:
- A formal schema describing component names, props, events, slots, ARIA roles, and reactivity categories.
- A **tiered conformance model** (Bronze, Silver, Gold) allowing incremental adoption.
- A **shared conformance test suite** that validates implementations against the spec.
- **Framework Mapping Documents** that translate the abstract spec into concrete, idiomatic APIs for each target language/framework.
- Tooling to generate code stubs, documentation, and migration scripts from the spec.

The scope **excludes**:
- Visual styling, CSS, or design tokens (these remain the domain of each implementation).
- Internal implementation details or component architecture.
- Runtime behaviour beyond what is exposed through the public API and accessibility expectations.

### 1.3 Document Conventions
- Requirements are uniquely identified as **BR‑XXX** (Business Requirement) or **SR‑XXX** (Stakeholder Requirement).
- Each requirement shall include a rationale and traceability to goals defined in the Vision document.
- The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

---

## 2. Business Context

### 2.1 Problem Statement
The shadcn/ui component collection has been ported to numerous frameworks and languages—including React, Vue, Svelte, Solid, Leptos (Rust), Dioxus (Rust), GPUI (Rust), SwiftUI, Jetpack Compose, and Flutter. Each port has evolved independently, resulting in:

- **API Inconsistency:** Component names, prop names, event signatures, and default behaviours differ between ports.
- **Component Inventory Gaps:** Some ports implement only a subset of the full shadcn/ui catalogue.
- **Fragmented Accessibility:** ARIA compliance varies widely; many ports lack complete keyboard navigation and screen‑reader support.
- **High Maintenance Burden for Tooling:** Cross‑framework tools (e.g., Quoin UCP) must maintain bespoke adapters for each target, limiting scalability and component coverage.

### 2.2 Opportunity
By establishing UCP as a **canonical, machine‑readable specification** for component APIs, the ecosystem can:
- Provide a clear, versioned target for all implementations to converge toward.
- Reduce integration effort for cross‑framework tooling from *O(n)* per‑framework mappings to a single, spec‑driven generator.
- Improve component quality and accessibility across all ports through shared testing and conformance visibility.
- Enable new classes of tooling—code generators, documentation sites, and migration assistants—that operate consistently across the entire component landscape.

### 2.3 Business Objectives
The following business objectives are derived from the Vision document and represent the measurable outcomes the initiative aims to achieve.

| ID | Objective | Success Criteria |
|----|-----------|------------------|
| **BO‑1** | **API Convergence** | At least five major framework ports achieve Bronze conformance for all components defined in UCP v1 within 12 months. |
| **BO‑2** | **Component Parity** | The UCP v1 component catalogue (50+ components) is fully implemented in at least five framework ports within 18 months. |
| **BO‑3** | **Tooling Efficiency** | Quoin UCP (or equivalent tool) reduces framework‑specific adapter code by ≥80% when using UCP as its single source of truth. |
| **BO‑4** | **Conformance Visibility** | A public dashboard displays conformance status (Bronze/Silver/Gold) for all participating implementations, updated on each release. |
| **BO‑5** | **Community Adoption** | UCP is referenced as a specification dependency in the package metadata of at least five major ports within 12 months. |

### 2.4 Business Risks
| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Maintainers reject alignment due to breaking changes. | High | Medium | Tiered conformance allows incremental, non‑breaking adoption; clear deprecation policy. |
| Divergent reactivity models prevent unified spec. | High | Low | Abstract prop categories (controlled/uncontrolled/static) with framework‑specific mapping documents. |
| Insufficient community engagement. | Medium | Medium | Governance model includes rotating spec editors from each ecosystem; SEP process is open and lightweight. |
| Testing harness complexity across platforms. | Medium | Medium | Define a common `ComponentHarness` trait; provide reference implementations for Web (Playwright) and native (custom test apps). |

---

## 3. Stakeholders

### 3.1 Stakeholder Identification

| Stakeholder Group | Representative(s) | Role & Interest |
|-------------------|-------------------|-----------------|
| **Library Maintainers** | Maintainers of React, Vue, Svelte, Solid, Leptos, Dioxus, GPUI, SwiftUI, Compose ports | Primary adopters; seek clear alignment targets and reduced maintenance burden. |
| **Cross‑Platform Tooling Developers** | Quoin UCP maintainers, other framework‑agnostic UI tool authors | Need stable, predictable component contracts to automate code generation. |
| **Framework Authors** | Creators of new UI frameworks (e.g., emerging Rust or Kotlin frameworks) | Want to offer shadcn/ui compatibility out‑of‑the‑box with minimal manual porting. |
| **UI Developers** | End‑users of the component libraries | Benefit from consistent APIs and accessibility across all frameworks they use. |
| **Design System Teams** | Enterprise teams maintaining multi‑framework design systems | Use UCP as the canonical source of truth for component definitions. |
| **Accessibility Specialists** | A11y advocates and auditors | Ensure accessibility requirements are specified and testable. |
| **Executive Sponsors** | Core maintainers of leading ports (Leptos‑shadcn/ui, Dioxus‑shadcn/ui, gpui‑component, shadcn/ui) | Provide authority, resources, and community influence. |

### 3.2 Stakeholder Needs Summary

| Stakeholder Group | Primary Needs |
|-------------------|---------------|
| Library Maintainers | Clear, versioned spec; incremental conformance tiers; automated conformance testing; minimal breaking changes. |
| Tooling Developers | Machine‑readable spec (JSON Schema); deterministic mapping rules; stable identifiers for components/props/events. |
| Framework Authors | Framework‑agnostic component definitions; guidance on reactivity mapping; reference implementations. |
| UI Developers | Consistent component APIs across frameworks; predictable behaviour; accessible components by default. |
| Design System Teams | Single source of truth; ability to extend spec for organisation‑specific components. |
| Accessibility Specialists | ARIA roles and keyboard interactions explicitly specified; testable conformance criteria. |

---

## 4. Business Requirements

Business requirements describe the high‑level capabilities the UCP initiative must deliver to satisfy business objectives.

### BR‑001: Component API Specification
**Description:** UCP SHALL define a machine‑readable specification for the public API of each shadcn/ui component, including name, props, events, slots/children, and ARIA roles.

**Rationale:** A formal spec is the foundation for all alignment, tooling, and conformance validation.

**Traceability:** BO‑1, BO‑2, BO‑3

### BR‑002: Tiered Conformance Model
**Description:** UCP SHALL define three conformance tiers—Bronze (API names match), Silver (reactive behaviour correct), Gold (full accessibility and edge cases)—allowing implementations to adopt the spec incrementally.

**Rationale:** Reduces adoption friction and enables maintainers to prioritise work based on community needs.

**Traceability:** BO‑1, BO‑4

### BR‑003: Shared Conformance Test Suite
**Description:** UCP SHALL provide a reusable, framework‑agnostic test suite that verifies an implementation's conformance against each tier.

**Rationale:** Automates validation, reduces manual testing burden, and provides objective conformance evidence.

**Traceability:** BO‑4, BO‑5

### BR‑004: Public Conformance Dashboard
**Description:** UCP SHALL maintain a public dashboard displaying the conformance status (tier per component) for all participating implementations, updated automatically on each release.

**Rationale:** Encourages adoption through transparency and healthy competition; provides visibility for tooling and end‑users.

**Traceability:** BO‑4, BO‑5

### BR‑005: Tooling Ecosystem Support
**Description:** UCP SHALL be designed to enable tooling (code generators, documentation generators, migration assistants) to consume the spec and produce framework‑specific outputs.

**Rationale:** Maximises the value of the spec beyond manual alignment, driving ecosystem efficiency.

**Traceability:** BO‑3

### BR‑006: Governance and Evolution Process
**Description:** UCP SHALL operate under a documented governance model, including a Spec Enhancement Proposal (SEP) process for changes, with approval requiring representation from at least three language ecosystems.

**Rationale:** Ensures the spec remains community‑driven, balanced across ecosystems, and evolves predictably.

**Traceability:** BO‑5 (adoption depends on trust in governance)

---

## 5. Stakeholder Requirements (High‑Level)

Stakeholder requirements refine the business requirements into more specific capabilities expected by key stakeholder groups. These are intentionally kept at a "black‑box" level—describing *what* the system must do, not *how*.

### 5.1 Specification Format and Structure

**SR‑001: Machine‑Readable Schema**
The specification SHALL be provided in a machine‑readable format (JSON Schema or equivalent) that can be parsed programmatically by tooling. The schema SHALL include stable identifiers for each component, prop, event, and slot.

**Rationale:** Tooling automation depends on reliable, parsable data.  
**Source:** Tooling Developers, Library Maintainers  
**Traceability:** BR‑001, BR‑005

**SR‑002: Human‑Readable Documentation**
The specification SHALL be accompanied by human‑readable documentation, generated from the machine‑readable source, describing each component's purpose, API surface, and conformance expectations.

**Rationale:** Maintainers and developers need accessible reference material.  
**Source:** All Stakeholders  
**Traceability:** BR‑001

**SR‑003: Versioning and Changelog**
The UCP spec SHALL follow semantic versioning (SemVer). Each release SHALL include a detailed changelog describing additions, modifications, and deprecations.

**Rationale:** Implementations need predictable upgrade paths.  
**Source:** Library Maintainers, Tooling Developers  
**Traceability:** BR‑006

### 5.2 Component API Definition

**SR‑004: Component Name Standardisation**
The spec SHALL define a canonical name for each component (e.g., `Button`, `Dialog`, `Combobox`). Implementations targeting UCP SHALL use this name (or a close, documented variant) as the exported component identifier.

**Rationale:** Reduces cognitive overhead for developers moving between frameworks.  
**Source:** UI Developers, Design System Teams  
**Traceability:** BR‑001, BR‑002 (Bronze tier)

**SR‑005: Prop Definition**
For each component, the spec SHALL define:

- Prop name (canonical)
- Expected data type category (e.g., `boolean`, `string`, `enum`, `function`, `node`)
- Reactivity category: **Controlled** (value managed by parent), **Uncontrolled** (initial value only), or **Static** (never changes after mount)
- Whether the prop is required or optional
- Default value, if any

**Rationale:** Enables consistent prop naming and behaviour across implementations.  
**Source:** Library Maintainers, Tooling Developers  
**Traceability:** BR‑001, BR‑002 (Silver tier for reactivity correctness)

**SR‑006: Event Definition**
For each component, the spec SHALL define:

- Event name (canonical, e.g., `onChange`, `onOpenChange`)
- Payload data type category
- Whether the event is cancellable (where applicable)

**Rationale:** Consistent event signatures reduce integration friction.  
**Source:** UI Developers, Tooling Developers  
**Traceability:** BR‑001

**SR‑007: Slot / Children Definition**
The spec SHALL define named slots (or child content areas) for components that support content projection, using canonical slot names (e.g., `trigger`, `content`, `footer`).

**Rationale:** Enables tooling to generate correct composition patterns.  
**Source:** Library Maintainers, Tooling Developers  
**Traceability:** BR‑001

**SR‑008: ARIA and Accessibility Requirements**
The spec SHALL include, for each component:

- Implicit ARIA role (if any)
- Required ARIA attributes and their expected values based on component state
- Keyboard interaction patterns (as prose or structured data)

**Rationale:** Accessibility is a core quality requirement; specification enables conformance testing.  
**Source:** Accessibility Specialists, UI Developers  
**Traceability:** BR‑001, BR‑002 (Gold tier)

### 5.3 Conformance Model and Testing

**SR‑009: Conformance Tier Definitions**
The spec SHALL clearly define the criteria for Bronze, Silver, and Gold conformance for each component.

- **Bronze:** Component name and exported prop/event names match the spec. Type compatibility is not required at this tier.
- **Silver:** All Bronze requirements, plus reactivity behaviour matches spec (controlled/uncontrolled/static props behave as defined). Event payloads are type‑compatible.
- **Gold:** All Silver requirements, plus full ARIA implementation, keyboard navigation per spec, and handling of all defined edge cases.

**Rationale:** Clear definitions enable objective assessment and incremental progress.  
**Source:** Library Maintainers  
**Traceability:** BR‑002

**SR‑010: Framework‑Agnostic Test Harness Interface**
The conformance test suite SHALL be built against a common `ComponentHarness` interface (trait/protocol) that each implementation provides. The harness abstracts framework‑specific rendering, prop setting, event listening, and DOM/accessibility tree inspection.

**Rationale:** A single test suite can validate any implementation, reducing duplication.  
**Source:** Library Maintainers, Tooling Developers  
**Traceability:** BR‑003

**SR‑011: Automated Conformance Reporting**
The test suite SHALL produce a machine‑readable report (e.g., JSON) detailing which conformance checks passed or failed, per component and per tier. This report SHALL be suitable for ingestion by the public conformance dashboard.

**Rationale:** Enables automated dashboard updates and CI integration.  
**Source:** Library Maintainers, Executive Sponsors  
**Traceability:** BR‑003, BR‑004

### 5.4 Tooling and Ecosystem

**SR‑012: Code Generation from Spec**
The spec SHALL be structured to enable deterministic code generation for any target framework, given a corresponding Framework Mapping Document. The mapping document defines how abstract types and reactivity categories translate to concrete language/framework idioms.

**Rationale:** Maximises automation and reduces manual boilerplate for implementers.  
**Source:** Tooling Developers, Framework Authors  
**Traceability:** BR‑005

**SR‑013: Framework Mapping Document Template**
UCP SHALL provide a template and guidelines for creating a Framework Mapping Document, covering:

- Naming conventions for components/props/events
- Type mappings from abstract categories to concrete types
- Reactivity implementation patterns (e.g., signals, state hooks, bindings)
- Event handler signatures

**Rationale:** Ensures consistency and reduces onboarding effort for new framework targets.  
**Source:** Framework Authors  
**Traceability:** BR‑005

### 5.5 Governance and Community

**SR‑014: Spec Enhancement Proposal (SEP) Process**
UCP SHALL maintain a public, documented process for proposing changes to the spec. The process SHALL include:

- A template for proposals (rationale, impact, migration path)
- A review period (minimum 2 weeks)
- Approval by at least one Spec Editor from three different language ecosystems
- A deprecation policy for breaking changes (minimum 6 months / 2 minor versions notice)

**Rationale:** Ensures spec evolution is transparent, fair, and considers cross‑ecosystem impact.  
**Source:** All Stakeholders  
**Traceability:** BR‑006

**SR‑015: Spec Editor Rotation**
The role of Spec Editor SHALL rotate among active community members, with a term length of 12 months. There SHALL be at least one editor from each of the following language groups: JavaScript/TypeScript, Rust, Swift/Kotlin (native mobile).

**Rationale:** Prevents capture by any single ecosystem and maintains broad perspective.  
**Source:** Library Maintainers, Executive Sponsors  
**Traceability:** BR‑006

---

## 6. Non‑Functional Requirements

Non‑functional requirements define quality attributes and constraints that apply to the UCP system (the specification and its supporting tooling/infrastructure).

### NFR‑001: Specification Stability
The UCP spec SHALL maintain backward compatibility within a major version. Breaking changes SHALL follow the documented deprecation policy (≥6 months notice).

### NFR‑002: Machine Readability Performance
The JSON Schema representation of the full component catalogue SHALL be parsable by standard JSON tooling in under 100ms on typical developer hardware.

### NFR‑003: Test Suite Execution Time
The conformance test suite for a single implementation, when run against the core 15 components, SHALL complete in under 5 minutes on CI infrastructure.

### NFR‑004: Dashboard Availability
The public conformance dashboard SHALL be available with 99% uptime, excluding scheduled maintenance.

### NFR‑005: Accessibility of Spec Documentation
The human‑readable documentation generated from the spec SHALL meet WCAG 2.1 AA accessibility standards.

### NFR‑006: Security
The specification repository and tooling SHALL not introduce vulnerabilities that could affect downstream implementations. Dependencies SHALL be regularly scanned.

---

## 7. Constraints and Assumptions

### 7.1 Constraints
- **C‑001: No Rewrites** – The initiative SHALL NOT require or mandate a complete rewrite of existing shadcn/ui ports. Changes must be incrementally adoptable.
- **C‑002: Framework Neutrality** – The core specification SHALL NOT contain framework‑specific syntax or semantics. All framework‑specific details are confined to Framework Mapping Documents.
- **C‑003: Platform Independence** – The spec SHALL NOT assume a particular runtime environment (Web, native, etc.). Components must remain implementable on their existing target platforms.
- **C‑004: Licensing** – The UCP specification, test suite, and core tooling SHALL be released under the MIT license to maximise adoption.

### 7.2 Assumptions
- **A‑001:** The maintainers of major shadcn/ui ports are willing to collaborate and incrementally align their APIs.
- **A‑002:** A sufficient subset of the community will contribute to the conformance test harness for different platforms (Web, native desktop, mobile).
- **A‑003:** The governance model (SEP process, rotating editors) will be accepted by the community as fair and effective.
- **A‑004:** Quoin UCP or similar tooling will provide a real‑world validation of the spec's utility for code generation.

---

## 8. Dependencies

| Dependency | Description | Impact if Unavailable |
|------------|-------------|-----------------------|
| **Community Engagement** | Active participation from maintainers of at least 3‑5 major ports. | Spec may lack real‑world validation; adoption stalls. |
| **CI/CD Infrastructure** | Hosting for spec repository, test suite execution, and conformance dashboard. | Delays in automated reporting; manual processes required. |
| **Framework Mapping Documents** | Creation of mapping docs for each target ecosystem. | Tooling cannot generate idiomatic code; adoption friction increases. |
| **Test Harness Implementations** | Reference harnesses for Web (Playwright) and at least one native platform. | Conformance testing cannot be automated for all targets. |

---

## 9. Traceability Matrix

The following matrix maps high‑level stakeholder requirements to business objectives and business requirements.

| Stakeholder Requirement | Business Objective(s) | Business Requirement(s) |
|-------------------------|-----------------------|------------------------|
| SR‑001 (Machine‑Readable Schema) | BO‑3 | BR‑001, BR‑005 |
| SR‑002 (Human‑Readable Docs) | BO‑5 | BR‑001 |
| SR‑003 (Versioning) | BO‑5 | BR‑006 |
| SR‑004 (Component Names) | BO‑1 | BR‑001, BR‑002 |
| SR‑005 (Prop Definition) | BO‑1, BO‑2 | BR‑001, BR‑002 |
| SR‑006 (Event Definition) | BO‑1 | BR‑001 |
| SR‑007 (Slot Definition) | BO‑1 | BR‑001 |
| SR‑008 (ARIA Requirements) | BO‑2, BO‑4 | BR‑001, BR‑002 |
| SR‑009 (Tier Definitions) | BO‑1, BO‑4 | BR‑002 |
| SR‑010 (Test Harness Interface) | BO‑4 | BR‑003 |
| SR‑011 (Automated Reporting) | BO‑4 | BR‑003, BR‑004 |
| SR‑012 (Code Generation) | BO‑3 | BR‑005 |
| SR‑013 (Mapping Doc Template) | BO‑5 | BR‑005 |
| SR‑014 (SEP Process) | BO‑5 | BR‑006 |
| SR‑015 (Editor Rotation) | BO‑5 | BR‑006 |

---

## 10. Approval

| Role | Name | Signature | Date |
|------|------|-----------|------|
| Executive Sponsor | [TBD] | | |
| Product Lead | [TBD] | | |
| Spec Editor (JS/TS) | [TBD] | | |
| Spec Editor (Rust) | [TBD] | | |
| Spec Editor (Swift/Kotlin) | [TBD] | | |

---

*End of Business & Stakeholder Requirements Specification*
# Software Requirements Specification  
**Universal Component Protocol (UCP)**

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Software Requirements Specification (SRS) |
| Version | 1.0 |
| Date | 2026-04-21 |
| Author | System Architect |
| Status | Draft — Pending Review |
| References | UCP Vision v1.0, UCP BRS v1.0 |

---

## 1. Introduction and Scope

### 1.1 Purpose
This Software Requirements Specification (SRS) defines the functional and non‑functional requirements for the **Universal Component Protocol (UCP)** system. UCP comprises:
- A machine‑readable **Component API Specification** (schema and documentation).
- A **Conformance Test Suite** for validating implementations against the specification.
- Supporting **Tooling and Infrastructure** (code generators, conformance dashboard, governance automation).

This document serves as the technical contract for implementation and verification. It is written to be implementation‑agnostic—specifying *what* the system must do, not *how* it must be built.

### 1.2 Document Scope
This SRS covers:
- The UCP specification schema, its structure, and evolution mechanisms.
- The conformance test harness and its interfaces.
- The public conformance dashboard and its data requirements.
- Tooling interfaces for code generation and framework mapping.
- Quality attributes (performance, reliability, security, maintainability) of the UCP system itself.

**Out of Scope:**
- The internal implementation details of any shadcn/ui port.
- Visual design, CSS, or styling of components.
- The Quoin UCP product itself (except where it consumes UCP specifications).

### 1.3 Product Overview
UCP provides a canonical, versioned, machine‑readable definition of the API surface for shadcn/ui components. The system consists of:

1. **UCP Specification Repository** – A Git repository containing JSON Schema definitions, human‑readable documentation (generated), and versioned releases.
2. **Conformance Test Suite** – A framework‑agnostic test harness that validates any implementation's compliance with the spec, producing machine‑readable conformance reports.
3. **Conformance Dashboard** – A public web application displaying the conformance status (Bronze/Silver/Gold per component) for all participating implementations.
4. **UCP CLI Tool** – A command‑line utility for validating implementations, generating code stubs, and interacting with the specification.

### 1.4 Definitions and Acronyms
See the UCP Glossary (Appendix A). Key terms used throughout this SRS:

| Term | Definition |
|------|------------|
| **UCP Spec** | The machine‑readable JSON Schema defining component APIs. |
| **Framework Mapping Document (FMD)** | A per‑target specification defining how UCP abstract types map to concrete language/framework idioms. |
| **Conformance Tier** | Bronze (API names match), Silver (reactive behaviour correct), Gold (full ARIA and edge cases). |
| **Component Harness** | A framework‑specific adapter that implements the `ComponentHarness` interface for the conformance test suite. |
| **SEP** | Spec Enhancement Proposal – the formal process for changing the UCP specification. |

---

## 2. Stakeholders and Business Goals

### 2.1 Stakeholders
| Stakeholder | Role | Primary Concerns |
|-------------|------|------------------|
| Library Maintainers | Implement UCP in their shadcn/ui ports | Clear, stable spec; conformance testing; incremental adoption path. |
| Tooling Developers | Consume UCP for code generation | Machine‑readable schema; deterministic mapping rules; stable identifiers. |
| Spec Editors | Govern spec evolution | SEP process; versioning; community consensus. |
| End‑User Developers | Use components across frameworks | Consistent APIs; predictable behaviour; accessible components. |

### 2.2 Business Goals and Success Metrics
Refer to UCP BRS Section 2.3. Key objectives relevant to this SRS:

| Goal ID | Objective | Success Criteria |
|---------|-----------|------------------|
| BO‑1 | API Convergence | 5+ framework ports achieve Bronze conformance within 12 months. |
| BO‑3 | Tooling Efficiency | Quoin UCP reduces adapter code by ≥80% using UCP. |
| BO‑4 | Conformance Visibility | Public dashboard displays up‑to‑date conformance status. |

---

## 3. System Context and Overview

### 3.1 System Boundary Diagram
```
┌─────────────────────────────────────────────────────────────────┐
│                        UCP System                                │
│  ┌─────────────────┐  ┌──────────────────┐  ┌────────────────┐  │
│  │ UCP Spec Repo   │  │ Conformance Test │  │ Conformance    │  │
│  │ (JSON Schema +  │  │ Suite (Harness + │  │ Dashboard      │  │
│  │  Docs)          │  │  Test Scenarios) │  │ (Web App)      │  │
│  └────────┬────────┘  └────────┬─────────┘  └───────┬────────┘  │
│           │                    │                     │           │
│           └──────────┬─────────┴─────────────────────┘           │
│                      │ UCP CLI Tool                              │
└──────────────────────┼───────────────────────────────────────────┘
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
┌───────────────┐ ┌───────────────┐ ┌───────────────┐
│ Leptos Port   │ │ React Port    │ │ SwiftUI Port  │
│ (Rust)        │ │ (TypeScript)  │ │ (Swift)       │
└───────────────┘ └───────────────┘ └───────────────┘
```

### 3.2 External Systems and Interfaces
| External System | Interface Description |
|-----------------|-----------------------|
| **GitHub** | Hosts UCP specification repository, SEP discussions, and conformance test suite. |
| **Package Registries** (crates.io, npm, Swift Package Index) | UCP specification published as a versioned package for tooling consumption. |
| **CI/CD Systems** (GitHub Actions, etc.) | Run conformance test suite for participating implementations; submit reports to dashboard. |
| **Framework Ports** | Implement `ComponentHarness` and submit conformance reports. |

### 3.3 Operating Environment
- UCP specification is static JSON/YAML files served via Git and package registries.
- Conformance test suite executes in Node.js / Rust / Swift environments depending on the target harness.
- Conformance dashboard is a static site (or lightweight serverless app) deployed to a CDN.

---

## 4. Functional Capabilities and Behavior

Functional requirements are grouped by major capability. Each requirement is uniquely identified and traceable to business objectives.

### 4.1 UCP Specification Schema

**CAP‑SPEC‑01: Component Definition Schema**

**SR‑SPEC‑001: Component Identifier**
> The UCP specification SHALL define a unique, stable identifier for each component (e.g., `accordion`, `button`, `dialog`).  
> **Rationale:** Enables consistent referencing across tools and implementations.  
> **Traceability:** BO‑1, SR‑004 (BRS)  
> **Verification Method:** Inspection (schema validation), Test (tooling parsing).

**SR‑SPEC‑002: Component Metadata**
> For each component, the specification SHALL include:
> - `name`: Canonical component name (string, kebab‑case).
> - `description`: Human‑readable description (string).
> - `category`: Grouping (e.g., `layout`, `form`, `overlay`, `data‑display`).
> - `since`: UCP version when component was introduced (SemVer string).
> - `deprecated`: Optional boolean and deprecation message.  
> **Rationale:** Provides context for tooling and documentation generation.  
> **Traceability:** BO‑3, SR‑002 (BRS)  
> **Verification Method:** Inspection.

**SR‑SPEC‑003: Prop Definition**
> For each component prop, the specification SHALL define:
> - `name`: Canonical prop name (camelCase).
> - `type`: Abstract type category (`string`, `boolean`, `number`, `enum`, `node`, `function`, `array`, `object`).
> - `reactivity`: Category (`controlled`, `uncontrolled`, `static`).
> - `required`: Boolean indicating if prop is mandatory.
> - `default`: Optional default value (expressed as JSON literal).
> - `description`: Human‑readable description.
> - `deprecated`: Optional deprecation info.  
> **Rationale:** Enables consistent prop naming and behaviour across frameworks.  
> **Traceability:** BO‑1, SR‑005 (BRS)  
> **Verification Method:** Inspection, Test (schema validation).

**SR‑SPEC‑004: Event Definition**
> For each component event, the specification SHALL define:
> - `name`: Canonical event name (e.g., `onChange`, `onOpenChange`).
> - `payload`: Abstract type category of event data.
> - `description`: Human‑readable description.  
> **Rationale:** Consistent event signatures reduce integration friction.  
> **Traceability:** BO‑1, SR‑006 (BRS)  
> **Verification Method:** Inspection.

**SR‑SPEC‑005: Slot Definition**
> For components supporting content projection, the specification SHALL define named slots:
> - `name`: Canonical slot name (e.g., `trigger`, `content`, `footer`).
> - `description`: Purpose of the slot.
> - `required`: Boolean indicating if slot must be provided.  
> **Rationale:** Enables tooling to generate correct composition patterns.  
> **Traceability:** BO‑1, SR‑007 (BRS)  
> **Verification Method:** Inspection.

**SR‑SPEC‑006: ARIA and Accessibility Requirements**
> For each component, the specification SHALL define:
> - `implicitRole`: ARIA role (if any).
> - `requiredAriaAttributes`: Array of required ARIA attributes with expected value patterns.
> - `keyboardInteractions`: Prose description of expected keyboard behaviour.  
> **Rationale:** Accessibility is a core quality requirement; specification enables conformance testing.  
> **Traceability:** BO‑2, SR‑008 (BRS)  
> **Verification Method:** Inspection, Test (Gold conformance checks).

**SR‑SPEC‑007: Machine‑Readable Format**
> The specification SHALL be provided in JSON Schema format (or equivalent machine‑readable format) that can be parsed programmatically.  
> **Rationale:** Tooling automation depends on reliable, parsable data.  
> **Traceability:** BO‑3, SR‑001 (BRS)  
> **Verification Method:** Test (parsing by reference implementation).

**SR‑SPEC‑008: Human‑Readable Documentation Generation**
> The UCP system SHALL generate human‑readable documentation (Markdown/HTML) from the machine‑readable specification.  
> **Rationale:** Maintainers and developers need accessible reference material.  
> **Traceability:** SR‑002 (BRS)  
> **Verification Method:** Test (documentation build output).

**SR‑SPEC‑009: Semantic Versioning**
> The UCP specification SHALL follow Semantic Versioning 2.0.0. Each release SHALL include a changelog describing additions, modifications, and deprecations.  
> **Rationale:** Implementations need predictable upgrade paths.  
> **Traceability:** SR‑003 (BRS)  
> **Verification Method:** Inspection (release process).

**SR‑SPEC‑010: Deprecation Policy**
> When a component, prop, or event is deprecated, the specification SHALL:
> - Mark the item as `deprecated: true`.
> - Include a `deprecationMessage` explaining the reason and migration path.
> - Maintain the item in the spec for at least **two minor versions** (≥6 months) before removal.  
> **Rationale:** Provides a predictable migration window for implementations.  
> **Traceability:** SR‑014 (BRS)  
> **Verification Method:** Inspection (release policy).

---

### 4.2 Conformance Tiers and Testing

**CAP‑TEST‑01: Conformance Tier Definitions**

**SR‑TEST‑001: Bronze Conformance Criteria**
> A component implementation SHALL be considered **Bronze** conformant if:
> - The exported component name matches the UCP canonical name (case‑sensitive, with framework‑specific naming convention allowed per FMD).
> - All props defined in the UCP spec are present as exported props (names match per FMD naming rules).
> - All events defined in the UCP spec are present as exported event handlers (names match per FMD).  
> **Rationale:** Provides a low‑friction entry point for alignment.  
> **Traceability:** SR‑009 (BRS)  
> **Verification Method:** Test (automated conformance suite).

**SR‑TEST‑002: Silver Conformance Criteria**
> A component implementation SHALL be considered **Silver** conformant if it meets all Bronze criteria AND:
> - Reactivity behaviour for each prop matches its declared category (`controlled` / `uncontrolled` / `static`) as defined by the Framework Mapping Document.
> - Event payload types are compatible with the UCP abstract type (per FMD type mappings).  
> **Rationale:** Ensures behavioural correctness beyond naming.  
> **Traceability:** SR‑009 (BRS)  
> **Verification Method:** Test (behavioural test scenarios).

**SR‑TEST‑003: Gold Conformance Criteria**
> A component implementation SHALL be considered **Gold** conformant if it meets all Silver criteria AND:
> - Implements all ARIA roles and attributes as specified.
> - Supports keyboard interactions as described in the spec.
> - Handles all defined edge cases (invalid inputs, boundary values) correctly.  
> **Rationale:** Gold represents full, production‑ready compliance.  
> **Traceability:** SR‑009 (BRS)  
> **Verification Method:** Test (ARIA checks, keyboard simulation, edge‑case scenarios).

**SR‑TEST‑004: ComponentHarness Interface**
> The conformance test suite SHALL define a `ComponentHarness` interface (trait/protocol) that each framework implementation must implement. The harness SHALL provide methods for:
> - Rendering a component with given props.
> - Updating props (for controlled reactivity tests).
> - Triggering events.
> - Inspecting the rendered output (DOM/accessibility tree/native view hierarchy).
> - Cleaning up after tests.  
> **Rationale:** Enables a single test suite to validate any implementation.  
> **Traceability:** SR‑010 (BRS)  
> **Verification Method:** Inspection (interface definition), Test (reference harness implementation).

**SR‑TEST‑005: Conformance Test Scenarios**
> For each component, the conformance test suite SHALL include automated test scenarios covering:
> - **Bronze**: Prop/event name presence checks.
> - **Silver**: Reactivity behaviour (controlled prop updates, uncontrolled initial values, static prop immutability).
> - **Gold**: ARIA attribute presence, keyboard navigation, edge‑case handling.  
> **Rationale:** Provides objective, repeatable conformance validation.  
> **Traceability:** BO‑4, SR‑011 (BRS)  
> **Verification Method:** Test (suite execution).

**SR‑TEST‑006: Machine‑Readable Conformance Report**
> The conformance test suite SHALL produce a JSON report containing:
> - Implementation identifier (name, version).
> - UCP spec version tested against.
> - Per‑component results: Bronze/Silver/Gold status, with detailed failure reasons.  
> **Rationale:** Enables automated dashboard updates and CI integration.  
> **Traceability:** SR‑011 (BRS)  
> **Verification Method:** Test (report output validation).

---

### 4.3 Conformance Dashboard

**CAP‑DASH‑01: Public Conformance Visibility**

**SR‑DASH‑001: Dashboard Data Ingestion**
> The conformance dashboard SHALL accept conformance reports (in the JSON format defined by SR‑TEST‑006) via an HTTP API or Git‑based submission (e.g., pull request to a data repository).  
> **Rationale:** Automates dashboard updates from CI pipelines.  
> **Traceability:** BO‑4  
> **Verification Method:** Test (API endpoint).

**SR‑DASH‑002: Implementation Registry**
> The dashboard SHALL maintain a registry of participating implementations, including:
> - Name and description.
> - Repository URL.
> - Framework/language.
> - Contact/maintainer information.
> - Conformance report submission history.  
> **Rationale:** Provides context for conformance data.  
> **Traceability:** BO‑5  
> **Verification Method:** Inspection.

**SR‑DASH‑003: Conformance Matrix Display**
> The dashboard SHALL display a matrix of components (rows) vs implementations (columns), with each cell showing:
> - Conformance tier (Bronze/Silver/Gold) as a colour‑coded badge.
> - Last tested date.
> - Link to detailed report.  
> **Rationale:** Provides at‑a‑glance conformance visibility.  
> **Traceability:** BO‑4  
> **Verification Method:** Demonstration.

**SR‑DASH‑004: Historical Trends**
> The dashboard SHALL display historical conformance trends for each implementation (e.g., number of Gold components over time).  
> **Rationale:** Encourages continuous improvement.  
> **Verification Method:** Test (data query).

**SR‑DASH‑005: Accessibility**
> The conformance dashboard web interface SHALL meet WCAG 2.1 Level AA accessibility standards.  
> **Rationale:** The dashboard itself should exemplify accessibility best practices.  
> **Traceability:** NFR‑005 (BRS)  
> **Verification Method:** Inspection (WCAG checklist), Test (automated a11y scans).

---

### 4.4 UCP CLI Tool

**CAP‑CLI‑01: Developer Tooling**

**SR‑CLI‑001: Specification Validation**
> The UCP CLI SHALL provide a `validate` command that checks a local implementation's component exports against the UCP spec and outputs a conformance report.  
> **Rationale:** Enables local testing before CI submission.  
> **Traceability:** BO‑3  
> **Verification Method:** Test.

**SR‑CLI‑002: Code Generation**
> The UCP CLI SHALL provide a `generate` command that, given a component name and target framework (via FMD), produces a skeleton component file with:
> - Correct imports.
> - Props interface/types defined.
> - Stubbed event handlers.
> - Comments linking to UCP spec.  
> **Rationale:** Accelerates implementation and reduces manual errors.  
> **Traceability:** BO‑3, SR‑012 (BRS)  
> **Verification Method:** Test (output comparison).

**SR‑CLI‑003: Framework Mapping Document Loading**
> The CLI SHALL load Framework Mapping Documents from a local file or remote URL. FMDs SHALL be validated against a JSON Schema.  
> **Rationale:** Ensures consistent mapping rules.  
> **Traceability:** SR‑013 (BRS)  
> **Verification Method:** Test (schema validation).

---

### 4.5 Governance and Evolution

**CAP‑GOV‑01: Spec Enhancement Proposal Process**

**SR‑GOV‑001: SEP Submission**
> The UCP system SHALL provide a mechanism for submitting Spec Enhancement Proposals (SEPs). This MAY be implemented as GitHub issue templates, pull request templates, or a dedicated web form.  
> **Rationale:** Formalises the change process.  
> **Traceability:** SR‑014 (BRS)  
> **Verification Method:** Inspection.

**SR‑GOV‑002: SEP Approval Workflow**
> A SEP SHALL be considered approved only when:
> - It has been open for review for at least 14 calendar days.
> - It has received approving reviews from at least one Spec Editor from three different language ecosystems (JavaScript/TypeScript, Rust, Swift/Kotlin).  
> **Rationale:** Ensures cross‑ecosystem consensus.  
> **Traceability:** SR‑014 (BRS)  
> **Verification Method:** Inspection (governance documentation).

**SR‑GOV‑003: Spec Editor Rotation**
> The list of active Spec Editors SHALL be publicly maintained in the UCP repository. Editor terms SHALL be 12 months, with a rotation schedule published.  
> **Rationale:** Prevents capture by any single ecosystem.  
> **Traceability:** SR‑015 (BRS)  
> **Verification Method:** Inspection.

---

## 5. Quality and Non‑Functional Requirements

Non‑functional requirements are organized by ISO/IEC 25010:2023 quality characteristics. Each NFR is expressed with a measurable fit criterion.

### 5.1 Performance Efficiency

**NFR‑PERF‑001: Specification Parsing Time**
> The JSON Schema representation of the full UCP component catalogue (50+ components) SHALL be parsable by standard JSON tooling in ≤ 100 ms on typical developer hardware (as defined in Appendix B).  
> **Fit Criterion:** Parsing benchmark completes within 100 ms for 95% of runs.  
> **Verification Method:** Test (automated benchmark).

**NFR‑PERF‑002: Conformance Test Suite Execution Time**
> The conformance test suite for a single implementation, when run against the core 15 components, SHALL complete in ≤ 5 minutes on CI infrastructure (2 vCPU, 8 GB RAM).  
> **Fit Criterion:** Suite duration ≤ 300 seconds in CI logs.  
> **Verification Method:** Test (CI timing measurement).

**NFR‑PERF‑003: Dashboard Page Load Time**
> The conformance dashboard SHALL load and render the conformance matrix (50 components × 10 implementations) in ≤ 3 seconds on a standard broadband connection.  
> **Fit Criterion:** Lighthouse performance score ≥ 90.  
> **Verification Method:** Test (Lighthouse CI).

### 5.2 Reliability

**NFR‑REL‑001: Specification Availability**
> The UCP specification (JSON Schema files) SHALL be available for download from package registries and Git with 99.9% uptime (excluding scheduled maintenance).  
> **Fit Criterion:** Uptime monitoring confirms ≥ 99.9% availability monthly.  
> **Verification Method:** Analysis (uptime monitoring), Test (synthetic checks).

**NFR‑REL‑002: Dashboard Availability**
> The conformance dashboard SHALL be available with 99% uptime, excluding scheduled maintenance.  
> **Fit Criterion:** Uptime monitoring confirms ≥ 99% availability monthly.  
> **Verification Method:** Analysis (uptime monitoring).

### 5.3 Security

**NFR‑SEC‑001: Specification Integrity**
> The UCP specification files SHALL be signed (e.g., with Sigstore/cosign) or provide checksums to verify integrity.  
> **Fit Criterion:** Verification command succeeds for official releases.  
> **Verification Method:** Test (signature verification).

**NFR‑SEC‑002: Dashboard Security**
> The conformance dashboard SHALL:
> - Enforce HTTPS for all connections.
> - Sanitize all user‑submitted data (implementation names, descriptions) to prevent XSS.
> - Require authentication for report submission (API key or OAuth).  
> **Fit Criterion:** OWASP ZAP scan reports no high‑risk alerts.  
> **Verification Method:** Test (security scanning).

### 5.4 Maintainability

**NFR‑MAINT‑001: Specification Change Impact**
> For 80% of non‑breaking spec changes (e.g., adding a new prop), the required update to a conforming implementation SHALL be limited to adding the new prop (no existing code changes required).  
> **Fit Criterion:** Maintainer survey confirms expectation met.  
> **Verification Method:** Analysis (change impact review).

**NFR‑MAINT‑002: Test Suite Modularity**
> The conformance test suite SHALL be organized such that adding a new component requires creating a new test file only, without modifying core harness code.  
> **Fit Criterion:** New component addition completed in ≤ 2 files changed.  
> **Verification Method:** Inspection (code review).

### 5.5 Compatibility

**NFR‑COMP‑001: Backward Compatibility**
> Within a major version, UCP specification changes SHALL be backward‑compatible: no existing prop or event shall be removed or have its type changed in a breaking way without a deprecation period.  
> **Fit Criterion:** Automated schema diff tool confirms no breaking changes.  
> **Verification Method:** Test (CI check).

**NFR‑COMP‑002: JSON Schema Version**
> The UCP specification SHALL use JSON Schema draft‑07 or later, ensuring broad tool compatibility.  
> **Fit Criterion:** Schema validates against the declared draft meta‑schema.  
> **Verification Method:** Test (schema validation).

### 5.6 Accessibility (Interaction Capability)

**NFR‑ACC‑001: Dashboard Accessibility**
> The conformance dashboard SHALL conform to WCAG 2.1 Level AA.  
> **Fit Criterion:** axe‑core automated scan passes with zero violations; manual keyboard navigation test passes.  
> **Verification Method:** Test (automated + manual a11y audit).

---

## 6. External Interfaces and Data Contracts

### 6.1 UCP Specification Schema (JSON)

**Interface Purpose:** Machine‑readable definition of component APIs for tooling consumption.

**Format:** JSON Schema (draft‑07 or later) with additional UCP‑specific vocabulary.

**Key Requirements:**
- **SR‑SPEC‑001 through SR‑SPEC‑008** define the required schema elements.
- The root object SHALL contain a `$schema` property, a `version` field (SemVer), and a `components` map keyed by component ID.
- Example (abbreviated):
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "ucpVersion": "1.0.0",
  "components": {
    "button": {
      "name": "button",
      "description": "A clickable button element.",
      "category": "form",
      "since": "1.0.0",
      "props": [
        {
          "name": "variant",
          "type": "enum",
          "values": ["default", "destructive", "outline", "secondary", "ghost", "link"],
          "reactivity": "static",
          "default": "default"
        }
      ],
      "events": [
        { "name": "onClick", "payload": "function" }
      ],
      "slots": [
        { "name": "default", "required": true }
      ],
      "aria": {
        "implicitRole": "button",
        "requiredAttributes": []
      }
    }
  }
}
```

### 6.2 Conformance Report Format

**Interface Purpose:** Machine‑readable conformance results for dashboard ingestion.

**Format:** JSON.

**Schema (key fields):**
```json
{
  "implementation": {
    "id": "leptos-shadcn",
    "name": "Leptos shadcn/ui",
    "version": "0.5.0",
    "framework": "Leptos (Rust)",
    "testedAt": "2026-04-21T10:30:00Z"
  },
  "ucpSpecVersion": "1.0.0",
  "results": [
    {
      "component": "button",
      "bronze": { "passed": true },
      "silver": { "passed": true },
      "gold": { "passed": false, "failures": ["missing aria-pressed for toggle button"] }
    }
  ]
}
```

### 6.3 Dashboard Submission API

**Interface Purpose:** Accept conformance reports from CI systems.

- **Endpoint:** `POST /api/v1/reports`
- **Authentication:** Bearer token (API key) per registered implementation.
- **Request Body:** JSON conforming to the Conformance Report Format (Section 6.2).
- **Response:** `201 Created` with report ID, or `400 Bad Request` with validation errors.

### 6.4 Framework Mapping Document Schema

**Interface Purpose:** Define how UCP abstract types map to concrete framework idioms.

**Format:** JSON Schema.

**Example (partial):**
```json
{
  "framework": "react",
  "version": "1.0.0",
  "naming": {
    "component": "PascalCase",
    "prop": "camelCase",
    "event": "camelCase (onXxx)"
  },
  "typeMappings": {
    "string": "string",
    "boolean": "boolean",
    "number": "number",
    "function": "() => void",
    "node": "React.ReactNode"
  },
  "reactivity": {
    "controlled": "value + onChange",
    "uncontrolled": "defaultValue",
    "static": "prop"
  }
}
```

---

## 7. Constraints, Assumptions, and Dependencies

### 7.1 Design and Implementation Constraints
- **C‑001: Language‑Agnostic Core** – The UCP specification SHALL NOT contain framework‑specific syntax. All framework details reside in FMDs.
- **C‑002: Open Source Licensing** – All UCP artefacts (spec, test suite, CLI, dashboard) SHALL be released under the MIT license.
- **C‑003: Git‑Based Workflow** – Specification changes SHALL be managed via Git pull requests and the SEP process.

### 7.2 Assumptions
- **A‑001:** Library maintainers are willing to implement the `ComponentHarness` interface for their framework.
- **A‑002:** Participating implementations have CI systems capable of running the conformance test suite.
- **A‑003:** The community will provide reference FMDs for major frameworks (React, Vue, Leptos, etc.).

### 7.3 Dependencies
| Dependency | Description | Version |
|------------|-------------|---------|
| JSON Schema | Specification format | draft‑07+ |
| Node.js | Runtime for CLI tool (optional, could be Rust) | ≥ 18 |
| GitHub | Hosting, CI, issue tracking | N/A |
| Package Registries | Distribution of spec packages | crates.io, npm, etc. |

---

## 8. Risks and Open Issues ("TBD" Log)

| TBD ID | Description | Owner | Status |
|--------|-------------|-------|--------|
| TBD‑001 | Define precise "standard developer hardware" for NFR‑PERF‑001 benchmark. | System Architect | Open |
| TBD‑002 | Determine if Gold+ tier should include visual regression testing. | Spec Editors | Deferred |
| TBD‑003 | Finalise authentication mechanism for dashboard report submission (API keys vs OAuth). | Dashboard Lead | Open |
| TBD‑004 | Define exact list of components for UCP v1.0 (target: 50+). | Spec Editors | In Progress |

---

## 9. Requirements Attributes and Traceability Model

### 9.1 Requirement Identifiers
- **Functional Requirements:** `SR‑<CAPABILITY>‑<NNN>` (e.g., `SR‑SPEC‑001`)
- **Non‑Functional Requirements:** `NFR‑<CHARACTERISTIC>‑<NNN>` (e.g., `NFR‑PERF‑001`)
- **Constraints:** `C‑<NNN>`

### 9.2 Attributes
Each requirement shall have the following attributes (maintained in the requirements management tool):
- **ID** (unique)
- **Type** (Functional / NFR / Constraint)
- **Priority** (MoSCoW: Must / Should / Could / Won't)
- **Status** (Proposed / Approved / Implemented / Verified)
- **Source** (BRS requirement ID, stakeholder, etc.)
- **Verification Method** (Inspection / Analysis / Demonstration / Test)
- **Trace Links** (upward to BRS goals, downward to test cases)

### 9.3 Traceability Model
The following trace links are maintained:

| Upward | Requirement | Downward |
|--------|-------------|----------|
| BRS Goal | SRS Requirement | Test Case / Verification Artefact |

Example:
- `BO‑1` (API Convergence) → `SR‑SPEC‑001` (Component Identifier) → `TC‑SPEC‑001` (Schema validation test)
- `SR‑TEST‑001` (Bronze Criteria) → `TC‑BRONZE‑BUTTON‑001` (Button name check)

---

## 10. Appendices

### Appendix A: Glossary
See separate `GLOSSARY.md` document in the UCP repository.

### Appendix B: Reference Hardware Specification
For performance benchmarks (NFR‑PERF‑001), "typical developer hardware" is defined as:
- CPU: Apple M1 or Intel i7 equivalent (4+ cores, 2.5+ GHz)
- RAM: 16 GB
- OS: macOS, Linux, or Windows 11
- Node.js version: Latest LTS

### Appendix C: References
- UCP Vision & Strategic Alignment v1.0
- UCP Business & Stakeholder Requirements Specification v1.0
- ISO/IEC/IEEE 29148:2018 – Systems and software engineering — Life cycle processes — Requirements engineering
- Semantic Versioning 2.0.0 (https://semver.org)

---

*End of Software Requirements Specification*
