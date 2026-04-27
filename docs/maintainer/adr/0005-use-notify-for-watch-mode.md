# ADR-0005: Use `notify` for Watch Mode

**Status:** Accepted
**Date:** 2026-04-27

## Context

UCP v0.11's watch mode needs to detect file changes in a source
directory and trigger re‑extraction, merge, and export.

## Decision

We will use the `notify` crate for cross‑platform file system event
monitoring. The watcher is configured with a debounce interval to
prevent rapid‑fire rebuilds on bulk changes.

## Alternatives Considered

- **Polling loop (manual):** Simple but CPU‑intensive for large directories.
- **`watchexec` integration:** Already used for `ucp bootstrap --watch`,
  but coupling to an external tool is less flexible.

## Consequences

**Positive:**
- Cross‑platform (inotify on Linux, FSEvents on macOS)
- Native event‑based notifications
- Configurable debounce

**Negative:**
- Adds dependency on platform‑specific system APIs
- Debounce logic adds complexity
