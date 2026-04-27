# UCP v0.12 – Documentation Release: Research & Plan

Below is the research synthesis and actionable documentation plan for v0.12, focusing exclusively on **Markdown files and code documentation** (no dedicated website).

---

## 1. Research Synthesis

### 1.1 The Diátaxis Framework — A Proven Structure for Technical Docs

The **Diátaxis framework** is increasingly the gold standard for structuring technical documentation. It identifies four distinct user needs, each requiring a different writing approach:

| Mode | Answers | Focus |
|------|---------|-------|
| **Tutorials** | "How do I get started?" | Learning-oriented, step-by-step |
| **How-to Guides** | "How do I accomplish X?" | Task-oriented, solving real problems |
| **Reference** | "What are the exact specs?" | Information-oriented, dry and complete |
| **Explanation** | "Why does it work this way?" | Understanding-oriented, background |

The key insight: tutorials and how-to guides are about *action*, while reference and explanation are about *cognition*. They should be kept separate because a reader looking for a quick answer doesn't want a tutorial, and a learner doesn't want a dry reference page.

This maps perfectly to UCP's v0.12 needs: user docs are mostly tutorials + how-to + reference (CLI commands), while maintainer docs are how-to + reference + explanation + decision records.

### 1.2 Rust Documentation Ecosystem — Two Complementary Tools

Rust provides **two documentation tools** that work together:

1. **`rustdoc` + `cargo doc`**: Extracts doc comments (`///` and `//!`) from source code and generates HTML. Supports full Markdown, code examples that are compiled as tests (`cargo test --doc`), and standard section headings like `# Examples`, `# Panics`, `# Errors`, and `# Safety`. Best practice is to document every `pub` interface with examples that are run as tests.

2. **`mdBook`**: A standalone tool (Rust-built) for creating book-style documentation from Markdown files in a `docs/` directory. Uses a `SUMMARY.md` for navigation and a `book.toml` for configuration. Commonly used alongside rustdoc for higher-level documentation (guides, architecture, contributing).

The recommended pattern from several open-source Rust projects: **crate-level documentation via rustdoc** for API reference and inline docs, **standalone Markdown books via mdBook** for tutorials, guides, and architecture docs. Both live in the same repository.

### 1.3 Audience-Split Documentation — README vs CONTRIBUTING vs GUIDE

Multiple sources converge on splitting documentation by audience in a GitHub-native way:

- **`README.md`** → for **users**: project name, badges, short description, quick-start examples, installation, basic usage, links to full docs.
- **`CONTRIBUTING.md`** → for **new contributors**: setup instructions, development workflow, coding standards, PR process, issue labeling.
- **`GUIDE.md` or `MAINTAINERS.md`** → for **maintainers**: release process, governance, architecture overview, decision history, merge/curation workflows.

This three-file split at the root level gives each audience exactly what they need without noise.

### 1.4 Code Documentation — What "Well-Documented" Rust Crates Do

The Rust API guidelines (`C_CRATE_DOC`, `C_EXAMPLE`) and practitioner consensus say:
- **Crate-level docs** (`//!` at the top of `lib.rs`) should explain what the crate does, link to key modules, and include a minimal usage example.
- **Every `pub` item** should have `///` docs with at minimum a description and an `# Examples` section.
- **Code examples in docs should compile** — `cargo test --doc` ensures they don't rot.
- **Module-level docs** (`//!` in `mod.rs`) should give an overview of the module's purpose and how its pieces fit together.

### 1.5 Architecture Decision Records (ADRs) as Maintainer Docs

ADR/MADR (Markdown Architectural Decision Records) is a widely adopted practice in open-source and enterprise projects for documenting *why* decisions were made. Key patterns:
- Store ADRs in `docs/adr/` with sequential numbering.
- Each ADR captures: context, decision, alternatives considered, consequences.
- ADRs are never deleted — superseded decisions are marked as such with links to the replacement.
- This is already part of UCP's `exec-plan.md` skill guidance; v0.12 should produce a full ADR log for the major v0.11 architectural decisions.

### 1.6 CLI Documentation — Beyond `--help`

For CLI tools specifically, multiple sources confirm:
- Comprehensive `--help` text using clap's `about`, `long_about`, and per-argument `help` strings.
- README should include a command reference table or `--help` output summary.
- Each subcommand should have usage examples in its help text.
- A dedicated `docs/commands/` directory with one Markdown file per subcommand for deeper walkthroughs.

### 1.7 Markdown-Only Documentation — What Works

The broader movement toward "Markdown-first" developer tools is well established. Key principles:
- **Co-locate docs with code** in the same repo — documentation can be forked, merged, and reviewed just like code.
- Use `docs/` as the top-level directory with subdirectories by topic.
- Keep files focused (under ~300 lines) and use clear naming conventions.
- Version control gives documentation history for free; no database or CMS needed.

---

## 2. Proposed v0.12 Documentation Structure

```
ucp/
├── README.md                          # User-facing: what, why, quickstart
├── CONTRIBUTING.md                    # Developer-facing: setup, workflow, PRs
├── MAINTAINERS.md                     # Maintainer-facing: release, governance
├── docs/
│   ├── SUMMARY.md                     # Table of contents for mdBook
│   ├── book.toml                      # mdBook configuration
│   ├── index.md                       # Documentation home / overview
│   ├── user/
│   │   ├── installation.md            # How to install UCP (Homebrew, cargo)
│   │   ├── quickstart.md              # 5-minute tutorial: bootstrap → spec
│   │   ├── tutorial-bootstrap.md      # Full tutorial: scanning a codebase
│   │   ├── tutorial-merge.md          # Full tutorial: merging two specs
│   │   ├── tutorial-curate.md         # Full tutorial: interactive curation
│   │   ├── tutorial-export.md         # Full tutorial: exporting to all formats
│   │   ├── howto-ci.md                # How-to: integrate UCP in CI/CD
│   │   ├── howto-watch.md             # How-to: continuous watch mode
│   │   ├── howto-tokens.md            # How-to: merge design tokens
│   │   ├── commands/
│   │   │   ├── index.md              # Command reference overview
│   │   │   ├── bootstrap.md          # ucp bootstrap reference
│   │   │   ├── merge.md              # ucp merge reference
│   │   │   ├── curate.md             # ucp curate reference
│   │   │   ├── diff.md               # ucp diff reference
│   │   │   ├── verify.md             # ucp verify reference
│   │   │   ├── registry.md           # ucp registry reference
│   │   │   ├── export-all.md         # ucp export-all reference
│   │   │   └── watch.md              # ucp watch reference
│   │   └── concepts/
│   │       ├── cam.md                 # Explanation: Canonical Abstract Model
│   │       ├── specs.md              # Explanation: UCP spec format
│   │       ├── provenance.md         # Explanation: provenance tracking
│   │       └── drift.md              # Explanation: drift detection
│   └── maintainer/
│       ├── architecture.md            # Architecture overview (already exists)
│       ├── pipeline.md               # How the extraction pipeline works
│       ├── merge-algorithm.md        # How merge/dedup/conflict detection works
│       ├── adding-extractor.md       # How-to: add a new framework extractor
│       ├── adding-exporter.md        # How-to: add a new export format
│       ├── release-process.md        # Release checklist and steps
│       ├── testing.md                # Test strategy and how to run tests
│       ├── adr/
│       │   ├── 0001-use-cam-for-unified-model.md
│       │   ├── 0002-use-semantic-fingerprint-for-dedup.md
│       │   ├── 0003-use-ratatui-for-curation-tui.md
│       │   ├── 0004-use-sqlite-for-registry.md
│       │   └── 0005-use-notify-for-watch-mode.md
│       └── ci-setup.md               # CI pipeline documentation
```

---

## 3. Code Documentation Plan (rustdoc)

For each crate in the workspace, v0.12 will ensure:

### 3.1 `ucp-core` (already well-documented, needs polishing)
- [x] Crate-level doc exists in `lib.rs`
- [ ] Add module-level docs to `cam/mod.rs`, `smdl/mod.rs`, `error.rs`
- [ ] Ensure every `pub` struct has `///` docs with examples
- [ ] Ensure every `pub` function has `///` docs

### 3.2 `ucp-synthesizer` (partially documented)
- [ ] Add comprehensive crate-level doc (`//!` in `lib.rs`) explaining extraction → unification → merge → export pipeline
- [ ] Document all public modules with module-level docs
- [ ] Add `# Examples` sections to key public functions: `merge_specs`, `run_pipeline`, export functions, generate functions
- [ ] Document `MergeOptions`, `PipelineOptions`, `SynthesisOutput` fields

### 3.3 `ucp-maintainer` (new in v0.11, needs full documentation)
- [ ] Crate-level doc explaining the maintainer toolkit purpose
- [ ] Document all public types: `DiffReport`, `TokenMergeOptions`, `DriftReport`, `SpecStore`, `Resolution`
- [ ] Document all public functions: `diff_specs`, `merge_token_files`, `verify_spec_against_source`, `curate_spec`, `run_curation_tui`, `run_watch`
- [ ] Each public function gets `# Examples`, `# Panics` (if applicable), `# Errors`

### 3.4 `ucp-cli` (minimal docs currently)
- [ ] Add crate-level doc explaining the binary
- [ ] Document the CLI architecture (how subcommands dispatch)
- [ ] Ensure every `#[command]` has `about` and `long_about` filled in

---

## 4. Key Documentation Artifacts to Create

### 4.1 User-Facing

| File | Type (Diátaxis) | Content |
|------|-----------------|---------|
| `README.md` | Landing | Vision, badges, install, 3-line quickstart, links |
| `docs/user/quickstart.md` | Tutorial | 5-min bootstrap a React repo → see the spec |
| `docs/user/tutorial-merge.md` | Tutorial | Bootstrap two repos → merge → inspect conflicts |
| `docs/user/tutorial-curate.md` | Tutorial | Run `ucp curate` → resolve conflicts → export |
| `docs/user/commands/*.md` | Reference | One file per subcommand, flag tables, examples |
| `docs/user/concepts/cam.md` | Explanation | What the CAM is, why it's framework-agnostic |
| `docs/user/howto-ci.md` | How-to | GitHub Actions example for auto-verify on PR |

### 4.2 Maintainer-Facing

| File | Type | Content |
|------|------|---------|
| `CONTRIBUTING.md` | How-to | Dev setup, just recipes, test commands, PR process |
| `MAINTAINERS.md` | Reference | Release checklist, governance, current maintainers |
| `docs/maintainer/architecture.md` | Explanation | C4 diagrams, crate responsibilities (already exists) |
| `docs/maintainer/pipeline.md` | Explanation | Extraction → unification → merge flow explained |
| `docs/maintainer/adding-extractor.md` | How-to | Step-by-step for adding a new framework extractor |
| `docs/maintainer/release-process.md` | How-to | Version bump, test, tag, publish checklist |
| `docs/maintainer/adr/*.md` | Decision record | MADR-format ADRs for key v0.11 decisions |

---

## 5. Implementation Plan (Suggested Sequencing)

### Phase 1: Foundation (crate-level docs + root files)
1. Write `//!` crate-level docs for `ucp-maintainer`, `ucp-synthesizer`, `ucp-cli`
2. Write module-level docs for every `pub mod`
3. Write/update `README.md`, `CONTRIBUTING.md`, `MAINTAINERS.md`

### Phase 2: User Documentation
4. Write `docs/user/quickstart.md` (5-minute tutorial)
5. Write `docs/user/tutorial-*.md` (4 tutorials)
6. Write `docs/user/commands/*.md` (8 command references)
7. Write `docs/user/concepts/*.md` (4 explanations)
8. Write `docs/user/howto-*.md` (3 how-to guides)

### Phase 3: Maintainer Documentation
9. Write `docs/maintainer/pipeline.md`, `docs/maintainer/merge-algorithm.md`
10. Write `docs/maintainer/adding-extractor.md`, `docs/maintainer/adding-exporter.md`
11. Write `docs/maintainer/release-process.md`, `docs/maintainer/testing.md`
12. Write `docs/maintainer/adr/*.md` (5 ADRs for v0.11 decisions)

### Phase 4: Inline API Documentation
13. Add `///` docs to every `pub` function in `ucp-maintainer`
14. Add `///` docs to key `pub` functions in `ucp-synthesizer`
15. Add `# Examples` sections with compilable code snippets
16. Configure `cargo test --doc` in CI

### Phase 5: mdBook Integration
17. Set up `docs/book.toml` and `docs/SUMMARY.md`
18. Add `just doc` recipe that runs `mdbook build docs/`
19. Optionally add `just doc-open` to open in browser

---

If you'd like, I can generate any of these documentation files now, or produce the full monolithic Bash script for the v0.12 documentation implementation.
