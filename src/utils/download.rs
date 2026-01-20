use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn download_file(url: &str, output: &Path) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let mut file = File::create(output)?;
    let content = response.bytes()?;
    std::io::copy(&mut content.as_ref(), &mut file)?;
    Ok(())
}
