#!/bin/bash

PLATFORM="$(uname -s)"

# Function to display an error and exit
error_exit()
{
    echo "$1" 1>&2
    exit 1
}

# Function to install a package via apt on Linux, brew on Mac, or manually on Windows
install_with_local_pkg_manager()
{
    case "${PLATFORM}" in
        Linux*) # Linux
            sudo apt install "$1" || error_exit "Failed to install $1. Aborting."
            ;;
        Darwin*) # Mac
            brew install "$1" || error_exit "Failed to install $1. Aborting."
            ;;
        CYGWIN*|MINGW*|MSYS*) # Windows (Cygwin, MinGW, MSYS)
            echo "Windows detected. Manual installation required."
            echo "Download $1 from $2 and add it to your PATH."
            ;;
    esac
}

# Function to install a package via cargo
install_with_cargo()
{
    cargo install "$1" || error_exit "Failed to install $1. Aborting."
}

# --------------------------------------------------
# Check for and install fundamental tools/commands
# --------------------------------------------------

# Check if vim is installed, if not, install it
if ! command -v vim >/dev/null 2>&1; then
    echo "Vim is not installed. Attempting to install Vim..."
    install_with_local_pkg_manager vim "https://www.vim.org/download.php"
fi

# make sure vimrc exists and syntax highlighting is enabled
if [ ! -f "${HOME}/.vimrc" ]; then
    echo "Vimrc not found. Creating vimrc..."
fi 
if ! grep -q "syntax on" "${HOME}/.vimrc"; then
    echo "syntax on" >> "${HOME}/.vimrc"
fi
if ! grep -q "filetype plugin indent on" "${HOME}/.vimrc"; then
    echo "filetype plugin indent on" >> "${HOME}/.vimrc"
fi

# Check if git is installed, if not, install it
if ! command -v git >/dev/null 2>&1; then
    echo "Git is not installed. Attempting to install Git..."
    install_with_local_pkg_manager git "https://git-scm.com/downloads"
fi

# Check if cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo is not installed. Attempting to install Rust..."

    # Installing Rust via rustup
    case "${PLATFORM}" in
        Linux*) # Linux
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh || error_exit "Failed to install Rust. Aborting."
            ;;
        Darwin*) # Mac
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh || error_exit "Failed to install Rust. Aborting."
            ;;
        CYGWIN*|MINGW*|MSYS*) # Windows (Cygwin, MinGW, MSYS)
            echo "Windows detected. Manual installation required."
            echo "Download Rust from https://www.rust-lang.org/tools/install and add it to your PATH."
            ;;
    esac

    # Source the cargo environment script for immediate use of cargo
    case "${PLATFORM}" in
        Linux*|Darwin*) # Linux or Mac
            source "${HOME}/.cargo/env" || error_exit "Failed to source the cargo environment script. Aborting."
            ;;
        CYGWIN*|MINGW*|MSYS*) # Windows (Cygwin, MinGW, MSYS)
            echo "Windows detected. Manual installation required."
            echo "Source the cargo environment script for immediate use of cargo."
            ;;
    esac
fi

# Check if nu is installed. if not, install it
if ! command -v nu >/dev/null 2>&1; then
    echo "Nu is not installed. Attempting to install Nu..."
    install_with_cargo nu
fi

# cargo-info is a cargo plugin to display information about a crate
# Check if cargo-info is installed. if not, install it
if ! command -v cargo-info >/dev/null 2>&1; then
    echo "Cargo-info is not installed. Attempting to install Cargo-info..."
    install_with_cargo cargo-info
fi


# --------------------------------------------------
# Aliasable cli programs installed via cargo
# --------------------------------------------------

# exa is a modern replacement for ls
# Check if exa is installed. if not, install it
if ! command -v exa >/dev/null 2>&1; then
    echo "Exa is not installed. Attempting to install Exa..."
    install_with_cargo exa
fi

# bat is a modern replacement for cat, which facilitates syntax highlighting and falls back to plain cat if the file is not recognized
# Check if bat is installed. if not, install it
if ! command -v bat >/dev/null 2>&1; then
    echo "Bat is not installed. Attempting to install Bat..."
    install_with_cargo bat
fi

# ripgrep is a modern replacemend for find and grep
# Check if ripgrep is installed. if not, install it
if ! command -v rg >/dev/null 2>&1; then
    echo "Ripgrep is not installed. Attempting to install Ripgrep..."
    install_with_cargo ripgrep
fi

# speedtest-rs is a command line interface for testing internet bandwidth using speedtest.net
# Check if speedtest-rs is installed. if not, install it
if ! command -v speedtest-rs >/dev/null 2>&1; then
    echo "Speedtest-rs is not installed. Attempting to install Speedtest-rs..."
    install_with_cargo speedtest-rs
fi


# --------------------------------------------------
# Developer tools 
# --------------------------------------------------

# check for vscode and install it if it is not install using cargo
if ! command -v code >/dev/null 2>&1; then
#    echo "VSCode is not installed. Attempting to install VSCode..."
#    install_with_cargo vscode
    echo "Install VS Code manually"
fi

# bacon is a background rust compiler with hot reloading https://crates.io/crates/bacon
# Check if bacon is installed. if not, install it
if ! command -v bacon >/dev/null 2>&1; then
    echo "Bacon is not installed. Attempting to install Bacon..."
    install_with_cargo bacon
fi



# --------------------------------------------------
# Addtional tools/commands installed via cargo
# --------------------------------------------------

# du-dust is a visual disk-usage analyzer
# check if du-dust is installed. if not, install it
if ! command -v dust >/dev/null 2>&1; then
    echo "Dust is not installed. Attempting to install Dust..."
    install_with_cargo du-dust
fi

# mprocs is a tmux like tool toptimized for long-running, non-interacive processes
# Check if mprocs is installed. if not, install it
if ! command -v mprocs >/dev/null 2>&1; then
    echo "Mprocs is not installed. Attempting to install Mprocs..."
    install_with_cargo mprocs
fi

# irust is an interactive rust REPL
# Check if irust is installed. if not, install it
if ! command -v irust >/dev/null 2>&1; then
    echo "Irust is not installed. Attempting to install Irust..."
    install_with_cargo irust
fi

# similary, there is evcxr_jupyter for jupyter notebooks, which is a rust kernel for jupyter
# Check if evcxr_jupyter is installed. if not, install it
if ! command -v evcxr_jupyter >/dev/null 2>&1; then
    echo "Evcxr_jupyter is not installed. Attempting to install Evcxr_jupyter..."
    install_with_cargo evcxr_jupyter
fi

# wiki-tui is a terminal user interface for wikipedia
# Check if wiki-tui is installed. if not, install it
if ! command -v wiki-tui >/dev/null 2>&1; then
    echo "Wiki-tui is not installed. Attempting to install Wiki-tui..."
    install_with_cargo wiki-tui
fi


# # ncspot is a spotify client for the terminal
# # Check if ncspot is installed. if not, install it
# if ! command -v ncspot >/dev/null 2>&1; then
#     echo "Ncspot is not installed. Attempting to install Ncspot..."
#     install_with_cargo ncspot
# fi


# --------------------------------------------------
# Build the oxygen project in release mode
# --------------------------------------------------
echo "Building the oxygen project..."
cargo build --release || error_exit "Cargo build failed. Aborting."

# move the binary to the appropriate directory based on the platform
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

