#[cfg(test)]
mod tests {
    use nu_plugin_sdkman::core::api;

    #[test]
    fn test_parse_versions_basic() {
        let text = "21.0.9-oracle,21.0.9-zulu,17.0.17-tem";
        let versions = api::parse_versions_text(text);
        
        assert_eq!(versions.len(), 3);
        assert_eq!(versions[0].version, "21.0.9-oracle");
        assert_eq!(versions[0].vendor, "oracle");
        assert_eq!(versions[1].version, "21.0.9-zulu");
        assert_eq!(versions[1].vendor, "zulu");
        assert_eq!(versions[2].version, "17.0.17-tem");
        assert_eq!(versions[2].vendor, "tem");
    }

    #[test]
    fn test_parse_versions_no_vendor() {
        let text = "8.5,8.4,8.3";
        let versions = api::parse_versions_text(text);
        
        assert_eq!(versions.len(), 3);
        assert_eq!(versions[0].version, "8.5");
        assert_eq!(versions[0].vendor, "");
        assert_eq!(versions[1].version, "8.4");
        assert_eq!(versions[1].vendor, "");
    }

    #[test]
    fn test_parse_versions_with_whitespace() {
        let text = " 21.0.9-oracle , 21.0.9-zulu , 17.0.17-tem ";
        let versions = api::parse_versions_text(text);
        
        assert_eq!(versions.len(), 3);
        assert_eq!(versions[0].version, "21.0.9-oracle");
    }

    #[test]
    fn test_parse_versions_empty() {
        let text = "";
        let versions = api::parse_versions_text(text);
        
        assert_eq!(versions.len(), 0);
    }

    #[test]
    fn test_parse_versions_multi_dash_vendor() {
        let text = "23.1.9.r21-mandrel,24.2.2.r24-nik";
        let versions = api::parse_versions_text(text);
        
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, "23.1.9.r21-mandrel");
        assert_eq!(versions[0].vendor, "mandrel");
        assert_eq!(versions[1].version, "24.2.2.r24-nik");
        assert_eq!(versions[1].vendor, "nik");
    }
}
