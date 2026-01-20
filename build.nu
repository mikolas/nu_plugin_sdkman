#!/usr/bin/env nu

# Build and install SDKMAN! Nushell plugin

print "Building SDKMAN! plugin for Nushell 0.110.0..."

cd sdkman-plugin

# Build in release mode
cargo build --release

if $env.LAST_EXIT_CODE != 0 {
    print "Build failed!"
    exit 1
}

print "Build successful!"
print ""

let plugin_path = (
    "target/release/nu_plugin_sdkman" 
    | path expand
    | path join $env.PWD
)

print $"Plugin binary: ($plugin_path)"
print ""
print "To install the plugin, run:"
print ""
print $"  plugin add ($plugin_path)"
print ""
print "Then restart Nushell or run:"
print ""
print "  plugin use sdkman"
print ""
print "Available commands:"
print "  sdk list [candidate]       - List candidates or versions"
print "  sdk install <candidate> [version] - Install SDK"
print "  sdk use <candidate> <version>     - Set current version"
print "  sdk current [candidate]    - Show current version"
print "  sdk uninstall <candidate> <version> - Remove SDK"
print "  sdk upgrade [candidate]    - Upgrade to latest"
