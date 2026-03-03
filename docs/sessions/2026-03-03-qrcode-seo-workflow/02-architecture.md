# Architecture et Schemas

## Vue globale du systeme

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ARCHITECTURE NOVANET + NIKA                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│                        ┌─────────────────────────────────┐                      │
│                        │         NIKA WORKFLOW           │                      │
│                        │  (recherche-seo.nika.yaml)      │                      │
│                        └───────────┬─────────────────────┘                      │
│                                    │                                            │
│              ┌─────────────────────┼─────────────────────┐                      │
│              │                     │                     │                      │
│              ▼                     ▼                     ▼                      │
│     ┌────────────────┐   ┌────────────────┐   ┌────────────────┐               │
│     │    AHREFS      │   │  PERPLEXITY    │   │    NOVANET     │               │
│     │   (HTTP MCP)   │   │    (MCP)       │   │     (MCP)      │               │
│     └───────┬────────┘   └───────┬────────┘   └───────┬────────┘               │
│             │                    │                    │                         │
│             │ volumes            │ validation         │ lecture/ecriture        │
│             │ keywords           │ contexte           │ graphe                  │
│             │                    │                    │                         │
│             └──────────┬─────────┴────────────────────┘                         │
│                        │                                                        │
│                        ▼                                                        │
│     ┌───────────────────────────────────────────────────────────────────────┐  │
│     │                          NEO4J                                         │  │
│     │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                │  │
│     │  │   Entity    │───►│EntityNative │───►│ SEOKeyword  │                │  │
│     │  │  (defined)  │    │  (authored) │    │  (imported) │                │  │
│     │  └─────────────┘    └─────────────┘    └─────────────┘                │  │
│     │         │                  │                  │                        │  │
│     │         │ REPRESENTS       │ FOR_LOCALE       │ TARGETS                │  │
│     │         ▼                  ▼                  ▼                        │  │
│     │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                │  │
│     │  │    Page     │───►│ BlockNative │◄───│   Locale    │                │  │
│     │  │  (defined)  │    │ (generated) │    │  (defined)  │                │  │
│     │  └─────────────┘    └─────────────┘    └─────────────┘                │  │
│     └───────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Flow de generation du slug

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  FLOW: Entity → EntityNative → BlockNative:head-seo-meta                        │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ETAPE 1: Entity (invariant, existe deja)                                       │
│  ─────────────────────────────────────────                                      │
│  Entity:qr-code                                                                 │
│  ├── key: "qr-code"                                                             │
│  ├── display_name: "QR Code"                                                    │
│  └── denomination_forms: [text: "qr code", title: "QR Code"]                    │
│                                                                                 │
│           │                                                                     │
│           │ Workflow Nika: recherche Ahrefs + Perplexity                        │
│           ▼                                                                     │
│                                                                                 │
│  ETAPE 2: Recherche vraie data                                                  │
│  ─────────────────────────────────────────                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  AHREFS pour fr-FR:                                                     │   │
│  │  ├── "qr code"           → 450,000/mois  ← GAGNANT                      │   │
│  │  ├── "creer qr code"     → 74,000/mois                                  │   │
│  │  ├── "qr code gratuit"   → 90,000/mois                                  │   │
│  │  └── "code qr"           → 12,000/mois   ← PAS utilise!                 │   │
│  │                                                                         │   │
│  │  PERPLEXITY validation:                                                 │   │
│  │  "En France, 'QR code' est utilise tel quel (anglicisme)."              │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│           │                                                                     │
│           │ Nika ecrit dans NovaNet                                             │
│           ▼                                                                     │
│                                                                                 │
│  ETAPE 3: Creer/MAJ SEOKeywords                                                 │
│  ─────────────────────────────────────────                                      │
│  SEOKeyword:seo-qr-code@fr-FR                                                   │
│  ├── keyword: "qr code"                                                         │
│  ├── slug_form: "qr-code"                                                       │
│  ├── search_volume: 450000                                                      │
│  └── intent: "informational"                                                    │
│                                                                                 │
│           │                                                                     │
│           │                                                                     │
│           ▼                                                                     │
│                                                                                 │
│  ETAPE 4: MAJ EntityNative.denomination_forms                                   │
│  ─────────────────────────────────────────                                      │
│  EntityNative:qr-code@fr-FR                                                     │
│  └── denomination_forms:                                                        │
│      ├── text: "qr code"         ← Derive de recherche Ahrefs                  │
│      ├── title: "QR Code"                                                       │
│      ├── abbrev: "qr"                                                           │
│      └── url: "qr-code"          ← Slug derive du keyword gagnant              │
│                                                                                 │
│           │                                                                     │
│           │ Workflow Nika: generation LLM                                       │
│           ▼                                                                     │
│                                                                                 │
│  ETAPE 5: Generer BlockNative:head-seo-meta                                     │
│  ─────────────────────────────────────────                                      │
│  BlockNative:head-seo-meta@fr-FR                                                │
│  └── content:                                                                   │
│      ├── slug: "qr-code"                    ← Copie de denomination_forms.url  │
│      ├── full_path: "/fr/qr-code"           ← Calcule avec parent              │
│      ├── meta_title: "QR Code Gratuit"      ← LLM genere                       │
│      └── meta_description: "Creez un..."    ← LLM genere                       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Propriete du slug (ADR-030)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  QUI POSSEDE LE SLUG?                                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Entity           → PAS de slug (c'est le QUOI semantique)                      │
│                                                                                 │
│  EntityNative     → denomination_forms.url (vocabulaire pour construire slug)  │
│                     C'est la SOURCE pour le slug, pas le slug final            │
│                                                                                 │
│  Page             → slug INVARIANT (anglais, structure URL)                     │
│                     Ex: "qr-code" (pas localise)                                │
│                                                                                 │
│  BlockNative      → LE SLUG FINAL LOCALISE ← C'EST ICI!                         │
│  :head-seo-meta     content.slug: le segment URL pour cette locale             │
│                     content.full_path: chemin complet avec parent              │
│                                                                                 │
│  POURQUOI BlockNative?                                                          │
│  - Permet versioning independant (slug peut changer sans changer Page)          │
│  - Integre avec pipeline SEO (meta_title, meta_description dans meme bloc)     │
│  - Traçabilite via [:DERIVED_SLUG_FROM] → EntityNative                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Structure Page avec Blocks

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Page:qr-code-landing                                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Page                                                                           │
│  ├── key: "page:qr-code-landing"                                                │
│  ├── slug: "qr-code" (invariant)                                                │
│  │                                                                              │
│  └──[HAS_BLOCK]──►                                                              │
│      │                                                                          │
│      ├── {order: 0} → Block:qr-code-head-seo-meta  ← PREMIER (TOUJOURS)        │
│      │                 └── OF_TYPE → BlockType:head-seo-meta                    │
│      │                                                                          │
│      ├── {order: 1} → Block:qr-code-hero                                        │
│      │                 └── OF_TYPE → BlockType:hero                             │
│      │                                                                          │
│      ├── {order: 2} → Block:qr-code-what-is                                     │
│      │                 └── OF_TYPE → BlockType:feature-section                  │
│      │                                                                          │
│      ├── {order: 3} → Block:qr-code-use-cases                                   │
│      │                 └── OF_TYPE → BlockType:use-cases                        │
│      │                                                                          │
│      └── {order: 4} → Block:qr-code-cta                                         │
│                        └── OF_TYPE → BlockType:cta                              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Liens Keywords → EntityNatives

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  UN KEYWORD PEUT ETRE LIE A PLUSIEURS ENTITYNATIVES                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  SEOKeyword:"creer qr code"                                                     │
│           │                                                                     │
│           │ [:TARGETS]                                                          │
│           │                                                                     │
│           ├───────────────► EntityNative:qr-code@fr-FR                          │
│           │                 (l'entity principale)                               │
│           │                                                                     │
│           ├───────────────► EntityNative:qr-generator@fr-FR                     │
│           │                 (l'outil de generation)                             │
│           │                                                                     │
│           └───────────────► EntityNative:create-qr@fr-FR                        │
│                             (l'action de creer)                                 │
│                                                                                 │
│  → Convergence boost = 1 + (3 × 0.2) = 1.6                                      │
│  → Ce keyword a plus de poids car il est au centre du cluster semantique       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```
