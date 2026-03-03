# Clarifications et Decisions - novanet_write

**Date**: 2026-03-03 | **Status**: En attente de validation

---

## 1. PROBLEME CRITIQUE: Slug Immutable

### Le Danger

```
Jour 1:  slug = "qr-code"     → URL: /fr/qr-code ✅
Jour 30: slug = "creer-qr"    → URL change → 404 → SEO DETRUIT! 💀
```

### Options

| Option | Description | Avantage | Inconvénient |
|--------|-------------|----------|--------------|
| **A** | `slug_locked: true` sur EntityNative | Simple | Mécanisme de lock à créer |
| **B** | `canonical_slug` séparé (set_once) | Clair | Change le schema |
| **C** | Slug dans PageNative (ADR-030) | Conforme architecture | Plus complexe |

### Recommandation: Option C

Conforme à ADR-030 "Slug Ownership":
- **EntityNative.denomination_forms** = vocabulaire pour CONSTRUIRE le slug
- **PageNative.content.slug** = slug FINAL, immutable après création

```
EntityNative:qr-code@fr-FR
└── denomination_forms: [text, title, abbrev]  ← PAS de url

PageNative:qr-code-landing@fr-FR
└── content:
    ├── slug: "qr-code"          ← SLUG FINAL (immutable)
    └── slug_locked: true        ← Verrou après déploiement
```

---

## 2. Nika Crée des Entity ?

### Décision: NON pour v1

Entity a trait `defined` = read-only. Raison: on ne veut pas que l'IA invente des concepts fondamentaux.

### Future (post-v1)

Nouveau trait potentiel: `discovered`

```yaml
Entity:qr-code-restaurant
  trait: discovered
  discovered_by: workflow:seo-research
  confidence: 0.85
  needs_review: true  # Flag pour validation humaine
```

---

## 3. "code qr" = Terme à NE PAS utiliser

### Décision: Filtrage + is_negative

1. **Filtrer les perdants évidents**: Si volume < 10% du gagnant avec même intent → IGNORE
2. **Cas intéressants**: Créer avec `is_negative: true`

```
SEOKeyword:seo:code-qr@fr-FR
├── keyword: "code qr"
├── search_volume: 12000
├── is_negative: true
└── reason: "Traduction littérale perdante vs anglicisme"
```

Le LLM voit dans son contexte: "À éviter: code qr"

---

## 4. Workflows Plus Intelligents

### Problème

Un simple appel Ahrefs n'est pas suffisant. Il faut:
- Multi-sources (Ahrefs, Semrush, SERP)
- Validation linguistique
- Débat et consensus

### Architecture Proposée

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PHASE 1: RECHERCHE PARALLELE                                              │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  agent: "ahrefs-researcher"     ─┐                                         │
│  agent: "semrush-researcher"    ─┼─► Fusion des résultats                  │
│  agent: "serp-analyzer"         ─┘                                         │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│  PHASE 2: VALIDATION                                                       │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  infer: "Linguistic validator"  ─┐                                         │
│  infer: "Cultural validator"    ─┴─► Vérifie naturalité + taboos          │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│  PHASE 3: DEBATE                                                           │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  agent: "advocate-1" (défend option A)                                     │
│  agent: "advocate-2" (défend option B)                                     │
│  agent: "judge" (évalue et décide)                                         │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│  PHASE 4: ECRITURE (seulement après consensus)                             │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  invoke: novanet_write (SEOKeywords, TARGETS, Terms)                       │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Création/Update de Terms

### Décision: OUI

Term a trait `imported` → writable par Nika

### Workflow

1. **Découverte**: SEO révèle termes associés (génération, personnalisation, scanner)
2. **Création Term**: `novanet_write` avec operation `upsert_node`
3. **Création USES_TERM**: Arc direct EntityNative → Term
4. **Loading**: `novanet_generate` charge en 1 hop (vs 4 hops via TermSet)

```
EntityNative:qr-code@fr-FR
├──[:USES_TERM {purpose: "action_verb", temperature: 0.8}]──► Term:"génération"
├──[:USES_TERM {purpose: "feature", temperature: 0.7}]──► Term:"personnalisation"
└──[:USES_TERM {purpose: "device", temperature: 0.6}]──► Term:"scanner"
```

---

## 6. Résumé des Décisions

| Question | Décision | Statut |
|----------|----------|--------|
| Slug immutable | BlockNative:head-seo-meta owns slug + slug_locked flag | ✅ VALIDÉ |
| Nika crée Entity | Non pour v1, future avec `discovered` | ✅ VALIDÉ |
| Keywords négatifs | is_negative flag + negative_reason + preferred_alternative | ✅ VALIDÉ |
| Workflow intelligent | Multi-agents + debate + for_each + include | ✅ VALIDÉ |
| Création Terms | Oui via USES_TERM avec purpose + temperature | ✅ VALIDÉ |
| Schema validation | Health check au startup | ✅ VALIDÉ |
| is_slug_source singleton | Takeover pattern (demote old, not reject) | ✅ VALIDÉ |

---

## 7. Impact sur le Plan d'Implémentation

### Modifications au Plan

1. **novanet_write** doit vérifier `slug_locked` avant update
2. **SEOKeyword schema** ajouter `is_negative: boolean`
3. **USES_TERM properties** ajouter `purpose`, `temperature`
4. **Workflow SEO** plus complexe que prévu (multi-agents)

### Nouveau Fichier à Créer

- `src/validation.rs`: Logique de validation slug_locked
- Schemas à mettre à jour: SEOKeyword, USES_TERM

---

## 8. Questions Ouvertes

1. **Qui lock le slug?** Workflow de déploiement? Manuellement?
2. **Seuil is_negative**: 10% du gagnant est-il le bon seuil?
3. **Debate workflow**: Combien d'advocates? Critères du juge?
4. **Temperature USES_TERM**: Comment calculer? Basé sur quoi?

---

## Next Steps

1. [ ] Valider les décisions ci-dessus
2. [ ] Mettre à jour les schemas si nécessaire
3. [ ] Commencer l'implémentation avec les clarifications intégrées
