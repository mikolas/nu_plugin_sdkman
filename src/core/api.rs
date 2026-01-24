use std::error::Error;

const API_BASE: &str = "https://api.sdkman.io/2";

/// Information about a candidate version from the SDKMAN API.
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: String,
    pub vendor: String,
    pub default: bool,
}

/// Fetches the list of all available candidates from the SDKMAN API.
///
/// # Returns
/// Vector of candidate names (e.g., ["java", "gradle", "maven"])
///
/// # Errors
/// Returns error if network request fails or response cannot be parsed
pub fn get_candidates() -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/candidates/all", API_BASE);
    let response = reqwest::blocking::get(&url)?;
    let text = response.text()?;
    let candidates: Vec<String> = text.split(',').map(|s| s.trim().to_string()).collect();
    Ok(candidates)
}

/// Fetches the formatted candidates list from the SDKMAN API.
///
/// Returns pre-formatted text suitable for display to users.
///
/// # Errors
/// Returns error if network request fails
pub fn get_candidates_list() -> Result<String, Box<dyn Error>> {
    let url = format!("{}/candidates/list", API_BASE);
    let response = reqwest::blocking::get(&url)?;
    Ok(response.text()?)
}

/// Fetches the formatted versions list for a candidate from the SDKMAN API.
///
/// Returns pre-formatted text with version table suitable for display.
///
/// # Arguments
/// * `candidate` - Candidate name (e.g., "java")
/// * `platform` - Platform identifier (e.g., "linuxx64")
/// * `current` - Currently active version (for highlighting)
/// * `installed` - Comma-separated list of installed versions
///
/// # Errors
/// Returns error if network request fails
pub fn get_versions_list(candidate: &str, platform: &str, current: &str, installed: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "{}/candidates/{}/{}/versions/list?current={}&installed={}",
        API_BASE, candidate, platform, current, installed
    );
    let response = reqwest::blocking::get(&url)?;
    Ok(response.text()?)
}

/// Parses version information from comma-separated text.
///
/// This function implements a simple parser that:
/// 1. Splits on commas
/// 2. Extracts vendor from version string (text after first hyphen)
/// 3. Creates VersionInfo structs
///
/// # Arguments
/// * `text` - Comma-separated version strings (e.g., "17.0.9-oracle,21.0.1-tem")
///
/// # Returns
/// Vector of VersionInfo structs
pub fn parse_versions_text(text: &str) -> Vec<VersionInfo> {
    text.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|version| {
            // Extract vendor from version string (e.g., "17.0.9-oracle" -> "oracle")
            let parts: Vec<&str> = version.split('-').collect();
            let vendor = if parts.len() > 1 {
                parts[1..].join("-")
            } else {
                String::new()
            };
            
            VersionInfo {
                version: version.to_string(),
                vendor,
                default: false,
            }
        })
        .collect()
}

/// Fetches all available versions for a candidate.
///
/// # Arguments
/// * `candidate` - Candidate name (e.g., "java")
/// * `platform` - Platform identifier (e.g., "linuxx64")
///
/// # Returns
/// Vector of VersionInfo structs
///
/// # Errors
/// Returns error if network request fails or response cannot be parsed
pub fn get_versions(candidate: &str, platform: &str) -> Result<Vec<VersionInfo>, Box<dyn Error>> {
    let url = format!("{}/candidates/{}/{}/versions/all", API_BASE, candidate, platform);
    let response = reqwest::blocking::get(&url)?;
    let text = response.text()?;
    Ok(parse_versions_text(&text))
}

/// Gets the default (latest) version for a candidate.
///
/// # Arguments
/// * `candidate` - Candidate name (e.g., "java")
/// * `platform` - Platform identifier (e.g., "linuxx64")
///
/// # Returns
/// Version string of the default version
///
/// # Errors
/// Returns error if network request fails or no versions are available
pub fn get_default_version(candidate: &str, platform: &str) -> Result<String, Box<dyn Error>> {
    let versions = get_versions(candidate, platform)?;
    versions
        .first()
        .map(|v| v.version.clone())
        .ok_or_else(|| "No versions found".into())
}

/// Constructs the download URL for a candidate version.
///
/// # Arguments
/// * `candidate` - Candidate name (e.g., "java")
/// * `version` - Version identifier (e.g., "17.0.9-oracle")
/// * `platform` - Platform identifier (e.g., "linuxx64")
pub fn get_download_url(candidate: &str, version: &str, platform: &str) -> String {
    format!("{}/broker/download/{}/{}/{}", API_BASE, candidate, version, platform)
}

/// Validates that a candidate exists in the SDKMAN registry.
///
/// # Arguments
/// * `candidate` - Candidate name to validate
///
/// # Errors
/// Returns error if candidate is not found or network request fails
pub fn validate_candidate(candidate: &str) -> Result<(), Box<dyn Error>> {
    let candidates = get_candidates()?;
    if !candidates.contains(&candidate.to_string()) {
        return Err(format!("Unknown candidate: {}", candidate).into());
    }
    Ok(())
}
