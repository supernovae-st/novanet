#!/usr/bin/env sh

# Secrets Check Hook
# Warns when editing files that might contain secrets
# Called as PreToolUse for Write and Edit tools

# Exit codes:
# 0 = proceed
# 1 = block (with message)

# Get the file path from environment (set by Claude Code)
FILE_PATH="${CLAUDE_FILE_PATH:-$1}"

# Exit if no file path
if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# =============================================================================
# BLOCKED FILES (always block)
# =============================================================================

case "$FILE_PATH" in
  *.env|*.env.*|.envrc)
    echo "⛔ BLOCKED: Environment file detected"
    echo "  File: $FILE_PATH"
    echo ""
    echo "Environment files may contain secrets."
    echo "Please edit this file manually."
    exit 1
    ;;
  *credentials*.json|*secrets*.json|*service-account*.json)
    echo "⛔ BLOCKED: Credentials file detected"
    echo "  File: $FILE_PATH"
    echo ""
    echo "This file likely contains secrets."
    echo "Please edit this file manually."
    exit 1
    ;;
  *id_rsa*|*id_ed25519*|*.pem|*.key)
    echo "⛔ BLOCKED: Private key file detected"
    echo "  File: $FILE_PATH"
    echo ""
    echo "Private keys should never be edited by AI."
    exit 1
    ;;
esac

# =============================================================================
# WARNED FILES (proceed with caution)
# =============================================================================

case "$FILE_PATH" in
  **/workflows/*.yml|**/workflows/*.yaml)
    echo "⚠️  WARNING: GitHub Actions workflow"
    echo "  Ensure no secrets are hardcoded."
    exit 0
    ;;
  **/docker-compose*.yml|**/docker-compose*.yaml)
    echo "⚠️  WARNING: Docker Compose file"
    echo "  Ensure passwords use environment variables."
    exit 0
    ;;
  *config*.json|*settings*.json)
    # Only warn, don't block
    : # Silent pass
    ;;
esac

# All other files: proceed
exit 0
