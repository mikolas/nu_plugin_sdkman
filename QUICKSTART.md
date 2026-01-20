# SDKMAN! Nushell Plugin - Quick Start

## Installation

1. **Build the plugin:**
   ```bash
   cd sdkman-plugin
   cargo build --release
   ```

2. **Register with Nushell:**
   ```nushell
   plugin add ./target/release/nu_plugin_sdkman
   ```

3. **Restart Nushell** or run:
   ```nushell
   plugin use sdkman
   ```

## Usage

```nushell
# List all candidates
sdk list

# List Java versions
sdk list java

# Install latest Java
sdk install java

# Install specific version
sdk install java 17

# Switch to Java 17
sdk use java 17

# Show current versions
sdk current

# Upgrade to latest
sdk upgrade java

# Uninstall
sdk uninstall java 17
```

## Verification

Check plugin is loaded:
```nushell
plugin list | where name =~ sdkman
```

## Troubleshooting

If commands don't work:
1. Ensure plugin is registered: `plugin list`
2. Restart Nushell
3. Check plugin path is correct
4. Rebuild with `cargo build --release`
