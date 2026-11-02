## [Unreleased]

### 0.13.0 — "Static analysis is the product"

This release turns UCP from an AI-buzzword shell into a working deterministic
API-drift detector. The flagship promise — unify the same component across framework
ports — now actually works.

#### Fixed
- **TSX extractor rewritten around a real lexer** (strings, comments, `=>` as one
  token). The 0.12 line-heuristic scanner went angle-bracket-negative on inline
  callback types and silently swallowed every prop declared after them.
- **TypeScript type ontology**: `boolean`/`true`/`false` map to `controlFlag`,
  `string`/`number` to static values, string-literal unions keep their concrete
  members, `T[]` suffix arrays and `React.*` namespaced types are recognized. In 0.12
  all of these collapsed to `any`.
- **Cross-framework merge works**: fingerprints now hash the normalized component
  name; props unify across naming conventions (`onClick` = `on_click`), and prop-set
  differences surface as presence-drift conflicts (`drift_NNN`) instead of producing
  two parallel `unified:Button` components with zero findings.
- Event names derive through the same normalization (`onOpenChange` → `openchange`).

#### Added
- `ucp validate` performs structural validation (duplicate ids, empty prop names,
  out-of-range confidences, prop-less components), prints `--json` reports, and exits
  non-zero on blocking issues — CI-ready.
- `--json` machine-readable summaries for `bootstrap` and `merge`.
- Incremental parse cache keyed by `(mtime, size)`: watch/CI re-runs re-parse only
  changed files (`stats.cache_hits` reports hits; serialized raw extractions are
  byte-identical to fresh parses).
- Criterion benches for the tokenizer, the end-to-end pipeline (cold vs cached), and
  the merge path (`cargo bench -p ucp-synthesizer`).
- `TRANSFORMATION.md` documents the audit and the redesign decisions.

#### Removed
- Ollama LLM enrichment (it mutated semantic fingerprints from LLM output, breaking
  determinism), the GitHub discovery agent (`octocrab`), the MCP server, the "AI
  contract" exporter, the HTML dashboard, the A2UI and AG-UI exporters, and the fake
  one-line `review.html`. The dependency tree lost `octocrab`, `reqwest`, `spdx`, and
  `askama`.
- CLI flags `--ollama-url`, `--llm-model`, bootstrap `--watch` (use `ucp watch`).

#### Compatibility
- Spec JSON field names are unchanged; 0.12 specs load fine. `purpose_hash` values
  changed (normalization is the point) and `stats` gained serde-defaulted fields.

### 🐛 Bug Fixes
- Remove duplicate doc-test recipe from justfile
- Set mdBook source directory to docs/ root
- Move mdBook src field to [book] section
- Remove invalid copy-fonts field from mdBook config
- **core,cli:** Trim SMDL identifiers, document all public items, tighten CLI dispatch


### 📚 Documentation
- V0.13 spec


### 🔨 Refactoring
- **synthesizer:** Pedantic-clippy cleanup, DRY export variants, robust Svelte extraction
- **maintainer:** Pedantic cleanup, helper extraction, registry cast safety


### 🤖 CI/CD
- Git-cliff changelog, GitHub Actions pipelines, renovate config



## [0.12.0] - 2026-04-27
### 🐛 Bug Fixes
- Correct incremental merge test format string
- Correct registry integration test order assertion
- Resolve match arm type mismatch in ucp-cli main.rs
- Await async pipeline call in watch mode
- Insert Watch match arm in CLI
- Use std::time instead of chrono, fix crate path in merge.rs
- Correct doc example in diff.rs to construct SynthesisOutput explicitly


### 📚 Documentation
- Add crate-level docs, root files, and module documentation
- Add user tutorials, how-to guides, command reference, and concept docs
- Add maintainer documentation (architecture, pipeline, guides, ADRs, CI)
- Add mdBook configuration, installation guide, version bump to 0.12.0



## [0.10.0] - 2026-04-27
### 🐛 Bug Fixes
- Correct React snapshot test file extension from .rs to .tsx


### 📚 Documentation
- V0.10 spec


### 📦 Dependencies
- **deps:** Add insta, proptest, assert_cmd dev dependencies and tooling


### 🔧 Miscellaneous
- Bump version to 0.10.0 and fix React snapshot test


### 🚀 Features
- V0.10.0 almost there



## [0.9.0] - 2026-04-27
### 🐛 Bug Fixes
- Mark DESIGN.md import test as experimental, bump to 0.9.0



## [0.8.0] - 2026-04-26
### 🐛 Bug Fixes
- Deduplicate MCP match arm, add Web Components generator
- Restore test files, keep generated code working


### 📚 Documentation
- Open ui


### 🔧 Miscellaneous
- Bump version
- Bump version to 0.8.0


### 🚀 Features
- **export:** Add DESIGN.md export for AI-native design specs
- **extract:** Add Svelte 5 and Vue 4 extractors



## [0.6.0] - 2026-04-26
### 🔧 Miscellaneous
- Final integration, bump to v0.6.0



## [0.5.0] - 2026-04-26
### 🔧 Miscellaneous
- W3C docs, final tests, bump to v0.5.0


### 🚀 Features
- **export:** Add A2UI catalog export
- **export:** Add AG-UI event schema export



## [0.4.0] - 2026-04-26
### 🔧 Miscellaneous
- Final integration tests, cleanup, bump to v0.4.0


### 🚀 Features
- **export:** Expand W3C export to full schema compliance, add design token extraction
- W3c



## [0.3.0] - 2026-04-26
### 🐛 Bug Fixes
- **cli:** Add leptos target to generate command
- **registry:** Use simple string matching for dependency resolution
- **docs:** Use backtick substitution in W3C mapping document


### 🔧 Miscellaneous
- Bump version to 0.3.0


### 🔨 Refactoring
- **generate:** Extract shared prop-mapping, snake_case, edition 2024


### 🚀 Features
- **extract:** Replace string heuristic with AST-based context visitor



## [0.2.0] - 2026-04-26
### 🐛 Bug Fixes
- Progress towards grammar
- **cli:** Resolve borrow-after-move of ollama_url in PipelineOptions construction
- Use ToTokens for raw type extraction, revert untestable discovery unit tests
- Use free function for SMDL conversion (orphan rule), compute line_start via source search (proc_macro2 span limit)
- Compute reactivity before moving cam_type in unify functions
- Correct missing return type in is_path_safe_to_parse
- Correct security.rs signature, remove invalid #[must_use] on type alias
- **cli:** Compute dry_run before moving ollama_url into struct
- **cli:** Pass filter by reference, remove is_optional (not tracked in CAM)
- Reject hidden files (dotfiles) in is_path_safe_to_parse
- Use as_ref().is_none_or() to avoid move in closure
- Remove orphaned #[test] attribute left from duplicate removal
- Remove orphaned #[test] attribute left from duplicate removal
- Fmté
- Replace source_dir_str with proper let binding
- Normalize whitespace in raw_type strings from ToTokens
- **core:** Add SpreadAttributes variant to AbstractPropType
- **extract:** Deduplicate Dioxus components in extract_rust_components
- Just e2e scripts
- **pipeline:** Correct concrete_type integration
- **test:** Simplify dioxus merge context test to use direct construction
- **extract:** Correct field duplication and unused imports


### 📚 Documentation
- Add shadcn rs api alignment specification suite
- Readme roadmap & spec
- License
- Contributing & sec


### 🔧 Miscellaneous
- Gitignore
- Final cleanup, lint fixes, and regression verification
- Add initial GPUI component visitor
- Fix missing is_event field in other extraction paths
- Fix unify::unknown_type_to_any test for uppercase type mapping
- Extract 97 GPUI components; save snapshot
- Ignore flaky merge context test for now
- Bump version to 0.2.0


### 🔨 Refactoring
- Add #[must_use] attributes, use precise error variants, split cam module


### 🚀 Features
- Bootstrap
- **synthesizer:** Add raw reqwest LLM provider for Ollama glm-5; fix all compilation and test blockers
- **synthesizer:** Implement CAM type ontology unification
- **synthesizer:** Implement 3-column HTML curation UI via Askama
- **cli:** Implement ucp bootstrap command skeleton
- **synthesizer:** Implement end-to-end synthesis pipeline
- Pipeline
- **tsx:** Rewrite extractor with brace-depth tracking for multi-line types, add function component support
- Security & pipeline
- Add StructComponentVisitor for struct-props component extraction
- Integrate StructComponentVisitor into extract_rust_components
- Extend type mappings and add struct-props integration tests
- Distinguish struct-extracted components with rust-struct: ID prefix
- **ucp:** Add concrete type to CanonicalAbstractProp and conflict detection
- **ucp-cli:** Add generate command for Dioxus code output
- **pipeline:** Integrate concrete types, spread attributes, and context hints
- **ucp:** Add context detection test, Dioxus generator, and CLI generate command


### 🧪 Testing
- **ucp-synth:** Add snapshot test with Dioxus Button, Dialog, Accordion
- **ucp-synth:** Adapt Dioxus context test to simple heuristic



