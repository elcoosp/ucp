# ADR-0002: Use Semantic Fingerprint for Deduplication

**Status:** Accepted
**Date:** 2026-04-27

## Context

When merging specs from multiple codebases, components that represent
the same logical entity (e.g., a Button) need to be identified even
when their identifiers differ across frameworks.

## Decision

We will compute a `SemanticFingerprint` consisting of a `purpose_hash`
(derived from the component name, optionally enriched by LLM) and
`normalized_prop_names`. Components with the same purpose hash are
considered equivalent and merged.

## Alternatives Considered

- **Exact ID matching:** Too strict; different frameworks use different
  naming conventions.
- **Fully LLM‑based matching:** Too slow and non‑deterministic for
  programmatic use.

## Consequences

**Positive:**
- Robust deduplication without requiring identical names
- LLM enrichment can improve matching quality
- Hash‑based comparison is fast

**Negative:**
- Different components with similar names may collide (low probability)
- LLM enrichment is optional, so hash quality varies
