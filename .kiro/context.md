# nu_plugin_sdkman Project Context

## Project Overview
Nushell plugin for SDKMAN! written in Rust. Provides native integration with SDKMAN! SDK management through Nushell's plugin protocol.

**Status:** v0.0.5 - Early development, core functionality complete, testing in progress

**Development Method:** 100% vibe coded with Amazon Q Developer (AI-assisted development)

## Architecture

### Module Structure
```
src/
├── commands/     # Plugin command implementations (24 commands + aliases)
├── core/         # Core business logic
│   ├── api.rs    # SDKMAN API client (HTTP requests, parsing)
│   ├── env.rs    # Environment/filesystem operations
│   └── install.rs # Installation logic
└── utils/        # Utilities
    ├── download.rs # HTTP downloads
    └── archive.rs  # tar.gz and zip extraction
```

### Key Components

**Plugin Protocol:**
- Nushell 0.110.0 plugin protocol
- MessagePack serialization
- Each command implements `PluginCommand` trait

**Data Flow:**
```
Nushell → Plugin Protocol → Command Handler → Core Logic → API/Filesystem → Response
```

**Directory Structure:**
```
~/.sdkman/
├── candidates/
│   └── <candidate>/
│       ├── <version>/          # Installed versions
│       └── current/            # Symlink (Unix) or marker (Windows)
```

## Key Design Decisions

### Cross-Platform Strategy
- **Unix:** Symlinks at `candidates/<candidate>/current` → version directory
- **Windows:** Directory with `.version` file containing version string
- **Why:** Symlinks work seamlessly on Unix, Windows needs alternative

### Test Isolation
- **Environment Variable Override:** `SDKMAN_DIR` env var overrides `~/.sdkman`
- **Temp Directories:** All tests use `tempfile::tempdir()`
- **Serial Execution:** Tests modifying env vars use `#[serial]` attribute
- **Why:** Never touch user's actual SDKMAN installation during tests

### Pure Rust Implementation
- **No External Tools:** Uses `flate2`, `tar`, `zip` crates instead of system tools
- **Why:** Cross-platform compatibility, no dependencies on system utilities

### Error Handling
- **Type:** `Result<T, Box<dyn Error>>` throughout
- **Propagation:** Use `?` operator, avoid `unwrap()` in production code
- **User Messages:** Clear, actionable error messages

## Dependencies

### Production
- `nu-plugin`, `nu-protocol` - Nushell plugin framework
- `reqwest` - HTTP client (rustls-tls, no OpenSSL)
- `flate2`, `tar`, `zip` - Archive handling
- `dirs` - Home directory detection
- `serde`, `serde_json` - Serialization

### Development
- `tempfile` - Temporary directories for tests
- `serial_test` - Sequential test execution

## Testing Strategy

### Coverage (22 tests, 100% passing)
- API version parsing (5 tests)
- Archive extraction - tar.gz and zip (5 tests)
- .sdkmanrc parsing (6 tests)
- Symlink/marker handling (3 tests)
- Integration workflows (3 tests)

### Test Principles
1. **Isolation:** Every test uses unique temp directory
2. **No Side Effects:** Never touch `~/.sdkman` or home directory
3. **Cleanup:** Always `remove_var("SDKMAN_DIR")` at test end
4. **Fixtures:** `tests/fixtures/` contains test archives

### What's NOT Tested
- Network failures (API mocking not implemented)
- Path traversal attacks
- Corrupted archives
- True plugin protocol integration (command execution through Nushell)

## Release Process

### Version Tagging
```bash
git tag -a v0.0.X -m "message"
git push --tags
```

### CI/CD
- GitHub Actions builds binaries for 5 platforms
- Artifacts: Linux (x64, ARM64), macOS (x64, ARM64), Windows (x64)
- Triggered on tag push

### Platforms
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

## Known Limitations

### Current Implementation
- `config` command just opens editor (no config management)
- `offline` mode is stubbed (not functional)
- `flush` has basic implementation
- No `broadcast` or `selfupdate` commands

### Security
- No path traversal validation (HIGH PRIORITY)
- No input sanitization for candidate/version names
- Trusts API responses

### Observability
- No logging/tracing
- Silent failures in some code paths
- Hard to debug issues

## Future Considerations

### Before v1.0
1. Path traversal validation
2. Custom error types (not Box<dyn Error>)
3. Logging/tracing infrastructure
4. Complete offline mode
5. Config file management

### Nice to Have
- Property-based testing (proptest)
- Benchmarks
- Plugin protocol integration tests
- API mocking for network tests
