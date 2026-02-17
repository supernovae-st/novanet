# NovaNet Meta-Graph: Structure Complète

**Date**: 2026-02-16
**Auteur**: Exploration avec Claude Code
**Version**: v0.13.1

## Vue d'ensemble

Le méta-graphe NovaNet est la **structure qui décrit la structure**. C'est le schéma qui définit comment organiser les données réelles.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  MÉTA-GRAPHE vs DONNÉES                                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  MÉTA-GRAPHE (Schema View)          DONNÉES (Graph View)                    │
│  ═══════════════════════             ════════════════════                   │
│                                                                             │
│  Class:Pattern                       Pattern:visual-abstract                │
│  ├── realm: shared                   ├── key: "visual-abstract"             │
│  ├── layer: knowledge                ├── pattern_type: "visual"             │
│  └── trait: imported                 └── description: "..."                 │
│                                                                             │
│  "Qu'est-ce qu'un Pattern?"          "Un Pattern concret"                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Statistiques Globales

**Résultat Query 9:**

| Type | Count | Rôle |
|------|-------|------|
| **Class** | 61 | Définitions de types de nœuds (NodeClass) |
| **ArcClass** | 178 | Définitions de types de relations |
| **Layer** | 9 | Couches fonctionnelles |
| **Trait** | 5 | Origines des données (ADR-024) |
| **ArcFamily** | 5 | Familles sémantiques d'arcs |
| **Realm** | 2 | Portée (shared/org) |

**Total méta-nœuds**: 260 nœuds qui décrivent le schéma.

---

## 1. Les Classes (NodeClass)

### Distribution par Realm et Layer

**Résultat Query 1:**

```
SHARED (40 nodes, READ-ONLY)
├── config       3 nodes    EntityCategory, Locale, SEOKeywordFormat
├── locale       6 nodes    Adaptation, Culture, Formatting...
├── geography    7 nodes    Continent, Country, GeoRegion...
└── knowledge   24 nodes    Term, Expression, Pattern, SEOKeyword...

ORG (21 nodes)
├── config       1 node     OrgConfig
├── foundation   6 nodes    Project, Brand, BrandDesign...
├── structure    3 nodes    Page, Block, ContentSlot
├── semantic     4 nodes    Entity, EntityNative, AudiencePersona...
├── instruction  4 nodes    PageInstruction, BlockInstruction...
└── output       3 nodes    PageNative, BlockNative, OutputArtifact

TOTAL: 61 Classes
```

### Distribution par Trait (Data Origin)

**Résultat Query 2:**

| Trait | Count | Qui crée? | Exemples |
|-------|-------|-----------|----------|
| **defined** | 33 | Humain, UNE FOIS | Page, Block, Entity, Locale |
| **imported** | 20 | Données externes | Term, SEOKeyword, GEOQuery |
| **generated** | 4 | Notre LLM | PageNative, BlockNative |
| **authored** | 2 | Humain, PAR locale | EntityNative, ProjectNative |
| **retrieved** | 2 | APIs externes | GEOAnswer, SEOKeywordMetrics |

**Référence**: ADR-024 (Trait = Data Origin)

---

## 2. Les Axes de Classification

### Realms (WHERE?)

**Résultat Query 3:**

| Key | Name | Rôle |
|-----|------|------|
| **shared** | Shared | Universel, READ-ONLY, même pour tous les orgs |
| **org** | Organization | Spécifique à l'organisation, données business |

**Référence**: ADR-012 (2-Realm Architecture), ADR-018 (global→shared, tenant→org)

### Layers (WHAT?)

**Résultat Query 4:**

9 Layers totales (distribution selon realm dans Query 1):

**SHARED (4 layers):**
- `config` — Configuration universelle (EntityCategory, Locale)
- `locale` — Paramètres de locale (Culture, Style)
- `geography` — Données géographiques (Continent, Country)
- `knowledge` — Atomes de connaissance (Term, Expression, Pattern, SEO, GEO)

**ORG (6 layers, mais Query 4 montre 5 car instruction apparaît aussi):**
- `config` — Configuration org (OrgConfig)
- `foundation` — Base du projet (Project, Brand)
- `structure` — Structure des pages (Page, Block)
- `semantic` — Sémantique métier (Entity, EntityNative)
- `instruction` — Instructions LLM (PageInstruction, BlockInstruction)
- `output` — Sortie générée (PageNative, BlockNative)

**Note**: Layer 9 = "instruction" qui apparaît dans les deux realms selon Query 1, mais Query 4 ne montre pas de relation explicite [:IN_REALM] pour les Layers. Les Layers sont des catégories "flottantes" et c'est la Class qui porte l'information `realm` + `layer`.

### Traits (WHERE does data come from?)

**Résultat Query 5:**

| Key | Name | Signification (ADR-024) |
|-----|------|-------------------------|
| **defined** | Defined | Défini par humain, créé UNE FOIS (structure/template) |
| **authored** | Authored | Écrit par humain, PAR locale (contenu éditorial) |
| **imported** | Imported | Données importées d'APIs externes ou bases |
| **generated** | Generated | Produit par NOTRE LLM (NovaNet génère ceci) |
| **retrieved** | Retrieved | Récupéré d'APIs EXTERNES (snapshots tiers) |

**Orthogonalité**:
- **Layer** répond: "QUELLE catégorie fonctionnelle?" (config, structure, semantic...)
- **Trait** répond: "D'OÙ viennent les données?" (defined, imported, generated...)

---

## 3. Les Arcs (Relations)

### ArcFamily (Familles sémantiques)

**Résultat Query 6:**

| Key | Name | Rôle |
|-----|------|------|
| **ownership** | Ownership | Parent→Child (HAS_PAGE, HAS_ENTITY, HAS_BLOCK) |
| **localization** | Localization | Liens locale (FOR_LOCALE, HAS_VOICE) |
| **semantic** | Semantic | Liens de sens (USES_ENTITY, REFERENCES) |
| **generation** | Generation | Pipeline LLM (GENERATED, COMPILED_FROM) |
| **mining** | Mining | SEO/GEO intelligence (TARGETS_KEYWORD, MONITORS_GEO) |

### Distribution des ArcClass par Family

**Résultat Query 7:**

| Family | Count | Exemples (Query 10d) |
|--------|-------|----------------------|
| **ownership** | 79 | HAS_PAGE, HAS_ENTITY, HAS_BLOCK, HAS_NATIVE |
| **semantic** | 61 | USES_ENTITY, REFERENCES, REPRESENTS |
| **localization** | 20 | FOR_LOCALE, HAS_VOICE, HAS_CULTURE |
| **generation** | 12 | GENERATED, COMPILED_FROM, ASSEMBLES, BUNDLES |
| **mining** | 6 | TARGETS_KEYWORD, MONITORS_GEO |

**Total**: 178 ArcClass (vs 169 dans CLAUDE.md — différence de version possiblement)

### Exemples d'ArcClass (Generation Family)

**Résultat Query 10d (10 premiers de generation):**

| Key | Cardinality | Scope | Description |
|-----|-------------|-------|-------------|
| ASSEMBLES | 1:N | intra_realm | PageNative assemble BlockNatives |
| BUNDLES | 1:N | intra_realm | OutputArtifact bundle PageNatives |
| COMPILED_FROM | N:1 | intra_realm | PromptArtifact compilé depuis Instruction |
| DERIVED_SLUG_FROM | N:1 | **cross_realm** | BlockNative dérive slug de SEOKeyword |
| GENERATED | 1:N | intra_realm | Instruction génère Native |
| GENERATED_FROM | N:1 | intra_realm | BlockNative généré depuis BlockType |
| INCLUDES_ENTITY | 1:N | intra_realm | PromptArtifact inclut Entity |
| INCLUDES_STYLE | N:1 | **cross_realm** | Instruction inclut Style |
| INFLUENCED_BY | N:M | intra_realm | BlockNative influencé par EntityNative |
| PREVIOUS_VERSION | 1:1 | intra_realm | Lien de versioning |

**Scope**:
- `intra_realm` — Source et cible dans le même realm
- `cross_realm` — Source (org) → Cible (shared), ex: DERIVED_SLUG_FROM

---

## 4. Relations du Méta-Graphe

### Class et ses axes de classification

**Résultat Query 8 (Pattern example):**

```cypher
(c:Class {key: 'pattern'})
  ├── Propriétés directes:
  │   ├── realm: "shared"
  │   ├── layer: "knowledge"
  │   └── trait: "imported"
  │
  └── Relations:
      ├── [:IN_REALM]->(Realm {key: 'shared'})   ✅ Existe
      ├── [:IN_LAYER]->(Layer {key: 'knowledge'}) ✅ Existe
      └── [:HAS_TRAIT]->(Trait {key: 'imported'}) ❌ N'existe PAS
```

**Observation**: Les Class ont à la fois:
1. Des **propriétés** (`c.realm`, `c.layer`, `c.trait`)
2. Des **relations** vers les nœuds méta (`[:IN_REALM]`, `[:IN_LAYER]`)

Mais `[:HAS_TRAIT]` ne semble pas créé (trait est uniquement une propriété).

### Exemple complet: Pattern Class

**Résultat Query 10:**

```yaml
Class: Pattern
  realm: shared
  layer: knowledge
  trait: imported
  llm_context: |
    Content structure pattern or template belonging to a
    locale's PatternSet.

  # Relations d'arcs (à vérifier si FROM_CLASS/TO_CLASS existent):
  outgoing_arcs: []  # Aucun arc trouvé dans Query 10
  incoming_arcs: []  # Aucun arc trouvé dans Query 10
```

**Note**: Query 10 n'a pas trouvé de relations `[:FROM_CLASS]` ou `[:TO_CLASS]` pour Pattern. Il est possible que ces relations existent pour d'autres Classes ou que la structure ait changé dans v0.13.1.

---

## 5. Workflows: Schema View vs Graph View

### Schema View (ce que tu vois sur le screenshot)

**Requête CLASSES_QUERY:**
```cypher
MATCH (c:Class)
RETURN c.name, c.realm, c.layer, c.trait
```

**Affiche**:
- Nœuds: `Class:Pattern`, `Realm:Shared`, `Layer:Knowledge`, `ArcClass:USES_PATTERN`
- Relations: `[:IN_REALM]`, `[:IN_LAYER]`, `[:FROM_CLASS]`, `[:TO_CLASS]`

**Analogie**: Le plan d'architecte (structure, types, schéma)

### Graph View (données réelles)

**Requête DATA_QUERY:**
```cypher
MATCH (n)
WHERE NOT n:Schema AND NOT n:Class AND NOT n:ArcClass
  AND NOT n:ArcFamily AND NOT n:Realm AND NOT n:Layer
  AND NOT n:Trait
OPTIONAL MATCH (n)-[r]->(m)
WHERE NOT m:Schema AND NOT m:Class AND NOT m:ArcClass
  AND NOT m:ArcFamily AND NOT m:Realm AND NOT m:Layer
  AND NOT m:Trait
RETURN n, r, m
LIMIT 250
```

**Affiche**:
- Nœuds: `Locale:fr-FR`, `EntityNative:qr-code@fr-FR`, `Term:generateur`
- Relations: `[:HAS_NATIVE]`, `[:CONTAINS_TERM]`, `[:FOR_LOCALE]`

**Analogie**: Les vraies maisons construites (instances, données)

---

## 6. Clés de Compréhension

### Pourquoi autant de nœuds méta?

Le méta-graphe permet:

1. **Navigation facile**: Trouver toutes les Classes de `layer: knowledge` = 24 nœuds
2. **Validation**: Vérifier qu'une Class a bien les propriétés requises
3. **Génération**: Les générateurs Rust lisent le méta-graphe pour produire TypeScript/Cypher
4. **Audit**: Compter les arcs par famille, les nodes par trait, etc.

### La double représentation (propriété + relation)

**Class a les deux**:
```cypher
(c:Class {realm: "shared"})  ← Propriété (rapide)
    |
    [:IN_REALM]
    ↓
(r:Realm {key: "shared"})    ← Relation (navigable)
```

**Avantages**:
- **Propriété**: Rapide pour filtrer (`WHERE c.realm = 'shared'`)
- **Relation**: Navigable pour exploration (`MATCH (c)-[:IN_REALM]->(r)`)

### YAML-First Architecture

Le méta-graphe est généré depuis YAML:

```
packages/core/models/
  ├── node-classes/
  │   ├── shared/
  │   │   └── knowledge/
  │   │       └── pattern.yaml   ← Source of truth
  │   └── org/
  └── arc-classes/
      └── generation/
          └── assembles.yaml     ← Source of truth

          ↓ cargo run -- schema generate

tools/novanet/src/
  └── 12 générateurs
      ↓
00-taxonomy.cypher (crée Realm, Layer, Trait nodes)
01-classes.cypher (crée Class nodes + [:IN_REALM], [:IN_LAYER])
02-arcs.cypher (crée ArcClass nodes + [:FROM_CLASS], [:TO_CLASS])
```

---

## 7. Références ADR

| Concept | ADR | Description |
|---------|-----|-------------|
| Realm names | ADR-018 | global→shared, tenant→org |
| 2-Realm architecture | ADR-012 | SHARED (40) + ORG (21) |
| Trait = Data Origin | ADR-024 | defined/authored/imported/generated/retrieved |
| Arc families | ADR-027 | Generation family semantics |
| YAML-first | ADR-003 | YAML = source of truth |
| Query-first | ADR-021 | Cypher query = source of truth for visualization |

---

## 8. Exploration Recommandée

Pour explorer toi-même dans Neo4j Browser (http://localhost:7474):

```cypher
// 1. Voir tous les Realms et leur contenu
MATCH (r:Realm)<-[:IN_REALM]-(c:Class)
RETURN r.key AS realm,
       count(c) AS classes,
       collect(c.label)[0..5] AS examples

// 2. Voir toutes les Layers et leur contenu
MATCH (l:Layer)<-[:IN_LAYER]-(c:Class)
RETURN l.key AS layer,
       count(c) AS classes,
       collect(c.label)[0..5] AS examples

// 3. Voir la distribution Trait
MATCH (c:Class)
RETURN c.trait AS trait,
       count(*) AS count
ORDER BY count DESC

// 4. Explorer les ArcClass d'une famille
MATCH (ac:ArcClass {family: 'generation'})
RETURN ac.key AS arc,
       ac.cardinality AS card,
       ac.scope AS scope
ORDER BY ac.key

// 5. Voir le méta-graphe complet (warning: gros!)
MATCH (n)
WHERE n:Class OR n:Realm OR n:Layer OR n:ArcClass OR n:ArcFamily
OPTIONAL MATCH (n)-[r]->(m)
WHERE m:Class OR m:Realm OR m:Layer OR m:ArcClass OR m:ArcFamily
RETURN n, r, m
LIMIT 500
```

---

## Conclusion

Le méta-graphe NovaNet est:

1. **Un schéma navigable** — pas juste des labels, mais des nœuds explorables
2. **Multi-axes** — Realm (WHERE?), Layer (WHAT?), Trait (FROM WHERE?)
3. **Généré depuis YAML** — ADR-003 YAML-first architecture
4. **Query-first** — ADR-021, le Cypher définit ce qu'on voit
5. **260 nœuds méta** — 61 Classes + 178 ArcClass + 21 axes

**C'est comme avoir le plan architectural en 3D dans la même base que les maisons construites.**
