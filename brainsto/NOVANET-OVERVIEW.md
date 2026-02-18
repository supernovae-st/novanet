# NovaNet — Quick Overview

## What Is It?

**NovaNet** = Native content generation engine for 200+ locales using Neo4j knowledge graphs.

## The Core Idea

```
WRONG:  English → Translate → French, Spanish, Japanese...
RIGHT:  Entity (concept) → Generate natively → Content in each locale
```

**Why?** Translation loses cultural nuance. Native generation preserves it.

## Real Example

```
Entity:qr-code-generator (defined once)
    │
    ├── EntityNative@fr-FR (authored)
    │   └── "Générateur de QR Code"
    │   └── keywords: générateur, créer, gratuit
    │
    ├── EntityNative@ja-JP (authored)
    │   └── "QRコードジェネレーター"
    │   └── keywords: QRコード作成, 無料
    │
    └── EntityNative@es-MX (authored)
        └── "Generador de Código QR"
        └── keywords: crear, código qr, gratis
```

Each locale gets **native** content with local SEO keywords, idioms, and cultural context.

## Architecture at a Glance

```
SHARED (40 nodes) — Universal, READ-ONLY
├── Locale definitions (fr-FR, en-US, ja-JP...)
├── Geographic data (countries, regions)
└── Knowledge atoms (Terms, Expressions, Culture)

ORG (21 nodes) — Your organization's content
├── Project, Brand, Pages, Blocks
├── Entities (semantic concepts)
└── Generated output (PageNative, BlockNative)
```

## Key Numbers

| Metric | Value |
|--------|-------|
| Node Types | 61 |
| Arc Types | 182 |
| Target Locales | 200+ |
| Target App | [QR Code AI](https://qrcode-ai.com) |

## Tech Stack

- **Graph DB**: Neo4j 5.26
- **Schema**: YAML (source of truth)
- **Frontend**: Next.js 16 + React 19
- **CLI/TUI**: Rust
- **AI**: Claude API

## The Value Proposition

| Traditional | NovaNet |
|-------------|---------|
| 200 locales = 200x translation cost | Define once, generate 200x |
| Translation loses nuance | Native generation preserves culture |
| Keyword research per locale | SEOKeyword nodes in graph |
| Manual content sync | Single graph = single source of truth |

---

*See NOVANET-SPEC.md for full architecture details.*
