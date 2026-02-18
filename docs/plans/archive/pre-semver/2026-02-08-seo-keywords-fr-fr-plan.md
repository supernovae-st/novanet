# Plan: SEO Keywords fr-FR pour QR Code AI

**Date**: 2026-02-08
**Status**: En cours
**Locale cible**: fr-FR

---

## Contexte

Import de ~75,000 keywords SEO liés aux QR codes pour la locale fr-FR.
Objectif: les lier aux EntityL10n pour alimenter le SEO de QR Code AI.

---

## Inventaire des sources

| Source | Fichier | Keywords |
|--------|---------|----------|
| SEO (Google Ahrefs) | `google_fr_qr_matching-terms_2026-02-06.csv` | ~75,000 |
| SEO (PAAS Questions) | `paas_export_qr code.csv` | ~27 |
| Shopping | `shopping-qr code-fr-fr.xlsx` | TBD |
| Social | `youtube/tiktok social media.xlsx` | TBD |
| Geo | `ai models-qr code.xlsx` | TBD |

---

## Entities existantes (phases 1-14)

| Phase | Type | Count | Exemples |
|-------|------|-------|----------|
| 1 | PRODUCT | 8 | qr-code, dynamic-qr-code, static-qr-code |
| 2 | THING (content types) | 45 | qr-code-url, qr-code-wifi, qr-code-vcard |
| 3 | BARCODE | 8 | barcode, ean-13, upc-a, code-128 |
| 4 | FEATURE/TOOL | 20 | analytics, bulk-creation, api |
| 5 | MEDIUM | 20 | business-cards, flyers, menus-printed |
| 6 | ACTION | 15 | create-qr-code, scan-qr-code |
| 7 | INDUSTRY | 25 | restaurants, retail, healthcare |
| 8 | BRAND | 25 | instagram, linkedin, spotify |
| 9 | INTEGRATION | 12 | zapier-integration, hubspot-integration |
| 10 | CONCEPT | 8 | error-correction, quiet-zone |
| 11 | THING (social subcats) | 4 | qr-code-messaging, qr-code-video-platform |
| 12 | USE_CASE | 12 | qr-code-wedding, qr-code-tattoo |
| 13 | GUIDE | 10 | how-to-create-qr-code, qr-code-design-guide |
| 14 | COMPARISON | 8 | qr-code-vs-barcode, dynamic-vs-static |

**Total: ~220 entities invariantes**

---

## Décisions prises

| Question | Décision |
|----------|----------|
| Volume minimum | 50+/mois (~40K keywords) |
| Jeux vidéo (Brawl Stars, Pokemon...) | Garder séparément + gros volumes |
| Variantes orthographiques | Arc `SIMILAR_TO` entre elles |
| Marques concurrentes | Garder pour comparaisons |
| Priorités | Tout (création, scan, brands, industries) |

---

## Taxonomie des keywords

### Catégories à GARDER (~40K après filtre)

| # | Catégorie | Pattern | Entity Target | ~Count |
|---|-----------|---------|---------------|--------|
| 1 | Création | `créer\|générer\|create\|generator\|maker` | create-qr-code, qr-code-generator | ~4,800 |
| 2 | Scan/Lecture | `scanner\|scan\|lire\|lecteur\|reader\|flasher` | scan-qr-code, qr-code-scanner | ~10,100 |
| 3 | Gratuit/Pricing | `gratuit\|free\|illimité\|sans inscription` | (modifier) | ~2,700 |
| 4 | Social/Brands | `instagram\|facebook\|linkedin\|whatsapp\|...` | qr-code-{brand} | ~2,400 |
| 5 | Content Types | `wifi\|vcard\|url\|sms\|email\|pdf\|menu` | qr-code-{type} | ~2,000 |
| 6 | Devices | `iphone\|android\|samsung\|mobile\|téléphone` | (nouveau?) | ~4,500 |
| 7 | Industries | `restaurant\|retail\|hotel\|education\|...` | {industry} | ~1,500 |
| 8 | Mediums | `carte de visite\|flyer\|poster\|sticker` | {medium} | ~1,000 |
| 9 | Barcode | `barcode\|code barre\|ean\|upc\|data matrix` | barcode, {type} | ~440 |
| 10 | Developer | `api\|python\|java\|zxing\|sdk` | developers, api | ~580 |
| 11 | Concurrents | `qr code monkey\|qr tiger\|unitag\|flowcode` | comparisons | ~2,000 |
| 12 | Gaming QR | `brawl stars\|pokemon\|yo-kai\|clash royale` | (catégorie séparée) | ~5,900 |
| 13 | Concepts | `error correction\|quiet zone\|module` | {concept} | ~200 |
| 14 | Use Cases | `wedding\|tattoo\|museum\|loyalty` | {use-case} | ~500 |
| 15 | Guides/How-to | `comment\|how to\|tutorial\|guide` | {guide} | ~3,000 |

### Keywords à SUPPRIMER (~35K)

| Catégorie | Raison |
|-----------|--------|
| Langues non-FR | `qrコード`, `código qr`, `线上qr code` |
| Events temporaires | `jo paris qr`, `pass sanitaire` |
| Volume < 50 | Ultra longue traîne |
| Spam/Nonsense | Requêtes cassées |

---

## Architecture de connexion (v2 - simplifié)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  ARCHITECTURE SIMPLIFIÉE (2026-02-08)                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Entity (invariant, semantic layer)                                         │
│    │ key: "create-qr-code"                                                  │
│    │ type: ACTION                                                           │
│    │ display_name: "Create QR Code"                                         │
│    │                                                                        │
│    └──[:HAS_L10N]──> EntityL10n (localized, semantic layer)                 │
│                        │                                                    │
│                        │ # Identity (denormalized)                          │
│                        │ entity_key: "create-qr-code"                       │
│                        │ locale_key: "fr-FR"                                │
│                        │ slug: "creer-qr-code"                              │
│                        │                                                    │
│                        │ # Content                                          │
│                        │ display_name: "Créer un QR Code"                   │
│                        │ description: "Générer un nouveau..."               │
│                        │ llm_context: "USE: ... TRIGGERS: ..."              │
│                        │                                                    │
│                        ├──[:FOR_LOCALE]──> Locale {key: "fr-FR"}            │
│                        │                                                    │
│                        └──[:TARGETS]──> SEOKeyword (knowledge, seo layer)   │
│                                           │ key: "creer-qr-code-fr"         │
│                                           │ value: "créer qr code"          │
│                                           │ volume: 7300                    │
│                                           │ difficulty: 35                  │
│                                           │ intent: "transactional"         │
│                                           │                                 │
│                                           └──[:HAS_QUESTIONS]──> SEOQuestion│
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  SIMPLIFICATIONS vs v1:                                                     │
│                                                                             │
│  1. ✓ Supprimé [:EXPRESSES] (redondant - utiliser [:TARGETS] inverse)       │
│  2. ✓ Supprimé Locale --[:HAS_SEO_KEYWORDS]--> SEOKeyword                   │
│       (locale déduite via EntityL10n --[:FOR_LOCALE]--> Locale)             │
│  3. ✓ Ajouté entity_key, locale_key, slug sur EntityL10n                    │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  QUERIES UTILES:                                                            │
│                                                                             │
│  # Tous les keywords fr-FR                                                  │
│  MATCH (el:EntityL10n {locale_key: "fr-FR"})-[:TARGETS]->(kw:SEOKeyword)    │
│  RETURN kw.value, kw.volume ORDER BY kw.volume DESC                         │
│                                                                             │
│  # Keywords pour une Entity                                                 │
│  MATCH (el:EntityL10n {entity_key: "create-qr-code", locale_key: "fr-FR"})  │
│        -[:TARGETS]->(kw:SEOKeyword)                                         │
│  RETURN kw.value, kw.volume                                                 │
│                                                                             │
│  # Quelle EntityL10n pour un keyword? (inverse de TARGETS)                  │
│  MATCH (kw:SEOKeyword {value: "créer qr code"})<-[:TARGETS]-(el:EntityL10n) │
│  RETURN el.entity_key, el.display_name                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Naming discussion (TODO)

Le nom `EntityL10n` peut créer confusion avec `BlockL10n`/`PageL10n` (output layer).
Alternatives à considérer:
- `EntityLocale` - "l'entity pour cette locale"
- `SemanticL10n` - référence au layer
- `ConceptL10n` - plus descriptif
- Garder `EntityL10n` - techniquement correct (L10n = localized)

---

## Étapes d'implémentation

### Phase A: EntityL10n fr-FR (PRIORITÉ)

1. [ ] Générer les EntityL10n pour les ~220 entities existantes
2. [ ] Locale: fr-FR
3. [ ] Propriétés à remplir:
   - `display_name` (traduction française)
   - `description` (description en français)
   - `llm_context` (contexte LLM en français)
   - `entity_summary` (résumé en français)

### Phase B: Nettoyage keywords

1. [ ] Filtrer volume < 50
2. [ ] Supprimer langues non-FR
3. [ ] Supprimer events temporaires
4. [ ] Supprimer spam/nonsense

### Phase C: Classification keywords

1. [ ] Appliquer regex patterns
2. [ ] Mapper vers catégories
3. [ ] Identifier entity targets

### Phase D: Import NovaNet

1. [ ] Créer node types SEOKeyword, SEOCategory
2. [ ] Import batch Cypher
3. [ ] Créer arcs [:TARGETS] vers EntityL10n
4. [ ] Créer arcs [:SIMILAR_TO] entre variantes
5. [ ] Créer arcs [:HAS_PARENT] (parent_keyword)

---

## Questions ouvertes

- [ ] Comment générer les EntityL10n fr-FR ? (LLM batch? Manuel?)
- [ ] Faut-il un node SEOCategory ou juste une propriété `category` sur SEOKeyword?
- [ ] Quelle structure pour les keywords "Gaming QR" (catégorie séparée)?

---

## Fichiers liés

- Entities: `packages/core/data/entities/qrcode-ai/phase-*.yaml`
- Keywords: `docs/assets/keywods/fr-fr_qr/`
