# UCP Documentation

Welcome to the UCP (Universal Component Protocol) documentation.

UCP is an AI‑native CLI for extracting, merging, curating, and exporting
cross‑framework UI component specifications.

## Quick Links

- **[Quickstart](user/quickstart.md)** – get running in 5 minutes
- **[Installation](user/installation.md)** – how to install
- **[Tutorials](user/tutorial-bootstrap.md)** – step‑by‑step guides
- **[Command Reference](user/commands/index.md)** – full CLI reference
- **[Concepts](user/concepts/cam.md)** – understand the CAM and spec format
- **[Contributing](../CONTRIBUTING.md)** – how to contribute
- **[Maintainer Guide](maintainer/architecture.md)** – for UCP maintainers
- **[ADRs](maintainer/adr/0001-use-cam-for-unified-model.md)** – architecture decisions

## Project Structure

UCP is a Rust workspace with four crates:

| Crate | Purpose |
|-------|---------|
| `ucp-core` | Canonical Abstract Model (CAM), SMDL parser |
| `ucp-synthesizer` | Extraction, unification, merge, code generation, export |
| `ucp-maintainer` | Curation, diff, tokens, verify, registry, watch |
| `ucp-cli` | CLI binary with all subcommands |

## License

MIT — see [LICENSE](https://github.com/elcoosp/ucp/blob/main/LICENSE).
