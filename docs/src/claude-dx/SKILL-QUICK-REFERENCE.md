# Claude Code Skills - Quick Reference Card

One-page cheat sheet for creating Claude Code skills.

---

## File Structure

```
.claude/skills/my-skill/
└── SKILL.md                    # Only file required
```

## Frontmatter (YAML)

```yaml
---
name: Display Name              # Required
description: What it does       # Required, include keywords
allowed-tools: Read, Grep, Glob # Optional: restrict tools
---
```

## Content Template

```markdown
# Skill Name

One sentence summary.

## When to Use This Skill

- Scenario 1
- Scenario 2
- Scenario 3

## [Prerequisites / How to Use / Steps]

Step-by-step instructions.

## Examples

✅ Correct example
❌ Incorrect example with explanation

## Validation Checklist

- [ ] Item 1
- [ ] Item 2

## See Also

- Related skill
- Documentation link
```

## Nika Verbs (For Nika Skills)

| Verb | Use For | Example |
|------|---------|---------|
| `infer:` | LLM tasks | `infer: "Generate text"` |
| `exec:` | Shell commands | `exec: "echo hello"` |
| `fetch:` | HTTP requests | `fetch: { url: "..." }` |
| `invoke:` | MCP tools | `invoke: novanet_generate` |
| `agent:` | Agent delegation | `agent: { agent: name }` |

## MCP Tools (Common)

```yaml
# Generate content with full context
invoke: novanet_generate
params:
  focus_key: entity-name
  locale: en-US

# Search entities
invoke: novanet_search
params:
  query: search-term
  mode: fulltext

# Custom Cypher query
invoke: novanet_query
params:
  cypher: "MATCH (n) RETURN n LIMIT 5"

# Get knowledge atoms
invoke: novanet_atoms
params:
  locale: en-US
  atom_type: term
```

## Description Best Practice

❌ **Too generic:**
```
"Helps with Nika workflows"
```

✅ **Specific with keywords:**
```
"Validate YAML workflows against Nika schema. Use when creating new workflows, debugging syntax errors, or code reviewing."
```

## Activation Keywords

Include in description:
- When: "Use when..."
- Why: "Use for..."
- Where: "Use with..."
- What: "Use to..."

Example:
```
Use when creating workflows, debugging errors, or code reviewing.
```

## Examples Pattern

Always show both:

```markdown
### Example 1: Correct ✅

[Good example with explanation]

### Example 2: Incorrect ❌

[Bad example with explanation of error]
```

## Testing Before Deployment

| Test | Method |
|------|--------|
| Activation | Try 5+ different prompts |
| Instructions | Walk through steps |
| Examples | Verify they work |
| Edge cases | Test error handling |
| Integration | Check other skills |
| Documentation | Verify links exist |

## Deployment Checklist

- [ ] Frontmatter valid (name, description)
- [ ] Markdown syntax valid
- [ ] 3+ trigger keywords in description
- [ ] 2+ examples (correct + incorrect)
- [ ] Validation checklist section
- [ ] All links work
- [ ] No contradictions with CLAUDE.md
- [ ] Tested with multiple prompts
- [ ] Ready: `git commit .claude/skills/my-skill/`

## Common Tool Restrictions

**Safe (read-only):**
```yaml
allowed-tools: Read, Grep, Glob
```

**With write access:**
```yaml
allowed-tools: Read, Grep, Glob, Edit, Write
```

**Full access (unrestricted):**
```yaml
# Don't specify allowed-tools
# Claude will ask for permission
```

## Naming Convention

| Item | Convention | Example |
|------|-----------|---------|
| Skill directory | kebab-case | `my-awesome-skill` |
| Skill file | `SKILL.md` | Always `SKILL.md` |
| Workflow name | kebab-case | `content-generator` |
| Step ID | snake_case | `load_entity` |
| Variable | snake_case | `$entity_context` |

## Variable Flow in Workflows

```yaml
# Step 1: Define variable
steps:
  - id: fetch_data
    fetch:
      url: "..."
    use.ctx: data              # ✅ Defines $data

# Step 2: Use variable
  - id: process
    infer: "Process: $data"     # ✅ Uses $data
    context: $data
    use.ctx: result
```

## Error Messages

| Error | Cause | Fix |
|-------|-------|-----|
| Skill doesn't activate | Generic description | Add specific keywords |
| Steps unclear | Too vague | Number steps, add examples |
| Examples don't work | Outdated tools | Verify tool names |
| Can't follow instructions | Missing context | Add prerequisites |

## One-Minute Skill Creation

```bash
# 1. Make directory
mkdir -p .claude/skills/my-skill

# 2. Create SKILL.md with template above
# 3. Fill in your content
# 4. Test with varied prompts
# 5. Commit to git
git add .claude/skills/my-skill/
git commit -m "feat: Add my-skill"
```

## Quick Links

| Need | Link |
|------|------|
| Full guide | `SKILL-CREATION-GUIDE.md` |
| Copy-paste skills | `NIKA-SKILLS-EXAMPLES.md` |
| Testing patterns | `SKILL-TESTING-PATTERNS.md` |
| Project rules | `.claude/CLAUDE.md` |
| Skills inventory | `.claude/SKILLS.md` |

## Red Flags (Don't Commit)

🚫 Frontmatter missing or malformed
🚫 Description too generic
🚫 No examples provided
🚫 Contradicts CLAUDE.md
🚫 Broken links or outdated references
🚫 Unclear instructions
🚫 Duplicate functionality with existing skill
🚫 Security concerns (unsafe tool usage)

## Pro Tips

✨ **Descriptive names:** `nika-workflow-validator` not `workflow-tool`

✨ **Clear keywords:** Description should answer "when should Claude use this?"

✨ **Real examples:** Use actual Nika workflows, not made-up scenarios

✨ **Error handling:** Show both success AND failure cases

✨ **Version info:** Update when tool versions change

✨ **Team sharing:** Commit to git for automatic team access

---

**Print this card for quick reference while creating skills!**

Last updated: 2026-03-04
