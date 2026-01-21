#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;
    use nu_plugin_sdkman::utils::archive;

    #[test]
    fn test_extract_tar_gz() {
        let temp = tempdir().unwrap();
        
        // Use the test fixture
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/test.tar.gz");
        
        // Extract to temp dir
        archive::extract_tar_gz(&fixture_path, temp.path()).unwrap();
        
        // Verify structure
        let extracted = temp.path().join("test-sdk");
        assert!(extracted.exists());
        assert!(extracted.is_dir());
        
        let bin_dir = extracted.join("bin");
        assert!(bin_dir.exists());
        assert!(bin_dir.is_dir());
        
        let test_file = bin_dir.join("test");
        assert!(test_file.exists());
        
        // Verify content
        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("#!/bin/sh"));
        assert!(content.contains("echo \"test\""));
    }

    #[test]
    fn test_extract_nonexistent_archive() {
        let temp = tempdir().unwrap();
        let fake_archive = temp.path().join("nonexistent.tar.gz");
        
        let result = archive::extract_tar_gz(&fake_archive, temp.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_to_nonexistent_dir() {
        let temp = tempdir().unwrap();
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/test.tar.gz");
        
        // Extract to non-existent subdirectory (should create it)
        let target = temp.path().join("subdir/nested");
        let result = archive::extract_tar_gz(&fixture_path, &target);
        
        // Should succeed and create directories
        assert!(result.is_ok());
        assert!(target.exists());
    }

    #[test]
    fn test_extract_zip() {
        let temp = tempdir().unwrap();
        
        // Use the test fixture
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/test.zip");
        
        // Extract to temp dir
        archive::extract_zip(&fixture_path, temp.path()).unwrap();
        
        // Verify structure
        let extracted = temp.path().join("test-sdk");
        assert!(extracted.exists());
        assert!(extracted.is_dir());
        
        let bin_dir = extracted.join("bin");
        assert!(bin_dir.exists());
        assert!(bin_dir.is_dir());
        
        let test_file = bin_dir.join("test");
        assert!(test_file.exists());
        
        // Verify content
        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("#!/bin/sh"));
        assert!(content.contains("echo \"test\""));
    }

    #[test]
    fn test_extract_zip_nonexistent() {
        let temp = tempdir().unwrap();
        let fake_archive = temp.path().join("nonexistent.zip");
        
        let result = archive::extract_zip(&fake_archive, temp.path());
        assert!(result.is_err());
    }
}
