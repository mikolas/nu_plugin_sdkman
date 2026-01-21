use std::error::Error;
use std::fs::File;
use std::path::Path;

/// Downloads a file from a URL to the specified output path.
///
/// Uses blocking HTTP client (reqwest::blocking) for simplicity.
/// Downloads entire file into memory before writing.
///
/// # Arguments
/// * `url` - URL to download from
/// * `output` - Path where file should be saved
///
/// # Errors
/// Returns error if:
/// - Network request fails
/// - File cannot be created
/// - Write operation fails
pub fn download_file(url: &str, output: &Path) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let mut file = File::create(output)?;
    let content = response.bytes()?;
    std::io::copy(&mut content.as_ref(), &mut file)?;
    Ok(())
}
