use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::{api, env};
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
    let sdkmanrc = std::env::current_dir()
        .map_err(|e| LabeledError::new(format!("Failed to get current directory: {}", e)))?
        .join(".sdkmanrc");
    
    if sdkmanrc.exists() {
        return Err(LabeledError::new(".sdkmanrc already exists in current directory"));
    }
    
    let mut content = String::from("# Enable auto-env through the sdkman_auto_env config\n");
    content.push_str("# Add key=value pairs of SDKs to use below\n");
    
    // Add current versions if any
    let candidates_dir = env::candidates_dir();
    if candidates_dir.exists() {
        if let Ok(entries) = fs::read_dir(&candidates_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry.path().is_dir() {
                    let candidate = entry.file_name().to_string_lossy().to_string();
                    if let Some(version) = env::get_current_version(&candidate) {
                        content.push_str(&format!("{}={}\n", candidate, version));
                    }
                }
            }
        }
    }
    
    fs::write(&sdkmanrc, content)
        .map_err(|e| LabeledError::new(format!("Failed to create .sdkmanrc: {}", e)))?;
    
    Ok(Value::string(
        format!("Created .sdkmanrc in {}", sdkmanrc.parent().unwrap().display()),
        call.head,
    ).into_pipeline_data())
}

fn env_install(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let sdkmanrc = std::env::current_dir()
        .map_err(|e| LabeledError::new(format!("Failed to get current directory: {}", e)))?
        .join(".sdkmanrc");
    
    if !sdkmanrc.exists() {
        return Err(LabeledError::new("Could not find .sdkmanrc in current directory. Run 'sdk env init' to create it."));
    }
    
    let versions = parse_sdkmanrc(&sdkmanrc)?;
    let mut results = Vec::new();
    
    for (candidate, version) in versions {
        if env::is_installed(&candidate, &version) {
            results.push(format!("{} {} already installed", candidate, version));
        } else {
            results.push(format!("Would install {} {} (install command not called from env)", candidate, version));
        }
    }
    
    Ok(Value::string(results.join("\n"), call.head).into_pipeline_data())
}

fn env_load(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let sdkmanrc = std::env::current_dir()
        .map_err(|e| LabeledError::new(format!("Failed to get current directory: {}", e)))?
        .join(".sdkmanrc");
    
    if !sdkmanrc.exists() {
        return Err(LabeledError::new("Could not find .sdkmanrc in current directory"));
    }
    
    let versions = parse_sdkmanrc(&sdkmanrc)?;
    let mut results = Vec::new();
    
    for (candidate, version) in versions {
        if !env::is_installed(&candidate, &version) {
            results.push(format!("{} {} not installed", candidate, version));
        } else {
            env::set_current_version(&candidate, &version)
                .map_err(|e| LabeledError::new(format!("Failed to set {}: {}", candidate, e)))?;
            results.push(format!("Using {} {}", candidate, version));
        }
    }
    
    Ok(Value::string(results.join("\n"), call.head).into_pipeline_data())
}

fn env_clear(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let sdkmanrc = std::env::current_dir()
        .map_err(|e| LabeledError::new(format!("Failed to get current directory: {}", e)))?
        .join(".sdkmanrc");
    
    if !sdkmanrc.exists() {
        return Err(LabeledError::new("Could not find .sdkmanrc in current directory"));
    }
    
    let versions = parse_sdkmanrc(&sdkmanrc)?;
    
    for (candidate, _) in versions {
        let current_link = env::candidate_current(&candidate);
        if current_link.exists() {
            fs::remove_dir_all(&current_link).ok();
        }
    }
    
    Ok(Value::string("Cleared environment", call.head).into_pipeline_data())
}

fn parse_sdkmanrc(path: &std::path::Path) -> Result<HashMap<String, String>, LabeledError> {
    let content = fs::read_to_string(path)
        .map_err(|e| LabeledError::new(format!("Failed to read .sdkmanrc: {}", e)))?;
    
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
