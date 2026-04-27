# ADR-0004: Use SQLite for Local Spec Registry

**Status:** Accepted
**Date:** 2026-04-27

## Context

UCP v0.11 needs a persistent local store for UCP specs, allowing
maintainers to store, list, retrieve, and delete specs.

## Decision

We will use SQLite (via `rusqlite` with bundled mode) as the storage
backend. The schema is a single table with an auto‑incrementing ID,
name, JSON content, and timestamp.

## Alternatives Considered

- **`sled` (embedded key‑value store):** Simpler API, no SQL dependency,
  but less support for structured queries and migration.
- **Filesystem (JSON files):** No dependency, but no indexing or querying.

## Consequences

**Positive:**
- Familiar SQL interface for querying
- Bundled mode requires no system dependency
- ACID transactions
- Easy to inspect/debug with any SQLite client

**Negative:**
- Adds `libsqlite3-sys` compile time
- Small overhead vs plain key‑value store
