# UCP v0.11 Specification Suite — Generated

Below are the five specification documents for the **UCP v0.11 “Spec Maintainer Toolkit”** release.
They have been produced using the `spec-writer` skill, with project context auto‑filled from the
prior brainstorm, the codebase dump, and targeted web research. Each document follows the structure
and quality criteria defined in the respective reference guide.

---

## 1. Product Vision & Strategic Alignment (`ucp-v011-vision.md`)

```markdown
# UCP v0.11 – Product Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | UCP (Universal Component Protocol) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP product team, assisted by AI |
| Status | Draft — Pending Review |

## 1. Vision & Elevator Pitch

**Vision (Pichler/Cagan style):**  
Make maintaining a unified, multi-source component specification as effortless as bootstrapping
one, by giving maintainers powerful CLI tools that merge, diff, curate, and govern specs with
surgical precision – so that component libraries stay coherent as they grow across codebases.

**Elevator Pitch (Moore’s template):**  
For **spec maintainers and design‑system librarians** who are tired of manually merging diverging
UCP specs from different codebases and losing provenance, UCP v0.11 is a **spec‑governance CLI**
that **semi‑automates merging, conflict curation, token unification, drift detection, and registry
publishing** – unlike today’s manual reconciliation, UCP v0.11 provides a guided interactive review
and a complete audit trail from extraction to consumable spec export.

## 2. Problem Statement & Business Context

Component libraries (shadcn‑style, design systems) are increasingly bootstrapped from multiple
frontend codebases (React, Dioxus, Leptos, etc.) using the existing UCP bootstrapper. The resulting
UCP specs are valuable, but **today there is no streamlined way to merge, curate, and maintain
them over time** when new codebases appear or source code evolves. Maintainers resort to manual
JSON diffing, ad‑hoc conflict resolution, and guesswork about which props come from which origin.
This is error‑prone, slow, and discourages adoption of multi‑source unification.

The business opportunity is to turn UCP from a one‑shot bootstrapper into a **governance platform**
for component specifications, enabling teams to continuously integrate new codebases, resolve
conflicts, publish curated specs to multiple formats, and monitor drift – all from the CLI.
This dramatically increases UCP’s value proposition and opens the door for registry‑based
distribution and enterprise use.

## 3. Target Users / Customers

- **Spec Maintainers** (the primary persona): developers or design‑system engineers responsible
  for maintaining canonical UCP specs that aggregate multiple codebases.
- **Design‑system Librarians**: those who manage tokens, assets, and component libraries across
  orgs; they need to merge token sets and publish DESIGN.md/registry files.
- **Platform Engineers**: who integrate UCP output into CI/CD pipelines and want to automate
  spec updates as source repositories change.
- **Open‑source maintainers** of cross‑framework component libraries (e.g., shadcn‑dioxus)
  who want to keep their registry entries up‑to‑date from several framework ports.

**Explicitly NOT targeting (non‑goals):**  
- End‑user developers who only consume the final generated components (they are downstream).
- Design tooling (Figma plugin) – out of scope for v0.11.

## 4. User Needs & Value Proposition

**Top 3 needs:**
1. “I need to merge UCP specs from different codebases without manually resolving every prop
   conflict by hand, while keeping a record of why decisions were made.”
2. “I need to see what changed between two versions of a spec, or between a spec and the original
   source code, so I can trust that my spec is still accurate.”
3. “I need to publish the merged spec to multiple target formats (A2UI, AG‑UI, W3C, registry, etc.)
   in one command and have the spec serve as the source of truth for component contracts.”

**Key differentiator:**  
Unlike generic merge/diff tools, UCP v0.11 understands component semantics (props, events, tokens,
state machines) and can suggest resolutions based on existing conflict metadata and (optionally)
LLM reasoning. It also maintains cryptographic provenance of every spec change via patch‑ts and
the provenance log.

## 5. Desired Outcomes & Success Metrics

| Outcome | Key Results |
|---------|-------------|
| O‑1: Reduce manual merge effort | Maintainers report ≥ 60% reduction in time spent reconciling props across specs after merging via `ucp curate` compared to current JSON‑diff workflow (measured by beta user survey) |
| O‑2: Increase spec accuracy | Less than 5% of props in a merged spec drift from their source codebases after 4 weeks (measured by `ucp verify` checks) |
| O‑3: Broaden ecosystem adoption | 3 – 5 open‑source component libraries adopt UCP v0.11’s merge/publish flow and generate their registries using it within 3 months of release |
| O‑4: Accelerate spec release cadence | Median time from new codebase extraction to published fused spec dropped by 50% (from ~2 hours to ≤1 hour) |

## 6. Strategic Constraints

- **Must integrate with the existing UCP codebase** (Rust workspace, just build, cargo nextest).
- Must be CLI‑first, with TUI for interactive curation.
- Must run on macOS (primary dev environment), Linux (CI), and ideally Windows (future).
- Must not break existing extraction/unification pipeline (backward compatibility).
- Must respect the current data model (CAM) and not require a breaking change to the core crate.
- LLM‑assisted curation is optional and requires external Ollama instance; offline mode must work.

## 7. Goals and Non‑goals

**Goals (v0.11 scope):**
- Provide `ucp curate` – interactive terminal merge curation.
- Provide `ucp diff` – structural spec diff.
- Provide `ucp merge-tokens` – merge DTCG token sets.
- Provide `ucp verify` – re‑extraction-based drift detection.
- Provide `ucp registry` – persistent local spec store and `pull`.
- Provide `ucp export-all` – single command to run all exporters.
- Provide `ucp watch` – file‑watch driven auto‑merge/auto‑export.
- Add provenance tracking (merge history) to `SynthesisOutput`.
- Introduce incremental merge and weighted merge strategies.
- Extend pipeline options for watch and continuous integration.

**Non‑goals (explicitly out of scope for v0.11):**
- A graphical web UI for curation (the TUI is the v0.11 limit; web UI may come later).
- Full‑fledged bi‑directional sync with source repos (v0.11 only detects drift; no automatic
  source‑code rewriting).
- Integration with external registries beyond shadcn v4 structure (no npm registry push).
- Real‑time collaboration features.
- Full compliance with formal standards like WCAG AA in the TUI (best effort).

## 8. Operational Concept & High‑Level Scenarios

**Concept of Operations:**  
A spec maintainer bootstraps several codebases into individual UCP spec files. They then run:
```bash
ucp curate --merged merged.json --output canonical.json
```
The tool launches an interactive terminal where the maintainer walks through each detected
conflict, accepts/rejects resolution suggestions, and upon completion the curated spec is saved.
They then run:
```bash
ucp export-all --spec canonical.json --output ./final
```
which generates all export formats simultaneously. Later, when source code changes, they run
```bash
ucp verify --spec canonical.json --source-dir ./new-codebase
```
to see a drift report.

**Key scenarios:**
1. *First‑time merge*: Merge two bootstrapped specs into a single canonical spec, resolve 10–15
   conflicts interactively, and export.
2. *Incremental update*: A new codebase is bootstrapped; run `ucp merge --incremental` to add
   its components to the existing canonical spec, with selective curation.
3. *Token unification*: Merge CSS custom property tokens from tailwind configs of two frameworks
   into a unified DTCG JSON.
4. *Audit and provenance*: Use `ucp diff` to compare today’s spec against last month’s baseline
   and show which decisions were made and why.
5. *Watch mode*: Run `ucp watch --source-dir ./src --spec canonical.json --output ./live` so that
   every save triggers re‑extraction and re‑export automatically.

## 9. Stakeholders, Sponsorship, and Governance

- **Executive Sponsor:** UCP project lead (elcoosp).
- **Product Lead:** UCP core team (collective ownership).
- **Key stakeholder groups:** spec maintainers (primary), open‑source library authors, framework
  integration developers.
- **Document owner:** UCP core team; reviewed quarterly.

## 10. Traceability & Alignment Notes

All goals (O‑1..O‑4) map to epics and features. The traceability scheme uses:
- Goal IDs: O‑1 .. O‑4
- Stakeholder needs: SN‑01..SN‑05
- System requirements (SRS): REQ‑FUNC‑xxx, NFR‑xxx

## 11. Risks, Assumptions, and Open Questions

**Assumptions:**
- The existing merge algorithm and conflict detection will serve as a solid base; no rewrite needed.
- Terminal UI library (ratatui or inquire) is suitable for future maintenance.
- Users will have Ollama available if they wish to use LLM assistance.

**Risks:**
- Complexity creep: too many commands may overwhelm maintainers; mitigation: start with the
  top‑three (curate, diff, export-all) and release others as experimental.
- Performance: merging large specs (hundreds of components) may be slow; mitigation: benchmark
  early and use incremental strategies.
- Backward compatibility: changes to `SynthesisOutput` for provenance might break existing
  serialization; mitigation: version the format and provide migration tooling.

**Open questions:**
- Should the TUI be mandatory or optional (with a CLI‑only batch mode)?
- Exact default weights for weighted merge?
- Which provenance fields are strictly needed for the initial rollout?
```

---

## 2. Business & Stakeholder Requirements Specification (`ucp-v011-brs.md`)

```markdown
# Business & Stakeholder Requirements Specification for UCP v0.11

| Field | Value |
|-------|-------|
| Project | UCP (Universal Component Protocol) |
| Document | Business & Stakeholder Requirements Specification (BRS) |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP team, assisted by AI |
| Status | Draft — Pending Review |

## 1. Business Context

**Purpose:**  
Enable spec maintainers and design‑system librarians to continuously manage and evolve unified
component specifications that aggregate multiple codebases, by providing a suite of CLI tools
for merging, curation, diffing, drift detection, token unification, local registry storage, and
batched exporting. This moves UCP from a one‑shot bootstrap tool to a **spec governance platform**.

**Business problem:**  
Today, merging two UCP specs requires manual JSON editing, with no assistance for conflict
resolution, no tracking of decisions, and no automation to keep the spec up‑to‑date. This
prevents wider adoption of multi‑source component catalogs.

**Scope boundaries:**  
In‑scope: CLI/TUI tools for merge curation, spec diff, token merge, drift verification, local
spec storage, batch export, and file‑watch integration.  
Out‑of‑scope: web‑based dashboard (may be future), integration with external package registries
beyond local file generation, or real‑time collaborative editing.

## 2. Business Goals, Objectives & Success Metrics

See Vision §5 for detailed OKRs. They are referenced here as BG‑01..BG‑04 with the same
descriptions. For traceability:

- BG‑01: Reduce manual merge effort (≥60% reduction)
- BG‑02: Increase spec accuracy (<5% prop drift)
- BG‑03: Broaden ecosystem adoption (3–5 libraries)
- BG‑04: Accelerate release cadence (50% faster spec release)

## 3. Business Model & Processes

**High‑level value streams:**
1. **Continuous Integration of Component Libraries:**  
   Codebases are scanned and bootstrapped → specs are merged and curated → curated spec is exported
   to multiple formats → spec is stored in a local registry. When source code changes, drift
   detection alerts maintainers, who then re‑merge and re‑export.
2. **Token system merge:** Separate token sets from multiple design systems are merged, conflicts
   resolved, and a unified DTCG token file is produced alongside DESIGN.md.

**Workshop output (Event Storming – big picture):**  
Key domain events: `Spec Bootstrapped`, `Merge Requested`, `Conflict Detected`, `Resolution Decided`,
`Curated Spec Published`, `Drift Detected`, `Registry Updated`. Hotspots: conflict resolution
heuristics, token value semantic clashes.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑001 | When a merge yields prop type conflicts, the system must present both alternatives and a suggested resolution based on confidence and majority. | Industry standard for merge tools |
| BR‑002 | Any decision made during interactive curation must be recorded with a timestamp and the user’s identity, forming an audit trail. | Provenance requirement |
| BR‑003 | Exported registries must conform to the shadcn v4 registry schema. | Community specification |
| BR‑004 | Token merging must preserve the original source of each token value; when values differ, the conflict must be flagged for manual resolution. | DTCG best practices |

## 5. Stakeholders & User Classes

**Stakeholder map:**

| Stakeholder | Role | Influence | Key needs |
|-------------|------|-----------|------------|
| Spec Maintainer | Primary user | High – direct user of all tools | Efficient conflict resolution, provenance, export pipeline |
| Design‑system Librarian | Secondary user | Medium – uses token merge and export | Unified token sets, DESIGN.md generation |
| Platform / CI Engineer | Enabler | Medium – integrates into pipelines | Watch mode, idempotent batch commands |
| Open‑source library author | Beneficiary | Low – downstream consumer of published specs | Clear, trusted specs; automated registry generation |

**User classes:**
- **Spec Maintainer (primary):** daily use, expects interactive TUI and fast batch.
- **CI Runner (secondary):** non‑interactive, expects exit codes and JSON output.

**Key persona – “Alex the Spec Maintainer”:**  
Alex maintains a cross‑framework component library for shadcn‑dioxus. He pulls new specs from the
React and Dioxus codebases monthly, spends hours comparing props manually, and dreads drift.
He wants a tool that saves time and gives him confidence in the published spec.

## 6. Glossary / Ubiquitous Language

| Term | Definition |
|------|------------|
| Spec / UCP spec | A JSON file conforming to `SynthesisOutput` structure, containing one or more component definitions |
| Canonical spec | The authoritative, curated spec resulting from merging and curation |
| Merge | The process of combining multiple `SynthesisOutput` objects into one, using semantic fingerprint deduplication |
| Curation | Interactive review and resolution of conflicts detected during merge |
| Provenance | Metadata that records which original specs contributed to which parts of a merged spec, plus curation decisions |
| Drift | Desynchronization between a canonical spec and the source code that originally produced it |
| DTCG tokens | Design tokens following the Design Token Community Group format (colors, spacing, typography) |
| Registry (shadcn) | A set of JSON files following the shadcn v4 registry schema, used to distribute component code |

## 7. Conceptual Domain Model

Core entities (high‑level):
- `ComponentSpec` (a collection of `CanonicalAbstractComponent` objects)
- `MergeSession` (inputs, algorithm parameters, result, conflicts)
- `Conflict` (id, field, alternatives, resolution, decision record)
- `CurationDecision` (who, when, what was decided, rationale)
- `TokenSet` (colors, spacing, typography, with sources)
- `DriftReport` (component name, prop differences, confidence)
- `RegistryIndex` (list of `RegistryItem` with files)

Relationships: A `MergeSession` produces a merged `ComponentSpec` and a set of `Conflict`s.
A `CurationDecision` resolves a `Conflict`. A `ComponentSpec` can be verified against source
code to produce a `DriftReport`. A `ComponentSpec` can be stored in a local `SpecStore`.

## 8. Stakeholder Needs & User Requirements

**As a Spec Maintainer** (SN‑01):
- I want to merge multiple UCP spec files from different codebases and resolve conflicts
  interactively, so that I have a single, coherent spec.
- I want to see the provenance of every component and prop after a merge, so I can trace
  decisions.
- I want to export the curated spec to several target formats with one command, so that
  I save time.

**As a Design‑system Librarian** (SN‑02):
- I want to merge CSS design tokens from different framework configs into a unified DTCG
  JSON, so that I can maintain a single source of truth for styling.

**As a Platform Engineer** (SN‑03):
- I want to run drift detection in CI and fail the build if the spec has diverged beyond
  acceptable thresholds, so that we avoid shipping stale specs.

**As an Open‑source author** (SN‑04):
- I want to publish my library to a shadcn‑compatible registry using UCP’s registry generation,
  with automatic resolution of framework references (namespaced dependencies).

(Detailed user requirements with IDs to be decomposed in SRS.)

## 9. System‑in‑Context & Operational Concept

(Summarized from Vision §8, expanded here:)
The UCP v0.11 tools extend the existing `ucp` CLI with subcommands: `curate`, `diff`,
`merge‑tokens`, `verify`, `registry`, `export‑all`, and `watch`. The system interacts with
the file system (reading/writing JSON spec files, token files, registry directories) and
optionally with an Ollama instance for LLM assistance. It is integrated into the existing
Rust workspace (`ucp‑cli` crate) and leverages the `ucp‑synthesizer` library.

## 10. Stakeholder‑Level Constraints & Quality Expectations

- **Usability:** A spec maintainer should be able to perform a first‑time merge and curation
  of two specs (≤20 components, ≤15 conflicts) within 10 minutes of training.
- **Performance:** Merging 100 components across 3 specs should complete in under 5 seconds
  on typical developer hardware.
- **Reliability:** The merge algorithm must be deterministic; given the same inputs, it must
  produce the same merge result.
- **Compatibility:** All new commands must work on macOS (primary) and Linux (CI).
- **Security:** If LLM assistance is used, network calls must be optional and data must not
  be sent without user opt‑in.

## 11. Risks, Assumptions & Open Issues

(List carried forward from Vision, with additional details:)
- Assumption: The TUI will be implemented using `ratatui` or `inquire`. Both are feasible;
  decision to be made in architecture.
- Risk: TUI may not be accessible via screen readers; mitigation: provide a non‑interactive
  batch mode for all curation operations.

## 12. Traceability Mapping to Vision

| Business Goal (Vision) | Stakeholder Needs | Features (Epics) |
|------------------------|-------------------|------------------|
| BG‑01 | SN‑01 (merge & curate) | `ucp curate`, `ucp merge --incremental` |
| BG‑02 | SN‑03 (drift detection) | `ucp verify` |
| BG‑03 | SN‑04 (registry) | `ucp registry`, `ucp export‑all` |
| BG‑04 | SN‑01, SN‑03 | `ucp watch`, pipeline integration |

```

---

## 3. Software Requirements Specification (`ucp-v011-srs.md`)

```markdown
# Software Requirements Specification for UCP v0.11

| Field | Value |
|-------|-------|
| Project | UCP (Universal Component Protocol) |
| Document | Software Requirements Specification (SRS) |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP team, assisted by AI |
| Status | Draft — Pending Review |

## 1. Introduction & Scope

This SRS specifies the software requirements for the v0.11 “Spec Maintainer Toolkit” release
of the UCP CLI and synthesizer library. It builds on the stakeholder needs captured in the BRS
and the Vision’s goals. This document defines functional and non‑functional requirements,
external interfaces, and constraints. It does **not** describe implementation details
(architecture), which are covered in the Architecture document.

**References:**  
- UCP Vision v0.11 (ucp-v011-vision.md)  
- UCP BRS v0.11 (ucp-v011-brs.md)  
- Existing UCP codebase (v0.10.0)  
- shadcn Registry v4 Schema (web reference)

## 2. System Context & Overview

The UCP v0.11 CLI is an extension of the existing `ucp` binary. It adds new subcommands that
work on local files and directories. The system context includes:

- **Users:** maintainers and CI agents (see BRS user classes).
- **External systems:** file system, optional Ollama instance for LLM assistance.
- **Inputs:** UCP spec JSON files (SynthesisOutput), DTCG token files, source directories.
- **Outputs:** merged/curated specs, diff reports, drift reports, registry JSONs, export
  files (A2UI, AG‑UI, DESIGN.md, etc.), local spec database.

## 3. Functional Capabilities & Behavior

Requirements are grouped by feature, using EARS syntax. Priority: MoSCoW.

### FEAT‑01: Interactive Merge Curation (`ucp curate`)

**REQ‑FUNC‑001 (Must):**  
*Event‑driven:* When the user invokes `ucp curate --merged <merged.json> --output <canonical.json>`, the system shall load the merged spec (which may contain unresolved conflicts), display each conflict sequentially in an interactive terminal interface, and allow the user to choose a resolution from the available options (including custom input), then save the curated spec with resolved conflicts and an embedded curation log.

**REQ‑FUNC‑002 (Should):**  
*Optional feature:* Where an Ollama instance is available and `--llm` flag is passed, the system shall for each conflict present an LLM‑generated resolution suggestion alongside the confidence score.

**REQ‑FUNC‑003 (Must):**  
*Ubiquitous:* The system shall preserve all non‑conflicting data exactly as in the merged spec and shall update each conflict’s status to “resolved” with the decision details.

**REQ‑FUNC‑004 (Must):**  
*Unwanted behaviour:* If the output file already exists and the user has not specified `--force`, the system shall prompt for confirmation before overwriting.

**REQ‑FUNC‑005 (Should):**  
*State‑driven:* While in curation mode, the user may navigate (next/previous conflict), skip, or exit, and the system shall save all decisions made so far to a recovery file to avoid data loss on unexpected termination.

### FEAT‑02: Spec Diff (`ucp diff`)

**REQ‑FUNC‑010 (Must):**  
*Event‑driven:* When the user runs `ucp diff --spec-a <a.json> --spec-b <b.json>`, the system shall compute a structural difference report showing added/removed/changed components and props, and output it in human‑readable text (with optional `--json` for machine output).

**REQ‑FUNC‑011 (Must):**  
*Ubiquitous:* The diff algorithm shall be based on component ID and semantic fingerprint, not on JSON line order.

### FEAT‑03: Token Merging (`ucp merge-tokens`)

**REQ‑FUNC‑020 (Must):**  
*Event‑driven:* When the user runs `ucp merge-tokens --input <glob> --output <tokens.json>`, the system shall merge multiple DTCG token files (colors, spacing, typography) and flag token name conflicts where the same key has different values.

**REQ‑FUNC‑021 (Must):**  
*Unwanted behaviour:* If a conflict is detected, the system shall output a list of conflicting tokens and shall not produce a merged output unless `--force` is used; with `--force` the first value wins (configurable via `--strategy`).

### FEAT‑04: Drift Detection (`ucp verify`)

**REQ‑FUNC‑030 (Must):**  
*Event‑driven:* When the user runs `ucp verify --spec <canonical.json> --source-dir <path>`, the system shall re‑extract components from the source directory, compare each component’s props against the canonical spec, and produce a drift report listing any differences with a confidence score.

**REQ‑FUNC‑031 (Should):**  
*Optional feature:* Where `--sync` is passed, the system shall automatically update the canonical spec for props with a high‑confidence match and produce a change log.

### FEAT‑05: Local Spec Registry (`ucp registry`)

**REQ‑FUNC‑040 (Must):**  
*Event‑driven:* When the user runs `ucp registry store <spec.json>`, the system shall persist the spec and its metadata (source, timestamp) in a local database (e.g., SQLite or sled) and assign a version identifier.

**REQ‑FUNC‑041 (Should):**  
*Event‑driven:* When the user runs `ucp registry list` / `ucp registry show <id>`, the system shall display stored specs and their details.

**REQ‑FUNC‑042 (Could):**  
*Event‑driven:* `ucp registry pull <url>` shall fetch a remote spec and index it locally.

### FEAT‑06: Batch Export (`ucp export-all`)

**REQ‑FUNC‑050 (Must):**  
*Event‑driven:* When the user runs `ucp export-all --spec <spec.json> --output <dir>`, the system shall generate all supported export formats (A2UI catalog, AG‑UI events, DTCG design tokens if present, W3C spec, DESIGN.md, LLMs.txt, AI contract, and shadcn registry) in a single command.

### FEAT‑07: Watch Mode (`ucp watch`)

**REQ‑FUNC‑060 (Should):**  
*Event‑driven:* When the user runs `ucp watch --source-dir <path> --spec <spec>`, the system shall monitor the source directory for file changes and automatically re‑run extraction, merging, curation (if applicable), and export, outputting the results to the specified output directory.

### FEAT‑08: Incremental Merge & Weighted Merge (*extending existing merge*)

**REQ‑FUNC‑070 (Must):**  
*Event‑driven:* When the user runs `ucp merge` with an existing merged spec as a base and new input specs, the system shall add only new or updated components, preserving previously curated decisions (incremental merge).

**REQ‑FUNC‑071 (Should):**  
*Optional feature:* Where `--weights` are provided, the system shall resolve conflicts by deferring to the source with the higher assigned weight.

## 4. Quality & Non‑functional Requirements

All NFRs are measurable and traceable to ASRs.

| ID | Quality characteristic | Requirement | Fit criterion |
|----|------------------------|-------------|---------------|
| NFR‑PERF‑001 | Performance efficiency (latency) | Merging two specs of 100 components each, with up to 50 conflicts, shall complete in under 3 seconds on a reference developer machine (M1 Mac, 16 GB RAM). | Measured via automated benchmark; p95 ≤ 3 s. |
| NFR‑PERF‑002 | Performance efficiency (throughput) | `ucp diff` on two 500‑component specs shall finish within 1 second. | p95 ≤ 1 s. |
| NFR‑REL‑001 | Reliability | The merge operation shall be deterministic; given identical inputs, identical outputs must be produced. | Verified by unit and property tests. |
| NFR‑USAB‑001 | Usability | 80% of first‑time users (spec maintainers) shall complete a merge+curation of two specs with ≤10 conflicts within 10 minutes in a controlled test. | Usability test after release. |
| NFR‑SEC‑001 | Security (data privacy) | When using LLM assistance, the system shall only send component metadata and code snippets to the Ollama endpoint; it shall never send full source directories or proprietary data without user confirmation. | Code review + demo. |
| NFR‑MAIN‑001 | Maintainability | All new modules shall have unit test coverage ≥80% and integration test coverage for key paths. | Measured by cargo‑llvm‑cov. |

## 5. External Interfaces & Data Contracts

**File‑based interfaces:**
- Input spec files: JSON conforming to `SynthesisOutput` schema (backward‑compatible with v0.9 format; new fields for provenance are additive).
- Output spec files: same schema, with added `provenance` array and `curation_log`.
- Registry files: shadcn v4 registry‑item JSON and index JSON.
- Export formats: A2UI catalog, AG‑UI events, DTCG tokens, W3C UI spec, DESIGN.md, LLMs.txt, AI contract (all defined by existing exporters).
- Local spec store: SQLite schema (or file‑based key‑value store) with tables for specs, metadata, and version history.

**Command‑line interface:** All new subcommands follow the existing `clap`‑based structure; exit codes as per Unix conventions (0 for success, non‑zero for errors).

## 6. Constraints, Assumptions & Dependencies

- **Constraints:** Must compile with Rust stable 1.80+, reuse existing `ucp‑core` and `ucp‑synthesizer` crates without breaking changes. Must not introduce mandatory cloud dependencies.
- **Assumptions:** The TUI will be built with `ratatui` (or `inquire`). The local registry will use `sled` or `rusqlite`. The diff engine will be built on `similar` crate.
- **Dependencies:** Existing crate dependencies remain; new dependencies: `ratatui` (or `inquire`), `similar`, `sled` (or `rusqlite`), `notify` (for watch mode).

## 7. TBD Log

| TBD ID | Description | Owner | Due |
|--------|-------------|-------|-----|
| TBD‑001 | Final choice of TUI library (ratatui vs inquire) | UCP core team | Before SRS baseline |
| TBD‑002 | Local storage backend (sled vs SQLite) | UCP core team | Before prototype |
| TBD‑003 | Exact provenance data fields (minimum set) | UCP core team | After architecture design |

## 8. Requirements Attributes & Traceability Model

All functional requirements have: ID, priority (MoSCoW), status, source (SN‑xxx from BRS or
BG‑xxx from Vision), verification method, and trace links. The traceability matrix will be
maintained in the Test Verification Plan.

Example trace:  
`FEAT‑01::curate → REQ‑FUNC‑001 → (upwards) SN‑01, BG‑01; (downwards) TestCase‑Curate‑001`.

```

---

## 4. Architecture & Design Specification (`ucp-v011-architecture.md`)

```markdown
# Architecture & Design Specification for UCP v0.11

| Field | Value |
|-------|-------|
| Project | UCP (Universal Component Protocol) |
| Document | Architecture & Design Specification |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP team, assisted by AI |
| Status | Draft — Pending Review |

## 1. Context & Scope

This document describes the software architecture for the v0.11 spec‑maintainer tools. It
builds on the existing UCP workspace and extends `ucp‑cli` and `ucp‑synthesizer` with new
modules. It addresses the ASRs extracted from the SRS.

**Key design drivers (ASRs):**
- ASR‑001: Deterministic merging (NFR‑REL‑001)
- ASR‑002: Merge performance (NFR‑PERF‑001)
- ASR‑003: Interactive curation TUI (usability)
- ASR‑004: Local spec storage and retrieval
- ASR‑005: Backward compatibility with existing `SynthesisOutput` format
- ASR‑006: File‑watch integration for continuous operation

## 2. Goals & Non‑goals (Design Level)

**Goals:**
- Add a new `ucp‑maintainer` crate (or extend `ucp‑cli` directly) to host the new commands.
- Keep `ucp‑core` unchanged (backward‑compatible extensions acceptable).
- Leverage `ucp‑synthesizer`’s existing merge and extraction functions.
- Use lightweight Rust crates for TUI, diff, storage, and file watching.

**Non‑goals:**
- No major refactoring of the extraction pipeline.
- No web server or daemon.
- No distributed consensus or multi‑user locking.

## 3. Architecturally Significant Requirements (ASRs)

| ID | ASR | Source | Impact |
|----|-----|--------|--------|
| ASR‑001 | Deterministic merge results | NFR‑REL‑001 | Merge algorithm must be pure and testable |
| ASR‑002 | Fast merge (<3 s for 100 comps) | NFR‑PERF‑001 | Use efficient data structures, avoid deep recursion |
| ASR‑003 | TUI usability | NFR‑USAB‑001 | Design interactive flow with clear navigation |
| ASR‑004 | Local spec store with query | REQ‑FUNC‑040,041 | Choose embedded DB |
| ASR‑005 | Backward compatibility with spec format | Constraint | Extend `SynthesisOutput` with optional fields; provide migration |
| ASR‑006 | File watch & auto‑rebuild | REQ‑FUNC‑060 | Use `notify` crate; debounce and queue events |

## 4. The Design

### 4.1 System Overview

The architecture adds a new crate `ucp‑maintainer` (or modules within `ucp‑cli`) that
orchestrates the high‑level commands. It sits above `ucp‑synthesizer` as a consumer of its
APIs (merge, extraction, export, etc.).

**C4 Level 1 – System Context:** (text description)  
Users interact via CLI → `ucp` binary. File system stores specs, tokens, source code.
Optional Ollama service (HTTP) for LLM.

**C4 Level 2 – Container diagram:**  
Containers:
- `ucp‑cli` (or `ucp‑maintainer` binary): hosts new subcommands.
- `ucp‑synthesizer` library: merged, unified, extracted components.
- `ucp‑core` library: CAM models, error types.
- Local DB (e.g., SQLite file) for spec registry.
- File watcher (in‑process via `notify`).

### 4.2 Key Data Flows

1. **Merge & Curate flow:**  
   Load specs → call `merge_specs()` (from synthesizer) → produce merge result with conflicts → render TUI to resolve → store curation decisions → save curated spec.
2. **Drift detection:**  
   Load canonical spec + source dir → call extraction pipeline → compare using structural diff → produce report.
3. **Watch:**  
   `notify` on source dir → trigger rebuild pipeline → export → update output files.

### 4.3 Data Model Extensions

`SynthesisOutput` gains optional fields:
```rust
pub provenance: Vec<MergeRecord>,
pub curation_log: Vec<CurationDecision>,
```
`MergeRecord`: source spec path/repo, timestamp, fingerprint info.
`CurationDecision`: conflict ID, chosen option, user rationale, timestamp.

These are serialized with `serde` and conditionally skipped if empty to preserve backward compat.

### 4.4 Security Architecture

LLM integration: the `ucp curate` command checks for an `OLLAMA_URL`; if absent, LLM features are disabled. When enabled, only the component’s properties and code snippets (from the source map) are sent. No credentials or other secrets are transmitted.

### 4.5 Deployment View

The tools are deployed as part of the `ucp` binary, which is a single static Rust binary installable via `cargo install` or Homebrew. No runtime external services except optional Ollama. The local spec store lives in a user‑configurable directory (default `~/.ucp/registry.db`).

## 5. Architecture Decision Records (ADRs)

**ADR‑0001: Use `ratatui` (or `inquire`) for TUI**

| Field | Value |
|-------|-------|
| Status | Proposed |
| Context | Need an interactive terminal interface for curation. Rust ecosystems offer `ratatui` (framework) and `inquire` (widgets). |
| Decision Drivers | ASR‑003 (usability), familiarity, maintenance |
| Options | A) `ratatui` – full TUIs; B) `inquire` – simple prompts; C) custom raw terminal |
| Decision Outcome | Choose `ratatui` for rich form‑based interaction and extensibility. (If prototyping finds it heavy, fallback to `inquire`.) |
| Consequences | Positive: flexible layout. Negative: learning curve, visual consistency. |

**ADR‑0002: Local spec store – `sled` vs `rusqlite`**

| Field | Value |
|-------|-------|
| Status | Proposed |
| Context | Need to persist specs and query/search. |
| Decision Drivers | ASR‑004, portability, dependency weight |
| Options | A) `sled` (pure Rust key‑value); B) `rusqlite` (SQLite). |
| Decision Outcome | Choose `rusqlite` for structured queries and migration support. |
| Consequences | Positive: familiar SQL, indexing. Negative: requires SQLite system library. |

**ADR‑0003: Structural diff engine**

| Field | Value |
|-------|-------|
| Status | Proposed |
| Context | Spec diff must understand component structure, not raw JSON text. |
| Decision Drivers | ASR‑006, performance |
| Options | A) Tree‑sitter‑based; B) custom recursive comparison; C) `similar` crate (text). |
| Decision Outcome | Implement custom recursive diff based on CAM types, with text fallback for rendering. |
| Consequences | Positive: semantic diff; Negative: must maintain when CAM evolves. |

## 6. API & Interface Contracts

All new commands expose a CLI contract defined via `clap`. The internal programmatic API between
`ucp‑maintainer` and `ucp‑synthesizer` will be defined as public functions in the synthesizer
crate, offering:
- `merge_specs_with_options(specs, options) -> Result<MergeOutput>` (extended from current)
- `extract_and_unify(source_dir) -> Result<SynthesisOutput>`
- `diff_specs(a, b) -> DiffReport`
- `merge_tokens(tokens) -> Result<DtcgTokens>`
and others. These are not external REST APIs but Rust library calls.

## 7. Cross‑cutting Concerns

- **Observability:** Use structured logging with `tracing`. Emit events for merge conflicts, curation actions, and watch mode triggers.
- **Error handling:** Use `anyhow`/`thiserror` with user‑friendly messages.
- **Testing strategy:** detailed in Test Plan.

## 8. Alternatives Considered

- **Embedding a web server for TUI:** Rejected – adds complexity, not needed.
- **Re‑implementing merge from scratch:** Rejected – existing merge logic is solid and well‑tested.

## 9. Traceability

ASR → design sections → ADRs. C4 diagrams are described textually; a visual C4 model will be maintained separately in the repo.
```

---

## 5. Behavioral Specification & Test Verification Plan (`ucp-v011-test.md`)

```markdown
# Behavioral Specification & Test Verification Plan for UCP v0.11

| Field | Value |
|-------|-------|
| Project | UCP (Universal Component Protocol) |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP team, assisted by AI |
| Status | Draft — Pending Review |

## 1. Introduction

This document defines the acceptance criteria, test strategy, and traceability for the v0.11
spec maintainer tools. It uses Specification by Example (SbE) / BDD practices and follows the
standards referenced in the verification guide.

## 2. Behavioral Specifications (SbE/BDD)

### Feature: Interactive Merge Curation (`ucp curate`)

**Scenario: Simple conflict resolution (default acceptance)**
```gherkin
Given a merged spec "merged.json" with a single type conflict on component "Button" props "disabled"
  And the conflict has suggested resolution "IncludeMajority"
When the user runs "ucp curate --merged merged.json --output curated.json"
  And the user accepts the suggestion for all conflicts
Then the output file "curated.json" shall contain the component "Button" with prop "disabled" resolved as ControlFlag
  And a curation log with one entry for the conflict shall be appended to the spec.
```

**Scenario: Custom resolution override**
```gherkin
Given a conflict where the user chooses to override
When the user selects "Custom input" and types "treat as SpreadAttributes"
Then the output spec shall reflect that prop type.
```

### Feature: Spec Diff

**Scenario: Detects added prop**
```gherkin
Given specifications A and B where B has an extra prop "tooltip" on component "Card"
When the user runs "ucp diff --spec-a A.json --spec-b B.json --json"
Then the JSON output shall include a section "added_props" with "tooltip" for component "Card".
```

### Feature: Drift Detection

**Scenario: Source code changed, spec stale**
```gherkin
Given a canonical spec "canonical.json" for a component "Button" with prop "disabled: bool"
  And a source directory where "Button" now has "disabled: Option<bool>"
When the user runs "ucp verify --spec canonical.json --source-dir ./src"
Then the drift report shall list "Button.disabled" type changed from "bool" to "Option<bool>"
  And the confidence score shall be >0.8.
```

### Feature: Token Merge

**Scenario: Conflicting values**
```gherkin
Given two token files "a.json" and "b.json" both defining "--primary" with different values
When the user runs "ucp merge-tokens --input a.json b.json"
Then the output shall flag "--primary" as a conflict and not produce a merged file.
When the user runs with "--strategy first-wins --force"
Then "--primary" shall take the value from the first file.
```

(Further scenarios for registry, export‑all, watch mode, etc., would be elaborated in the ultimate living documentation.)

## 3. Test Strategy & Plan

**Test pyramid approach:**
- Many unit tests for merge, diff, curation logic (in `ucp‑synthesizer` and `ucp‑maintainer`).
- Integration tests for CLI commands (using `assert_cmd`).
- End‑to‑end tests for complete workflows (using existing `.just‑e2e` infrastructure).
- Exploratory testing for TUI usability (charters).

**Tools:** `cargo test`, `cargo nextest`, `insta` for snapshots, `proptest` for property‑based
testing of merge/diff determinism, `assert_cmd` for CLI tests.

**Risk‑based prioritization:** Merge and curation are the highest risk (core new value),
followed by diff and drift detection.

## 4. Test Case Specifications (Examples)

**TC‑Curate‑001: Deterministic merge**
- **Precondition:** Two spec files with known conflicts.
- **Steps:** Run `ucp merge` to create a merged file, then run `ucp curate` twice with the same inputs and decisions (via batch file).
- **Expected:** Both outputs are identical (deterministic).

**TC‑Diff‑001: Component added**
- **Steps:** Create two specs where B has an additional component, run `ucp diff`.
- **Expected:** Text output lists the new component.

## 5. NFR Verification Plans

**Performance (NFR‑PERF‑001):**
- **Method:** Benchmark test (criterion) that merges two 100‑component specs and measures elapsed time.
- **Acceptance:** p95 ≤ 3 s on reference hardware; benchmark integrated into CI.

**Reliability (NFR‑REL‑001):**
- **Method:** Property‑based tests with `proptest` generating random specs, merging, and checking idempotence/determinism.

**Usability (NFR‑USAB‑001):**
- **Method:** Post‑release user testing with 5–10 spec maintainers, recording task completion time and errors.

## 6. Requirements Traceability Matrix (RTM) — Extract

| Business Goal | Stakeholder Need | System Requirement | BDD Scenario / Example | Test Case | Verification Method |
|---------------|------------------|--------------------|------------------------|-----------|---------------------|
| BG‑01 | SN‑01 | REQ‑FUNC‑001 | Scenario: Simple conflict resolution | TC‑Curate‑001 | Test + Inspection |
| BG‑02 | SN‑03 | REQ‑FUNC‑030 | Scenario: Source code changed | TC‑Drift‑001 | Test |
| BG‑03 | SN‑04 | REQ‑FUNC‑050 | (export‑all) | TC‑Export‑001 | Test |
| BG‑01 | SN‑01 | NFR‑REL‑001 | deterministic merge | PT‑Merge‑Det | Test (proptest) |

## 7. Living Documentation Strategy

All Gherkin feature files and example tables will be stored in the repository under `tests/features/`
and executed as part of the CI suite (using `cucumber` or a custom runner). Reports will be
published as HTML artifacts. The test plan itself is a lightweight Markdown file in the repo,
updated alongside code changes.
