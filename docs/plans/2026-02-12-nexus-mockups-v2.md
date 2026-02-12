# Nexus Mockups v2 — Real Examples for Beginners

**Date**: 2026-02-12
**Purpose**: Ultra-concrete mockups with real YAML/Neo4j examples
**Target**: Someone who has NEVER seen NovaNet before

---

## The Core Problem NovaNet Solves

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                                     │
│  TRADITIONAL APPROACH: Translation                                                                  │
│  ═══════════════════════════════════                                                                │
│                                                                                                     │
│    English content        French content         Japanese content                                   │
│    ─────────────────      ─────────────────      ─────────────────                                  │
│    "Create a QR Code"  →  "Créer un QR Code"  →  "QRコードを作成"                                    │
│                        ↑                      ↑                                                     │
│                    translate              translate                                                 │
│                                                                                                     │
│    PROBLEMS:                                                                                        │
│    • "Create" translated literally loses the nuance                                                 │
│    • Japanese would say "QRコードを作成する" (with する)                                             │
│    • 200 locales = 200× translation cost                                                            │
│    • Sync nightmare when English changes                                                            │
│                                                                                                     │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  NOVANET APPROACH: Native Generation                                                                │
│  ═══════════════════════════════════════                                                            │
│                                                                                                     │
│    Entity (defined once)     +    Locale Knowledge    →    Native Content                           │
│    ─────────────────────          ────────────────         ─────────────────                        │
│    key: "create-qr-code"          fr-FR:                   EntityContent@fr-FR:                     │
│    display_name: "Create QR"      • Terms: "créer",        slug: "créer-qr-code"                    │
│    description: "Generate a       "générer"                display_name: "Créer un QR Code"         │
│    new QR code"                   • Style: formal          description: "Générez votre QR Code"     │
│                                   • Culture: French                                                 │
│                                                                                                     │
│                                   ja-JP:                   EntityContent@ja-JP:                     │
│                                   • Terms: "作成する"       slug: "qr-code-sakusei"                  │
│                                   • Style: keigo           display_name: "QRコード作成"              │
│                                   • Culture: Japanese      description: "QRコードを作成できます"     │
│                                                                                                     │
│    BENEFITS:                                                                                        │
│    • Native quality (sounds natural to locals)                                                      │
│    • Write once, generate 200×                                                                      │
│    • Knowledge graph ensures consistency                                                            │
│                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## [I] Intro — Page 1: What is NovaNet?

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet       [1]Graph  [2]Nexus●                              ?:help  q:quit                       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ LEARN > Intro                                                                          [Page 1/3]   │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  ╔═══════════════════════════════════════════════════════════════════════════════════════════════╗  │
│  ║                         WHAT IS NOVANET?                                                      ║  │
│  ╚═══════════════════════════════════════════════════════════════════════════════════════════════╝  │
│                                                                                                     │
│  NovaNet is a KNOWLEDGE GRAPH that helps generate website content in 200+ languages.                │
│                                                                                                     │
│  Instead of translating content (which loses cultural nuance), NovaNet GENERATES                    │
│  content natively for each locale using local knowledge (vocabulary, culture, style).               │
│                                                                                                     │
│  ┌─ REAL EXAMPLE ───────────────────────────────────────────────────────────────────────────────┐  │
│  │                                                                                              │  │
│  │  You define ONE Entity:                                                                      │  │
│  │  ┌──────────────────────────────────────────────────────────────────────────────────────┐   │  │
│  │  │  Entity:                                                                             │   │  │
│  │  │    key: "qr-code"                                                                    │   │  │
│  │  │    display_name: "QR Code"                                                           │   │  │
│  │  │    description: "Two-dimensional barcode that encodes data in a scannable pattern"   │   │  │
│  │  └──────────────────────────────────────────────────────────────────────────────────────┘   │  │
│  │                                                                                              │  │
│  │  NovaNet generates NATIVE content for each locale:                                           │  │
│  │                                                                                              │  │
│  │  ┌─ French (fr-FR) ────────────────┐  ┌─ Japanese (ja-JP) ─────────────────┐               │  │
│  │  │  slug: "qr-code"                │  │  slug: "qr-code"                   │               │  │
│  │  │  display_name: "QR Code"        │  │  display_name: "QRコード"          │               │  │
│  │  │  description: "Code-barres 2D   │  │  description: "スマートフォンで     │               │  │
│  │  │  qui encode des données dans    │  │  スキャンできる2次元マトリックス   │               │  │
│  │  │  un motif scannable"            │  │  バーコード"                       │               │  │
│  │  └─────────────────────────────────┘  └────────────────────────────────────┘               │  │
│  │                                                                                              │  │
│  │  ┌─ Arabic (ar-AE) ────────────────┐  ┌─ Afrikaans (af-ZA) ────────────────┐               │  │
│  │  │  slug: "qr-code"                │  │  slug: "qr-kode"                   │               │  │
│  │  │  display_name: "رمز QR"         │  │  display_name: "QR-kode"           │               │  │
│  │  │  text_direction: "rtl" ←        │  │  description: "'n Tweedimensionele │               │  │
│  │  └─────────────────────────────────┘  │  strepieskode..."                  │               │  │
│  │                                       └────────────────────────────────────┘               │  │
│  └──────────────────────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                                     │
│  [j/k: scroll]  [Enter: next page]  [U: start Tutorial]                                             │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ✨ NovaNet currently supports 200 locales from Afrikaans (af-ZA) to Zulu (zu-ZA)        [1/10] [n]  │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## [I] Intro — Page 2: The Two Types of Nodes

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet       [1]Graph  [2]Nexus●                              ?:help  q:quit                       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ LEARN > Intro                                                                          [Page 2/3]   │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  ╔═══════════════════════════════════════════════════════════════════════════════════════════════╗  │
│  ║                         THE TWO TYPES OF NODES                                                ║  │
│  ╚═══════════════════════════════════════════════════════════════════════════════════════════════╝  │
│                                                                                                     │
│  NovaNet has TWO types of nodes. Understanding this is the key to everything.                       │
│                                                                                                     │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                                             │   │
│  │      META NODES (60 total)                     DATA NODES (200,000+)                        │   │
│  │      ══════════════════════                    ═════════════════════                        │   │
│  │      Define WHAT types exist                   Actual content instances                     │   │
│  │                                                                                             │   │
│  │      Think of it like:                         Think of it like:                            │   │
│  │      "What is a Locale?"                       "French (France) is a Locale"                │   │
│  │      "What is an Entity?"                      "QR Code is an Entity"                       │   │
│  │                                                                                             │   │
│  │      ┌─────────────────────┐                   ┌─────────────────────┐                      │   │
│  │      │ Kind: Locale        │                   │ Locale: fr-FR       │                      │   │
│  │      │ ─────────────────── │                   │ ─────────────────── │                      │   │
│  │      │ realm: shared       │    "fr-FR is     │ display_name:       │                      │   │
│  │      │ layer: config       │    a Locale"     │   "French (France)" │                      │   │
│  │      │ trait: invariant    │ ◄────────────────│ language_code: "fr" │                      │   │
│  │      │                     │                   │ country_code: "FR"  │                      │   │
│  │      │ (definition)        │                   │ script: "latin"     │                      │   │
│  │      └─────────────────────┘                   │ text_direction: ltr │                      │   │
│  │                                                └─────────────────────┘                      │   │
│  │                                                                                             │   │
│  │      ┌─────────────────────┐                   ┌─────────────────────┐                      │   │
│  │      │ Kind: Entity        │                   │ Entity: qr-code     │                      │   │
│  │      │ ─────────────────── │    "qr-code is   │ ─────────────────── │                      │   │
│  │      │ realm: org          │    an Entity"    │ display_name:       │                      │   │
│  │      │ layer: semantic     │ ◄────────────────│   "QR Code"         │                      │   │
│  │      │ trait: invariant    │                   │ is_pillar: true     │                      │   │
│  │      │                     │                   │ description: "2D    │                      │   │
│  │      │ (definition)        │                   │   barcode that..."  │                      │   │
│  │      └─────────────────────┘                   └─────────────────────┘                      │   │
│  │                                                                                             │   │
│  │      The relationship is called: OF_KIND                                                    │   │
│  │      (:Locale {key: 'fr-FR'})-[:OF_KIND]->(:Kind {label: 'Locale'})                        │   │
│  │                                                                                             │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                     │
│  [j/k: scroll]  [Enter: next page]  [p: previous]                                                   │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ✨ META = schema (60 definitions), DATA = instances (200K+ actual content)              [2/10] [n]  │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## [I] Intro — Page 3: Classification (Realm, Layer, Trait)

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet       [1]Graph  [2]Nexus●                              ?:help  q:quit                       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ LEARN > Intro                                                                          [Page 3/3]   │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  ╔═══════════════════════════════════════════════════════════════════════════════════════════════╗  │
│  ║                         HOW NODES ARE CLASSIFIED                                              ║  │
│  ╚═══════════════════════════════════════════════════════════════════════════════════════════════╝  │
│                                                                                                     │
│  Every Kind (type definition) has 3 classification properties:                                      │
│                                                                                                     │
│  ┌─ REALM: Where does it live? ─────────────────────────────────────────────────────────────────┐  │
│  │                                                                                              │  │
│  │  ◉ shared (39 nodes)                          ◎ org (21 nodes)                               │  │
│  │  Universal knowledge                          Organization-specific                          │  │
│  │  READ-ONLY (cannot modify)                    Your business content                          │  │
│  │                                                                                              │  │
│  │  Examples:                                    Examples:                                      │  │
│  │  • Locale (fr-FR, ja-JP, ar-AE)              • Entity (qr-code, wifi-qr)                    │  │
│  │  • Term ("abonnement mensuel")               • Page (homepage, pricing)                     │  │
│  │  • Culture (French formality rules)          • EntityContent (qr-code@fr-FR)                │  │
│  │                                                                                              │  │
│  └──────────────────────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                                     │
│  ┌─ LAYER: What is its function? ───────────────────────────────────────────────────────────────┐  │
│  │                                                                                              │  │
│  │  shared/config      → Locale, EntityCategory         (definitions)                          │  │
│  │  shared/knowledge   → Term, Expression, Culture      (locale expertise)                     │  │
│  │  org/semantic       → Entity, EntityContent          (your content)                         │  │
│  │  org/structure      → Page, Block                    (website structure)                    │  │
│  │  org/output         → PageGenerated, BlockGenerated  (LLM output)                           │  │
│  │                                                                                              │  │
│  └──────────────────────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                                     │
│  ┌─ TRAIT: How does it behave with locales? ────────────────────────────────────────────────────┐  │
│  │                                                                                              │  │
│  │  ■ invariant   Same everywhere           Entity, Page, Block (defined once)                 │  │
│  │  □ localized   Generated per locale      EntityContent@fr-FR (native content)               │  │
│  │  ◇ knowledge   Locale expertise          Term, Culture (local vocabulary)                   │  │
│  │  ★ generated   LLM output                PageGenerated (final HTML)                         │  │
│  │  ▪ aggregated  Computed metrics          SEOKeywordMetrics (analytics)                      │  │
│  │                                                                                              │  │
│  └──────────────────────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                                     │
│  ┌─ REAL YAML EXAMPLE ──────────────────────────────────────────────────────────────────────────┐  │
│  │                                                                                              │  │
│  │  # packages/core/models/node-kinds/org/semantic/entity.yaml                                 │  │
│  │  node:                                                                                       │  │
│  │    name: Entity                          ← The type name                                    │  │
│  │    realm: org                            ← WHERE: organization-specific                     │  │
│  │    layer: semantic                       ← WHAT: meaning/content layer                      │  │
│  │    trait: invariant                      ← HOW: same across all locales                     │  │
│  │    description: "Semantic unit representing products, features, concepts"                   │  │
│  │                                                                                              │  │
│  └──────────────────────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                                     │
│  [U: Start Tutorial]  [G: Glossary]  [T: Explore Traits]  [1: Try Graph Mode]                       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ✨ Ready to explore? Press [U] for the guided tutorial or [1] to jump into Graph mode!  [3/10] [n]  │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## [G] Glossary — Real Examples

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet       [1]Graph  [2]Nexus●                              ?:help  q:quit                       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ LEARN > Glossary                                                               [/] Search: _       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  ┌─ CONCEPTS (15) ────────────────────┬─ DEFINITION ────────────────────────────────────────────┐  │
│  │                                    │                                                         │  │
│  │  ▼ Graph Basics (4)                │  ══════════════════════════════════════════════════════ │  │
│  │      Meta Node                     │  ENTITY                                                 │  │
│  │      Data Node                     │  ══════════════════════════════════════════════════════ │  │
│  │    ● Entity               ◄────────│                                                         │  │
│  │      EntityContent                 │  A semantic unit representing products, features,       │  │
│  │                                    │  concepts, actions, or tools in your application.       │  │
│  │  ▷ Classification (3)              │                                                         │  │
│  │  ▷ Locale System (4)               │  CLASSIFICATION                                         │  │
│  │  ▷ Relationships (3)               │  ─────────────                                          │  │
│  │  ▷ Architecture (1)                │  • Realm: org (organization-specific)                   │  │
│  │                                    │  • Layer: semantic (meaning layer)                      │  │
│  │                                    │  • Trait: invariant (same across locales)               │  │
│  │                                    │                                                         │  │
│  │                                    │  YAML DEFINITION                                        │  │
│  │                                    │  ───────────────                                        │  │
│  │                                    │  node:                                                  │  │
│  │                                    │    name: Entity                                         │  │
│  │                                    │    realm: org                                           │  │
│  │                                    │    layer: semantic                                      │  │
│  │                                    │    trait: invariant                                     │  │
│  │                                    │                                                         │  │
│  │                                    │  REAL EXAMPLE                                           │  │
│  │                                    │  ────────────                                           │  │
│  │                                    │  (:Entity {                                             │  │
│  │                                    │    key: "qr-code",                                      │  │
│  │                                    │    display_name: "QR Code",                             │  │
│  │                                    │    is_pillar: true,                                     │  │
│  │                                    │    description: "Two-dimensional barcode..."            │  │
│  │                                    │  })                                                     │  │
│  │                                    │                                                         │  │
│  │                                    │  RELATIONSHIPS                                          │  │
│  │                                    │  ─────────────                                          │  │
│  │                                    │  • HAS_CONTENT → EntityContent (localized versions)     │  │
│  │                                    │  • BELONGS_TO → EntityCategory (THING, ACTION, etc.)    │  │
│  │                                    │  • SEMANTIC_LINK → Entity (related entities)            │  │
│  │                                    │                                                         │  │
│  │                                    │  SEE ALSO: EntityContent, Kind, Trait                   │  │
│  │                                    │                                                         │  │
│  └────────────────────────────────────┴─────────────────────────────────────────────────────────┘  │
│                                                                                                     │
│  [j/k: navigate]  [Enter: see related]  [/: search]  [y: copy]                                      │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ✨ Entity is the WHAT you're describing. EntityContent is the HOW for each locale.      [4/10] [n]  │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## [G] Glossary — EntityContent (with real fr-FR example)

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet       [1]Graph  [2]Nexus●                              ?:help  q:quit                       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ LEARN > Glossary                                                               [/] Search: _       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  ┌─ CONCEPTS (15) ────────────────────┬─ DEFINITION ────────────────────────────────────────────┐  │
│  │                                    │                                                         │  │
│  │  ▼ Graph Basics (4)                │  ══════════════════════════════════════════════════════ │  │
│  │      Meta Node                     │  ENTITY CONTENT                                         │  │
│  │      Data Node                     │  ══════════════════════════════════════════════════════ │  │
│  │      Entity                        │                                                         │  │
│  │    ● EntityContent        ◄────────│  Locale-native content for an Entity.                   │  │
│  │                                    │  NOT translated — GENERATED natively using locale       │  │
│  │  ▷ Classification (3)              │  knowledge (Terms, Culture, Style).                     │  │
│  │  ▷ Locale System (4)               │                                                         │  │
│  │  ▷ Relationships (3)               │  CLASSIFICATION                                         │  │
│  │  ▷ Architecture (1)                │  ─────────────                                          │  │
│  │                                    │  • Realm: org                                           │  │
│  │                                    │  • Layer: semantic                                      │  │
│  │                                    │  • Trait: localized (one per locale)                    │  │
│  │                                    │                                                         │  │
│  │                                    │  KEY PATTERN                                            │  │
│  │                                    │  ───────────                                            │  │
│  │                                    │  Composite key: entity:{entity_key}@{locale}            │  │
│  │                                    │  Example: "entity:qr-code@fr-FR"                        │  │
│  │                                    │                                                         │  │
│  │                                    │  REAL EXAMPLE (French)                                  │  │
│  │                                    │  ─────────────────────                                  │  │
│  │                                    │  (:EntityContent {                                      │  │
│  │                                    │    key: "entity:qr-code@fr-FR",                         │  │
│  │                                    │    entity_key: "qr-code",                               │  │
│  │                                    │    locale_key: "fr-FR",                                 │  │
│  │                                    │    slug: "qr-code",                                     │  │
│  │                                    │    display_name: "QR Code",                             │  │
│  │                                    │    description: "Code-barres 2D qui encode..."          │  │
│  │                                    │  })                                                     │  │
│  │                                    │                                                         │  │
│  │                                    │  REAL EXAMPLE (Japanese)                                │  │
│  │                                    │  ────────────────────────                               │  │
│  │                                    │  (:EntityContent {                                      │  │
│  │                                    │    key: "entity:qr-code@ja-JP",                         │  │
│  │                                    │    display_name: "QRコード",                            │  │
│  │                                    │    description: "スマートフォンでスキャンできる..."      │  │
│  │                                    │    cultural_notes: "Use keigo for business"             │  │
│  │                                    │  })                                                     │  │
│  │                                    │                                                         │  │
│  │                                    │  SEE ALSO: Entity (invariant parent), Locale, Term      │  │
│  │                                    │                                                         │  │
│  └────────────────────────────────────┴─────────────────────────────────────────────────────────┘  │
│                                                                                                     │
│  [j/k: navigate]  [Enter: see related]  [/: search]  [y: copy]                                      │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ✨ Entity = defined once, EntityContent = generated 200× (one per locale)               [5/10] [n]  │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## [G] Glossary — Term (Knowledge Atom)

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet       [1]Graph  [2]Nexus●                              ?:help  q:quit                       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ LEARN > Glossary                                                               [/] Search: _       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  ┌─ CONCEPTS (15) ────────────────────┬─ DEFINITION ────────────────────────────────────────────┐  │
│  │                                    │                                                         │  │
│  │  ▷ Graph Basics (4)                │  ══════════════════════════════════════════════════════ │  │
│  │  ▷ Classification (3)              │  TERM (Knowledge Atom)                                  │  │
│  │  ▼ Locale System (4)               │  ══════════════════════════════════════════════════════ │  │
│  │      Locale                        │                                                         │  │
│  │    ● Term                 ◄────────│  A vocabulary word or phrase specific to a locale.      │  │
│  │      Expression                    │  Part of the "Knowledge" that makes native generation   │  │
│  │      Culture                       │  possible.                                              │  │
│  │                                    │                                                         │  │
│  │  ▷ Relationships (3)               │  WHY IT MATTERS                                         │  │
│  │  ▷ Architecture (1)                │  ───────────────                                        │  │
│  │                                    │  When generating content for fr-FR, the LLM loads       │  │
│  │                                    │  French Terms to use native vocabulary instead of       │  │
│  │                                    │  translating from English.                              │  │
│  │                                    │                                                         │  │
│  │                                    │  CLASSIFICATION                                         │  │
│  │                                    │  ─────────────                                          │  │
│  │                                    │  • Realm: shared (universal)                            │  │
│  │                                    │  • Layer: knowledge                                     │  │
│  │                                    │  • Trait: knowledge (locale expertise)                  │  │
│  │                                    │                                                         │  │
│  │                                    │  REAL EXAMPLE (French pricing term)                     │  │
│  │                                    │  ───────────────────────────────────                    │  │
│  │                                    │  (:Term {                                               │  │
│  │                                    │    key: "subscription_monthly",                         │  │
│  │                                    │    value: "abonnement mensuel",                         │  │
│  │                                    │    domain: "pricing",                                   │  │
│  │                                    │    register: "formal",                                  │  │
│  │                                    │    synonyms: ["formule mensuelle", "forfait mensuel"],  │  │
│  │                                    │    avoid_in_context: ["legal documents"]                │  │
│  │                                    │  })                                                     │  │
│  │                                    │                                                         │  │
│  │                                    │  HOW IT'S USED                                          │  │
│  │                                    │  ────────────                                           │  │
│  │                                    │  1. Entity "pricing-monthly" needs French content       │  │
│  │                                    │  2. LLM loads Terms where domain = "pricing"            │  │
│  │                                    │  3. Uses "abonnement mensuel" instead of translating    │  │
│  │                                    │     "monthly subscription" literally                    │  │
│  │                                    │                                                         │  │
│  │                                    │  SEE ALSO: Locale, Expression, TermSet                  │  │
│  │                                    │                                                         │  │
│  └────────────────────────────────────┴─────────────────────────────────────────────────────────┘  │
│                                                                                                     │
│  [j/k: navigate]  [Enter: see related]  [/: search]  [y: copy]                                      │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ✨ Terms = native vocabulary, Expressions = idioms, Culture = social rules              [6/10] [n]  │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## [U] Tutorial — Step 1: Graph Fundamentals (Real Data)

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet       [1]Graph  [2]Nexus●                              ?:help  q:quit                       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ LEARN > Tutorial                                                                                    │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  YOUR JOURNEY    ◉━━━━━━━━━━○━━━━━━━━━━○━━━━━━━━━━○━━━━━━━━━━○    Step 1/5                          │
│                  Graph       Class      Arcs       Flow       Tree                                  │
│                                                                                                     │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                                             │   │
│  │  STEP 1: UNDERSTANDING THE GRAPH                                        ◉ In Progress      │   │
│  │  ═══════════════════════════════════════════════════════════════════════════════════════   │   │
│  │                                                                                             │   │
│  │  🎯 GOAL: Understand that NovaNet has definitions (META) and actual data (DATA)            │   │
│  │                                                                                             │   │
│  │  📖 THE KEY INSIGHT                                                                         │   │
│  │                                                                                             │   │
│  │  NovaNet stores TWO kinds of information:                                                   │   │
│  │                                                                                             │   │
│  │  ┌───────────────────────────────────────────────────────────────────────────────────┐     │   │
│  │  │                                                                                   │     │   │
│  │  │   META: "What types exist?"              DATA: "What instances exist?"            │     │   │
│  │  │   ─────────────────────────              ─────────────────────────────            │     │   │
│  │  │                                                                                   │     │   │
│  │  │   Kind: Locale                           Locale: af-ZA (Afrikaans)                │     │   │
│  │  │   Kind: Locale                           Locale: ar-AE (Arabic UAE)               │     │   │
│  │  │   Kind: Locale              ◄─────────   Locale: fr-FR (French France)            │     │   │
│  │  │   Kind: Locale                           Locale: ja-JP (Japanese)                 │     │   │
│  │  │   (1 definition)                         ... (200 instances)                      │     │   │
│  │  │                                                                                   │     │   │
│  │  │   Kind: Entity                           Entity: qr-code                          │     │   │
│  │  │   Kind: Entity              ◄─────────   Entity: wifi-qr                          │     │   │
│  │  │   (1 definition)                         Entity: vcard-qr                         │     │   │
│  │  │                                          ... (many instances)                     │     │   │
│  │  │                                                                                   │     │   │
│  │  └───────────────────────────────────────────────────────────────────────────────────┘     │   │
│  │                                                                                             │   │
│  │  ✋ TRY IT: Press [1] to go to Graph mode                                                   │   │
│  │                                                                                             │   │
│  │     ☐  Find "Locale" in the tree (it's under shared > config)                              │   │
│  │     ☐  Expand it to see actual locales: fr-FR, ja-JP, ar-AE...                             │   │
│  │     ☐  Click on "fr-FR" to see its properties (language_code: "fr", script: "latin")       │   │
│  │                                                                                             │   │
│  │  When done, press [2] to come back here.                                                   │   │
│  │                                                                                             │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                     │
│  [1: Go explore]  [Enter: Mark complete]  [n: Skip]                                                 │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ✨ Locale has 200 instances: from af-ZA (Afrikaans) to zu-ZA (Zulu)!                     [1/10] [n]  │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## [U] Tutorial — Step 3: Relationships (Real Arcs)

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ NovaNet       [1]Graph  [2]Nexus●                              ?:help  q:quit                       │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ LEARN > Tutorial                                                                                    │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  YOUR JOURNEY    ●━━━━━━━━━━●━━━━━━━━━━◉━━━━━━━━━━○━━━━━━━━━━○    Step 3/5                          │
│                  Graph ✓     Class ✓    Arcs       Flow       Tree                                  │
│                                                                                                     │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                                             │   │
│  │  STEP 3: RELATIONSHIPS (ARCS)                                           ◉ In Progress      │   │
│  │  ═══════════════════════════════════════════════════════════════════════════════════════   │   │
│  │                                                                                             │   │
│  │  🎯 GOAL: Understand how nodes connect to each other                                        │   │
│  │                                                                                             │   │
│  │  📖 THE KEY INSIGHT                                                                         │   │
│  │                                                                                             │   │
│  │  Nodes are connected by ARCS (relationships). Here's the main pattern:                      │   │
│  │                                                                                             │   │
│  │  ┌───────────────────────────────────────────────────────────────────────────────────┐     │   │
│  │  │                                                                                   │     │   │
│  │  │   Entity (invariant)                    EntityContent (localized)                 │     │   │
│  │  │   ════════════════════                  ═════════════════════════                 │     │   │
│  │  │                                                                                   │     │   │
│  │  │   (:Entity {                            (:EntityContent {                         │     │   │
│  │  │     key: "qr-code",         HAS_CONTENT   key: "entity:qr-code@fr-FR",            │     │   │
│  │  │     display_name: "QR Code" ──────────▶   display_name: "QR Code",                │     │   │
│  │  │   })                                      description: "Code-barres 2D..."        │     │   │
│  │  │                             HAS_CONTENT })                                        │     │   │
│  │  │                             ──────────▶ (:EntityContent {                         │     │   │
│  │  │                                           key: "entity:qr-code@ja-JP",            │     │   │
│  │  │                                           display_name: "QRコード"                │     │   │
│  │  │                                         })                                        │     │   │
│  │  │                                                                                   │     │   │
│  │  │   The Entity "qr-code" has ONE definition but MANY localized contents.            │     │   │
│  │  │                                                                                   │     │   │
│  │  └───────────────────────────────────────────────────────────────────────────────────┘     │   │
│  │                                                                                             │   │
│  │  ARC FAMILIES (how relationships are grouped):                                              │   │
│  │                                                                                             │   │
│  │  → ownership     Parent owns child (Entity HAS_CONTENT EntityContent)                      │   │
│  │  ⇢ localization  Links to locale (EntityContent FOR_LOCALE Locale)                         │   │
│  │  ~ semantic      Meaning connections (Entity SEMANTIC_LINK Entity)                         │   │
│  │  ⇒ generation    LLM pipeline (Block HAS_GENERATED BlockGenerated)                         │   │
│  │                                                                                             │   │
│  │  ✋ TRY IT: Press [1] to go to Graph mode, find Entity "qr-code" and see its arcs          │   │
│  │                                                                                             │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                     │
│  [1: Go explore]  [Enter: Mark complete]  [p: Previous]  [n: Skip]                                  │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ✨ HAS_CONTENT connects invariant nodes to their localized versions                      [3/10] [n]  │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Full Concept List for Glossary

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  GLOSSARY CONCEPTS (15)                                                                             │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│  GRAPH BASICS (4)                                                                                   │
│  ─────────────────                                                                                  │
│  • Meta Node      — Schema definitions (:Meta:Kind, :Meta:Realm)                                    │
│  • Data Node      — Actual instances (:Locale, :Entity, :Page)                                      │
│  • Entity         — Semantic unit (qr-code, wifi-qr, create-qr-code)                               │
│  • EntityContent  — Localized content (entity:qr-code@fr-FR)                                        │
│                                                                                                     │
│  CLASSIFICATION (3)                                                                                 │
│  ──────────────────                                                                                 │
│  • Realm          — WHERE: shared (universal) vs org (your business)                                │
│  • Layer          — WHAT: config, knowledge, semantic, output (10 total)                            │
│  • Trait          — HOW: invariant, localized, knowledge, generated, aggregated                     │
│                                                                                                     │
│  LOCALE SYSTEM (4)                                                                                  │
│  ─────────────────                                                                                  │
│  • Locale         — BCP-47 identifier (fr-FR, ja-JP, ar-AE) with properties                        │
│  • Term           — Native vocabulary ("abonnement mensuel" for fr-FR)                              │
│  • Expression     — Idioms and phrases ("C'est du gâteau" = it's easy)                             │
│  • Culture        — Social rules (formality, taboos, humor style)                                   │
│                                                                                                     │
│  RELATIONSHIPS (3)                                                                                  │
│  ─────────────────                                                                                  │
│  • Arc            — Connection between nodes (Entity -[:HAS_CONTENT]-> EntityContent)               │
│  • Family         — Arc category (ownership, localization, semantic, generation, mining)            │
│  • Scope          — intra_realm (same realm) vs cross_realm (shared ↔ org)                         │
│                                                                                                     │
│  ARCHITECTURE (1)                                                                                   │
│  ────────────────                                                                                   │
│  • Native Gen     — Generate content using locale knowledge, NOT translate                          │
│                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

Ces maquettes utilisent de vrais exemples du codebase. Ça te paraît plus clair pour un débutant ?