# Software Requirements Specification (SRS)

| Field | Value |
|-------|-------|
| Project | Shadcn UI API Alignment for Quoin UCP |
| Document | Software Requirements Specification |
| Version | 0.1 (Draft) |
| Date | 2026-04-21 |
| Author | System Architect (assisted by AI) |
| Status | Draft — Pending Review |

## 1. Introduction and Scope

### 1.1 Purpose
This SRS defines the functional and non‑functional requirements for:
1. The **Common API Specification** for shadcn/ui components.
2. The **Conformance Test Suite** that validates Leptos‑shadcn/ui and Dioxus‑shadcn/ui against the spec.
3. The required updates to the existing libraries to achieve conformance.

### 1.2 Scope
- All 50+ shadcn/ui components.
- Public API surface: component names, prop names, event names, ARIA roles.
- Behavioral consistency (DOM output, event handling, validation).

**Out‑of‑Scope:**
- Internal implementation details of the libraries.
- Visual styling or CSS class names.

## 2. System Context and Overview

The system comprises:
- **Common API Specification:** JSON schemas defining component APIs.
- **Leptos‑shadcn/ui** and **Dioxus‑shadcn/ui** – existing libraries that will be updated to conform.
- **Conformance Test Suite:** WebDriver‑based tests validating DOM output against the spec.

## 3. Functional Capabilities and Behavior

### 3.1 Capability: Common API Specification

| ID | Requirement (EARS Pattern) | Priority |
|----|----------------------------|----------|
| **F‑SPEC‑001** | The common spec shall define a unique identifier for each component (e.g., `"button"`). | Must |
| **F‑SPEC‑002** | For each component, the spec shall list all props, including name, description, and whether the prop is reactive. | Must |
| **F‑SPEC‑003** | The spec shall define supported events, including name and payload type. | Must |
| **F‑SPEC‑004** | The spec shall define expected ARIA roles, states, and properties. | Must |
| **F‑SPEC‑005** | The spec shall be stored in a machine‑readable format (JSON Schema) to enable tooling. | Must |

### 3.2 Capability: Library API Alignment

| ID | Requirement | Priority |
|----|-------------|----------|
| **F‑LIB‑001** | The Leptos‑shadcn/ui library shall expose components with names matching the common spec. | Must |
| **F‑LIB‑002** | The Dioxus‑shadcn/ui library shall expose components with names matching the common spec. | Must |
| **F‑LIB‑003** | Prop names in both libraries shall match the common spec. | Must |
| **F‑LIB‑004** | Event handler names shall match the common spec (e.g., `on_click` in Leptos, `onclick` in Dioxus; the spec defines the logical name, and each library uses its idiomatic casing). | Must |
| **F‑LIB‑005** | If a component exists in only one library, the missing library shall implement it within two release cycles. | Should |

### 3.3 Capability: Behavioral Consistency

| ID | Requirement | Priority |
|----|-------------|----------|
| **F‑BEH‑001** | For equivalent props, the generated DOM structure (element types, class names, ARIA attributes) shall be identical in both libraries. | Must |
| **F‑BEH‑002** | Event handling semantics (when events fire, payload structure) shall be identical. | Must |
| **F‑BEH‑003** | Validation rules and error display shall be identical. | Should |

### 3.4 Capability: Conformance Test Suite

| ID | Requirement | Priority |
|----|-------------|----------|
| **F‑TEST‑001** | A shared test suite shall be developed that can be executed against both libraries. | Must |
| **F‑TEST‑002** | The test suite shall cover at least 90% of behavioral requirements. | Should |
| **F‑TEST‑003** | Test results shall be published as a dashboard showing per‑component compliance. | Should |

## 4. Quality and Non‑functional Requirements

| ID | Category | Requirement | Fit Criterion |
|----|----------|-------------|---------------|
| **NFR‑MAIN‑001** | Maintainability | The common spec shall be versioned semantically. | Major version changes indicate breaking API changes. |
| **NFR‑COMP‑001** | Compatibility | Libraries shall continue to support their minimum Rust version and WASM target. | Existing CI passes. |

## 5. External Interfaces and Data Contracts

### 5.1 Common Spec Format (Example: Button)

 ```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://shadcn-rs.org/spec/button.json",
  "title": "Button",
  "props": {
    "variant": {
      "type": "string",
      "enum": ["default", "destructive", "outline", "secondary", "ghost", "link"],
      "default": "default",
      "reactive": true
    },
    "size": {
      "type": "string",
      "enum": ["default", "sm", "lg", "icon"],
      "default": "default",
      "reactive": true
    },
    "disabled": { "type": "boolean", "default": false, "reactive": true },
    "loading": { "type": "boolean", "default": false, "reactive": true }
  },
  "events": {
    "onClick": { "description": "Fired when the button is clicked." }
  },
  "aria": { "role": "button", "attributes": ["aria-disabled", "aria-busy"] }
}
 ```

### 5.2 Library API Examples (Conforming)

**Leptos‑shadcn/ui (target):**
 ```rust
#[component]
pub fn Button(
    #[prop(into, optional)] variant: MaybeProp<ButtonVariant>,
    #[prop(into, optional)] size: MaybeProp<ButtonSize>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] loading: MaybeProp<bool>,
    #[prop(optional)] on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView
 ```

**Dioxus‑shadcn/ui (target):**
 ```rust
#[component]
pub fn Button(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    disabled: Option<bool>,
    loading: Option<bool>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element
 ```

## 6. Constraints, Assumptions, and Dependencies

- **C‑001:** Leptos library uses Leptos v0.8+.
- **C‑002:** Dioxus library uses Dioxus v0.7+.
- **A‑001:** Both library maintainers agree to align with the common spec.
- **D‑001:** Conformance test suite depends on `fantoccini` and `wasm-bindgen-test`.

## 7. TBD Log

| ID | Item | Owner |
|----|------|-------|
| TBD‑001 | Finalize JSON Schema for all components. | Spec Editor |
| TBD‑002 | Define deprecation and migration plan for breaking changes. | Maintainers |

## 8. Requirements Attributes and Traceability Model

- **ID Scheme:** `F‑{CATEGORY}‑{NNN}`.
- Each requirement traces to a Stakeholder Need (SN‑xxx) from the BRS.
