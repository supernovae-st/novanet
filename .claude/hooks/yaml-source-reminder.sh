#!/bin/bash
# Hook: Remind about YAML source of truth when reading model files
# Triggered by: PostToolUse on Read matching models/**/*.yaml or taxonomy.yaml
#
# Purpose: Prevent generating architecture diagrams from memory.
# When exploring model YAML, this reminds to use YAML data in any diagrams/explanations.

FILE_PATH="${CLAUDE_FILE_PATH:-$1}"

# Only remind for significant architecture files, not every YAML read
if [[ "$FILE_PATH" == *"taxonomy.yaml"* ]] || \
   [[ "$FILE_PATH" == *"node-classes"* ]] || \
   [[ "$FILE_PATH" == *"arc-classes"* ]] || \
   [[ "$FILE_PATH" == *"meta/layers"* ]] || \
   [[ "$FILE_PATH" == *"meta/realms"* ]]; then

  # Output reminder (will appear as hook feedback)
  echo "<yaml-source-reminder>"
  echo "YAML SOURCE OF TRUTH: Use data from this file (not memory) for any architecture diagrams or explanations."
  echo "</yaml-source-reminder>"
fi
