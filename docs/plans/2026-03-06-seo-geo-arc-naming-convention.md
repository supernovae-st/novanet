# SEO/GEO Arc Naming Convention

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Standardize arc naming for SEO/GEO connections with LLM-optimized conventions.

**Architecture:** 4 arcs total (2 SEO + 2 GEO), suffix indicates target type, verb indicates intention.

**Tech Stack:** YAML schema definitions, Cypher migrations, Neo4j

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Suffix** | Target type (`_KEYWORD`, `_QUERY`) | LLM knows target node type from arc name |
| **Verbs** | Domain-specific (`TARGETS` vs `TRACKS`) | SEO=ranking intent, GEO=visibility monitoring |
| **Levels** | Different verbs (`CURATES`/`MONITORS` vs `TARGETS`/`TRACKS`) | Project=curation, Content=objective |
| **Sources** | Multiple per arc | 4 arcs instead of 12, simpler for LLM |

---

## Final Arc Schema

### 4 New Arcs

| Arc | Sources | Target | Family | Scope |
|-----|---------|--------|--------|-------|
| `CURATES_KEYWORD` | ProjectSEOScope | SEOKeyword | semantic | cross_realm |
| `MONITORS_QUERY` | ProjectGEOScope | GEOQuery | semantic | cross_realm |
| `TARGETS_KEYWORD` | EntityNative, Page, Block | SEOKeyword | mining | cross_realm |
| `TRACKS_QUERY` | EntityNative, Page, Block | GEOQuery | mining | cross_realm |

### Arcs to Delete

| Old Arc | Replaced By |
|---------|-------------|
| `TARGETS` (EntityNative → SEOKeyword) | `TARGETS_KEYWORD` |
| `MONITORS_GEO` (EntityNative → GEOQuery) | `TRACKS_QUERY` |
| `TARGETS_GEO` (Page/Block → GEOQuery) | `TRACKS_QUERY` |
| `CURATES` (ProjectSEOScope → SEOKeyword) | `CURATES_KEYWORD` |
| `MONITORS` (ProjectGEOScope → GEOQuery) | `MONITORS_QUERY` |
| `TARGETS_KEYWORD` (Page/Block → SEOKeyword) | Keep but add EntityNative as source |

---

## Implementation Tasks

### Task 1: Create CURATES_KEYWORD arc

**Files:**
- Create: `brain/models/arc-classes/semantic/curates-keyword.yaml`
- Delete: `brain/models/arc-classes/semantic/curates.yaml` (if exists)

**Step 1: Create arc definition**

```yaml
# brain/models/arc-classes/semantic/curates-keyword.yaml
arc:
  name: CURATES_KEYWORD
  family: semantic
  scope: cross_realm
  temperature_threshold: 0.5

  description: "Project SEO scope curates (selects) keywords for active optimization."
  llm_context: |
    USE: when a project scope selects keywords for SEO optimization.
    TRIGGERS: "curates keyword", "SEO scope", "keyword selection", "project SEO".
    NOT: for content targeting (use TARGETS_KEYWORD), for GEO (use MONITORS_QUERY).
    RELATES: ProjectSEOScope (source, org/foundation), SEOKeyword (target, shared/knowledge).

  source: ProjectSEOScope
  target: SEOKeyword
  cardinality: many_to_many

  properties:
    - name: priority
      type: string
      required: true
      enum: [high, medium, low]
      description: "Curation priority for this keyword within the project scope"

    - name: curator
      type: string
      required: true
      enum: [human, ai-suggested, auto-imported]
      description: "Who/what added this keyword to the scope"

    - name: curated_at
      type: datetime
      required: true
      description: "When this keyword was added to the scope"

    - name: strategy
      type: string
      required: false
      enum: [pillar, cluster, long-tail, branded]
      description: "SEO strategy classification"

    - name: verified
      type: boolean
      required: false
      default: false
      description: "True if human verified an AI-suggested curation"

  cypher_pattern: "(ProjectSEOScope)-[:CURATES_KEYWORD {priority, curator, curated_at}]->(SEOKeyword)"

  examples:
    high_priority:
      cypher: |
        MATCH (scope:ProjectSEOScope {key: 'seo-scope:qrcode-ai@fr-FR'})-[c:CURATES_KEYWORD]->(k:SEOKeyword)
        WHERE c.priority = 'high'
        RETURN k.value, c.strategy, c.curator
        ORDER BY k.volume DESC
      description: "Get high-priority curated keywords for French SEO scope"
```

**Step 2: Verify with schema validate**

```bash
cd /Users/thibaut/dev/supernovae/novanet && cargo run -- schema validate
```

---

### Task 2: Create MONITORS_QUERY arc

**Files:**
- Create: `brain/models/arc-classes/semantic/monitors-query.yaml`
- Delete: `brain/models/arc-classes/semantic/monitors.yaml`

**Step 1: Create arc definition**

```yaml
# brain/models/arc-classes/semantic/monitors-query.yaml
arc:
  name: MONITORS_QUERY
  family: semantic
  scope: cross_realm
  temperature_threshold: 0.5

  description: "Project GEO scope monitors queries for AI visibility tracking."
  llm_context: |
    USE: when a project scope monitors GEO queries for AI visibility.
    TRIGGERS: "monitors query", "GEO scope", "AI visibility", "project GEO".
    NOT: for content tracking (use TRACKS_QUERY), for SEO (use CURATES_KEYWORD).
    RELATES: ProjectGEOScope (source, org/foundation), GEOQuery (target, shared/knowledge).

  source: ProjectGEOScope
  target: GEOQuery
  cardinality: many_to_many

  properties:
    - name: priority
      type: string
      required: true
      enum: [high, medium, low]
      description: "Monitoring priority for this query within the project scope"

    - name: monitor_frequency
      type: string
      required: false
      enum: [realtime, hourly, daily, weekly]
      description: "How often to check this query across AI engines"
      default: daily

    - name: curator
      type: string
      required: true
      enum: [human, ai-suggested, auto-imported]
      description: "Who/what added this query to the monitoring scope"

    - name: monitored_at
      type: datetime
      required: true
      description: "When this query was added to the monitoring scope"

    - name: platforms
      type: string[]
      required: false
      description: "Override platforms to check for this specific query"

    - name: verified
      type: boolean
      required: false
      default: false
      description: "True if human verified an AI-suggested monitoring"

  cypher_pattern: "(ProjectGEOScope)-[:MONITORS_QUERY {priority, curator, monitored_at}]->(GEOQuery)"

  examples:
    high_priority:
      cypher: |
        MATCH (scope:ProjectGEOScope {key: 'geo-scope:qrcode-ai@fr-FR'})-[m:MONITORS_QUERY]->(q:GEOQuery)
        WHERE m.priority = 'high'
        OPTIONAL MATCH (q)-[:HAS_GEO_ANSWER]->(a:GEOAnswer)
        RETURN q.value, m.monitor_frequency, count(a) AS answer_count
        ORDER BY answer_count DESC
      description: "Get high-priority monitored queries for French GEO scope"
```

---

### Task 3: Create TARGETS_KEYWORD arc

**Files:**
- Modify: `brain/models/arc-classes/mining/targets-keyword.yaml` (add EntityNative as source)

**Step 1: Update arc definition with multiple sources**

```yaml
# brain/models/arc-classes/mining/targets-keyword.yaml
arc:
  name: TARGETS_KEYWORD
  family: mining
  scope: cross_realm
  temperature_threshold: 0.6

  description: "Content (entity, page, block) targets a keyword for SEO ranking."
  llm_context: |
    USE: when content wants to rank for a specific SEO keyword.
    TRIGGERS: "targets keyword", "SEO target", "rank for", "optimize for", "keyword targeting".
    NOT: for project-level curation (use CURATES_KEYWORD), for GEO (use TRACKS_QUERY).
    RELATES: EntityNative/Page/Block (sources, org), SEOKeyword (target, shared/knowledge).
    PATTERN: Content declares intent to rank; actual ranking tracked via SEOKeywordMetrics.

  source: [EntityNative, Page, Block]
  target: SEOKeyword
  cardinality: many_to_many

  properties:
    - name: priority
      type: string
      required: true
      enum: [primary, secondary, tertiary]
      description: "How important this keyword is for the content"

    - name: target_position
      type: int
      required: false
      description: "Target SERP position (1-10)"

    - name: current_position
      type: int
      required: false
      description: "Current SERP position (updated by mining)"

    - name: targeted_at
      type: datetime
      required: true
      description: "When this targeting was established"

    - name: curator
      type: string
      required: true
      enum: [human, ai-suggested, auto-derived]
      description: "Who/what established this targeting"

  cypher_pattern: "(EntityNative|Page|Block)-[:TARGETS_KEYWORD {priority, targeted_at}]->(SEOKeyword)"

  examples:
    entity_targeting:
      cypher: |
        MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})-[t:TARGETS_KEYWORD]->(k:SEOKeyword)
        WHERE t.priority = 'primary'
        RETURN k.value, k.volume, t.target_position
        ORDER BY k.volume DESC
      description: "Get primary keywords targeted by French QR Code entity"

    page_targeting:
      cypher: |
        MATCH (p:Page)-[t:TARGETS_KEYWORD]->(k:SEOKeyword)
        WHERE p.key STARTS WITH 'page:pricing'
        RETURN p.key, k.value, t.priority
      description: "Get keywords targeted by pricing pages"
```

---

### Task 4: Create TRACKS_QUERY arc

**Files:**
- Create: `brain/models/arc-classes/mining/tracks-query.yaml`
- Delete: `brain/models/arc-classes/semantic/monitors-geo.yaml`
- Delete: `brain/models/arc-classes/mining/targets-geo.yaml`

**Step 1: Create arc definition**

```yaml
# brain/models/arc-classes/mining/tracks-query.yaml
arc:
  name: TRACKS_QUERY
  family: mining
  scope: cross_realm
  temperature_threshold: 0.6

  description: "Content (entity, page, block) tracks a GEO query for AI visibility monitoring."
  llm_context: |
    USE: when content monitors its visibility in AI responses for a query.
    TRIGGERS: "tracks query", "GEO visibility", "AI mentions", "visibility tracking".
    NOT: for project-level monitoring (use MONITORS_QUERY), for SEO (use TARGETS_KEYWORD).
    RELATES: EntityNative/Page/Block (sources, org), GEOQuery (target, shared/knowledge).
    PATTERN: Content declares visibility interest; actual visibility tracked via GEOAnswer.

  source: [EntityNative, Page, Block]
  target: GEOQuery
  cardinality: many_to_many

  properties:
    - name: priority
      type: string
      required: true
      enum: [primary, secondary, tertiary]
      description: "How important this query is for the content"

    - name: alert_threshold
      type: float
      required: false
      description: "Visibility score below which to trigger alert (0.0-1.0)"

    - name: target_engines
      type: string[]
      required: false
      description: "Target AI engines (gemini, gpt, perplexity, claude)"

    - name: tracked_at
      type: datetime
      required: true
      description: "When this tracking was established"

    - name: curator
      type: string
      required: true
      enum: [human, ai-suggested, auto-derived]
      description: "Who/what established this tracking"

  cypher_pattern: "(EntityNative|Page|Block)-[:TRACKS_QUERY {priority, tracked_at}]->(GEOQuery)"

  examples:
    entity_tracking:
      cypher: |
        MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})-[t:TRACKS_QUERY]->(q:GEOQuery)
        WHERE t.priority = 'primary'
        OPTIONAL MATCH (q)-[:HAS_GEO_ANSWER]->(a:GEOAnswer)
        WITH q, t, a ORDER BY a.observed_at DESC
        WITH q, t, collect(a)[0] AS latest
        RETURN q.value, t.alert_threshold, latest.relevance_score AS current_visibility
      description: "Get primary queries tracked by French QR Code entity with latest visibility"

    page_tracking:
      cypher: |
        MATCH (p:Page)-[t:TRACKS_QUERY]->(q:GEOQuery)
        WHERE p.key STARTS WITH 'page:pricing'
        RETURN p.key, q.value, t.priority, t.target_engines
      description: "Get queries tracked by pricing pages"
```

---

### Task 5: Update EntityNative relations

**Files:**
- Modify: `brain/models/node-classes/org/semantic/entity-native.yaml`

**Step 1: Update relations section**

Replace:
```yaml
    - type: TARGETS
      to: SEOKeyword
```

With:
```yaml
    - type: TARGETS_KEYWORD
      to: SEOKeyword
      cardinality: "N:N"
      description: "SEO keywords this entity targets for ranking"

    - type: TRACKS_QUERY
      to: GEOQuery
      cardinality: "N:N"
      description: "GEO queries this entity tracks for AI visibility"
```

---

### Task 6: Delete old arc files

**Files to delete:**
- `brain/models/arc-classes/semantic/curates.yaml` (if exists)
- `brain/models/arc-classes/semantic/monitors.yaml`
- `brain/models/arc-classes/semantic/monitors-geo.yaml`
- `brain/models/arc-classes/mining/targets-geo.yaml`

---

### Task 7: Create migration for Neo4j

**Files:**
- Create: `brain/seed/migrations/046-rename-seo-geo-arcs.cypher`

**Step 1: Write migration**

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// 046-rename-seo-geo-arcs.cypher
// Rename SEO/GEO arcs to new naming convention
// v0.17.0 - 2026-03-06
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// PART 1: Rename CURATES → CURATES_KEYWORD (ProjectSEOScope → SEOKeyword)
// ─────────────────────────────────────────────────────────────────────────────────

MATCH (scope:ProjectSEOScope)-[old:CURATES]->(k:SEOKeyword)
CREATE (scope)-[new:CURATES_KEYWORD]->(k)
SET new = properties(old)
DELETE old;

// ─────────────────────────────────────────────────────────────────────────────────
// PART 2: Rename MONITORS → MONITORS_QUERY (ProjectGEOScope → GEOQuery)
// ─────────────────────────────────────────────────────────────────────────────────

MATCH (scope:ProjectGEOScope)-[old:MONITORS]->(q:GEOQuery)
CREATE (scope)-[new:MONITORS_QUERY]->(q)
SET new = properties(old)
DELETE old;

// ─────────────────────────────────────────────────────────────────────────────────
// PART 3: Rename TARGETS → TARGETS_KEYWORD (EntityNative → SEOKeyword)
// ─────────────────────────────────────────────────────────────────────────────────

MATCH (en:EntityNative)-[old:TARGETS]->(k:SEOKeyword)
CREATE (en)-[new:TARGETS_KEYWORD]->(k)
SET new = properties(old)
DELETE old;

// ─────────────────────────────────────────────────────────────────────────────────
// PART 4: Rename MONITORS_GEO → TRACKS_QUERY (EntityNative → GEOQuery)
// ─────────────────────────────────────────────────────────────────────────────────

MATCH (en:EntityNative)-[old:MONITORS_GEO]->(q:GEOQuery)
CREATE (en)-[new:TRACKS_QUERY]->(q)
SET new = properties(old)
DELETE old;

// ─────────────────────────────────────────────────────────────────────────────────
// PART 5: Rename TARGETS_GEO → TRACKS_QUERY (Page/Block → GEOQuery)
// ─────────────────────────────────────────────────────────────────────────────────

MATCH (p:Page)-[old:TARGETS_GEO]->(q:GEOQuery)
CREATE (p)-[new:TRACKS_QUERY]->(q)
SET new = properties(old)
DELETE old;

MATCH (b:Block)-[old:TARGETS_GEO]->(q:GEOQuery)
CREATE (b)-[new:TRACKS_QUERY]->(q)
SET new = properties(old)
DELETE old;

// ═══════════════════════════════════════════════════════════════════════════════
// VERIFICATION QUERIES
// ═══════════════════════════════════════════════════════════════════════════════

// Run these after migration to verify:
// MATCH ()-[r:CURATES|MONITORS|TARGETS|MONITORS_GEO|TARGETS_GEO]->() RETURN type(r), count(r);
// Should return 0 rows (all old arcs deleted)

// MATCH ()-[r:CURATES_KEYWORD|MONITORS_QUERY|TARGETS_KEYWORD|TRACKS_QUERY]->() RETURN type(r), count(r);
// Should show counts for new arcs
```

---

### Task 8: Regenerate schema and validate

**Step 1: Generate schema artifacts**

```bash
cd /Users/thibaut/dev/supernovae/novanet && cargo run -- schema generate
```

**Step 2: Validate schema**

```bash
cd /Users/thibaut/dev/supernovae/novanet && cargo run -- schema validate
```

**Step 3: Run audit**

```bash
# Use novanet_audit to verify CSR remains 100%
```

---

### Task 9: Commit and push

**Step 1: Commit brain changes**

```bash
cd /Users/thibaut/dev/supernovae/brain
git add .
git commit -m "refactor(schema): standardize SEO/GEO arc naming convention

BREAKING CHANGE: Arc renames for LLM-optimized naming

Renames:
- CURATES → CURATES_KEYWORD (ProjectSEOScope → SEOKeyword)
- MONITORS → MONITORS_QUERY (ProjectGEOScope → GEOQuery)
- TARGETS → TARGETS_KEYWORD (EntityNative → SEOKeyword)
- MONITORS_GEO → TRACKS_QUERY (EntityNative → GEOQuery)
- TARGETS_GEO → TRACKS_QUERY (Page/Block → GEOQuery)

Design decisions:
- Suffix = target type (_KEYWORD, _QUERY)
- Verb = intention (TARGETS=ranking, TRACKS=visibility)
- Level = semantics (CURATES/MONITORS=curation, TARGETS/TRACKS=objective)
- Multiple sources per arc (4 arcs instead of 12)

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
git push
```

**Step 2: Commit novanet changes**

```bash
cd /Users/thibaut/dev/supernovae/novanet
git add .
git commit -m "docs(plans): add SEO/GEO arc naming convention plan

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
git push
```

---

## Summary

| Before | After | Change |
|--------|-------|--------|
| 5+ inconsistent arcs | 4 standardized arcs | Simplified |
| Mixed suffixes | Consistent `_KEYWORD`/`_QUERY` | LLM-friendly |
| Confusing verbs | Semantic verbs by level | Clear intent |
| 5+ llm_context | 4 llm_context | Less tokens |
