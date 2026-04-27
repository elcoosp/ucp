# UCP Documentation

[Introduction](index.md)

# User Guide

- [Quickstart](user/quickstart.md)
- [Installation](user/installation.md)

## Tutorials

- [Bootstrap a Library](user/tutorial-bootstrap.md)
- [Merge Two Specs](user/tutorial-merge.md)
- [Curate Conflicts](user/tutorial-curate.md)
- [Export to All Formats](user/tutorial-export.md)

## How-to Guides

- [Integrate in CI/CD](user/howto-ci.md)
- [Continuous Watch Mode](user/howto-watch.md)
- [Merge Design Tokens](user/howto-tokens.md)

## Concepts

- [Canonical Abstract Model](user/concepts/cam.md)
- [UCP Spec Format](user/concepts/specs.md)
- [Provenance Tracking](user/concepts/provenance.md)
- [Drift Detection](user/concepts/drift.md)

## Command Reference

- [Overview](user/commands/index.md)
- [bootstrap](user/commands/bootstrap.md)
- [merge](user/commands/merge.md)
- [curate](user/commands/curate.md)
- [diff](user/commands/diff.md)
- [verify](user/commands/verify.md)
- [registry](user/commands/registry.md)
- [export-all](user/commands/export-all.md)
- [watch](user/commands/watch.md)

# Maintainer Guide

- [Architecture Overview](maintainer/architecture.md)
- [Extraction Pipeline](maintainer/pipeline.md)
- [Merge Algorithm](maintainer/merge-algorithm.md)
- [Adding an Extractor](maintainer/adding-extractor.md)
- [Adding an Exporter](maintainer/adding-exporter.md)
- [Release Process](maintainer/release-process.md)
- [Testing Strategy](maintainer/testing.md)
- [CI Setup](maintainer/ci-setup.md)

## Architecture Decision Records

- [ADR-0001: Canonical Abstract Model](maintainer/adr/0001-use-cam-for-unified-model.md)
- [ADR-0002: Semantic Fingerprint](maintainer/adr/0002-use-semantic-fingerprint-for-dedup.md)
- [ADR-0003: Ratatui for Curation](maintainer/adr/0003-use-ratatui-for-curation-tui.md)
- [ADR-0004: SQLite for Registry](maintainer/adr/0004-use-sqlite-for-registry.md)
- [ADR-0005: `notify` for Watch Mode](maintainer/adr/0005-use-notify-for-watch-mode.md)
