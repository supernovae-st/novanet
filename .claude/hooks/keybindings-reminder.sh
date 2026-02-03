#!/bin/bash
# Hook: Remind to update KEYBINDINGS.md when TUI keybinding code changes
# Triggered by: PostToolUse on Write|Edit matching tui/*.rs

FILE_PATH="$CLAUDE_FILE_PATH"

# Check if the edited file is related to TUI keybindings
if [[ "$FILE_PATH" == *"tui/app.rs"* ]] || \
   [[ "$FILE_PATH" == *"tui/events.rs"* ]] || \
   [[ "$FILE_PATH" == *"tui/palette.rs"* ]] || \
   [[ "$FILE_PATH" == *"tui/search.rs"* ]] || \
   [[ "$FILE_PATH" == *"tui/dialogs.rs"* ]]; then

  # Check if the edit involves keybinding-related code
  if grep -q "KeyCode\|handle_key\|Char('\|Key::" "$FILE_PATH" 2>/dev/null; then
    echo "KEYBINDING_CHANGE_DETECTED"
    echo ""
    echo "You modified TUI keybinding code in: $(basename "$FILE_PATH")"
    echo ""
    echo "Please update: tools/novanet/KEYBINDINGS.md"
    echo "Source of truth: src/tui/app.rs (handle_key_event)"
  fi
fi
