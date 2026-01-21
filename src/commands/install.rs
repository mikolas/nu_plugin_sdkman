use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::{api, env, install};

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
        
        let platform = env::detect_platform()
            .map_err(|e| LabeledError::new(e.to_string()))?;
        
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
            install::install_local(&candidate, &install_version, std::path::Path::new(&local))
                .map_err(|e| LabeledError::new(format!("Local install failed: {}", e)))?;
        } else {
            install::install_candidate(&candidate, &install_version, &platform)
                .map_err(|e| LabeledError::new(format!("Install failed: {}", e)))?;
        }
        
        env::set_current_version(&candidate, &install_version)
            .map_err(|e| LabeledError::new(format!("Failed to set current version: {}", e)))?;
        
        Ok(Value::string(
            format!("{} {} installed successfully", candidate, install_version),
            call.head,
        ).into_pipeline_data())
    }
}
