#!/bin/bash
#
# MCP-Guard Enterprise Installation Script
#
# Quick install:
#   curl -fsSL https://mcp-guard.io/install-enterprise.sh | bash
#
# With license key:
#   curl -fsSL https://mcp-guard.io/install-enterprise.sh | MCP_GUARD_LICENSE_KEY=ent_xxx bash
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Configuration
DOWNLOAD_URL="https://download.mcp-guard.io/download"
TIER="enterprise"
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

print_enterprise() {
    echo -e "${PURPLE}★${NC} $1"
}

print_header() {
    echo ""
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${PURPLE}  MCP-Guard Enterprise Installer${NC}"
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
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
        print_enterprise "Enterprise license required for installation"
        print_info "Don't have a license? Contact sales: ${PURPLE}sales@mcp-guard.io${NC}"
        echo ""
        read -r -p "Enter your Enterprise license key (ent_xxx...): " license_key
        echo ""
    fi

    # Validate format
    if [[ ! "$license_key" =~ ^ent_ ]]; then
        print_error "Invalid license key format (must start with 'ent_')"
        print_info "Enterprise licenses begin with 'ent_'"
        print_info "If you have a Pro license (pro_xxx), use install-pro.sh instead"
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

    print_info "Downloading MCP-Guard Enterprise for ${platform}..."
    print_enterprise "Validating license via Keygen.sh..."

    if command -v curl &> /dev/null; then
        if ! curl -fsSL -o "$temp_file" "$url"; then
            print_error "Download failed"
            print_info "Please check your license key and network connection"
            print_info "Enterprise licenses require online validation"
            rm -f "$temp_file"
            exit 1
        fi
    elif command -v wget &> /dev/null; then
        if ! wget -q -O "$temp_file" "$url"; then
            print_error "Download failed"
            print_info "Please check your license key and network connection"
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
# MCP-Guard Enterprise License Key
# Generated by installer on $(date)
#
# IMPORTANT: Enterprise licenses require online validation via Keygen.sh
# Ensure your servers can reach api.keygen.sh for license validation
# Offline grace period: 30 days
MCP_GUARD_LICENSE_KEY=${license_key}
EOF

    chmod 600 "$env_file"

    print_success "License saved to ${env_file}"
    print_info "To use MCP-Guard Enterprise, source this file:"
    echo ""
    echo -e "  ${GREEN}source ${env_file}${NC}"
    echo ""
    print_info "Or add to your shell profile (~/.bashrc, ~/.zshrc):"
    echo ""
    echo -e "  ${GREEN}export MCP_GUARD_LICENSE_KEY='${license_key}'${NC}"
    echo ""
}

# Print Enterprise features
print_enterprise_features() {
    echo ""
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${PURPLE}  Enterprise Features Enabled${NC}"
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    print_enterprise "mTLS (Mutual TLS) authentication"
    print_enterprise "Multi-server routing"
    print_enterprise "SIEM integration (Splunk, Datadog)"
    print_enterprise "OpenTelemetry distributed tracing"
    print_enterprise "Per-tool rate limiting"
    print_enterprise "Admin CLI tools"
    print_enterprise "Priority support (4-hour SLA)"
    echo ""
}

# Print next steps
print_next_steps() {
    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}  Installation Complete!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    print_success "MCP-Guard Enterprise is now installed"
    echo ""

    print_enterprise_features

    echo -e "${BLUE}Quick Start:${NC}"
    echo ""
    echo "  1. Verify installation:"
    echo -e "     ${GREEN}mcp-guard version${NC}"
    echo ""
    echo "  2. Generate Enterprise configuration:"
    echo -e "     ${GREEN}mcp-guard init --enterprise${NC}"
    echo ""
    echo "  3. Edit configuration:"
    echo -e "     ${GREEN}\$EDITOR mcp-guard.toml${NC}"
    echo ""
    echo "  4. Start MCP-Guard:"
    echo -e "     ${GREEN}mcp-guard run${NC}"
    echo ""
    echo -e "${BLUE}Enterprise Documentation:${NC}"
    echo "  • Getting Started: https://mcp-guard.io/docs/enterprise"
    echo "  • mTLS Setup:      https://mcp-guard.io/docs/enterprise/mtls"
    echo "  • Multi-Server:    https://mcp-guard.io/docs/enterprise/multi-server"
    echo "  • SIEM Integration: https://mcp-guard.io/docs/enterprise/siem"
    echo ""
    echo -e "${BLUE}Enterprise Support:${NC}"
    echo "  • Email:  support@mcp-guard.io (4-hour SLA)"
    echo "  • Slack:  enterprise.mcp-guard.io/slack"
    echo "  • Phone:  Available in customer portal"
    echo ""
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${PURPLE}  Thank you for choosing MCP-Guard Enterprise!${NC}"
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

# Check network connectivity
check_network() {
    print_info "Checking network connectivity to Keygen.sh..."

    if command -v curl &> /dev/null; then
        if curl -fsSL --max-time 5 https://api.keygen.sh &> /dev/null; then
            print_success "Network connectivity OK"
        else
            print_warning "Cannot reach api.keygen.sh"
            print_info "Enterprise licenses require online validation"
            print_info "Ensure your firewall allows HTTPS access to api.keygen.sh"
            echo ""
            read -r -p "Continue anyway? (y/N): " continue_install
            if [[ ! "$continue_install" =~ ^[Yy]$ ]]; then
                print_error "Installation cancelled"
                exit 1
            fi
        fi
    fi
}

# Main installation flow
main() {
    print_header

    # Check network connectivity
    check_network

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
