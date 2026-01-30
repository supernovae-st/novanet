# NovaNet Config

Configuration files for content generation.

## Files

| File | Purpose |
|------|---------|
| `prompt.md` | LLM prompt template for sub-agent content generation |
| `locales.yaml` | Registry of 200 BCP 47 locale codes |

## locales.yaml

Source: Imported from `ath-know-l10n` knowledge base.

Contains all supported locales organized by:
- **Primary locales** - Major markets with full Locale Knowledge
- **Language families** - Grouped by ISO 639-1 language code
- **Flat list** - All 200 codes for validation

Usage:
```typescript
import locales from '../config/locales.yaml';

// Check if locale is supported
const isValid = locales.all.includes('fr-FR');

// Get primary locales
const primary = locales.primary; // ['en-US', 'fr-FR', 'de-DE', ...]
```

## prompt.md

Template for sub-agent content generation. Uses placeholders:
- `{page_key}` - Page being generated
- `{block_key}` - Block assigned to this agent
- `{locale}` - Target locale (BCP 47)
- `{concepts}` - Relevant ConceptL10n data
- `{locale_knowledge}` - LocaleVoice, LocaleLexicon data

## Environment Variables

Create `.env` from `.env.example`:
```bash
cp .env.example .env
```

Required:
- `NEO4J_URI` - Neo4j connection string
- `NEO4J_USER` - Database user
- `NEO4J_PASSWORD` - Database password
