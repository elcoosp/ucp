# UCP Maintainer Guide

This document covers the release process, governance, and maintenance
tasks for UCP.

## Maintainers

- **elcoosp** — project lead, core development

## Release Checklist

1. Ensure all tests pass: `just test`
2. Ensure linting passes: `just lint && just fmt-check`
3. Update version in all `Cargo.toml` files:
   ``` bash
   for crate in . ucp-core ucp-synthesizer ucp-maintainer ucp-cli; do
       sed -i '' 's/version = "X.Y.Z"/version = "NEXT.VER"/' "$crate/Cargo.toml"
   done
   ```
4. Run `cargo check` to verify
5. Commit: `git add -A && git commit -m "chore: bump version to X.Y.Z"`
6. Tag: `git tag -a vX.Y.Z -m "UCP vX.Y.Z"`
7. Push: `git push origin main --tags`
8. Publish crates (in dependency order):
   ``` bash
   cargo publish -p ucp-core
   cargo publish -p ucp-synthesizer
   cargo publish -p ucp-maintainer
   cargo publish -p ucp-cli
   ```
9. Create GitHub Release from the tag, with changelog

## Governance

UCP is a solo-maintainer project. Decisions are documented via ADRs
in `docs/maintainer/adr/`. Major changes should be discussed via issues
before implementation.

## Key Contacts

- GitHub Issues: preferred for bug reports and feature requests
- Email: via GitHub profile
