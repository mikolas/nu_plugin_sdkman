pub mod commands;
pub mod core;
pub mod utils;

use nu_plugin::{Plugin, PluginCommand};
use commands::{Sdk, List, Install, Uninstall, Use, Current, Upgrade, Default, Home, Version};
use commands::{Env, Update, Flush, Config, Offline};
use commands::{I, Rm, Ls, U, C, Ug, D, H, V};

pub struct SdkmanPlugin;

impl Plugin for SdkmanPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            // Main commands
            Box::new(Sdk),
            Box::new(List),
            Box::new(Install),
            Box::new(Uninstall),
            Box::new(Use),
            Box::new(Current),
            Box::new(Upgrade),
            Box::new(Default),
            Box::new(Home),
            Box::new(Version),
            Box::new(Env),
            Box::new(Update),
            Box::new(Flush),
            Box::new(Config),
            Box::new(Offline),
            // Aliases
            Box::new(Ls),
            Box::new(I),
            Box::new(Rm),
            Box::new(U),
            Box::new(C),
            Box::new(Ug),
            Box::new(D),
            Box::new(H),
            Box::new(V),
        ]
    }
}
