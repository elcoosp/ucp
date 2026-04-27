# UCP Spec Format

A UCP spec is a JSON file conforming to the `SynthesisOutput` schema.

## Top-level Structure

``` json
{
  "ucp_version": "4.0.0",
  "components": [ ... ],
  "stats": { ... },
  "provenance": [ ... ],
  "curation_log": [ ... ]
}
```

## Component

Each component has:

- `id` – unique identifier (e.g., `rust:button.rs:Button`)
- `semantic_fingerprint` – purpose hash + normalized prop names
- `props` – array of canonical props with abstract types
- `events` – semantic events
- `extracted_state_machine` – optional SMDL state machine
- `extracted_parts` – slot/part definitions
- `source_repos` – where it was extracted from

## Stats

``` json
{
  "files_scanned": 10,
  "files_parsed": 8,
  "components_found": 5,
  "conflicts_detected": 2,
  "llm_enriched": false
}
```

## Related

- [CAM Overview](cam.md)
- [Provenance Tracking](provenance.md)
