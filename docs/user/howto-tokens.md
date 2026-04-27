# How-to: Merge Design Tokens

Unify design tokens (CSS custom properties, Tailwind config) from multiple
sources.

## Merge CSS Token Files

``` bash
# Extract tokens from a Tailwind config
ucp export --target dtcg --spec spec.json --output tokens-a.json

# Merge two token files
ucp merge-tokens --input tokens-a.json --input tokens-b.json --output merged.json
```

## Handling Conflicts

By default, conflicting token values cause an error:

``` bash
ucp merge-tokens --input a.json --input b.json --output merged.json
# Error: Token merge has 2 conflict(s)
```

Add `--force` with a strategy:

``` bash
ucp merge-tokens --input a.json --input b.json --output merged.json \
  --strategy first-wins --force
```

Strategies: `error` (default), `first-wins`, `last-wins`.
