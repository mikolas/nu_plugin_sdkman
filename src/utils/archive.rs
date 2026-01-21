use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use tar::Archive;
use zip::ZipArchive;

pub fn extract_tar_gz(archive: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);
    
    std::fs::create_dir_all(destination)?;
    
    archive.unpack(destination)?;
    
    Ok(())
}

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

pub fn extract(archive: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    let ext = archive.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    match ext {
        "gz" => extract_tar_gz(archive, destination),
        "zip" => extract_zip(archive, destination),
        _ => Err("Unsupported archive format".into()),
    }
}
