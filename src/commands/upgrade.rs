use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::{api, env, install};

pub struct Upgrade;

impl PluginCommand for Upgrade {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk upgrade"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("candidate", SyntaxShape::String, "Candidate to upgrade (upgrades all if not specified)")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Upgrade candidate to latest version"
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
            upgrade_candidate(&candidate, call)
        } else {
            upgrade_all(call)
        }
    }
}

fn upgrade_candidate(candidate: &str, call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let current = env::get_current_version(candidate);
    
    if current.is_none() {
        return Err(LabeledError::new(format!("No {} version currently in use", candidate)));
    }
    
    let platform = env::detect_platform()
        .map_err(|e| LabeledError::new(e.to_string()))?;
    let latest = api::get_default_version(candidate, &platform)
        .map_err(|e| LabeledError::new(format!("Failed to get latest version: {}", e)))?;
    
    let current = current.unwrap();
    
    if current == latest {
        return Ok(Value::string(
            format!("{} is already at the latest version ({})", candidate, latest),
            call.head,
        ).into_pipeline_data());
    }
    
    if env::is_installed(candidate, &latest) {
        env::set_current_version(candidate, &latest)
            .map_err(|e| LabeledError::new(format!("Failed to set current version: {}", e)))?;
        
        return Ok(Value::string(
            format!("Upgraded {} from {} to {}", candidate, current, latest),
            call.head,
        ).into_pipeline_data());
    }
    
    install::install_candidate(candidate, &latest, &platform)
        .map_err(|e| LabeledError::new(format!("Upgrade failed: {}", e)))?;
    
    env::set_current_version(candidate, &latest)
        .map_err(|e| LabeledError::new(format!("Failed to set current version: {}", e)))?;
    
    Ok(Value::string(
        format!("Upgraded {} from {} to {}", candidate, current, latest),
        call.head,
    ).into_pipeline_data())
}

fn upgrade_all(call: &EvaluatedCall) -> Result<nu_protocol::PipelineData, LabeledError> {
    let candidates_dir = env::candidates_dir()
        .map_err(|e| LabeledError::new(e.to_string()))?;
    
    if !candidates_dir.exists() {
        return Ok(Value::string("No candidates installed", call.head).into_pipeline_data());
    }
    
    let mut results = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&candidates_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if entry.path().is_dir() {
                let candidate = entry.file_name().to_string_lossy().to_string();
                if env::get_current_version(&candidate).is_some() {
                    match upgrade_candidate(&candidate, call) {
                        Ok(pd) => {
                            if let Ok(v) = pd.into_value(call.head) {
                                results.push(v);
                            }
                        }
                        Err(e) => results.push(Value::string(
                            format!("{}: {}", candidate, e),
                            call.head,
                        )),
                    }
                }
            }
        }
    }
    
    Ok(Value::list(results, call.head).into_pipeline_data())
}
