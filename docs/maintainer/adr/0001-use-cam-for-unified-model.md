# ADR-0001: Use Canonical Abstract Model for Unified Representation

**Status:** Accepted
**Date:** 2026-04-27

## Context

UCP needs a single representation for UI components extracted from
multiple frameworks (React, Dioxus, Leptos, GPUI, etc.). These frameworks
represent the same concepts differently (e.g., `boolean` vs `bool` vs
`Signal<bool>`).

## Decision

We will use the **Canonical Abstract Model (CAM)** as the intermediate
representation. The CAM defines abstract types (ControlFlag, StaticValue,
ControlledValue, etc.) that capture the *semantics* of a prop without
tying to any specific framework's syntax.

## Alternatives Considered

- **Unified Type System (e.g., TypeScript types):** Too complex to map
  all Rust type patterns to a single type system.
- **Per‑framework schemas:** Would make merging impossible without
  manual mapping.

## Consequences

**Positive:**
- Framework‑agnostic specs enable cross‑framework merging
- Clean separation between extraction and generation
- Easier to add new frameworks

**Negative:**
- Requires mapping logic for every framework
- Abstract types lose some framework‑specific detail
- Concrete type tracking adds complexity
