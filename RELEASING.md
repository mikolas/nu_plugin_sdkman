# Release Process

## Prerequisites

- Write access to the repository
- All tests passing (when test suite is implemented)
- CHANGELOG.md updated with changes

## Creating a New Release

1. **Update version in Cargo.toml:**
   ```toml
   version = "0.1.0"
   ```

2. **Update CHANGELOG.md:**
   - Add release date to `[Unreleased]` section
   - Create new `[Unreleased]` section for future changes

3. **Commit the version bump:**
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Release v0.1.0"
   git push
   ```

4. **Create and push a tag:**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

5. **GitHub Actions will automatically:**
   - Build binaries for all platforms (Linux, macOS, Windows)
   - Create a GitHub release
   - Attach binaries to the release
   - Generate release notes from commits

## Release Artifacts

The CI builds and uploads these binaries:

- `nu_plugin_sdkman-x86_64-unknown-linux-gnu` (Linux x86_64)
- `nu_plugin_sdkman-aarch64-unknown-linux-gnu` (Linux ARM64)
- `nu_plugin_sdkman-x86_64-apple-darwin` (macOS Intel)
- `nu_plugin_sdkman-aarch64-apple-darwin` (macOS Apple Silicon)
- `nu_plugin_sdkman-x86_64-pc-windows-msvc.exe` (Windows x86_64)

## Testing a Release

After the release is published:

1. **Download and test a binary:**
   ```bash
   # Download for your platform
   wget https://github.com/mikolas/nu_plugin_sdkman/releases/download/v0.1.0/nu_plugin_sdkman-x86_64-unknown-linux-gnu
   chmod +x nu_plugin_sdkman-x86_64-unknown-linux-gnu
   
   # Register with Nushell
   plugin add ./nu_plugin_sdkman-x86_64-unknown-linux-gnu
   ```

2. **Verify basic functionality:**
   ```nushell
   sdk version
   sdk list
   sdk list java
   ```

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes to plugin interface
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

Current status: Pre-1.0 (0.x.x) - API may change

## Troubleshooting

### Build Fails

- Check [GitHub Actions logs](https://github.com/mikolas/nu_plugin_sdkman/actions)
- Verify all dependencies compile for all targets
- Test cross-compilation locally:
  ```bash
  cargo build --release --target x86_64-unknown-linux-gnu
  ```

### Release Not Created

- Ensure tag follows format `vX.Y.Z` (with 'v' prefix)
- Check that GitHub Actions workflow has write permissions
- Verify the tag was pushed: `git ls-remote --tags origin`

### Binary Doesn't Work

- Check Nushell version compatibility (requires 0.110.0+)
- Verify binary is executable: `chmod +x nu_plugin_sdkman-*`
- Check plugin registration: `plugin list | where name =~ sdkman`

## Post-Release

1. Announce on relevant channels (if applicable)
2. Monitor GitHub issues for bug reports
3. Update documentation if needed
