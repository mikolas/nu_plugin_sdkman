use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::{env, install};
use std::fs;
use std::collections::HashMap;

pub struct Env;

impl PluginCommand for Env {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk env"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("subcommand", SyntaxShape::String, "Subcommand: init, install, clear, or empty to load")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Manage .sdkmanrc files for project-specific SDK versions"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let subcommand: Option<String> = call.opt(0)?;
        
        match subcommand.as_deref() {
            Some("init") => env_init(call),
            Some("install") => env_install(call),
            Some("clear") => env_clear(call),
            None => env_load(call),
            Some(cmd) => Err(LabeledError::new(format!("Unknown subcommand: {}", cmd))),
        }
    }
}

fn env_init(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    // Get current directory from Nushell's environment, not the plugin process
    let current_dir = std::env::var("PWD")
        .ok()
        .and_then(|p| std::path::PathBuf::from(p).canonicalize().ok())
        .or_else(|| std::env::current_dir().ok())
        .ok_or_else(|| LabeledError::new("Failed to get current directory"))?;
    
    let sdkmanrc = current_dir.join(".sdkmanrc");
    let local_sdkman = current_dir.join(".sdkman");
    
    if sdkmanrc.exists() {
        return Err(LabeledError::new(".sdkmanrc already exists in current directory"));
    }
    
    // Create .sdkmanrc
    let mut content = String::from("# SDKMAN local environment\n");
    content.push_str("# Add key=value pairs of SDKs to use below\n");
    
    // Add current global versions if any
    let candidates_dir = env::candidates_dir()
        .map_err(|e| LabeledError::new(e.to_string()))?;
        
    if candidates_dir.exists() {
        if let Ok(entries) = fs::read_dir(&candidates_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry.path().is_dir() {
                    let candidate = entry.file_name().to_string_lossy().to_string();
                    if candidate != "current" {
                        if let Some(version) = env::get_current_version(&candidate) {
                            content.push_str(&format!("{}={}\n", candidate, version));
                        }
                    }
                }
            }
        }
    }
    
    fs::write(&sdkmanrc, content)
        .map_err(|e| LabeledError::new(format!("Failed to create .sdkmanrc: {}", e)))?;
    
    // Create .sdkman directory structure
    fs::create_dir_all(local_sdkman.join("candidates"))
        .map_err(|e| LabeledError::new(format!("Failed to create .sdkman directory: {}", e)))?;
    
    // Create activation script
    let env_script = r#"# SDKMAN Local Environment
# Source this file to activate local SDK versions

export-env {
    let local_bins = (
        try {
            ls .sdkman/candidates/*/current/bin 
            | get name 
            | path expand
        } catch {
            []
        }
    )
    
    if ($local_bins | length) > 0 {
        $env.PATH = ($env.PATH | prepend $local_bins)
    }
}
"#;
    
    fs::write(local_sdkman.join("env.nu"), env_script)
        .map_err(|e| LabeledError::new(format!("Failed to create env.nu: {}", e)))?;
    
    let message = format!(
        "Created local SDKMAN environment in {}\n\n\
        To activate:\n  source .sdkman/env.nu\n\n\
        To install SDKs locally:\n  source .sdkman/env.nu\n  sdk env install\n\n\
        Note: SDKs are installed globally but symlinked locally for isolation.",
        current_dir.display()
    );
    
    Ok(Value::string(message, call.head).into_pipeline_data())
}

fn env_install(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let current_dir = std::env::var("PWD")
        .ok()
        .and_then(|p| std::path::PathBuf::from(p).canonicalize().ok())
        .or_else(|| std::env::current_dir().ok())
        .ok_or_else(|| LabeledError::new("Failed to get current directory"))?;
    
    let sdkmanrc = current_dir.join(".sdkmanrc");
    
    if !sdkmanrc.exists() {
        return Err(LabeledError::new("Could not find .sdkmanrc in current directory. Run 'sdk env init' to create it."));
    }
    
    let versions = parse_sdkmanrc(&sdkmanrc)?;
    let mut results = Vec::new();
    let platform = env::detect_platform()
        .map_err(|e| LabeledError::new(e.to_string()))?;
    
    let is_local = env::is_local_env();
    
    for (candidate, version) in versions {
        // Always install to global location
        if env::is_installed(&candidate, &version) {
            results.push(format!("{} {} already installed", candidate, version));
        } else {
            install::install_candidate(&candidate, &version, &platform)
                .map_err(|e| LabeledError::new(format!("Failed to install {} {}: {}", candidate, version, e)))?;
            results.push(format!("Installed {} {}", candidate, version));
        }
        
        // Set current version (local or global depending on mode)
        if is_local {
            env::set_local_current_version(&candidate, &version)
                .map_err(|e| LabeledError::new(format!("Failed to set local {}: {}", candidate, e)))?;
            results.push(format!("Linked {} {} locally", candidate, version));
        } else {
            env::set_current_version(&candidate, &version)
                .map_err(|e| LabeledError::new(format!("Failed to set {}: {}", candidate, e)))?;
        }
    }
    
    Ok(Value::string(results.join("\n"), call.head).into_pipeline_data())
}

fn env_load(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let current_dir = std::env::var("PWD")
        .ok()
        .and_then(|p| std::path::PathBuf::from(p).canonicalize().ok())
        .or_else(|| std::env::current_dir().ok())
        .ok_or_else(|| LabeledError::new("Failed to get current directory"))?;
    
    let sdkmanrc = current_dir.join(".sdkmanrc");
    
    if !sdkmanrc.exists() {
        return Err(LabeledError::new("Could not find .sdkmanrc in current directory"));
    }
    
    let versions = parse_sdkmanrc(&sdkmanrc)?;
    let mut results = Vec::new();
    let is_local = env::is_local_env();
    
    for (candidate, version) in versions {
        if !env::is_installed(&candidate, &version) {
            results.push(format!("{} {} not installed", candidate, version));
        } else {
            if is_local {
                env::set_local_current_version(&candidate, &version)
                    .map_err(|e| LabeledError::new(format!("Failed to set local {}: {}", candidate, e)))?;
                results.push(format!("Using {} {} (local)", candidate, version));
            } else {
                env::set_current_version(&candidate, &version)
                    .map_err(|e| LabeledError::new(format!("Failed to set {}: {}", candidate, e)))?;
                results.push(format!("Using {} {}", candidate, version));
            }
        }
    }
    
    Ok(Value::string(results.join("\n"), call.head).into_pipeline_data())
}

fn env_clear(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let current_dir = std::env::var("PWD")
        .ok()
        .and_then(|p| std::path::PathBuf::from(p).canonicalize().ok())
        .or_else(|| std::env::current_dir().ok())
        .ok_or_else(|| LabeledError::new("Failed to get current directory"))?;
    
    let sdkmanrc = current_dir.join(".sdkmanrc");
    
    if !sdkmanrc.exists() {
        return Err(LabeledError::new("Could not find .sdkmanrc in current directory"));
    }
    
    let versions = parse_sdkmanrc(&sdkmanrc)?;
    
    let is_local = env::is_local_env();
    
    for (candidate, _) in versions {
        if is_local {
            // Remove local symlinks only
            if let Some(local_dir) = env::local_sdkman_dir() {
                let local_current = local_dir.join("candidates").join(&candidate).join("current");
                if local_current.exists() {
                    fs::remove_dir_all(&local_current).ok();
                }
            }
        } else {
            // Remove global symlinks
            let current_link = env::candidate_current(&candidate);
            if let Ok(path) = current_link {
                 if path.exists() {
                     fs::remove_dir_all(&path).ok();
                 }
            }
        }
    }
    
    let message = if is_local {
        "Cleared local environment (global installations unaffected)"
    } else {
        "Cleared environment"
    };
    
    Ok(Value::string(message, call.head).into_pipeline_data())
}

fn parse_sdkmanrc(path: &std::path::Path) -> Result<HashMap<String, String>, LabeledError> {
    let content = fs::read_to_string(path)
        .map_err(|e| LabeledError::new(format!("Failed to read .sdkmanrc: {}", e)))?;
    
    parse_sdkmanrc_content(&content)
}

/// Parse .sdkmanrc content (extracted for testing)
pub fn parse_sdkmanrc_content(content: &str) -> Result<HashMap<String, String>, LabeledError> {
    let mut versions = HashMap::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            versions.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    
    Ok(versions)
}
