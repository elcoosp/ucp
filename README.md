<div align="center">
  <img src="https://img.shields.io/badge/Status-AI_Synthesized-blue?style=for-the-badge" alt="AI Synthesized">
  <img src="https://img.shields.io/badge/Targets-Web_Desktop_Mobile-purple?style=for-the-badge" alt="Multi-Platform">
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="License">
  
  <h1>Universal Component Protocol (UCP)</h1>
  
  <p>
    <strong>The AI-synthesized standard for UI components.</strong><br>
    Stop writing specs. Start computing consensus.
  </p>
</div>

---

## 💡 The Paradigm Shift

For years, the UI ecosystem has tried to unify component libraries (React, Vue, Leptos, Dioxus, GPUI, SwiftUI) by **manually writing** specification documents. This always fails. Libraries diverge, APIs change, and behavioral nuances (like focus trapping or scroll locking) are impossible to standardize via text.

**UCP v4.0 doesn't ask you to write a spec. It computes one.**

Using a multi-agent AI pipeline, UCP discovers existing shadcn/ui ports across the internet, deeply extracts their APIs and behaviors via AST parsing and LLM reasoning, and intelligently unifies them into a single, mathematically resolved canonical specification.

Your existing codebase isn't conforming to a document—**your codebase *is* the document.**

---

## ✨ What Makes UCP Awesome?

UCP isn't just a list of props. It is the deepest component contract ever attempted.

*   🤖 **AI-Bootstrapped:** Run `ucp bootstrap` and watch the AI discover, extract, and unify 50+ components across 5 frameworks in minutes.
*   🧠 **Behavioral State Machines:** Components aren't just defined by props, but by exact state transitions (e.g., `closed → open`) and strict side-effects (focus trap, scroll lock, portaling).
*   🧩 **Component Anatomy:** Defines `parts` (e.g., `button::label`) and `state attributes` (e.g., `[disabled]`), enabling true framework-agnostic theming.
*   🌍 **Multi-Platform Profiles:** Separate conformance criteria for Web (DOM/ARIA) and Desktop (Native Accessibility APIs/Builder Patterns).
*   🤖 **LLM-Ready Annotations:** Every component includes `behaviorDescription`, `antiPatterns`, and `fewShotExamples`, making UCP the ultimate training data for AI coding agents (Cursor, v0, Bolt).
*   🧬 **Signal-Aware Types:** Natively understands `Signal<T>`, `MaybeSignal<T>`, `Callback<T>`, and `Entity<T>`—not just `string` and `boolean`.
*   📡 **Compound Context Contracts:** Explicitly defines the shared state linking compound components (like `Dialog` + `DialogTrigger`).

---

## 🚀 Quick Start

Install the UCP CLI and synthesize your first spec from the wild:

```bash
# Install the UCP toolchain
cargo install ucp-cli

# Bootstrap a spec from existing shadcn/ui ports
ucp bootstrap \
  --discover "github topic:shadcn" \
  --extract \
  --unify \
  --generate \
  --output-dir ./ucp-spec-v1/

# Review the AI-generated conflicts and accept/reject them
ucp resolve ./ucp-spec-v1-proposal/graph.json

# Generate a Leptos component from the unified spec
ucp generate button --target leptos --fmd ./fmd/leptos.json
```

---

## 🧠 The AI Unification Pipeline

The core of UCP is the `ucp-synthesizer`. Here is how it turns fragmented codebases into a single source of truth:

```text
1. DISCOVER          2. EXTRACT          3. UNIFY            4. GENERATE
┌─────────────┐    ┌──────────────┐    ┌───────────────┐    ┌──────────────┐
│ GitHub/NPM  │───▶│ AST + LLM    │───▶│ Canonical      │───▶│ UCP JSON     │
│ Crates.io   │    │ Analysis     │    │ Abstract Model │    │ Schema +     │
│ GitLab      │    │              │    │ (Equivalence   │    │ Auto-FMDs    │
└─────────────┘    └──────────────┘    │ Resolution)   │    └──────────────┘
                                        └───────────────┘
```

1.  **Discovery Agent:** Finds `shadcn-ui` in React, `leptos-shadcn`, `dioxus-shadcn`, and `gpui-component`.
2.  **Extraction Agent:** Uses Rust's `syn`/TS compilers to get types, then uses an LLM to infer *behavior* (Does it trap focus? Does it portal?).
3.  **Unification Agent:** Maps React's `useState` + `onChange` to Leptos's `RwSignal<T>` and GPUI's `Model<T>` into a `CanonicalAbstractModel`. Flags conflicts.
4.  **Generation Agent:** Outputs the final UCP JSON, complete with auto-generated LLM annotations and Framework Mapping Documents (FMDs).

---

## 📄 A Glimpse of the Output

When the AI synthesizes a `Dialog`, it doesn't just list props. It outputs this:

```json
{
  "name": "dialog",
  "stateMachine": {
    "states": ["closed", "open", "closing"],
    "transitions": [
      {
        "from": "closed", "to": "open", "trigger": "onOpenChange(true)",
        "sideEffects": [
          "focus: move to first focusable in [part=content]",
          "scroll: lock body scroll",
          "portal: render [part=content] in body portal"
        ]
      }
    ]
  },
  "parts": [
    { "name": "overlay", "selectable": true },
    { "name": "content", "selectable": true }
  ],
  "llm": {
    "behaviorDescription": "A modal overlay that traps focus and locks scroll.",
    "antiPatterns": ["Do NOT render inline in the DOM tree."]
  }
}
```

---

## ⚙️ Architecture

UCP is a modular Rust workspace designed for speed, safety, and extensibility.

| Crate | Purpose |
| :--- | :--- |
| `ucp-core` | Data structures, JSON Schema, validation, and the `CanonicalAbstractModel`. |
| `ucp-synthesizer` | The AI multi-agent pipeline (Discovery, Extraction, Unification). |
| `ucp-cli` | The command-line interface (`bootstrap`, `generate`, `test`, `resolve`). |
| `ucp-conform` | Compile-time proc-macro (`#[ucp_conform(spec = "button.json")]`). |
| `ucp-harness-leptos` | Web/DOM conformance test harness. |
| `ucp-harness-gpui` | Desktop/Manifest conformance test harness. |
| `ucp-dashboard` | Static site generator for the conformance matrix. |

---

## 🛡️ Tiered Conformance (Per Platform)

Because a desktop app (GPUI) has no DOM, UCP conformance is scoped by **Platform Profile**:

*   🥉 **Bronze (API & Anatomy):** Prop/part names match. Canonical stories render.
*   🥈 **Silver (State Machines):** The exact state machine transitions execute correctly. Focus and scroll side-effects pass.
*   🥇 **Gold (Accessibility):** ARIA roles (Web) or Native APIs (Desktop) match. Inter-component constraints (e.g., `aria-labelledby`) verified.

---

## 🤝 Governance: Human-in-the-Loop

The AI proposes, humans dispose. 

The output of `ucp bootstrap` is a **Synthesis SEP** (Spec Enhancement Proposal). It includes a visual graph showing exactly *why* the AI mapped `MaybeProp<T>` to a specific React type, and flags conflicts where 3 out of 4 libraries have a `loading` prop. Spec Editors use `ucp resolve` to accept, reject, or tweak the AI's decisions before the spec becomes canonical.

---

## 📜 License

UCP is released under the [MIT License](./LICENSE).

---
<div align="center">
  <sub>Built with 🦀 by the community, computed by AI.</sub>
</div>
