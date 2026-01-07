#!/bin/bash
# Fix .env file confusion
# This script consolidates your environment files

set -e

cd /home/austingreen/Documents/botzr/projects/mcp-guard

echo "=================================================="
echo "Environment Files Cleanup"
echo "=================================================="
echo ""

# Show current files
echo "Current .env files:"
ls -la .env* 2>/dev/null || echo "No .env files found"
echo ""

# Backup existing .env
if [ -f .env ]; then
    echo "Backing up current .env to .env.backup..."
    cp .env .env.backup
    echo "✓ Backup created"
fi

echo ""
echo "Your Rust backend ONLY loads .env (not .env.local or .env.production)"
echo ""
echo "Options:"
echo "  1) Keep .env as-is and just add STRIPE_SECRET_KEY to it (RECOMMENDED)"
echo "  2) Copy .env.local to .env (if you want those settings)"
echo "  3) Keep all files (no changes)"
echo ""

read -p "Choose option (1/2/3): " -n 1 -r
echo

case $REPLY in
    1)
        echo ""
        echo "Keeping current .env file..."
        echo ""

        # Check if STRIPE_SECRET_KEY exists
        if grep -q "STRIPE_SECRET_KEY" .env; then
            echo "✓ STRIPE_SECRET_KEY already in .env"
        else
            echo "Adding STRIPE_SECRET_KEY placeholder..."
            echo "" >> .env
            echo "# Stripe Configuration" >> .env
            echo "STRIPE_SECRET_KEY=sk_test_YOUR_TEST_KEY_HERE" >> .env
            echo "✓ Added STRIPE_SECRET_KEY placeholder"
            echo ""
            echo "⚠️  Replace sk_test_YOUR_TEST_KEY_HERE with your actual test key"
        fi

        echo ""
        echo "Archiving unused files..."
        mkdir -p archive
        mv .env.local archive/ 2>/dev/null && echo "✓ Moved .env.local to archive/" || true
        mv .env.production archive/ 2>/dev/null && echo "✓ Moved .env.production to archive/" || true
        ;;

    2)
        echo ""
        echo "Copying .env.local to .env..."
        cp .env.local .env
        echo "✓ .env updated from .env.local"

        # Add STRIPE_SECRET_KEY if missing
        if ! grep -q "STRIPE_SECRET_KEY" .env; then
            echo "" >> .env
            echo "# Stripe Configuration" >> .env
            echo "STRIPE_SECRET_KEY=sk_test_YOUR_TEST_KEY_HERE" >> .env
            echo "✓ Added STRIPE_SECRET_KEY placeholder"
        fi

        echo ""
        echo "Archiving unused files..."
        mkdir -p archive
        mv .env.production archive/ 2>/dev/null && echo "✓ Moved .env.production to archive/" || true
        ;;

    3)
        echo ""
        echo "No changes made. Current .env files kept as-is."
        echo ""
        echo "⚠️  Remember: Only .env is loaded by Rust!"
        echo "   .env.local and .env.production are NOT automatically used."
        ;;

    *)
        echo "Invalid option. No changes made."
        exit 1
        ;;
esac

echo ""
echo "=================================================="
echo "Summary"
echo "=================================================="
echo ""
echo "Active file (loaded by Rust):"
echo "  .env"
echo ""
echo "Content:"
head -20 .env | grep -v "SECRET\|KEY" || head -20 .env
echo "  ..."
echo ""
echo "⚠️  Make sure to set your actual Stripe test key in .env:"
echo "   STRIPE_SECRET_KEY=sk_test_..."
echo ""
echo "Then run:"
echo "  cargo run"
echo ""
