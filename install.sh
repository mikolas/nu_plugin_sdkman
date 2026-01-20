#!/usr/bin/env bash
# SDKMAN! Nushell Plugin Installer

set -e

REPO="YOUR_GITHUB_USERNAME/nu_plugin_sdkman"
INSTALL_DIR="${HOME}/.local/bin"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "${OS}" in
    linux)
        case "${ARCH}" in
            x86_64) PLATFORM="linux-x86_64" ;;
            aarch64|arm64) PLATFORM="linux-aarch64" ;;
            *) echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
        esac
        BINARY_NAME="nu_plugin_sdkman"
        ;;
    darwin)
        case "${ARCH}" in
            x86_64) PLATFORM="darwin-x86_64" ;;
            arm64) PLATFORM="darwin-aarch64" ;;
            *) echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
        esac
        BINARY_NAME="nu_plugin_sdkman"
        ;;
    *)
        echo "Unsupported OS: ${OS}"
        exit 1
        ;;
esac

echo "Installing SDKMAN! Nushell Plugin for ${PLATFORM}..."

# Get latest release
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "${LATEST_RELEASE}" ]; then
    echo "Failed to get latest release"
    exit 1
fi

echo "Latest version: ${LATEST_RELEASE}"

# Download binary
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_RELEASE}/nu_plugin_sdkman-${PLATFORM}"
TEMP_FILE=$(mktemp)

echo "Downloading from ${DOWNLOAD_URL}..."
curl -L -o "${TEMP_FILE}" "${DOWNLOAD_URL}"

# Install
mkdir -p "${INSTALL_DIR}"
mv "${TEMP_FILE}" "${INSTALL_DIR}/${BINARY_NAME}"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

echo "✓ Binary installed to ${INSTALL_DIR}/${BINARY_NAME}"

# Check if in PATH
if ! echo "${PATH}" | grep -q "${INSTALL_DIR}"; then
    echo ""
    echo "⚠️  ${INSTALL_DIR} is not in your PATH"
    echo "Add this to your shell config:"
    echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
fi

echo ""
echo "To complete installation, run in Nushell:"
echo "  plugin add ${INSTALL_DIR}/${BINARY_NAME}"
echo ""
echo "Then restart Nushell and run: sdk"
