# UCP Transformation Plan — 0.12.0 → 0.13.0

## Verdict on 0.12.0

The pipeline exists and ~115 tests pass, but the flagship promise — *unify the same
component across framework ports* — fails on the simplest possible input:

- TSX extractor loses every prop declared after an inline callback type
  (`=>` decrements the angle-bracket depth below zero in `parse_props_from_body`).
- TS primitives (`boolean`, `string`, `number`) are unmapped → `AbstractPropType::Any`.
- `SemanticFingerprint` hashes **raw** prop names, so `onClick` and `on_click` never
  cluster: React `Button` + Leptos `Button` merge into **two** `unified:Button`s with
  **zero** conflicts.
- Presence drift (prop exists in port A, missing in port B) is never reported.
- `ucp validate` loads a file and prints stats; it validates nothing.

Buzzword cargo (removed in 0.13.0): Ollama LLM enrichment (which mutated semantic
fingerprints from LLM output, breaking determinism), GitHub discovery agent
(octocrab), MCP server, "AI contract", HTML dashboard, A2UI/AG-UI exports, fake
`review.html`, and the "multi-agent" roadmap.

## 0.13.0 identity

**UCP is a deterministic, offline, fast API-drift detector and spec unifier for
multi-framework component libraries.** Static analysis *is* the product. No LLM.

Pillars:
1. **Correctness** — real tokenizer for TSX, TS-aware type ontology, naming-convention-
   normalized fingerprints.
2. **Determinism** — identical input → byte-identical spec, always.
3. **Performance** — rayon parallel parse (existing), incremental mtime+hash cache for
   watch/CI re-runs, lean dependency tree (octocrab/reqwest/askama/spdx dropped).
4. **DX** — honest CLI surface, `--json` machine output, real exit codes, structural
   validation, docs that match reality.

## Workstreams (TDD: failing test first, then implementation)

| # | Workstream | Crate |
|---|------------|-------|
| 2 | `normalize_prop_name` (camel/snake/kebab, `on`-prefix → event name) + proptest | ucp-core |
| 3 | TS type ontology: primitives, string-literal unions, generic arrays | ucp-synthesizer/unify |
| 4 | Tokenizer-based TSX interface scanner (fixes arrow-swallow, strings, comments) | ucp-synthesizer/extract |
| 5 | Fingerprint v2 over normalized names; presence conflicts in merge | ucp-synthesizer |
| 6 | Vaporware removal + dependency diet | workspace |
| 7 | CLI DX: honest about, `--json`, exit codes, structural `validate` | ucp-cli |
| 8 | Incremental extraction cache (`cache_hits` stat, serde-defaulted) | ucp-synthesizer |
| 9 | Criterion benches: extraction, merge | ucp-synthesizer/benches |
| 10 | Docs honesty pass; version 0.13.0 | docs/ |

## Compatibility

- Spec JSON field names unchanged; old specs still load.
- `purpose_hash` values change (normalization) — acceptable pre-1.0, noted in CHANGELOG.
- `stats.cache_hits` added with `#[serde(default)]` for back-compat.
- Removed: CLI subcommands `mcp`, `mcp-server-json`, `contract`, `dashboard`;
  export targets `a2ui`, `ag-ui`; flags `--ollama-url`, `--llm-model`, bootstrap `--watch`.
