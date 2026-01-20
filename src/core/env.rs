use std::path::PathBuf;

pub fn detect_platform() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    
    match (os, arch) {
        ("linux", "x86_64") => "linuxx64",
        ("linux", "aarch64") => "linuxarm64",
        ("macos", "x86_64") => "darwinx64",
        ("macos", "aarch64") => "darwinarm64",
        ("windows", "x86_64") => "windowsx64",
        _ => panic!("Unsupported platform: {} {}", os, arch),
    }
    .to_string()
}

pub fn sdkman_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".sdkman")
}

pub fn candidates_dir() -> PathBuf {
    sdkman_dir().join("candidates")
}

pub fn candidate_dir(candidate: &str, version: &str) -> PathBuf {
    candidates_dir().join(candidate).join(version)
}

pub fn candidate_current(candidate: &str) -> PathBuf {
    candidates_dir().join(candidate).join("current")
}

pub fn is_installed(candidate: &str, version: &str) -> bool {
    candidate_dir(candidate, version).exists()
}

pub fn get_installed_versions(candidate: &str) -> Vec<String> {
    let base = candidates_dir().join(candidate);
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
    let current = candidate_current(candidate);
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

pub fn set_current_version(candidate: &str, version: &str) -> std::io::Result<()> {
    let current = candidate_current(candidate);
    let target = candidate_dir(candidate, version);
    
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
