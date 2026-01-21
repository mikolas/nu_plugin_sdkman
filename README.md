# SDKMAN! Nushell Plugin

> ⚡ **100% Vibe Coded** - This entire project was created through AI-assisted development using Amazon Q Developer.

A native Nushell plugin for SDKMAN! written in Rust, using the Nushell 0.110.0 plugin protocol.

## Status

✅ **Production Ready** - Core functionality complete with comprehensive test coverage and documentation.

## Features

- Native Nushell plugin (binary, not scripts)
- Full SDKMAN! API integration
- Command coverage matching bash SDKMAN (24 commands + aliases)
- Structured data output for Nushell pipelines
- Cross-platform support (Linux, macOS, Windows)
- Fast and efficient Rust implementation
- Pure Rust archive handling (no external tools required)
- Comprehensive test suite (22 tests, 100% passing)
- Full API documentation

## Building

### Prerequisites

- Rust toolchain (1.70+)
- Nushell 0.110.0+

### Build

```bash
cargo build --release
```

## Installation

### Quick Install (Recommended)

Run the install script:

```bash
# Using bash/zsh
curl -fsSL https://raw.githubusercontent.com/mikolas/nu_plugin_sdkman/master/install.sh | bash

# Or using Nushell
http get https://raw.githubusercontent.com/mikolas/nu_plugin_sdkman/master/install.nu | save install.nu
nu install.nu
```

The installer will:
1. Download the correct binary for your platform
2. Install to `~/.local/bin/nu_plugin_sdkman`
3. Register the plugin with Nushell
4. Create `~/.sdkman/bin/sdkman-init.nu` for PATH setup

Then add these lines to your `~/.config/nushell/config.nu`:

```nushell
# SDKMAN
$env.SDKMAN_DIR = ($env.HOME | path join ".sdkman")
source ~/.sdkman/bin/sdkman-init.nu
```

Restart Nushell and you're ready!

### Manual Installation

#### From Pre-built Binaries

1. Download the binary for your platform from [Releases](https://github.com/mikolas/nu_plugin_sdkman/releases/latest):
   - Linux x86_64: `nu_plugin_sdkman-x86_64-unknown-linux-gnu`
   - Linux ARM64: `nu_plugin_sdkman-aarch64-unknown-linux-gnu`
   - macOS x86_64: `nu_plugin_sdkman-x86_64-apple-darwin`
   - macOS ARM64: `nu_plugin_sdkman-aarch64-apple-darwin`
   - Windows x86_64: `nu_plugin_sdkman-x86_64-pc-windows-msvc.exe`

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

5. Create the init script:
   ```bash
   mkdir -p ~/.sdkman/bin
   cat > ~/.sdkman/bin/sdkman-init.nu << 'EOF'
   # SDKMAN Nushell initialization
   $env.PATH = ($env.PATH | prepend (
       ls ~/.sdkman/candidates/*/current/bin 
       | get name
   ))
   EOF
   ```

6. Add to `~/.config/nushell/config.nu`:
   ```nushell
   # SDKMAN
   $env.SDKMAN_DIR = ($env.HOME | path join ".sdkman")
   source ~/.sdkman/bin/sdkman-init.nu
   ```

7. Restart Nushell

#### From Source

**Prerequisites:**
- Rust toolchain (1.70+)
- Nushell 0.110.0+

**Build and install:**
```bash
git clone https://github.com/mikolas/nu_plugin_sdkman.git
cd nu_plugin_sdkman
cargo build --release
plugin add ./target/release/nu_plugin_sdkman
```

Then follow steps 5-7 from "From Pre-built Binaries" above.

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

Output format:
- `sdk list` (no args) returns pre-formatted text from SDKMAN API
- `sdk list <candidate>` returns structured table with columns: vendor, use, version, dist, status, identifier

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

- `sdk list` (no args) returns pre-formatted text from SDKMAN API
- `sdk list <candidate>` returns structured table with columns: vendor, use, version, dist, status, identifier
- Other commands return structured data or status messages

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

The plugin creates symlinks (Unix) or version markers (Windows) at `~/.sdkman/candidates/<candidate>/current` that point to the active version.

### How It Works

When you run `sdk use java 17` or `sdk default java 17`, the plugin updates the `current` symlink to point to that version. The install script creates `~/.sdkman/bin/sdkman-init.nu` which automatically adds all `current/bin` directories to your PATH.

**In your `config.nu`:**
```nushell
# SDKMAN
$env.SDKMAN_DIR = ($env.HOME | path join ".sdkman")
source ~/.sdkman/bin/sdkman-init.nu
```

**The init script dynamically discovers all installed candidates:**
```nushell
# ~/.sdkman/bin/sdkman-init.nu
$env.PATH = ($env.PATH | prepend (
    ls ~/.sdkman/candidates/*/current/bin 
    | get name
))
```

### Usage

After setup, switching versions is automatic:

```nushell
sdk use java 17        # Updates the 'current' symlink to Java 17
java --version         # Uses Java 17

sdk use java 21        # Updates the 'current' symlink to Java 21
java --version         # Uses Java 21
```

No need to modify PATH or restart Nushell - the symlink/marker handles it dynamically.

## Differences from Bash SDKMAN

1. **PATH Management**: Uses Nushell init script (similar to bash's `sdkman-init.sh`)
2. **Binary Plugin**: Compiled Rust binary, not shell scripts
3. **Explicit Commands**: All operations are explicit plugin commands
4. **Feature Complete**: All major bash commands implemented (24 commands total)
5. **Aliases Supported**: All bash aliases (i, rm, ls, u, c, ug, d, h, v) work
6. **Compatible**: Can coexist with bash SDKMAN (shares same directory structure)

**Note:** Some commands have basic implementations:
- `config` - Opens editor but doesn't manage config file yet
- `offline` - Command exists but offline mode not fully implemented
- `flush` - Clears directories but cache management is basic

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

**Test Coverage:**
- ✅ API version parsing (5 tests)
- ✅ Archive extraction (5 tests - tar.gz and zip)
- ✅ .sdkmanrc parsing (6 tests)
- ✅ Symlink handling (3 tests)
- ✅ Integration tests (3 tests)

**Total: 22/22 tests passing (100%)**

**Test Isolation:**
- All tests use temporary directories
- No tests touch user's home directory
- Environment variable isolation with `#[serial]`

### Documentation

Generate API documentation:
```bash
cargo doc --open
```

All public APIs are documented with:
- Function purpose and behavior
- Arguments and return values
- Error conditions
- Platform-specific notes
- Examples where helpful

## Requirements

- Nushell 0.110.0 or later
- Internet connection for downloading SDKs

## License

Apache License 2.0 (same as SDKMAN!)

## Development Story

This project was **entirely vibe coded** using [Amazon Q Developer](https://aws.amazon.com/q/developer/) - from initial concept to full implementation, including:
- Complete Rust codebase (24 commands + aliases)
- GitHub Actions CI/CD pipeline
- Cross-platform builds (Linux, macOS, Windows)
- Installation scripts
- Documentation

No manual coding was required. The entire development process was conversational, demonstrating the power of AI-assisted development.

## Contributing

This is a Nushell plugin implementation of SDKMAN!. For the original project:
https://github.com/sdkman/sdkman-cli

Issues and pull requests are welcome at:
https://github.com/mikolas/nu_plugin_sdkman
