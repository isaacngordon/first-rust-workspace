#!/bin/bash

# Function to display an error and exit
error_exit()
{
    echo "$1" 1>&2
    exit 1
}

# Check if cargo is installed
command -v cargo >/dev/null 2>&1 || error_exit "Cargo is required, but it's not installed. Aborting."

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

