//! Framework code generators: turn a [`PackageManifest`] into
//! compilable component-library skeletons for each target framework.

/// Shared helpers (case conversion, type mapping, Cargo.toml template).
pub mod common;
/// Dioxus generator.
pub mod dioxus;
/// GPUI generator.
pub mod gpui;
/// Leptos generator.
pub mod leptos;
/// React/TSX generator.
pub mod react;
/// In-memory component registry used by tests and tooling.
pub mod registry;
/// The [`crate::generate::traits::ComponentGenerator`] trait.
pub mod traits;
/// Web Components generator.
pub mod web_components;
