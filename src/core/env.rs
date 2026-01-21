use std::path::PathBuf;

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

pub fn sdkman_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Check for test override first
    if let Ok(dir) = std::env::var("SDKMAN_DIR") {
        return Ok(PathBuf::from(dir));
    }
    
    // Normal behavior
    dirs::home_dir()
        .map(|p| p.join(".sdkman"))
        .ok_or_else(|| "Could not find home directory".into())
}

pub fn candidates_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    sdkman_dir().map(|p| p.join("candidates"))
}

pub fn candidate_dir(candidate: &str, version: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    candidates_dir().map(|p| p.join(candidate).join(version))
}

pub fn candidate_current(candidate: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    candidates_dir().map(|p| p.join(candidate).join("current"))
}

pub fn is_installed(candidate: &str, version: &str) -> bool {
    candidate_dir(candidate, version).map(|p| p.exists()).unwrap_or(false)
}

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

pub fn get_current_version(candidate: &str) -> Option<String> {
    let current = candidate_current(candidate).ok()?;
    if !current.exists() {
        return None;
    }
    
    #[cfg(unix)]
    {
        std::fs::read_link(&current)
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
    }
    
    #[cfg(windows)]
    {
        current.join(".version")
            .exists()
            .then(|| std::fs::read_to_string(current.join(".version")).ok())
            .flatten()
            .map(|s| s.trim().to_string())
    }
}

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
