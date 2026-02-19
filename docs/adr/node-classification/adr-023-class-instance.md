---
id: "023"
title: "Class/Instance Terminology"
version: "v11.8"
status: "active"
domain: "node-classification"
---

# ADR-023: Class/Instance Terminology

**Status**: Approved (v11.8)

**Problem**: Two terminology issues caused confusion:

1. **"Kind" was non-standard**: Not graph theory or ontology terminology. LLMs have less training data on "Kind" vs "Class". French "Genre" was awkward.

2. **"Meta" was ambiguous**: Facebook collision (Meta company), Spanish "meta" means "goal", too abstract for humans, mixed usage across "Meta Node", "KindMeta", Neo4j `:Meta:` labels.

**Decision**:
- Rename schema-level terminology from "Kind" to "Class"
- Data-level stays "Instance"
- **ELIMINATE "Meta" prefix/suffix entirely** - use semantic names instead

**Changes**:

| Before | After | Context |
|--------|-------|---------|
| **Kind → Class** | | |
| NodeKind | NodeClass | Rust/TypeScript struct |
| ArcKind | ArcClass | Rust/TypeScript struct |
| KindInfo | ClassInfo | TUI struct |
| TreeItem::Kind | TreeItem::Class | Rust enum variant |
| [:FROM_KIND] | [:FROM_CLASS] | Neo4j relationship |
| [:TO_KIND] | [:TO_CLASS] | Neo4j relationship |
| [:HAS_KIND] | [:HAS_CLASS] | Neo4j relationship |
| "Node Kinds" | "Classes" | UI label |
| **Meta → Semantic Names** | | |
| KindMeta | Classification | TypeScript interface (realm/layer/trait axes) |
| KIND_META | CLASS_TAXONOMY | TypeScript constant |
| :Meta:Kind | :Schema:Class | Neo4j label (Meta→Schema) |
| :Meta:ArcKind | :Schema:ArcClass | Neo4j label |
| "Meta Node" | "Class" | Glossary term |
| "Data Node" | "Instance" | Glossary term |
| "Meta mode" | "Schema view" | Studio ViewPicker |
| "Data mode" | "Graph view" | Studio ViewPicker |

**Rationale**:

**Class/Instance:**
1. **LLM Semantic Clarity**: `rdfs:Class`, `owl:Class` are in LLM training data millions of times. "Class/Instance" is THE canonical OOP and ontology pairing.
2. **Ontology Standard**: RDF Schema and OWL use "Class" for schema-level definitions. NovaNet is a knowledge graph - aligning with semantic web standards improves interoperability.
3. **Universal Understanding**: Every programmer knows Class/Instance from OOP. Non-programmers understand "a class of things" from everyday English.
4. **Internationalization**: "Classe/Instance" (French), "Clase/Instancia" (Spanish), "Klasse/Instanz" (German), クラス/インスタンス (Japanese) - perfect cognates.

**Meta Elimination:**
5. **Semantic names > abstract labels**: `Classification` describes WHAT it contains (realm/layer/trait axes). `Schema` describes WHAT it is (the schema, not data). "Meta" described NOTHING.
6. **No collisions**: Avoids Facebook "Meta" confusion in searches and discussions.
7. **International clarity**: "Schema" and "Classification" are universal technical terms. "Meta" has different meanings (Spanish "meta" = goal, Greek μετά = after).
8. **Consistency**: Single terminology change instead of half-measures. No more `:Meta:` labels in Neo4j, no more `*Meta` suffixes in code.

**Impact**:

| Zone | Files | Changes | Effort |
|------|-------|---------|--------|
| Rust | 43 | 721 | 4-8h |
| TypeScript | 19 | 93+ | 2-4h |
| TUI/Nexus | 20+ | 1,299 | 3-5h |
| Documentation | 14 | ~50 | 1-2h |
| Studio | 8 | ~30 | 2-3h |
| Neo4j Migration | - | Schema | 1h |

**Migration**: Requires coordinated update across Rust, TypeScript, Neo4j, and documentation. Neo4j schema migration must happen first or synchronously with code changes.

**Reference**: Brainstorming session 2026-02-12, devil's advocate analysis comparing 15 terminology options.
