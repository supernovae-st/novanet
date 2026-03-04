# Claude Code Skill Testing & Validation Patterns

Test patterns for validating Claude Code skills before production deployment.

**Date:** 2026-03-04
**Target:** Nika skills validation framework
**Applies to:** All Claude Code projects using `.claude/skills/`

---

## Table of Contents

1. [Quick Checklist](#quick-checklist)
2. [Testing Framework](#testing-framework)
3. [Test Patterns](#test-patterns)
4. [Validation Criteria](#validation-criteria)
5. [Debugging Failed Tests](#debugging-failed-tests)
6. [Integration with CI/CD](#integration-with-cicd)

---

## Quick Checklist

Before deploying a skill to production (git commit):

**Skill Structure**
- [ ] Directory: `.claude/skills/skill-name/SKILL.md`
- [ ] Frontmatter present: `---` markers, `name:`, `description:`
- [ ] Description includes 3+ trigger keywords
- [ ] Markdown syntax valid (no broken links, proper formatting)

**Content Quality**
- [ ] "When to Use" section lists 3+ specific scenarios
- [ ] Prerequisites documented (what should reader know?)
- [ ] Step-by-step instructions (numbered, clear)
- [ ] 2+ examples (show correct AND incorrect)
- [ ] Validation checklist before claiming success
- [ ] "See Also" section with related skills/docs

**Tool Restrictions**
- [ ] `allowed-tools:` specified if limiting scope
- [ ] Default tools (Read, Grep, Glob) appropriate
- [ ] No security concerns with tool access

**Integration**
- [ ] No contradictions with `.claude/CLAUDE.md`
- [ ] No duplicate functionality with existing skills
- [ ] References checked (links should exist)
- [ ] Ready for git: `git add .claude/skills/skill-name/`

---

## Testing Framework

### Framework: RED-GREEN-REFACTOR for Skills

**Inspiration:** TDD cycle, adapted for skill validation

```
BASELINE (RED)
  └─ Run without skill, see it fail
     ├─ User makes request
     ├─ Claude doesn't have skill
     └─ Response is generic/incomplete

IMPLEMENTATION (GREEN)
  └─ Create skill
     ├─ Write SKILL.md
     ├─ Test with varied prompts
     └─ Skill activates automatically

VALIDATION (REFACTOR)
  └─ Run comprehensive test suite
     ├─ Edge cases
     ├─ Integration
     ├─ Documentation accuracy
     └─ Success criteria met
```

### Test Phases

| Phase | Goal | Methods |
|-------|------|---------|
| 1. Structure | Valid frontmatter, valid markdown | Lint, format check |
| 2. Activation | Skill activates on keywords | Manual prompts, varied wording |
| 3. Instructions | Steps are clear and executable | Walk-through execution |
| 4. Examples | Examples are correct and work | Validate example YAML/code |
| 5. Edge Cases | Handles errors gracefully | Test broken inputs, unclear requests |
| 6. Integration | No conflicts with existing skills | Compare with similar skills |
| 7. Documentation | References exist and are current | Link verification |

---

## Test Patterns

### Pattern 1: Activation Testing

**Objective:** Verify skill activates automatically on relevant prompts

**Test Setup:**
```
User prompt: "Describe your skill activation keywords"
Skill response: Should auto-activate
Expected: Skill content appears in response
```

**Test Cases:**

```bash
# Test 1: Explicit keyword
"I need to validate a Nika workflow"
→ Should activate nika-workflow-validator skill

# Test 2: Implicit trigger
"I'm creating a workflow that calls novanet_generate"
→ Should activate nika-workflow-generator and/or nika-mcp-helper

# Test 3: Negative test (should NOT activate)
"Tell me about Claude Code"
→ Should NOT activate workflow-related skills

# Test 4: Multiple keywords
"Generate and validate a Nika workflow"
→ Should activate both generator and validator skills

# Test 5: Varied wording
"Check my YAML workflow for errors"
→ Should recognize "check" = "validate"
```

**Success Criteria:**
- [ ] Skill appears in response
- [ ] Content is relevant to request
- [ ] Other unrelated skills don't activate

### Pattern 2: Instruction Clarity Testing

**Objective:** Verify steps are clear and can be followed

**Test Method:** Manual walk-through

```
Step 1: Read the skill instructions
Step 2: Follow each step exactly as written
Step 3: Note any ambiguities or unclear points
Step 4: Verify you can complete the task
```

**Example Test:**

```
Skill: nika-workflow-validator
Task: "Validate this workflow"

Step 1: "Read workflow file" - Clear ✅
Step 2: "Extract metadata" - Clear ✅
Step 3: "Validate each verb" - Needs clarification ⚠️
  - Should list expected verbs explicitly
Step 4: "Validate references" - Clear ✅
Step 5: "Report results" - Clear ✅

Result: Mostly clear, one improvement needed
```

**Success Criteria:**
- [ ] All steps understood without external reference
- [ ] Steps are in logical order
- [ ] Unclear points have been clarified
- [ ] Can follow instructions without getting stuck

### Pattern 3: Example Validation Testing

**Objective:** Verify examples are correct and help users

**Test Cases:**

```
For each example:
1. Read the example
2. Verify it's correct (would actually work)
3. Check it teaches the concept
4. Ensure it's not too simple or too complex
```

**Example Test:**

```yaml
# Example from skill: Valid Nika Workflow

workflow: content-generation
steps:
  - id: load_entity
    invoke: novanet_generate
    params:
      entity: "qr-code"
      locale: "fr-FR"
    use.ctx: entity_context

Verification:
✅ Workflow name is descriptive
✅ Step ID follows naming convention
✅ Tool (novanet_generate) is valid
✅ Parameters (entity, locale) are correct
✅ use.ctx: assignment is clear
✅ Example is realistic and runnable

Result: Example is valid and helpful
```

**Success Criteria:**
- [ ] All examples are factually correct
- [ ] Examples progress in complexity (simple → complex)
- [ ] Examples teach core concepts
- [ ] Examples are realistic and applicable

### Pattern 4: Edge Case Testing

**Objective:** Skill handles unexpected inputs gracefully

**Test Cases:**

```
# Test 1: Malformed input
User: "Validate this invalid YAML [broken code]"
Expected: Skill recognizes malformation, provides feedback

# Test 2: Unclear request
User: "Help with workflow"
Expected: Skill asks clarifying questions, doesn't assume

# Test 3: Out of scope
User: "Generate a Rust async function"
Expected: Skill recognizes out-of-scope, suggests alternative

# Test 4: Missing prerequisites
User: "Create a skill for Nika"
Expected: Skill provides prerequisite information

# Test 5: Contradictory request
User: "Validate this - wait, never mind"
Expected: Skill handles gracefully, asks if user wants help
```

**Success Criteria:**
- [ ] Skill provides helpful feedback for errors
- [ ] Doesn't crash or provide incorrect info
- [ ] Asks clarifying questions when uncertain
- [ ] Gracefully handles out-of-scope requests

### Pattern 5: Integration Testing

**Objective:** Skill doesn't conflict with other skills

**Test Setup:**

```
1. List all related skills in project
2. Test each skill on same prompt
3. Verify no contradictions
4. Check for duplicate functionality
```

**Example Test:**

```
Related skills in .claude/skills/:
- nika-workflow-validator
- nika-workflow-generator
- nika-mcp-integration-helper
- nika-agent-skill-definer

Prompt: "I need to create a workflow that uses MCP tools"

Expected Results:
✅ nika-workflow-generator activates (suggests generation)
✅ nika-mcp-integration-helper activates (explains tools)
✅ nika-workflow-validator ready (for validation)
✅ No contradictions between suggestions
✅ Skills work together without conflicts
```

**Success Criteria:**
- [ ] No contradictions between activated skills
- [ ] No duplicate responses
- [ ] Skills complement each other
- [ ] Each skill stays in its scope

### Pattern 6: Documentation Accuracy Testing

**Objective:** All references and examples are current

**Test Checklist:**

```
For each skill:
- [ ] All file paths exist
- [ ] All code examples are valid
- [ ] All tool names are current (novanet_*, nika:*)
- [ ] All external links work
- [ ] All ADR references are correct
- [ ] Version numbers match current project version
```

**Example Test:**

```
Skill: nika-workflow-validator
Reference Check:

1. "See `.nika/README.md`"
   Check: Does file exist? Is path correct?
   ✅ File exists at correct path

2. "MCP tools: novanet_generate, novanet_query, ..."
   Check: Are these the latest tools?
   ✅ Verified against novanet-mcp/Cargo.toml v0.15.1

3. "Related skill: nika-workflow-generator"
   Check: Does skill exist?
   ✅ File exists at .claude/skills/nika-workflow-generator/SKILL.md

4. "ADR-033 Denomination Forms"
   Check: Is ADR number correct and content current?
   ✅ ADR-033 exists and matches reference

Result: All documentation is accurate and current
```

**Success Criteria:**
- [ ] All file paths resolve correctly
- [ ] All tool/command names are valid
- [ ] All version references are current
- [ ] All links point to existing resources
- [ ] No outdated information

### Pattern 7: Comprehensive Test Suite

**Objective:** Run all test patterns together

**Test Sequence:**

```
1. Structure Check (automated)
   └─ Lint markdown, validate YAML frontmatter

2. Activation Tests (manual)
   └─ Try 5+ different prompts, check activation

3. Instruction Tests (manual walk-through)
   └─ Follow steps exactly, note any issues

4. Example Validation (verification)
   └─ Verify each example is correct

5. Edge Case Tests (manual)
   └─ Try 5+ edge cases, verify graceful handling

6. Integration Tests (multi-skill)
   └─ Test with related skills, check no conflicts

7. Documentation Review (link verification)
   └─ Check all references and external links

Final Report:
├─ ✅ PASS all tests
├─ ✅ READY for production
└─ Ready for: git commit & team sharing
```

---

## Validation Criteria

### Skill Passes Validation If:

**Essential (MUST HAVE)**
- [ ] Frontmatter is valid YAML with `name:` and `description:`
- [ ] Markdown syntax is valid
- [ ] Description includes 3+ trigger keywords
- [ ] At least 2 examples provided (correct and incorrect)
- [ ] Validation checklist section exists
- [ ] No contradiction with project CLAUDE.md
- [ ] Tool restrictions appropriate (allows safe operations)

**Important (SHOULD HAVE)**
- [ ] "When to Use" section with 3+ scenarios
- [ ] Prerequisites documented
- [ ] Step-by-step instructions (numbered)
- [ ] "See Also" section with related resources
- [ ] Examples are realistic and executable
- [ ] No duplicate functionality with existing skills

**Nice to Have (OPTIONAL)**
- [ ] Optional examples directory
- [ ] Embedded diagrams or ASCII art
- [ ] Test cases for different user types
- [ ] Video/screencast reference
- [ ] Community contributions

### Skill Fails Validation If:

**CRITICAL (MUST FIX)**
- ❌ Markdown syntax errors
- ❌ Invalid YAML frontmatter
- ❌ Contradicts project CLAUDE.md
- ❌ Examples are incorrect or don't work
- ❌ Security concerns (unsafe tool usage)

**IMPORTANT (SHOULD FIX)**
- ⚠️ Description too generic (no trigger keywords)
- ⚠️ No examples provided
- ⚠️ Instructions are unclear or incomplete
- ⚠️ Duplicate functionality with existing skill
- ⚠️ All links are broken or outdated

**MINOR (NICE TO FIX)**
- ◐ Could have more examples
- ◐ Could have better formatting
- ◐ Could reference more resources

---

## Debugging Failed Tests

### Issue: Skill Doesn't Activate on Keywords

**Symptoms:**
```
User: "I need to validate a Nika workflow"
Expected: Skill activates
Actual: Skill doesn't appear
```

**Debugging Steps:**

1. **Check frontmatter**
   ```yaml
   ---
   name: nika-workflow-validator
   description: "Validate YAML workflows..."  # Has keywords?
   ---
   ```

2. **Check description keywords**
   - Must include: "validate", "workflow", "YAML"
   - Should include when/where/why

3. **Test alternate phrasings**
   ```
   "Check my workflow"      (check = validate?)
   "Fix this broken YAML"   (YAML = workflow?)
   "Debug my workflow"      (debug = validate?)
   ```

4. **Check for conflicting skills**
   - Do other skills have same description?
   - Do keywords conflict?

**Fix:**
```yaml
# ❌ Before (too generic)
description: "Helps with workflows"

# ✅ After (specific with keywords)
description: "Validate YAML workflows against Nika schema. Use when creating new workflows, debugging syntax errors, or code reviewing."
```

### Issue: Examples Don't Work

**Symptoms:**
```
Example workflow shows: invoke: novanet_xyz
But error: "Unknown tool: novanet_xyz"
```

**Debugging Steps:**

1. **Verify tool exists**
   ```bash
   # Check novanet-mcp tools
   grep -r "pub fn novanet_" tools/novanet-mcp/src/
   ```

2. **Check spelling**
   - `novanet_generate` (not `generate`)
   - `novanet_query` (not `query`)

3. **Verify parameters**
   ```yaml
   # Check tool schema
   # Example: novanet_generate requires: entity, locale
   invoke: novanet_generate
   params:
     entity: "qr-code"         # ✅ Required
     locale: "en-US"           # ✅ Required
   ```

**Fix:**
```yaml
# ❌ Wrong tool name
invoke: novanet_xyz

# ✅ Correct tool name
invoke: novanet_generate
params:
  entity: "my-entity"
  locale: "en-US"
```

### Issue: Steps Are Unclear

**Symptoms:**
```
User tries to follow instructions and gets stuck:
"Step 3: Validate something... but I don't know what to validate or how"
```

**Debugging Steps:**

1. **Read step aloud** - Does it sound clear?
2. **Ask yourself** - Could you do it without extra help?
3. **Test with non-expert** - Would they understand?

**Fix:**
```markdown
# ❌ Too vague
## Step 3: Validate References
Ensure all variables are defined.

# ✅ Clear with example
## Step 3: Check Variable Definitions

1. List all `$variable` references in the workflow
2. For each reference:
   - Find the step that defines it (with `use.ctx:`)
   - Verify definition happens BEFORE reference
3. Report errors for undefined variables

Example:
  ❌ Step 5 uses $data but step 1 never defines it
  ✅ Step 1 defines $data, step 5 uses it
```

### Issue: Instructions Contradict Each Other

**Symptoms:**
```
"Use infer: for LLM tasks"
Later: "For reasoning, use invoke:"
```

**Debugging Steps:**

1. **List all contradictions**
2. **Check against official docs** (CLAUDE.md, Nika docs)
3. **Pick correct version** and update

**Fix:**
```markdown
# ❌ Contradictory
Use `infer:` for LLM tasks.
...
For reasoning, use `invoke:`.

# ✅ Consistent
Use `infer:` for LLM tasks including reasoning.
Use `invoke:` for calling MCP tools (novanet_*, etc.).
```

---

## Integration with CI/CD

### GitHub Actions: Skill Validation

**File:** `.github/workflows/skill-validation.yml`

```yaml
name: Skill Validation

on:
  pull_request:
    paths:
      - '.claude/skills/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Markdown lint
        run: |
          find .claude/skills -name "SKILL.md" \
            -exec grep -l "^---" {} \;  # Check frontmatter

      - name: YAML frontmatter validation
        run: |
          find .claude/skills -name "SKILL.md" -exec yq . {} \;

      - name: Check for required fields
        run: |
          find .claude/skills -name "SKILL.md" | while read f; do
            echo "Checking: $f"
            grep -q "^name:" "$f" || exit 1
            grep -q "^description:" "$f" || exit 1
            grep -q "^# " "$f" || exit 1
            grep -q "## When to Use" "$f" || exit 1
          done

      - name: Comment on PR
        if: failure()
        uses: actions/github-script@v6
        with:
          script: |
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: '❌ Skill validation failed. Please check frontmatter and required sections.'
            })
```

### Pre-commit Hook: Local Skill Validation

**File:** `.git/hooks/pre-commit`

```bash
#!/bin/bash

# Check for SKILL.md files in changes
SKILL_FILES=$(git diff --cached --name-only | grep "\.claude/skills.*SKILL.md")

if [ -z "$SKILL_FILES" ]; then
  exit 0
fi

echo "Validating skills..."

for SKILL in $SKILL_FILES; do
  # Check frontmatter
  if ! head -1 "$SKILL" | grep -q "^---"; then
    echo "❌ $SKILL missing frontmatter start"
    exit 1
  fi

  # Check required fields
  if ! grep -q "^name:" "$SKILL"; then
    echo "❌ $SKILL missing 'name:' field"
    exit 1
  fi

  if ! grep -q "^description:" "$SKILL"; then
    echo "❌ $SKILL missing 'description:' field"
    exit 1
  fi

  echo "✅ $SKILL is valid"
done

exit 0
```

---

## Testing Checklist for Production Deployment

**Before `git commit .claude/skills/my-skill/`:**

### Skill Structure
- [ ] Directory exists: `.claude/skills/my-skill/`
- [ ] File exists: `SKILL.md`
- [ ] No unnecessary files in skill directory

### Frontmatter
- [ ] Starts with `---`
- [ ] Contains `name:` field
- [ ] Contains `description:` field
- [ ] Ends with `---`
- [ ] Valid YAML syntax

### Content Quality
- [ ] Main heading matches skill name
- [ ] "When to Use" section (3+ scenarios)
- [ ] Prerequisites listed
- [ ] Step-by-step instructions (numbered)
- [ ] At least 2 examples (working + broken)
- [ ] Validation checklist provided
- [ ] "See Also" section with links
- [ ] All markdown links work (local + external)

### Activation
- [ ] Description has 3+ trigger keywords
- [ ] Keywords natural and relevant
- [ ] Not generic (avoid: "helps with X")

### Documentation
- [ ] No broken internal links
- [ ] No outdated version references
- [ ] Tool names match current versions
- [ ] ADR numbers correct
- [ ] File paths exist

### Integration
- [ ] No contradictions with CLAUDE.md
- [ ] No duplicate skills in project
- [ ] Tool restrictions appropriate
- [ ] No security concerns

### Testing
- [ ] Manually tested activation (5+ prompts)
- [ ] Manually tested instructions (walked through)
- [ ] Validated all examples work
- [ ] Tested edge cases (malformed input, etc.)
- [ ] Tested with related skills (no conflicts)

### Final Review
- [ ] Markdown is well-formatted
- [ ] Professional tone and language
- [ ] No typos or grammar errors
- [ ] Ready for team git repository

---

**Version:** 1.0
**Created:** 2026-03-04
**Status:** Ready for production use
