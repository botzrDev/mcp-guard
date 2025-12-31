#!/bin/bash
#
# MCP-Guard Pro Installation Script
#
# Quick install:
#   curl -fsSL https://mcp-guard.io/install-pro.sh | bash
#
# With license key:
#   curl -fsSL https://mcp-guard.io/install-pro.sh | MCP_GUARD_LICENSE_KEY=pro_xxx bash
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOWNLOAD_URL="https://download.mcp-guard.io/download"
TIER="pro"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="mcp-guard"

# Print colored output
print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1" >&2
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_header() {
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  MCP-Guard Pro Installer${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

# Detect platform
detect_platform() {
    local os
    local arch
    local platform

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)
            case "$arch" in
                x86_64)
                    # Check for musl vs glibc
                    if ldd /bin/ls 2>/dev/null | grep -q musl; then
                        platform="x86_64-linux-musl"
                    else
                        platform="x86_64-linux"
                    fi
                    ;;
                *)
                    print_error "Unsupported architecture: $arch"
                    print_info "Supported architectures: x86_64"
                    exit 1
                    ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64)
                    platform="x86_64-darwin"
                    ;;
                arm64)
                    platform="aarch64-darwin"
                    ;;
                *)
                    print_error "Unsupported architecture: $arch"
                    exit 1
                    ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*)
            print_error "Windows installation not supported via this script"
            print_info "Please download manually from: https://mcp-guard.io/downloads"
            exit 1
            ;;
        *)
            print_error "Unsupported operating system: $os"
            print_info "Supported systems: Linux, macOS"
            exit 1
            ;;
    esac

    echo "$platform"
}

# Get license key
get_license_key() {
    local license_key

    # Check environment variable
    if [ -n "$MCP_GUARD_LICENSE_KEY" ]; then
        license_key="$MCP_GUARD_LICENSE_KEY"
    else
        # Prompt user
        echo ""
        print_info "Pro license required for installation"
        print_info "Don't have a license? Get one at: ${BLUE}https://mcp-guard.io/pricing${NC}"
        echo ""
        read -r -p "Enter your Pro license key (pro_xxx...): " license_key
        echo ""
    fi

    # Validate format
    if [[ ! "$license_key" =~ ^pro_ ]]; then
        print_error "Invalid license key format (must start with 'pro_')"
        exit 1
    fi

    echo "$license_key"
}

# Download binary
download_binary() {
    local platform="$1"
    local license_key="$2"
    local temp_file
    local url

    temp_file=$(mktemp)
    url="${DOWNLOAD_URL}?tier=${TIER}&platform=${platform}&license=${license_key}"

    print_info "Downloading MCP-Guard Pro for ${platform}..."

    if command -v curl &> /dev/null; then
        if ! curl -fsSL -o "$temp_file" "$url"; then
            print_error "Download failed"
            print_info "Please check your license key and try again"
            rm -f "$temp_file"
            exit 1
        fi
    elif command -v wget &> /dev/null; then
        if ! wget -q -O "$temp_file" "$url"; then
            print_error "Download failed"
            print_info "Please check your license key and try again"
            rm -f "$temp_file"
            exit 1
        fi
    else
        print_error "curl or wget is required but not installed"
        exit 1
    fi

    # Check if download was successful (file should be binary, not JSON error)
    if file "$temp_file" | grep -q "JSON"; then
        print_error "Download failed - received error response:"
        cat "$temp_file"
        rm -f "$temp_file"
        exit 1
    fi

    echo "$temp_file"
}

# Install binary
install_binary() {
    local temp_file="$1"
    local install_path="${INSTALL_DIR}/${BINARY_NAME}"

    print_info "Installing to ${install_path}..."

    # Check if we need sudo
    if [ -w "$INSTALL_DIR" ]; then
        mv "$temp_file" "$install_path"
        chmod +x "$install_path"
    else
        print_warning "Elevated privileges required for installation"
        sudo mv "$temp_file" "$install_path"
        sudo chmod +x "$install_path"
    fi

    print_success "Installed to ${install_path}"
}

# Verify installation
verify_installation() {
    local install_path="${INSTALL_DIR}/${BINARY_NAME}"

    print_info "Verifying installation..."

    if [ ! -x "$install_path" ]; then
        print_error "Binary not found or not executable at ${install_path}"
        exit 1
    fi

    # Run version command
    if ! "$install_path" version &> /dev/null; then
        print_error "Binary installed but failed to run"
        print_info "Try running: ${install_path} version"
        exit 1
    fi

    print_success "Installation verified"
}

# Save license to config
save_license_config() {
    local license_key="$1"
    local config_dir="$HOME/.config/mcp-guard"
    local env_file="${config_dir}/.env"

    print_info "Saving license key..."

    mkdir -p "$config_dir"

    # Create .env file with license
    cat > "$env_file" <<EOF
# MCP-Guard Pro License Key
# Generated by installer on $(date)
MCP_GUARD_LICENSE_KEY=${license_key}
EOF

    chmod 600 "$env_file"

    print_success "License saved to ${env_file}"
    print_info "To use MCP-Guard Pro, source this file:"
    echo ""
    echo -e "  ${GREEN}source ${env_file}${NC}"
    echo ""
    print_info "Or add to your shell profile (~/.bashrc, ~/.zshrc):"
    echo ""
    echo -e "  ${GREEN}export MCP_GUARD_LICENSE_KEY='${license_key}'${NC}"
    echo ""
}

# Print next steps
print_next_steps() {
    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}  Installation Complete!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    print_success "MCP-Guard Pro is now installed"
    echo ""
    echo -e "${BLUE}Next Steps:${NC}"
    echo ""
    echo "  1. Verify installation:"
    echo -e "     ${GREEN}mcp-guard version${NC}"
    echo ""
    echo "  2. Generate configuration:"
    echo -e "     ${GREEN}mcp-guard init${NC}"
    echo ""
    echo "  3. Edit configuration:"
    echo -e "     ${GREEN}\$EDITOR mcp-guard.toml${NC}"
    echo ""
    echo "  4. Start MCP-Guard:"
    echo -e "     ${GREEN}mcp-guard run${NC}"
    echo ""
    echo -e "${BLUE}Documentation:${NC}"
    echo "  https://mcp-guard.io/docs/pro"
    echo ""
    echo -e "${BLUE}Support:${NC}"
    echo "  support@mcp-guard.io"
    echo ""
}

# Main installation flow
main() {
    print_header

    # Detect platform
    print_info "Detecting platform..."
    platform=$(detect_platform)
    print_success "Detected platform: ${platform}"

    # Get license key
    license_key=$(get_license_key)

    # Download binary
    temp_file=$(download_binary "$platform" "$license_key")
    print_success "Downloaded successfully"

    # Install binary
    install_binary "$temp_file"

    # Verify installation
    verify_installation

    # Save license configuration
    save_license_config "$license_key"

    # Print next steps
    print_next_steps
}

# Run main function
main "$@"
