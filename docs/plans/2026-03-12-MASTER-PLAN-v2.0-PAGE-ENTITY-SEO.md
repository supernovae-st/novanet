# MASTER PLAN v2.0 — Page-Entity-SEO Architecture

> **Version**: 2.2
> **Date**: 2026-03-12
> **Status**: EN COURS - BRAINSTORMING DOCUMENTÉ
> **Auteur**: Claude + Thibaut

---

## Today's Brainstorming Summary (2026-03-12)

### Session Overview

4 Opus 4.5 agents ont analysé l'architecture en parallèle pour vérifier la cohérence du plan.

### Agent 1: Entity → EntityNative Analysis

**Findings validés:**
- Entity (org/semantic, defined) → EntityNative (org/semantic, authored) via HAS_NATIVE
- EntityNative → Locale (shared/config) via FOR_LOCALE
- denomination_forms sur Entity: [text, title, abbrev] — PAS de url (ADR-030)
- denomination_forms sur EntityNative: [text, title, abbrev, url] — url ajouté par SEO pipeline write-back
- Statut workflow: draft → reviewed → published

**Décision:** Ski Slope #1 conforme aux ADRs.

### Agent 2: Page → PageNative Analysis

**Findings validés:**
- Page (org/structure, defined) → Entity via ABOUT arc (remplace REPRESENTS)
- Système de rôles: focus (exactement 1), support (0..N), reference (0..N), compare (0..N)
- Weight property (0.0-1.0) pour spreading activation
- ANCHORS arc pour références spreading activation (Page → Entity)
- Page → PageNative via HAS_NATIVE, slug copié depuis EntityNative.denomination_forms.url

**Décision:** ADR-046 à créer pour documenter ABOUT arc.

### Agent 3: Slugification Pipeline Analysis

**Règles par type de script identifiées:**

| Type | Locales | Règle | Exemple |
|------|---------|-------|---------|
| latin_strip | en-US, es-ES | Supprimer diacritiques | "créer" → "creer" |
| latin_preserve | fr-FR | Garder diacritiques | "créer" → "créer" |
| latin_transform | de-DE | Transformer umlauts | "für" → "fuer" |
| native_script | ja-JP, zh-CN, ko-KR | Romaniser toujours | "作成" → "sakusei" |
| transliterate | ru-RU, uk-UA, el-GR | Translittérer | "код" → "kod" |

**Décision:** URLs toujours en ASCII-safe pour compatibilité universelle.

### Agent 4: Arc Inventory Audit

**Résultat:** 153 arc classes actuellement (après ajout des 5 inverse arcs)

**Arcs manquants par rapport au plan:**

| Arc | Status | Action |
|-----|--------|--------|
| ABOUT | ❌ Manquant | À créer (remplace REPRESENTS) |
| ABOUT_OF | ❌ Manquant | À créer (inverse) |
| COMPETES_WITH | ❌ Manquant | À créer (Entity↔Entity) |
| HAS_PART | ❌ Manquant | À créer (Entity→Entity) |
| PART_OF | ❌ Manquant | À créer (inverse) |
| ENABLES | ❌ Manquant | À créer (Entity→Entity) |
| ENABLED_BY | ❌ Manquant | À créer (inverse) |
| MENTIONS | ❌ Manquant | À créer (Block→Entity) |

**Arcs à déprécier:**

| Arc | Status | Action |
|-----|--------|--------|
| REPRESENTS | ⚠️ Encore actif | Déplacer vers deprecated/ |
| REPRESENTED_BY | ⚠️ Encore actif | Déplacer vers deprecated/ |

### Key Architectural Insights

1. **ABOUT vs ANCHORS clarification:**
   - ABOUT: Relation sémantique "de quoi parle la page" (obligatoire, 1 focus)
   - ANCHORS: Références spreading activation pour context assembly (optionnel)
   - Les deux ont role + weight mais usages différents

2. **Weight property universelle:**
   - Tous les arcs sémantiques doivent avoir weight: 0.0-1.0
   - Utilisé pour spreading activation: `activation = parent × weight × decay^distance`

3. **Validation stricte:**
   - Exactement 1 ABOUT avec role="focus" par Page (constraint Neo4j)
   - Weight doit être dans range [0.0, 1.0]

4. **SEO Pipeline Write-back:**
   - EntityNative.denomination_forms.url est calculé APRÈS Entity bootstrap
   - PageNative.slug copie cette valeur
   - Pas de calcul de slug à la volée

---

## Today's Progress (2026-03-12)

| Item | Status | Details |
|------|--------|---------|
| 5 inverse arcs created | ✅ | about.yaml, anchor-of.yaml, targets-geo.yaml, scope-of.yaml, enrichment-of.yaml |
| Tests updated | ✅ | 148 → 153 arc classes |
| Schema validation | ✅ | 0 errors, 0 warnings |
| Clippy | ✅ | Zero warnings |
| Ski slope diagrams | ✅ | Entity, Page, Slugification documented |

---

## Ski Slope #1: Entity → EntityNative

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ENTITY → ENTITYNATIVE FLOW (ADR-029 *Native Pattern)                         ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Entity (org/semantic - defined)                                              ║
║  ├── key: "entity:qr-code"                                                    ║
║  ├── display_name: "QR Code"                                                  ║
║  ├── denomination_forms: [text, title, abbrev] (NO url - ADR-030)             ║
║  └── is_pillar: true                                                          ║
║       │                                                                       ║
║       │ [:HAS_NATIVE] (ownership, one_to_many)                                ║
║       ▼                                                                       ║
║  EntityNative (org/semantic - authored)                                       ║
║  ├── key: "entity:qr-code@fr-FR"                                              ║
║  ├── denomination_forms: [                                                    ║
║  │     {type: "text",   value: "code qr"},                                    ║
║  │     {type: "title",  value: "Code QR"},                                    ║
║  │     {type: "abbrev", value: "qr"},                                         ║
║  │     {type: "url",    value: "code-qr"}  ← POST-SEO pipeline write-back     ║
║  │   ]                                                                        ║
║  ├── semantic_content: {definition, context, cultural_context}                ║
║  └── status: draft | reviewed | published                                     ║
║       │                                                                       ║
║       │ [:FOR_LOCALE] (localization, many_to_one)                             ║
║       ▼                                                                       ║
║  Locale (shared/config)                                                       ║
║  ├── key: "fr-FR"                                                             ║
║  ├── script: latin | cyrillic | cjk | arabic                                  ║
║  └── region: europe | asia | americas                                         ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Ski Slope #2: Page → PageNative

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  PAGE → PAGENATIVE FLOW (ADR-046 ABOUT Arc)                                   ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Page (org/structure - defined)                                               ║
║  ├── key: "page:qr-code-generator"                                            ║
║  ├── display_name: "QR Code Generator"                                        ║
║  └── is_pillar: true                                                          ║
║       │                                                                       ║
║       ├──[:ABOUT {role:"focus", weight:0.9}]──→ Entity (EXACTLY 1)            ║
║       ├──[:ABOUT {role:"support", weight:0.6}]──→ Entity (0..N)               ║
║       ├──[:ABOUT {role:"reference", weight:0.3}]──→ Entity (0..N)             ║
║       ├──[:ABOUT {role:"compare", weight:0.5}]──→ Entity (0..N)               ║
║       │                                                                       ║
║       ├──[:ANCHORS {role, weight}]──→ Entity (spreading activation refs)      ║
║       │                                                                       ║
║       ├──[:HAS_NATIVE]──→ PageNative (org/output - generated)                 ║
║       │                    ├── key: "page:qr-code-generator@fr-FR"            ║
║       │                    ├── slug: "code-qr" ← from EntityNative.url        ║
║       │                    ├── meta_title: "Créer un Code QR"                 ║
║       │                    └── status: draft | published                      ║
║       │                                                                       ║
║       └──[:HAS_BLOCK {order: 1}]──→ Block                                     ║
║                                     ├── key: "qr-code-generator:hero:1"       ║
║                                     └──[:HAS_NATIVE]──→ BlockNative           ║
║                                                                               ║
║  VALIDATION: Exactly 1 ABOUT arc with role="focus" per Page                   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Ski Slope #3: Slugification Pipeline (ADR-030 + ADR-032 + ADR-033)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  SLUGIFICATION PIPELINE                                                       ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Entity.key = "entity:qr-code"                                                ║
║       │                                                                       ║
║       │ [:HAS_NATIVE]                                                         ║
║       ▼                                                                       ║
║  EntityNative (per locale)                                                    ║
║       │                                                                       ║
║       ├── en-US: denomination_forms.url = "qr-code-generator"                 ║
║       │          (latin_strip: remove diacritics)                             ║
║       │                                                                       ║
║       ├── fr-FR: denomination_forms.url = "créer-qr-code"                     ║
║       │          (latin_preserve: keep diacritics)                            ║
║       │                                                                       ║
║       ├── de-DE: denomination_forms.url = "qr-code-erstellen"                 ║
║       │          (latin_transform: ü→ue, ö→oe)                                ║
║       │                                                                       ║
║       ├── ja-JP: denomination_forms.url = "qr-code-sakusei"                   ║
║       │          (native_script: ALWAYS romanized ASCII for URL)              ║
║       │                                                                       ║
║       └── ru-RU: denomination_forms.url = "qr-kod"                            ║
║                  (transliterate: Cyrillic → Latin)                            ║
║       │                                                                       ║
║       │ SEO Pipeline Write-Back                                               ║
║       ▼                                                                       ║
║  PageNative.slug = EntityNative.denomination_forms.url                        ║
║       │                                                                       ║
║       ▼                                                                       ║
║  URL: /{locale}/{slug} → /fr-FR/créer-qr-code                                 ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Executive Summary

Refonte complète de l'architecture Page-Entity-SEO pour NovaNet avec:
- Remplacement de REPRESENTS par ABOUT (flexible N:M avec validation 1 focus)
- Anchor = Entity reference (pas string)
- Weight sur tous les arcs sémantiques pour spreading activation
- Nouveaux arcs Entity↔Entity (COMPETES_WITH, HAS_PART, ENABLES)
- Seeds propres dès le départ (PAS de migrations)

---

## Décisions Validées

| ID | Décision | Rationale |
|----|----------|-----------|
| D1 | Supprimer REPRESENTS, utiliser ABOUT | Plus flexible, 1:1 garanti par validation |
| D2 | ABOUT avec {role, weight} | Roles: focus/support/compare/reference |
| D3 | Validation: exactement 1 role="focus" par Page | Préserve principe 1:1 via règle |
| D4 | anchor_entity + anchor_form (pas string) | Multilingue auto via EntityNative |
| D5 | Weight sur tous arcs Entity↔Entity | Spreading activation uniforme |
| D6 | Nouveaux arcs: COMPETES_WITH, HAS_PART, ENABLES | Relations sémantiques riches |
| D7 | Seeds propres, PAS de migrations | Projet clean au démarrage |
| D8 | Supprimer tout legacy/pollution | Pas de code mort |

---

## Architecture Cible

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ARCHITECTURE v2.0                                                            ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  PAGE → ENTITY                                                                ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  [:ABOUT {role: "focus|support|compare|reference", weight: 0.0-1.0}]          ║
║  • Cardinality: N:M                                                           ║
║  • Validation: exactement 1 role="focus" par Page                             ║
║  • Inverse: ABOUT_OF                                                          ║
║                                                                               ║
║  BLOCK → PAGE (liens SEO)                                                     ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  [:LINKS_TO {                                                                 ║
║     anchor_entity: "entity:xxx",     ← Entity reference                       ║
║     anchor_form: "text|title|abbrev", ← denomination_forms key                ║
║     fragment: "#section",             ← anchor dans la page                   ║
║     context: "cta|body|related|nav",                                          ║
║     seo_weight: 0.0-1.0,                                                      ║
║     nofollow: false                                                           ║
║  }]                                                                           ║
║                                                                               ║
║  BLOCK → ENTITY (mentions)                                                    ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  [:MENTIONS {weight: 0.0-1.0}]                                                ║
║                                                                               ║
║  ENTITY → ENTITY (relations sémantiques)                                      ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  [:SIMILAR_TO {weight, description}]      bidirectionnel                      ║
║  [:RELATED_TO {weight, description}]      bidirectionnel                      ║
║  [:COMPETES_WITH {weight}]                bidirectionnel                      ║
║  [:HAS_PART {weight}]                     directionnel → PART_OF              ║
║  [:ENABLES {weight}]                      directionnel → ENABLED_BY           ║
║  [:HAS_CHILD {weight, position}]          directionnel → CHILD_OF             ║
║  [:HAS_FEATURE {weight, priority}]        directionnel → FEATURE_OF           ║
║  [:SEMANTIC_LINK {temperature, type}]     self-referential (déjà OK)          ║
║                                                                               ║
║  SPREADING ACTIVATION                                                         ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  activation(node) = parent_activation × arc.weight × decay^distance           ║
║  Tous les arcs lisent weight depuis Neo4j (pas hardcodé)                      ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Fichiers à SUPPRIMER (Legacy/Pollution)

### Arc YAML (Archiver → deprecated/)

| Fichier | Raison |
|---------|--------|
| `arc-classes/semantic/represents.yaml` | Remplacé par ABOUT |
| `arc-classes/semantic/represented-by.yaml` | Remplacé par ABOUT_OF |
| `packages/db/archive/2026-03-10/053-invert-represents-direction.cypher` | Migration obsolète |

### Documentation Legacy (Vérifier pertinence)

| Fichier | Action |
|---------|--------|
| `docs/plans/2026-03-07-entity-page-architecture-cleanup.md` | Review, possiblement supprimer |
| Autres docs mentionnant REPRESENTS | Mettre à jour ou archiver |

---

## Fichiers à CRÉER (Clean)

### Arc YAML — Nouveaux Arcs

| Fichier | Description |
|---------|-------------|
| `arc-classes/semantic/about.yaml` | Page ABOUT Entity (N:M, role+weight) |
| `arc-classes/semantic/about-of.yaml` | Inverse de ABOUT |
| `arc-classes/semantic/competes-with.yaml` | Entity COMPETES_WITH Entity |
| `arc-classes/ownership/has-part.yaml` | Entity HAS_PART Entity |
| `arc-classes/ownership/part-of.yaml` | Inverse de HAS_PART |
| `arc-classes/semantic/enables.yaml` | Entity ENABLES Entity |
| `arc-classes/semantic/enabled-by.yaml` | Inverse de ENABLES |
| `arc-classes/semantic/mentions.yaml` | Block MENTIONS Entity (si n'existe pas) |

### Documentation

| Fichier | Description |
|---------|-------------|
| `dx/adr/novanet/arc-design/adr-046-about-arc.md` | ADR pour nouvelle architecture |

---

## Fichiers à MODIFIER

### Arc YAML — Ajouter Weight

| Fichier | Modification |
|---------|--------------|
| `arc-classes/ownership/has-child.yaml` | +weight property |
| `arc-classes/ownership/child-of.yaml` | +weight property |
| `arc-classes/semantic/has-feature.yaml` | +weight property |
| `arc-classes/semantic/feature-of.yaml` | +weight property |
| `arc-classes/semantic/similar-to.yaml` | +weight property (vérifier si existe) |
| `arc-classes/semantic/related-to.yaml` | +weight property (vérifier si existe) |

### Cypher Seeds (PROPRES, pas migrations)

| Fichier | Modification |
|---------|--------------|
| `seed/content/40-page-block-instances.cypher` | [:REPRESENTS] → [:ABOUT {role:"focus", weight:0.9}] |
| `seed/content/48-page-block-qr-code.cypher` | [:REPRESENTS] → [:ABOUT {role:"focus", weight:0.9}] |
| `seed/02-arc-classes.cypher` | Régénéré via `cargo run -- schema generate` |

### TypeScript

| Fichier | Modification |
|---------|--------------|
| `packages/core/src/filters/CypherGenerator.ts` | REPRESENTS → ABOUT |
| `packages/core/src/schemas/relations.schema.ts` | Nouveaux arcs, LINKS_TO enrichi |
| `packages/core/src/filters/NovaNetFilter.ts` | Méthodes pour ABOUT |
| `apps/studio/src/config/relationshipTypes.ts` | Couleurs nouveaux arcs |

### Rust

| Fichier | Modification |
|---------|--------------|
| `tools/novanet/src/generators/arc_class.rs` | Commentaires ADR |
| `tools/novanet/src/blueprint/validation.rs` | REPRESENTS → ABOUT validation |
| `tools/novanet/src/tui/data.rs` | Help text TUI |
| `tools/novanet-mcp/src/tools/assemble.rs` | Lire weight depuis Neo4j |

### Documentation

| Fichier | Modification |
|---------|--------------|
| `dx/adr/novanet/schema-architecture/adr-028-page-entity.md` | Note "Superseded by ADR-046" |
| `novanet/CLAUDE.md` | Mettre à jour arc counts |
| `.claude/rules/adr-quick-reference.md` | Ajouter ADR-046 |

---

## Phases d'Exécution avec Checkpoints

### PHASE 1: Schema YAML (Source of Truth)

| Step | Action | Vérification |
|------|--------|--------------|
| 1.1 | Créer `deprecated/` folder | `ls packages/core/models/arc-classes/deprecated/` |
| 1.2 | Déplacer represents.yaml, represented-by.yaml | Fichiers dans deprecated/ |
| 1.3 | Créer about.yaml | YAML valide, properties correctes |
| 1.4 | Créer about-of.yaml | Inverse correct |
| 1.5 | Créer nouveaux arcs Entity↔Entity | 6 fichiers créés |
| 1.6 | Ajouter weight aux arcs existants | Properties ajoutées |
| 1.7 | `cargo run -- schema validate` | ✅ Zero errors |

**CHECKPOINT 1**: `cargo run -- schema validate` passe

### PHASE 2: Regenerate Cypher

| Step | Action | Vérification |
|------|--------|--------------|
| 2.1 | `cargo run -- schema generate` | Fichiers .cypher générés |
| 2.2 | Vérifier 02-arc-classes.cypher | Pas de REPRESENTS, ABOUT présent |
| 2.3 | Modifier 40-page-block-instances.cypher | [:ABOUT] avec role+weight |
| 2.4 | Modifier 48-page-block-qr-code.cypher | [:ABOUT] avec role+weight |
| 2.5 | Grep "REPRESENTS" dans seed/ | Zero résultats |

**CHECKPOINT 2**: `grep -r "REPRESENTS" packages/db/seed/` = vide

### PHASE 3: TypeScript Updates

| Step | Action | Vérification |
|------|--------|--------------|
| 3.1 | Modifier CypherGenerator.ts | REPRESENTS → ABOUT |
| 3.2 | Modifier relations.schema.ts | Nouveaux arcs, LINKS_TO enrichi |
| 3.3 | Modifier NovaNetFilter.ts | Méthodes ABOUT |
| 3.4 | Modifier relationshipTypes.ts | Couleurs |
| 3.5 | `pnpm type-check` | ✅ Zero errors |
| 3.6 | `pnpm test` | ✅ Tests passent |

**CHECKPOINT 3**: `pnpm type-check && pnpm test` passent

### PHASE 4: Rust Updates

| Step | Action | Vérification |
|------|--------|--------------|
| 4.1 | Modifier validation.rs | ABOUT validation |
| 4.2 | Modifier data.rs | Help text |
| 4.3 | Modifier assemble.rs | Lire weight Neo4j |
| 4.4 | `cargo clippy` | Zero warnings |
| 4.5 | `cargo test` | ✅ Tests passent |

**CHECKPOINT 4**: `cargo clippy && cargo test` passent

### PHASE 5: Database Reset

| Step | Action | Vérification |
|------|--------|--------------|
| 5.1 | `pnpm infra:down` | Neo4j arrêté |
| 5.2 | `pnpm infra:up` | Neo4j démarré |
| 5.3 | `pnpm infra:seed` | Seeds appliqués |
| 5.4 | Cypher: `MATCH ()-[r:REPRESENTS]->() RETURN count(r)` | = 0 |
| 5.5 | Cypher: `MATCH ()-[r:ABOUT]->() RETURN count(r)` | > 0 |
| 5.6 | Cypher: Pages with focus | Toutes ont 1 focus |

**CHECKPOINT 5**: Neo4j propre, ABOUT fonctionne

### PHASE 6: Documentation

| Step | Action | Vérification |
|------|--------|--------------|
| 6.1 | Créer ADR-046 | Fichier existe |
| 6.2 | Modifier ADR-028 | Note superseded |
| 6.3 | Modifier CLAUDE.md | Arc counts à jour |
| 6.4 | Modifier adr-quick-reference.md | ADR-046 listé |

**CHECKPOINT 6**: Documentation cohérente

### PHASE 7: Ralph Wiggum Audit

| Step | Action | Vérification |
|------|--------|--------------|
| 7.1 | Grep "REPRESENTS" tout le projet | Zero (sauf deprecated/, docs historiques) |
| 7.2 | Grep "REPRESENTED_BY" tout le projet | Zero (sauf deprecated/) |
| 7.3 | Vérifier tous les arcs ont weight | Tous sauf exceptions documentées |
| 7.4 | Vérifier LINKS_TO a anchor_entity | Property présente |
| 7.5 | Test E2E: génération page | Fonctionne avec ABOUT |

**CHECKPOINT 7 (FINAL)**: Audit complet passé

---

## Validation Cypher

```cypher
// 1. Vérifier REPRESENTS supprimé
MATCH ()-[r:REPRESENTS]->() RETURN count(r) AS old_represents;
// Expected: 0

// 2. Vérifier ABOUT créé
MATCH ()-[r:ABOUT]->() RETURN count(r) AS about_count;
// Expected: > 0

// 3. Vérifier chaque Page a exactement 1 focus
MATCH (p:Page)
WITH p, size([(p)-[:ABOUT {role: 'focus'}]->() | 1]) AS focus_count
WHERE focus_count != 1
RETURN p.key AS page_without_single_focus;
// Expected: 0 rows

// 4. Vérifier poids dans range
MATCH ()-[r:ABOUT]->()
WHERE r.weight < 0.0 OR r.weight > 1.0
RETURN count(r) AS invalid_weights;
// Expected: 0

// 5. Vérifier nouveaux arcs existent
MATCH (ac:Schema:ArcClass)
WHERE ac.key IN ['ABOUT', 'ABOUT_OF', 'COMPETES_WITH', 'HAS_PART', 'PART_OF', 'ENABLES', 'ENABLED_BY']
RETURN ac.key AS new_arc;
// Expected: 7 rows
```

---

## Rollback Plan

Si problème critique:
1. Git: `git checkout HEAD~N` pour revenir
2. Seeds: `pnpm infra:reset` avec anciens seeds
3. Les fichiers deprecated/ permettent de restaurer REPRESENTS si nécessaire

---

## Estimation

| Phase | Durée estimée |
|-------|---------------|
| Phase 1: Schema YAML | ~30 min |
| Phase 2: Cypher Seeds | ~15 min |
| Phase 3: TypeScript | ~30 min |
| Phase 4: Rust | ~45 min |
| Phase 5: Database Reset | ~10 min |
| Phase 6: Documentation | ~20 min |
| Phase 7: Audit | ~15 min |
| **TOTAL** | ~2h45 |

---

## Prochaines Étapes

1. **CONFIRMATION** de Thibaut sur ce plan
2. Exécution Phase 1
3. Checkpoint 1 validation
4. Continue...

---

## Références

- ADR-028: Page-Entity Architecture (à superseder partiellement)
- ADR-033: Denomination Forms
- ADR-037: Arc Weight Property
- Schema.org: mainEntity, about, mentions
- FOAF: primaryTopic
- GraphRAG: weight-based spreading activation
