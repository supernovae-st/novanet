# NovaNet Terminology (v8.1.0)

## Core Concepts

| Term | Definition |
|------|------------|
| **Project** | Business entity with brand identity, audiences, and content structure |
| **Concept** | Abstract semantic meaning (business concept) that content references |
| **Page** | Content page template containing blocks |
| **Block** | Section/component within a page, typed by BlockType |
| **Locale** | Language/region code (BCP 47, e.g., "fr-FR", "en-US") |
| **Expression** | Specific phrase/word variant in a locale's lexicon |

## Node Scopes (3 scopes, 35 nodes)

| Scope | Count | Node Types |
|-------|-------|------------|
| **🌍 Global** | 15 | Locale, LocaleIdentity, LocaleVoice, LocaleCulture, LocaleCultureReferences, LocaleMarket, LocaleLexicon, LocaleRulesAdaptation, LocaleRulesFormatting, LocaleRulesSlug, Expression, Reference, Metaphor, Constraint, Pattern |
| **📦 Project** | 14 | Project, BrandIdentity, ProjectL10n, Page, Block, BlockType, PageType, Concept, ConceptL10n, PagePrompt, BlockPrompt, BlockRules, PageL10n, BlockL10n |
| **🎯 Shared** | 6 | SEOKeywordL10n, SEOKeywordMetrics, SEOMiningRun, GEOSeedL10n, GEOSeedMetrics, GEOMiningRun |

## Localization Pattern

| Invariant Node | L10n Node | Relation |
|----------------|-----------|----------|
| Concept | ConceptL10n | `HAS_L10N` |
| Project | ProjectL10n | `HAS_L10N` |
| Audience | AudienceL10n | `HAS_L10N` |

## Locale Knowledge Structure (14 nodes)

| Node | Purpose |
|------|---------|
| **LocaleIdentity** | Script, timezone, technical characteristics |
| **LocaleVoice** | Formality, tone, pronunciation rules |
| **LocaleCulture** | Cultural norms, taboos |
| **LocaleCultureReferences** | Cultural references container |
| **Reference** | Specific cultural references |
| **Metaphor** | Cultural metaphors |
| **Constraint** | Cultural constraints |
| **LocaleMarket** | Market data, demographics, platforms |
| **LocaleLexicon** | Vocabulary preferences per locale |
| **Expression** | Specific phrase variants |
| **LocaleRulesAdaptation** | Content adaptation rules |
| **LocaleRulesFormatting** | Format rules (dates, numbers) |
| **LocaleRulesSlug** | URL slug generation rules |
| **Pattern** | Reusable formatting patterns |

## Standard Properties (all nodes)

| Property | Type | Description |
|----------|------|-------------|
| `key` | string | Unique identifier with semantic prefix |
| `display_name` | string | Human-readable name |
| `llm_context` | string | AI hints: "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]." |
| `priority` | enum | 'critical' \| 'high' \| 'medium' \| 'low' |
| `freshness` | enum | 'realtime' \| 'hourly' \| 'daily' \| 'static' |
| `created_at` | date | Creation timestamp |
| `updated_at` | date | Last update timestamp |

## Key Relationships

| Relation | From → To | Description |
|----------|-----------|-------------|
| `HAS_CONCEPT` | Project → Concept | Project owns concepts |
| `HAS_PAGE` | Project → Page | Project owns pages |
| `SUPPORTS_LOCALE` | Project → Locale | Available locales (with `default` flag) |
| `HAS_L10N` | Invariant → L10n | Unified localization relation |
| `HAS_BLOCK` | Page → Block | Page structure (with `position`) |
| `OF_TYPE` | Block → BlockType | Block template type |
| `USES_CONCEPT` | Page/Block → Concept | Content references concept |
| `HAS_OUTPUT` | Page/Block → Output | Generated content |
| `HAS_PROMPT` | Page/Block → Prompt | AI generation instructions |
| `TARGETS_SEO` | Concept → SEOKeywordL10n | SEO targeting |
| `TARGETS_GEO` | Concept → GEOSeedL10n | GEO targeting |
| `HAS_SEO_TARGET` | ConceptL10n → SEOKeywordL10n | Locale-aligned SEO |
| `HAS_GEO_TARGET` | ConceptL10n → GEOSeedL10n | Locale-aligned GEO |
| `FOR_LOCALE` | L10n → Locale | Locale assignment |

## Abbreviations

- **L10n** - Localization (localized variant of invariant node)
- **SEO** - Search Engine Optimization
- **GEO** - Generative Engine Optimization (ChatGPT, Perplexity)
- **LLM** - Large Language Model (AI context)
