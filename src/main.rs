use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_sdkman::SdkmanPlugin;

fn main() {
    serve_plugin(&SdkmanPlugin, MsgPackSerializer);
}
