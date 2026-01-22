/// Constants for SDKMAN directory structure and configuration.

// SDKMAN directory structure
pub const SDKMAN_DIR_NAME: &str = ".sdkman";
pub const SDKMAN_RC_FILE: &str = ".sdkmanrc";
pub const CANDIDATES_DIR: &str = "candidates";
pub const CURRENT_LINK: &str = "current";
pub const TMP_DIR: &str = "tmp";
pub const VAR_DIR: &str = "var";
pub const ETC_DIR: &str = "etc";
pub const METADATA_DIR: &str = "metadata";

// Environment variables
pub const ENV_SDKMAN_DIR: &str = "SDKMAN_DIR";
pub const ENV_PWD: &str = "PWD";

// Windows-specific
pub const VERSION_MARKER: &str = ".version";

// Activation scripts
pub const ENV_SCRIPT_NU: &str = "env.nu";
pub const ENV_SCRIPT_SH: &str = "env.sh";
pub const ENV_SCRIPT_FISH: &str = "env.fish";

// Config files
pub const CONFIG_FILE: &str = "config";
pub const VERSION_FILE: &str = "version";
