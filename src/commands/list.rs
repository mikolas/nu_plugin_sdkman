use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::{api, env};

pub struct List;

impl PluginCommand for List {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk list"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("candidate", SyntaxShape::String, "Candidate to list versions for")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "List available candidates or versions"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let candidate: Option<String> = call.opt(0)?;
        
        if let Some(candidate) = candidate {
            list_versions(&candidate, call)
        } else {
            list_candidates(call)
        }
    }
}

fn list_candidates(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let text = api::get_candidates_list()
        .map_err(|e| LabeledError::new(format!("Failed to fetch candidates: {}", e)))?;
    
    Ok(Value::string(text, call.head).into_pipeline_data())
}

fn list_versions(candidate: &str, call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    api::validate_candidate(candidate)
        .map_err(|e| LabeledError::new(e.to_string()))?;
    
    let platform = env::detect_platform();
    let current = env::get_current_version(candidate).unwrap_or_default();
    let installed = env::get_installed_versions(candidate).join(",");
    
    let text = api::get_versions_list(candidate, &platform, &current, &installed)
        .map_err(|e| LabeledError::new(format!("Failed to fetch versions: {}", e)))?;
    
    Ok(Value::string(text, call.head).into_pipeline_data())
}
