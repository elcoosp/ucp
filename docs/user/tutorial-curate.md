# Tutorial: Interactive Curation

Walk through resolving merge conflicts with the interactive TUI.

## Prerequisites

- A merged spec with conflicts (see [Merge Tutorial](tutorial-merge.md))

## Step 1: Launch Curation

``` bash
ucp curate --merged merged.json --output curated.json
```

## Step 2: TUI Controls

The terminal UI displays one conflict at a time:

| Key | Action |
|-----|--------|
| `←` / `→` | Previous / next conflict |
| `a` | Accept suggested resolution |
| `r` | Reject (leave unresolved) |
| `s` | Skip (leave unresolved) |
| `q` | Quit and save |

## Step 3: Example Resolution

For the Button `disabled` prop type conflict:

- **Present in:** `spec-a/ucp-spec.json` (ControlFlag)
- **Absent in:** `spec-b/ucp-spec.json`
- **Suggestion:** `IncludeMajority`

Press `a` to accept. The prop type will be set to `ControlFlag` and the
conflict marked as resolved.

## Step 4: After Curation

``` bash
ucp validate curated.json
```

Should show:

```
✅ Spec is valid with no conflicts.
```

The curation log is embedded in the spec:

``` bash
cat curated.json | python3 -c "import sys,json; d=json.load(sys.stdin); print(json.dumps(d.get('curation_log',[]), indent=2))"
```

## Step 5: Batch Mode (No TUI)

For CI or non‑interactive use, provide resolutions as a JSON file:

``` bash
echo '[{"conflict_id":"conf_001","chosen_resolution":"IncludeMajority"}]' > resolutions.json
# (not yet fully implemented – coming in v0.12)
```

## Next Steps

- [Export to all formats](tutorial-export.md)
- [Integrate into CI](howto-ci.md)
