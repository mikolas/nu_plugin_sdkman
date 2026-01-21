use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Record, Signature, SyntaxShape, Value, IntoPipelineData};
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
    
    let platform = env::detect_platform()
        .map_err(|e| LabeledError::new(e.to_string()))?;
    let current = env::get_current_version(candidate).unwrap_or_default();
    let installed = env::get_installed_versions(candidate).join(",");
    
    let text = api::get_versions_list(candidate, &platform, &current, &installed)
        .map_err(|e| LabeledError::new(format!("Failed to fetch versions: {}", e)))?;
    
    let mut rows = Vec::new();
    let mut current_vendor = String::new();
    
    for line in text.lines() {
        let line_trim = line.trim();
        if line_trim.starts_with("====") || line_trim.starts_with("Available") || line_trim.contains("Vendor") || line_trim.starts_with("----") {
            continue;
        }
        
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 6 {
            continue;
        }
        
        let vendor = parts[0].trim();
        if !vendor.is_empty() {
            current_vendor = vendor.to_string();
        }
        
        let use_marker = parts[1].trim();
        let version = parts[2].trim();
        let dist = parts[3].trim();
        let status = parts[4].trim();
        let identifier = parts[5].trim();
        
        if identifier.is_empty() {
            continue;
        }
        
        rows.push(Value::record(
            Record::from_iter(vec![
                ("vendor".into(), Value::string(current_vendor.clone(), call.head)),
                ("use".into(), Value::string(use_marker, call.head)),
                ("version".into(), Value::string(version, call.head)),
                ("dist".into(), Value::string(dist, call.head)),
                ("status".into(), Value::string(status, call.head)),
                ("identifier".into(), Value::string(identifier, call.head)),
            ]),
            call.head,
        ));
    }
    
    if rows.is_empty() {
        // Fallback if parsing failed or text was empty (though we filtered headers)
        // If text was just headers, rows is empty.
        // Maybe return the raw text if parsing found nothing? 
        // No, user expects table now. Empty table is better than confusion if valid but empty.
        // But if text was NOT empty but parsing failed (layout change), returning raw text might be better for debugging.
        // Let's stick to table.
    }
    
    Ok(Value::list(rows, call.head).into_pipeline_data())
}
