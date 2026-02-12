#!/usr/bin/env bash
# Validate NovaNet MCP Server starts and responds to initialize
#
# Usage: ./scripts/validate-mcp.sh [binary_path]
#
# Prerequisites:
#   - Neo4j running at localhost:7687
#   - NOVANET_MCP_NEO4J_PASSWORD environment variable set

set -euo pipefail

BINARY_PATH="${1:-./target/release/novanet-mcp}"

# Check binary exists
if [[ ! -x "$BINARY_PATH" ]]; then
    echo "Error: Binary not found or not executable: $BINARY_PATH"
    echo "Run 'cargo build --release' first"
    exit 1
fi

# Check Neo4j password is set
if [[ -z "${NOVANET_MCP_NEO4J_PASSWORD:-}" ]]; then
    echo "Error: NOVANET_MCP_NEO4J_PASSWORD environment variable not set"
    exit 1
fi

echo "Testing MCP server initialization..."

# Send initialize request and capture response
RESPONSE=$(echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | \
    timeout 10 "$BINARY_PATH" 2>/dev/null | head -1)

# Validate response
if [[ -z "$RESPONSE" ]]; then
    echo "Error: No response from server"
    exit 1
fi

# Check for valid JSON-RPC response
if echo "$RESPONSE" | grep -q '"result"'; then
    echo "Success: MCP server initialized correctly"
    echo ""
    echo "Response (truncated):"
    echo "$RESPONSE" | head -c 500
    echo ""
    exit 0
elif echo "$RESPONSE" | grep -q '"error"'; then
    echo "Error: MCP server returned error"
    echo "$RESPONSE"
    exit 1
else
    echo "Error: Invalid response format"
    echo "$RESPONSE"
    exit 1
fi
