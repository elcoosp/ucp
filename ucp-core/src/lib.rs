//! UCP Core – Data structures and validation for the Universal Component Protocol.
//!
//! This crate is the foundation of the UCP workspace. It defines:
//!
//! - The **Canonical Abstract Model (CAM)**: framework-agnostic types
//!   ([`cam::CanonicalAbstractComponent`], [`cam::CanonicalAbstractProp`], ...)
//!   that describe UI components independently of any frontend framework.
//! - The **SMDL parser**: a small DSL ([`smdl`]) for describing component
//!   state machines, parsed with `pest`.
//! - A shared error type ([`UcpError`]) used across the workspace.
//!
//! The crate has no I/O and no async runtime dependencies, so it can be used
//! from any context (CLI, WASM, server).

pub mod cam;
pub mod error;
pub mod naming;
pub mod smdl;
pub mod validation;

pub use error::{Result, UcpError};

/// Version of the UCP specification produced by this workspace.
///
/// Stored in every `SynthesisOutput::ucp_version` and used for
/// forward-compatibility checks when loading specs.
pub const UCP_SPEC_VERSION: &str = "4.0.0";

/// Deterministic FNV-1a 64-bit hash.
///
/// `std::collections::hash_map::DefaultHasher` is *not* guaranteed to be
/// stable across Rust releases, but UCP fingerprints are persisted inside
/// spec files and compared across versions. FNV-1a is trivial to implement
/// and stable by construction, so semantic fingerprints remain valid forever.
#[must_use]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a64_is_deterministic_and_matches_reference_vectors() {
        // Reference vectors from the FNV-1a specification.
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(fnv1a64(b"foobar"), 0x8594_4171_f739_67e8);
        // Deterministic across repeated calls.
        assert_eq!(fnv1a64(b"button"), fnv1a64(b"button"));
    }
}
