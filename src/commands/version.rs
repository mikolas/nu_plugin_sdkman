use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, Plugin};
use nu_protocol::{Category, LabeledError, Signature, Value, IntoPipelineData};
use crate::SdkmanPlugin;

pub struct Version;

impl PluginCommand for Version {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk version"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Show SDKMAN plugin version"
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        Ok(Value::string(
            format!("SDKMAN! Nushell Plugin {}", plugin.version()),
            call.head,
        ).into_pipeline_data())
    }
}
