# Installation

## Homebrew (macOS)

``` bash
brew install elcoosp/tap/ucp
```

## Cargo (any platform)

``` bash
cargo install ucp-cli
```

Requirements:
- Rust toolchain (stable 1.80+) — install via [rustup](https://rustup.rs)

## Verify Installation

``` bash
ucp --version
```

Should print the version (e.g., `ucp 0.12.0`).

## Build from Source

``` bash
git clone https://github.com/elcoosp/ucp.git
cd ucp
cargo build --release
./target/release/ucp --version
```
