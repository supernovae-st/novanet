# Executive Summary: QR Code SEO Workflow

**Session**: 2026-03-03 | **Status**: novanet_write IMPLEMENTED + Workflows CREATED

---

## TL;DR

On construit un workflow Nika qui:
1. **Recherche** les vrais termes SEO via DataForSEO (pas deviner)
2. **Ecrit** les resultats dans NovaNet (SEOKeyword, EntityNative, Term)
3. **Genere** du contenu qui utilise les vrais termes

**Probleme actuel**: NovaNet MCP est READ-ONLY. On a besoin de `novanet_write`.

---

## DataForSEO Test Results (2026-03-03)

**API fonctionne.** Resultats reels pour fr-FR (France):

| Keyword | Volume/mois | Competition | CPC |
|---------|-------------|-------------|-----|
| qr code | 135,000 | MEDIUM | 0.94€ |
| code qr | 135,000 | MEDIUM | 0.94€ |
| qr code gratuit | 40,500 | MEDIUM | 0.94€ |
| generateur qr code | 9,900 | HIGH | 1.01€ |
| creer qr code | 5,400 | HIGH | 1.30€ |

**Observation**: Google traite "qr code" et "code qr" comme synonymes (meme volume).
**Decision**: Utiliser "qr-code" pour le slug (forme internationale, sans accents).

---

## Le Probleme Fondamental

```
MAUVAISE APPROCHE (ce que je faisais):
┌─────────────────────────────────────────────────────────────────────────────┐
│  Claude devine "code QR" pour fr-FR                                         │
│  └── FAUX! Les francais cherchent "QR code" (anglicisme)                    │
│      Volume Ahrefs: "QR code" = 110K/mois vs "code QR" = 8.5K/mois          │
└─────────────────────────────────────────────────────────────────────────────┘

BONNE APPROCHE (ce qu'on construit):
┌─────────────────────────────────────────────────────────────────────────────┐
│  Nika workflow → Ahrefs → decouvre "QR code" gagne                          │
│  └── Ecrit dans NovaNet → EntityNative.denomination_forms.text = "QR code"  │
│      └── Generations futures utilisent le BON terme                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Architecture Cle

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CYCLE VERTUEUX                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. RECHERCHE (DataForSEO MCP) ✅ FONCTIONNE                                │
│     └── Retourne keywords avec volumes REELS                                │
│     └── Config: ~/.spn/mcp.yaml + ~/.spn/env                                │
│                                                                             │
│  2. SCORING (Nika infer)                                                    │
│     └── score = volume x sem_coef x intent_boost x trend_factor             │
│                                                                             │
│  3. ECRITURE (novanet_write) ← BLOQUE                                       │
│     ├── SEOKeyword (volumes reels)                                          │
│     ├── EntityNative (denomination_forms)                                   │
│     ├── Term (vocabulaire decouvert)                                        │
│     └── Arcs: TARGETS, USES_TERM                                            │
│                                                                             │
│  4. GENERATION FUTURE                                                       │
│     └── novanet_generate retourne les VRAIS termes                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Etat Actuel Neo4j

| Node | Status | Notes |
|------|--------|-------|
| Entity:qr-code | Existe | 130+ entities QR-related |
| EntityNative:qr-code@fr-FR | N'existe pas | A creer avec vraie data |
| SEOKeyword fr-FR | 0 | en-US a 1487, fr-FR = vide |
| Term fr-FR | 0 | TermSet vide |
| Block:qr-code-head-seo-meta | Cree | order=0, connecte a Page |
| Locale:fr-FR → Slugification | Existe | slug_rule: "latin_preserve" |

---

## Ce Qui Bloquait → RÉSOLU ✅

~~**novanet_write** n'existe pas encore.~~ → **IMPLÉMENTÉ !**

Le MCP NovaNet a maintenant **12 tools** (11 read + 1 write):
- ✅ `novanet_write` - Opérations: upsert_node, create_arc, update_props
- ✅ Validation schema via introspect
- ✅ Permissions par trait (authored/imported/generated = writable)
- ✅ 412 tests passent

**Peut écrire**:
- `SEOKeyword` (trait: imported) — keywords DataForSEO
- `EntityNative` (trait: authored) — contenu locale
- `Term` (trait: imported) — vocabulaire découvert
- Arcs: `TARGETS`, `USES_TERM`

**Design**: `07-brainstorm-novanet-write.md` | **Code**: `tools/novanet-mcp/src/tools/write.rs`

---

## Plan d'Implementation

### Phase 1: novanet_write (autre terminal)
- 3 operations: UpsertNode, CreateArc, UpdateProps
- Validation schema via introspect
- Permissions par trait (authored/imported/generated = writable)
- ~1600 lignes, 4 phases

### Phase 2: Configuration DataForSEO ✅ FAIT
- Ajoute dans `~/.spn/mcp.yaml` ✅
- Credentials dans `~/.spn/env` ✅
- Test API reussi (135K volume pour "qr code" en France) ✅

### Phase 3: Workflow Nika Complet
```yaml
tasks:
  - invoke: novanet_describe     # Load Entity
  - invoke: novanet_traverse     # Get Slugification rules
  - invoke: dataforseo.search_volume  # Get REAL keywords
  - infer: Score and select      # Formula
  - invoke: novanet_write        # Write SEOKeyword, EntityNative, Term
```

### Phase 4: Test fr-FR
- Entity: qr-code
- Locale: fr-FR
- Valider que "QR code" gagne (pas "code QR")

---

## Formule de Scoring

```
score = volume x sem_coef x intent_boost x trend_factor

sem_coef (relation semantique):
  same_as:      1.00  ("qr code" = Entity)
  action_for:   0.95  ("creer qr code")
  produces:     0.85  ("generateur qr")
  subtopic_of:  0.70  ("qr code wifi")
  related_to:   0.50  ("flashcode")
  attribute_of: 0.30  ("qr code couleur")

intent_boost:
  transactional:  1.20
  commercial:     1.10
  informational:  1.00
  navigational:   0.80

trend_factor:
  rising:   1.15
  stable:   1.00
  declining:0.85
```

---

## Fichiers Cles

| Fichier | But |
|---------|-----|
| `07-brainstorm-novanet-write.md` | Design complet novanet_write |
| `07-deep-architecture.md` | Architecture NovaNet profonde |
| `05-workflow-plan.md` | Workflow Nika (version draft) |
| `tools/novanet-mcp/` | Code MCP a modifier |
| `~/.spn/mcp.yaml` | Config MCP globale |

---

## Echelle Cible

- 2000-3000 Entities
- 200 Locales
- 10 keywords par Entity x Locale
- = 4M+ SEOKeyword nodes
- = 400K+ EntityNative nodes

**Strategie**: Batch par locale, keyword convergence (1 keyword → N entities)

---

## Prochaines Actions

1. [x] **Valider decisions** — Voir `08-clarifications-et-decisions.md`
2. [x] **novanet_write** — Implémenté ! 412 tests passent ✅
3. [x] **DataForSEO MCP** — Configuration dans ~/.spn/mcp.yaml ✅
4. [x] **Workflows créés** — 5 workflows progressifs dans `workflows/` ✅
   - 01-test-novanet-write.nika.yaml
   - 02-seo-discovery-single.nika.yaml
   - 03-seo-discovery-multi-locale.nika.yaml
   - 04-seo-discovery-with-terms.nika.yaml
   - 05-seo-discovery-full.nika.yaml
5. [ ] **Test fr-FR** — Exécuter workflow 01, puis 02
6. [ ] **Scale** — Exécuter workflow 03 (5 locales) puis 05 (200 locales)

---

## Points Critiques - TOUS VALIDÉS ✅

| Point | Decision Finale | Statut |
|-------|-----------------|--------|
| **Slug immutable** | BlockNative:head-seo-meta owns slug + slug_locked flag | ✅ VALIDÉ |
| **Nika cree Entity** | Non pour v1, future avec trait `discovered` | ✅ VALIDÉ |
| **Keywords negatifs** | is_negative flag + negative_reason + preferred_alternative | ✅ VALIDÉ |
| **Workflow intelligent** | Multi-agents + debate + for_each + include | ✅ VALIDÉ |
| **Creation Terms** | Oui via USES_TERM avec purpose/temperature | ✅ VALIDÉ |
| **Schema validation** | Health check au startup | ✅ VALIDÉ |
| **is_slug_source** | Takeover pattern (demote old, not reject) | ✅ VALIDÉ |

**Details**: Voir `08-clarifications-et-decisions.md`

---

## Plan d'Implementation COMPLET

**Fichier**: `tools/novanet-mcp/docs/plans/2026-03-03-novanet-write.md`

| Phase | Tâches | Lignes |
|-------|--------|--------|
| Phase 1 | Error types + hints | 2 tasks |
| Phase 2 | Schema cache | 3 tasks |
| Phase 3 | Write tool core | 7 tasks |
| Phase 4 | Special validations | 2 tasks |
| Phase 5 | Handler integration | 2 tasks |
| Phase 6 | Docs + tests | 3 tasks |
| **Total** | **19 tasks** | ~1600 lines |

**Execution**: Use `superpowers:executing-plans` or `superpowers:subagent-driven-development`
