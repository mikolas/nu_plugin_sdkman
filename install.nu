#!/usr/bin/env nu
# SDKMAN! Nushell Plugin Installer

const REPO = "mikolas/nu_plugin_sdkman"

def get-install-dir [] {
    $env.HOME | path join ".local" "bin"
}

# Detect platform
def detect-platform [] {
    let os = (sys host | get name)
    let arch = (sys host | get cpu | first | get arch)
    
    match [$os, $arch] {
        ["Linux", "x86_64"] => "x86_64-unknown-linux-gnu"
        ["Linux", "aarch64"] => "aarch64-unknown-linux-gnu"
        ["Darwin", "x86_64"] => "x86_64-apple-darwin"
        ["Darwin", "aarch64"] => "aarch64-apple-darwin"
        _ => (error make {msg: $"Unsupported platform: ($os) ($arch)"})
    }
}

print "Installing SDKMAN! Nushell Plugin..."

let platform = (detect-platform)
print $"Platform: ($platform)"

# Get latest release
let releases_url = $"https://api.github.com/repos/($REPO)/releases/latest"
let release = (http get $releases_url)
let version = $release.tag_name

print $"Latest version: ($version)"

# Download binary
let binary_name = $"nu_plugin_sdkman-($platform)"
let download_url = $"https://github.com/($REPO)/releases/download/($version)/($binary_name)"

print $"Downloading from ($download_url)..."

let temp_file = (mktemp)
http get $download_url | save -f $temp_file

# Install
let install_dir = (get-install-dir)
mkdir $install_dir
let install_path = ($install_dir | path join "nu_plugin_sdkman")

# Move and make executable (cross-platform)
mv -f $temp_file $install_path
if (sys host | get name) != "Windows" {
    ^chmod +x $install_path
}

print $"✓ Binary installed to ($install_path)"

# Register plugin
print "\nRegistering plugin with Nushell..."
try {
    plugin add $install_path
    print "✓ Plugin registered"
} catch {
    print $"⚠️  Failed to register plugin. Run manually: plugin add ($install_path)"
}

print "\nInstallation complete! Restart Nushell and run: sdk"
