#!/usr/bin/env bash
# Grok Build Community Edition - macOS/Linux Installer
# Run: curl -fsSL https://raw.githubusercontent.com/Cashmeran/grok-build-community/main/install.sh | bash

set -e

REPO="Cashmeran/grok-build-community"
INSTALL_DIR="$HOME/.grok/bin"

echo "Grok Build Community Edition Installer"
echo "====================================="
echo ""

mkdir -p "$INSTALL_DIR"

# Detect platform
case "$(uname -s)" in
    Linux)   PLATFORM="linux";;
    Darwin)  PLATFORM="macos|darwin";;
    *)       echo "Unsupported platform: $(uname -s)"; exit 1;;
esac

case "$(uname -m)" in
    x86_64|amd64)  ARCH="x86_64|amd64";;
    aarch64|arm64) ARCH="aarch64|arm64";;
    *)             echo "Unsupported architecture: $(uname -m)"; exit 1;;
esac

echo "Fetching latest release..."
RELEASE_JSON=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null) || {
    echo "GitHub API rate limited. Try again later or set GITHUB_TOKEN."
    echo "Manual download: https://github.com/$REPO/releases/latest"
    exit 1
}

DOWNLOAD_URL=$(echo "$RELEASE_JSON" | grep -o "\"browser_download_url\": *\"[^\"]*\"" | grep -iE "$PLATFORM" | grep -iE "$ARCH" | head -1 | cut -d'"' -f4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "No matching binary found for your platform."
    exit 1
fi

FILENAME=$(basename "$DOWNLOAD_URL")
echo "Downloading $FILENAME..."
curl -fsSL -o "$INSTALL_DIR/$FILENAME" "$DOWNLOAD_URL"

# Handle archive formats
cd "$INSTALL_DIR"
case "$FILENAME" in
    *.tar.gz)  tar xzf "$FILENAME" && rm "$FILENAME";;
    *.zip)     unzip -o "$FILENAME" && rm "$FILENAME";;
esac

# Make binary executable
chmod +x "$INSTALL_DIR/grok" 2>/dev/null || true
chmod +x "$INSTALL_DIR/xai-grok-pager" 2>/dev/null || true

echo ""
echo "Installed to $INSTALL_DIR" | GREP_COLORS='ms=32' grep --color=always "." 2>/dev/null || echo "Installed to $INSTALL_DIR"

# PATH hint
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo ""
    echo "Add this to your shell config (~/.bashrc / ~/.zshrc):"
    echo "  export PATH=\"\$HOME/.grok/bin:\$PATH\""
fi

echo ""
echo "Grok Build Community Edition installed!"
echo "Run 'grok' to start."
