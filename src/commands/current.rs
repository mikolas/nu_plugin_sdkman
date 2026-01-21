use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Record, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::env;

pub struct Current;

impl PluginCommand for Current {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk current"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("candidate", SyntaxShape::String, "Candidate to show current version for")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Show current version in use"
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
            show_current(&candidate, call)
        } else {
            show_all_current(call)
        }
    }
}

fn show_current(candidate: &str, call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let current = env::get_current_version(candidate);
    
    if let Some(version) = current {
        Ok(Value::record(
            Record::from_iter(vec![
                ("candidate".into(), Value::string(candidate, call.head)),
                ("version".into(), Value::string(version, call.head)),
            ]),
            call.head,
        ).into_pipeline_data())
    } else {
        Err(LabeledError::new(format!("No {} version in use", candidate)))
    }
}

fn show_all_current(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let candidates_dir = env::candidates_dir()
        .map_err(|e| LabeledError::new(e.to_string()))?;
    
    if !candidates_dir.exists() {
        return Ok(Value::list(vec![], call.head).into_pipeline_data());
    }
    
    let mut records = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&candidates_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if entry.path().is_dir() {
                let candidate = entry.file_name().to_string_lossy().to_string();
                if let Some(version) = env::get_current_version(&candidate) {
                    records.push(Value::record(
                        Record::from_iter(vec![
                            ("candidate".into(), Value::string(candidate, call.head)),
                            ("version".into(), Value::string(version, call.head)),
                        ]),
                        call.head,
                    ));
                }
            }
        }
    }
    
    Ok(Value::list(records, call.head).into_pipeline_data())
}
