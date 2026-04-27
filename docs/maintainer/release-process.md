# Release Process

Checklist for publishing a new UCP release.

## Pre‑release

1. Ensure all tests pass: `just test`
2. Ensure linting passes: `just lint && just fmt-check`
3. Check that documentation builds: `just doc` (if mdBook is configured)
4. Review open issues and PRs for the milestone

## Version Bump

Update the version in all `Cargo.toml` files. The workspace version is
defined in the root `Cargo.toml` and inherited by member crates:

``` bash
# Set new version
NEW_VERSION="X.Y.Z"

# Update root workspace
sed -i '' "s/version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Member crates (if they override workspace version)
for crate in ucp-core ucp-synthesizer ucp-maintainer ucp-cli; do
    sed -i '' "s/version = \".*\"/version = \"$NEW_VERSION\"/" "$crate/Cargo.toml"
done
```

## Verification

``` bash
cargo check --all-targets
just test
```

## Tag and Push

``` bash
git add -A
git commit -m "chore: bump version to v$NEW_VERSION"
git tag -a "v$NEW_VERSION" -m "UCP v$NEW_VERSION"
git push origin main --tags
```

## Publish Crates

Publish in dependency order:

``` bash
cargo publish -p ucp-core
cargo publish -p ucp-synthesizer
cargo publish -p ucp-maintainer
cargo publish -p ucp-cli
```

## GitHub Release

1. Go to [GitHub Releases](https://github.com/elcoosp/ucp/releases)
2. Create a new release from the tag
3. Write release notes summarizing changes since the last version
4. Publish

## Post‑release

- Update Homebrew formula if applicable
- Close the milestone
- Open the next milestone
