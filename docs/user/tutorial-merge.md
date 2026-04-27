# Tutorial: Merging Two Specs

Combine specs from different codebases into one canonical spec.

## Prerequisites

- Two UCP spec files from different codebases (see [Bootstrap Tutorial](tutorial-bootstrap.md))

## Step 1: Create Two Specs

For this tutorial, we'll simulate two codebases with different prop types
for the same component:

``` bash
# Codebase A: disabled is a boolean
mkdir -p codebase-a/src
echo '#[component] pub fn Button(disabled: bool) -> () { () }' > codebase-a/src/button.rs
ucp bootstrap --source-dir codebase-a --output-dir spec-a

# Codebase B: disabled is a string
mkdir -p codebase-b/src
echo '#[component] pub fn Button(disabled: String) -> () { () }' > codebase-b/src/button.rs
ucp bootstrap --source-dir codebase-b --output-dir spec-b
```

## Step 2: Merge

``` bash
ucp merge --input spec-a/ucp-spec.json --input spec-b/ucp-spec.json -o merged.json
```

Output:

```
   📁 Files scanned:   1
   📄 Files parsed:    1
   🧩 Components:     1
   ⚠️  Conflicts:       1
✅ Merge complete!
```

## Step 3: Inspect Conflicts

``` bash
ucp validate merged.json
```

You'll see:

```
⚠️  1 unresolved conflict(s).
```

To see details:

``` bash
ucp components merged.json --verbose
```

## Step 4: Resolve Conflicts

Use the interactive curation tool:

``` bash
ucp curate --merged merged.json --output curated.json
```

Navigate conflicts with arrow keys, press `a` to accept the suggestion,
`s` to skip, `q` to quit. The resolved spec is saved as `curated.json`.

## Step 5: Verify Resolution

``` bash
ucp validate curated.json
```

Should now show "valid with no conflicts".

## Next Steps

- [Curate conflicts interactively](tutorial-curate.md)
- [Export the curated spec](tutorial-export.md)
- [Learn about conflict detection](../concepts/specs.md)
