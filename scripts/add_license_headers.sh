#!/bin/bash
#
# Add SPDX license headers to all Rust source files
#
# This script adds appropriate license headers based on which crate the file belongs to:
# - mcp-guard-core: AGPL-3.0 (open source)
# - mcp-guard-cli: AGPL-3.0 (open source)
# - mcp-guard-pro: LicenseRef-Commercial (proprietary)
# - mcp-guard-enterprise: LicenseRef-Commercial (proprietary)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# License header templates
read -r -d '' AGPL_HEADER << 'EOF' || true
// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: AGPL-3.0
//
// This file is part of MCP-Guard.
//
// MCP-Guard is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// MCP-Guard is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with MCP-Guard. If not, see <https://www.gnu.org/licenses/>.

EOF

read -r -d '' PRO_HEADER << 'EOF' || true
// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: LicenseRef-Commercial
//
// This file is part of MCP-Guard Pro, a commercial product.
//
// MCP-Guard Pro requires a valid commercial license for use.
// Unauthorized use, modification, or distribution is prohibited.
//
// For licensing information, visit: https://mcp-guard.io/pricing
// For support, contact: austin@botzr.dev

EOF

read -r -d '' ENTERPRISE_HEADER << 'EOF' || true
// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: LicenseRef-Commercial
//
// This file is part of MCP-Guard Enterprise, a commercial product.
//
// MCP-Guard Enterprise requires a valid commercial license for use.
// Unauthorized use, modification, or distribution is prohibited.
//
// For licensing information, visit: https://mcp-guard.io/pricing
// For support, contact: austin@botzr.dev

EOF

# Function to add header to a file
add_header() {
    local file=$1
    local header=$2

    # Check if file already has a copyright header
    if head -n 1 "$file" | grep -q "Copyright"; then
        echo "  [SKIP] $file (already has header)"
        return
    fi

    # Create temp file with header + original content
    echo "$header" > "$file.tmp"
    cat "$file" >> "$file.tmp"
    mv "$file.tmp" "$file"

    echo "  [ADD]  $file"
}

# Process each crate
echo "Adding license headers to Rust source files..."
echo ""

# mcp-guard-core (AGPL-3.0)
echo "Processing mcp-guard-core (AGPL-3.0)..."
find "$PROJECT_ROOT/crates/mcp-guard-core/src" -name "*.rs" | while read -r file; do
    add_header "$file" "$AGPL_HEADER"
done
echo ""

# mcp-guard-cli (AGPL-3.0)
echo "Processing mcp-guard-cli (AGPL-3.0)..."
find "$PROJECT_ROOT/crates/mcp-guard-cli/src" -name "*.rs" 2>/dev/null | while read -r file; do
    add_header "$file" "$AGPL_HEADER"
done
# Also check if mcp-guard is the CLI (old structure)
if [ -d "$PROJECT_ROOT/crates/mcp-guard/src" ]; then
    find "$PROJECT_ROOT/crates/mcp-guard/src" -name "*.rs" 2>/dev/null | while read -r file; do
        add_header "$file" "$AGPL_HEADER"
    done
fi
echo ""

# mcp-guard-pro (LicenseRef-Commercial)
echo "Processing mcp-guard-pro (LicenseRef-Commercial)..."
find "$PROJECT_ROOT/crates/mcp-guard-pro/src" -name "*.rs" 2>/dev/null | while read -r file; do
    add_header "$file" "$PRO_HEADER"
done
echo ""

# mcp-guard-enterprise (LicenseRef-Commercial)
echo "Processing mcp-guard-enterprise (LicenseRef-Commercial)..."
find "$PROJECT_ROOT/crates/mcp-guard-enterprise/src" -name "*.rs" 2>/dev/null | while read -r file; do
    add_header "$file" "$ENTERPRISE_HEADER"
done
echo ""

echo "✓ License headers added successfully!"
echo ""
echo "Summary:"
echo "  - mcp-guard-core:       AGPL-3.0 (Open Source)"
echo "  - mcp-guard-cli:        AGPL-3.0 (Open Source)"
echo "  - mcp-guard-pro:        LicenseRef-Commercial"
echo "  - mcp-guard-enterprise: LicenseRef-Commercial"
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Verify compilation: cargo check --all-features"
echo "  3. Commit: git add -A && git commit -m 'Add SPDX license headers to all source files'"
