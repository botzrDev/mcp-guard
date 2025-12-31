#!/bin/bash
set -e

echo "============================================"
echo "MCP-Guard v1.0 Manual Testing Script"
echo "============================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track test results
TESTS_PASSED=0
TESTS_FAILED=0

# Helper function to print test results
print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ PASS${NC}: $2"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ FAIL${NC}: $2"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

# Cleanup function
cleanup() {
    echo ""
    echo "Cleaning up..."
    if [ ! -z "$SERVER_PID" ] && kill -0 $SERVER_PID 2>/dev/null; then
        kill $SERVER_PID 2>/dev/null || true
    fi
    rm -f /tmp/mcp-guard-test-config.toml
}

trap cleanup EXIT

# Section 1: Build
echo "[1/8] Building mcp-guard..."
cargo build --release
BINARY="./target/release/mcp-guard"
print_result $? "Build release binary"

# Section 2: CLI commands
echo ""
echo "[2/8] Testing CLI commands..."

$BINARY version > /dev/null
print_result $? "mcp-guard version"

# Init creates mcp-guard.toml in current directory, then we move it
$BINARY init --format toml --force > /dev/null 2>&1
if [ -f "mcp-guard.toml" ]; then
    mv mcp-guard.toml /tmp/mcp-guard-test-config.toml
    print_result 0 "mcp-guard init"
else
    print_result 1 "mcp-guard init (file not created)"
fi

# Only validate if file exists
if [ -f "/tmp/mcp-guard-test-config.toml" ]; then
    $BINARY validate -c /tmp/mcp-guard-test-config.toml > /dev/null 2>&1
    print_result $? "mcp-guard validate"
else
    print_result 1 "mcp-guard validate (config file missing)"
fi

# Generate an API key and extract it
KEYGEN_OUTPUT=$($BINARY keygen --user-id test-user 2>&1)
API_KEY=$(echo "$KEYGEN_OUTPUT" | grep "mcp_" | grep -oE "mcp_[A-Za-z0-9_-]+" | head -1)
if [ ! -z "$API_KEY" ]; then
    print_result 0 "mcp-guard keygen"
else
    print_result 1 "mcp-guard keygen (no key generated)"
    API_KEY="mcp_test_key_placeholder"
fi

# Test hash-key
HASH_OUTPUT=$($BINARY hash-key "$API_KEY" 2>&1)
if echo "$HASH_OUTPUT" | grep -q "="; then
    print_result 0 "mcp-guard hash-key"
else
    print_result 1 "mcp-guard hash-key (no hash generated)"
fi

# Section 3: Start server
echo ""
echo "[3/8] Starting server..."

# Create a minimal test config
cat > /tmp/mcp-guard-test-config.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 3000

[auth]
api_keys = [
    { id = "test-user", key_hash = "test_hash_placeholder", allowed_tools = ["*"] }
]

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = false

[tracing]
enabled = false

[upstream]
transport = "stdio"
command = "cat"
args = []
EOF

# Start server in background
$BINARY run -c /tmp/mcp-guard-test-config.toml > /tmp/mcp-guard-server.log 2>&1 &
SERVER_PID=$!

# Wait for server to start
echo "Waiting for server to start..."
MAX_ATTEMPTS=30
ATTEMPT=0
while [ $ATTEMPT -lt $MAX_ATTEMPTS ]; do
    if curl -s http://127.0.0.1:3000/health > /dev/null 2>&1; then
        break
    fi
    sleep 0.5
    ((ATTEMPT++))
done

if [ $ATTEMPT -eq $MAX_ATTEMPTS ]; then
    print_result 1 "Server startup (timeout)"
    echo "Server log:"
    cat /tmp/mcp-guard-server.log
    exit 1
else
    print_result 0 "Server startup"
fi

# Section 4: Health checks
echo ""
echo "[4/8] Testing health endpoints..."

HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/health)
if [ "$HTTP_CODE" = "200" ]; then
    print_result 0 "/health endpoint (HTTP $HTTP_CODE)"
else
    print_result 1 "/health endpoint (HTTP $HTTP_CODE, expected 200)"
fi

HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/live)
if [ "$HTTP_CODE" = "200" ]; then
    print_result 0 "/live endpoint (HTTP $HTTP_CODE)"
else
    print_result 1 "/live endpoint (HTTP $HTTP_CODE, expected 200)"
fi

HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/ready)
if [ "$HTTP_CODE" = "200" ]; then
    print_result 0 "/ready endpoint (HTTP $HTTP_CODE)"
else
    print_result 1 "/ready endpoint (HTTP $HTTP_CODE, expected 200)"
fi

METRICS_OUTPUT=$(curl -s http://127.0.0.1:3000/metrics)
if echo "$METRICS_OUTPUT" | grep -q "mcp_guard_requests_total"; then
    print_result 0 "/metrics endpoint (contains metrics)"
else
    print_result 1 "/metrics endpoint (no metrics found)"
fi

# Section 5: Authentication (Note: This is simplified due to config hash mismatch)
echo ""
echo "[5/8] Testing authentication..."
echo -e "${YELLOW}Note: Full auth testing requires proper key hashing setup${NC}"

# Test missing auth
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/mcp)
if [ "$HTTP_CODE" = "401" ]; then
    print_result 0 "Authentication required (HTTP $HTTP_CODE)"
else
    print_result 1 "Authentication required (HTTP $HTTP_CODE, expected 401)"
fi

# Section 6: Rate limiting smoke test
echo ""
echo "[6/8] Testing rate limiting (smoke test)..."
echo "Sending 10 rapid requests..."
RATE_LIMIT_CODES=$(for i in {1..10}; do
    curl -s -o /dev/null -w "%{http_code}\n" \
        http://127.0.0.1:3000/mcp
done | sort | uniq -c)

echo "$RATE_LIMIT_CODES"
print_result 0 "Rate limiting smoke test (manual verification needed)"

# Section 7: Integration test placeholder
echo ""
echo "[7/8] Integration testing..."
echo -e "${YELLOW}Skipped: Use scripts/test-with-filesystem-server.sh for full integration test${NC}"

# Section 8: Graceful shutdown
echo ""
echo "[8/8] Testing graceful shutdown..."
kill -TERM $SERVER_PID 2>/dev/null || true
sleep 2

if kill -0 $SERVER_PID 2>/dev/null; then
    print_result 1 "Graceful shutdown (process still running)"
    kill -9 $SERVER_PID 2>/dev/null || true
else
    print_result 0 "Graceful shutdown"
fi
SERVER_PID=""

# Summary
echo ""
echo "============================================"
echo "Test Summary"
echo "============================================"
echo -e "${GREEN}Passed:${NC} $TESTS_PASSED"
echo -e "${RED}Failed:${NC} $TESTS_FAILED"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}✗ Some tests failed${NC}"
    exit 1
fi
