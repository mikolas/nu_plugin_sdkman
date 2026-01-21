#!/usr/bin/env nu

# Build and install SDKMAN! Nushell plugin

print "Building SDKMAN! plugin for Nushell 0.110.0..."

# Build in release mode
cargo build --release

if $env.LAST_EXIT_CODE != 0 {
    print "Build failed!"
    exit 1
}

print "Build successful!"
print ""

let plugin_path = ("target/release/nu_plugin_sdkman" | path expand)

print $"Plugin binary: ($plugin_path)"
print ""
print "To install the plugin, run:"
print ""
print $"  plugin add ($plugin_path)"
print ""
print "Then restart Nushell and run: sdk"
