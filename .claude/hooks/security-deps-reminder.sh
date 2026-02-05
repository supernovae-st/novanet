#!/bin/bash
# NovaNet Security Dependencies Reminder Hook
# Triggers when Cargo.toml, package.json, or pnpm-lock.yaml is edited
# Reminds to run security checks after dependency changes

# This hook is triggered by PostToolUse:Write/Edit on dependency files
# Path pattern: "*/Cargo.toml|*/package.json|*/pnpm-lock.yaml"

cat << 'EOF'
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🔒 DEPENDENCY CHANGE DETECTED                                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  You modified dependency files. Run security checks before committing:        ║
║                                                                               ║
║  Rust (Cargo.toml):                                                           ║
║    cargo deny check        # License/security policy                          ║
║    cargo audit             # RustSec vulnerability scan                       ║
║    cargo machete           # Unused dependencies                              ║
║                                                                               ║
║  TypeScript (package.json):                                                   ║
║    pnpm audit              # npm security audit                               ║
║                                                                               ║
║  Full security audit:                                                         ║
║    /security-audit all     # Run comprehensive checks                         ║
║                                                                               ║
║  See: .claude/rules/security.md for full security policy                      ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
EOF
