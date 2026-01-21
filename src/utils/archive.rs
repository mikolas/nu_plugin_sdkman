use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tar::Archive;
use zip::ZipArchive;

/// Extracts a tar.gz archive to the destination directory.
///
/// Uses pure Rust implementation (no external tar command required).
///
/// # Arguments
/// * `archive` - Path to the .tar.gz file
/// * `destination` - Directory to extract into
///
/// # Errors
/// Returns error if file cannot be opened, decompressed, or extracted
pub fn extract_tar_gz(archive: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);
    
    std::fs::create_dir_all(destination)?;
    
    archive.unpack(destination)?;
    
    Ok(())
}

/// Extracts a zip archive to the destination directory.
///
/// Uses pure Rust implementation (no external unzip command required).
/// Handles both files and directories within the archive.
///
/// # Arguments
/// * `archive` - Path to the .zip file
/// * `destination` - Directory to extract into
///
/// # Errors
/// Returns error if file cannot be opened, read, or extracted
pub fn extract_zip(archive: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut archive = ZipArchive::new(file)?;
    
    std::fs::create_dir_all(destination)?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let path = file.mangled_name();
        
        let dest = destination.join(path);
        
        if file.is_dir() {
            std::fs::create_dir_all(&dest)?;
        } else {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&dest)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    
    Ok(())
}

/// Extracts an archive based on file extension.
///
/// Automatically detects format and calls appropriate extraction function.
///
/// # Supported Formats
/// - `.gz` - tar.gz archives (Unix/Linux standard)
/// - `.zip` - zip archives (Windows standard)
///
/// # Arguments
/// * `archive` - Path to the archive file
/// * `destination` - Directory to extract into
///
/// # Errors
/// Returns error if format is unsupported or extraction fails
pub fn extract(archive: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let ext = archive.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    match ext {
        "gz" => extract_tar_gz(archive, destination),
        "zip" => extract_zip(archive, destination),
        _ => Err("Unsupported archive format".into()),
    }
}
