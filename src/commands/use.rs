use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::env;

pub struct Use;

impl PluginCommand for Use {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk use"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate to use")
            .required("version", SyntaxShape::String, "Version to use")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Set a candidate version as current"
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
                "{} {} is not installed. Run 'sdk install {} {}' first",
                candidate, version, candidate, version
            )));
        }
        
        let message = if env::is_local_env() {
            env::set_local_current_version(&candidate, &version)
                .map_err(|e| LabeledError::new(format!("Failed to set local current version: {}", e)))?;
            format!("Using {} {} (local)", candidate, version)
        } else {
            env::set_current_version(&candidate, &version)
                .map_err(|e| LabeledError::new(format!("Failed to set current version: {}", e)))?;
            format!("Using {} {}", candidate, version)
        };
        
        Ok(Value::string(message, call.head).into_pipeline_data())
    }
}
