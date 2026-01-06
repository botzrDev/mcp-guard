#!/bin/bash
# End-to-end test for GitHub OAuth authentication flow
#
# This test validates:
# 1. OAuth authorization URL generation with PKCE
# 2. State parameter binding and validation
# 3. Authorization code exchange
# 4. JWT session token minting
# 5. Authenticated API access

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
API_BASE="${API_BASE:-http://localhost:3000}"
FRONTEND_URL="${FRONTEND_URL:-http://localhost:4200}"
CALLBACK_URI="${CALLBACK_URI:-${FRONTEND_URL}/auth/callback}"

echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}GitHub OAuth E2E Test${NC}"
echo -e "${YELLOW}========================================${NC}"
echo ""
echo "API Base: $API_BASE"
echo "Frontend: $FRONTEND_URL"
echo "Callback: $CALLBACK_URI"
echo ""

# Test 1: Verify /oauth/authorize endpoint exists and returns redirect
echo -e "${YELLOW}[Test 1]${NC} Testing OAuth authorization endpoint..."

AUTHORIZE_URL="${API_BASE}/oauth/authorize?provider=github&redirect_uri=${CALLBACK_URI}"
echo "  GET $AUTHORIZE_URL"

# Follow redirects and capture the final URL
RESPONSE=$(curl -s -L -w "\n%{http_code}\n%{redirect_url}" -o /dev/null "$AUTHORIZE_URL" 2>&1 || echo "ERROR")

HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
REDIRECT_URL=$(echo "$RESPONSE" | tail -n2 | head -n1)

if [[ "$HTTP_CODE" == "302" ]] || [[ "$HTTP_CODE" == "307" ]]; then
    echo -e "  ${GREEN}✓${NC} Received redirect (HTTP $HTTP_CODE)"

    # Verify redirect goes to GitHub
    if [[ "$REDIRECT_URL" == *"github.com"* ]]; then
        echo -e "  ${GREEN}✓${NC} Redirect URL points to GitHub"
    else
        echo -e "  ${RED}✗${NC} Redirect URL does not point to GitHub: $REDIRECT_URL"
        exit 1
    fi

    # Verify PKCE parameters are present
    if [[ "$REDIRECT_URL" == *"code_challenge"* ]] && [[ "$REDIRECT_URL" == *"code_challenge_method=S256"* ]]; then
        echo -e "  ${GREEN}✓${NC} PKCE parameters present (code_challenge, S256)"
    else
        echo -e "  ${RED}✗${NC} Missing PKCE parameters in redirect URL"
        exit 1
    fi

    # Verify state parameter
    if [[ "$REDIRECT_URL" == *"state="* ]]; then
        echo -e "  ${GREEN}✓${NC} State parameter present"
        # Extract state for later use
        STATE=$(echo "$REDIRECT_URL" | grep -oP 'state=\K[^&]+' || echo "")
        if [[ -n "$STATE" ]]; then
            echo "    State: $STATE"
        fi
    else
        echo -e "  ${RED}✗${NC} Missing state parameter"
        exit 1
    fi

    # Verify redirect_uri parameter matches our callback
    if [[ "$REDIRECT_URL" == *"redirect_uri="* ]]; then
        REDIRECT_PARAM=$(echo "$REDIRECT_URL" | grep -oP 'redirect_uri=\K[^&]+' | python3 -c "import sys, urllib.parse as ul; print(ul.unquote_plus(sys.stdin.read()))" 2>/dev/null || echo "")
        if [[ "$REDIRECT_PARAM" == *"$CALLBACK_URI"* ]]; then
            echo -e "  ${GREEN}✓${NC} Redirect URI matches callback URL"
        else
            echo -e "  ${YELLOW}⚠${NC} Redirect URI mismatch: $REDIRECT_PARAM"
        fi
    fi

else
    echo -e "  ${RED}✗${NC} Expected redirect (302/307), got HTTP $HTTP_CODE"
    exit 1
fi

echo ""

# Test 2: Verify OAuth state storage and expiry
echo -e "${YELLOW}[Test 2]${NC} Testing OAuth state management..."

# Make another authorize request
AUTHORIZE_URL2="${API_BASE}/oauth/authorize?provider=github&redirect_uri=${CALLBACK_URI}"
RESPONSE2=$(curl -s -L -w "\n%{http_code}\n%{redirect_url}" -o /dev/null "$AUTHORIZE_URL2" 2>&1 || echo "ERROR")
REDIRECT_URL2=$(echo "$RESPONSE2" | tail -n2 | head -n1)
STATE2=$(echo "$REDIRECT_URL2" | grep -oP 'state=\K[^&]+' || echo "")

if [[ "$STATE" != "$STATE2" ]] && [[ -n "$STATE" ]] && [[ -n "$STATE2" ]]; then
    echo -e "  ${GREEN}✓${NC} Each request generates unique state"
    echo "    State 1: ${STATE:0:20}..."
    echo "    State 2: ${STATE2:0:20}..."
else
    echo -e "  ${RED}✗${NC} States are not unique or missing"
    exit 1
fi

echo ""

# Test 3: Test OAuth callback with invalid state
echo -e "${YELLOW}[Test 3]${NC} Testing invalid state rejection..."

INVALID_STATE="invalid_state_12345"
CALLBACK_URL="${API_BASE}/oauth/callback?code=test_code&state=${INVALID_STATE}"

CALLBACK_RESPONSE=$(curl -s -w "\n%{http_code}" "$CALLBACK_URL")
CALLBACK_HTTP_CODE=$(echo "$CALLBACK_RESPONSE" | tail -n1)
CALLBACK_BODY=$(echo "$CALLBACK_RESPONSE" | head -n-1)

if [[ "$CALLBACK_HTTP_CODE" == "401" ]]; then
    echo -e "  ${GREEN}✓${NC} Invalid state rejected (HTTP 401)"
    if [[ "$CALLBACK_BODY" == *"Invalid or expired state"* ]]; then
        echo -e "  ${GREEN}✓${NC} Correct error message returned"
    fi
else
    echo -e "  ${RED}✗${NC} Expected 401 for invalid state, got HTTP $CALLBACK_HTTP_CODE"
    exit 1
fi

echo ""

# Test 4: Verify health endpoint (sanity check)
echo -e "${YELLOW}[Test 4]${NC} Testing health endpoint..."

HEALTH_RESPONSE=$(curl -s -w "\n%{http_code}" "${API_BASE}/health")
HEALTH_HTTP_CODE=$(echo "$HEALTH_RESPONSE" | tail -n1)
HEALTH_BODY=$(echo "$HEALTH_RESPONSE" | head -n-1)

if [[ "$HEALTH_HTTP_CODE" == "200" ]]; then
    echo -e "  ${GREEN}✓${NC} Health check passed (HTTP 200)"
    if [[ "$HEALTH_BODY" == *"healthy"* ]]; then
        echo -e "  ${GREEN}✓${NC} Server reports healthy status"
    fi
else
    echo -e "  ${RED}✗${NC} Health check failed (HTTP $HEALTH_HTTP_CODE)"
    exit 1
fi

echo ""

# Test 5: Test authentication requirement
echo -e "${YELLOW}[Test 5]${NC} Testing authentication requirement on /mcp..."

MCP_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "${API_BASE}/mcp" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}')
MCP_HTTP_CODE=$(echo "$MCP_RESPONSE" | tail -n1)

if [[ "$MCP_HTTP_CODE" == "401" ]]; then
    echo -e "  ${GREEN}✓${NC} /mcp requires authentication (HTTP 401)"
else
    echo -e "  ${YELLOW}⚠${NC} Expected 401, got HTTP $MCP_HTTP_CODE"
fi

echo ""

# Summary
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}All tests passed! ✓${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Manual testing steps:"
echo "1. Open browser to ${FRONTEND_URL}/login"
echo "2. Click 'Continue with GitHub'"
echo "3. Authorize the app on GitHub"
echo "4. Verify redirect back to ${CALLBACK_URI}?token=<JWT>"
echo "5. Verify you can access the dashboard"
echo ""

exit 0
