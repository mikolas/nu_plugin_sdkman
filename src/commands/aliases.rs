// Command aliases matching bash SDKMAN

use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape};
use crate::SdkmanPlugin;
use crate::commands::{Install, Uninstall, List, Use, Current, Upgrade, Default, Home, Version};

// i -> install
pub struct I;
impl PluginCommand for I {
    type Plugin = SdkmanPlugin;
    fn name(&self) -> &str { "sdk i" }
    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate to install")
            .optional("version", SyntaxShape::String, "Version to install")
            .category(Category::Custom("sdk".into()))
    }
    fn description(&self) -> &str { "Alias for 'sdk install'" }
    fn run(&self, plugin: &Self::Plugin, engine: &EngineInterface, call: &EvaluatedCall, input: nu_protocol::PipelineData) -> Result<nu_protocol::PipelineData, LabeledError> {
        Install.run(plugin, engine, call, input)
    }
}

// rm -> uninstall
pub struct Rm;
impl PluginCommand for Rm {
    type Plugin = SdkmanPlugin;
    fn name(&self) -> &str { "sdk rm" }
    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate to uninstall")
            .required("version", SyntaxShape::String, "Version to uninstall")
            .category(Category::Custom("sdk".into()))
    }
    fn description(&self) -> &str { "Alias for 'sdk uninstall'" }
    fn run(&self, plugin: &Self::Plugin, engine: &EngineInterface, call: &EvaluatedCall, input: nu_protocol::PipelineData) -> Result<nu_protocol::PipelineData, LabeledError> {
        Uninstall.run(plugin, engine, call, input)
    }
}

// ls -> list
pub struct Ls;
impl PluginCommand for Ls {
    type Plugin = SdkmanPlugin;
    fn name(&self) -> &str { "sdk ls" }
    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("candidate", SyntaxShape::String, "Candidate to list versions for")
            .category(Category::Custom("sdk".into()))
    }
    fn description(&self) -> &str { "Alias for 'sdk list'" }
    fn run(&self, plugin: &Self::Plugin, engine: &EngineInterface, call: &EvaluatedCall, input: nu_protocol::PipelineData) -> Result<nu_protocol::PipelineData, LabeledError> {
        List.run(plugin, engine, call, input)
    }
}

// u -> use
pub struct U;
impl PluginCommand for U {
    type Plugin = SdkmanPlugin;
    fn name(&self) -> &str { "sdk u" }
    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate to use")
            .required("version", SyntaxShape::String, "Version to use")
            .category(Category::Custom("sdk".into()))
    }
    fn description(&self) -> &str { "Alias for 'sdk use'" }
    fn run(&self, plugin: &Self::Plugin, engine: &EngineInterface, call: &EvaluatedCall, input: nu_protocol::PipelineData) -> Result<nu_protocol::PipelineData, LabeledError> {
        Use.run(plugin, engine, call, input)
    }
}

// c -> current
pub struct C;
impl PluginCommand for C {
    type Plugin = SdkmanPlugin;
    fn name(&self) -> &str { "sdk c" }
    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("candidate", SyntaxShape::String, "Candidate to show current version for")
            .category(Category::Custom("sdk".into()))
    }
    fn description(&self) -> &str { "Alias for 'sdk current'" }
    fn run(&self, plugin: &Self::Plugin, engine: &EngineInterface, call: &EvaluatedCall, input: nu_protocol::PipelineData) -> Result<nu_protocol::PipelineData, LabeledError> {
        Current.run(plugin, engine, call, input)
    }
}

// ug -> upgrade
pub struct Ug;
impl PluginCommand for Ug {
    type Plugin = SdkmanPlugin;
    fn name(&self) -> &str { "sdk ug" }
    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("candidate", SyntaxShape::String, "Candidate to upgrade")
            .category(Category::Custom("sdk".into()))
    }
    fn description(&self) -> &str { "Alias for 'sdk upgrade'" }
    fn run(&self, plugin: &Self::Plugin, engine: &EngineInterface, call: &EvaluatedCall, input: nu_protocol::PipelineData) -> Result<nu_protocol::PipelineData, LabeledError> {
        Upgrade.run(plugin, engine, call, input)
    }
}

// d -> default
pub struct D;
impl PluginCommand for D {
    type Plugin = SdkmanPlugin;
    fn name(&self) -> &str { "sdk d" }
    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate to set default for")
            .optional("version", SyntaxShape::String, "Version to set as default")
            .category(Category::Custom("sdk".into()))
    }
    fn description(&self) -> &str { "Alias for 'sdk default'" }
    fn run(&self, plugin: &Self::Plugin, engine: &EngineInterface, call: &EvaluatedCall, input: nu_protocol::PipelineData) -> Result<nu_protocol::PipelineData, LabeledError> {
        Default.run(plugin, engine, call, input)
    }
}

// h -> home
pub struct H;
impl PluginCommand for H {
    type Plugin = SdkmanPlugin;
    fn name(&self) -> &str { "sdk h" }
    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("candidate", SyntaxShape::String, "Candidate name")
            .required("version", SyntaxShape::String, "Version")
            .category(Category::Custom("sdk".into()))
    }
    fn description(&self) -> &str { "Alias for 'sdk home'" }
    fn run(&self, plugin: &Self::Plugin, engine: &EngineInterface, call: &EvaluatedCall, input: nu_protocol::PipelineData) -> Result<nu_protocol::PipelineData, LabeledError> {
        Home.run(plugin, engine, call, input)
    }
}

// v -> version
pub struct V;
impl PluginCommand for V {
    type Plugin = SdkmanPlugin;
    fn name(&self) -> &str { "sdk v" }
    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Custom("sdk".into()))
    }
    fn description(&self) -> &str { "Alias for 'sdk version'" }
    fn run(&self, plugin: &Self::Plugin, engine: &EngineInterface, call: &EvaluatedCall, input: nu_protocol::PipelineData) -> Result<nu_protocol::PipelineData, LabeledError> {
        Version.run(plugin, engine, call, input)
    }
}
