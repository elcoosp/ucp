# Provenance Tracking

Every merged UCP spec records where its components and props came from.

## Provenance Array

After a merge, the spec includes a `provenance` array recording each
input's fingerprint and merge timestamp.

``` json
"provenance": [
  {
    "source_spec_path": "spec-a/ucp-spec.json",
    "source_fingerprint": "0000000000000002",
    "merged_at": "1747276800"
  },
  ...
]
```

## Curation Log

When conflicts are resolved interactively, a `curation_log` records
every decision:

``` json
"curation_log": [
  {
    "conflict_id": "conf_001",
    "chosen_resolution": "IncludeMajority",
    "rationale": "Accepted via TUI",
    "timestamp": "1747276800"
  }
]
```

## Source Attribution

Each prop includes a `sources` array mapping back to original file and
type name:

``` json
"sources": [
  {
    "repo_id": "src/button.rs",
    "original_name": "disabled",
    "original_type": "bool"
  }
]
```
