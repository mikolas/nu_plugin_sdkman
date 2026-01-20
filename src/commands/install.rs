use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::{api, env};
use crate::utils::{download, archive};

pub struct Install;

impl PluginCommand for Install {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk install"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate to install")
            .optional("version", SyntaxShape::String, "Version to install (defaults to latest)")
            .named("local", SyntaxShape::String, "Install from local archive path", Some('l'))
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Install a candidate version"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let candidate: String = call.req(0)?;
        let version: Option<String> = call.opt(1)?;
        let local_path: Option<String> = call.get_flag("local")?;
        
        api::validate_candidate(&candidate)
            .map_err(|e| LabeledError::new(e.to_string()))?;
        
        let platform = env::detect_platform();
        
        let install_version = if let Some(v) = version {
            v
        } else {
            api::get_default_version(&candidate, &platform)
                .map_err(|e| LabeledError::new(format!("Failed to get default version: {}", e)))?
        };
        
        if env::is_installed(&candidate, &install_version) {
            return Err(LabeledError::new(format!(
                "{} {} is already installed",
                candidate, install_version
            )));
        }
        
        if let Some(local) = local_path {
            install_from_local(&candidate, &install_version, &local)?;
        } else {
            install_from_remote(&candidate, &install_version, &platform)?;
        }
        
        env::set_current_version(&candidate, &install_version)
            .map_err(|e| LabeledError::new(format!("Failed to set current version: {}", e)))?;
        
        Ok(Value::string(
            format!("{} {} installed successfully", candidate, install_version),
            call.head,
        ).into_pipeline_data())
    }
}

fn install_from_local(candidate: &str, version: &str, local_path: &str) -> Result<(), LabeledError> {
    let local_file = std::path::Path::new(local_path);
    
    if !local_file.exists() {
        return Err(LabeledError::new(format!("Local file not found: {}", local_path)));
    }
    
    let install_dir = env::candidate_dir(candidate, version);
    archive::extract(local_file, &install_dir)
        .map_err(|e| LabeledError::new(format!("Extraction failed: {}", e)))?;
    
    Ok(())
}

fn install_from_remote(candidate: &str, version: &str, platform: &str) -> Result<(), LabeledError> {
    let download_url = api::get_download_url(candidate, version, platform);
    
    let temp_dir = std::env::temp_dir().join(format!("sdkman-{}-{}", candidate, version));
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| LabeledError::new(format!("Failed to create temp dir: {}", e)))?;
    
    let archive_ext = if cfg!(windows) { ".zip" } else { ".tar.gz" };
    let archive_file = temp_dir.join(format!("{}-{}{}", candidate, version, archive_ext));
    
    download::download_file(&download_url, &archive_file)
        .map_err(|e| LabeledError::new(format!("Download failed: {}", e)))?;
    
    let install_dir = env::candidate_dir(candidate, version);
    archive::extract(&archive_file, &install_dir)
        .map_err(|e| LabeledError::new(format!("Extraction failed: {}", e)))?;
    
    std::fs::remove_dir_all(&temp_dir).ok();
    
    Ok(())
}
