//! Canonical Abstract Model — unified representation of UI components
//! across frameworks (React, Leptos, GPUI, etc.).

mod fingerprint;
mod props;
mod state_machine;
mod types;

// Re-export everything at the `cam` level so existing imports like
// `ucp_core::cam::CanonicalAbstractComponent` continue to work.
pub use fingerprint::SemanticFingerprint;
pub use props::{AbstractPropType, AbstractReactivity, Conflict, ResolutionStrategy};
pub use state_machine::{StateMachine, StateNode, Transition};
pub use types::PackageManifest;
pub use types::{
    CanonicalAbstractComponent, CanonicalAbstractEvent, CanonicalAbstractProp, ExtractedPart,
    PropSourceMapping, SourceAttribution,
};
