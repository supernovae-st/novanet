# Invariants vs Localized Nodes

> **TL;DR:** Les invariants définissent QUOI générer. Les localisés contiennent CE QUI A ÉTÉ généré nativement pour chaque marché.

## Principe fondamental

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  🔷 INVARIANT = Ce qui NE CHANGE PAS entre les locales                          │
│  🟢 LOCALISÉ  = Ce qui EST GÉNÉRÉ nativement par locale                         │
╰─────────────────────────────────────────────────────────────────────────────────╯
```

NovaNet est un système de **génération native**, PAS de traduction.

## Exemple : le Concept "QR Code"

```
🔷 INVARIANT (Concept)                    🟢 LOCALISÉ (ConceptL10n)
────────────────────────                  ────────────────────────────────────
key: "qr-code"
llm_context: "2D barcode                  fr-FR:
  readable by smartphone.                   title: "QR Code"
  Invented 1994 by Denso.                   definition: "Code-barres 2D lisible
  Use: payments, links, auth"                 par smartphone..."
                                            benefits: ["Instantané", "Gratuit"]
        │
        │ :HAS_L10N                        ja-JP:
        └──────────────────────────────►    title: "QRコード"
                                            definition: "スマホで読み取れる2Dコード"
                                            benefits: ["即時", "無料"]
```

### Ce que contient l'invariant

- **Identité sémantique** : ce que représente le concept
- **Instructions LLM** (`llm_context`) : comment générer du contenu
- **Relations** : liens avec d'autres concepts (`SEMANTIC_LINK`)

### Ce que contient le localisé

- **Contenu natif** : généré authentiquement pour chaque marché
- **Pas une traduction** : une création culturellement adaptée

## Pourquoi cette séparation ?

### Le problème de la traduction

```
❌ TRADUCTION (ce qu'on NE FAIT PAS)
─────────────────────────────────────
"Create a QR code" → Google Translate → "Créer un code QR"
                                         ↑
                                         Grammaticalement correct
                                         mais personne dit ça en France
```

### L'approche génération native

```
✅ GÉNÉRATION NATIVE (ce qu'on FAIT)
────────────────────────────────────
Concept.llm_context ──► LLM génère en FR ──► "Créez votre QR Code"
                        avec contexte         ↑
                        culturel FR           Naturel, idiomatique
```

Le LLM reçoit le contexte **entièrement dans la langue cible**. Il génère nativement, il ne traduit pas.

## Les invariants du système

| Invariant | Rôle | Relation | Localisé |
|-----------|------|----------|----------|
| **Concept** | Unité sémantique réutilisable | `:HAS_L10N` | ConceptL10n |
| **Page** | Structure de page | `:HAS_OUTPUT` | PageL10n |
| **Block** | Section de page | `:HAS_OUTPUT` | BlockL10n |
| **BlockType** | Template/règles | - | - |
| **Locale** | Identité d'un marché | `:HAS_*` | LocaleKnowledge* |
| **Project** | Config projet | `:HAS_L10N` | ProjectL10n |

## Flow de génération

```
                    INVARIANTS                          LOCALISÉS
                    ──────────                          ─────────

Page (pricing)  ────────────────────────────────►  PageL10n (fr-FR)
    │                                                    │
    ├─ Block (hero)  ───────────────────────────►  BlockL10n (fr-FR)
    │      │                                             │
    │      └─ uses @tier-pro ──► Concept ──► ConceptL10n (fr-FR)
    │                                │
    │                                └─ SEMANTIC_LINK ──► Concept (qr-code)
    │                                                          │
    │                                                    ConceptL10n (fr-FR)
    │
    └─ Locale (fr-FR) ──► LocaleVoice (formality, tone)
                      └─► LocaleLexicon (vocabulary)
```

## Avantages de cette architecture

### 1. Scalabilité

- Ajouter une locale = générer les `*L10n` et `*Output`
- Les invariants restent inchangés

### 2. Cohérence

- Un Concept a la même signification dans toutes les locales
- Seule l'expression change

### 3. Qualité native

- Chaque marché reçoit du contenu authentique
- Pas de "traductionese"

### 4. Maintenance

- Modifier un invariant impacte toutes les locales
- Modifier un localisé n'impacte qu'une locale

## Nomenclature

| Suffixe | Signification | Exemple |
|---------|---------------|---------|
| `*L10n` | Définitions localisées (stable, curées) | ConceptL10n, ProjectL10n |
| `*Output` | Contenu généré par LLM | PageL10n, BlockL10n |
| `Locale*` | Connaissance d'une locale | LocaleVoice, LocaleCulture |

## Relations clés

```cypher
// Invariant → Localisé (définitions)
(Concept)-[:HAS_L10N]->(ConceptL10n)-[:FOR_LOCALE]->(Locale)

// Invariant → Localisé (généré)
(Page)-[:HAS_OUTPUT]->(PageL10n)-[:FOR_LOCALE]->(Locale)
(Block)-[:HAS_OUTPUT]->(BlockL10n)-[:FOR_LOCALE]->(Locale)

// Locale → Connaissance
(Locale)-[:HAS_VOICE]->(LocaleVoice)
(Locale)-[:HAS_CULTURE]->(LocaleCulture)
(Locale)-[:HAS_LEXICON]->(LocaleLexicon)
```

## Résumé

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                                                                 │
│   INVARIANT                           LOCALISÉ                                  │
│   ─────────                           ────────                                  │
│                                                                                 │
│   "Qu'est-ce qu'on génère ?"          "Qu'est-ce qui a été généré ?"           │
│                                                                                 │
│   - Structure (Page, Block)           - Contenu natif (PageL10n)              │
│   - Sémantique (Concept)              - Définitions locales (ConceptL10n)       │
│   - Règles (BlockType)                - Connaissance locale (LocaleVoice)       │
│                                                                                 │
│   Stable, partagé                     Par locale, authentique                   │
│                                                                                 │
╰─────────────────────────────────────────────────────────────────────────────────╯
```
