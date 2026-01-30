# NovaNet Terminology (v7.2.3)

## Core Concepts

| Term | Definition |
|------|------------|
| **Project** | Business entity with brand identity, audiences, and content structure |
| **Concept** | Abstract semantic meaning (business concept) that content references |
| **Page** | Content page template containing blocks |
| **Block** | Section/component within a page, typed by BlockType |
| **Locale** | Language/region code (BCP 47, e.g., "fr-FR", "en-US") |
| **Expression** | Specific phrase/word variant in a locale's lexicon |

## Node Categories (7)

| Category | Purpose | Node Types |
|----------|---------|------------|
| **project** | Business definition | Project, BrandIdentity, Audience, ProjectL10n, AudienceL10n, ValuePropL10n, SocialProofL10n |
| **content** | Semantic structure | Concept, ConceptL10n, Page, Block, BlockType |
| **locale** | Language knowledge | Locale, LocaleIdentity, LocaleVoice, LocaleCulture, LocaleMarket, LocaleLexicon, Expression |
| **generation** | AI prompts & outputs | PagePrompt, BlockPrompt, BlockRules, PageOutput, BlockOutput |
| **seo** | Search optimization | SEOKeyword, SEOVariation, SEOSnapshot, SEOMiningRun |
| **geo** | Generative engine optimization | GEOSeed, GEOReformulation, GEOCitation, GEOMiningRun |
| **analytics** | Performance metrics | PageMetrics |

## Localization Pattern

| Invariant Node | L10n Node | Relation |
|----------------|-----------|----------|
| Concept | ConceptL10n | `HAS_L10N` |
| Project | ProjectL10n | `HAS_L10N` |
| Audience | AudienceL10n | `HAS_L10N` |

## Locale Knowledge Structure

| Node | Purpose |
|------|---------|
| **LocaleIdentity** | Script, timezone, technical characteristics |
| **LocaleVoice** | Formality, tone, pronunciation rules |
| **LocaleCulture** | Cultural norms, taboos, references |
| **LocaleMarket** | Market data, demographics, platforms |
| **LocaleLexicon** | Vocabulary preferences per locale |

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
| `TARGETS_SEO` | Concept → SEOKeyword | SEO targeting |
| `TARGETS_GEO` | Concept → GEOSeed | GEO targeting |

## Abbreviations

- **L10n** - Localization (localized variant of invariant node)
- **SEO** - Search Engine Optimization
- **GEO** - Generative Engine Optimization (ChatGPT, Perplexity)
- **LLM** - Large Language Model (AI context)
