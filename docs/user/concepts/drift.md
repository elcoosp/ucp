# Drift Detection

Source code changes over time, and a canonical spec can become stale.
Drift detection compares the spec against a fresh extraction from source.

## How It Works

1. Re‑extract components from the source directory.
2. Compare each component's props against the canonical spec.
3. Report any differences (added, removed, or changed types).

## Running Drift Detection

``` bash
ucp verify --spec canonical.json --source-dir src
```

## Understanding the Report

```
⚠️  Drift detected:
  ~ rust:button.rs:Button (confidence: 0.90)
      disabled: ControlFlag → UncontrolledValue(ControlFlag)
  + rust:dialog.rs:Dialog (in source, not in spec)
  - rust:old.rs:OldComponent (in spec, missing from source)
```

- **Drifted props**: type changed between spec and source.
- **New in source**: component exists in source but not in spec.
- **Missing from source**: component exists in spec but not in source.

## Confidence

Confidence is high (0.9) for single‑prop changes and lower (0.7) for
multi‑prop changes, indicating more uncertainty about whether the change
is intentional.

## Next Steps

After detecting drift, you can:

1. Re‑merge the fresh extraction with the canonical spec.
2. Curate the resulting conflicts.
3. Update the canonical spec.

See [Merge Tutorial](../tutorial-merge.md) and
[Curation Tutorial](../tutorial-curate.md).
