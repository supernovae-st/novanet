---
paths:
  - "**/*.rs"
  - "**/*.ts"
  - "**/*.tsx"
  - "**/*.cypher"
  - "**/Cargo.toml"
  - "**/package.json"
---

# Security Rules

## Overview

NovaNet uses a multi-layer security approach:
- **Rust**: `cargo-deny` (license/advisory), `cargo-audit` (RustSec), `cargo-machete` (unused deps)
- **TypeScript**: `pnpm audit`, ESLint security rules
- **Neo4j**: Parameterized Cypher only, credential isolation
- **CI**: Security checks run on every PR

## Rust Security (`tools/novanet/`)

### Dependencies Policy

All dependencies must pass `cargo deny check`:

```bash
# Run before every commit that touches Cargo.toml
cargo deny check

# Expected output: advisories ok, bans ok, licenses ok, sources ok
```

**Allowed licenses**: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, MPL-2.0, CDLA-Permissive-2.0

**Advisory handling**:
- Direct deps: Fix immediately or document exception in `deny.toml`
- Transitive deps: Document in `deny.toml` with comment explaining upstream status

### Code Patterns

```rust
// BAD: Raw unwrap in library code
let data = file.read().unwrap();

// GOOD: Propagate errors
let data = file.read()?;

// BAD: String interpolation in Cypher (injection risk)
let query = format!("MATCH (n) WHERE n.key = '{}'", user_input);

// GOOD: Parameterized queries
let query = "MATCH (n) WHERE n.key = $key";
graph.run(query).param("key", user_input).await?;
```

### Vulnerability Scanning

```bash
# Full security scan (also runs cargo-deny)
cargo audit

# Check for unused dependencies (reduce attack surface)
cargo machete
```

## TypeScript Security (`packages/`, `apps/`)

### Dependencies Policy

```bash
# Run before commits touching package.json
pnpm audit --audit-level=moderate

# Expected: No moderate+ vulnerabilities
```

### Code Patterns

```typescript
// BAD: Unvalidated external input
const data = req.body as UserData;

// GOOD: Zod validation
const data = userSchema.parse(req.body);

// BAD: Render raw HTML (XSS risk)
// Use sanitizer library like DOMPurify if needed

// BAD: Interpolated Cypher (injection risk)
const query = `MATCH (n:${nodeType})`;

// GOOD: Use parameterized builder
const query = session.run('MATCH (n:Node) WHERE n.type = $type', { type: nodeType });
```

### API Routes (Next.js)

```typescript
// ALWAYS: Validate environment variables at startup
const neo4jPassword = process.env.NEO4J_PASSWORD;
if (!neo4jPassword) throw new Error('NEO4J_PASSWORD required');

// ALWAYS: Rate limiting on public endpoints
export const config = { api: { externalResolver: true } };

// NEVER: Expose credentials in responses
```

## Neo4j / Cypher Security

### Query Patterns

```cypher
// BAD: String concatenation (injection risk)
MATCH (n) WHERE n.key = '" + userInput + "'

// GOOD: Parameters
MATCH (n) WHERE n.key = $key
```

### Credential Management

- **Local**: Use `.env.local` (gitignored)
- **CI**: Use GitHub Secrets
- **Production**: Use Vault or similar secret manager

### Connection Isolation

```typescript
// ALWAYS: Close connections properly
try {
  const session = driver.session();
  // ... work
} finally {
  await session.close();
}
```

## Pre-Commit Checklist

Before committing code that touches security-sensitive areas:

### Rust Changes
- [ ] `cargo deny check` passes
- [ ] `cargo audit` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] No `.unwrap()` or `.expect()` in library code
- [ ] All Cypher queries use parameters

### TypeScript Changes
- [ ] `pnpm audit` passes (or known exceptions documented)
- [ ] Zod schemas validate all external input
- [ ] No raw HTML rendering without sanitization
- [ ] No credentials in source code

### Dependency Changes
- [ ] License is in allowlist
- [ ] No known vulnerabilities (or exception documented)
- [ ] Dependency is actively maintained

## Exception Policy

When adding security exceptions (e.g., advisory ignores):

1. **Document the reason** in `deny.toml` or code comment
2. **Include upstream issue link** if waiting for fix
3. **Set review date** (quarterly review recommended)
4. **Track in security-audit skill** for visibility

Example:
```toml
[advisories]
ignore = [
    "RUSTSEC-2024-0384",  # instant crate (neo4rs transitive) - waiting on neo4rs 0.9.0
]
```

## CI Integration

All security checks run in `.github/workflows/ci.yml`:

```yaml
# Rust security
- name: Security Audit (cargo-deny)
  run: cargo deny check

# TypeScript security
- name: Audit dependencies
  run: pnpm audit --audit-level=moderate

# Secret scanning
- name: Check for secrets in code
  uses: trufflesecurity/trufflehog@main
```

## Quick Reference

| Tool | Command | When |
|------|---------|------|
| cargo-deny | `cargo deny check` | Rust dependency changes |
| cargo-audit | `cargo audit` | Weekly + release |
| cargo-machete | `cargo machete` | Monthly cleanup |
| pnpm audit | `pnpm audit` | TypeScript dependency changes |
| TruffleHog | CI only | Every PR |

## Related

- `/security-audit` skill - Run comprehensive security audit
- `/codebase-audit` skill - Includes security checks
- `tools/novanet/deny.toml` - Rust security policy
