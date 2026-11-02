# UCP Roadmap

> Last updated: 2026-11. Philosophy: ship working static analysis, not visions.

## Shipped in 0.13.0

- Tokenizer-based TSX extractor (fixes silent prop loss after inline callbacks).
- TypeScript type ontology: `boolean`/`string`/`number`, literal unions, suffix arrays,
  `React.`-namespaced types.
- Cross-framework unification: naming-convention normalization (`onClick` = `on_click`),
  name-driven fingerprint clustering, presence-drift conflicts.
- Structural `validate` with `--json` and real exit codes.
- Incremental parse cache (O(changed) re-runs for watch/CI) + criterion benches.
- Removal of the LLM/Ollama enrichment, GitHub discovery agent, MCP server, "AI
  contract", dashboard, A2UI/AG-UI exporters, and the fake review page. See
  [TRANSFORMATION.md](TRANSFORMATION.md).

## Next (0.14 — candidates, in rough priority order)

1. **Vue and Svelte extractors to parity.** Both exist as rough scanners; port them to
   the tokenizer approach and property-test them like TSX.
2. **Conflict resolution presets for CI.** `ucp merge --fail-on drift` so pipelines can
   fail on presence drift but tolerate type variants.
3. **Per-port profiles in one spec.** Scope port-specific props instead of flagging
   them (the CAM `ResolutionStrategy::ScopeToProfile` path exists but has no UI yet).
4. **Ship a release binary + install script.** `cargo install` today; add
   GitHub Releases artifacts.

## Under consideration (no promises)

- JSON Schema export of the CAM (`schemars` is already a dependency).
- `ucp lsp`: hover info for component props inside consuming apps.
- State-machine extraction from SMDL files embedded next to components (parser exists,
  pipeline wiring is stubbed out).

## Explicitly rejected

- LLM-based extraction or enrichment. Static analysis is the product; hallucinated
  props are a liability, not a feature.
- Speculative "agent protocol" exports. Formats get added when a real consumer exists.
