# Agents Reference

NovaNet agents are specialized subagents for complex tasks.

## Overview

Agents are defined in `.claude/agents/` and provide focused expertise for specific domains.

## Available Agents

### neo4j-architect

**Purpose**: Design and optimize Neo4j graph schemas.

**Expertise**:
- Graph schema design
- Cypher query optimization
- Index strategy
- Constraint design
- Performance tuning

**When to use**:
- Adding new node/arc types
- Query performance issues
- Schema migration planning
- Complex traversal patterns

**Example tasks**:
- "Design a schema for user sessions"
- "Optimize this slow Cypher query"
- "Add indexes for faceted search"

### code-reviewer

**Purpose**: Review code for quality and NovaNet conventions.

**Focus areas**:
- Code quality
- Security vulnerabilities
- NovaNet conventions
- Performance issues
- Test coverage

**When to use**:
- After implementing features
- Before merging PRs
- After major refactoring

**Example tasks**:
- "Review this new generator"
- "Check this TUI module for issues"
- "Verify security of Neo4j queries"

## Agent Structure

Each agent has a markdown definition:

```
.claude/agents/neo4j-architect.md
```

Contents:
- Agent role description
- Expertise areas
- Guidelines and constraints
- Example interactions

## Using Agents

Agents are invoked via the Task tool with:

```
subagent_type: "neo4j-architect"
```

They receive:
- Full conversation context
- Relevant file access
- Specialized instructions

## Creating New Agents

1. Create `.md` file in `.claude/agents/`
2. Define:
   - Role and expertise
   - When to use
   - Guidelines
   - Example tasks
3. Test with relevant tasks

## Agent vs Skill

| Aspect | Skill | Agent |
|--------|-------|-------|
| Activation | Automatic | Manual |
| Scope | Context injection | Task execution |
| Complexity | Simple patterns | Complex reasoning |
| Duration | Instant | Multi-turn |

Use skills for quick context, agents for deep analysis.

## Best Practices

- Define clear expertise boundaries
- Include specific guidelines
- Provide example interactions
- Test with edge cases
