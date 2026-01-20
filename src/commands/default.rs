use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::env;

pub struct Default;

impl PluginCommand for Default {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk default"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate to set default for")
            .optional("version", SyntaxShape::String, "Version to set as default (uses current if not specified)")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Set default version for a candidate"
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
        
        let target_version = if let Some(v) = version {
            v
        } else {
            env::get_current_version(&candidate)
                .ok_or_else(|| LabeledError::new(format!("No current version set for {}", candidate)))?
        };
        
        if !env::is_installed(&candidate, &target_version) {
            return Err(LabeledError::new(format!(
                "{} {} is not installed. Run 'sdk install {} {}' first",
                candidate, target_version, candidate, target_version
            )));
        }
        
        env::set_current_version(&candidate, &target_version)
            .map_err(|e| LabeledError::new(format!("Failed to set default version: {}", e)))?;
        
        Ok(Value::string(
            format!("Default {} version set to {}", candidate, target_version),
            call.head,
        ).into_pipeline_data())
    }
}
