# NovaNet Config

Configuration files for the NovaNet system.

## Files

| File | Purpose |
|------|---------|
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
