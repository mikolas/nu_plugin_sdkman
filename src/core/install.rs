use std::error::Error;
use std::path::Path;
use crate::core::{api, env};
use crate::utils::{download, archive};

pub fn install_candidate(candidate: &str, version: &str, platform: &str) -> Result<(), Box<dyn Error>> {
    let download_url = api::get_download_url(candidate, version, platform);
    
    // Create a unique temp directory for this installation
    let temp_base = std::env::temp_dir().join(format!("sdkman-install-{}-{}", candidate, version));
    std::fs::create_dir_all(&temp_base)?;
    
    let archive_name = format!("{}-{}{}", candidate, version, if cfg!(windows) { ".zip" } else { ".tar.gz" });
    let archive_path = temp_base.join(&archive_name);
    
    // Download
    download::download_file(&download_url, &archive_path)?;
    
    // Extract to a 'source' subdirectory in temp
    let extract_dir = temp_base.join("source");
    archive::extract(&archive_path, &extract_dir)?;
    
    // Determine final installation path
    let install_dir = env::candidate_dir(candidate, version)?;
    
    // Move and normalize
    move_and_normalize(&extract_dir, &install_dir)?;
    
    // Cleanup
    std::fs::remove_dir_all(&temp_base).ok();
    
    Ok(())
}

pub fn install_local(candidate: &str, version: &str, local_path: &Path) -> Result<(), Box<dyn Error>> {
    if !local_path.exists() {
        return Err(format!("Local file not found: {}", local_path.display()).into());
    }
    
    let temp_base = std::env::temp_dir().join(format!("sdkman-local-{}-{}", candidate, version));
    std::fs::create_dir_all(&temp_base)?;
    
    let extract_dir = temp_base.join("source");
    archive::extract(local_path, &extract_dir)?;
    
    let install_dir = env::candidate_dir(candidate, version)?;
    
    move_and_normalize(&extract_dir, &install_dir)?;
    
    std::fs::remove_dir_all(&temp_base).ok();
    
    Ok(())
}

fn move_and_normalize(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    // Check if source contains a single directory
    let entries: Vec<_> = std::fs::read_dir(source)?
        .filter_map(|e| e.ok())
        .collect();
        
    let final_source = if entries.len() == 1 && entries[0].path().is_dir() {
        entries[0].path()
    } else {
        source.to_path_buf()
    };
    
    if destination.exists() {
        std::fs::remove_dir_all(destination)?;
    }
    
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Try atomic rename first, fallback to recursive copy
    std::fs::rename(&final_source, destination).or_else(|_| {
        copy_dir_recursive(&final_source, destination)?;
        std::fs::remove_dir_all(&final_source)?;
        Ok(())
    })
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
