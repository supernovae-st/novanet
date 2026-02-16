# Claude Code Hooks Documentation Index

Complete reference documentation for Claude Code hooks architecture, patterns, and best practices.

---

## 📚 Documentation Suite

### 1. **CLAUDE-CODE-HOOKS-SUMMARY.md** (START HERE)
**Entry point for understanding hooks**

- What are hooks?
- Core concepts
- Quick reference
- Common patterns overview
- Best practices summary
- Next steps

**Read this first** (15 min) for comprehensive understanding.

---

### 2. **claude-code-hooks-patterns.md** (DEEP DIVE)
**Complete architecture and design patterns**

Sections:
- Hook architecture overview
- Optimal hook structure & organization
- JSON input/output format patterns
- Event filtering (matchers & path patterns)
- Hook handler types (command, prompt, agent)
- Decision control patterns
- Configuration-driven patterns
- Performance considerations
- Practical implementation examples
- Best practices summary
- Hook lifecycle diagram
- Troubleshooting checklist

**Read this** (30 min) for comprehensive understanding of how hooks work internally.

---

### 3. **claude-code-hooks-recipes.md** (COPY-PASTE)
**18 Ready-to-use hook configurations**

Recipes included:
1. Block dangerous commands
2. Protect sensitive files
3. Auto-format code
4. Log all tool usage
5. Require tests pass
6. Desktop notifications
7. Auto-lint before stopping
8. Inject context after compaction
9. Validate file paths
10. Auto-update timestamps
11. Multi-hook chain
12. Permission decision
13. MCP tool hooks
14. Dynamic input modification
15. Git pre-commit integration
16. Conditional async logging
17. Rate limiting
18. MCP authorization

**Use this** (5 min) to copy working configurations for your use case.

---

### 4. **claude-code-hooks-architecture.md** (VISUAL REFERENCE)
**Diagrams and flowcharts**

Diagrams included:
1. Hook execution architecture
2. Event filtering flow
3. JSON input/output flow
4. PreToolUse decision tree
5. Complete hook lifecycle
6. Handler type decision tree
7. Configuration priority & merging
8. Error handling & exit codes
9. Matcher regex engine
10. JSON output structure (PreToolUse)
11. MCP tool naming pattern
12. Deduplication logic
13. Async hook flow
14. Stop hook infinite loop prevention
15. Summary: Hook architecture

**Reference this** (3 min) when you need to visualize how hooks work.

---

## 🎯 Quick Navigation by Task

### I want to...

| Goal | Document | Section |
|------|----------|---------|
| Understand what hooks are | SUMMARY | What Are Hooks? |
| Learn hook events | PATTERNS | Section 3 |
| Understand matchers | PATTERNS | Section 4 |
| Block dangerous commands | RECIPES | Recipe 1 |
| Protect sensitive files | RECIPES | Recipe 2 |
| Auto-format code | RECIPES | Recipe 3 |
| Require tests before stop | RECIPES | Recipe 4 |
| See full lifecycle | ARCHITECTURE | Diagram 5 |
| Understand JSON I/O | PATTERNS | Section 3 |
| Debug my hook | PATTERNS | Section 12 |
| Set timeouts | PATTERNS | Section 8 |
| Use async hooks | ARCHITECTURE | Diagram 13 |
| Handle MCP tools | RECIPES | Recipe 13 |
| Prevent infinite loops | ARCHITECTURE | Diagram 14 |

---

## 📋 Hook Events Reference

### Lifecycle Events (In Order)

1. **SessionStart** - Session begins or resumes
   - Matchers: startup, resume, clear, compact
   - Use: Inject context, set env vars
   - Docs: PATTERNS S2, RECIPES R7, R8

2. **UserPromptSubmit** - User submits prompt
   - Matchers: None
   - Use: Validate input, add context
   - Docs: PATTERNS S3

3. **PreToolUse** - Before tool executes (can block)
   - Matchers: tool_name (regex)
   - Use: Block dangerous, modify input, validate
   - Docs: PATTERNS S4, RECIPES R1-R3

4. **PostToolUse** - Tool executed successfully
   - Matchers: tool_name
   - Use: Format, log, audit
   - Docs: RECIPES R3

5. **PostToolUseFailure** - Tool failed
   - Matchers: tool_name
   - Use: Log errors, suggest fixes
   - Docs: PATTERNS S10

6. **PermissionRequest** - Permission needed
   - Matchers: tool_name
   - Use: Auto-approve/deny, modify perms
   - Docs: RECIPES R12

7. **Stop** - Claude stops responding
   - Matchers: None
   - Use: Require tests, block until done
   - Docs: RECIPES R4, R5, ARCHITECTURE D14

8. **SubagentStart** - Subagent spawned
   - Matchers: agent_type
   - Use: Track parallel tasks
   - Docs: PATTERNS S10

9. **SubagentStop** - Subagent finished
   - Matchers: agent_type
   - Use: Aggregate results
   - Docs: PATTERNS S10

10. **PreCompact** - Context compacting
    - Matchers: manual, auto
    - Use: Archive transcript
    - Docs: PATTERNS S3

11. **Notification** - Status message
    - Matchers: notification_type
    - Use: Send to Slack/webhooks
    - Docs: RECIPES R6

12. **SessionEnd** - Session closes
    - Matchers: clear, logout, other
    - Use: Cleanup, final logging
    - Docs: PATTERNS S2

---

## 🔧 Configuration Reference

### Hook Event Names
```
SessionStart, UserPromptSubmit, PreToolUse, PostToolUse,
PostToolUseFailure, PermissionRequest, Stop, SubagentStart,
SubagentStop, PreCompact, Notification, SessionEnd, TeammateIdle,
TaskCompleted
```

### Common Matchers
```
PreToolUse:    Bash, Write|Edit, Read, Glob, Grep, mcp__.*
SessionStart:  startup, resume, clear, compact
Notification:  permission_prompt, idle_prompt, auth_success
SubagentStart: Bash, Explore, Plan, or custom agent names
```

### Handler Types
```
command  →  Shell script execution
prompt   →  Single LLM call (Haiku)
agent    →  Multi-turn subagent with tools
```

### Configuration Locations
```
.claude/settings.local.json     (highest priority)
.claude/settings.json           (team shared)
~/.claude/settings.json         (user global)
Managed policies (org)
Plugin hooks/hooks.json
Skill/Agent frontmatter
```

---

## 📊 Quick Reference Table

| Aspect | Details | Docs |
|--------|---------|------|
| **Hook Input** | JSON via stdin, event-specific fields | PATTERNS S3 |
| **Hook Output** | Exit codes (0/2) or JSON + exit 0 | PATTERNS S3 |
| **Matchers** | Regex patterns, case-sensitive, event-specific | PATTERNS S4 |
| **Timeouts** | Command: 600s, Prompt: 30s, Agent: 60s | PATTERNS S8 |
| **Async** | Can run non-blocking with async: true | PATTERNS S8 |
| **Dedup** | Identical handlers run once | ARCHITECTURE D12 |
| **Chaining** | Multiple hooks run in order, parallel exec | PATTERNS S7 |
| **Config Merge** | Multiple sources combined, not replaced | ARCHITECTURE D7 |
| **Exit Codes** | 0=allow, 2=deny, 1/3+=non-blocking error | PATTERNS S3 |

---

## 🚀 Getting Started

### Step 1: Understand (15 min)
Read: **CLAUDE-CODE-HOOKS-SUMMARY.md**

### Step 2: Learn (30 min)
Read: **claude-code-hooks-patterns.md** sections 1-6

### Step 3: Choose Pattern (5 min)
Browse: **claude-code-hooks-recipes.md** for your use case

### Step 4: Implement (10 min)
Copy recipe to `.claude/settings.json` or `.claude/hooks/script.sh`

### Step 5: Test (5 min)
```bash
echo '{"tool_name":"Bash"}' | ./my-hook.sh
/hooks  # Check in menu
```

### Step 6: Deploy (2 min)
Commit to repo or add to personal settings

---

## ⚡ Common Workflows

### Workflow: Add Security Hook

1. Read: RECIPES R1 (Block dangerous commands)
2. Copy script template
3. Test: `echo '{"tool_name":"Bash","tool_input":{"command":"rm -rf /"}}' | ./script.sh`
4. Add to `.claude/settings.json`
5. Verify: `/hooks` menu shows hook

### Workflow: Auto-Format on Save

1. Read: RECIPES R3 (Auto-format code)
2. Copy JSON configuration
3. Add to `.claude/settings.json`
4. Test by editing a file
5. Verify: File formatted automatically

### Workflow: Require Tests Before Stop

1. Read: RECIPES R4 (Require tests)
2. Copy script, make executable
3. Add to `.claude/settings.json`
4. Test by editing + asking to stop
5. Verify: Tests run, stop blocked until pass

---

## 🐛 Debugging

### Check Hook Status
```bash
/hooks  # Interactive menu shows all hooks
```

### Test Script Manually
```bash
echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
echo $?  # Exit code
```

### View Verbose Output
```bash
Ctrl+O  # Toggle verbose mode in Claude Code
```

### Common Issues
- See: PATTERNS Section 12 (Troubleshooting)
- Or: RECIPES (Error Handling Patterns)

---

## 📖 Format Guide

**SUMMARY** (this doc):
- Overview, quick start
- 15 minutes to understand
- Entry point for new users

**PATTERNS**:
- Comprehensive deep dive
- 30 minutes to read
- Reference for internal workings

**RECIPES**:
- Copy-paste configurations
- 5 minutes per recipe
- Quick implementation

**ARCHITECTURE**:
- Visual flowcharts
- 3 minutes to reference
- When you need diagrams

---

## 🔗 External Resources

- **Official Hooks Guide**: https://code.claude.com/docs/en/hooks-guide
- **Hooks Reference**: https://code.claude.com/docs/en/hooks
- **Agent SDK Hooks**: https://docs.anthropic.com/en/docs/agent-sdk/hooks
- **Examples**: https://github.com/anthropics/claude-code/tree/main/examples/hooks

---

## ✨ Key Insights

1. **Hooks are deterministic** — use for enforcement, not judgment
2. **Matchers are critical** — precise filtering reduces overhead
3. **JSON I/O is simple** — stdin, exit codes, stdout
4. **Three handler types** — command (common), prompt (judgment), agent (complex)
5. **Async support** — non-blocking background tasks
6. **Configuration merges** — multiple sources combine
7. **Deduplication automatic** — no harm in redundancy
8. **Stop hooks require care** — prevent infinite loops with `stop_hook_active`

---

## 📝 Version & Date

- **Analysis Date**: February 2026
- **Claude Code Version**: Latest (knowledge cutoff)
- **Hook Events Covered**: 12+ events
- **Handler Types**: 3
- **Total Recipes**: 18
- **Total Documentation**: 4 files (this index + 3 deep-dives)

---

## 🎓 Recommended Reading Order

For beginners:
1. SUMMARY (15 min)
2. RECIPES (pick 1-2, 5 min each)
3. Test & iterate

For intermediate:
1. SUMMARY (15 min)
2. PATTERNS S1-6 (20 min)
3. ARCHITECTURE D5 (2 min)
4. RECIPES (all patterns, 30 min)
5. Build custom hooks

For advanced:
1. PATTERNS (all, 40 min)
2. ARCHITECTURE (all, 10 min)
3. Read official reference
4. Build sophisticated multi-hook systems

---

Last updated: February 16, 2026
