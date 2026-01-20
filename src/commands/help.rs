use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Value, IntoPipelineData};
use crate::SdkmanPlugin;

pub struct Sdk;

impl PluginCommand for Sdk {
    type Plugin = SdkmanPlugin;

    fn name(&self) -> &str {
        "sdk"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .category(Category::Custom("sdk".into()))
    }

    fn description(&self) -> &str {
        "SDKMAN! - The Software Development Kit Manager"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let help = r#"SDKMAN! - The Software Development Kit Manager

Usage: sdk <command> [args]

Commands:
  list (ls) [candidate]              List available candidates or versions
  install (i) <candidate> [version]  Install a candidate version
  uninstall (rm) <candidate> <version> Uninstall a candidate version
  use (u) <candidate> <version>      Set a candidate version as current
  default (d) <candidate> [version]  Set default version for a candidate
  current (c) [candidate]            Show current version in use
  upgrade (ug) [candidate]           Upgrade candidate to latest version
  home (h) <candidate> <version>     Print home directory path
  env [init|install|clear]           Manage .sdkmanrc files
  version (v)                        Show SDKMAN plugin version
  update                             Update local candidate cache
  flush [tmp|metadata|version]       Clear caches
  config                             Edit configuration file
  offline [enable|disable]           Enable/disable offline mode

Examples:
  sdk list                           List all candidates
  sdk list java                      List Java versions
  sdk install java                   Install latest Java
  sdk install java 17                Install Java 17
  sdk install java 17 --local /path  Install from local archive
  sdk use java 17                    Switch to Java 17
  sdk default java 17                Set Java 17 as default
  sdk current                        Show all current versions
  sdk home java 17                   Show Java 17 home directory
  sdk env init                       Create .sdkmanrc in current directory
  sdk upgrade java                   Upgrade Java to latest

Aliases: i, rm, ls, u, c, ug, d, h, v

For more help: sdk <command> --help
"#;
        
        Ok(Value::string(help, call.head).into_pipeline_data())
    }
}
