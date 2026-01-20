# SDKMAN! Nushell Plugin

A native Nushell plugin for SDKMAN! written in Rust, using the Nushell 0.110.0 plugin protocol.

## Features

- Native Nushell plugin (binary, not scripts)
- Full SDKMAN! API integration
- Feature parity with bash SDKMAN (24 commands + aliases)
- Pre-formatted text output matching bash SDKMAN
- Cross-platform support (Linux, macOS, Windows)
- Fast and efficient Rust implementation

## Building

### Prerequisites

- Rust toolchain (1.70+)
- Nushell 0.110.0

### Build

```bash
cargo build --release
```

Or use the build script:

```nushell
nu build.nu
```

## Installation

### Quick Install (Recommended)

**Using bash:**
```bash
curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/nu_plugin_sdkman/main/install.sh | bash
```

**Using Nushell:**
```nushell
http get https://raw.githubusercontent.com/YOUR_USERNAME/nu_plugin_sdkman/main/install.nu | save install.nu
nu install.nu
```

### Manual Installation

#### From Pre-built Binaries

1. Download the binary for your platform from [Releases](https://github.com/YOUR_USERNAME/nu_plugin_sdkman/releases/latest):
   - Linux x86_64: `nu_plugin_sdkman-linux-x86_64`
   - Linux ARM64: `nu_plugin_sdkman-linux-aarch64`
   - macOS x86_64: `nu_plugin_sdkman-darwin-x86_64`
   - macOS ARM64: `nu_plugin_sdkman-darwin-aarch64`
   - Windows: `nu_plugin_sdkman-windows-x86_64.exe`

2. Make it executable (Unix):
   ```bash
   chmod +x nu_plugin_sdkman-*
   ```

3. Move to a directory in your PATH:
   ```bash
   mv nu_plugin_sdkman-* ~/.local/bin/nu_plugin_sdkman
   ```

4. Register with Nushell:
   ```nushell
   plugin add ~/.local/bin/nu_plugin_sdkman
   ```

5. Restart Nushell

#### From Source

**Prerequisites:**
- Rust toolchain (1.70+)
- Nushell 0.110.0+

**Build and install:**
```bash
git clone https://github.com/YOUR_USERNAME/nu_plugin_sdkman.git
cd nu_plugin_sdkman
cargo build --release
plugin add ./target/release/nu_plugin_sdkman
```

Then restart Nushell.

## Usage

### Help

```nushell
sdk                         # Show help and available commands
sdk --help                  # Show help
sdk list --help             # Show help for list command
```

### List Candidates

```nushell
sdk list                    # List all candidates with descriptions
sdk ls                      # Alias for list
sdk list | less             # Paged view
sdk list java               # List Java versions grouped by vendor
sdk list java | less        # Paged view
```

Output is pre-formatted text from SDKMAN API, matching the bash version.

### Install SDKs

```nushell
sdk install java            # Install latest Java
sdk i java 17               # Alias: install Java 17
sdk install java 17 --local /path/to/java.tar.gz  # Install from local archive
```

### Use/Switch Versions

```nushell
sdk use java 17             # Switch to Java 17
sdk u java 17               # Alias for use
sdk default java 17         # Set Java 17 as default
sdk d java 17               # Alias for default
```

### Check Current Versions

```nushell
sdk current                 # Show all current versions
sdk c                       # Alias for current
sdk current java            # Show current Java version
```

### Home Directory

```nushell
sdk home java 17            # Print Java 17 home directory path
sdk h java 17               # Alias for home
```

### Environment Files (.sdkmanrc)

```nushell
sdk env init                # Create .sdkmanrc in current directory
sdk env install             # Install all SDKs from .sdkmanrc
sdk env                     # Load and use versions from .sdkmanrc
sdk env clear               # Clear environment
```

### Maintenance

```nushell
sdk upgrade                 # Upgrade all to latest
sdk ug java                 # Alias: upgrade Java to latest
sdk update                  # Update local candidate cache
sdk flush                   # Clear all caches
sdk flush tmp               # Clear temp files only
sdk version                 # Show plugin version
sdk v                       # Alias for version
```

### Configuration

```nushell
sdk config                  # Edit configuration file
sdk offline enable          # Enable offline mode
sdk offline disable         # Disable offline mode
```

### Uninstall SDKs

```nushell
sdk uninstall java 17       # Remove Java 17
sdk rm java 17              # Alias for uninstall
```

## Architecture

### Plugin Protocol

Uses Nushell 0.110.0 plugin protocol with MessagePack serialization for efficient communication between Nushell and the plugin binary.

### Commands

All commands are implemented as `PluginCommand` traits:

**Main Commands:**
- `sdk` - Show help and usage
- `sdk list` - List candidates/versions (formatted text)
- `sdk install` - Download and install SDKs (supports --local)
- `sdk uninstall` - Remove installations
- `sdk use` - Set current version
- `sdk default` - Set default version
- `sdk current` - Show current versions
- `sdk upgrade` - Upgrade to latest
- `sdk home` - Print home directory path
- `sdk env` - Manage .sdkmanrc files (init/install/clear)
- `sdk version` - Show plugin version
- `sdk update` - Update candidate cache
- `sdk flush` - Clear caches
- `sdk config` - Edit configuration
- `sdk offline` - Enable/disable offline mode

**Aliases:**
- `ls` → `list`
- `i` → `install`
- `rm` → `uninstall`
- `u` → `use`
- `d` → `default`
- `c` → `current`
- `ug` → `upgrade`
- `h` → `home`
- `v` → `version`

### Data Flow

```
Nushell → Plugin Protocol → Command Handler → API/Filesystem → Response → Nushell
```

List commands return pre-formatted text from SDKMAN API. Other commands return structured data or status messages.

### Directory Structure

```
~/.sdkman/
├── candidates/
│   ├── java/
│   │   ├── 17.0.9/
│   │   ├── 21.0.1/
│   │   └── current → 17.0.9  (symlink on Unix)
│   └── gradle/
│       └── 8.5/
```

## Environment Integration

The plugin manages SDK installations but doesn't automatically modify your shell environment. To use installed SDKs:

### Option 1: Manual PATH

```nushell
$env.PATH = ($env.PATH | prepend ~/.sdkman/candidates/java/current/bin)
```

### Option 2: Nushell Config

Add to your `config.nu`:

```nushell
def --env sdk-env [candidate: string] {
    let current = (sdk current $candidate | get version)
    let bin_dir = $"~/.sdkman/candidates/($candidate)/($current)/bin"
    $env.PATH = ($env.PATH | prepend $bin_dir)
}
```

Then use:

```nushell
sdk-env java
```

## Differences from Bash SDKMAN

1. **No Auto-Environment**: Plugin doesn't modify shell environment automatically
2. **Binary Plugin**: Compiled Rust binary, not shell scripts
3. **Explicit Commands**: All operations are explicit plugin commands
4. **Feature Complete**: All major bash commands implemented (24 commands total)
5. **Aliases Supported**: All bash aliases (i, rm, ls, u, c, ug, d, h, v) work

## Development

### Project Structure

```
src/
├── main.rs              # Plugin entry point
├── lib.rs               # Plugin registration
├── commands/            # Command implementations
│   ├── help.rs
│   ├── list.rs
│   ├── install.rs
│   ├── uninstall.rs
│   ├── use.rs
│   ├── default.rs
│   ├── current.rs
│   ├── upgrade.rs
│   ├── home.rs
│   ├── version.rs
│   ├── env.rs
│   ├── update.rs
│   ├── flush.rs
│   ├── config.rs
│   ├── offline.rs
│   └── aliases.rs
├── core/                # Core functionality
│   ├── api.rs          # SDKMAN API client
│   └── env.rs          # Environment/filesystem
└── utils/               # Utilities
    ├── download.rs     # HTTP downloads
    └── archive.rs      # Archive extraction
```

### Adding Commands

1. Create new file in `src/commands/`
2. Implement `PluginCommand` trait
3. Register in `src/lib.rs`

### Testing

```bash
cargo test
```

## Requirements

- Nushell 0.110.0 or later
- Internet connection for downloading SDKs
- `tar` and `gzip` for Unix systems
- `unzip` for Windows

## License

Apache License 2.0 (same as SDKMAN!)

## Contributing

This is a Nushell plugin implementation of SDKMAN!. For the original project:
https://github.com/sdkman/sdkman-cli
