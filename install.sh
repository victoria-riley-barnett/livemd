#!/bin/bash

# livemd installer script
# Downloads and installs the livemd binary

set -e

REPO="victoria-riley-barnett/livemd"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    echo -e "${GREEN}ℹ️  $1${NC}"
}

warn() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
}

# Detect platform and architecture
detect_platform() {
    case "$(uname -s)" in
        Linux)
            PLATFORM="linux"
            ;;
        Darwin)
            PLATFORM="macos"
            ;;
        *)
            error "Unsupported platform: $(uname -s)"
            exit 1
            ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)
            ARCH="x64"
            ;;
        aarch64|arm64)
            ARCH="arm64"
            ;;
        *)
            error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
}

# Get the latest release version
get_latest_version() {
    info "Fetching latest version..."
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$VERSION" ]; then
        error "Failed to get latest version"
        exit 1
    fi
    info "Latest version: $VERSION"
}

# Download and install binary
install_binary() {
    BINARY_NAME="livemd-${PLATFORM}-${ARCH}"
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME.tar.gz"

    info "Downloading $BINARY_NAME..."
    if ! curl -L -o "/tmp/$BINARY_NAME.tar.gz" "$DOWNLOAD_URL"; then
        error "Failed to download binary"
        exit 1
    fi

    info "Installing to $INSTALL_DIR..."
    if [ ! -w "$INSTALL_DIR" ]; then
        warn "Need sudo to install to $INSTALL_DIR"
        sudo mkdir -p "$INSTALL_DIR"
        sudo tar -xzf "/tmp/$BINARY_NAME.tar.gz" -C "$INSTALL_DIR"
        sudo chmod +x "$INSTALL_DIR/livemd"
    else
        mkdir -p "$INSTALL_DIR"
        tar -xzf "/tmp/$BINARY_NAME.tar.gz" -C "$INSTALL_DIR"
        chmod +x "$INSTALL_DIR/livemd"
    fi

    # Clean up
    rm "/tmp/$BINARY_NAME.tar.gz"

    info "Installation complete!"
    info "Run 'livemd --help' to get started"
}

main() {
    info "livemd-rs installer"
    echo

    detect_platform
    info "Detected platform: $PLATFORM-$ARCH"

    get_latest_version
    install_binary

    echo
    info "Installation successful! 🎉"
    echo
    echo "Usage examples:"
    echo "  livemd explain rust ownership    # AI query (no quotes needed!)"
    echo "  livemd --file README.md"
    echo "  livemd --cmd 'cat file.md'"
    
    # Note about zsh globbing
    if [ "$SHELL" = "/bin/zsh" ] || [ "$SHELL" = "/usr/bin/zsh" ] || [ -n "$ZSH_VERSION" ]; then
        echo
        echo "💡 Zsh users: For queries with ?, *, [, ] characters, use quotes:"
        echo "   livemd \"what is gnosticism?\""
        echo "   # Or use: noglob livemd what is gnosticism?"
    fi
}

main "$@"