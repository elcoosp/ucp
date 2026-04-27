# UCP v0.14 Specification Suite — "UCP Lint"

---

## 1. Product Vision & Strategic Alignment (`ucp-v014-vision.md`)

```markdown
# UCP v0.14 – Product Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | UCP (Universal Component Protocol) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.1 (Draft) |
| Date | 2025-07-13 |
| Author | UCP product team |
| Status | Draft — Pending Review |

## 1. Vision & Elevator Pitch

**Vision:**  
Make UCP's component specifications enforceable — so that every prop mismatch,
every missing required prop, and every unknown attribute is caught before code
reaches production, whether by a human reviewer or an automated CI pipeline.

**Elevator Pitch:**  
For **design-system maintainers and platform engineers** who need to prevent
spec drift across growing codebases, UCP v0.14 is a **spec compliance linter**
that **scans source files, compares component usage against the canonical
spec, and reports violations with actionable fix suggestions — unlike generic
TypeScript or Rust linters that only check types, UCP understands component
semantics: required vs optional props, allowed enum values, deprecated APIs,
and cross-framework prop conventions.**

## 2. Problem Statement

UCP already extracts rich, curated component specifications. But specs are
only useful if they're enforced. Today, there is no way to answer:

- "Is my codebase using Button correctly according to the spec?"
- "Which files would break if I change Dialog's props?"
- "Are developers using deprecated component APIs?"

This means spec drift goes undetected. Components accumulate mismatches over
sprints. By the time anyone notices, fixing it requires touching dozens of files.
AI coding assistants make this worse — they generate code based on stale
descriptions, not the canonical spec.

By adding a linter, UCP becomes a **quality gate**: specs aren't just
documentation, they're enforceable contracts.

## 3. Target Users

| User | Primary Use Case |
|------|-----------------|
| Design-system maintainer | Run `ucp lint` before every release to catch drift |
| Platform engineer | Add `ucp lint` to CI pipeline as a quality gate |
| AI-assisted developer | See lint warnings in their editor from `ucp lint` |
| Open-source library author | Ship `ucp lint` as a recommended CI check |

**Explicitly NOT targeting:** Real-time IDE linting (that's a language server, v0.15+), semantic component search (out of scope), auto-fix beyond simple defaults (v0.15+).

## 4. Desired Outcomes

| Outcome | Key Result |
|---------|------------|
| O-1: CI Adoption | At least 1 team adds `ucp lint` to CI within 30 days of release |
| O-2: Drift Detection | `ucp lint` catches ≥95% of intentional prop mismatches on a test corpus |
| O-3: False Positive Rate | <5% false positive rate on a reference codebase with 50+ component usages |
| O-4: Startup Time | `ucp lint` scans 1000 files in <5 seconds |
| O-5: SARIF Integration | `ucp lint --format sarif` produces valid SARIF for GitHub Code Scanning |

## 5. Goals and Non-goals

**Goals:**
- Provide `ucp lint --spec <path> --source <dir>` with 5 core lint rules
- Support JSX (React), RSX (Dioxus), and Vue SFC extraction
- Output: human-readable (terminal), JSON (for CI), SARIF (for GitHub)
- Exit code 0 = clean, 1 = violations, 2 = error

**Non-goals:**
- Real-time IDE integration (LSP, v0.15)
- Auto-fix beyond adding missing optional props with defaults
- Custom rule configuration (v0.15)
- HTML/Web Components support (v0.15)
- Import resolution across packages (v0.14 handles single-project imports)

## 6. Strategic Constraints

- Must integrate with existing `SynthesisOutput` format without changes.
- Must not require the source code to compile (it's a linter, not a compiler).
- Must work on macOS, Linux, and in CI (GitHub Actions, GitLab CI).
- Must produce SARIF v2.1.0 for GitHub Code Scanning integration.
- Rules must be deterministic: same input → same output every time.
```

---

## 2. Business & Stakeholder Requirements (`ucp-v014-brs.md`)

```markdown
# Business & Stakeholder Requirements for UCP v0.14

| Field | Value |
|-------|-------|
| Project | UCP |
| Document | Business & Stakeholder Requirements Specification |
| Version | 0.1 (Draft) |
| Date | 2025-07-13 |
| Status | Draft — Pending Review |

## 1. Business Context

UCP extracts component specifications but provides no mechanism to verify that
source code conforms to those specs. This is a critical gap: specs without
enforcement are suggestions, not contracts. v0.14 closes this gap with a
dedicated linting tool.

## 2. Business Goals

| ID | Goal | Success Metric |
|----|------|---------------|
| BG-01 | CI Integration | ≥1 team ships `ucp lint` in CI within 30 days |
| BG-02 | Spec Accuracy | ≥95% intentional drift detection on reference corpus |
| BG-03 | Developer Experience | Developer can run `ucp lint` for first time and get actionable output in under 60 seconds |
| BG-04 | Ecosystem Fit | Valid SARIF output triggers GitHub Code Scanning annotations |

## 3. Value Streams

**CI/CD Quality Gate:**
1. Maintainer runs `ucp bootstrap` → spec generated
2. Spec is committed and tagged
3. `ucp lint --spec spec.json --source ./src` runs in CI
4. Violations block merge or produce warnings
5. Spec drift is caught at PR time, not release time

**Developer Feedback:**
1. Developer writes `<Button label="text" />`
2. `ucp lint` (via editor integration or manual) shows: `warning: unknown prop "label" on Button`
3. Developer checks spec, fixes usage

## 4. Business Rules

| ID | Rule | Source |
|----|------|--------|
| BR-001 | Lint rules operate on component usages extracted from source files, not on the spec itself. | Scoping |
| BR-002 | Import resolution maps source identifiers to spec component names within a single project. | Scoping |
| BR-003 | All lint output must include: file path, line number, rule ID, component name, and human-readable message. | UX |
| BR-004 | Exit code 0 means clean; 1 means violations found; 2 means tool error (bad spec, unreadable file). | CI |
| BR-005 | SARIF output must be compatible with GitHub Code Scanning (SARIF v2.1.0). | Integration |

## 5. Stakeholder Needs

**SN-01 (Platform Engineer):** "I want to add `ucp lint` to my CI pipeline so PRs
that introduce spec violations are flagged automatically."

**SN-02 (Design-System Maintainer):** "I want to run `ucp lint` before releases
to generate a drift report showing what changed since the last tagged spec."

**SN-03 (Developer):** "I want `ucp lint` to tell me exactly which prop is wrong
and what the spec says it should be, so I can fix it without reading the spec JSON."

## 6. Traceability

| Business Goal | Stakeholder Need | SRS Requirement |
|---------------|------------------|----------------|
| BG-01 | SN-01 | REQ-FUNC-001 |
| BG-02 | SN-01, SN-02 | REQ-FUNC-002..006 |
| BG-03 | SN-03 | NFR-USAB-001 |
| BG-04 | SN-01 | REQ-FUNC-010 |
```

---

## 3. Software Requirements Specification (`ucp-v014-srs.md`)

```markdown
# Software Requirements Specification for UCP v0.14

| Field | Value |
|-------|-------|
| Project | UCP |
| Document | Software Requirements Specification |
| Version | 0.1 (Draft) |
| Date | 2025-07-13 |
| Status | Draft — Pending Review |

## 1. Introduction

This SRS specifies the `ucp lint` subcommand for v0.14 "UCP Lint." It builds
on the existing `SynthesisOutput` format and adds source-file scanning with
spec-conformance checking.

## 2. System Context

```
                    ┌──────────────┐
  spec.json ──────▶│              │
  (SynthesisOutput)│   ucp lint    │────▶ Terminal / JSON / SARIF
                    │              │
  source/           │  Extractors  │
  ├── Button.jsx    │  ├── jsx.rs  │
  ├── Dialog.tsx    │  ├── rsx.rs  │
  └── Modal.vue     │  └── vue.rs  │
                    │  Rules       │
                    │  ├── unknown  │
                    │  ├── missing  │
                    │  ├── type-mis  │
                    │  ├── deprec    │
                    │  └── enum      │
                    └──────────────┘
```

## 3. Functional Requirements

### FEAT-01: CLI Interface

**REQ-FUNC-001 (Must):**  
*Event-driven:* When the user invokes `ucp lint --spec <path> --source <dir>`,
the system shall scan all source files in `<dir>`, compare component usages
against the spec, and report violations.

**REQ-FUNC-002 (Must):**  
*Ubiquitous:* The system shall accept `--format <fmt>` with values `terminal`
(default), `json`, and `sarif`.

**REQ-FUNC-003 (Should):**  
*Event-driven:* The system shall accept `--rules <rule1>,<rule2>` to restrict
which rules run. If omitted, all rules run.

**REQ-FUNC-004 (Should):**  
*Event-driven:* The system shall accept `--ignore <pattern>` to skip files
matching a glob pattern.

**REQ-FUNC-005 (Must):**  
*Unwanted behaviour:* If the spec file cannot be loaded, the system shall exit
with code 2 and an error message before scanning any source files.

**REQ-FUNC-006 (Must):**  
*Unwanted behaviour:* If a source file cannot be parsed (binary, encoding error),
the system shall skip it with a warning and continue scanning other files.

### FEAT-02: Source Extraction

**REQ-FUNC-010 (Must):**  
*Ubiquitous:* The system shall extract component usages from `.jsx`, `.tsx`,
`.rs` (RSX), and `.vue` files.

**REQ-FUNC-011 (Must):**  
*Ubiquitous:* For JSX/RSX files, the system shall resolve local imports to map
source identifiers to spec component names (e.g., `import { Button } from './button'`
maps `<Button>` to the `Button` component in the spec).

**REQ-FUNC-012 (Should):**  
*Ubiquitous:* The system shall handle default exports
(e.g., `import Button from './Button'`).

**REQ-FUNC-013 (May):**  
*Optional:* The system shall handle aliased imports
(e.g., `import { Button as Btn } from './button'`).

**REQ-FUNC-014 (Must):**  
*Ubiquitous:* For each component usage, the system shall extract the
component name, file path, line number, and all prop assignments.

**REQ-FUNC-015 (Should):**  
*Event-driven:* The system shall handle JSX prop spread patterns by skipping
the spread prop with a note rather than failing.

### FEAT-03: Lint Rules

**REQ-FUNC-020 (Must): Rule `unknown-prop`**  
*Event-driven:* When a component usage includes a prop that does not exist in the
spec's prop list, the system shall emit a violation with severity `error`.

**REQ-FUNC-021 (Must): Rule `missing-required`**  
*Event-driven:* When a component usage omits a prop that is marked as required
(reactivity = `staticValue` / `Static`), the system shall emit a violation
with severity `error`.

**REQ-FUNC-022 (Should): Rule `type-mismatch`**  
*Event-driven:* When a prop value's detected type does not match the spec's
declared type, the system shall emit a violation with severity `warning`.

**REQ-FUNC-023 (Should): Rule `deprecated`**  
*Event-driven:* When a component or prop is marked as deprecated in the spec's
provenance or curation log, the system shall emit a violation with
severity `warning`.

**REQ-FUNC-024 (May): Rule `enum-violation`**  
*Event-driven:* When a prop's concrete type is `enum: val1,val2` and the usage
provides a value not in the enum, the system shall emit a violation with
severity `error`.

**REQ-FUNC-025 (Should): Rule `no-spec`**  
*Event-driven:* When a JSX/RSX element matches a known import but no matching
component exists in the spec, the system shall emit a violation with
severity `warning`.

### FEAT-04: Output Formats

**REQ-FUNC-030 (Must):**  
*Ubiquitous:* Terminal output shall group violations by file, then by rule,
showing file path, line, column, rule ID, severity, and message.

**REQ-FUNC-031 (Must):**  
*Ubiquitous:* JSON output shall conform to:
```json
{
  "version": "1.0",
  "spec": "<path>",
  "source_dir": "<path>",
  "files_scanned": 42,
  "violations": [
    {
      "file": "src/App.tsx",
      "line": 15,
      "column": 5,
      "rule": "unknown-prop",
      "severity": "error",
      "component": "Button",
      "prop": "colorz",
      "message": "Unknown prop \"colorz\" on Button. Did you mean \"color\"?",
      "suggestion": null
    }
  ],
  "summary": { "errors": 3, "warnings": 1, "clean": false }
}
```

**REQ-FUNC-032 (Must):**  
*Ubiquitous:* SARIF output shall conform to SARIF v2.1.0 with:
- Tool name: `ucp-lint`
- Rule IDs matching the rule identifiers
- Results mapped to physical locations

**REQ-FUNC-033 (Should):**  
*Event-driven:* When a violation has an obvious fix (e.g., unknown prop is a
typo of a known prop), the system shall include a `suggestion` field in JSON
output.

## 4. Non-functional Requirements

| ID | Characteristic | Requirement | Fit Criterion |
|----|----------------|-------------|---------------|
| NFR-PERF-001 | Scan speed | Scan 1000 files (< 500 lines each) in < 5 seconds | Measured on reference hardware |
| NFR-PERF-002 | Startup time | Load spec and initialize in < 200ms | Measured |
| NFR-REL-001 | Multi-language | Support JSX, RSX, and Vue SFC in a single invocation | Verified with test fixtures |
| NFR-REL-002 | SARIF compat | Valid SARIF v2.1.0 accepted by GitHub Code Scanning | Verified with `sarif-validator` |
| NFR-USAB-001 | Actionability | Each violation message includes the component name, prop name, and what the spec expects | Manual review of sample output |
| NFR-USAB-002 | Low noise | False positive rate < 5% on a reference codebase | Measured |
| NFR-SEC-001 | No code execution | Linter must not compile, run, or execute source code | Verified by design review |
| NFR-ROB-001 | Determinism | Same input always produces same output | Verified with idempotency test |

## 5. Constraints

- Must compile with Rust stable 1.80+.
- Must not require Node.js, Python, or any external runtime.
- Must handle source files up to 10,000 lines without stack overflow.
- Must be path-length aware (max 4096 chars per line).
```

---

## 4. Architecture & Design (`ucp-v014-architecture.md`)

```markdown
# Architecture & Design Specification for UCP v0.14

| Field | Value |
|-------|-------|
| Project | UCP |
| Document | Architecture & Design Specification |
| Version | 0.1 (Draft) |
| Date | 2025-07-13 |
| Status | Draft — Pending Review |

## 1. Architecturally Significant Requirements

| ID | ASR | Impact |
|----|-----|--------|
| ASR-001 | Multi-language extraction (NFR-REL-001) | Separate extractor modules per framework |
| ASR-002 | Scan speed <5s for 1000 files (NFR-PERF-001) | Parallel file scanning, no compilation |
| ASR-003 | SARIF v2.1.0 (NFR-REL-002) | SARIF builder module |
| ASR-004 | No code execution (NFR-SEC-001) | Regex/text-based extraction only |
| ASR-005 | Determinism (NFR-ROB-001) | No randomization, sorted output |

## 2. Module Structure

```
ucp-maintainer/src/lint/
├── mod.rs                    # Public API: run_lint(), LintConfig, LintResult
├── extractors/
│   ├── mod.rs                # Extractor trait
│   ├── jsx.rs                # JSX/TSX extraction
│   ├── rsx.rs                # Dioxus RSX extraction
│   └── vue.rs                # Vue SFC extraction
├── rules/
│   ├── mod.rs                # Rule trait, RuleViolation, Severity
│   ├── unknown_prop.rs
│   ├── missing_required.rs
│   ├── type_mismatch.rs
│   ├── deprecated.rs
│   ├── enum_violation.rs
│   └── no_spec.rs
├── output/
│   ├── mod.rs                # Output formatter trait
│   ├── terminal.rs           # Human-readable colored output
│   ├── json.rs               # JSON output
│   └── sarif.rs              # SARIF v2.1.0 output
└── import_resolve.rs         # Map source identifiers to spec names
```

## 3. Key Abstractions

### Extractor Trait
```rust
trait Extractor {
    fn language(&self) -> &str;           // "jsx", "rsx", "vue"
    fn extensions(&self) -> &[&str];      // [".jsx", ".tsx"]
    fn extract_usages(&self, content: &str, file_path: &Path)
        -> Vec<ComponentUsage>;
}
```

### ComponentUsage
```rust
struct ComponentUsage {
    file: PathBuf,
    line: usize,
    column: usize,
    element_name: String,     // As written in source (e.g., "Button")
    resolved_name: Option<String>, // After import resolution (e.g., "Button")
    props: Vec<PropAssignment>,
    is_self_closing: bool,
}

struct PropAssignment {
    name: String,
    value_span: (usize, usize),  // (start, end) in source
    value_text: String,          // Raw text for display
    detected_type: DetectedType, // String, Number, Bool, Object, Expression, Unknown
}
```

### Rule Trait
```rust
trait Rule {
    fn id(&self) -> &str;               // "unknown-prop"
    fn description(&self) -> &str;
    fn default_severity(&self) -> Severity;

    fn check(&self, usage: &ComponentUsage, spec: &SynthesisOutput)
        -> Option<RuleViolation>;
}
```

### RuleViolation
```rust
struct RuleViolation {
    file: PathBuf,
    line: usize,
    column: usize,
    rule_id: String,
    severity: Severity,
    component: String,
    message: String,
    suggestion: Option<String>,
}
```

## 4. Data Flow

```
1. Load SynthesisOutput
2. Walk source directory (parallel with rayon)
3. For each file:
   a. Match extension to extractor
   b. Extract ComponentUsages
   c. Resolve imports → map element_name to spec component name
4. For each usage × each enabled rule:
   a. Call rule.check(usage, spec)
   b. Collect violations
5. Sort violations by (file, line)
6. Format output
7. Exit with 0 (clean), 1 (violations), or 2 (error)
```

## 5. JSX Extraction Strategy

**Import resolution:**
1. Scan for `import { Name }` and `import Name from` patterns
2. Build a map: `Name → Name` (local imports resolve to themselves)
3. `<Name .../>` is then looked up in the map AND in the spec

**Prop extraction (regex-based for v0.14):**
```
Pattern:  name="value"              → String
Pattern:  name='value'              → String  
Pattern:  name={expr}               → Expression (skip type check)
Pattern:  name={42}                 → Number
Pattern:  name                       → Bool (shorthand)
Pattern:  name={...}                 → Unknown (skip type check)
Pattern:  {...props}                → Spread (skip, note in output)
```

**RSX extraction (Dioxus):**
Similar to JSX but uses `rsx!` macro boundaries. Detect component usage
within `rsx!` blocks by finding uppercase-starting identifiers followed by
`<` or `/>`.

**Vue SFC extraction:**
Parse `<template>` block content, apply JSX-like extraction rules.

## 6. Type Detection Heuristics

For `type-mismatch` rule (v0.14 is heuristic-based, not compiler-accurate):

| Detected Type | String Pattern | Number Pattern | Bool Pattern |
|---------------|----------------|-----------------|-------------|
| String | `"..."` or `'...'` | — | — |
| Number | — | digits, possibly with `.` | — |
| Bool | — | — | no `=`, no value |
| Object | `{` at start | — | — |
| Expression | `{` after `=` | — | — |

This is intentionally conservative: if the type is `Expression` or `Unknown`,
the type-mismatch rule does **not** fire (to avoid false positives).

## 7. ADRs

**ADR-0008: Regex-based extraction over AST parsing**

| Field | Value |
|-------|-------|
| Status | Proposed |
| Context | Need to extract component usages from JSX/RSX/Vue. Options: tree-sitter (accurate, heavy), regex (fast, fragile), custom parser |
| Decision | Use regex-based extraction for v0.14. Accept ~10% extraction inaccuracy on complex patterns (spread, dynamic components) |
| Consequences | Positive: no external dependencies, fast. Negative: misses some edge cases, higher false negatives. |

**ADR-0009: Parallel file scanning with rayon**

| Field | Value |
|-------|-------|
| Status | Proposed |
| Context | Need to scan thousands of files within 5 seconds |
| Decision | Use `rayon::par_iter` for file walking and extraction |
| Consequences | Positive: meets NFR-PERF-001. Negative: adds rayon dependency (already in workspace) |

## 8. Traceability

| SRS Requirement | Architecture Component | ADR |
|----------------|----------------------|-----|
| REQ-FUNC-010..015 | `extractors/` modules | ADR-0008 |
| REQ-FUNC-020..025 | `rules/` modules | — |
| REQ-FUNC-030 | `output/terminal.rs` | — |
| REQ-FUNC-031 | `output/json.rs` | — |
| REQ-FUNC-032 | `output/sarif.rs` | — |
| NFR-PERF-001 | rayon parallel scanning | ADR-0009 |
```

---

## 5. Behavioral Specification & Test Plan (`ucp-v014-test.md``

```markdown
# Behavioral Specification & Test Verification Plan for UCP v0.14

| Field | Value |
|-------|-------|
| Project | UCP |
| Document | Behavioral Specification & Test Verification Plan |
| Version | 0.1 (Draft) |
| Date | 2025-07-13 |
| Status | Draft — Pending Review |

## 1. Behavioral Specifications

### Feature: CLI Interface

**Scenario: Basic invocation**
```gherkin
Given a valid spec file "spec.json"
And a source directory "./src" with JSX files
When the user runs "ucp lint --spec spec.json --source ./src"
Then the exit code shall be 0 if clean, 1 if violations found, 2 on error
And output shall include the spec path and source directory
```

**Scenario: JSON output**
```gherkin
Given a spec with a Button component that has a "label" prop
And a source file with "<Button labelz=\"click\" />"
When the user runs "ucp lint --spec spec.json --source ./src --format json"
Then the output shall be valid JSON
And shall contain a "violations" array with one entry
And the entry shall have rule "unknown-prop" and include a suggestion
```

**Scenario: SARIF output**
```gherkin
Given a spec and source directory with violations
When the user runs "ucp lint --spec spec.json --source ./src --format sarif"
Then the output shall be valid SARIF v2.1.0 JSON
And shall be accepted by GitHub Code Scanning
```

**Scenario: Skip binary files**
```gherkin
Given a source directory containing an image file "logo.png"
When the user runs "ucp lint"
Then the system shall skip the file with a warning
And shall continue scanning other files
```

### Feature: Import Resolution

**Scenario: Named import**
```gherkin
Given a file with "import { Button } from './button'"
And the spec has a component with id ending in ":Button"
When the user runs "ucp lint"
And the file contains "<Button label=\"text\" />"
Then the usage shall be resolved to the Button component in the spec
And lint rules shall check against Button's props
```

**Scenario: Default import**
```gherkin
Given a file with "import Button from './Button'"
And the spec has a Button component
When the user runs "ucp lint"
Then the import shall be resolved correctly
```

### Feature: Lint Rules

**Scenario: unknown-prop (exact match)**
```gherkin
Given a spec where Button has props: ["label", "onClick"]
And a source file with "<Button colorz=\"blue\" />"
When the unknown-prop rule runs
Then a violation shall be emitted with severity "error"
And the message shall mention "colorz" and the available props
```

**Scenario: unknown-prop (typo suggestion)**
```gherkin
Given a spec where Button has props: ["label", "onClick"]
And a source file with "<Button lable=\"text\" />"
When the unknown-prop rule runs
Then a violation shall be emitted
And the suggestion shall be "Did you mean \"label\"?"
```

**Scenario: missing-required**
```gherkin
Given a spec where Button has a required prop "label" (reactivity: staticValue)
And a source file with "<Button onClick={handler} />" (no label)
When the missing-required rule runs
Then a violation shall be emitted with severity "error"
And the message shall say "Required prop \"label\" not provided"
```

**Scenario: missing-required with optional prop**
```gherkin
Given a spec where Button has an optional prop "variant" (reactivity: not Static)
And a source file with "<Button />" (no variant)
When the missing-required rule runs
Then no violation shall be emitted for "variant"
```

**Scenario: type-mismatch string vs number**
```gherkin
Given a spec where Button has a prop "count" with type "Number"
And a source file with "<Button count=\"three\" />"
When the type-mismatch rule runs
Then a violation shall be emitted with severity "warning"
And the message shall say "Expected Number, got String"
```

**Scenario: type-mismatch with expression (no fire)**
```gherkin
Given a spec where Button has a prop "count" with type "Number"
And a source file with "<Button count={calcCount()} />"
When the type-mismatch rule runs
Then no violation shall be emitted (expression type is unknown)
```

**Scenario: no-spec**
```gherkin
Given a file importing { Widget } from './widget'
And the spec has no component matching "Widget"
When the no-spec rule runs
Then a violation shall be emitted with severity "warning"
And the message shall say "Component \"Widget\" not found in spec"
```

**Scenario: enum-violation**
```gherkin
Given a spec where Button has prop "variant" with concrete_type "enum: primary,secondary"
And a source file with "<Button variant=\"danger\" />"
When the enum-violation rule runs
Then a violation shall be emitted with severity "error"
And the message shall list the allowed values
```

**Scenario: deprecated (provenance)**
```gherkin
Given a spec where Button's provenance marks it as deprecated
And a source file using "<Button />"
When the deprecated rule runs
Then a violation shall be emitted with severity "warning"
And the message shall say "Component \"Button\" is deprecated"
```

### Feature: Performance

**Scenario: Scan speed**
```gherkin
Given a source directory with 1000 JSX files (avg 200 lines each)
When the user runs "ucp lint --source ./src"
Then the command shall complete in under 5 seconds on reference hardware
```

## 2. Test Strategy

| Level | Scope | Tool |
|-------|-------|------|
| Unit | Per-rule: `check()` returns correct violations | cargo test |
| Unit | Per-extractor: extracts usages from fixture strings | cargo test |
| Unit | Import resolver: maps identifiers correctly | cargo test |
| Integration | Full `run_lint()` on fixture directories | cargo test |
| Integration | JSON/SARIF output validation | cargo test |
| Integration | Exit codes: 0/1/2 for clean/dirty/error | cargo test |
| E2E | `ucp lint` binary invocation on real repo | cargo nextest |
| Performance | 1000-file scan under 5 seconds | cargo test (with timing) |

## 3. Requirements Traceability Matrix (Extract)

| ID | Rule | Scenario | Test Case |
|----|------|----------|-----------|
| REQ-FUNC-001 | Run lint | Basic invocation | TC-LINT-001 |
| REQ-FUNC-002 | Output formats | JSON output | TC-LINT-002 |
| REQ-FUNC-003 | Rule filtering | — | TC-LINT-003 |
| REQ-FUNC-010 | JSX extraction | Named import | TC-EXT-001 |
| REQ-FUNC-011 | Import resolve | Default import | TC-EXT-002 |
| REQ-FUNC-014 | Prop extraction | — | TC-EXT-003 |
| REQ-FUNC-020 | unknown-prop | Exact match | TC-RULE-001 |
| REQ-FUNC-020 | unknown-prop | Typo suggestion | TC-RULE-002 |
| REQ-FUNC-021 | missing-required | Required prop omitted | TC-RULE-003 |
| REQ-FUNC-021 | missing-required | Optional prop omitted | TC-RULE-004 |
| REQ-FUNC-022 | type-mismatch | String vs number | TC-RULE-005 |
| REQ-FUNC-022 | type-mismatch | Expression (no fire) | TC-RULE-006 |
| REQ-FUNC-024 | enum-violation | Invalid enum value | TC-RULE-007 |
| REQ-FUNC-025 | no-spec | Unknown import | TC-RULE-008 |
| NFR-PERF-001 | Scan speed | 1000 files | TC-PERF-001 |
| NFR-ROB-001 | Determinism | Idempotent run | TC-DET-001 |
```

## 4. Test Fixtures

```
ucp-maintainer/tests/lint/
├── fixtures/
│   ├── jsx/
│   │   ├── basic_usage.jsx        # Clean usage, no violations
│   │   ├── unknown_prop.jsx       # One unknown prop
│   │   ├── missing_required.jsx   # Missing a required prop
│   │   ├── type_mismatch.jsx     # Wrong type for a prop
│   │   ├── enum_violation.jsx    # Value not in enum
│   │   ├── spread_props.jsx      # Spread pattern (skipped gracefully)
│   │   ├── dynamic_component.jsx # Dynamic component (skipped)
│   │   ├── no_spec_import.jsx     # Import of unknown component
│   │   ├── deprecated.jsx        # Deprecated component usage
│   │   └── default_import.jsx   # Default export import
│   ├── rsx/
│   │   ├── basic_usage.rs        # Dioxus RSX clean usage
│   │   └── unknown_prop.rs        # RSX unknown prop
│   ├── vue/
│   │   └── basic_usage.vue       # Vue SFC clean usage
│   └── spec.json                 # Test spec matching fixtures
├── extractors/
│   ├── test_jsx.rs
│   ├── test_rsx.rs
│   └── test_vue.rs
├── rules/
│   ├── test_unknown_prop.rs
│   ├── test_missing_required.rs
│   ├── test_type_mismatch.rs
│   ├── test_enum_violation.rs
│   ├── test_deprecated.rs
│   └── test_no_spec.rs
├── test_import_resolve.rs
├── test_output_json.rs
├── test_output_sarif.rs
├── test_exit_codes.rs
└── test_performance.rs
```
```

---

## Summary

The v0.14 spec suite is laser-focused on **one thing done well**: a spec compliance linter that catches prop mismatches with low false positives and integrates into CI via SARIF output. The architecture is modular (extractors + rules + output formatters) so each piece can be improved independently in future versions.
