# Tutorial: Exporting a Spec

Convert a curated UCP spec into multiple target formats.

## Prerequisites

- A curated spec (see [Curation Tutorial](tutorial-curate.md))

## Step 1: Export to a Single Format

``` bash
ucp export --target design-md --spec curated.json --output ./design --library my-lib --version 1.0.0
cat design/DESIGN.md
```

## Step 2: Export All Formats at Once

``` bash
ucp export-all --spec curated.json --output ./exports --library my-lib --version 1.0.0
```

This produces:

```
exports/
├── a2ui/
│   └── a2ui-catalog.json       # A2UI v0.9 catalog
├── ag-ui/
│   └── ag-ui-events.json       # AG-UI event schema
├── w3c/
│   └── ucp-spec.w3c.json       # W3C UI Specification format
├── design-md/
│   └── DESIGN.md               # AI‑native design spec
├── llms-txt/
│   └── llms.txt                # LLM‑friendly documentation
├── ai-contract.json            # AI contract for tool use
└── registry/
    ├── registry.json           # shadcn v4 registry index
    └── registry-item-*.json    # Individual component registry items
```

## Step 3: Generate Framework Code

``` bash
# Generate Dioxus components
ucp generate --target dioxus --spec curated.json --output ./generated/dioxus

# Generate React components
ucp generate --target react --spec curated.json --output ./generated/react
```

## Next Steps

- [Command Reference](commands/index.md)
- [Understanding provenance](../concepts/provenance.md)
