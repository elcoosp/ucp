# ADR-0003: Use Ratatui for Curation TUI

**Status:** Accepted
**Date:** 2026-04-27

## Context

UCP v0.11 requires an interactive terminal interface for conflict
curation. The maintainer needs to navigate conflicts, view details,
and apply resolutions.

## Decision

We will use the `ratatui` crate for building the TUI, with `crossterm`
as the terminal backend. This provides a reactive, widget‑based TUI
framework that supports custom layouts.

## Alternatives Considered

- **`inquire`:** Simpler for form‑based input, but less flexible for
  complex layouts with side‑by‑side information.
- **Plain stdin/stdout prompt loop:** Functional but poor UX for
  navigating multiple conflicts.

## Consequences

**Positive:**
- Rich, interactive interface
- Keyboard‑navigable
- Well‑maintained Rust ecosystem

**Negative:**
- Larger dependency footprint
- Learning curve for widget‑based UI
- Requires raw terminal mode, which may not work in all environments
