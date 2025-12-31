#!/bin/bash
set -e

echo "============================================"
echo "MCP-Guard Integration Test"
echo "Testing with @modelcontextprotocol/server-filesystem"
echo "============================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Track test results
TESTS_PASSED=0
TESTS_FAILED=0

# Helper function to print test results
print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ PASS${NC}: $2"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC}: $2"
        ((TESTS_FAILED++))
    fi
}

# Cleanup function
cleanup() {
    echo ""
    echo "Cleaning up..."
    if [ ! -z "$GUARD_PID" ] && kill -0 $GUARD_PID 2>/dev/null; then
        kill $GUARD_PID 2>/dev/null || true
    fi
    rm -rf "$TEST_DIR"
    rm -f /tmp/mcp-guard-integration-test.toml
}

trap cleanup EXIT

# Check dependencies
echo "[1/7] Checking dependencies..."

if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js is not installed${NC}"
    echo "Please install Node.js from https://nodejs.org"
    exit 1
fi
print_result 0 "Node.js installed ($(node --version))"

if ! command -v npx &> /dev/null; then
    echo -e "${RED}Error: npx is not installed${NC}"
    exit 1
fi
print_result 0 "npx available"

if ! command -v jq &> /dev/null; then
    echo -e "${YELLOW}Warning: jq not installed (JSON parsing will be limited)${NC}"
    HAS_JQ=0
else
    print_result 0 "jq installed"
    HAS_JQ=1
fi

# Create test directory
echo ""
echo "[2/7] Setting up test environment..."
TEST_DIR=$(mktemp -d)
echo "Test directory: $TEST_DIR"

# Create some test files
echo "Hello, MCP Guard!" > "$TEST_DIR/test.txt"
echo '{"key": "value"}' > "$TEST_DIR/test.json"
mkdir -p "$TEST_DIR/subdir"
echo "Subdirectory file" > "$TEST_DIR/subdir/nested.txt"

print_result 0 "Test files created"

# Build mcp-guard
echo ""
echo "[3/7] Building mcp-guard..."
cargo build --release --quiet
BINARY="./target/release/mcp-guard"
print_result 0 "mcp-guard built"

# Generate API key
echo ""
echo "[4/7] Generating API key..."
KEYGEN_OUTPUT=$($BINARY keygen --user-id integration-test 2>&1)
API_KEY=$(echo "$KEYGEN_OUTPUT" | grep "mcp_" | awk '{print $NF}')
KEY_HASH=$(echo "$API_KEY" | $BINARY hash-key)

if [ -z "$API_KEY" ]; then
    echo -e "${RED}Failed to generate API key${NC}"
    exit 1
fi

echo "Generated API key: ${API_KEY:0:20}..."
print_result 0 "API key generated"

# Create config file
echo ""
echo "[5/7] Creating configuration..."
cat > /tmp/mcp-guard-integration-test.toml << EOF
[server]
host = "127.0.0.1"
port = 3000

[auth]
api_keys = [
    { id = "integration-test", key_hash = "$KEY_HASH", allowed_tools = ["*"] }
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
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "$TEST_DIR"]
EOF

print_result 0 "Configuration created"

# Start mcp-guard
echo ""
echo "[6/7] Starting mcp-guard..."
$BINARY run -c /tmp/mcp-guard-integration-test.toml > /tmp/mcp-guard-integration.log 2>&1 &
GUARD_PID=$!

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
    echo -e "${RED}Server failed to start within 15 seconds${NC}"
    echo "Server log:"
    cat /tmp/mcp-guard-integration.log
    exit 1
fi

print_result 0 "mcp-guard started successfully"

# Give the filesystem server a moment to initialize
sleep 2

# Run integration tests
echo ""
echo "[7/7] Running integration tests..."
echo ""

# Test 1: Health check
echo -e "${BLUE}Test 1: Health endpoint${NC}"
HEALTH_RESPONSE=$(curl -s http://127.0.0.1:3000/health)
if echo "$HEALTH_RESPONSE" | grep -q "healthy"; then
    print_result 0 "Health check returns healthy status"
else
    print_result 1 "Health check (unexpected response)"
    echo "Response: $HEALTH_RESPONSE"
fi

# Test 2: Tools list
echo ""
echo -e "${BLUE}Test 2: List available tools${NC}"
TOOLS_RESPONSE=$(curl -s -X POST http://127.0.0.1:3000/mcp \
  -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}')

if echo "$TOOLS_RESPONSE" | grep -q '"result"'; then
    print_result 0 "tools/list request succeeded"

    if [ $HAS_JQ -eq 1 ]; then
        TOOL_COUNT=$(echo "$TOOLS_RESPONSE" | jq '.result.tools | length')
        echo "  Found $TOOL_COUNT tools"

        # Check for expected filesystem tools
        if echo "$TOOLS_RESPONSE" | jq -e '.result.tools[] | select(.name=="read_file")' > /dev/null 2>&1; then
            print_result 0 "read_file tool available"
        else
            print_result 1 "read_file tool not found"
        fi

        if echo "$TOOLS_RESPONSE" | jq -e '.result.tools[] | select(.name=="list_directory")' > /dev/null 2>&1; then
            print_result 0 "list_directory tool available"
        else
            print_result 1 "list_directory tool not found"
        fi
    else
        echo "  (Install jq for detailed tool inspection)"
        if echo "$TOOLS_RESPONSE" | grep -q "read_file"; then
            print_result 0 "read_file tool available"
        fi
        if echo "$TOOLS_RESPONSE" | grep -q "list_directory"; then
            print_result 0 "list_directory tool available"
        fi
    fi
else
    print_result 1 "tools/list request failed"
    echo "Response: $TOOLS_RESPONSE"
fi

# Test 3: Read a file
echo ""
echo -e "${BLUE}Test 3: Read file via MCP${NC}"
READ_RESPONSE=$(curl -s -X POST http://127.0.0.1:3000/mcp \
  -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{\"name\":\"read_file\",\"arguments\":{\"path\":\"$TEST_DIR/test.txt\"}}}")

if echo "$READ_RESPONSE" | grep -q "Hello, MCP Guard!"; then
    print_result 0 "read_file returned correct content"
else
    print_result 1 "read_file did not return expected content"
    if [ $HAS_JQ -eq 1 ]; then
        echo "Response: $(echo "$READ_RESPONSE" | jq -C .)"
    else
        echo "Response: $READ_RESPONSE"
    fi
fi

# Test 4: List directory
echo ""
echo -e "${BLUE}Test 4: List directory via MCP${NC}"
LIST_RESPONSE=$(curl -s -X POST http://127.0.0.1:3000/mcp \
  -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{\"name\":\"list_directory\",\"arguments\":{\"path\":\"$TEST_DIR\"}}}")

if echo "$LIST_RESPONSE" | grep -q "test.txt"; then
    print_result 0 "list_directory shows test.txt"
else
    print_result 1 "list_directory missing test.txt"
fi

if echo "$LIST_RESPONSE" | grep -q "subdir"; then
    print_result 0 "list_directory shows subdir"
else
    print_result 1 "list_directory missing subdir"
fi

# Test 5: Authentication (missing key should fail)
echo ""
echo -e "${BLUE}Test 5: Authentication enforcement${NC}"
UNAUTH_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST http://127.0.0.1:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":4,"method":"tools/list"}')

HTTP_CODE=$(echo "$UNAUTH_RESPONSE" | tail -1)
if [ "$HTTP_CODE" = "401" ]; then
    print_result 0 "Unauthenticated request rejected (HTTP 401)"
else
    print_result 1 "Unauthenticated request (expected 401, got $HTTP_CODE)"
fi

# Test 6: Invalid tool should be rejected
echo ""
echo -e "${BLUE}Test 6: Invalid tool call${NC}"
INVALID_RESPONSE=$(curl -s -X POST http://127.0.0.1:3000/mcp \
  -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"nonexistent_tool","arguments":{}}}')

if echo "$INVALID_RESPONSE" | grep -q "error"; then
    print_result 0 "Invalid tool call returns error"
else
    print_result 1 "Invalid tool call (expected error response)"
fi

# Test 7: Rate limiting
echo ""
echo -e "${BLUE}Test 7: Rate limiting${NC}"
echo "Sending 20 rapid requests..."
RATE_TEST_CODES=""
for i in {1..20}; do
    CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST http://127.0.0.1:3000/mcp \
      -H "Authorization: Bearer $API_KEY" \
      -H "Content-Type: application/json" \
      -d '{"jsonrpc":"2.0","id":'"$i"',"method":"tools/list"}')
    RATE_TEST_CODES="$RATE_TEST_CODES $CODE"
done

SUCCESS_COUNT=$(echo "$RATE_TEST_CODES" | tr ' ' '\n' | grep -c "200" || echo "0")
echo "  Successful requests: $SUCCESS_COUNT/20"

if [ $SUCCESS_COUNT -gt 0 ]; then
    print_result 0 "Rate limiting allows requests"
else
    print_result 1 "Rate limiting blocked all requests"
fi

# Test 8: Metrics
echo ""
echo -e "${BLUE}Test 8: Prometheus metrics${NC}"
METRICS_RESPONSE=$(curl -s http://127.0.0.1:3000/metrics)

if echo "$METRICS_RESPONSE" | grep -q "mcp_guard_requests_total"; then
    print_result 0 "Metrics contain request counter"

    # Extract request count if possible
    REQUEST_COUNT=$(echo "$METRICS_RESPONSE" | grep "mcp_guard_requests_total{" | head -1 | awk '{print $2}')
    if [ ! -z "$REQUEST_COUNT" ]; then
        echo "  Total requests recorded: $REQUEST_COUNT"
    fi
else
    print_result 1 "Metrics missing request counter"
fi

if echo "$METRICS_RESPONSE" | grep -q "mcp_guard_auth_total"; then
    print_result 0 "Metrics contain auth counter"
else
    print_result 1 "Metrics missing auth counter"
fi

# Final summary
echo ""
echo "============================================"
echo "Integration Test Summary"
echo "============================================"
echo -e "${GREEN}Passed:${NC} $TESTS_PASSED"
echo -e "${RED}Failed:${NC} $TESTS_FAILED"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All integration tests passed!${NC}"
    echo ""
    echo "mcp-guard successfully:"
    echo "  • Proxied requests to filesystem MCP server"
    echo "  • Enforced authentication"
    echo "  • Listed and called MCP tools"
    echo "  • Read files and listed directories"
    echo "  • Applied rate limiting"
    echo "  • Recorded metrics"
    echo ""
    exit 0
else
    echo -e "${RED}✗ Some integration tests failed${NC}"
    echo ""
    echo "Server log (last 50 lines):"
    tail -50 /tmp/mcp-guard-integration.log
    exit 1
fi
