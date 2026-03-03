# Etat Neo4j (2026-03-03)

## Entity:qr-code

```cypher
MATCH (e:Entity {key: 'qr-code'})
RETURN e
```

**Resultat**:
```json
{
  "key": "qr-code",
  "display_name": "QR Code",
  "denomination_forms": [
    {"type": "text", "value": "qr code"},
    {"type": "title", "value": "QR Code"},
    {"type": "abbrev", "value": "qr"}
  ],
  "is_pillar": true
}
```

**Relations**:
- 5 HAS_CHILD: custom-qr-code, static-qr-code, qr-code-scanner, qr-code-generator, dynamic-qr-code
- 2 HAS_KEYWORD: seo-qr-code, seo-free-qr-code
- 1 SEMANTIC_LINK: barcode
- 1 HAS_NATIVE: entity:qr-code@fr-FR

## EntityNative:qr-code@fr-FR

```json
{
  "key": "entity:qr-code@fr-FR",
  "entity_key": "qr-code",
  "locale_key": "fr-FR",
  "display_name": "QR Code",
  "curation_status": "ai_generated",
  "denomination_forms": {
    "text": "qr code",
    "title": "QR Code",
    "abbrev": "qr",
    "url": "creer-un-qr-code"  // ⚠️ DEVINES, pas recherche!
  }
}
```

## Page:qr-code-landing

```json
{
  "key": "page:qr-code-landing",
  "slug": "qr-code",
  "display_name": "QR Code Landing Page"
}
```

**Blocks (HAS_BLOCK)**:
| order | block_key |
|-------|-----------|
| 0 | block:qr-code-head-seo-meta (CREE CETTE SESSION) |
| 1 | block:qr-code-hero |
| 2 | block:qr-code-what-is |
| 3 | block:qr-code-use-cases |
| 4 | block:qr-code-cta |

## SEOKeywords (VIDES!)

```json
{
  "key": "seo-qr-code",
  "keyword": null,        // ⚠️ VIDE
  "search_volume": null,  // ⚠️ VIDE
  "intent": null          // ⚠️ VIDE
}
```

## BlockType:head-seo-meta

```json
{
  "key": "head-seo-meta",
  "display_name": "SEO Meta Block",
  "description": "SEO metadata block - MUST be first block (order=0)",
  "structure": {
    "slug": "string (URL segment)",
    "meta_title": "string (max 60)",
    "meta_description": "string (max 160)"
  }
}
```

## Ce qu'il manque

1. **SEOKeywords avec vraies donnees**
   - Creer via workflow Nika + Ahrefs

2. **EntityNative.denomination_forms.url corrige**
   - Actuellement: "creer-un-qr-code" (devine)
   - Devrait etre: "qr-code" (base sur vraies recherches)

3. **BlockNative:head-seo-meta@fr-FR**
   - A generer une fois les donnees SEO presentes
