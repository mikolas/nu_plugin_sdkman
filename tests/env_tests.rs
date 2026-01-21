#[cfg(test)]
mod tests {
    use nu_plugin_sdkman::commands::env;

    #[test]
    fn test_parse_sdkmanrc_basic() {
        let content = "java=17.0.9-oracle\ngradle=8.5\nmaven=3.9.6\n";
        let parsed = env::parse_sdkmanrc_content(content).unwrap();
        
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed.get("java"), Some(&"17.0.9-oracle".to_string()));
        assert_eq!(parsed.get("gradle"), Some(&"8.5".to_string()));
        assert_eq!(parsed.get("maven"), Some(&"3.9.6".to_string()));
    }

    #[test]
    fn test_parse_sdkmanrc_with_comments() {
        let content = "# This is a comment\njava=17.0.9-oracle\n# Another comment\ngradle=8.5\n";
        let parsed = env::parse_sdkmanrc_content(content).unwrap();
        
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed.get("java"), Some(&"17.0.9-oracle".to_string()));
        assert_eq!(parsed.get("gradle"), Some(&"8.5".to_string()));
    }

    #[test]
    fn test_parse_sdkmanrc_with_whitespace() {
        let content = "  java = 17.0.9-oracle  \n\n  gradle = 8.5  \n";
        let parsed = env::parse_sdkmanrc_content(content).unwrap();
        
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed.get("java"), Some(&"17.0.9-oracle".to_string()));
        assert_eq!(parsed.get("gradle"), Some(&"8.5".to_string()));
    }

    #[test]
    fn test_parse_sdkmanrc_empty() {
        let content = "";
        let parsed = env::parse_sdkmanrc_content(content).unwrap();
        
        assert_eq!(parsed.len(), 0);
    }

    #[test]
    fn test_parse_sdkmanrc_malformed_lines() {
        let content = "java=17.0.9-oracle\ninvalid line without equals\ngradle=8.5\n=nokey\nnovalue=\n";
        let parsed = env::parse_sdkmanrc_content(content).unwrap();
        
        // Should parse valid lines and skip invalid ones
        // Lines with = are parsed: java=..., gradle=..., =nokey (empty key), novalue= (empty value)
        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed.get("java"), Some(&"17.0.9-oracle".to_string()));
        assert_eq!(parsed.get("gradle"), Some(&"8.5".to_string()));
        assert_eq!(parsed.get(""), Some(&"nokey".to_string())); // =nokey creates empty key
        assert_eq!(parsed.get("novalue"), Some(&"".to_string()));
    }

    #[test]
    fn test_parse_sdkmanrc_only_comments() {
        let content = "# Comment 1\n# Comment 2\n# Comment 3\n";
        let parsed = env::parse_sdkmanrc_content(content).unwrap();
        
        assert_eq!(parsed.len(), 0);
    }
}
