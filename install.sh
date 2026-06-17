#!/bin/sh
set -e

# Linux Soft Cleaner Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/anilcan-kara/linux-soft-cleaner/master/install.sh | sh

REPO="anilcan-kara/linux-soft-cleaner"
VERSION="0.1.0"

echo "Detecting system architecture..."
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)
    case "$ARCH" in
      x86_64) TARGET="x86_64-unknown-linux-musl" ;;
      aarch64|arm64) TARGET="aarch64-unknown-linux-musl" ;;
      *) echo "Error: Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  darwin)
    case "$ARCH" in
      x86_64) TARGET="x86_64-apple-darwin" ;;
      aarch64|arm64) TARGET="aarch64-apple-darwin" ;;
      *) echo "Error: Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Error: Unsupported operating system: $OS"
    exit 1
    ;;
esac

URL="https://github.com/$REPO/releases/download/v$VERSION/linux-soft-cleaner-$TARGET.tar.gz"
echo "Downloading linux-soft-cleaner for $TARGET..."
TEMP_DIR=$(mktemp -d)
curl -L "$URL" -o "$TEMP_DIR/linux-soft-cleaner.tar.gz"

echo "Extracting..."
tar -xzf "$TEMP_DIR/linux-soft-cleaner.tar.gz" -C "$TEMP_DIR"

INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
  echo "No write permission to $INSTALL_DIR. Attempting to install to $HOME/.local/bin..."
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

mv "$TEMP_DIR/linux-soft-cleaner" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/linux-soft-cleaner"

# Cleanup
rm -rf "$TEMP_DIR"

echo "Installation complete! linux-soft-cleaner installed to $INSTALL_DIR/linux-soft-cleaner"
if [ "$INSTALL_DIR" = "$HOME/.local/bin" ]; then
  echo "Make sure $HOME/.local/bin is in your PATH."
fi
