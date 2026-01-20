# Release Process

## Creating a New Release

1. **Update version in Cargo.toml:**
   ```toml
   version = "0.2.0"
   ```

2. **Commit the version bump:**
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.2.0"
   git push
   ```

3. **Create and push a tag:**
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. **GitHub Actions will automatically:**
   - Build binaries for all platforms
   - Create a GitHub release
   - Attach binaries to the release
   - Generate release notes

5. **Update install scripts if needed:**
   - Edit `install.sh` and `install.nu` to update `REPO` variable with your GitHub username
   - Commit and push

## Testing a Release

After the release is created:

1. Test the installation script:
   ```bash
   curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/nu_plugin_sdkman/main/install.sh | bash
   ```

2. Verify the plugin works:
   ```nushell
   sdk version
   sdk list
   ```

## Platforms Built

- Linux x86_64
- Linux ARM64 (aarch64)
- macOS x86_64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x86_64

## Troubleshooting

If the build fails:
- Check the GitHub Actions logs
- Ensure all dependencies are properly specified in Cargo.toml
- Test local builds for each target:
  ```bash
  cargo build --release --target x86_64-unknown-linux-gnu
  ```
