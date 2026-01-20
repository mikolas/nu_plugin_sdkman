use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::env;
use std::process::Command;

pub struct Config;

impl PluginCommand for Config {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk config"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Edit SDKMAN configuration file"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let config_file = env::sdkman_dir().join("etc").join("config");
        
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        
        Command::new(&editor)
            .arg(&config_file)
            .status()
            .map_err(|e| LabeledError::new(format!("Failed to open editor: {}", e)))?;
        
        Ok(Value::string(
            format!("Edited {}", config_file.display()),
            call.head,
        ).into_pipeline_data())
    }
}
