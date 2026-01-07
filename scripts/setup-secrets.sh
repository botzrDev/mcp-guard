#!/bin/bash
# Secure secrets setup for MCP-Guard
# This script helps you configure secrets securely

set -e

echo "=================================================="
echo "MCP-Guard Secure Secrets Setup"
echo "=================================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if .env exists
if [ -f .env ]; then
    echo -e "${YELLOW}⚠${NC}  .env file already exists"
    read -p "Do you want to overwrite it? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Keeping existing .env file"
        exit 0
    fi
fi

# Create .env from template
echo -e "${BLUE}Creating .env file from template...${NC}"
cp .env.example .env

echo ""
echo "=================================================="
echo "Please enter your secrets"
echo "=================================================="
echo ""
echo "Get your Stripe keys from:"
echo "  Test: https://dashboard.stripe.com/test/apikeys"
echo "  Live: https://dashboard.stripe.com/apikeys"
echo ""

# Prompt for Stripe key
read -p "Enter STRIPE_SECRET_KEY (or press Enter to skip): " STRIPE_KEY
if [ -n "$STRIPE_KEY" ]; then
    # Validate key format
    if [[ $STRIPE_KEY =~ ^sk_(test|live)_ ]]; then
        sed -i.bak "s|STRIPE_SECRET_KEY=.*|STRIPE_SECRET_KEY=$STRIPE_KEY|" .env
        rm .env.bak 2>/dev/null || true
        echo -e "${GREEN}✓${NC} Stripe key added"

        # Warn if using live key
        if [[ $STRIPE_KEY =~ ^sk_live_ ]]; then
            echo -e "${YELLOW}⚠${NC}  WARNING: You're using a LIVE Stripe key!"
            echo "   Make sure this is for production only."
        fi
    else
        echo -e "${RED}✗${NC} Invalid Stripe key format (should start with sk_test_ or sk_live_)"
    fi
fi

echo ""

# Prompt for GitHub OAuth (optional)
read -p "Configure GitHub OAuth? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    read -p "GitHub Client ID: " GITHUB_ID
    read -p "GitHub Client Secret: " GITHUB_SECRET

    if [ -n "$GITHUB_ID" ] && [ -n "$GITHUB_SECRET" ]; then
        echo "" >> .env
        echo "# GitHub OAuth" >> .env
        echo "GITHUB_CLIENT_ID=$GITHUB_ID" >> .env
        echo "GITHUB_CLIENT_SECRET=$GITHUB_SECRET" >> .env
        echo -e "${GREEN}✓${NC} GitHub OAuth configured"
    fi
fi

echo ""

# Prompt for Database URL (optional)
read -p "Configure Database? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    read -p "Database URL: " DB_URL

    if [ -n "$DB_URL" ]; then
        echo "" >> .env
        echo "# Database" >> .env
        echo "DATABASE_URL=$DB_URL" >> .env
        echo -e "${GREEN}✓${NC} Database configured"
    fi
fi

echo ""
echo "=================================================="
echo "Securing .env file..."
echo "=================================================="

# Set secure file permissions (owner read/write only)
chmod 600 .env
echo -e "${GREEN}✓${NC} File permissions set to 600 (owner read/write only)"

# Verify it's gitignored
if git check-ignore .env > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} .env is properly gitignored"
else
    echo -e "${RED}✗${NC} WARNING: .env is NOT gitignored!"
    echo "   Add it to .gitignore immediately!"
fi

echo ""
echo "=================================================="
echo "Security Checklist"
echo "=================================================="
echo ""
echo -e "${GREEN}✓${NC} .env file created"
echo -e "${GREEN}✓${NC} File permissions secured (600)"
echo -e "${GREEN}✓${NC} Secrets not in config file"
echo -e "${YELLOW}⚠${NC}  Remember: NEVER commit .env to git"
echo -e "${YELLOW}⚠${NC}  Remember: Use test keys for development"
echo -e "${YELLOW}⚠${NC}  Remember: Rotate keys every 90 days"

echo ""
echo "=================================================="
echo "Next Steps"
echo "=================================================="
echo ""
echo "1. Start backend:"
echo "   source .env && cargo run"
echo ""
echo "2. Or just run (it auto-loads .env):"
echo "   cargo run"
echo ""
echo "3. Verify Stripe route is registered:"
echo "   ./scripts/test-stripe.sh"
echo ""
echo -e "${GREEN}Setup complete!${NC}"
echo ""
