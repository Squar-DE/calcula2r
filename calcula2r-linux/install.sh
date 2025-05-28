#!/bin/sh

BIN_NAME="calcula2r"
INSTALL_DIR="/usr/local/bin"

if [ "$(id -u)" -ne 0 ]; then
    echo "Please run this script with sudo:"
    echo "  sudo ./install.sh"
    exit 1
fi

# Extract the binary
tar -xzf "$BIN_NAME.tar.gz" || { echo "Extraction failed"; exit 1; }

# Move and install
mv "$BIN_NAME" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/$BIN_NAME"

echo "$BIN_NAME installed successfully! Run '$BIN_NAME' to start."

