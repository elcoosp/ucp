I'll generate the full specification suite for UCP v0.13 “UCP for Agents” – focusing on the MCP server (P0) and Registry server (P1) as the primary deliverables, with the remaining features noted as deferred.

These documents are produced using the `spec-writer` skill methodology, with context from the existing codebase, the v0.11 implementation, and the ecosystem research just completed.

---

## 1. Product Vision & Strategic Alignment (`ucp-v013-vision.md`)

```markdown
# UCP v0.13 – Product Vision & Strategic Alignment

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
Make UCP’s component intelligence directly accessible to AI coding assistants and
self‑hosted registries – so that every developer, human or machine, always knows
exactly which components exist, what they expose, and whether they’re being used
correctly.

**Elevator Pitch (Moore’s template):**  
For **AI‑powered development teams and design‑system maintainers** who need to
give agents and tools programmatic access to component specifications, UCP v0.13
is a **component intelligence service** that **exposes a live MCP server (for AI
coding assistants) and a self‑hosted shadcn‑compatible registry (for automated
distribution) – unlike today’s purely CLI‑based workflow, UCP v0.13 closes the
loop from extraction to AI‑assisted consumption.**

## 2. Problem Statement & Business Context

Modern AI coding assistants (Copilot, Cursor, Claude Code) and design‑system
tooling are converging on two interfaces: the Model Context Protocol (MCP) for
AI agent interaction, and federated component registries (shadcn v4) for
distribution. UCP already extracts, curates, and stores rich component
specifications, but today those specs are only accessible via CLI commands or
static JSON files. This means:

- AI agents can’t query UCP for component availability or type constraints.
- Teams must manually publish shadcn registries by copying generated JSON files.
- There is no way to enforce spec‑correct usage across a growing codebase.
- UCP’s curation and provenance data are invisible to downstream tooling.

By adding an MCP server and a registry server, UCP becomes the **intelligence
layer** that feeds AI agents and component registries with verified, curated,
cross‑framework component metadata.

## 3. Target Users / Customers

- **AI‑assisted developers** (primary for MCP): want their coding agent to know
  which components are available and how to use them correctly.
- **Design‑system maintainers** (primary for registry): want to self‑host a
  registry that stays in sync with their canonical specs.
- **Platform engineers**: want to integrate UCP into CI/CD and AI‑agent workflows.
- **Open‑source library authors**: want to publish their component specs as
  self‑hosted registries for community consumption.

**Explicitly NOT targeting (non‑goals for v0.13):**
- A public, cloud‑hosted registry service (that’s a separate product decision).
- Full semantic component search across registries (v0.13 is per‑server only).
- Spec‑guided code generation for AI agents (P3 – deferred).

## 4. User Needs & Value Proposition

**Top 3 needs:**
1. “I want my AI coding assistant to automatically know what components are
   available and what props they take, without me having to paste specs every time.”
2. “I want to host a shadcn‑compatible registry from my UCP specs, so other
   developers can add my components with a single command.”
3. “I want to know when my codebase is out of sync with the component spec,
   ideally as part of my code review process.”

**Key differentiator:**  
UCP’s specs are already curated, merged, and provenance‑tracked. v0.13 connects
this existing intelligence to the industry‑standard interfaces for AI agents and
component registries – without requiring any external infrastructure.

## 5. Desired Outcomes & Success Metrics

| Outcome | Key Results |
|---------|-------------|
| O‑1: AI agent accessibility | Within 30 days of release, at least 2 external developers report using UCP’s MCP server with Claude Code or Cursor to query component specs |
| O‑2: Self‑hosted registry adoption | At least 1 open‑source component library publishes a shadcn v4 registry using `ucp registry serve` within 60 days |
| O‑3: Integration simplicity | A developer new to UCP can start the MCP server with a spec file in under 2 minutes |
| O‑4: Spec correctness in code | `ucp lint` catches prop‑type mismatches in source code with <5% false positive rate on a reference codebase |

## 6. Strategic Constraints

- Must integrate with the existing UCP workspace (Rust, just, cargo nextest).
- Must run as local, self‑hosted services (no mandatory cloud dependency).
- Must be backward‑compatible with existing `SynthesisOutput` format.
- MCP server must implement the MCP specification (stdio transport, tools/capabilities).
- Registry server must be compatible with the shadcn v4 registry resolution protocol.
- All features must ship with unit, integration, property, and doc tests.

## 7. Goals and Non‑goals

**Goals (v0.13 scope):**
- Provide `ucp mcp` – a stdio‑based MCP server exposing UCP spec tools.
- Provide `ucp registry serve` – an HTTP server exposing a shadcn v4 registry.
- Add E2E tests for both servers.
- Update user and maintainer documentation for the new features.

**Non‑goals (explicitly out of scope for v0.13):**
- OAuth / authentication for the registry server (basic bearer token at most).
- Semantic component search across namespaces.
- MCP resources or prompts (tools only for v0.13).
- Cloud hosting, SaaS, or managed registry service.
- Spec‑guided AI review rules (P3 – deferred to v0.14).
- CEM export (P4 – deferred).

## 8. Stakeholders, Sponsorship, and Governance

- **Executive Sponsor:** UCP project lead (elcoosp).
- **Product Lead:** UCP core team.
- **Document owner:** UCP core team; reviewed quarterly.

## 9. Traceability & Alignment Notes

All goals (O‑1..O‑4) map to epics. Traceability scheme:
- Goal IDs: O‑1 .. O‑4
- Stakeholder needs: SN‑01..SN‑04
- System requirements (SRS): REQ‑FUNC‑xxx, NFR‑xxx
```

---

## 2. Business & Stakeholder Requirements Specification (`ucp-v013-brs.md`)

```markdown
# Business & Stakeholder Requirements Specification for UCP v0.13

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
Extend UCP from a CLI‑based spec toolkit into a **component intelligence
service** that AI coding assistants and automated tooling can query in real time,
and that design‑system maintainers can use to self‑host shadcn‑compatible
component registries.

**Business problem:**  
UCP currently produces rich, curated component specifications (specs, provenance,
token sets), but they are only accessible via static files or CLI commands.
AI coding assistants and CI pipelines need programmatic access to this
information – ideally through industry‑standard interfaces (MCP, shadcn registry
protocol) – to use components correctly and keep dependencies up to date.

**Scope boundaries:**  
In‑scope: MCP server with tools for component listing, search, consistency
checking, and provenance retrieval; registry HTTP server for shadcn v4‑style
component distribution.  
Out‑of‑scope: cloud registry service, OAuth integration, semantic search across
namespaces, spec‑guided AI review rules.

## 2. Business Goals, Objectives & Success Metrics

See Vision §5 for detailed OKRs. Referenced here as BG‑01..BG‑04.

## 3. Business Model & Processes

**High‑level value streams:**
1. **AI Agent Integration:** Maintainer runs `ucp mcp` locally → AI coding
   assistant discovers the MCP server → assistant queries UCP for component
   availability, prop types, and consistency during coding → developer receives
   real‑time guidance.
2. **Self‑Hosted Registry:** Maintainer runs `ucp registry serve` from curated
   spec → consumers run `shadcn add https://internal.registry/r/button.json` →
   component is installed with correct types and dependencies.

**Key domain events:**  
`MCP Server Started`, `Tool Query Received`, `Registry Requested`,
`Component Resolved`, `Registry Index Served`.

## 4. Business Rules & Policies

| ID | Rule | Source |
|----|------|--------|
| BR‑001 | All MCP tool responses must include the UCP spec version and provenance information for the component being queried. | Standards alignment |
| BR‑002 | The registry server must respond to `GET /r/{name}.json` with a valid shadcn v4 registry item or 404. | shadcn v4 protocol |
| BR‑003 | The registry server must serve a `registry.json` index of all available items. | shadcn v4 protocol |
| BR‑004 | Both servers must be startable with a single command and a spec file path. | Usability requirement |

## 5. Stakeholders & User Classes

| Stakeholder | Role | Influence | Key needs |
|-------------|------|-----------|------------|
| AI‑assisted developer | Primary user of MCP | High | Real‑time component intelligence in coding agent |
| Design‑system maintainer | Primary user of registry | High | Self‑hosted distribution, zero‑ops deployment |
| Platform engineer | Secondary user | Medium | CI integration of servers, MCP endpoint for pipelines |

**User classes:**
- **AI Developer (primary for MCP):** wants `ucp mcp` running locally, connected
  to their coding agent.
- **Registry Operator (primary for registry):** wants `ucp registry serve` to
  publish a spec as a registry.
- **CI Runner (secondary):** wants to start both servers in CI for automated
  consistency checks and artifact distribution.

**Key persona – “Sam the AI Pair‑Programmer”:**  
Sam uses Claude Code daily. He’s maintaining a cross‑framework component library
and wants his AI agent to know exactly what components are available and whether
the code he’s writing matches the spec. He runs `ucp mcp` in the background and
connects Claude Code to it.

## 6. Glossary / Ubiquitous Language

(Extends v0.11 glossary with new terms)

| Term | Definition |
|------|------------|
| MCP | Model Context Protocol – an open standard for AI tools to interact with external services |
| MCP server | A process that implements the MCP specification, exposing `tools/capabilities` |
| Registry server | An HTTP server that implements the shadcn v4 registry resolution protocol (`GET /r/{name}.json`, `GET /registry.json`) |
| Tool (MCP) | A named, parameterized operation exposed by an MCP server (e.g., `list_components`) |

## 7. Conceptual Domain Model

New entities:
- `MCPServer`: configuration (spec path, transport), active tools.
- `RegistryServer`: configuration (spec path, port, auth token), served registry items.
- `ToolDefinition`: name, description, input schema, handler function.

Relationships: An `MCPServer` is instantiated from a `SynthesisOutput` spec and offers
a set of `ToolDefinition`s. A `RegistryServer` is instantiated from a `SynthesisOutput`
spec and serves `RegistryItem`s generated from its components.

## 8. Stakeholder Needs & User Requirements

**As an AI‑assisted developer** (SN‑01):
- I want to start an MCP server from a UCP spec so my AI coding agent can list
  available components and their props.
- I want my agent to be able to check whether a component usage matches the spec.

**As a design‑system maintainer** (SN‑02):
- I want to serve my curated spec as a shadcn‑compatible registry so consumers
  can add my components with `shadcn add`.

**As a platform engineer** (SN‑03):
- I want to run the MCP server and registry server in CI to provide programmatic
  access to the latest specs.

## 9. System‑in‑Context & Operational Concept

UCP v0.13 adds two new executable modes to the existing `ucp` binary:
- `ucp mcp --spec <path>` starts a stdio MCP server.
- `ucp registry serve --spec <path> [--port <n>]` starts an HTTP registry server.

Both read a `SynthesisOutput` JSON file at startup and expose its contents.
The MCP server uses the MCP specification (stdio transport, JSON‑RPC messages).
The registry server uses HTTP with the shadcn v4 registry URL template pattern.

## 10. Stakeholder‑Level Constraints & Quality Expectations

- **Startup time:** Both servers must start and respond to their first request
  within 1 second on reference hardware.
- **Compatibility:** MCP server must be compatible with Claude Desktop, Cursor,
  and any other MCP‑compliant client. Registry server must be compatible with
  shadcn CLI v4.
- **Security:** No authentication by default; optional bearer token for registry.

## 11. Risks, Assumptions & Open Issues

- Assumption: The MCP Rust ecosystem (e.g., `mcp-rs` or a manual implementation)
  is mature enough for v0.13.
- Assumption: shadcn v4 registry resolution protocol is stable and well‑documented.
- Risk: MCP specification may evolve; mitigation: implement against a specific
  version and document compatibility.

## 12. Traceability Mapping to Vision

| Business Goal | Stakeholder Needs | Features (Epics) |
|---------------|------------------|------------------|
| BG‑01 (AI accessibility) | SN‑01 | `ucp mcp` |
| BG‑02 (registry adoption) | SN‑02 | `ucp registry serve` |
| BG‑03 (integration simplicity) | SN‑01, SN‑03 | CLI UX, documentation |
| BG‑04 (spec correctness) | SN‑03 | deferred to v0.14 |
```

---

## 3. Software Requirements Specification (`ucp-v013-srs.md`)

```markdown
# Software Requirements Specification for UCP v0.13

| Field | Value |
|-------|-------|
| Project | UCP (Universal Component Protocol) |
| Document | Software Requirements Specification (SRS) |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP team, assisted by AI |
| Status | Draft — Pending Review |

## 1. Introduction & Scope

This SRS specifies the software requirements for v0.13 “UCP for Agents.” It
builds on the stakeholder needs captured in the BRS and the Vision’s goals. v0.13
adds two new capabilities to the UCP CLI: an MCP server and a registry HTTP
server. All existing v0.11 functionality is preserved.

**References:** UCP Vision v0.13, UCP BRS v0.13, UCP codebase v0.12.0, MCP
specification (modelcontextprotocol.io), shadcn v4 registry protocol.

## 2. System Context & Overview

The UCP v0.13 CLI gains two new subcommands that run as long‑lived servers:

- **MCP Server** (`ucp mcp`): exposes UCP spec tools via stdio to AI coding
  assistants (Claude Desktop, Cursor, etc.) using the Model Context Protocol.
- **Registry Server** (`ucp registry serve`): exposes an HTTP endpoint serving
  shadcn v4‑compatible registry JSON for component consumption.

Both servers read a `SynthesisOutput` spec file at startup and present its
contents through their respective protocols.

## 3. Functional Capabilities & Behavior

### FEAT‑01: MCP Server (`ucp mcp`)

**REQ‑FUNC‑001 (Must):**  
*Event‑driven:* When the user invokes `ucp mcp --spec <path>`, the system shall
start an MCP server over stdio, register the tools defined in REQ‑FUNC‑002
through REQ‑FUNC‑005, and remain running until the client closes the connection.

**REQ‑FUNC‑002 (Must):**  
*Ubiquitous:* The system shall provide a `list_components` tool that returns all
component names and IDs from the loaded spec.

**REQ‑FUNC‑003 (Must):**  
*Event‑driven:* The system shall provide a `get_component` tool that, given a
component name or ID, returns the component’s full metadata (props with types,
events, state machine, parts, source attribution, and provenance).

**REQ‑FUNC‑004 (Should):**  
*Event‑driven:* The system shall provide a `check_consistency` tool that, given a
code snippet, checks whether it uses a known component with correct prop types,
and returns a list of mismatches or a confirmation.

**REQ‑FUNC‑005 (May):**  
*Event‑driven:* The system shall provide a `get_tokens` tool that returns the
design tokens (colors, spacing, typography) if present in the spec.

**REQ‑FUNC‑006 (Must):**  
*Unwanted behaviour:* If the spec file cannot be loaded, the server shall exit
with a clear error message before accepting any MCP requests.

### FEAT‑02: Registry Server (`ucp registry serve`)

**REQ‑FUNC‑010 (Must):**  
*Event‑driven:* When the user invokes `ucp registry serve --spec <path>`, the
system shall start an HTTP server on the specified port (default 3000) and serve
a shadcn v4‑compatible component registry.

**REQ‑FUNC‑011 (Must):**  
*Ubiquitous:* The system shall respond to `GET /registry.json` with a valid
shadcn v4 registry index containing all components from the spec, including
schema, name, homepage, and items array.

**REQ‑FUNC‑012 (Must):**  
*Event‑driven:* The system shall respond to `GET /r/{name}.json` with the
corresponding registry item JSON for the named component, or a 404 if not found.

**REQ‑FUNC‑013 (May):**  
*Optional feature:* Where a `--token <TOKEN>` flag is provided, the system shall
require a matching `Authorization: Bearer <TOKEN>` header on all requests, and
return 401 if missing or invalid.

**REQ‑FUNC‑014 (Should):**  
*Ubiquitous:* The system shall log each request (method, path, status, latency)
to stdout in a structured format.

## 4. Quality & Non‑functional Requirements

| ID | Quality characteristic | Requirement | Fit criterion |
|----|------------------------|-------------|---------------|
| NFR‑PERF‑001 | Startup time | MCP server must be ready to accept requests within 500 ms of invocation. | Measured on reference hardware. |
| NFR‑PERF‑002 | Response time | Registry server must respond to `GET /registry.json` within 50 ms for a spec with up to 100 components. | p95 ≤ 50 ms. |
| NFR‑REL‑001 | Compatibility | MCP server must be compatible with Claude Desktop, Cursor, and any client implementing MCP spec v2024‑11‑05. | Verified via integration tests with a reference MCP client. |
| NFR‑REL‑002 | Compatibility | Registry server must be compatible with shadcn CLI v4. | Verified via integration test: `shadcn add http://localhost:3000/r/button.json` succeeds. |
| NFR‑SEC‑001 | Security | Registry server must not serve files outside the spec’s generated registry directory. | Verified by path traversal tests. |
| NFR‑USAB‑001 | Usability | A developer can start the MCP server and receive a valid `tools/list` response in under 2 minutes from first invocation. | Timed usability test. |

## 5. External Interfaces & Data Contracts

### MCP Server Interface

- **Transport:** stdio (JSON‑RPC 2.0)
- **Protocol:** MCP specification v2024‑11‑05
- **Capabilities:** tools only (no resources, prompts, or sampling for v0.13)
- **Tool definitions:**
  - `list_components`: no required parameters
  - `get_component`: requires `name` or `id` parameter
  - `check_consistency`: requires `code` and `component` parameters
  - `get_tokens`: no required parameters

### Registry Server Interface

- **Transport:** HTTP/1.1
- **Protocol:** shadcn v4 registry resolution
- **Endpoints:**
  - `GET /registry.json` → `{ "$schema": "...", "name": "...", "items": [...] }`
  - `GET /r/{name}.json` → `{ "$schema": "...", "name": "...", "type": "registry:ui", "files": [...], ... }`
- **Authentication:** optional bearer token

## 6. Constraints, Assumptions & Dependencies

- **Constraints:** Must compile with Rust stable 1.80+. Must reuse existing
  registry generation code from `ucp-synthesizer`. Must not require external
  runtime services (databases, message queues).
- **Assumptions:** MCP Rust ecosystem is usable. shadcn v4 registry protocol
  is stable enough for integration testing.
- **Dependencies:** New crate dependencies likely include `tokio` (already
  present), `hyper` or `actix-web` or `axum` for HTTP, potentially `mcp-rs` or a
  manual MCP implementation.

## 7. TBD Log

| TBD ID | Description | Owner | Due |
|--------|-------------|-------|-----|
| TBD‑001 | Choice of HTTP framework for registry server (axum vs hyper) | UCP core team | Before implementation |
| TBD‑002 | MCP implementation approach (use `mcp-rs` crate vs manual JSON‑RPC over stdio) | UCP core team | Before implementation |
| TBD‑003 | Exact schema for `get_component` tool response | UCP core team | During implementation |
```

---

## 4. Architecture & Design Specification (`ucp-v013-architecture.md`)

```markdown
# Architecture & Design Specification for UCP v0.13

| Field | Value |
|-------|-------|
| Project | UCP (Universal Component Protocol) |
| Document | Architecture & Design Specification |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP team, assisted by AI |
| Status | Draft — Pending Review |

## 1. Context & Scope

This document describes the software architecture for the v0.13 “UCP for Agents”
release. It adds two new server modules to `ucp‑maintainer` and exposes them as
CLI subcommands in `ucp‑cli`.

**Key design drivers (ASRs):**
- ASR‑001: MCP protocol compliance (NFR‑REL‑001)
- ASR‑002: Fast startup (NFR‑PERF‑001)
- ASR‑003: shadcn v4 registry compatibility (NFR‑REL‑002)
- ASR‑004: Backward compatibility with `SynthesisOutput` and existing code
- ASR‑005: Reuse of existing registry generation logic

## 2. Goals & Non‑goals

**Goals:**
- Add `ucp‑maintainer::mcp` module implementing MCP server over stdio.
- Add `ucp‑maintainer::registry_server` module implementing HTTP registry server.
- Expose both as `ucp mcp` and `ucp registry serve` subcommands.
- Reuse existing `SynthesisOutput` loading and `generate_registry` logic.

**Non‑goals:**
- MCP resources, prompts, or sampling (tools only for v0.13).
- Authentication beyond optional bearer token for registry.
- Separate binary or daemon process (both servers are CLI subcommands).

## 3. Architecturally Significant Requirements

| ID | ASR | Source | Impact |
|----|-----|--------|--------|
| ASR‑001 | MCP protocol compliance | NFR‑REL‑001 | Must implement MCP spec v2024‑11‑05 over stdio |
| ASR‑002 | Fast startup < 500 ms | NFR‑PERF‑001 | Load spec at startup, minimal init |
| ASR‑003 | shadcn v4 compatibility | NFR‑REL‑002 | Must implement URL template `GET /r/{name}.json` |
| ASR‑004 | Backward compatibility | Constraint | No changes to `SynthesisOutput` or core types |
| ASR‑005 | Reuse registry generation | Constraint | Call `generate_registry` for each component on demand |

## 4. The Design

### 4.1 System Overview

Two new modules are added to `ucp‑maintainer`:

- **`mcp.rs`**: Reads a `SynthesisOutput` at construction, implements the MCP
  protocol (JSON‑RPC over stdio), and dispatches tool calls to functions that
  query the spec.
- **`registry_server.rs`**: Reads a `SynthesisOutput` at construction, starts
  an HTTP server (axum), and serves routes for `GET /registry.json` and
  `GET /r/{name}.json`. Each registry item is generated on‑the‑fly using the
  existing `generate_registry` and `generate_component_code_for_registry`
  functions from `ucp‑synthesizer`.

### 4.2 Key Data Flows

**MCP tool call:**
1. MCP client sends `tools/call` JSON‑RPC message.
2. `mcp.rs` parses the method name and parameters.
3. Dispatches to handler (e.g., `handle_list_components`, `handle_get_component`).
4. Reads data from the loaded `SynthesisOutput`.
5. Constructs JSON‑RPC response and writes to stdout.

**Registry request:**
1. HTTP client sends `GET /r/button.json`.
2. `registry_server.rs` parses the component name from the URL.
3. Looks up the component in the loaded `SynthesisOutput`.
4. Calls `generate_registry_item` (a new function or existing logic) to produce
   a shadcn v4‑compatible `RegistryItem` JSON.
5. Returns JSON response or 404.

### 4.3 Data Model

No changes to `SynthesisOutput`. New types:

- `McpServer`: holds loaded spec, tool registry, IO streams.
- `ToolDefinition`: name, description, input schema (JSON Schema), handler.
- `RegistryServerConfig`: spec path, port, optional auth token.

### 4.4 Security

Registry server serves only content derived from the spec. No file system access
beyond the spec file. No arbitrary file serving. If a token is configured,
reject unauthenticated requests with 401.

## 5. ADRs

**ADR‑0006: Use JSON‑RPC over stdio for MCP implementation**

| Field | Value |
|-------|-------|
| Status | Proposed |
| Context | Need to implement MCP spec. Options: use external crate `mcp-rs`, or manually implement JSON‑RPC over stdio. |
| Decision | Manually implement JSON‑RPC over stdio using `serde_json` and async line reading/writing. |
| Consequences | Positive: no external dependency risk, full control. Negative: more code to write and maintain. |

**ADR‑0007: Use `axum` for registry server**

| Field | Value |
|-------|-------|
| Status | Proposed |
| Context | Need a lightweight HTTP server for registry. |
| Decision | Use `axum` (already in dependency tree via `tokio`) for its simplicity and ecosystem fit. |
| Consequences | Positive: async, well‑maintained, minimal boilerplate. Negative: adds `axum` dependency. |

## 6. Traceability

ASR → design sections → ADRs. Both servers trace to their respective SRS
requirements and BRS stakeholder needs.
```

---

## 5. Behavioral Specification & Test Verification Plan (`ucp-v013-test.md`)

```markdown
# Behavioral Specification & Test Verification Plan for UCP v0.13

| Field | Value |
|-------|-------|
| Project | UCP (Universal Component Protocol) |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP team, assisted by AI |
| Status | Draft — Pending Review |

## 1. Introduction

This document defines the acceptance criteria, test strategy, and traceability
for the v0.13 MCP server and registry server features.

## 2. Behavioral Specifications (SbE/BDD)

### Feature: MCP Server

**Scenario: List components empty spec**
```gherkin
Given an empty spec file "empty.json"
When the MCP server is started with "--spec empty.json"
And the client sends a "tools/call" for "list_components"
Then the response shall contain an empty "components" array and "count": 0
```

**Scenario: Get component by name**
```gherkin
Given a spec with a component "Button"
When the client sends "tools/call" for "get_component" with name "Button"
Then the response shall contain the component's props, events, and source attribution
```

**Scenario: Get component not found**
```gherkin
Given a spec with no component named "Missing"
When the client sends "tools/call" for "get_component" with name "Missing"
Then the response shall contain an error with message "Not found"
```

**Scenario: Invalid spec file**
```gherkin
Given a nonexistent file "nonexistent.json"
When the user tries to start the MCP server with "--spec nonexistent.json"
Then the server shall exit with a non‑zero exit code and an error message on stderr
```

### Feature: Registry Server

**Scenario: Serve registry index**
```gherkin
Given a spec with two components "Button" and "Dialog"
When the user starts the registry server on port 3000
And the client requests "GET /registry.json"
Then the response shall be a JSON object with an "items" array of length 2
And each item shall have a "$schema" field
```

**Scenario: Serve component by name**
```gherkin
Given a spec with component "Button"
When the client requests "GET /r/button.json"
Then the response shall be a JSON object with "name": "button" and "type": "registry:ui"
And the response shall include a "files" array with the component's source code
```

**Scenario: Component not found**
```gherkin
Given a spec with no component "Missing"
When the client requests "GET /r/missing.json"
Then the response shall have status 404
```

**Scenario: Authenticated registry access**
```gherkin
Given the registry server is started with "--token secret123"
When the client requests "GET /registry.json" without an Authorization header
Then the response shall have status 401

When the client requests "GET /registry.json" with "Authorization: Bearer secret123"
Then the response shall have status 200
```

## 3. Test Strategy & Plan

- **Unit tests:** For MCP message parsing, tool dispatch, spec query functions.
- **Integration tests:** For full MCP JSON‑RPC exchange (start server, send message, verify response). For registry HTTP request/response cycle.
- **E2E tests:** For CLI invocation (`ucp mcp`, `ucp registry serve`) with real spec files.
- **Compatibility tests:** Against a real MCP client (e.g., Claude Desktop) and real shadcn CLI.
- **Snapshot tests:** For generated registry JSON output.

## 4. Requirements Traceability Matrix (Extract)

| Goal | Need | Requirement | BDD Scenario | Test Case |
|------|------|-------------|--------------|-----------|
| BG‑01 | SN‑01 | REQ‑FUNC‑001 | Start MCP server | TC‑MCP‑001 |
| BG‑01 | SN‑01 | REQ‑FUNC‑002 | List components | TC‑MCP‑002 |
| BG‑01 | SN‑01 | REQ‑FUNC‑003 | Get component | TC‑MCP‑003 |
| BG‑02 | SN‑02 | REQ‑FUNC‑010 | Start registry | TC‑REG‑001 |
| BG‑02 | SN‑02 | REQ‑FUNC‑011 | Registry index | TC‑REG‑002 |
| BG‑02 | SN‑02 | REQ‑FUNC‑012 | Component by name | TC‑REG‑003 |
| BG‑03 | SN‑03 | NFR‑PERF‑001 | Fast startup | PERF‑MCP‑001 |
