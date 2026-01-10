#!/bin/bash
# Pre-commit hook for Claude Code
# Runs formatter and linter before git commit

set -e

# Read tool input from stdin
INPUT=$(cat)

# Extract the command from tool_input
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Check if this is a git commit command
if echo "$COMMAND" | grep -qE '^\s*git\s+commit(\s|$)'; then
  echo "Running formatter and linter before commit..."

  # Change to project directory
  cd "$CLAUDE_PROJECT_DIR"

  # Run formatter (auto-fix)
  if ! pnpm format; then
    echo "Formatter failed"
    exit 2
  fi

  # Run linter (auto-fix)
  if ! pnpm lint:fix; then
    echo "Linter failed"
    exit 2
  fi

  # Run Rust formatter
  if [ -d "src-tauri" ]; then
    echo "Running Rust formatter..."
    if ! cargo fmt --manifest-path src-tauri/Cargo.toml; then
      echo "Rust formatter failed"
      exit 2
    fi
  fi

  echo "Formatter and linter completed successfully"
fi

exit 0
