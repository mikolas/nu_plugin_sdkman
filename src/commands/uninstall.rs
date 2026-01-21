use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::env;

pub struct Uninstall;

impl PluginCommand for Uninstall {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk uninstall"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate to uninstall")
            .required("version", SyntaxShape::String, "Version to uninstall")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Uninstall a candidate version"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let candidate: String = call.req(0)?;
        let version: String = call.req(1)?;
        
        if !env::is_installed(&candidate, &version) {
            return Err(LabeledError::new(format!(
                "{} {} is not installed",
                candidate, version
            )));
        }
        
        let install_dir = env::candidate_dir(&candidate, &version)
            .map_err(|e| LabeledError::new(e.to_string()))?;
        std::fs::remove_dir_all(&install_dir)
            .map_err(|e| LabeledError::new(format!("Failed to remove installation: {}", e)))?;
        
        let current = env::get_current_version(&candidate);
        if current.as_ref() == Some(&version) {
            if let Ok(current_link) = env::candidate_current(&candidate) {
                std::fs::remove_dir_all(&current_link).ok();
            }
        }
        
        Ok(Value::string(
            format!("{} {} uninstalled successfully", candidate, version),
            call.head,
        ).into_pipeline_data())
    }
}
