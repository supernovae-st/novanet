# Session: EntityNative + SEO Workflow

**Date**: 2026-03-03
**Objectif**: Valider la méthodologie NovaNet + Nika pour génération native multilingue
**Focus**: Entity "qr-code" comme cas d'usage pilote

---

## 1. Contexte du Problème

### Le serpent qui se mord la queue

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  PROBLÈME : Comment créer EntityNative AVANT d'avoir les SEO keywords ?         │
│             Mais les SEO keywords dépendent de l'Entity...                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Entity (defined)                                                               │
│     │                                                                           │
│     │  "C'est QUOI un QR code ?" ← On définit le concept d'abord                │
│     │                                                                           │
│     ▼                                                                           │
│  EntityNative (authored) ← MAIS comment savoir les bons termes locaux ?         │
│     │                                                                           │
│     │  "Comment dit-on QR code en fr-FR ?"                                      │
│     │  → "qr code" ou "code qr" ?                                               │
│     │  → Quelles subtilités culturelles ?                                       │
│     │                                                                           │
│     ▼                                                                           │
│  SEOKeyword (imported) ← Recherche Perplexity + Ahrefs                          │
│     │                                                                           │
│     │  Perplexity: "Les français disent 'qr code', pas 'code qr'"               │
│     │  Ahrefs: "qr code" = 450K/mois, "code qr" = 12K/mois                      │
│     │                                                                           │
│     ▼                                                                           │
│  EntityNative.denomination_forms.url ← Slug basé sur recherches réelles         │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Solution : Workflow en 2 phases

```
PHASE 1: Définition (sans SEO)
══════════════════════════════
Entity:qr-code (defined)
  └── EntityNative:qr-code@fr-FR (authored - draft)
        ├── denomination_forms: [text, title, abbrev] ← Termes initiaux
        └── url: null ← À REMPLIR après recherche SEO

PHASE 2: Enrichissement SEO (workflow Nika)
════════════════════════════════════════════
1. Recherche Perplexity: "Comment dit-on QR code en français ?"
2. Recherche Ahrefs: Volume pour chaque variante
3. Appliquer la FORMULE de choix (pas juste le volume max)
4. Créer SEOKeyword nodes avec slug_form
5. Lier EntityNative -[:TARGETS {is_slug_source: true}]-> SEOKeyword
6. Mettre à jour EntityNative.denomination_forms.url
```

---

## 2. La Formule de Choix SEO

**Source**: `docs/plans/2026-02-14-v0125-architecture-visual.md`

### Pourquoi pas juste le volume max ?

```
PROBLÈME : Volume seul donne le mauvais slug
═══════════════════════════════════════════

EntityNative:qr-generator@fr-FR TARGETS:
├── "créer qr code"      (vol: 50,000)
├── "qr code gratuit"    (vol: 80,000)  ← Plus gros mais FAUX!
└── "générateur qr"      (vol: 5,000)

"gratuit" est un ATTRIBUT, pas un substitut de "generator"
"créer" est L'ACTION, sémantiquement équivalent

→ On ne veut PAS "/qr-code-gratuit" pour la page generator
→ On veut "/creer-qr-code" (l'action = l'outil)
```

### La Formule

```
score = volume × sem_coef × convergence_boost
```

| Facteur | Description |
|---------|-------------|
| **volume** | Volume de recherche Ahrefs |
| **sem_coef** | Coefficient sémantique (0.0 - 1.0) basé sur SEMANTIC_LINK.type |
| **convergence_boost** | 1 + (N × 0.2) où N = nombre d'entities liées au même keyword |

### Coefficients Sémantiques

| link_type | sem_coef | Description |
|-----------|----------|-------------|
| same_as | 1.0 | Synonyme parfait |
| used_for | 0.95 | Outil → action (highest) |
| is_action_on | 0.85 | Action sur X |
| variant_of | 0.9 | Variante |
| type_of | 0.8 | Taxonomie |
| enables | 0.8 | Capacité |
| part_of | 0.85 | Composant |
| exhibits | 0.7 | Caractéristique |
| related_to | 0.5 | Association générale |
| contrasts | 0.4 | Comparaison/alternative |
| attribute_of | 0.3 | **Attribut pénalisé !** (ex: "gratuit") |

### Exemple Calcul

```
Page:qr-generator @fr-FR
════════════════════════

"générateur qr":
  volume = 5,000
  sem_coef = 1.0 (direct)
  convergence = 1.0
  SCORE = 5,000 × 1.0 × 1.0 = 5,000

"qr code gratuit":
  volume = 80,000
  sem_coef = 0.3 (attribute_of → pénalisé!)
  convergence = 1.0
  SCORE = 80,000 × 0.3 × 1.0 = 24,000

"créer qr code":
  volume = 50,000
  sem_coef = 0.95 (is_action_on)
  convergence = 1.2 (2 entities: qr-generator + create-qr)
  SCORE = 50,000 × 0.95 × 1.2 = 57,000  ← WINNER!

RESULT:
  slug = "créer-qr-code" (pas "qr-code-gratuit" !)
```

---

## 3. État Actuel dans Neo4j

### Ce qu'on a

| Node | Status | Problème |
|------|--------|----------|
| Entity:qr-code | ✅ Existe | OK |
| EntityNative:qr-code@fr-FR | ✅ Existe | url="créer-un-qr-code" (deviné, pas recherché) |
| Page:qr-code-landing | ✅ Existe | slug="qr-code" |
| SEOKeyword:seo-qr-code | ⚠️ VIDE | keyword=null, volume=null |
| Block:head-seo-meta | ❓ À vérifier | Peut-être pas créé |
| BlockNative:head-seo-meta@fr-FR | ❌ N'existe pas | À créer |

### Ce qu'il faut faire

1. **Recherche SEO** (workflow Nika)
   - Perplexity: Comment dit-on "QR code" en fr-FR ?
   - Ahrefs: Volume pour "qr code", "code qr", "créer qr code", etc.

2. **Créer/Mettre à jour SEOKeyword**
   ```cypher
   MERGE (k:SEOKeyword {key: 'seo-qr-code@fr-FR'})
   SET k.keyword = 'qr code',
       k.slug_form = 'qr-code',
       k.search_volume = 450000,
       k.intent = 'informational'
   ```

3. **Lier EntityNative → SEOKeyword**
   ```cypher
   MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
   MATCH (k:SEOKeyword {key: 'seo-qr-code@fr-FR'})
   MERGE (en)-[:TARGETS {rank: 'primary', is_slug_source: true}]->(k)
   ```

4. **Mettre à jour denomination_forms.url**
   ```cypher
   MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
   SET en.denomination_forms = '[
     {"type": "text", "value": "qr code", "priority": 1},
     {"type": "title", "value": "QR Code", "priority": 1},
     {"type": "abbrev", "value": "qr", "priority": 1},
     {"type": "url", "value": "qr-code", "priority": 1}
   ]'
   ```

5. **Créer Block:head-seo-meta**
   - Premier bloc de Page:qr-code-landing (order=0)

6. **Générer BlockNative:head-seo-meta@fr-FR**
   - slug, meta_title, meta_description

---

## 4. Architecture du Slug (Rappel)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  QUI POSSÈDE QUOI (ADR-030)                                                     │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Entity             → Pas de slug. Concept sémantique invariant.                │
│                       key: "qr-code"                                            │
│                                                                                 │
│  EntityNative       → denomination_forms contient le vocabulaire localisé       │
│                       text: "qr code"                                           │
│                       title: "QR Code"                                          │
│                       abbrev: "qr"                                              │
│                       url: "qr-code" ← SOURCE pour le slug final                │
│                                                                                 │
│  Page               → slug INVARIANT (anglais, structure)                       │
│                       slug: "qr-code"                                           │
│                                                                                 │
│  BlockNative        → LE SLUG FINAL LOCALISÉ                                    │
│  :head-seo-meta       content.slug: "qr-code"                                   │
│                       content.full_path: "/fr/qr-code"                          │
│                       content.meta_title: "QR Code Gratuit | QR Code AI"        │
│                       content.meta_description: "Créez un QR code..."           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Workflow Nika Proposé

### Phase 1: Recherche SEO (populate-seo-keywords.nika.yaml)

```yaml
workflow: populate-seo-keywords
description: "Recherche SEO pour une Entity dans une locale"

params:
  entity_key: string    # ex: "qr-code"
  locale: string        # ex: "fr-FR"

tasks:
  # 1. Récupérer l'Entity depuis NovaNet
  - id: get_entity
    invoke: novanet_describe
    params:
      describe: entity
      entity_key: $entity_key
    use.ctx: entity_data

  # 2. Recherche Perplexity - Comment dit-on ce terme ?
  - id: search_perplexity
    invoke: perplexity_search
    params:
      query: |
        Comment dit-on "$entity_data.display_name" en $locale ?
        Quel est le terme le plus utilisé ?
        Y a-t-il des subtilités culturelles ?
    use.ctx: perplexity_result

  # 3. Recherche Ahrefs - Volumes de recherche
  - id: search_ahrefs
    invoke: ahrefs_keywords
    params:
      keywords:
        - "$entity_data.display_name"
        - variants from perplexity_result
      country: $locale.country
    use.ctx: ahrefs_data

  # 4. Appliquer la formule de choix
  - id: calculate_best_slug
    infer: |
      Analysez les données SEO et choisissez le meilleur slug.

      Formule: score = volume × sem_coef × convergence_boost

      Données Perplexity: $perplexity_result
      Données Ahrefs: $ahrefs_data

      Retournez:
      - best_keyword: le terme gagnant
      - slug_form: le slug dérivé
      - rationale: pourquoi ce choix
    use.ctx: slug_decision

  # 5. Créer/mettre à jour SEOKeyword dans NovaNet
  - id: create_keyword
    invoke: novanet_query
    params:
      cypher: |
        MERGE (k:SEOKeyword {key: 'seo-$slug_decision.slug_form@$locale'})
        SET k.keyword = $slug_decision.best_keyword,
            k.slug_form = $slug_decision.slug_form,
            k.search_volume = $ahrefs_data.volume,
            k.intent = 'informational'
        RETURN k

  # 6. Mettre à jour EntityNative
  - id: update_entity_native
    invoke: novanet_query
    params:
      cypher: |
        MATCH (en:EntityNative {key: 'entity:$entity_key@$locale'})
        MATCH (k:SEOKeyword {key: 'seo-$slug_decision.slug_form@$locale'})
        MERGE (en)-[:TARGETS {rank: 'primary', is_slug_source: true}]->(k)
        // Update denomination_forms with url
        ...
```

### Phase 2: Génération BlockNative (generate-head-seo.nika.yaml)

```yaml
workflow: generate-head-seo
description: "Génère le BlockNative:head-seo-meta pour une Page/Locale"

params:
  page_key: string
  locale: string

tasks:
  # 1. Charger contexte complet depuis NovaNet
  - id: load_context
    invoke: novanet_generate
    params:
      focus_key: $page_key
      locale: $locale
      mode: block
      token_budget: 4000
    use.ctx: generation_context

  # 2. Générer le head-seo-meta
  - id: generate_seo_block
    infer: |
      Générez le bloc head-seo-meta pour cette page.

      Context: $generation_context

      RÈGLES:
      - slug: Utilisez EXACTEMENT denomination_forms.url
      - meta_title: Max 60 chars, utilisez denomination_forms.title
      - meta_description: Max 160 chars, inclure CTA

      Retournez JSON:
      {
        "slug": "...",
        "full_path": "/fr/...",
        "meta_title": "...",
        "meta_description": "..."
      }
    use.ctx: seo_content

  # 3. Créer BlockNative dans NovaNet
  - id: create_block_native
    invoke: novanet_query
    params:
      cypher: |
        MATCH (b:Block {key: '$page_key:head-seo-meta:0'})
        MERGE (bn:BlockNative {key: 'block:$page_key:head-seo-meta@$locale'})
        SET bn.content = $seo_content,
            bn.block_type = 'head-seo-meta',
            bn.locale_key = '$locale',
            bn.status = 'draft'
        MERGE (b)-[:HAS_NATIVE]->(bn)
        RETURN bn
```

---

## 6. Questions Ouvertes

1. **Ahrefs MCP** - Est-il disponible ? Je ne le vois pas dans les outils configurés.

2. **Locales pilotes** - On commence par quelles locales ?
   - Suggestion: fr-FR, en-US, de-DE, es-ES, ja-JP (5 représentatives)

3. **Keywords à rechercher pour qr-code** - Liste initiale ?
   - "qr code", "code qr", "créer qr code", "générateur qr code", "qr code gratuit", etc.

4. **Validation humaine** - À quel moment ?
   - Après recherche SEO (avant de créer les nodes) ?
   - Après génération BlockNative ?

---

## 7. Prochaines Étapes

- [ ] Vérifier si Block:head-seo-meta existe pour Page:qr-code-landing
- [ ] Vérifier l'état des 200 locales
- [ ] Définir le workflow Nika exact
- [ ] Tester sur fr-FR d'abord
- [ ] Étendre à 5 locales pilotes
- [ ] Automatiser pour 200 locales

---

*Session en cours - document mis à jour au fur et à mesure*
