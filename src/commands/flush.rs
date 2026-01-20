use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value, IntoPipelineData};
use crate::SdkmanPlugin;
use crate::core::env;
use std::fs;

pub struct Flush;

impl PluginCommand for Flush {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk flush"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("target", SyntaxShape::String, "What to flush: tmp, metadata, version, or all")
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "Clear caches (tmp/metadata/version)"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let target: Option<String> = call.opt(0)?;
        
        let sdkman_dir = env::sdkman_dir();
        let mut flushed = Vec::new();
        
        match target.as_deref() {
            Some("tmp") | Some("temp") => {
                flush_dir(&sdkman_dir.join("tmp"), &mut flushed)?;
            }
            Some("metadata") => {
                flush_dir(&sdkman_dir.join("var").join("metadata"), &mut flushed)?;
            }
            Some("version") => {
                let version_file = sdkman_dir.join("var").join("version");
                if version_file.exists() {
                    fs::remove_file(&version_file)
                        .map_err(|e| LabeledError::new(format!("Failed to remove version file: {}", e)))?;
                    flushed.push("version file".to_string());
                }
            }
            _ => {
                // Flush all
                flush_dir(&sdkman_dir.join("tmp"), &mut flushed)?;
                flush_dir(&sdkman_dir.join("var").join("metadata"), &mut flushed)?;
            }
        }
        
        let message = if flushed.is_empty() {
            "Nothing to flush".to_string()
        } else {
            format!("Flushed: {}", flushed.join(", "))
        };
        
        Ok(Value::string(message, call.head).into_pipeline_data())
    }
}

fn flush_dir(path: &std::path::Path, flushed: &mut Vec<String>) -> Result<(), LabeledError> {
    if !path.exists() {
        return Ok(());
    }
    
    let count = fs::read_dir(path)
        .map(|entries| entries.count())
        .unwrap_or(0);
    
    fs::remove_dir_all(path).ok();
    fs::create_dir_all(path).ok();
    
    if count > 0 {
        flushed.push(format!("{} ({} items)", path.file_name().unwrap().to_string_lossy(), count));
    }
    
    Ok(())
}
