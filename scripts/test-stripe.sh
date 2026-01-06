#!/bin/bash
# Test script for Stripe integration
# Usage: ./scripts/test-stripe.sh

set -e

echo "=================================================="
echo "MCP-Guard Stripe Integration Test"
echo "=================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if backend is running
echo "1. Checking if backend is running..."
if curl -s http://localhost:3000/health > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Backend is running"
else
    echo -e "${RED}✗${NC} Backend is not running"
    echo "   Start with: cargo run"
    exit 1
fi

echo ""

# Check if Stripe route is registered
echo "2. Testing Stripe checkout endpoint..."
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST http://localhost:3000/api/billing/checkout \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "price_id": "price_test_dummy",
    "success_url": "http://localhost:4200/success",
    "cancel_url": "http://localhost:4200/pricing"
  }')

HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | head -n-1)

if [ "$HTTP_CODE" = "404" ]; then
    echo -e "${RED}✗${NC} Stripe route not registered (404)"
    echo "   This means STRIPE_SECRET_KEY is not set"
    echo ""
    echo "   Fix:"
    echo "   1. Create .env file: cp .env.example .env"
    echo "   2. Add: STRIPE_SECRET_KEY=sk_test_your_key"
    echo "   3. Restart backend: cargo run"
    exit 1
elif [ "$HTTP_CODE" = "500" ]; then
    echo -e "${YELLOW}⚠${NC}  Stripe route is registered but configuration invalid (500)"
    echo "   Response: $BODY"
    echo ""
    echo "   Possible issues:"
    echo "   - Invalid Stripe secret key"
    echo "   - Invalid price ID"
    echo "   - Network connection to Stripe failed"
    echo ""
    echo "   Check backend logs for details"
    exit 1
elif [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "201" ]; then
    echo -e "${GREEN}✓${NC} Stripe route is registered and working!"
    echo "   Response: $BODY"
else
    echo -e "${YELLOW}⚠${NC}  Unexpected response (HTTP $HTTP_CODE)"
    echo "   Response: $BODY"
fi

echo ""

# Check for Stripe keys in environment
echo "3. Checking environment configuration..."
if [ -f .env ]; then
    echo -e "${GREEN}✓${NC} .env file exists"

    if grep -q "STRIPE_SECRET_KEY" .env; then
        # Check if it's not the placeholder
        if grep "STRIPE_SECRET_KEY=sk_" .env | grep -qv "YOUR_KEY"; then
            echo -e "${GREEN}✓${NC} STRIPE_SECRET_KEY is configured"
        else
            echo -e "${YELLOW}⚠${NC}  STRIPE_SECRET_KEY is set to placeholder"
            echo "   Replace with actual key from: https://dashboard.stripe.com/test/apikeys"
        fi
    else
        echo -e "${RED}✗${NC} STRIPE_SECRET_KEY not found in .env"
        echo "   Add: STRIPE_SECRET_KEY=sk_test_your_key"
    fi
else
    echo -e "${YELLOW}⚠${NC}  .env file not found"
    echo "   Create from example: cp .env.example .env"
fi

echo ""

# Check frontend configuration
echo "4. Checking frontend configuration..."
if [ -f landing/src/environments/environment.ts ]; then
    echo -e "${GREEN}✓${NC} Frontend environment file exists"

    if grep -q "pk_test_YOUR_TEST_KEY_HERE" landing/src/environments/environment.ts; then
        echo -e "${YELLOW}⚠${NC}  Frontend using placeholder Stripe key"
        echo "   Update: landing/src/environments/environment.ts"
        echo "   Replace: stripePublishableKey with actual pk_test_... key"
    else
        echo -e "${GREEN}✓${NC} Frontend Stripe key is configured"
    fi

    if grep -q "price_test_YOUR_TEST_PRICE_HERE" landing/src/environments/environment.ts; then
        echo -e "${YELLOW}⚠${NC}  Frontend using placeholder price ID"
        echo "   Update: landing/src/environments/environment.ts"
        echo "   Replace: stripePriceId with actual price_... ID"
    else
        echo -e "${GREEN}✓${NC} Frontend price ID is configured"
    fi
else
    echo -e "${RED}✗${NC} Frontend environment file not found"
fi

echo ""
echo "=================================================="
echo "Summary"
echo "=================================================="

# Final recommendations
if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "201" ]; then
    echo -e "${GREEN}Status: Ready to test!${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Start frontend: cd landing && npm start"
    echo "2. Open: http://localhost:4200/signup"
    echo "3. Test with Stripe test card: 4242 4242 4242 4242"
elif [ "$HTTP_CODE" = "404" ]; then
    echo -e "${RED}Status: Backend not configured${NC}"
    echo ""
    echo "Required steps:"
    echo "1. cp .env.example .env"
    echo "2. Edit .env and set STRIPE_SECRET_KEY=sk_test_..."
    echo "3. cargo run"
    echo "4. Run this test again"
else
    echo -e "${YELLOW}Status: Needs attention${NC}"
    echo ""
    echo "Check the warnings above and fix any issues"
fi

echo ""
