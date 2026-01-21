# Coding Rules for nu_plugin_sdkman

## Code Style

### General Principles
- **Minimal Code:** Write only what's needed, avoid verbose implementations
- **Clarity:** Code should be self-documenting, add comments for "why" not "what"
- **Consistency:** Follow existing patterns in the codebase
- **Rust Idioms:** Use standard Rust patterns and conventions

### Error Handling
```rust
// ✅ DO: Use Result and ? operator
pub fn do_something() -> Result<(), Box<dyn Error>> {
    let value = might_fail()?;
    Ok(())
}

// ❌ DON'T: Use unwrap() in production code
pub fn do_something() {
    let value = might_fail().unwrap(); // Will panic!
}

// ✅ DO: Provide context in errors
return Err(format!("Failed to install {} {}: {}", candidate, version, e).into());

// ❌ DON'T: Generic errors
return Err("Failed".into());
```

### Function Size
- Keep functions under 50 lines
- Extract complex logic into helper functions
- One function = one responsibility

### Naming Conventions
- Functions: `snake_case`
- Types: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Be descriptive: `get_installed_versions()` not `get_vers()`

## Testing Rules

### Test Isolation (CRITICAL)
```rust
// ✅ DO: Always use temp directories
#[test]
fn test_something() {
    let temp = tempdir().unwrap();
    std::env::set_var("SDKMAN_DIR", temp.path().to_str().unwrap());
    
    // ... test code ...
    
    std::env::remove_var("SDKMAN_DIR");
}

// ❌ DON'T: Ever touch home directory
#[test]
fn test_something() {
    let home = dirs::home_dir().unwrap();
    // NEVER DO THIS!
}
```

### Test Attributes
```rust
// ✅ DO: Use #[serial] for tests that modify env vars
#[test]
#[serial]
fn test_with_env_var() {
    std::env::set_var("SDKMAN_DIR", "/tmp/test");
    // ...
}

// ✅ DO: Use #[cfg] for platform-specific tests
#[test]
#[cfg(unix)]
fn test_symlinks() {
    // Unix-only test
}
```

### Test Structure
- **Arrange:** Set up test data
- **Act:** Execute the code under test
- **Assert:** Verify results
- **Cleanup:** Remove env vars, temp files

### Test Naming
- `test_<function>_<scenario>` - e.g., `test_parse_versions_empty`
- Be descriptive: what is being tested and expected outcome

## Security Rules

### Input Validation
```rust
// ✅ DO: Validate all user input
fn validate_candidate_name(name: &str) -> Result<(), Box<dyn Error>> {
    if name.contains("..") || name.contains("/") || name.contains("\\") {
        return Err("Invalid candidate name".into());
    }
    Ok(())
}

// ❌ DON'T: Trust user input
fn install(candidate: &str) {
    let path = format!("~/.sdkman/candidates/{}", candidate);
    // What if candidate is "../../etc/passwd"?
}
```

### Path Handling
- Always use `PathBuf` and `Path` for filesystem operations
- Never concatenate strings to build paths
- Check for path traversal attempts

### API Responses
- Don't trust API responses blindly
- Validate data before using
- Handle unexpected formats gracefully

## Module Organization

### Visibility
- Keep modules private by default
- Only expose what's needed for public API
- Use `pub(crate)` for internal APIs
- Tests should not require making everything public

### Dependencies
- Minimize external dependencies
- Prefer standard library when possible
- Document why each dependency is needed
- No dependencies without justification

## Documentation

### Public APIs
```rust
/// Installs a candidate version from the SDKMAN API.
///
/// # Arguments
/// * `candidate` - The candidate name (e.g., "java")
/// * `version` - The version to install (e.g., "17.0.9-oracle")
/// * `platform` - The platform identifier
///
/// # Errors
/// Returns error if download fails or extraction fails
pub fn install_candidate(candidate: &str, version: &str, platform: &str) -> Result<(), Box<dyn Error>> {
    // ...
}
```

### Comments
- Explain "why" not "what"
- Document non-obvious behavior
- Add TODO/FIXME with context

## Performance

### Avoid Premature Optimization
- Write clear code first
- Optimize only when needed
- Measure before optimizing

### Common Sense
- Don't read entire files into memory if streaming works
- Reuse allocations when possible
- Avoid unnecessary clones

## Git Commit Messages

### Format
```
<type>: <short summary>

<optional detailed description>
```

### Types
- `feat:` - New feature
- `fix:` - Bug fix
- `test:` - Add or modify tests
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

### Examples
```
feat: add support for .sdkmanrc files

Implements env init, env install, and env clear commands
to manage project-specific SDK versions.
```

```
fix: create parent directory before symlink

set_current_version() now ensures the parent directory exists
before creating the symlink, preventing "No such file" errors.
```

## Don'ts (Critical)

### Never
- ❌ Touch `~/.sdkman` in tests
- ❌ Use `unwrap()` in production code without justification
- ❌ Commit code with compiler warnings
- ❌ Add dependencies without discussion
- ❌ Break existing tests
- ❌ Ignore security implications
- ❌ Leave TODO comments without context
- ❌ Copy-paste code (extract to function)

### Avoid
- ⚠️ Long functions (>50 lines)
- ⚠️ Deep nesting (>3 levels)
- ⚠️ Magic numbers (use constants)
- ⚠️ Generic variable names (x, tmp, data)
- ⚠️ Commented-out code (delete it)

## Code Review Checklist

Before committing:
- [ ] All tests pass (`cargo test`)
- [ ] No compiler warnings (`cargo build`)
- [ ] Code follows style guidelines
- [ ] New code has tests
- [ ] Documentation updated if needed
- [ ] No hardcoded paths or values
- [ ] Error messages are clear
- [ ] Security implications considered
