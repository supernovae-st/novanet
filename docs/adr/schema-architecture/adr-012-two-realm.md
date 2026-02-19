---
id: "012"
title: "2-Realm Architecture"
version: "v10.6"
status: "active"
domain: "schema-architecture"
updated: "v11.2, v0.12.5"
---

# ADR-012: 2-Realm Architecture

**Status**: Approved (v10.6, updated v11.2)

**Decision**: Consolidate 3 realms into 2 realms: SHARED + ORG.

```
v10.5 (3 realms):  global / organization / project
v10.6 (2 realms):  global / tenant
v11.2 (2 realms):  shared / org  (renamed for clarity)
```

**Architecture** (v0.12.5):
- **SHARED** (4 layers): config, locale, geography, knowledge - Universal, READ-ONLY (40 nodes)
- **ORG** (6 layers): config, foundation, structure, semantic, instruction, output - Business-specific (21 nodes)

> **v0.12.5 Changes**:
> - `global` -> `shared` (describes WHAT: shared resources)
> - `tenant` -> `org` (describes WHO: organization-specific, familiar terminology)
> - Brand Architecture: Brand, BrandDesign, BrandPrinciples, PromptStyle, Country (ADR-028)
> - Total: 61 nodes (40 shared + 21 org)

**Rationale**:
- Organization + Project distinction added unnecessary complexity
- Org is the natural isolation boundary for multi-tenant SaaS
- Single realm for all business content simplifies queries and permissions
- 10 total layers (4 shared + 6 org) provides sufficient granularity
- v11.5: SEO/GEO consolidated to shared/knowledge
- v11.2: `shared` describes purpose, `org` is familiar (GitHub/Slack orgs)

**Migration path**:
- `global` -> `shared` (rename)
- `tenant` -> `org` (rename)
- All node types from both organization and project now live under org
