#!/bin/bash

# Function to display an error and exit
error_exit()
{
    echo "$1" 1>&2
    exit 1
}

# Check if cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo is not installed. Attempting to install Rust..."

    # Installing Rust via rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # Check if the Rust installation was successful
    if [ $? -ne 0 ]; then
        error_exit "Failed to install Rust. Aborting."
    fi

    # Source the cargo environment script for immediate use of cargo
    source $HOME/.cargo/env
fi

# Build the project in release mode
echo "Building the oxygen project..."
cargo build --release || error_exit "Cargo build failed. Aborting."

# Determine the platform
PLATFORM="$(uname -s)"
case "${PLATFORM}" in
    Linux*|Darwin*) # Linux or Mac
        # Set target directory
        TARGET_DIR="/usr/local/bin"
        
        # Check if target directory is in PATH
        if [[ ":$PATH:" != *":$TARGET_DIR:"* ]]; then
            echo "Warning: Your path does not include ${TARGET_DIR}, the executable might not run from anywhere."
        fi

        # Move the binary
        echo "Installing oxygen to ${TARGET_DIR}"
        sudo mv "target/release/oxygen" "${TARGET_DIR}/oxygen" || error_exit "Failed to move the binary. Aborting."
        ;;
    CYGWIN*|MINGW*|MSYS*) # Windows (Cygwin, MinGW, MSYS)
        echo "Windows detected. Manual installation required."
        echo "Move target/release/oxygen.exe to a directory in your PATH, e.g., C:\\Program Files."
        ;;
    *)
        error_exit "Unsupported platform: ${PLATFORM}"
        ;;
esac

echo "Installation complete. You can now run oxygen from anywhere."

