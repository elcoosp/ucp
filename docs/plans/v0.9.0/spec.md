# UCP v0.9.0 — Full Specification Suite

## Product Vision & Strategic Alignment – UCP v0.9.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.9.0‑vision‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Vision Statement

> **UCP becomes the definitive bridge between design systems and AI agents.**
> v0.9.0 makes any extracted component library immediately consumable by AI agents via a production MCP server, DESIGN.md 1.0‑compliant files, LLMs.txt, A2UI catalogs, and AG-UI schemas — while joining the W3C Generative UI standardization effort.

### 2. Elevator Pitch

> **For** AI‑application developers and design‑system maintainers
> **who need** their component libraries to be natively understood by AI coding assistants,
> **our product** is a CLI pipeline and MCP server that extracts, unifies, and exports UI component intelligence
> **that provides** DESIGN.md 1.0‑compliant files, LLMs.txt generation, a production‑grade MCP server serving all exports, and publishes to the MCP Registry.
> **Unlike** manual documentation or framework‑specific tools,
> **our solution** makes any design system AI‑readable and serves it through the same protocol that Claude, Cursor, and other AI tools already use.

### 3. Problem Statement

Google open‑sourced the DESIGN.md specification (April 22, 2026). MCP became enterprise infrastructure — JFrog launched a universal MCP Registry, Microsoft published MCP publishing guides, and 1,200 developers attended the MCP Dev Summit. LLMs.txt is emerging as the standard for AI‑coding‑assistant project context. Yet no tool bridges these three: extracting components from existing code, exposing them through MCP, and producing DESIGN.md + LLMs.txt. UCP is the only tool that can.

### 4. Business Objectives

| ID | Objective | Key Results |
|----|-----------|-------------|
| G‑01 | Become the standard DESIGN.md generator | DESIGN.md 1.0 export passes upstream validation |
| G‑02 | Ship a production MCP server | Server passes JFrog Registry validation; serves all UCP exports |
| G‑03 | Generate LLMs.txt from extracted libraries | LLMs.txt covers components, props, usage patterns |
| G‑04 | Join W3C and shape Generative UI standards | W3C Community Group membership confirmed |
| G‑05 | Maintain 100% test pass rate | All existing tests continue to pass |

### 5. Goals and Non‑Goals

**Goals (v0.9.0):**
- DESIGN.md 1.0 full compliance (bidirectional: read DESIGN.md → generate components).
- Production MCP server: serves A2UI catalogs, AG‑UI schemas, DESIGN.md, W3C specs, LLMs.txt.
- LLMs.txt generation from extracted libraries.
- MCP Registry publishing workflow.
- W3C Community Group membership.

**Non‑Goals:**
- No new framework extractors or code generators.
- No visual design tools or Figma integration.
- No real‑time collaboration features.

---

## Software Requirements Specification – UCP v0.9.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | SRS v0.9.0 |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Introduction

This SRS defines v0.9.0 capabilities: DESIGN.md 1.0 compliance, production MCP server, LLMs.txt generation, MCP Registry publishing, and W3C participation.

### 2. Functional Requirements

#### 2.1 DESIGN.md 1.0

**REQ‑DSN‑010 – Conform to Open‑Source DESIGN.md Specification**
> **The DESIGN.md exporter** shall produce output that conforms to the published DESIGN.md specification (open‑sourced April 2026).

*Acceptance:* Output passes any upstream DESIGN.md validator.

**REQ‑DSN‑011 – Bidirectional Support**
> **When** given a valid DESIGN.md file, **the system** shall parse it and produce a `PackageManifest` that can be used to generate components.

*Acceptance:* Round‑trip: DESIGN.md → CAM → generate Dioxus stubs.

#### 2.2 Production MCP Server

**REQ‑MCP‑020 – Full MCP Tool Registry**
> **The MCP server** shall expose tools for every export format: `export_a2ui`, `export_agui`, `export_design_md`, `export_w3c`, `export_dtcg`, `generate_llms_txt`, `list_components`, `get_component`.

*Acceptance:* All tools are discoverable via `tools/list` and callable via `tools/call`.

**REQ‑MCP‑021 – JFrog Registry Compatibility**
> **The MCP server** shall include a `server.json` manifest conforming to the JFrog MCP Registry specification.

**REQ‑MCP‑022 – Structured Component Data**
> **The `get_component` tool** shall return full component metadata: props (with types, defaults, required), events, variants, state machine, parts, and source attribution.

#### 2.3 LLMs.txt Generation

**REQ‑LLM‑001 – Generate LLMs.txt**
> **When** the user runs `ucp export --target llms-txt`, **the system** shall produce an `llms.txt` file describing all components, their props, events, and usage patterns.

*Acceptance:* Output conforms to the LLMs.txt standard.

**REQ‑LLM‑002 – Component‑Level Documentation**
> **The LLMs.txt file** shall include per‑component sections with prop tables, event descriptions, and usage examples derived from CAM metadata.

#### 2.4 W3C Participation

**REQ‑W3C‑030 – Join W3C Community Group**
> **The UCP project** shall join the W3C UI Specification Schema Community Group.

*Acceptance:* Membership confirmed via mailing list.

**REQ‑W3C‑031 – Submit Reference Implementation**
> **The UCP project** shall submit its W3C export format as a reference implementation for the group's consideration.

### 3. Quality Requirements

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑MNT‑001 | All 137+ tests pass | `just test` exits 0 |
| NFR‑MNT‑002 | Zero clippy warnings | `cargo clippy` exits 0 |
| NFR‑REL‑001 | MCP server passes JFrog validation | CI step |
| NFR‑REL‑002 | DESIGN.md output passes upstream validation | CI step |

### 4. Traceability

| Goal | Capability | SRS Requirements |
|------|------------|------------------|
| G‑01 | DESIGN.md 1.0 | REQ‑DSN‑010, REQ‑DSN‑011 |
| G‑02 | Production MCP | REQ‑MCP‑020, REQ‑MCP‑021, REQ‑MCP‑022 |
| G‑03 | LLMs.txt | REQ‑LLM‑001, REQ‑LLM‑002 |
| G‑04 | W3C | REQ‑W3C‑030, REQ‑W3C‑031 |

---

## Architecture & Design Specification – UCP v0.9.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Architecture & Design Specification |
| Version | 0.9.0‑arch‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Context and Scope

v0.9.0 is the "AI‑readiness" release. It upgrades the MCP server to production quality, adds LLMs.txt generation, achieves full DESIGN.md 1.0 compliance, and joins the W3C Generative UI effort. No new framework extractors or code generators are added.

### 2. Architecturally Significant Requirements

| ID | Requirement | Source |
|----|-------------|--------|
| ASR‑001 | DESIGN.md 1.0 + bidirectional | REQ‑DSN‑010, REQ‑DSN‑011 |
| ASR‑002 | Production MCP server | REQ‑MCP‑020, REQ‑MCP‑021, REQ‑MCP‑022 |
| ASR‑003 | LLMs.txt generation | REQ‑LLM‑001, REQ‑LLM‑002 |

### 3. System Design

#### 3.1 Enhanced: `export::design_md` → DESIGN.md 1.0

Update the YAML front matter structure to match the published specification. Add a `parse_design_md` function that reads DESIGN.md and extracts a `PackageManifest`.

#### 3.2 Enhanced: `contract::mcp_server` → Production MCP

Add a full tool registry. Each export format becomes a callable MCP tool. Add `server.json` manifest generation. Use proper JSON‑RPC error codes throughout.

#### 3.3 New Module: `export::llms_txt`

Generate an `llms.txt` file with structured documentation for all components in the spec.

#### 3.4 CLI: New export targets

- `llms-txt` — generate LLMs.txt
- `mcp-server-json` — generate JFrog‑compatible `server.json`
- DESIGN.md parsing via `ucp import --target design-md`

### 4. Key Design Decisions (ADRs)

| ADR | Decision | Drivers |
|-----|----------|---------|
| ADR‑033 | MCP server becomes the primary AI‑interface | ASR‑002 |
| ADR‑034 | DESIGN.md parser uses serde_yaml for YAML, regex for Markdown | ASR‑001 |
| ADR‑035 | LLMs.txt follows the standard Markdown template | ASR‑003 |

### 5. Traceability

| ASR | Design Elements | ADRs |
|-----|----------------|------|
| ASR‑001 | `export::design_md` bidirectional | ADR‑034 |
| ASR‑002 | `contract::mcp_server` tool registry | ADR‑033 |
| ASR‑003 | `export::llms_txt` | ADR‑035 |

---

## Behavioral Specification & Test Verification Plan – UCP v0.9.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Behavioral Spec & Test Verification |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Behavioral Specifications

```gherkin
Feature: DESIGN.md 1.0
  Scenario: Export matches open-source spec
    Given a SynthesisOutput with a Button component
    When `ucp export --target design-md` runs
    Then the YAML front matter matches the DESIGN.md 1.0 schema
    And the Markdown body includes props, events, variants

  Scenario: Import DESIGN.md and generate components
    Given a valid DESIGN.md file
    When `ucp import --target design-md` runs
    Then a PackageManifest is created
    And `ucp generate --target dioxus` produces valid stubs

Feature: Production MCP Server
  Scenario: MCP server lists all export tools
    Given the MCP server is started with a spec
    When `tools/list` is called
    Then the response includes `export_a2ui`, `export_agui`, `export_design_md`, `export_w3c`, `export_dtcg`, `generate_llms_txt`

  Scenario: MCP server serves component details
    Given the MCP server is started with a spec containing a Button component
    When `tools/call` with `get_component` and `{"name": "Button"}` is called
    Then the response includes props, events, variants, state machine

Feature: LLMs.txt Generation
  Scenario: Export LLMs.txt from a spec
    Given a SynthesisOutput with 3 components
    When `ucp export --target llms-txt` runs
    Then an `llms.txt` file is created
    And it contains per-component prop tables
    And usage patterns derived from CAM metadata
```

### 2. Test Strategy

| Layer | Goal | Tools |
|-------|------|-------|
| Unit tests | DESIGN.md parsing, MCP tool dispatch, LLMs.txt generation | Rust #[test] |
| Integration tests | Round‑trip: DESIGN.md → CAM → generate | tempfile, tokio |
| Manual verification | MCP server with MCP Inspector | Claude/Cursor MCP integration |
| Regression tests | All v0.8.0 tests continue to pass | cargo nextest |

### 3. Requirements Traceability Matrix

| Requirement | Test Case | Verification |
|-------------|-----------|--------------|
| REQ‑DSN‑010 | design-md-spec-validation | Schema validation |
| REQ‑DSN‑011 | design-md-import-roundtrip | Integration test |
| REQ‑MCP‑020 | mcp-tools-list-all-exports | Test |
| REQ‑MCP‑022 | mcp-get-component-details | Test |
| REQ‑LLM‑001 | llms-txt-export | Test |
| REQ‑W3C‑030 | w3c-membership | Manual confirmation |

---

This completes the specification suite for v0.9.0. Ready for implementation planning whenever you are.
