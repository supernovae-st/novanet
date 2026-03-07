# Standard Properties Validation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Achieve 100% CSR (Constraint Satisfaction Rate) for NovaNet knowledge graph standard properties

**Architecture:** Fix data quality issues identified by 6-sniper audit, document required_properties, implement novanet_check validation

**Tech Stack:** Neo4j, Cypher migrations, novanet_check MCP tool, YAML schema

---

## Executive Summary

6-agent sniper audit revealed:
- **50% of Expressions** missing `display_name`
- **100% of SEOKeywords** orphaned (no TARGETS_KEYWORD arcs)
- **1 EntityNative** (fr-FR) with corrupted denomination_forms
- **4 Blocks** missing HAS_NATIVE
- **"romanized"** strategy undocumented in ADR-032

**Current CSR:** ~85% (critical issues)
**Target CSR:** 100%

---

## Audit Findings Matrix

| Issue | Severity | Nodes Affected | Fix |
|-------|----------|----------------|-----|
| Expression missing display_name | 🟡 Medium | 17,048 | Populate from `text` field |
| Term missing display_name | 🟡 Medium | 70 | Populate from `text` field |
| entity:qr-code@fr-FR corrupted | 🔴 Critical | 1 | Fix denomination_forms |
| SEOKeyword orphans | 🔴 Critical | 52 | Create TARGETS_KEYWORD arcs |
| Blocks missing HAS_NATIVE | 🔴 Critical | 4 | Generate BlockNatives |
| romanized undocumented | 🟡 Medium | 3 locales | Update ADR-032 |
| SLUGIFIES arc unused | 🟢 Low | 0 | Wire or mark future |

---

## Task 1: Fix Critical Data Quality Issues

### Task 1.1: Fix entity:qr-code@fr-FR denomination_forms

**Files:**
- Create: `brain/seed/migrations/041-fix-qr-code-fr-denomination.cypher`

**Step 1: Inspect current state**

```cypher
MATCH (en:EntityNative {key: "entity:qr-code@fr-FR"})
RETURN en.denomination_forms
```

Expected: corrupted values (`: `, ` `, ``, ``)

**Step 2: Create migration**

```cypher
// Migration 041: Fix qr-code@fr-FR denomination_forms
// ADR-033 compliance: text, title, abbrev, url forms

MATCH (en:EntityNative {key: "entity:qr-code@fr-FR"})
SET en.denomination_forms = '[
  {"type": "text", "value": "code QR", "priority": 1},
  {"type": "title", "value": "Code QR", "priority": 1},
  {"type": "abbrev", "value": "QR", "priority": 1},
  {"type": "url", "value": "code-qr", "priority": 1},
  {"type": "plural", "value": "codes QR", "priority": 5}
]',
    en.updated_at = datetime()
RETURN en.key, en.denomination_forms;
```

**Step 3: Verify**

Run: `novanet_audit target=coverage scope.locale=fr-FR`
Expected: CSR = 1.0 for fr-FR EntityNatives

**Step 4: Commit**

```bash
git add brain/seed/migrations/041-fix-qr-code-fr-denomination.cypher
git commit -m "fix(seed): correct qr-code@fr-FR denomination_forms (ADR-033)"
```

---

### Task 1.2: Create TARGETS_KEYWORD arcs for SEOKeywords

**Files:**
- Create: `brain/seed/migrations/042-seo-keyword-targets.cypher`

**Step 1: Analyze SEOKeyword → Entity mapping**

```cypher
// Find which entity each keyword should target
MATCH (sk:SEOKeyword)
RETURN sk.key,
       split(sk.key, '@')[0] as keyword_part,
       split(sk.key, '@')[1] as locale
LIMIT 20
```

**Step 2: Create migration**

```cypher
// Migration 042: Create TARGETS_KEYWORD arcs
// Links SEOKeyword to EntityNative based on key pattern

// Pattern: seo:<keyword>@<locale> → entity:<keyword>@<locale>
MATCH (sk:SEOKeyword)
WHERE sk.key STARTS WITH "seo:"
WITH sk,
     replace(sk.key, "seo:", "entity:") as target_key
MATCH (en:EntityNative {key: target_key})
MERGE (sk)-[:TARGETS_KEYWORD]->(en)
RETURN count(*) as arcs_created;
```

**Step 3: Verify**

```cypher
MATCH (sk:SEOKeyword)-[r:TARGETS_KEYWORD]->(en:EntityNative)
RETURN count(r) as linked_keywords
```

Expected: >0 (was 0)

**Step 4: Commit**

```bash
git add brain/seed/migrations/042-seo-keyword-targets.cypher
git commit -m "feat(seed): create TARGETS_KEYWORD arcs for SEOKeywords"
```

---

### Task 1.3: Generate BlockNatives for orphan Blocks

**Files:**
- Create: `brain/seed/migrations/043-create-missing-blocknatives.cypher`

**Step 1: List orphan blocks**

```cypher
MATCH (b:Block)
WHERE NOT (b)-[:HAS_NATIVE]->(:BlockNative)
RETURN b.key
```

Expected: 4 blocks (hero, what-is, use-cases, cta)

**Step 2: Create migration**

```cypher
// Migration 043: Create BlockNatives for orphan blocks
// Generates en-US and fr-FR natives for 4 missing blocks

// Get orphan blocks
MATCH (b:Block)
WHERE NOT (b)-[:HAS_NATIVE]->(:BlockNative)
WITH b

// Create en-US native
MATCH (l:Locale {key: "en-US"})
CREATE (bn:BlockNative {
  key: b.key + "@en-US",
  display_name: b.display_name + " (en-US)",
  description: b.description,
  content: "Generated content placeholder",
  created_at: datetime(),
  updated_at: datetime()
})
MERGE (b)-[:HAS_NATIVE]->(bn)
MERGE (bn)-[:FOR_LOCALE]->(l);

// Also create fr-FR native
MATCH (b:Block)
WHERE b.key IN ["block:qr-code-hero", "block:qr-code-what-is", "block:qr-code-use-cases", "block:qr-code-cta"]
MATCH (l:Locale {key: "fr-FR"})
CREATE (bn:BlockNative {
  key: b.key + "@fr-FR",
  display_name: b.display_name + " (fr-FR)",
  description: b.description,
  content: "Contenu généré placeholder",
  created_at: datetime(),
  updated_at: datetime()
})
MERGE (b)-[:HAS_NATIVE]->(bn)
MERGE (bn)-[:FOR_LOCALE]->(l);
```

**Step 3: Verify**

```cypher
MATCH (b:Block)-[:HAS_NATIVE]->(bn:BlockNative)
RETURN b.key, count(bn) as natives
```

Expected: All blocks have ≥1 native

**Step 4: Commit**

```bash
git add brain/seed/migrations/043-create-missing-blocknatives.cypher
git commit -m "feat(seed): create BlockNatives for 4 orphan blocks"
```

---

## Task 2: Document required_properties

### Task 2.1: Update CLAUDE.md with required_properties table

**Files:**
- Modify: `CLAUDE.md` (add section after "Standard Properties")

**Step 1: Add required_properties section**

```markdown
## Required Properties by Node Class

| Node Class | key | display_name | description | created_at | updated_at | Additional |
|------------|-----|--------------|-------------|------------|------------|------------|
| **Entity** | ✅ | ✅ | ✅ | ✅ | ✅ | — |
| **EntityNative** | ✅ | ✅ | ✅ | ✅ | ✅ | denomination_forms, locale |
| **Page** | ✅ | ✅ | ✅ | ✅ | ✅ | — |
| **PageNative** | ✅ | ✅ | ✅ | ✅ | ✅ | locale, slug |
| **Block** | ✅ | ✅ | ✅ | ✅ | ✅ | block_type |
| **BlockNative** | ✅ | ✅ | ✅ | ✅ | ✅ | locale, content |
| **SEOKeyword** | ✅ | ✅ | ✅ | ✅ | ✅ | locale |
| **Term** | ✅ | ⚠️ text | ✅ | ✅ | ✅ | locale_key |
| **Expression** | ✅ | ⚠️ text | — | ✅ | ⚠️ | locale, text |

**Legend:**
- ✅ Required and present
- ⚠️ Uses alternative field (text instead of display_name)
```

**Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: add required_properties table to CLAUDE.md"
```

---

## Task 3: Implement novanet_check ADR-033 validation

### Task 3.1: Add denomination_forms validation to novanet_check

**Files:**
- Modify: `tools/novanet-mcp/src/tools/check.rs`

**Step 1: Read current implementation**

Check existing validation logic in check.rs

**Step 2: Add denomination_forms validation**

```rust
// In validate_node_properties() or similar

// ADR-033: EntityNative must have denomination_forms with 4 core forms
if class_name == "EntityNative" {
    if let Some(forms) = properties.get("denomination_forms") {
        // Parse JSON
        let forms: Vec<DenominationForm> = serde_json::from_value(forms.clone())?;

        // Check for 4 required forms
        let required = ["text", "title", "abbrev", "url"];
        for form_type in required {
            if !forms.iter().any(|f| f.form_type == form_type) {
                warnings.push(format!(
                    "ADR-033: Missing '{}' form in denomination_forms",
                    form_type
                ));
            }
        }

        // Check for empty values
        for form in &forms {
            if form.value.trim().is_empty() {
                errors.push(format!(
                    "ADR-033: '{}' form has empty value",
                    form.form_type
                ));
            }
        }
    } else {
        errors.push("ADR-033: EntityNative requires denomination_forms property".to_string());
    }
}
```

**Step 3: Test**

```bash
cd tools/novanet-mcp
cargo test test_check_denomination_forms
```

**Step 4: Commit**

```bash
git add tools/novanet-mcp/src/tools/check.rs
git commit -m "feat(mcp): add ADR-033 denomination_forms validation to novanet_check"
```

---

## Task 4: Update ADR-032 with romanized strategy

### Task 4.1: Document romanized strategy in ADR-032

**Files:**
- Modify: `dx/adr/novanet/032-slugification-rules.md`

**Step 1: Add romanized to strategies table**

```markdown
| Strategy | Description | Example | Locales |
|----------|-------------|---------|---------|
| latin_preserve | Keep diacritics | café → café | fr-FR, es-MX, pt-BR |
| latin_strip | Remove diacritics | café → cafe | en-US, af-ZA |
| latin_transform | Phonetic mapping | ß → ss, ü → ue | de-DE, de-AT, de-CH |
| native_script | Keep native script | 東京 → 東京 | ja-JP, ar-SA, hi-IN |
| **romanized** | Convert to Romanization | 生成 → shengcheng | zh-CN, zh-SG, zh-TH |
```

**Step 2: Add section explaining romanized**

```markdown
### Romanized Strategy (CJK)

For Simplified Chinese locales (zh-CN, zh-SG, zh-TH), URLs use Hanyu Pinyin romanization:

- Input: 生成二维码
- Output: shengcheng-erweima

This differs from `native_script` (which would preserve Chinese characters) because:
1. URL compatibility requires ASCII-safe characters
2. SEO benefits from romanized keywords in Western search engines
3. Consistency with existing Chinese web standards
```

**Step 3: Commit**

```bash
git add dx/adr/novanet/032-slugification-rules.md
git commit -m "docs(adr): add romanized strategy to ADR-032"
```

---

## Verification Checklist

After all tasks, run:

```bash
# 1. Run full audit
novanet_audit target=all

# 2. Check CSR
# Expected: ≥0.95 (was ~0.85)

# 3. Verify denomination_forms
novanet_query cypher="MATCH (en:EntityNative) WHERE en.denomination_forms IS NULL RETURN count(en)"
# Expected: 0

# 4. Verify TARGETS_KEYWORD
novanet_query cypher="MATCH (sk:SEOKeyword)-[:TARGETS_KEYWORD]->(en) RETURN count(*)"
# Expected: >0

# 5. Verify orphan blocks
novanet_query cypher="MATCH (b:Block) WHERE NOT (b)-[:HAS_NATIVE]->() RETURN count(b)"
# Expected: 0
```

---

## Summary

| Task | Description | Status |
|------|-------------|--------|
| 1.1 | Fix entity:qr-code@fr-FR denomination_forms | ⏳ Pending |
| 1.2 | Create TARGETS_KEYWORD arcs | ⏳ Pending |
| 1.3 | Generate BlockNatives for orphans | ⏳ Pending |
| 2.1 | Document required_properties | ⏳ Pending |
| 3.1 | Implement ADR-033 validation | ⏳ Pending |
| 4.1 | Update ADR-032 romanized | ⏳ Pending |

**Estimated effort:** 2-3 hours
**Target CSR:** 100%
