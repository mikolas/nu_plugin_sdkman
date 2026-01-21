# Coding Rules for nu_plugin_sdkman

## Rust Best Practices (Official Guidelines)

### Common Traits (C-COMMON-TRAITS)
Types should eagerly implement common traits when applicable:
- `Copy` - For types that can be copied bitwise
- `Clone` - For types that can be duplicated
- `Eq`, `PartialEq` - For equality comparisons
- `Ord`, `PartialOrd` - For ordering
- `Hash` - For use in hash maps/sets
- `Default` - For default values
- `Debug` - For debugging (always implement)
- `Display` - For user-facing output
- `Serialize`, `Deserialize` - For data interchange

### Type Conversions (C-CONV-TRAITS)
Implement standard conversion traits:
- `From<T>` / `Into<T>` - Infallible conversions
- `TryFrom<T>` / `TryInto<T>` - Fallible conversions
- `AsRef<T>` / `AsMut<T>` - Cheap reference conversions
- `Deref` / `DerefMut` - Smart pointer behavior

### Builder Pattern (C-BUILDER)
For types with many optional parameters:
```rust
pub struct Config {
    host: String,
    port: u16,
    timeout: Duration,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<Duration>,
}

impl ConfigBuilder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    
    pub fn build(self) -> Result<Config, Box<dyn Error>> {
        // Validate and construct
    }
}
```

### Semantic Versioning (C-SEMVER)
- Breaking changes require major version bump
- New features require minor version bump
- Bug fixes require patch version bump

### Documentation (C-DOCS)
```rust
/// Brief one-line summary.
///
/// More detailed explanation if needed.
///
/// # Arguments
/// * `param` - Description
///
/// # Returns
/// Description of return value
///
/// # Errors
/// When this function returns an error and why
///
/// # Panics
/// When this function panics (if ever)
///
/// # Examples
/// ```
/// let result = function(arg);
/// assert_eq!(result, expected);
/// ```
pub fn function(param: Type) -> Result<ReturnType, Error> {
    // ...
}
```

### Word Order (C-WORD-ORDER)
Maintain consistent word order in names:
- `StrBuf` and `StrBufMut` (not `MutStrBuf`)
- `IpAddr` and `SocketAddr` (not `AddrIp`)

## Code Style

### General Principles
- **Minimal Code:** Write only what's needed, avoid verbose implementations
- **Clarity:** Code should be self-documenting, add comments for "why" not "what"
- **Consistency:** Follow existing patterns in the codebase
- **Rust Idioms:** Use standard Rust patterns and conventions

### Error Handling (Official Guidelines)
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

**Panic vs Result (C-FAILURE):**
- Use `Result` for expected failures (network, I/O, user input)
- Use `panic!` only for programming errors (invariant violations)
- Document panic conditions in doc comments

**Error Types:**
- Consider custom error enums for libraries
- Use `thiserror` for deriving error traits
- Implement `std::error::Error` for custom errors

### Function Size
- Keep functions under 50 lines
- Extract complex logic into helper functions
- One function = one responsibility

### Naming Conventions (RFC 430 + API Guidelines)
- Functions/methods: `snake_case`
- Types/traits: `UpperCamelCase`
- Enum variants: `UpperCamelCase`
- Constants/statics: `SCREAMING_SNAKE_CASE`
- Type parameters: Single uppercase letter `T`, `E`, `K`, `V`
- Lifetimes: Short lowercase `'a`, `'de`, `'src`
- Acronyms: `Uuid` not `UUID`, `is_xid_start` not `is_XID_start`
- Be descriptive: `installed_versions()` not `get_vers()`

**Conversions (C-CONV):**
- `as_` - Cheap reference-to-reference conversion
- `to_` - Expensive conversion (allocates/copies)
- `into_` - Consuming conversion (takes ownership)

**Getters (C-GETTER):**
- No `get_` prefix: `version()` not `get_version()`
- Use `get_` only when there's a matching `set_` or for mutable access

**Constructors:**
- `new()` - Standard constructor
- `with_capacity()` - Constructor with details
- `from_*()` - Conversion constructors

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

### Public APIs (C-DOCS)
```rust
/// Installs a candidate version from the SDKMAN API.
///
/// # Arguments
/// * `candidate` - The candidate name (e.g., "java")
/// * `version` - The version to install (e.g., "17.0.9-oracle")
/// * `platform` - The platform identifier
///
/// # Returns
/// `Ok(())` on successful installation
///
/// # Errors
/// Returns error if:
/// - Download fails (network issues)
/// - Extraction fails (corrupted archive)
/// - Filesystem operations fail (permissions)
///
/// # Examples
/// ```no_run
/// install_candidate("java", "17.0.9-oracle", "linux64")?;
/// ```
pub fn install_candidate(candidate: &str, version: &str, platform: &str) -> Result<(), Box<dyn Error>> {
    // ...
}
```

**Documentation Requirements:**
- All public items must have doc comments
- Include examples for non-trivial functions
- Document all error conditions
- Document panic conditions (if any)
- Use `///` for item docs, `//!` for module docs

### Comments
- Explain "why" not "what"
- Document non-obvious behavior
- Add TODO/FIXME with context and date

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

## References

### Official Rust Guidelines
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - Official naming, design patterns, and best practices
- [RFC 430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md) - Naming conventions
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) - The Rust Book chapter on errors

### Additional Resources
- [Microsoft Rust Guidelines](https://github.com/microsoft/code-with-engineering-playbook/tree/main/docs/rust) - Enterprise Rust practices
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) - Common patterns and anti-patterns

Content was rephrased for compliance with licensing restrictions.

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
