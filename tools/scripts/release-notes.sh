#!/usr/bin/env bash
# Generate release notes from conventional commits between two git refs.
#
# Usage:
#   ./tools/scripts/release-notes.sh <version> [base-ref]
#
# Examples:
#   ./tools/scripts/release-notes.sh v9.0.0              # auto-detect previous tag
#   ./tools/scripts/release-notes.sh v9.0.0 v8.3.0       # explicit base
#   ./tools/scripts/release-notes.sh v9.1.0 v9.0.0       # between two versions
#
# Output: Markdown release notes to stdout. Pipe to file:
#   ./tools/scripts/release-notes.sh v9.0.0 > release-notes.md

set -euo pipefail

VERSION="${1:?Usage: release-notes.sh <version> [base-ref]}"
BASE="${2:-}"

# Auto-detect base: previous tag by date
if [[ -z "$BASE" ]]; then
  BASE=$(git tag --sort=-creatordate | head -1)
  if [[ -z "$BASE" ]]; then
    echo "Error: no previous tag found. Specify base-ref manually." >&2
    exit 1
  fi
  echo "# Auto-detected base: $BASE" >&2
fi

REF="HEAD"
COMMIT_COUNT=$(git log --oneline "$BASE".."$REF" | wc -l | tr -d ' ')
DIFF_STAT=$(git diff --stat "$BASE".."$REF" | tail -1)

echo "## $VERSION"
echo ""
echo "**Base**: \`$BASE\` | **Commits**: $COMMIT_COUNT | **Changes**: $DIFF_STAT"
echo ""

# Categorize commits by conventional commit type
declare -A CATEGORIES
CATEGORIES=(
  ["feat"]="### Added"
  ["fix"]="### Fixed"
  ["perf"]="### Performance"
  ["refactor"]="### Changed"
  ["docs"]="### Documentation"
  ["test"]="### Tests"
  ["chore"]="### Maintenance"
  ["security"]="### Security"
)

# Order of sections
SECTION_ORDER=("feat" "fix" "perf" "refactor" "docs" "test" "chore" "security")

for TYPE in "${SECTION_ORDER[@]}"; do
  HEADER="${CATEGORIES[$TYPE]}"
  COMMITS=$(git log --oneline --format='- %s (%h)' "$BASE".."$REF" | grep "^- ${TYPE}" || true)

  if [[ -n "$COMMITS" ]]; then
    echo "$HEADER"
    echo ""
    echo "$COMMITS"
    echo ""
  fi
done

# Breaking changes (look for BREAKING CHANGE in commit bodies or ! in type)
BREAKING=$(git log --format='%H %s' "$BASE".."$REF" | grep -E '!:' || true)
if [[ -n "$BREAKING" ]]; then
  echo "### Breaking Changes"
  echo ""
  while IFS= read -r line; do
    HASH=$(echo "$line" | cut -d' ' -f1 | cut -c1-7)
    MSG=$(echo "$line" | cut -d' ' -f2-)
    echo "- $MSG ($HASH)"
  done <<< "$BREAKING"
  echo ""
fi

# Stats
echo "---"
echo ""
echo "| Metric | Value |"
echo "|--------|-------|"
echo "| Commits | $COMMIT_COUNT |"
echo "| Diff | $DIFF_STAT |"

# Rust tests (if Cargo.toml exists)
if [[ -f "tools/novanet/Cargo.toml" ]]; then
  RUST_TESTS=$(cargo test --manifest-path tools/novanet/Cargo.toml 2>&1 | grep 'test result' | head -1 || echo "N/A")
  echo "| Rust tests | $RUST_TESTS |"
fi

echo ""
echo "**Full Changelog**: https://github.com/supernovae-st/novanet-dev/compare/${BASE}...${VERSION}"
