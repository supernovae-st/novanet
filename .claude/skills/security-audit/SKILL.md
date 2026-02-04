---
name: security-audit
description: Run comprehensive security audit across Rust and TypeScript. Checks cargo-deny, cargo-audit, pnpm audit, and reports vulnerabilities with remediation steps.
disable-model-invocation: false
user-invocable: true
allowed-tools: Bash, Grep, Glob, Read, TodoWrite
argument-hint: [rust|typescript|all|exceptions]
---

# Security Audit

Comprehensive security audit for NovaNet codebase.

## Quick Start

| Argument | What it does |
|----------|--------------|
| `rust` | Audit Rust dependencies only |
| `typescript` | Audit TypeScript dependencies only |
| `all` (default) | Full audit: Rust + TypeScript + exceptions review |
| `exceptions` | List all security exceptions and their status |

## Rust Audit (`rust`)

### Step 1: cargo-deny (License + Advisory + Bans)

```bash
cd tools/novanet && cargo deny check 2>&1
```

**Expected output:**
```
advisories ok
bans ok
licenses ok
sources ok
```

### Step 2: cargo-audit (RustSec Database)

```bash
cd tools/novanet && cargo audit 2>&1
```

**Expected:** No vulnerabilities, or documented exceptions.

### Step 3: cargo-machete (Unused Dependencies)

```bash
cd tools/novanet && cargo machete 2>&1
```

**Expected:** Only intentionally-kept deps (see `[package.metadata.cargo-machete]` in Cargo.toml).

### Step 4: Report

Create a summary:

```markdown
## Rust Security Audit

| Check | Status | Notes |
|-------|--------|-------|
| cargo-deny | PASS/FAIL | details |
| cargo-audit | PASS/FAIL | details |
| cargo-machete | PASS/FAIL | details |

### Exceptions Active
- RUSTSEC-XXXX: reason (expires: YYYY-MM-DD)
```

## TypeScript Audit (`typescript`)

### Step 1: pnpm audit

```bash
pnpm audit --audit-level=moderate 2>&1
```

**Expected:** No moderate+ vulnerabilities.

### Step 2: Check for Known Patterns

Search for risky patterns (see `.claude/rules/security.md` for details):

```bash
# Check for XSS-prone patterns
grep -rE "innerHTML|outerHTML" apps/studio/src --include="*.tsx" -l 2>/dev/null || echo "None found"

# Check for hardcoded credentials
grep -rE "(password|secret|apikey|api_key).*=.*['\"]" packages apps --include="*.ts" --include="*.tsx" -l 2>/dev/null | grep -v ".test." || echo "None found"
```

### Step 3: Report

```markdown
## TypeScript Security Audit

| Check | Status | Notes |
|-------|--------|-------|
| pnpm audit | PASS/FAIL | X vulnerabilities |
| XSS patterns | PASS/FAIL | X files |
| hardcoded creds | PASS/FAIL | X files |
```

## Full Audit (`all`)

Run both Rust and TypeScript audits, plus:

### Step 1: CI Security Check

Verify CI is configured correctly:

```bash
# Check cargo-deny in CI
grep -A5 "cargo-deny" .github/workflows/ci.yml

# Check pnpm audit in CI
grep -A5 "pnpm audit" .github/workflows/ci.yml

# Check TruffleHog
grep -A5 "trufflehog" .github/workflows/ci.yml
```

### Step 2: Secret Scanning

Check for potential secrets in codebase:

```bash
# List all .env files (should be gitignored)
find . -name ".env*" -type f 2>/dev/null | grep -v node_modules

# Check gitignore for secrets patterns
grep -E "(\.env|secret|credential|password)" .gitignore
```

### Step 3: Consolidated Report

```markdown
## Full Security Audit Report

### Rust
- cargo-deny: PASS/FAIL
- cargo-audit: PASS/FAIL
- cargo-machete: PASS/FAIL

### TypeScript
- pnpm audit: PASS/FAIL
- Code patterns: PASS/FAIL

### CI/CD
- cargo-deny in CI: YES/NO
- pnpm audit in CI: YES/NO
- TruffleHog: YES/NO

### Exceptions Active: X
### Action Items: Y
```

## Exceptions Review (`exceptions`)

List and review all security exceptions:

### Step 1: Rust Exceptions (deny.toml)

```bash
grep -A2 "RUSTSEC" tools/novanet/deny.toml
```

### Step 2: Cargo.toml Machete Ignores

```bash
grep -A10 "cargo-machete" tools/novanet/Cargo.toml
```

### Step 3: Review Status

For each exception:
1. Check if upstream has fixed the issue
2. Check if we can update the dependency
3. Document if exception is still needed

```bash
# Check neo4rs latest version
cargo search neo4rs --limit 1
```

### Step 4: Exception Report

```markdown
## Security Exceptions Review

| ID | Package | Reason | Added | Review Date |
|----|---------|--------|-------|-------------|
| RUSTSEC-2025-0012 | backoff | neo4rs transitive | 2026-02-04 | 2026-05-04 |
| ... | ... | ... | ... | ... |

### Actions Needed
- [ ] Check if neo4rs 0.9.0 removes backoff dependency
- [ ] Review quarterly (next: YYYY-MM-DD)
```

## Remediation Workflows

### When cargo-deny Fails

1. **License issue**: Add to `[[licenses.exceptions]]` with justification
2. **Advisory issue**:
   - Direct dep: Update or patch
   - Transitive: Add to `[advisories.ignore]` with comment
3. **Banned crate**: Remove or add to `[bans.skip]`

### When pnpm audit Fails

1. **Direct dep**: Update with `pnpm update <package>`
2. **Transitive**: Check if parent has update, or use `overrides`
3. **No fix available**: Document risk and add to exceptions

### When Code Patterns Found

1. **XSS patterns**: Use sanitization library or React's built-in escaping
2. **Hardcoded creds**: Move to environment variables

## Integration with Other Skills

| Skill | Integration |
|-------|-------------|
| `/codebase-audit` | Includes security as part of full audit |
| `/token-audit` | No security overlap |
| `/novanet-sync` | Run after schema changes |

## Automation

### Pre-Release Checklist

```bash
# Run before any release
cd tools/novanet && cargo deny check && cargo audit
pnpm audit --audit-level=moderate

# If all pass, proceed with release
```

### Quarterly Review

1. Run `/security-audit exceptions`
2. Check if any exceptions can be removed
3. Update review dates in deny.toml comments
4. Document any new risks

## Files

- `tools/novanet/deny.toml` - Rust security policy
- `.github/workflows/ci.yml` - CI security checks
- `.claude/rules/security.md` - Security coding guidelines
