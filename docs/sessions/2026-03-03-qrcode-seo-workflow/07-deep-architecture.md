# Architecture Profonde NovaNet

## Le graphe auto-descriptif

NovaNet n'est PAS une simple base de donnees. Le SCHEMA lui-meme vit DANS le graphe.

### Meta-nodes (le schema EST des nodes)

```
Realm ──────────► Node de type Realm (pas un tag!)
├── "shared"
└── "org"

Layer ──────────► Node de type Layer (pas un tag!)
├── config, locale, geography, knowledge (shared)
└── foundation, structure, semantic, instruction, output (org)

Class ──────────► Node de type Class
├── Entity, Page, Block, Locale...
└── Chaque instance a un arc [:OF_CLASS] vers sa Class
```

### Introspection via Cypher

```cypher
-- Le graphe peut se decrire lui-meme!
MATCH (l:Layer {key: "semantic"})-[:HAS_CLASS]->(c:Class)
RETURN c.name, c.trait

-- Voir tous les arcs possibles entre deux classes
MATCH (ac:ArcClass)-[:FROM_CLASS]->(source:Class {name: "Entity"})
MATCH (ac)-[:TO_CLASS]->(target:Class)
RETURN ac.name, target.name
```

## Locale → Slugification (1:1)

Chaque Locale a exactement UNE configuration de slugification.

### Arc: HAS_SLUGIFICATION

```
(Locale:fr-FR)-[:HAS_SLUGIFICATION]->(Slugification)
```

### Proprietes de Slugification

| Propriete | fr-FR | en-US | ar-SA | ja-JP |
|-----------|-------|-------|-------|-------|
| slug_rule | latin_preserve | latin_strip | native_script | transliterate |
| preserve_diacritics | true | false | - | - |
| stop_words | le,la,les,un | a,the,an | - | - |
| text_direction | ltr | ltr | rtl | ltr |

### 4 slug_rules

1. **latin_preserve**: Garde les accents (fr-FR, es-ES) → "créer-un-qr-code"
2. **latin_strip**: Enleve les accents (en-US, de-DE) → "create-qr-code"
3. **native_script**: Garde le script natif (ar-SA, he-IL) → "مولد-qr"
4. **transliterate**: Romanise (ja-JP, ru-RU) → "qr-kodo"

## Knowledge Atoms (6 types)

### Les 6 types d'atoms

| Type | But | Exemple fr-FR |
|------|-----|---------------|
| **Term** | Vocabulaire | "qr code", "code-barres" |
| **Expression** | Idiomes, formules | "Veuillez agreer..." |
| **Pattern** | Templates | "Decouvrez {product} des maintenant" |
| **CultureRef** | References culturelles | "14 juillet", "Asterix" |
| **Taboo** | Mots interdits | "mort" (severity: high) |
| **AudienceTrait** | Segments audience | "b2b_enterprise" |

### Connexion: Locale → Container → Atom

```
Locale:fr-FR
├── [:HAS_TERMS] → TermSet {domain: "technical"}
│   └── [:CONTAINS_TERM] → Term {value: "qr code"}
│
├── [:HAS_EXPRESSIONS] → ExpressionSet
│   └── [:CONTAINS_EXPRESSION] → Expression
│
├── [:HAS_CULTURE] → CultureSet
│   └── [:CONTAINS_CULTURE_REF] → CultureRef
│
└── [:HAS_TABOOS] → TabooSet
    └── [:CONTAINS_TABOO] → Taboo
```

### Selective Loading via USES_*

Le probleme: fr-FR peut avoir 20,000 Terms. On ne veut pas tout charger!

Solution: EntityNative a des arcs directs vers les atoms pertinents.

```
EntityNative:qr-code@fr-FR
├── [:USES_TERM {purpose: "primary", temperature: 0.9}] → Term:qr_code
├── [:USES_TERM {purpose: "secondary", temperature: 0.7}] → Term:code_barres
├── [:USES_EXPRESSION {purpose: "cta"}] → Expression:decouvrez_maintenant
└── [:USES_TABOO] → Taboo:mort
```

- `temperature`: poids d'activation (0.0-1.0)
- `purpose`: primary, secondary, contextual, avoid

## llm_context (self-documenting)

Chaque node a un champ `llm_context` qui explique son usage aux LLMs.

```yaml
# Dans entity.yaml
llm_context: |
  USE: when loading semantic context for content generation.
  TRIGGERS: "entity", "semantic", "concept", "product".
  NOT: for localized content (use EntityNative).
  RELATES: Project, EntityNative, EntityCategory.
```

→ Les LLMs peuvent LIRE ces instructions pour savoir quoi charger!

## MCP Tools pour exploiter tout ca

| Tool | Usage |
|------|-------|
| `novanet_describe` | Bootstrap: charge Entity + metadata |
| `novanet_traverse` | Suit les arcs: Locale → Slugification |
| `novanet_atoms` | Charge atoms selectivement (50 pas 20K) |
| `novanet_generate` | Assemble tout pour generation LLM |
| `novanet_introspect` | Interroge le meta-schema |

## Impact sur le workflow Nika

Le workflow doit:
1. Charger Locale et sa Slugification (regles)
2. Charger les Terms/Expressions via USES_* (contexte)
3. Appliquer les regles de slugification au keyword gagnant
4. Ecrire le resultat avec les bons liens
