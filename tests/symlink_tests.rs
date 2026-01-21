#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;
    use serial_test::serial;
    use nu_plugin_sdkman::core::env;

    #[test]
    #[serial]
    #[cfg(unix)]
    fn test_set_and_get_current_version_unix() {
        let temp = tempdir().unwrap();
        std::env::set_var("SDKMAN_DIR", temp.path());

        // Create fake installation
        let java_17 = temp.path().join("candidates/java/17.0.9");
        fs::create_dir_all(&java_17).unwrap();

        // Set current version
        env::set_current_version("java", "17.0.9").unwrap();

        // Verify symlink exists
        let current = temp.path().join("candidates/java/current");
        assert!(current.exists());
        assert!(current.is_symlink());

        // Verify symlink points to correct target
        let target = fs::read_link(&current).unwrap();
        assert!(target.ends_with("17.0.9"));

        // Verify get_current_version reads it back
        let version = env::get_current_version("java");
        assert_eq!(version, Some("17.0.9".to_string()));

        std::env::remove_var("SDKMAN_DIR");
    }

    #[test]
    #[serial]
    #[cfg(windows)]
    fn test_set_and_get_current_version_windows() {
        let temp = tempdir().unwrap();
        std::env::set_var("SDKMAN_DIR", temp.path());

        // Create fake installation
        let java_17 = temp.path().join("candidates/java/17.0.9");
        fs::create_dir_all(&java_17).unwrap();

        // Set current version
        env::set_current_version("java", "17.0.9").unwrap();

        // Verify current directory exists
        let current = temp.path().join("candidates/java/current");
        assert!(current.exists());
        assert!(current.is_dir());

        // Verify .version file exists with correct content
        let version_file = current.join(".version");
        assert!(version_file.exists());
        let content = fs::read_to_string(&version_file).unwrap();
        assert_eq!(content.trim(), "17.0.9");

        // Verify get_current_version reads it back
        let version = env::get_current_version("java");
        assert_eq!(version, Some("17.0.9".to_string()));

        std::env::remove_var("SDKMAN_DIR");
    }

    #[test]
    #[serial]
    fn test_get_current_version_not_set() {
        let temp = tempdir().unwrap();
        std::env::set_var("SDKMAN_DIR", temp.path());

        // Create candidates dir but no current version
        fs::create_dir_all(temp.path().join("candidates/java")).unwrap();

        let version = env::get_current_version("java");
        assert_eq!(version, None);

        std::env::remove_var("SDKMAN_DIR");
    }

    #[test]
    #[serial]
    fn test_switch_versions() {
        let temp = tempdir().unwrap();
        std::env::set_var("SDKMAN_DIR", temp.path());

        // Create two fake installations
        let java_17 = temp.path().join("candidates/java/17.0.9");
        let java_21 = temp.path().join("candidates/java/21.0.1");
        fs::create_dir_all(&java_17).unwrap();
        fs::create_dir_all(&java_21).unwrap();

        // Set to 17
        env::set_current_version("java", "17.0.9").unwrap();
        assert_eq!(env::get_current_version("java"), Some("17.0.9".to_string()));

        // Switch to 21
        env::set_current_version("java", "21.0.1").unwrap();
        assert_eq!(env::get_current_version("java"), Some("21.0.1".to_string()));

        std::env::remove_var("SDKMAN_DIR");
    }
}
