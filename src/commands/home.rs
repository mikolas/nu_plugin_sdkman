use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::env;

pub struct Home;

impl PluginCommand for Home {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk home"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate name")
            .required("version", SyntaxShape::String, "Version")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Print home directory path of installed candidate version"
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
        
        let home_dir = env::candidate_dir(&candidate, &version);
        
        Ok(Value::string(
            home_dir.to_string_lossy().to_string(),
            call.head,
        ).into_pipeline_data())
    }
}
