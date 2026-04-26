# CAM to Open UI Specification Mapping

| CAM Field | Open UI Field | Notes |
|-----------|---------------|-------|
| `id` | component identifier | Direct mapping |
| `ExtractedPart` | anatomy parts | Each part maps to an anatomy element |
| `StateMachine` | states & behaviors | States map to enumerated states; transitions map to behaviors |
| `CanonicalAbstractEvent` | events / behaviors | Event dispatching maps to behavior descriptions |
| `CanonicalAbstractProp` | properties | Typed props with defaults |
| `concrete_type` | property type | Uses enum values for variants |
