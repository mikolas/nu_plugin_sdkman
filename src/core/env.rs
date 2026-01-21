use std::path::PathBuf;

/// Detects the current platform and returns the SDKMAN API platform identifier.
///
/// # Returns
/// Platform string in SDKMAN format (e.g., "linuxx64", "darwinarm64")
///
/// # Errors
/// Returns error if the OS/architecture combination is not supported
pub fn detect_platform() -> Result<String, Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    
    match (os, arch) {
        ("linux", "x86_64") => Ok("linuxx64".to_string()),
        ("linux", "aarch64") => Ok("linuxarm64".to_string()),
        ("macos", "x86_64") => Ok("darwinx64".to_string()),
        ("macos", "aarch64") => Ok("darwinarm64".to_string()),
        ("windows", "x86_64") => Ok("windowsx64".to_string()),
        _ => Err(format!("Unsupported platform: {} {}", os, arch).into()),
    }
}

/// Returns the SDKMAN installation directory path.
///
/// Checks `SDKMAN_DIR` environment variable first (used for test isolation),
/// then falls back to `~/.sdkman`.
///
/// # Returns
/// Path to SDKMAN directory
///
/// # Errors
/// Returns error if home directory cannot be determined
pub fn sdkman_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Check for test override first - allows tests to use temp directories
    // without touching the user's actual ~/.sdkman installation
    if let Ok(dir) = std::env::var("SDKMAN_DIR") {
        return Ok(PathBuf::from(dir));
    }
    
    // Normal behavior: use ~/.sdkman
    dirs::home_dir()
        .map(|p| p.join(".sdkman"))
        .ok_or_else(|| "Could not find home directory".into())
}

/// Returns the candidates directory path (`~/.sdkman/candidates`).
pub fn candidates_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    sdkman_dir().map(|p| p.join("candidates"))
}

/// Returns the installation directory for a specific candidate version.
///
/// # Arguments
/// * `candidate` - Candidate name (e.g., "java")
/// * `version` - Version identifier (e.g., "17.0.9-oracle")
pub fn candidate_dir(candidate: &str, version: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    candidates_dir().map(|p| p.join(candidate).join(version))
}

/// Returns the "current" symlink/marker path for a candidate.
///
/// On Unix: This is a symlink pointing to the active version directory.
/// On Windows: This is a directory containing a `.version` file.
pub fn candidate_current(candidate: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    candidates_dir().map(|p| p.join(candidate).join("current"))
}

/// Checks if a specific candidate version is installed.
pub fn is_installed(candidate: &str, version: &str) -> bool {
    candidate_dir(candidate, version).map(|p| p.exists()).unwrap_or(false)
}

/// Returns a list of all installed versions for a candidate.
///
/// # Arguments
/// * `candidate` - Candidate name (e.g., "java")
///
/// # Returns
/// Vector of version strings, or empty vector if none installed
pub fn get_installed_versions(candidate: &str) -> Vec<String> {
    let base = match candidates_dir() {
        Ok(dir) => dir.join(candidate),
        Err(_) => return vec![],
    };

    if !base.exists() {
        return vec![];
    }
    
    std::fs::read_dir(&base)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter(|e| e.file_name() != "current")
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default()
}

/// Gets the currently active version for a candidate.
///
/// On Unix: Reads the symlink target and extracts the version directory name.
/// On Windows: Reads the `.version` file content from the current directory.
///
/// # Arguments
/// * `candidate` - Candidate name (e.g., "java")
///
/// # Returns
/// Some(version) if a version is set, None otherwise
pub fn get_current_version(candidate: &str) -> Option<String> {
    let current = candidate_current(candidate).ok()?;
    if !current.exists() {
        return None;
    }
    
    #[cfg(unix)]
    {
        // On Unix: read the symlink and extract the version directory name
        std::fs::read_link(&current)
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
    }
    
    #[cfg(windows)]
    {
        // On Windows: read the .version marker file
        // We use a marker file instead of symlinks because Windows symlinks
        // require admin privileges or developer mode
        current.join(".version")
            .exists()
            .then(|| std::fs::read_to_string(current.join(".version")).ok())
            .flatten()
            .map(|s| s.trim().to_string())
    }
}

/// Sets the current version for a candidate.
///
/// On Unix: Creates a symlink from `current` to the version directory.
/// On Windows: Creates a `current` directory with a `.version` marker file.
///
/// # Arguments
/// * `candidate` - Candidate name (e.g., "java")
/// * `version` - Version to set as current
///
/// # Errors
/// Returns error if:
/// - Target version is not installed
/// - Parent directory cannot be created
/// - Symlink/marker creation fails
pub fn set_current_version(candidate: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current = candidate_current(candidate)?;
    let target = candidate_dir(candidate, version)?;
    
    // Check if target version exists
    if !target.exists() {
        return Err(format!("{} {} is not installed", candidate, version).into());
    }
    
    // Ensure parent directory exists
    if let Some(parent) = current.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    if current.exists() {
        std::fs::remove_dir_all(&current)?;
    }
    
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&target, &current)?;
    }
    
    #[cfg(windows)]
    {
        std::fs::create_dir_all(&current)?;
        std::fs::write(current.join(".version"), version)?;
    }
    
    Ok(())
}

/// Checks if current directory has a local SDKMAN environment.
pub fn is_local_env() -> bool {
    std::env::var("PWD")
        .ok()
        .and_then(|p| std::path::PathBuf::from(p).canonicalize().ok())
        .or_else(|| std::env::current_dir().ok())
        .map(|p| p.join(".sdkman").exists())
        .unwrap_or(false)
}

/// Returns the local SDKMAN directory path if it exists.
pub fn local_sdkman_dir() -> Option<PathBuf> {
    std::env::var("PWD")
        .ok()
        .and_then(|p| std::path::PathBuf::from(p).canonicalize().ok())
        .or_else(|| std::env::current_dir().ok())
        .map(|p| p.join(".sdkman"))
        .filter(|p| p.exists())
}

/// Sets the current version in a local environment.
///
/// Creates a symlink in `.sdkman/candidates/<candidate>/current` that points
/// to the global installation at `~/.sdkman/candidates/<candidate>/<version>`.
///
/// # Arguments
/// * `candidate` - Candidate name (e.g., "java")
/// * `version` - Version to set as current
///
/// # Errors
/// Returns error if:
/// - Local .sdkman directory doesn't exist
/// - Target version is not installed globally
/// - Symlink creation fails
pub fn set_local_current_version(candidate: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let local_dir = local_sdkman_dir()
        .ok_or("No local .sdkman directory found")?;
    
    // Target points to global installation
    let global_target = candidate_dir(candidate, version)?;
    if !global_target.exists() {
        return Err(format!("{} {} is not installed", candidate, version).into());
    }
    
    // Current symlink is in local directory
    let local_current = local_dir.join("candidates").join(candidate).join("current");
    
    // Ensure parent directory exists
    if let Some(parent) = local_current.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    if local_current.exists() {
        std::fs::remove_dir_all(&local_current)?;
    }
    
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&global_target, &local_current)?;
    }
    
    #[cfg(windows)]
    {
        std::fs::create_dir_all(&local_current)?;
        std::fs::write(local_current.join(".version"), version)?;
    }
    
    Ok(())
}
