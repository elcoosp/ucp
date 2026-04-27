# Canonical Abstract Model (CAM)

The CAM is UCP's framework‑agnostic representation of a UI component.
It captures *what* a component is and *what properties/events* it exposes,
without tying to any specific implementation.

## Why a Canonical Model?

Different frameworks describe the same component differently:

| Framework | `disabled` prop type |
|-----------|---------------------|
| React | `boolean` |
| Dioxus | `bool` |
| Leptos | `MaybeSignal<bool>` |
| GPUI | `bool` with builder |

The CAM normalizes these to abstract types:

| Abstract Type | Meaning |
|---------------|---------|
| `ControlFlag` | Boolean toggle |
| `StaticValue(T)` | Immutable value of type T |
| `ControlledValue(T)` | Two‑way bound signal |
| `UncontrolledValue(T)` | Optional initial value |
| `AsyncEventHandler` | Callback / event handler |
| `Renderable` | Slotted content (children) |
| `SpreadAttributes` | Passthrough HTML attributes |

## Related

- [UCP Spec Format](specs.md)
- [Provenance Tracking](provenance.md)
- [Drift Detection](drift.md)
