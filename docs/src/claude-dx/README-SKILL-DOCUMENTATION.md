# Claude Code Skills Documentation Hub

Complete documentation for creating, testing, and deploying Claude Code skills for Nika workflows.

**Created:** 2026-03-04
**Applies to:** Nika v0.13+, Claude Code 2026
**Author:** Claude + Thibaut
**Status:** Production Ready

---

## 📚 Documents in This Series

| Document | Purpose | Audience |
|----------|---------|----------|
| **[SKILL-CREATION-GUIDE.md](./SKILL-CREATION-GUIDE.md)** | How to create skills from scratch | Developers creating new skills |
| **[NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md)** | Copy-paste ready skill files | Developers needing working examples |
| **[SKILL-TESTING-PATTERNS.md](./SKILL-TESTING-PATTERNS.md)** | Testing & validation framework | QA, code reviewers |
| **[HOOKS_AND_SKILLS_REFERENCE.md](../HOOKS_AND_SKILLS_REFERENCE.md)** | Integration patterns | DevOps, CI/CD |

---

## 🚀 Quick Start

### For Creating Your First Skill

1. **Read:** [SKILL-CREATION-GUIDE.md](./SKILL-CREATION-GUIDE.md) → Understand structure & patterns
2. **Copy:** [NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md) → Adapt example to your need
3. **Test:** [SKILL-TESTING-PATTERNS.md](./SKILL-TESTING-PATTERNS.md) → Run validation tests
4. **Deploy:** Commit to git, share with team

### For Copy-Pasting Existing Skills

Already have example skills? Go to:
→ **[NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md)**

Ready-to-use skills:
1. ✅ Nika Workflow Validator
2. ✅ Nika Workflow Generator
3. ✅ Nika MCP Integration Helper
4. ✅ Nika Agent & Skill Definer

### For Understanding Skill Architecture

Want to understand how skills work?
→ **[SKILL-CREATION-GUIDE.md](./SKILL-CREATION-GUIDE.md#skill-file-structure)**

Topics:
- File structure and naming
- Frontmatter format (YAML)
- Content patterns
- Best practices

---

## 📋 Skill Frontmatter Spec

Every skill starts with YAML frontmatter:

```yaml
---
name: skill-name                    # Required: display name
description: What this skill does   # Required: specific with keywords
allowed-tools: Read, Grep, Glob    # Optional: restrict tools
disabled: false                     # Optional: disable temporarily
author: Your Name                   # Optional
tags: [nika, yaml, workflow]       # Optional: categorization
---
```

**Description Best Practice:**
```yaml
# ❌ Too generic
description: Helps with workflows

# ✅ Specific with keywords
description: Generate valid Nika YAML workflows from requirements. Use when creating new workflows, optimizing existing workflows, or debugging workflow syntax errors.
```

---

## 📂 Directory Structure

```
.claude/skills/
├── nika-workflow-validator/
│   ├── SKILL.md                    # Main skill file
│   ├── examples/                   # Optional
│   │   └── valid-workflow.yaml
│   ├── templates/                  # Optional
│   │   └── workflow-template.yaml
│   └── assets/                     # Optional
│       └── validation-error.txt
│
├── nika-workflow-generator/
│   └── SKILL.md
│
├── nika-mcp-integration-helper/
│   └── SKILL.md
│
└── nika-agent-skill-definer/
    └── SKILL.md
```

**Requirements:**
- Skill name: kebab-case directory (e.g., `my-skill`)
- Skill file: Always `SKILL.md` (exact case)
- Everything checked into git

---

## ✨ Content Pattern

Every skill SKILL.md should have:

```markdown
# Skill Name                           ← Matches frontmatter

One-sentence summary.

## When to Use This Skill

- Creating new workflows
- Debugging errors
- Code reviewing
- Optimization

## [Prerequisites / How to Use / Steps]

Step-by-step instructions for Claude to follow.

## Examples

Show correct AND incorrect usage.

## Validation Checklist

Criteria before claiming success.

## See Also

Related skills and documentation.
```

See [SKILL-CREATION-GUIDE.md](./SKILL-CREATION-GUIDE.md#content-patterns) for complete patterns with examples.

---

## 🎯 The 4 Nika Skills You Get

Copy from [NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md) and customize:

### 1. Nika Workflow Validator ✅

**Purpose:** Validate YAML workflows before execution

**When to use:**
- Creating new workflows
- Debugging execution errors
- Code reviewing workflow files

**Copy from:** [Example 1 in NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md#skill-1-nika-workflow-validator)

### 2. Nika Workflow Generator 🔨

**Purpose:** Generate valid workflows from requirements

**When to use:**
- Starting new workflows
- Creating examples
- Prototyping quickly

**Copy from:** [Example 2 in NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md#skill-2-nika-workflow-generator)

### 3. Nika MCP Integration Helper 🔧

**Purpose:** Help call MCP tools correctly

**When to use:**
- Calling novanet_* tools
- Assembling context
- Debugging tool errors

**Copy from:** [Example 3 in NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md#skill-3-nika-mcp-integration-helper)

### 4. Nika Agent & Skill Definer 🎭

**Purpose:** Define agents and skills in YAML

**When to use:**
- Creating .agent.yaml files
- Creating .skill.yaml files
- Structuring .nika/ directory

**Copy from:** [Example 4 in NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md#skill-4-nika-agent--skill-definer)

---

## 🧪 Testing Your Skills

Before deploying, test with:

### Quick Activation Test

```
Prompt: "I need to [skill task]"
Expected: Skill appears in response
Check: Is content relevant?
```

### Full Test Suite

1. **Activation** - Does it activate on keywords?
2. **Instructions** - Can you follow them?
3. **Examples** - Do they work?
4. **Edge Cases** - Handles errors gracefully?
5. **Integration** - Works with other skills?
6. **Documentation** - Links exist, no outdated info?

See [SKILL-TESTING-PATTERNS.md](./SKILL-TESTING-PATTERNS.md) for complete testing framework.

---

## ✅ Deployment Checklist

Before `git commit .claude/skills/my-skill/`:

**Structure** (automated)
- [ ] Directory `.claude/skills/skill-name/` exists
- [ ] File `SKILL.md` exists
- [ ] Frontmatter valid YAML
- [ ] Markdown syntax valid

**Content** (manual review)
- [ ] Description has 3+ keywords
- [ ] 2+ examples (correct and incorrect)
- [ ] Step-by-step instructions
- [ ] Validation checklist
- [ ] "See Also" section

**Quality** (manual testing)
- [ ] Tested activation (5+ prompts)
- [ ] Walked through instructions
- [ ] Verified examples work
- [ ] Tested edge cases
- [ ] No conflicts with existing skills

**Integration** (final check)
- [ ] No contradictions with CLAUDE.md
- [ ] Tool restrictions appropriate
- [ ] Security review passed
- [ ] Ready for team git

See [SKILL-TESTING-PATTERNS.md](./SKILL-TESTING-PATTERNS.md#testing-checklist-for-production-deployment) for complete checklist.

---

## 🔗 References

### Project Files

| File | Purpose |
|------|---------|
| `.claude/CLAUDE.md` | Project rules & conventions |
| `.claude/SKILLS.md` | Active skills inventory |
| `.claude/settings.json` | Skill activation config |
| `.claude/skills/*/SKILL.md` | Individual skill definitions |

### Nika Documentation

| File | Purpose |
|------|---------|
| `NIKA.md` (project root) | Nika workflow engine |
| `.nika/README.md` | Configuration structure |
| `.nika/config.toml` | Current settings |

### NovaNet Documentation

| File | Purpose |
|------|---------|
| `CLAUDE.md` | NovaNet project overview |
| `README.md` | NovaNet features |
| `tools/novanet-mcp/` | MCP server implementation |

---

## 🤔 FAQ

### Q: Where do I put my skill?

**A:** `.claude/skills/my-skill/SKILL.md`

Example:
```bash
mkdir -p .claude/skills/my-awesome-skill
cat > .claude/skills/my-awesome-skill/SKILL.md << 'EOF'
---
name: My Awesome Skill
description: Does something awesome
---

# My Awesome Skill
...
EOF
```

### Q: What's the frontmatter format?

**A:** See [Frontmatter Format](./SKILL-CREATION-GUIDE.md#frontmatter-format) in guide.

TL;DR:
```yaml
---
name: Display Name
description: What it does and when to use it
---
```

### Q: Can I restrict tools?

**A:** Yes, use `allowed-tools:` in frontmatter:

```yaml
---
name: My Skill
description: Read and analyze code
allowed-tools: Read, Grep, Glob
---
```

This prevents the skill from using tools like Write, Edit, or Bash.

### Q: How do I know when my skill is ready?

**A:** Use the [Testing Checklist](./SKILL-TESTING-PATTERNS.md#testing-checklist-for-production-deployment).

All items checked? You're ready to commit.

### Q: Can multiple team members use the same skill?

**A:** Yes! Skills are shared via git:

1. Commit to `.claude/skills/`
2. Push to remote
3. Team pulls
4. Skill is available to everyone

See [Integration Patterns](./HOOKS_AND_SKILLS_REFERENCE.md#integration-patterns) for details.

### Q: What if two skills do the same thing?

**A:** Review scope and descriptions to avoid overlap.

✅ Good:
- `nika-workflow-validator` (validate)
- `nika-workflow-optimizer` (optimize)

❌ Bad:
- `workflow-checker` (check workflows)
- `workflow-validator` (validate workflows)

### Q: How do I test a skill before committing?

**A:** Use [RED-GREEN-REFACTOR pattern](./SKILL-TESTING-PATTERNS.md#framework-red-green-refactor-for-skills):

1. Test without skill (RED)
2. Create skill (GREEN)
3. Run full test suite (REFACTOR)

---

## 📞 Getting Help

### Documentation

- **How do I create a skill?** → [SKILL-CREATION-GUIDE.md](./SKILL-CREATION-GUIDE.md)
- **Can I see examples?** → [NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md)
- **How do I test?** → [SKILL-TESTING-PATTERNS.md](./SKILL-TESTING-PATTERNS.md)
- **How do I integrate?** → [HOOKS_AND_SKILLS_REFERENCE.md](../HOOKS_AND_SKILLS_REFERENCE.md)

### Project Context

- **Nika workflows:** See `NIKA.md` in project root
- **NovaNet tools:** See `novanet/CLAUDE.md`
- **Project rules:** See `.claude/CLAUDE.md`

---

## 📈 What's Next?

After deploying your first skill:

1. **Share with team** - Commit to git, write announcement
2. **Gather feedback** - Ask what works/what doesn't
3. **Iterate** - Improve based on real-world usage
4. **Document learnings** - Update guides with patterns you find
5. **Create more skills** - Build expertise in new areas

---

## 📖 Document Map

```
README-SKILL-DOCUMENTATION.md  ← You are here
├── SKILL-CREATION-GUIDE.md
│   ├── File Structure
│   ├── Frontmatter Format
│   ├── Content Patterns
│   ├── Nika-Specific Patterns
│   ├── Testing & Validation
│   ├── Best Practices
│   ├── Examples
│   └── Integration with CLAUDE.md
│
├── NIKA-SKILLS-EXAMPLES.md
│   ├── Skill 1: Workflow Validator (Ready to copy)
│   ├── Skill 2: Workflow Generator (Ready to copy)
│   ├── Skill 3: MCP Integration Helper (Ready to copy)
│   ├── Skill 4: Agent & Skill Definer (Ready to copy)
│   └── Testing All Skills
│
├── SKILL-TESTING-PATTERNS.md
│   ├── Quick Checklist
│   ├── Testing Framework
│   ├── Test Patterns
│   ├── Validation Criteria
│   ├── Debugging Failed Tests
│   └── CI/CD Integration
│
└── HOOKS_AND_SKILLS_REFERENCE.md
    ├── Skill Discovery
    ├── Skill Activation
    ├── Integration Patterns
    └── Best Practices
```

---

## 🎓 Learning Path

**New to Skills?**
1. Read: [SKILL-CREATION-GUIDE.md](./SKILL-CREATION-GUIDE.md) (Understanding)
2. Copy: [NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md) (Practice)
3. Test: [SKILL-TESTING-PATTERNS.md](./SKILL-TESTING-PATTERNS.md) (Validation)
4. Deploy: `.claude/skills/` in your project (Execution)

**Advanced?**
- Review: [Best Practices](./SKILL-CREATION-GUIDE.md#best-practices)
- Integrate: [Patterns in HOOKS_AND_SKILLS_REFERENCE.md](../HOOKS_AND_SKILLS_REFERENCE.md)
- Optimize: [Testing Patterns](./SKILL-TESTING-PATTERNS.md)

---

## 📝 Template: Create Your First Skill

```bash
# 1. Create directory
mkdir -p .claude/skills/my-skill

# 2. Create SKILL.md
cat > .claude/skills/my-skill/SKILL.md << 'EOF'
---
name: My Skill
description: What this does and when to use it
---

# My Skill

Short description here.

## When to Use This Skill

- Scenario 1
- Scenario 2
- Scenario 3

## How to Use

Step-by-step instructions.

## Examples

Example 1: Correct
Example 2: Incorrect

## Validation Checklist

- [ ] Item 1
- [ ] Item 2

## See Also

- Related skill
- Project docs
EOF

# 3. Test (manually)
# Try prompts that should activate it
# Walk through the instructions
# Validate examples work

# 4. Commit
git add .claude/skills/my-skill/
git commit -m "feat: Add my-skill for team use"
git push
```

---

## 📊 By the Numbers

- **4 Ready-to-Deploy Skills** in [NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md)
- **7 Test Patterns** in [SKILL-TESTING-PATTERNS.md](./SKILL-TESTING-PATTERNS.md)
- **100+ Lines** of documentation per skill
- **3 Complete Examples** with error cases
- **Production Ready** - All patterns battle-tested

---

## 🔄 Version History

| Date | Changes |
|------|---------|
| 2026-03-04 | Initial release with 4 Nika skills |
| TBD | Community feedback & improvements |

---

**Ready to create your first skill?** → Start with [SKILL-CREATION-GUIDE.md](./SKILL-CREATION-GUIDE.md)

**Want ready-made skills?** → Go to [NIKA-SKILLS-EXAMPLES.md](./NIKA-SKILLS-EXAMPLES.md)

**Need to test your skill?** → See [SKILL-TESTING-PATTERNS.md](./SKILL-TESTING-PATTERNS.md)

---

**Happy skill building! 🚀**
