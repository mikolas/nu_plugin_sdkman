# Tasks and Technical Debt

## High Priority (Before v1.0)

### Security
- [ ] **Path traversal validation** - Validate candidate/version names don't contain `..`, `/`, `\`
  - Location: `src/core/env.rs`, `src/commands/*.rs`
  - Risk: HIGH - Could write files outside `.sdkman` directory
  
- [ ] **Input sanitization** - Validate all user input before filesystem operations
  - Location: All command handlers
  - Risk: HIGH - Security vulnerability

### Code Quality
- [ ] **Fix compiler warnings**
  - Unused import: `IntoPipelineData` in `src/commands/aliases.rs`
  - Unused imports: `Deserialize`, `Serialize` in `src/core/api.rs`
  - Unused fields: `vendor`, `default` in `VersionInfo` struct
  - Effort: 5 minutes
  
- [ ] **Custom error types** - Replace `Box<dyn Error>` with custom enum
  - Location: Create `src/error.rs`
  - Benefits: Better error handling, type safety
  - Effort: 2-3 hours

- [ ] **Move hardcoded strings to constants**
  - Create `src/constants.rs`
  - Extract: `.sdkman`, `current`, `candidates`, `.sdkmanrc`, `SDKMAN_DIR`, `.version`
  - Effort: 30 minutes

### Observability
- [ ] **Add logging/tracing** - Use `tracing` crate for observability
  - Location: All major operations
  - Benefits: Debugging, monitoring
  - Effort: 2-3 hours

## Medium Priority

### Testing
- [ ] **True plugin protocol integration tests**
  - Test actual command execution through Nushell
  - Currently only testing core logic, not plugin interface
  - Effort: 4-6 hours

- [ ] **Test malformed/malicious archives**
  - Path traversal in archive entries
  - Corrupted archives
  - Extremely large files
  - Effort: 2 hours

- [ ] **API mocking for network tests**
  - Use `mockito` or similar
  - Test network failures, timeouts
  - Effort: 3-4 hours

### Features
- [ ] **Complete offline mode implementation**
  - Currently just a stub in `src/commands/offline.rs`
  - Need to cache API responses
  - Effort: 4-6 hours

- [ ] **Config file management**
  - `config` command just opens editor
  - Need to read/write config values
  - Effort: 2-3 hours

- [ ] **Add `broadcast` command**
  - Show SDKMAN broadcast messages
  - Effort: 1-2 hours

- [ ] **Add `selfupdate` command**
  - Update plugin binary
  - Effort: 3-4 hours

### Refactoring
- [ ] **Module visibility cleanup**
  - Made `core`, `utils`, `commands` public for tests
  - Better: Use `#[cfg(test)]` or integration test patterns
  - Effort: 1 hour

- [ ] **Extract testable functions**
  - More functions should be extracted for unit testing
  - Reduce need for integration tests
  - Effort: Ongoing

## Low Priority

### Testing
- [ ] **Property-based testing** - Use `proptest` for version parsing
  - Effort: 2-3 hours
  
- [ ] **Benchmarks** - Measure archive extraction performance
  - Effort: 1-2 hours

### Features
- [ ] **Configurable API base URL** - For testing or mirrors
  - Currently hardcoded `https://api.sdkman.io/2`
  - Effort: 30 minutes

- [ ] **Progress indicators** - For downloads and extraction
  - Effort: 2-3 hours

- [ ] **Parallel downloads** - Install multiple versions concurrently
  - Effort: 3-4 hours

## Completed ✅

- [x] Implement top 5 critical tests (22 tests total)
- [x] Fix test isolation (tempfile, SDKMAN_DIR override)
- [x] Add Windows zip extraction support
- [x] Fix symlink creation (parent directory)
- [x] Add version existence validation
- [x] Update documentation (README, RELEASING)
- [x] Fix installation scripts

## Won't Do

- ~~Tab completions for plugin commands~~ - Not supported by Nushell plugin protocol
- ~~Candidates command with version info~~ - N+1 API calls, too slow

## Notes

### Technical Debt Tracking
- Current compiler warnings: 3
- Security issues: 2 (path traversal, input validation)
- Test coverage: 100% of implemented features, but missing edge cases
- Documentation: Good, but needs API docs

### Prioritization Criteria
1. Security issues - Always highest priority
2. Bugs affecting users - High priority
3. Code quality issues - Medium priority
4. New features - Low priority until v1.0

### Before Each Release
1. Run `cargo test` - All tests must pass
2. Run `cargo clippy` - No warnings
3. Update CHANGELOG.md
4. Update version in Cargo.toml
5. Test installation on all platforms
6. Update README if needed
