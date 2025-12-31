#!/bin/bash
#
# Generate Ed25519 keypair for Pro license signing
#
# This script generates a keypair for signing Pro licenses.
# The private key MUST be stored in a secure vault (1Password, AWS Secrets Manager, etc.)
# The public key will be embedded in the Pro license validator.
#
# Usage: ./scripts/generate_license_keypair.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
KEYS_DIR="$PROJECT_ROOT/.keys"

echo "🔐 MCP-Guard Pro License Keypair Generator"
echo "=========================================="
echo ""

# Create keys directory (gitignored)
mkdir -p "$KEYS_DIR"

# Check if keys already exist
if [ -f "$KEYS_DIR/pro_license_private.pem" ]; then
    echo "⚠️  WARNING: Keypair already exists!"
    echo "   Private key: $KEYS_DIR/pro_license_private.pem"
    echo "   Public key:  $KEYS_DIR/pro_license_public.pem"
    echo ""
    read -p "Overwrite existing keys? (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        echo "Aborted."
        exit 0
    fi
fi

# Generate Ed25519 keypair using OpenSSL
echo "Generating Ed25519 keypair..."
openssl genpkey -algorithm ED25519 -out "$KEYS_DIR/pro_license_private.pem"
openssl pkey -in "$KEYS_DIR/pro_license_private.pem" -pubout -out "$KEYS_DIR/pro_license_public.pem"

# Extract base64-encoded public key for Rust code
PUBLIC_KEY_BASE64=$(grep -v "BEGIN\|END" "$KEYS_DIR/pro_license_public.pem" | tr -d '\n')

echo ""
echo "✅ Keypair generated successfully!"
echo ""
echo "📁 Files created:"
echo "   Private key: $KEYS_DIR/pro_license_private.pem"
echo "   Public key:  $KEYS_DIR/pro_license_public.pem"
echo ""
echo "🔒 CRITICAL SECURITY STEPS:"
echo ""
echo "1. Store private key in secure vault immediately:"
echo "   - 1Password: Create secure note with private key content"
echo "   - AWS Secrets Manager: aws secretsmanager create-secret"
echo "   - HashiCorp Vault: vault kv put secret/mcp-guard/pro-license-key"
echo ""
echo "2. Delete local private key after storing:"
echo "   shred -u $KEYS_DIR/pro_license_private.pem"
echo ""
echo "3. Update Pro license validator with this public key:"
echo "   File: crates/mcp-guard-pro/src/license.rs"
echo "   Replace PRO_LICENSE_PUBLIC_KEY with:"
echo ""
echo "   const PRO_LICENSE_PUBLIC_KEY: &str = \"$PUBLIC_KEY_BASE64\";"
echo ""
echo "4. Never commit private key to git!"
echo "   (Already protected by .gitignore)"
echo ""
echo "📝 To sign a license with this key, use:"
echo "   cargo run --bin sign-pro-license -- --user-id user@example.com"
echo ""
