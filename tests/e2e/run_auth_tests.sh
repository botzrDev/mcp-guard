#!/bin/bash
# Run all authentication end-to-end tests
#
# Usage:
#   ./tests/e2e/run_auth_tests.sh              # Test against local server
#   ./tests/e2e/run_auth_tests.sh --prod       # Test against production

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Parse arguments
ENVIRONMENT="local"
if [[ "${1:-}" == "--prod" ]]; then
    ENVIRONMENT="production"
fi

# Configuration based on environment
if [[ "$ENVIRONMENT" == "production" ]]; then
    export API_BASE="https://mcpg.botzr.com"
    export FRONTEND_URL="https://mcpg.botzr.com"
else
    export API_BASE="http://localhost:3000"
    export FRONTEND_URL="http://localhost:4200"
fi

export CALLBACK_URI="${FRONTEND_URL}/auth/callback"

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   MCP Guard Authentication Test Suite ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Environment:${NC} $ENVIRONMENT"
echo -e "${YELLOW}API Base:${NC} $API_BASE"
echo -e "${YELLOW}Frontend:${NC} $FRONTEND_URL"
echo ""

# Check if server is running
echo -e "${YELLOW}Checking server availability...${NC}"
if curl -s -f "${API_BASE}/health" > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Server is running"
else
    echo -e "${RED}✗${NC} Server is not accessible at ${API_BASE}"
    echo ""
    echo "Please start the backend server first:"
    echo "  cd /home/austingreen/Documents/botzr/projects/mcp-guard"
    echo "  cargo run --bin mcp-guard run"
    echo ""
    exit 1
fi

echo ""

# Track test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Run GitHub OAuth test
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Running GitHub OAuth Tests${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

TOTAL_TESTS=$((TOTAL_TESTS + 1))
if ./tests/e2e/github_oauth_test.sh; then
    PASSED_TESTS=$((PASSED_TESTS + 1))
    echo -e "${GREEN}GitHub OAuth tests: PASSED${NC}"
else
    FAILED_TESTS=$((FAILED_TESTS + 1))
    echo -e "${RED}GitHub OAuth tests: FAILED${NC}"
fi

echo ""

# Run Google OAuth test (if implemented)
if [[ -f "./tests/e2e/google_oauth_test.sh" ]]; then
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Running Google OAuth Tests${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if ./tests/e2e/google_oauth_test.sh; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        echo -e "${GREEN}Google OAuth tests: PASSED${NC}"
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        echo -e "${RED}Google OAuth tests: FAILED${NC}"
    fi
    echo ""
else
    echo -e "${YELLOW}⊘ Skipping Google OAuth tests (not implemented)${NC}"
    echo ""
fi

# Run Magic Link test (if implemented)
if [[ -f "./tests/e2e/magic_link_test.sh" ]]; then
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Running Magic Link Tests${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if ./tests/e2e/magic_link_test.sh; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        echo -e "${GREEN}Magic Link tests: PASSED${NC}"
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        echo -e "${RED}Magic Link tests: FAILED${NC}"
    fi
    echo ""
else
    echo -e "${YELLOW}⊘ Skipping Magic Link tests (not implemented)${NC}"
    echo ""
fi

# Print summary
echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║           Test Results Summary         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "Total tests:  $TOTAL_TESTS"
echo -e "${GREEN}Passed:       $PASSED_TESTS${NC}"
if [[ $FAILED_TESTS -gt 0 ]]; then
    echo -e "${RED}Failed:       $FAILED_TESTS${NC}"
else
    echo -e "Failed:       $FAILED_TESTS"
fi
echo ""

# Exit with appropriate code
if [[ $FAILED_TESTS -eq 0 ]]; then
    echo -e "${GREEN}All tests passed! ✓${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed ✗${NC}"
    exit 1
fi
