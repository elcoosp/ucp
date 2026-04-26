# UCP v0.6.0 — Full Specification Suite

## Product Vision & Strategic Alignment

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Product Vision & Strategic Alignment |
| Version | 0.6.0‑vision‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Vision Statement

> **UCP achieves full framework parity — extract, unify, and regenerate across all supported ecosystems.**
> v0.6.0 closes the round‑trip loop for React and GPUI, extends context detection to Leptos and React, and delivers a unified design‑token pipeline that works across all frameworks. Every framework UCP extracts from can now generate idiomatic, compilable code back.

### 2. Elevator Pitch

> **For** component‑library maintainers who support multiple frameworks
> **who need** a single pipeline that extracts from any source and regenerates for any target,
> **our product** is a CLI toolchain that unifies UI components
> **that now provides** full round‑trip for React, GPUI, Dioxus, and Leptos — plus cross‑framework context detection, spread‑attribute handling, and design‑token extraction.
> **Unlike** manual porting or single‑framework tools,
> **our solution** lets you extract a shadcn React library, generate a Dioxus port, detect conflicts, produce shadcn CLI v4 registries with both React and Dioxus source code, and publish A2UI catalogs — all from one pipeline.

### 3. Problem Statement

v0.5.0 made UCP the bridge to AI agents and shadcn CLI v4. But the pipeline is asymmetric: React and GPUI components can be extracted but not generated. Leptos and React lack context detection. Design tokens only work for Dioxus. This asymmetry prevents true round‑tripping and limits the value of registry exports — a React‑first ecosystem can't consume a registry whose items contain only Dioxus source code.

### 4. Business Objectives

| ID | Objective | Key Results |
|----|-----------|-------------|
| G-01 | Full round-trip parity | React & GPUI code generators produce compilable output |
| G-02 | Extraction parity | React `...rest` & context; Leptos context detection |
| G-03 | Unified design-token pipeline | Token extraction works for Dioxus, React, Leptos |
| G-04 | Registry items carry framework-native code | React registry items contain React source; Dioxus items contain Dioxus source |

### 5. Goals and Non-Goals

**Goals (v0.6.0):**
- React code generator (FC + function components, JSX output)
- GPUI code generator (struct + builder pattern)
- React context detection (`useContext`, `createContext`, `...rest` props)
- Leptos context detection (`provide_context`, `use_context`)
- Multi‑framework design‑token extraction (Tailwind config, CSS modules)

**Non-Goals:**
- No new framework extractors (Svelte, Vue, Solid — deferred)
- No visual component preview in dashboard

---

## Software Requirements Specification – UCP v0.6.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | SRS v0.6.0 |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Introduction

This SRS defines v0.6.0 capabilities: React code generator, GPUI code generator, React context extraction, Leptos context detection, and multi‑framework design‑token extraction.

### 2. Functional Requirements

#### 2.1 React Code Generator

**REQ‑RCT‑001 – Generate React functional components**
> **When** given a `PackageManifest` with `frameworks: ["react"]` and `--target react`, **the system** shall produce a `src/` directory containing one `.tsx` file per component.

*Acceptance criteria:*
- Each file exports a named functional component.
- Props are defined as a TypeScript interface.
- Default values use destructured defaults or `defaultProps`.
- The `cargo check` equivalent (`tsc --noEmit`) passes.
- Generated code uses idiomatic React patterns (not Dioxus/Leptos syntax).

**REQ‑RCT‑002 – Prop mapping for React**
> **The React generator** shall map CAM types to TypeScript:
> - `ControlFlag` → `boolean`
> - `StaticValue(Any)` with concrete `"String"` → `string`
> - `SpreadAttributes` → `React.HTMLAttributes<HTMLElement>` via `...rest`
> - `AsyncEventHandler` → `() => void`
> - `Renderable` → `React.ReactNode`
> - `ControlledValue` / `UncontrolledValue` → generics or `any`

#### 2.2 GPUI Code Generator

**REQ‑GPU‑001 – Generate GPUI component stubs**
> **When** given a `PackageManifest` with `frameworks: ["gpui"]` and `--target gpui`, **the system** shall produce a `src/` directory with one `.rs` file per component using GPUI's `#[derive(IntoElement)]` pattern.

*Acceptance criteria:*
- Each component has a struct with builder methods.
- Props map to struct fields with appropriate types.
- The generated project compiles with `cargo check`.

#### 2.3 React Context Detection

**REQ‑RCT‑003 – Detect `useContext` and `createContext`**
> **The TSX extractor** shall detect `React.createContext` and `useContext` calls, storing the context type name in `provided_context` and `consumed_contexts`.

**REQ‑RCT‑004 – Detect spread props (`...rest`)**
> **When** a TSX interface contains `...other` or extends `React.HTMLAttributes`, **the extractor** shall set `is_spread_attributes = true` for that prop.

#### 2.4 Leptos Context Detection

**REQ‑LEP‑003 – Detect `provide_context` and `use_context`**
> **The Rust AST visitor** shall detect `provide_context::<Type>()` and `use_context::<Type>()` calls in Leptos `#[component]` functions, mirroring the existing Dioxus context detection.

#### 2.5 Multi‑Framework Design Token Extraction

**REQ‑TOK‑004 – Extract tokens from Tailwind config**
> **The token extractor** shall parse `tailwind.config.{js,ts}` and extract theme colors, spacing, and typography into the DTCG token structure.

**REQ‑TOK‑005 – Extract tokens from CSS modules**
> **The token extractor** shall parse `.module.css` and styled‑components patterns in TSX/TS files.

### 3. Quality Requirements

| ID | Requirement | Fit Criterion |
|----|-------------|---------------|
| NFR‑REL‑001 | Generated React project passes `tsc --noEmit` | CI step |
| NFR‑REL‑002 | Generated GPUI project compiles | `cargo check` in CI |
| NFR‑MNT‑001 | Test coverage ≥ 85% for new modules | Coverage report |
| NFR‑MNT‑002 | Zero clippy warnings | CI lint gate |

### 4. Traceability

| Goal | Capability | SRS Requirements |
|------|------------|------------------|
| G‑01 | React code generator | REQ‑RCT‑001, REQ‑RCT‑002 |
| G‑01 | GPUI code generator | REQ‑GPU‑001 |
| G‑02 | React context/extraction | REQ‑RCT‑003, REQ‑RCT‑004 |
| G‑02 | Leptos context detection | REQ‑LEP‑003 |
| G‑03 | Unified token extraction | REQ‑TOK‑004, REQ‑TOK‑005 |

---

## Architecture & Design Specification – UCP v0.6.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Architecture & Design Specification |
| Version | 0.6.0‑arch‑1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Context and Scope

v0.6.0 adds two new code generators (React, GPUI), extraction enhancements for React and Leptos, and multi‑framework design‑token extraction. No new frameworks are added; the focus is on achieving parity among the four already‑supported ecosystems.

### 2. Architecturally Significant Requirements

| ID | Requirement | Source |
|----|-------------|--------|
| ASR‑001 | React code generator with TypeScript output | REQ‑RCT‑001, REQ‑RCT‑002 |
| ASR‑002 | GPUI code generator | REQ‑GPU‑001 |
| ASR‑003 | React context + spread props extraction | REQ‑RCT‑003, REQ‑RCT‑004 |
| ASR‑004 | Leptos context detection | REQ‑LEP‑003 |
| ASR‑005 | Multi‑framework token extraction | REQ‑TOK‑004, REQ‑TOK‑005 |

### 3. System Design

#### 3.1 New Module: `generate::react`

**Purpose:** Transform `PackageManifest` into a React TypeScript project.

**Design:**
- Mirrors the structure of `generate::dioxus` and `generate::leptos`.
- Produces `.tsx` files with functional components and TypeScript interfaces.
- Uses a `tsconfig.json` template for the generated project.
- Share `to_snake_case` from `common`; file names become kebab‑case (React convention).

**Key type mapping:**
```
CAM ControlFlag + concrete "bool" → TypeScript "boolean"
CAM StaticValue + concrete "String" → TypeScript "string"
CAM SpreadAttributes → "...rest: React.HTMLAttributes<HTMLElement>"
CAM AsyncEventHandler → "() => void"
CAM Renderable → "React.ReactNode"
```

#### 3.2 New Module: `generate::gpui`

**Purpose:** Transform `PackageManifest` into a GPUI project.

**Design:**
- Produces `.rs` files with `#[derive(IntoElement)]` structs and builder methods.
- Each struct field becomes a builder method.
- Children handling via `ParentElement` trait implementation.

#### 3.3 Enhanced: `extract::tsx_ast`

Add visitors to detect:
- `React.createContext(Type)` / `useContext(TypeContext)` → store in `RawTsxExtraction` (new fields).
- `...rest` patterns and `extends React.HTMLAttributes` → set `is_spread_attributes = true`.

#### 3.4 Enhanced: `extract::rust_ast`

Add a `LeptosContextVisitor` that mirrors `DioxusVisitor`'s `ContextVisitor` but targets `provide_context::<Type>()` and `use_context::<Type>()`.

#### 3.5 Enhanced: `extract::tokens`

Add parsers for:
- `tailwind.config.{js,ts}` — extract `theme.colors`, `theme.spacing`, `theme.fontFamily`.
- `.module.css` — extract `:root` custom properties.
- Styled‑components tagged template literals.

### 4. Key Design Decisions (ADRs)

| ADR | Decision | Drivers |
|-----|----------|---------|
| ADR‑018 | React generator produces TypeScript (not JavaScript) | ASR‑001, industry standard for React projects |
| ADR‑019 | GPUI generator uses builder pattern | ASR‑002, matches GPUI idioms |
| ADR‑020 | TSX extractor enhanced in‑place (not new module) | ASR‑003, minimal churn |
| ADR‑021 | Token extraction uses pluggable parsers per file type | ASR‑005, extensible to future formats |

### 5. Traceability

| ASR | Design Elements | ADRs |
|-----|----------------|------|
| ASR‑001 | `generate::react` module | ADR‑018 |
| ASR‑002 | `generate::gpui` module | ADR‑019 |
| ASR‑003 | `extract::tsx_ast` enhancements | ADR‑020 |
| ASR‑004 | `LeptosContextVisitor` in `extract::rust_ast` | – |
| ASR‑005 | Pluggable parsers in `extract::tokens` | ADR‑021 |

---

## Behavioral Specification & Test Verification Plan – UCP v0.6.0

| Field | Value |
|-------|-------|
| Project | Universal Component Protocol (UCP) |
| Document | Behavioral Spec & Test Verification |
| Version | 0.1 (Draft) |
| Date | 2026-04-27 |
| Author | UCP Team |
| Status | Draft — Pending Review |

### 1. Behavioral Specifications

#### Feature: React Code Generator

```gherkin
Scenario: Generate a React button component
  Given a PackageManifest with a Button component:
    - prop "disabled": concrete_type="bool", abstract_type=ControlFlag
    - prop "label": concrete_type="String", abstract_type=StaticValue
    - prop "attributes": abstract_type=SpreadAttributes
    - prop "children": abstract_type=Renderable
  When the React generator runs
  Then a "src/button.tsx" file is created
  And it contains:
    - interface ButtonProps { disabled?: boolean; label: string; }
    - ...rest: React.HTMLAttributes<HTMLElement>
    - children: React.ReactNode
    - export function Button({ disabled = false, label, children, ...rest }) { ... }
  And tsc --noEmit passes on the generated project
```

#### Feature: GPUI Code Generator

```gherkin
Scenario: Generate a GPUI button component
  Given a PackageManifest with a Button component
  When the GPUI generator runs
  Then a "button.rs" file is created
  And it contains:
    - #[derive(IntoElement)]
    - struct Button { disabled: bool, label: SharedString, children: Option<AnyElement> }
    - impl Button { pub fn disabled(mut self, value: bool) -> Self { self.disabled = value; self } }
    - impl Render for Button { ... }
  And cargo check passes on the generated project
```

#### Feature: React Context Detection

```gherkin
Scenario: Detect React context provider
  Given a TSX file containing:
    ```
    const ThemeContext = React.createContext("light");
    export function ThemeProvider({ children }) {
      return <ThemeContext.Provider value="dark">{children}</ThemeContext.Provider>;
    }
    ```
  When the TSX extractor runs
  Then ThemeProvider has provided_context = Some("ThemeContext")

Scenario: Detect spread props in TSX interface
  Given a TSX interface: `interface ButtonProps extends React.HTMLAttributes<HTMLButtonElement> { ... }`
  When the TSX extractor processes it
  Then the "rest" prop has is_spread_attributes = true
```

#### Feature: Leptos Context Detection

```gherkin
Scenario: Detect Leptos provided context
  Given a Leptos component containing: `provide_context::<AppState>(state);`
  When the Rust extractor runs
  Then the component has provided_context = Some("AppState")
```

### 2. Test Strategy

| Layer | Goal | Tools |
|-------|------|-------|
| Unit tests | React/GPUI generator, context detection, token extraction | Rust #[test] |
| Integration tests | Full pipeline: extract → generate → validate | tempfile, tokio |
| Compilation tests | Generated React/GPUI projects compile | tsc, cargo check |
| Regression tests | All v0.5.0 tests continue to pass | cargo nextest |

### 3. Requirements Traceability Matrix

| Requirement | Test Case | Verification |
|-------------|-----------|--------------|
| REQ‑RCT‑001 | SCN‑RCT‑generate‑button | Test + tsc |
| REQ‑RCT‑002 | SCN‑RCT‑prop‑mapping | Unit test |
| REQ‑GPU‑001 | SCN‑GPU‑generate‑button | Test + cargo check |
| REQ‑RCT‑003 | SCN‑RCT‑context‑provider | Unit test |
| REQ‑RCT‑004 | SCN‑RCT‑spread‑props | Unit test |
| REQ‑LEP‑003 | SCN‑LEP‑context | Unit test |
| REQ‑TOK‑004 | SCN‑TOK‑tailwind | Unit test |
| REQ‑TOK‑005 | SCN‑TOK‑css‑modules | Unit test |

---

This completes the specification suite for v0.6.0. Would you like me to produce the implementation plan next?
