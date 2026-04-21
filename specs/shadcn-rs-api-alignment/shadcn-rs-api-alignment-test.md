# Behavioral Specification & Test Verification Plan

| Field | Value |
|-------|-------|
| Project | Shadcn UI API Alignment for Quoin UCP |
| Document | Behavioral Spec & Test Verification |
| Version | 0.1 (Draft) |
| Date | 2026-04-21 |
| Author | System Architect (assisted by AI) |
| Status | Draft — Pending Review |

## 1. Behavioral Specifications (Specification by Example)

These scenarios define the expected behavior for components. They serve as both acceptance criteria and the basis for conformance tests.

### 1.1 Button Component

**Scenario: Default button renders correctly**
- **Given** a Button with variant "default" and children "Click me"
- **When** rendered
- **Then** DOM contains `<button>` with class `bg-primary text-primary-foreground` and text "Click me"
- **And** `role="button"`

**Scenario: Button click fires callback**
- **Given** a Button with `onClick` handler
- **When** clicked
- **Then** handler invoked exactly once.

**Scenario: Disabled button does not fire callback**
- **Given** a Button with `disabled=true` and `onClick` handler
- **When** clicked
- **Then** handler not invoked.

**Scenario: Loading button shows spinner**
- **Given** a Button with `loading=true`
- **When** rendered
- **Then** button contains an SVG with class `animate-spin`
- **And** `aria-busy="true"`.

### 1.2 Input Component

**Scenario: Required field shows error on blur**
- **Given** an Input with `required=true` and empty value
- **When** blurred
- **Then** error message "This field is required" displayed
- **And** `aria-invalid="true"`.

**Scenario: Email validation fails**
- **Given** an Input with `type="email"`
- **When** value "not-an-email" is entered and blurred
- **Then** error message indicates invalid email.

## 2. Test Strategy & Plan

### 2.1 Test Suite Structure

The conformance test suite is a separate crate (`shadcn-rs-conformance`) that both libraries use as a dev‑dependency.

**Running tests for Leptos:**
 ```bash
cd leptos-shadcn-ui
cargo test --features conformance
 ```

**Running tests for Dioxus:**
 ```bash
cd dioxus-shadcn-ui
cargo test --features conformance
 ```

### 2.2 Test Harness

Each library provides a minimal test harness that:
- Mounts a component with given props.
- Provides a way to simulate events.
- Exposes the DOM for querying.

The conformance suite calls into this harness via a trait.

### 2.3 Coverage Goals

- **Phase 1:** Core components (Button, Input, Card, Dialog, etc.) – 100% scenario coverage.
- **Phase 2:** All remaining components – 100% scenario coverage.

## 3. Test Case Specifications (Examples)

| ID | Requirement | Preconditions | Steps | Expected Result |
|----|-------------|---------------|-------|-----------------|
| TC‑BTN‑001 | F‑BEH‑001 | Button variant=destructive | Render | `<button>` has class `bg-destructive` |
| TC‑INP‑001 | F‑BEH‑003 | Input required=true, empty | Focus then blur | Error message displayed |
| TC‑DLG‑001 | F‑BEH‑002 | Dialog open | Press Tab 5 times | Focus trapped in dialog |

## 4. Conformance Dashboard

A static site (GitHub Pages) displays, for each component:
- Spec version.
- Leptos conformance status (pass/fail).
- Dioxus conformance status (pass/fail).

This provides transparency and motivates alignment.

## 5. Requirements Traceability Matrix (RTM) Excerpt

| SRS Requirement | Test Case | Verification Method |
|-----------------|-----------|---------------------|
| F‑BEH‑001 | TC‑BTN‑001 | Conformance Test |
| F‑BEH‑003 | TC‑INP‑001 | Conformance Test |
| F‑BEH‑002 | TC‑DLG‑001 | Conformance Test |

## 6. Living Documentation

- The common spec is the source of truth for component APIs.
- The conformance dashboard shows real‑time compliance.
- Quoin's UCP renderer uses the spec to generate code, ensuring it stays in sync.
