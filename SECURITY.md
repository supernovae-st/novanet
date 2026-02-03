# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 9.0.x   | :white_check_mark: |
| 8.x.x   | :x:                |
| < 8.0   | :x:                |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in NovaNet,
please report it responsibly.

### How to Report

1. **Do NOT** open a public GitHub issue for security vulnerabilities
2. Email your findings to the repository maintainers via GitHub's private vulnerability reporting:
   - Go to the [Security tab](https://github.com/supernovae-st/novanet-dev/security)
   - Click "Report a vulnerability"
   - Provide a detailed description of the vulnerability

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Resolution target**: Within 30 days for critical issues

### Scope

This security policy applies to:

- `@novanet/core` package
- `@novanet/studio` application
- `tools/novanet` Rust CLI/TUI
- Neo4j database configurations

### Out of Scope

- Third-party dependencies (report to their maintainers)
- Issues in development/test environments only
- Social engineering attacks

## Security Measures

NovaNet implements the following security measures:

- **CodeQL scanning**: Automated security analysis on PRs and weekly
- **Dependabot**: Automated dependency updates for security patches
- **Input validation**: Zod schemas for all external inputs
- **Neo4j access**: Credentials managed via environment variables
- **API routes**: Server-side validation and sanitization

## Acknowledgments

We appreciate responsible disclosure and will acknowledge security researchers
who help improve NovaNet's security.
