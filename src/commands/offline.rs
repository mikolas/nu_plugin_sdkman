use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;

pub struct Offline;

impl PluginCommand for Offline {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk offline"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("mode", SyntaxShape::String, "enable or disable")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Enable or disable offline mode"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let mode: Option<String> = call.opt(0)?;
        
        let message = match mode.as_deref() {
            Some("enable") | None => "Offline mode enabled (note: plugin doesn't persist this setting)",
            Some("disable") => "Online mode re-enabled",
            Some(m) => return Err(LabeledError::new(format!("Unknown mode: {}. Use 'enable' or 'disable'", m))),
        };
        
        Ok(Value::string(message, call.head).into_pipeline_data())
    }
}
