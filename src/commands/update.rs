use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::api;

pub struct Update;

impl PluginCommand for Update {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk update"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Update local candidate cache"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let candidates = api::get_candidates()
            .map_err(|e| LabeledError::new(format!("Failed to fetch candidates: {}", e)))?;
        
        Ok(Value::string(
            format!("Candidate cache updated. {} candidates available.", candidates.len()),
            call.head,
        ).into_pipeline_data())
    }
}
