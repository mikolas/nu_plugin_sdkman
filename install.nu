#!/usr/bin/env nu
# SDKMAN! Nushell Plugin Installer

const REPO = "YOUR_GITHUB_USERNAME/nu_plugin_sdkman"
const INSTALL_DIR = ($env.HOME | path join ".local" "bin")

# Detect platform
def detect-platform [] {
    let os = (sys host | get name)
    let arch = (sys host | get cpu | first | get arch)
    
    match [$os, $arch] {
        ["Linux", "x86_64"] => "linux-x86_64"
        ["Linux", "aarch64"] => "linux-aarch64"
        ["Darwin", "x86_64"] => "darwin-x86_64"
        ["Darwin", "aarch64"] => "darwin-aarch64"
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
mkdir $INSTALL_DIR
let install_path = ($INSTALL_DIR | path join "nu_plugin_sdkman")
mv -f $temp_file $install_path
chmod +x $install_path

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
