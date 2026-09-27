#!/bin/bash
set -e

echo "Installing jev CLI..."

# Determine OS and Arch
OS="$(uname -s)"
ARCH="$(uname -m)"

if [ "$OS" != "Linux" ]; then
    echo "Error: This script is only for Linux/WSL environments."
    exit 1
fi

if [ "$ARCH" = "x86_64" ]; then
    TARGET="x86_64-unknown-linux-gnu"
else
    echo "Error: Unsupported architecture: $ARCH. Only x86_64 is supported currently."
    exit 1
fi

# Fetch the latest release URL from GitHub API
REPO="Salv4d/jev-cli"
echo "Fetching latest release information from GitHub..."

# Extract the browser_download_url for the linux tar.gz file
LATEST_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url.*$TARGET.tar.gz" | cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
    echo "Error: Failed to find the latest release for $TARGET."
    exit 1
fi

echo "Downloading $LATEST_URL..."
curl -sL "$LATEST_URL" -o jev-cli.tar.gz

echo "Extracting binary..."
tar -xzf jev-cli.tar.gz

# We install to ~/.local/bin so we don't need sudo, which is safer and common for user tools
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Move and rename the binary to "jev"
mv jev-cli "$INSTALL_DIR/jev"
chmod +x "$INSTALL_DIR/jev"

# Clean up
rm jev-cli.tar.gz

echo "--------------------------------------------------------"
echo "✓ Successfully installed 'jev' to $INSTALL_DIR/jev"
echo ""
echo "Make sure $INSTALL_DIR is in your PATH."
echo "If it's not, you can add it by appending this line to your ~/.bashrc or ~/.zshrc:"
echo 'export PATH="$HOME/.local/bin:$PATH"'
echo ""
echo "Run 'jev --help' to get started!"
echo "--------------------------------------------------------"
