//! # UCP Maintainer
//!
//! The spec‑maintainer toolkit for the Universal Component Protocol.
//!
//! This crate provides CLI commands and library functions for:
//!
//! - **Interactive merge curation** (`curate` / `curate_tui`)
//! - **Structural spec diff** (`diff`)
//! - **Design token merging** (`tokens`)
//! - **Drift detection** (`verify`) — re‑extract and compare against source
//! - **Local spec registry** (`registry`) — SQLite‑backed spec store
//! - **File‑watch continuous rebuild** (`watch`)
//!
//! All modules are re‑exported at the crate root for use by `ucp‑cli`.

pub mod curate;
pub mod curate_tui;
pub mod diff;
pub mod tokens;
pub mod verify;
pub mod registry;
pub mod watch;
pub mod registry_server;
