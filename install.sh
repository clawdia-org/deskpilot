#!/bin/sh
# Deskpilot installer script
# Usage: curl -fsSL https://raw.githubusercontent.com/clawdia-org/deskpilot/main/install.sh | sh

set -e

REPO="clawdia-org/deskpilot"
BINARY_NAME="deskpilot"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    echo "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Darwin*)    echo "apple-darwin" ;;
        Linux*)     echo "unknown-linux-gnu" ;;
        CYGWIN*|MINGW*|MSYS*)    echo "pc-windows-msvc" ;;
        *)          error "Unsupported OS: $(uname -s)" ;;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "x86_64" ;;
        arm64|aarch64)  echo "aarch64" ;;
        *)              error "Unsupported architecture: $(uname -m)" ;;
    esac
}

# Get latest release version
get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
}

# Download and verify binary
download_binary() {
    VERSION="$1"
    TARGET="$2"
    ARTIFACT="${BINARY_NAME}-${TARGET}"

    if [ "$(detect_os)" = "pc-windows-msvc" ]; then
        EXT="zip"
    else
        EXT="tar.gz"
    fi

    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARTIFACT}.${EXT}"
    CHECKSUM_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARTIFACT}.${EXT}.sha256"

    info "Downloading ${ARTIFACT}.${EXT}..."
    curl -fsSL -o "/tmp/${ARTIFACT}.${EXT}" "${DOWNLOAD_URL}"

    # Download checksum
    CHECKSUM_FILE="/tmp/${ARTIFACT}.${EXT}.sha256"
    if curl -fsSL -o "${CHECKSUM_FILE}" "${CHECKSUM_URL}" 2>/dev/null; then
        info "Verifying checksum..."
        cd /tmp
        if command -v sha256sum >/dev/null 2>&1; then
            sha256sum -c "${CHECKSUM_FILE}" || error "Checksum verification failed"
        elif command -v shasum >/dev/null 2>&1; then
            shasum -a 256 -c "${CHECKSUM_FILE}" || error "Checksum verification failed"
        else
            warn "sha256sum not found, skipping checksum verification"
        fi
    else
        warn "Checksum file not found, skipping verification"
    fi

    # Extract
    info "Extracting..."
    cd /tmp
    if [ "${EXT}" = "zip" ]; then
        unzip -o "${ARTIFACT}.${EXT}"
    else
        tar -xzf "${ARTIFACT}.${EXT}"
    fi
}

# Install binary
install_binary() {
    if [ ! -f "/tmp/${BINARY_NAME}" ] && [ ! -f "/tmp/${BINARY_NAME}.exe" ]; then
        error "Binary not found after extraction"
    fi

    # Determine binary path
    if [ -f "/tmp/${BINARY_NAME}.exe" ]; then
        BINARY_PATH="/tmp/${BINARY_NAME}.exe"
        DEST="${INSTALL_DIR}/${BINARY_NAME}.exe"
    else
        BINARY_PATH="/tmp/${BINARY_NAME}"
        DEST="${INSTALL_DIR}/${BINARY_NAME}"
    fi

    # Check if we can write to INSTALL_DIR
    if [ ! -w "${INSTALL_DIR}" ]; then
        warn "Cannot write to ${INSTALL_DIR}, using ~/.local/bin instead"
        INSTALL_DIR="${HOME}/.local/bin"
        mkdir -p "${INSTALL_DIR}"
        if [ -f "/tmp/${BINARY_NAME}.exe" ]; then
            DEST="${INSTALL_DIR}/${BINARY_NAME}.exe"
        else
            DEST="${INSTALL_DIR}/${BINARY_NAME}"
        fi
    fi

    info "Installing to ${DEST}..."
    cp "${BINARY_PATH}" "${DEST}"
    chmod +x "${DEST}"

    # Check if INSTALL_DIR is in PATH
    case ":${PATH}:" in
        *":${INSTALL_DIR}:"*) ;;
        *)
            warn "${INSTALL_DIR} is not in your PATH"
            echo ""
            echo "Add the following to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
            echo ""
            echo "    export PATH=\"\${PATH}:${INSTALL_DIR}\""
            echo ""
            ;;
    esac
}

# Main
main() {
    info "Installing deskpilot..."

    OS_SUFFIX=$(detect_os)
    ARCH=$(detect_arch)
    TARGET="${ARCH}-${OS_SUFFIX}"

    info "Detected target: ${TARGET}"

    VERSION="${VERSION:-$(get_latest_version)}"
    if [ -z "${VERSION}" ]; then
        error "Could not determine latest version. Set VERSION environment variable."
    fi

    info "Installing version: ${VERSION}"

    download_binary "${VERSION}" "${TARGET}"
    install_binary

    info "Installation complete!"
    info "Run 'deskpilot --help' to get started."
}

main "$@"
