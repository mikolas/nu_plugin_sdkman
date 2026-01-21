#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;
    use serial_test::serial;
    use nu_plugin_sdkman::core::env;
    use nu_plugin_sdkman::utils::archive;

    #[test]
    #[serial]
    fn test_install_and_use_flow() {
        let temp = tempdir().unwrap();
        let sdkman_path = temp.path().to_str().unwrap();
        std::env::set_var("SDKMAN_DIR", sdkman_path);

        // Simulate installation by extracting test archive
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/test.tar.gz");
        
        let install_dir = temp.path().join("candidates/testsdk/1.0.0");
        fs::create_dir_all(&install_dir).unwrap();
        
        // Extract archive to installation directory
        archive::extract_tar_gz(&fixture_path, &install_dir).unwrap();
        
        // Verify installation directory and files exist
        assert!(install_dir.exists());
        assert!(install_dir.join("test-sdk/bin/test").exists());

        // Set as current version
        env::set_current_version("testsdk", "1.0.0").unwrap();

        // Verify current symlink/marker created
        let current = temp.path().join("candidates/testsdk/current");
        assert!(current.exists());

        // Verify get_current_version returns correct version
        let version = env::get_current_version("testsdk");
        assert_eq!(version, Some("1.0.0".to_string()));

        std::env::remove_var("SDKMAN_DIR");
    }

    #[test]
    #[serial]
    fn test_install_multiple_versions() {
        let temp = tempdir().unwrap();
        let sdkman_path = temp.path().to_str().unwrap();
        std::env::set_var("SDKMAN_DIR", sdkman_path);

        // Create two versions
        let v1 = temp.path().join("candidates/testsdk/1.0.0");
        let v2 = temp.path().join("candidates/testsdk/2.0.0");
        fs::create_dir_all(&v1).unwrap();
        fs::create_dir_all(&v2).unwrap();

        // Verify directories exist
        assert!(v1.exists());
        assert!(v2.exists());

        // Get all installed versions
        let versions = env::get_installed_versions("testsdk");
        assert_eq!(versions.len(), 2);
        assert!(versions.contains(&"1.0.0".to_string()));
        assert!(versions.contains(&"2.0.0".to_string()));

        std::env::remove_var("SDKMAN_DIR");
    }

    #[test]
    #[serial]
    fn test_use_nonexistent_version() {
        let temp = tempdir().unwrap();
        let sdkman_path = temp.path().to_str().unwrap();
        std::env::set_var("SDKMAN_DIR", sdkman_path);

        // Try to set current to non-existent version
        let result = env::set_current_version("testsdk", "99.99.99");
        
        // Should fail because version doesn't exist
        assert!(result.is_err());

        std::env::remove_var("SDKMAN_DIR");
    }
}
