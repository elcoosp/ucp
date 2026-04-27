# Merge Algorithm

How `merge_specs` combines multiple `SynthesisOutput` specs into one.

## Steps

### 1. Semantic Fingerprint Deduplication

Each component has a `semantic_fingerprint` containing:
- `purpose_hash`: hash of the component name (optionally enriched by LLM)
- `normalized_prop_names`: sorted list of prop names

Components with the same `purpose_hash` are considered the same logical
component and merged.

### 2. Prop Merging

For each deduplicated component:

- Props with the same `canonical_name` **and** the same `abstract_type`
  are merged: sources are combined, confidence takes the higher value.
- Props with the same name but **different** types are added as
  separate entries (this creates conflicts).

### 3. Event Merging

Events with the same `canonical_name` are deduplicated. New events
from other sources are added.

### 4. Conflict Detection

After deduplication, the algorithm scans for props that have different
types across sources. For each conflicting prop:

- A `Conflict` struct is created with:
  - `present_in`: source files where this prop exists
  - `absent_in`: source files where this prop is absent
  - `confidence`: based on how many sources agree (higher agreement = higher confidence)
  - `resolution_suggestion`: `IncludeMajority`, `ScopeToProfile`, or `FlagForHumanReview`

Conflicts are attached to each prop's `conflicts` vector.

### 5. Merge Options

#### Incremental Merge

If `MergeOptions::incremental_base` is provided, the base spec's
components are used as a starting point. Only new or updated components
from the input specs are added or merged.

#### Weighted Merge

If `MergeOptions::weights` is provided, sources with higher weights
are preferred when resolving type conflicts. The prop type from the
highest‑weight source wins.

## Merge Result

The `SynthesisOutput` returned includes:
- Merged, deduplicated components
- `stats.conflicts_detected`: total number of conflicts
- `provenance`: records of which inputs contributed
- `curation_log`: initially `None`, populated after curation

## Test Coverage

Merge behavior is tested in:
- `ucp-synthesizer/src/merge.rs` (unit tests for deduplication, conflict detection)
- `ucp-synthesizer/tests/incremental_merge.rs` (incremental and weighted merge)
- `ucp-synthesizer/tests/proptest_conflicts.rs` (property‑based conflict testing)
