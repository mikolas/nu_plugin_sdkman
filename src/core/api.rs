use serde::{Deserialize, Serialize};
use std::error::Error;

const API_BASE: &str = "https://api.sdkman.io/2";

#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: String,
    pub vendor: String,
    pub default: bool,
}

pub fn get_candidates() -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/candidates/all", API_BASE);
    let response = reqwest::blocking::get(&url)?;
    let text = response.text()?;
    let candidates: Vec<String> = text.split(',').map(|s| s.trim().to_string()).collect();
    Ok(candidates)
}

pub fn get_candidates_list() -> Result<String, Box<dyn Error>> {
    let url = format!("{}/candidates/list", API_BASE);
    let response = reqwest::blocking::get(&url)?;
    Ok(response.text()?)
}

pub fn get_versions_list(candidate: &str, platform: &str, current: &str, installed: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "{}/candidates/{}/{}/versions/list?current={}&installed={}",
        API_BASE, candidate, platform, current, installed
    );
    let response = reqwest::blocking::get(&url)?;
    Ok(response.text()?)
}

pub fn get_versions(candidate: &str, platform: &str) -> Result<Vec<VersionInfo>, Box<dyn Error>> {
    let url = format!("{}/candidates/{}/{}/versions/all", API_BASE, candidate, platform);
    let response = reqwest::blocking::get(&url)?;
    let text = response.text()?;
    
    let versions: Vec<VersionInfo> = text
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|version| {
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
        .collect();
    
    Ok(versions)
}

pub fn get_default_version(candidate: &str, platform: &str) -> Result<String, Box<dyn Error>> {
    let versions = get_versions(candidate, platform)?;
    versions
        .first()
        .map(|v| v.version.clone())
        .ok_or_else(|| "No versions found".into())
}

pub fn get_download_url(candidate: &str, version: &str, platform: &str) -> String {
    format!("{}/broker/download/{}/{}/{}", API_BASE, candidate, version, platform)
}

pub fn validate_candidate(candidate: &str) -> Result<(), Box<dyn Error>> {
    let candidates = get_candidates()?;
    if !candidates.contains(&candidate.to_string()) {
        return Err(format!("Unknown candidate: {}", candidate).into());
    }
    Ok(())
}
