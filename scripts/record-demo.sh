#!/bin/bash
# Record demo GIF for README
# Requires: asciinema, svg-term-cli (npm install -g svg-term-cli)

set -e

# Clean up
rm -f mcp-guard.toml

# Record
asciinema rec demo.cast -c '
echo "$ mcp-guard init"
sleep 0.5
mcp-guard init
sleep 1

echo ""
echo "$ mcp-guard run &"
sleep 0.5
mcp-guard run &
sleep 2

echo ""
echo "$ curl http://localhost:3000/health"
sleep 0.5
curl -s http://localhost:3000/health | jq .
sleep 1

echo ""
echo "# Success! MCP server is now protected."
sleep 2

pkill mcp-guard
'

# Create output directory
mkdir -p docs/assets

# Convert to SVG
svg-term --in demo.cast --out docs/assets/demo.svg --window --width 80 --height 24

echo "Demo recorded to docs/assets/demo.svg"
echo "Add to README: ![Demo](docs/assets/demo.svg)"
