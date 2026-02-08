# Arc Quality Fixes Execution Plan

**Date**: 2026-02-08
**Version**: v10.7
**Total Fixes**: ~95 arcs + 7 llm_context updates
**Estimated Time**: 7 batches

## Executive Summary

10-agent audit revealed semantic arc quality issues across 281 Entity nodes.
This plan fixes arc classifications, directions, and adds missing connections.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  EXECUTION OVERVIEW                                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Batch 1: Schema (3 new arc types)           ████░░░░░░░░░░░░░░░░  10%         │
│  Batch 2: SIMILAR_TO → ALTERNATIVE_TO        ████████░░░░░░░░░░░░  25%         │
│  Batch 3: ENABLES direction fixes            ██████████░░░░░░░░░░  35%         │
│  Batch 4: APPLIES_TO migrations              ██████████████░░░░░░  55%         │
│  Batch 5: TYPE_OF → VARIANT_OF               ████████████████░░░░  65%         │
│  Batch 6: Orphan industry connections        ██████████████████░░  85%         │
│  Batch 7: llm_context fixes                  ████████████████████  100%        │
│                                                                                 │
│  Total: ~95 arc changes + 7 property updates                                   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Batch 1: Create Missing Arc Types

**Priority**: CRITICAL (blocks Batch 4)
**Files**: `packages/core/models/arc-kinds/semantic/`

### New Arc Types Required

#### 1.1 ACTS_ON Arc

```yaml
# packages/core/models/arc-kinds/semantic/acts-on.yaml
arc:
  name: ACTS_ON
  family: semantic
  scope: intra_realm
  cardinality: many_to_many
  description: "Action operates directly on an entity (ACTION → Entity)"

  source_kinds: [Entity]
  target_kinds: [Entity]

  constraints:
    source_type: ACTION
    target_type: [THING, CONCEPT, CONTENT_TYPE]

  properties:
    strength:
      type: float
      required: false
      default: 0.8
    operation_type:
      type: string
      required: false
      description: "Type of operation (create, read, update, delete, scan)"
```

#### 1.2 ENHANCES Arc

```yaml
# packages/core/models/arc-kinds/semantic/enhances.yaml
arc:
  name: ENHANCES
  family: semantic
  scope: intra_realm
  cardinality: many_to_many
  description: "Feature enhances/improves an entity (FEATURE → Entity)"

  source_kinds: [Entity]
  target_kinds: [Entity]

  constraints:
    source_type: FEATURE
    target_type: [THING, CONCEPT, CONTENT_TYPE]

  properties:
    strength:
      type: float
      required: false
      default: 0.75
    enhancement_type:
      type: string
      required: false
      description: "How it enhances (adds, improves, extends)"
```

#### 1.3 READS Arc (optional)

```yaml
# packages/core/models/arc-kinds/semantic/reads.yaml
arc:
  name: READS
  family: semantic
  scope: intra_realm
  cardinality: many_to_many
  description: "Tool reads/decodes an entity (scanner READS barcode)"

  source_kinds: [Entity]
  target_kinds: [Entity]

  constraints:
    source_type: TOOL
    target_type: [THING, CONTENT_TYPE]
```

### Execution

```bash
# 1. Create YAML files
# 2. Regenerate schema
cargo run -- schema generate
cargo run -- schema validate
# 3. Seed new arc kinds
cargo run -- db seed
```

---

## Batch 2: SIMILAR_TO → ALTERNATIVE_TO

**Priority**: HIGH
**Count**: 12 arcs
**Reason**: Platform-specific alternatives incorrectly marked as similar

### Arcs to Migrate

| Source | Target | Reason |
|--------|--------|--------|
| vcard-qr | mecard-qr | Contact format alternatives |
| business-card-qr | email-signature-qr | Professional contact alternatives |
| event-rsvp | booking-appointment | Scheduling alternatives |
| google-maps-qr | apple-maps-qr | Platform alternatives |
| google-maps-qr | waze-qr | Navigation alternatives |
| app-store-qr | play-store-qr | Platform alternatives |
| pix-qr | upi-qr | Country-specific payment alternatives |
| bitcoin-qr | ethereum-qr | Crypto payment alternatives |
| spotify-qr | apple-music-qr | Music platform alternatives |
| paypal-qr | venmo-qr | Payment platform alternatives |
| qr-code-whatsapp | qr-code-telegram | Messaging alternatives |
| qr-code-facebook | qr-code-twitter | Social platform alternatives |

### Cypher Migration

```cypher
// Batch 2: SIMILAR_TO → ALTERNATIVE_TO
// Run in single transaction

// vCard ↔ MeCard
MATCH (a:Entity {key: 'qr-code-vcard'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-mecard'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.9, context: 'contact_format'}]->(b);

// Business Card ↔ Email Signature
MATCH (a:Entity {key: 'qr-code-business-card'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-email-signature'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.85, context: 'professional_contact'}]->(b);

// Event RSVP ↔ Booking
MATCH (a:Entity {key: 'event-rsvp'})-[r:SIMILAR_TO]->(b:Entity {key: 'booking-appointment'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.8, context: 'scheduling'}]->(b);

// Google Maps ↔ Apple Maps
MATCH (a:Entity {key: 'qr-code-google-maps'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-apple-maps'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.95, context: 'navigation_platform'}]->(b);

// Google Maps ↔ Waze
MATCH (a:Entity {key: 'qr-code-google-maps'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-waze'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.9, context: 'navigation_app'}]->(b);

// App Store ↔ Play Store
MATCH (a:Entity {key: 'qr-code-app-store'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-play-store'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.95, context: 'app_store_platform'}]->(b);

// PIX ↔ UPI
MATCH (a:Entity {key: 'qr-code-pix'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-upi'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.85, context: 'country_payment'}]->(b);

// Bitcoin ↔ Ethereum
MATCH (a:Entity {key: 'qr-code-bitcoin'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-ethereum'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.9, context: 'cryptocurrency'}]->(b);

// Spotify ↔ Apple Music
MATCH (a:Entity {key: 'qr-code-spotify'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-apple-music'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.95, context: 'music_platform'}]->(b);

// PayPal ↔ Venmo
MATCH (a:Entity {key: 'qr-code-paypal'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-venmo'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.9, context: 'payment_platform'}]->(b);

// WhatsApp ↔ Telegram
MATCH (a:Entity {key: 'qr-code-whatsapp'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-telegram'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.85, context: 'messaging_app'}]->(b);

// Facebook ↔ Twitter
MATCH (a:Entity {key: 'qr-code-facebook'})-[r:SIMILAR_TO]->(b:Entity {key: 'qr-code-twitter'})
DELETE r
CREATE (a)-[:ALTERNATIVE_TO {strength: 0.8, context: 'social_platform'}]->(b);
```

### Verification

```cypher
// Verify migration
MATCH ()-[r:ALTERNATIVE_TO]->() RETURN count(r) as new_alternatives;
// Expected: 12+ (existing + new)
```

---

## Batch 3: ENABLES Direction Fixes

**Priority**: HIGH
**Count**: 8 arcs
**Reason**: Integration→Industry should be Industry→Integration

### Arcs to Reverse

| Current Source | Current Target | Fix |
|---------------|----------------|-----|
| hubspot-integration | marketing-agencies | REVERSE |
| salesforce-integration | enterprise | REVERSE |
| shopify-integration | retail | REVERSE |
| woocommerce-integration | small-business | REVERSE |
| zapier | api | Change to INTEGRATES_WITH |
| make | api | Change to INTEGRATES_WITH |
| n8n | api | Change to INTEGRATES_WITH |
| retail | product-packaging | Change to USES_MEDIUM |

### Cypher Migration

```cypher
// Batch 3: ENABLES direction fixes

// Reverse Integration → Industry to Industry → Integration
MATCH (i:Entity {key: 'hubspot-integration'})-[r:ENABLES]->(ind:Entity {key: 'marketing-agencies'})
DELETE r
CREATE (ind)-[:USES_INTEGRATION {strength: 0.8}]->(i);

MATCH (i:Entity {key: 'salesforce-integration'})-[r:ENABLES]->(ind:Entity {key: 'enterprise'})
DELETE r
CREATE (ind)-[:USES_INTEGRATION {strength: 0.8}]->(i);

MATCH (i:Entity {key: 'shopify-integration'})-[r:ENABLES]->(ind:Entity {key: 'retail'})
DELETE r
CREATE (ind)-[:USES_INTEGRATION {strength: 0.85}]->(i);

MATCH (i:Entity {key: 'woocommerce-integration'})-[r:ENABLES]->(ind:Entity {key: 'small-business'})
DELETE r
CREATE (ind)-[:USES_INTEGRATION {strength: 0.8}]->(i);

// Automation platforms: ENABLES → INTEGRATES_WITH
MATCH (a:Entity {key: 'zapier'})-[r:ENABLES]->(b:Entity {key: 'api'})
DELETE r
CREATE (a)-[:INTEGRATES_WITH {strength: 0.9}]->(b);

MATCH (a:Entity {key: 'make'})-[r:ENABLES]->(b:Entity {key: 'api'})
DELETE r
CREATE (a)-[:INTEGRATES_WITH {strength: 0.9}]->(b);

MATCH (a:Entity {key: 'n8n'})-[r:ENABLES]->(b:Entity {key: 'api'})
DELETE r
CREATE (a)-[:INTEGRATES_WITH {strength: 0.9}]->(b);

// Industry → Medium: ENABLES → USES_MEDIUM
MATCH (a:Entity {key: 'retail'})-[r:ENABLES]->(b:Entity {key: 'product-packaging'})
DELETE r
CREATE (a)-[:USES_MEDIUM {strength: 0.85}]->(b);
```

### Note

This batch requires 2 new arc types:
- `USES_INTEGRATION` (or use existing REQUIRES)
- `INTEGRATES_WITH`

---

## Batch 4: APPLIES_TO Migrations

**Priority**: HIGH
**Count**: ~28 arcs
**Reason**: APPLIES_TO used as catch-all, needs semantic precision

### Migration Rules

| Source Type | Target Type | Current | Migrate To |
|-------------|-------------|---------|------------|
| ACTION | THING/CONCEPT | APPLIES_TO | ACTS_ON |
| FEATURE | THING | APPLIES_TO | ENABLES |
| TOOL | THING | APPLIES_TO | PRODUCES or READS |

### Cypher Migration

```cypher
// Batch 4: APPLIES_TO migrations

// ACTION → THING: migrate to ACTS_ON
MATCH (a:Entity)-[r:APPLIES_TO]->(t:Entity)
WHERE a.type = 'ACTION' AND t.type IN ['THING', 'CONCEPT']
WITH a, r, t
DELETE r
CREATE (a)-[:ACTS_ON {strength: 0.8}]->(t);

// FEATURE → THING: migrate to ENABLES
MATCH (f:Entity)-[r:APPLIES_TO]->(t:Entity)
WHERE f.type = 'FEATURE' AND t.type IN ['THING', 'CONCEPT']
WITH f, r, t
DELETE r
CREATE (f)-[:ENABLES {strength: 0.75}]->(t);

// TOOL (generator) → THING: migrate to PRODUCES
MATCH (tool:Entity)-[r:APPLIES_TO]->(t:Entity)
WHERE tool.type = 'TOOL' AND tool.key CONTAINS 'generator' AND t.type = 'THING'
WITH tool, r, t
DELETE r
CREATE (tool)-[:PRODUCES {strength: 0.9}]->(t);

// TOOL (scanner) → THING: migrate to READS
MATCH (tool:Entity)-[r:APPLIES_TO]->(t:Entity)
WHERE tool.type = 'TOOL' AND tool.key CONTAINS 'scanner' AND t.type = 'THING'
WITH tool, r, t
DELETE r
CREATE (tool)-[:READS {strength: 0.9}]->(t);
```

### Keep APPLIES_TO For

- CONCEPT → THING (correct usage)
- COMPARISON → THING/BRAND (correct usage)
- GUIDE → ACTION (correct usage)
- USE_CASE → INDUSTRY (correct usage)

---

## Batch 5: TYPE_OF → VARIANT_OF

**Priority**: MEDIUM
**Count**: 6 arcs
**Reason**: Specializations incorrectly marked as taxonomy

### Arcs to Migrate

| Source | Target | Reason |
|--------|--------|--------|
| batch-qr-generator | qr-code-generator | Specialization |
| vcard-generator | qr-code-generator | Content-specific variant |
| wifi-qr-generator | qr-code-generator | Content-specific variant |
| link-in-bio-builder | landing-page-builder | Social media specialization |
| menu-builder | landing-page-builder | Restaurant specialization |
| smart-link | short-link | Enhanced version |

### Cypher Migration

```cypher
// Batch 5: TYPE_OF → VARIANT_OF

MATCH (a:Entity {key: 'batch-qr-generator'})-[r:TYPE_OF]->(b:Entity {key: 'qr-code-generator'})
DELETE r
CREATE (a)-[:VARIANT_OF {strength: 0.85, variant_type: 'specialization'}]->(b);

MATCH (a:Entity {key: 'vcard-generator'})-[r:TYPE_OF]->(b:Entity {key: 'qr-code-generator'})
DELETE r
CREATE (a)-[:VARIANT_OF {strength: 0.85, variant_type: 'content_specific'}]->(b);

MATCH (a:Entity {key: 'wifi-qr-generator'})-[r:TYPE_OF]->(b:Entity {key: 'qr-code-generator'})
DELETE r
CREATE (a)-[:VARIANT_OF {strength: 0.85, variant_type: 'content_specific'}]->(b);

MATCH (a:Entity {key: 'link-in-bio-builder'})-[r:TYPE_OF]->(b:Entity {key: 'landing-page-builder'})
DELETE r
CREATE (a)-[:VARIANT_OF {strength: 0.85, variant_type: 'platform_specific'}]->(b);

MATCH (a:Entity {key: 'menu-builder'})-[r:TYPE_OF]->(b:Entity {key: 'landing-page-builder'})
DELETE r
CREATE (a)-[:VARIANT_OF {strength: 0.85, variant_type: 'industry_specific'}]->(b);

MATCH (a:Entity {key: 'smart-link'})-[r:TYPE_OF]->(b:Entity {key: 'short-link'})
DELETE r
CREATE (a)-[:VARIANT_OF {strength: 0.9, variant_type: 'enhanced_version'}]->(b);
```

---

## Batch 6: Orphan Industry Connections

**Priority**: HIGH
**Count**: ~35 new arcs
**Reason**: 7 industries with zero connections

### Industries to Connect

1. **beauty** (salons, spas)
2. **construction** (building, infrastructure)
3. **consulting** (business consulting)
4. **fitness** (gyms, wellness)
5. **government** (public sector)
6. **nonprofits** (charities, NGOs)
7. **transportation** (airlines, logistics)

### Cypher - Connect Industries

```cypher
// Batch 6: Connect orphan industries

// BEAUTY
MATCH (ind:Entity {key: 'beauty'})
MATCH (vcard:Entity {key: 'qr-code-vcard'})
MATCH (review:Entity {key: 'qr-code-review'})
MATCH (coupon:Entity {key: 'qr-code-coupon'})
MATCH (retail:Entity {key: 'retail'})
CREATE (ind)-[:ENABLES]->(vcard)
CREATE (ind)-[:ENABLES]->(review)
CREATE (ind)-[:ENABLES]->(coupon)
CREATE (ind)-[:SIMILAR_TO]->(retail);

// FITNESS
MATCH (ind:Entity {key: 'fitness'})
MATCH (attendance:Entity {key: 'qr-code-attendance'})
MATCH (coupon:Entity {key: 'qr-code-coupon'})
MATCH (healthcare:Entity {key: 'healthcare'})
CREATE (ind)-[:ENABLES]->(attendance)
CREATE (ind)-[:ENABLES]->(coupon)
CREATE (ind)-[:SIMILAR_TO]->(healthcare);

// CONSTRUCTION
MATCH (ind:Entity {key: 'construction'})
MATCH (vcard:Entity {key: 'qr-code-vcard'})
MATCH (pdf:Entity {key: 'qr-code-pdf'})
MATCH (manufacturing:Entity {key: 'manufacturing'})
CREATE (ind)-[:ENABLES]->(vcard)
CREATE (ind)-[:ENABLES]->(pdf)
CREATE (ind)-[:SIMILAR_TO]->(manufacturing);

// CONSULTING
MATCH (ind:Entity {key: 'consulting'})
MATCH (vcard:Entity {key: 'qr-code-vcard'})
MATCH (pdf:Entity {key: 'qr-code-pdf'})
MATCH (calendar:Entity {key: 'qr-code-calendar'})
MATCH (enterprise:Entity {key: 'enterprise'})
CREATE (ind)-[:ENABLES]->(vcard)
CREATE (ind)-[:ENABLES]->(pdf)
CREATE (ind)-[:ENABLES]->(calendar)
CREATE (ind)-[:SIMILAR_TO]->(enterprise);

// GOVERNMENT
MATCH (ind:Entity {key: 'government'})
MATCH (pdf:Entity {key: 'qr-code-pdf'})
MATCH (url:Entity {key: 'qr-code-url'})
MATCH (education:Entity {key: 'education'})
CREATE (ind)-[:ENABLES]->(pdf)
CREATE (ind)-[:ENABLES]->(url)
CREATE (ind)-[:SIMILAR_TO]->(education);

// NONPROFITS
MATCH (ind:Entity {key: 'nonprofits'})
MATCH (payment:Entity {key: 'qr-code-payment'})
MATCH (url:Entity {key: 'qr-code-url'})
MATCH (event:Entity {key: 'qr-code-event'})
CREATE (ind)-[:ENABLES]->(payment)
CREATE (ind)-[:ENABLES]->(url)
CREATE (ind)-[:ENABLES]->(event);

// TRANSPORTATION
MATCH (ind:Entity {key: 'transportation'})
MATCH (ticket:Entity {key: 'qr-code-ticket'})
MATCH (url:Entity {key: 'qr-code-url'})
MATCH (payment:Entity {key: 'qr-code-payment'})
MATCH (logistics:Entity {key: 'logistics'})
CREATE (ind)-[:ENABLES]->(ticket)
CREATE (ind)-[:ENABLES]->(url)
CREATE (ind)-[:ENABLES]->(payment)
CREATE (ind)-[:SIMILAR_TO]->(logistics);
```

---

## Batch 7: llm_context Fixes

**Priority**: MEDIUM
**Count**: 7 entities
**Reason**: Generic or poorly formatted context

### Entities to Update

| Key | Issue | Fix |
|-----|-------|-----|
| qr-code-messaging | Awkward triggers | Improve specificity |
| qr-code-content | Too abstract | Add concrete use cases |
| short-link | Confusing NOT section | Clarify |
| smart-link | Missing geo-targeting triggers | Add |
| announcement | Too generic | Add specific triggers |
| webhooks | "trigger" too generic | Be specific |
| event-rsvp | "invitation" too broad | Narrow scope |

### Cypher Updates

```cypher
// Batch 7: llm_context improvements

MATCH (e:Entity {key: 'qr-code-messaging'})
SET e.llm_context = "USE: when grouping or categorizing messaging-related QR codes. TRIGGERS: messaging qr codes, chat app qr, instant messaging category, message qr types. NOT: specific apps (use WhatsApp QR, Telegram QR), email (use Email QR).";

MATCH (e:Entity {key: 'qr-code-content'})
SET e.llm_context = "USE: when discussing general QR code data types, content encoding concepts, or categorizing QR codes by function. TRIGGERS: qr content types, content category, qr data encoding, payload types, what qr codes can store. NOT: specific types (use URL QR, WiFi QR), styling (use QR Code Style).";

MATCH (e:Entity {key: 'short-link'})
SET e.llm_context = "USE: when discussing URL shortening technology, basic link tracking, or creating compact URLs. TRIGGERS: short link, shortened url, url shortener, link shortening, compact link. NOT: intelligent routing (use Smart Link), QR code generation (use QR Code), custom branded domains (use Custom Domain).";

MATCH (e:Entity {key: 'smart-link'})
SET e.llm_context = "USE: when discussing intelligent URL routing with device detection, geo-targeting, or conditional redirects. TRIGGERS: smart link, intelligent url, routing link, conditional redirect, targeted link, device targeting, geo redirect. NOT: basic URL shortening (use Short Link), static QR codes (use QR Code).";

MATCH (e:Entity {key: 'announcement'})
SET e.llm_context = "USE: when discussing one-time or time-sensitive announcements, news updates, or public notifications. TRIGGERS: announcement page, news alert, public notice, update notification, company announcement. NOT: event invitations (use Event RSVP), permanent content (use Link in Bio).";

MATCH (e:Entity {key: 'webhooks'})
SET e.llm_context = "USE: when discussing webhooks, event-driven notifications, HTTP callbacks, or automated scan event handling. TRIGGERS: webhooks, webhook integration, event notification, http callback, scan webhook. NOT: REST API (use API Access), viewing analytics (use Analytics Dashboard).";

MATCH (e:Entity {key: 'event-rsvp'})
SET e.llm_context = "USE: when discussing event registration, RSVP collection, guest list management, or event confirmation pages. TRIGGERS: event rsvp, event registration, rsvp page, guest list, rsvp form, event signup. NOT: general forms (use Forms), appointment booking (use Booking/Appointment).";
```

---

## Execution Checklist

```
[ ] Batch 1: Create arc type YAMLs (ACTS_ON, ENHANCES, READS)
[ ] Batch 1: Run schema generate + validate
[ ] Batch 1: Seed new arc kinds to Neo4j
[ ] Batch 2: Run SIMILAR_TO → ALTERNATIVE_TO migration
[ ] Batch 2: Verify 12 new ALTERNATIVE_TO arcs
[ ] Batch 3: Run ENABLES direction fixes
[ ] Batch 3: Verify 8 arcs corrected
[ ] Batch 4: Run APPLIES_TO migrations
[ ] Batch 4: Verify ~28 arcs migrated
[ ] Batch 5: Run TYPE_OF → VARIANT_OF migration
[ ] Batch 5: Verify 6 arcs migrated
[ ] Batch 6: Run orphan industry connections
[ ] Batch 6: Verify 35 new arcs created
[ ] Batch 7: Run llm_context updates
[ ] Batch 7: Verify 7 entities updated
[ ] Final: Run full entity audit to verify improvements
[ ] Final: Commit all changes with detailed message
```

---

## Verification Queries

```cypher
// Post-execution verification

// Count by arc type
MATCH (e1:Entity)-[r]->(e2:Entity)
RETURN type(r) as arc_type, count(*) as count
ORDER BY count DESC;

// Verify no orphan industries
MATCH (e:Entity {type: 'INDUSTRY'})
WHERE NOT (e)-[:ENABLES|SIMILAR_TO]-()
RETURN e.key as orphan;

// Verify ALTERNATIVE_TO count
MATCH ()-[r:ALTERNATIVE_TO]->()
RETURN count(r) as alternatives;
// Expected: 12+

// Verify new arc types
MATCH ()-[r:ACTS_ON|ENHANCES|READS]->()
RETURN type(r) as new_arc, count(*) as count;
```

---

## Rollback Plan

If issues occur, restore from Neo4j backup or re-run seed:

```bash
# Full reset if needed
cargo run -- db reset
```

Individual arc rollback queries are inverses of migration queries.

---

## Dependencies

```
Batch 1 (schema) ─────┬─────> Batch 4 (APPLIES_TO needs ACTS_ON)
                      │
                      └─────> Batch 3 (ENABLES needs new arc types)

Batch 2 (SIMILAR_TO) ───────> Independent
Batch 5 (TYPE_OF) ──────────> Independent
Batch 6 (industries) ───────> Independent
Batch 7 (llm_context) ──────> Independent
```

**Critical path**: Batch 1 → Batch 4

---

## Estimated Impact

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Total arcs | 457 | ~530 | +73 |
| ALTERNATIVE_TO arcs | 2 | 14 | +12 |
| Orphan entities | 11 | 0 | -11 |
| Semantic precision | ~70% | ~95% | +25% |
