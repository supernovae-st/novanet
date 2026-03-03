# Validated SEO Discovery Workflows

**Date:** 2026-03-03
**Status:** VALIDATED (tested against Neo4j)

## Overview

5 Nika workflows for SEO keyword discovery with progressive complexity:

| Workflow | Complexity | MCP Tools | Pattern |
|----------|------------|-----------|---------|
| 01 | Test | novanet_write | Basic upsert + arc creation |
| 02 | Single | novanet + dataforseo | 1 Entity + 1 Locale |
| 03 | Multi-Locale | novanet + dataforseo | 1 Entity + 5 Locales (for_each) |
| 04 | With Terms | novanet + dataforseo + perplexity | Extracts Terms from keywords |
| 05 | Production | novanet + dataforseo + perplexity + firecrawl | 200 locales, multi-agent |

## Key Patterns (Validated)

### Entity Key Patterns

```yaml
# EntityNative key pattern (CORRECT)
entity:{entity_key}@{locale}
# Example: entity:qr-code@fr-FR

# SEOKeyword key pattern
seo:{slug}@{locale}
# Example: seo:qr-code@fr-FR

# Term key pattern
{domain}:{value}
# Example: technical:qr-code
```

### Arc Direction

```yaml
# TARGETS: EntityNative -> SEOKeyword (CORRECT)
from_key: "entity:qr-code@fr-FR"  # EntityNative
to_key: "seo:qr-code@fr-FR"       # SEOKeyword

# USES_TERM: EntityNative -> Term
from_key: "entity:qr-code@fr-FR"  # EntityNative
to_key: "technical:qr-code"       # Term
```

### DataForSEO Parameters

```yaml
# CORRECT: Uses location_name (string), NOT location_code (number)
tool: dataforseo_labs_google_keywords_for_site
params:
  target: "qrcode-ai.com"
  location_name: "France"    # String, not 2250
  language_code: "fr"
  limit: 20
```

### novanet_batch Limitations

```yaml
# novanet_batch can ONLY call novanet_* tools
# For external APIs (DataForSEO, Perplexity), use separate for_each tasks
```

## Validation Tests Performed

1. **Neo4j Connection** - Verified database running with seed data
2. **EntityNative Exists** - `entity:qr-code@fr-FR` confirmed
3. **Key Pattern Regex** - Matches schema: `^entity:[a-z][a-z0-9-]*@[a-z]{2}-[A-Z]{2}$`
4. **SEOKeyword Write** - MERGE operation successful
5. **TARGETS Arc Create** - EntityNative -> SEOKeyword with properties
6. **Schema Verification** - TARGETS arc source=EntityNative, target=SEOKeyword

## Files

```
validated-workflows/
├── 01-test-novanet-write.nika.yaml    # Basic test workflow
├── 02-seo-discovery-single.nika.yaml  # Single locale discovery
├── 03-seo-discovery-multi-locale.nika.yaml  # Multi-locale for_each
├── 04-seo-discovery-with-terms.nika.yaml    # Terms extraction
├── 05-seo-discovery-full.nika.yaml    # Production pipeline
└── README.md                          # This file
```

## Usage

```bash
# Run with Nika CLI
cd nika
cargo run -- run validated-workflows/02-seo-discovery-single.nika.yaml

# Or via Nika Studio
cargo run -- studio validated-workflows/02-seo-discovery-single.nika.yaml
```

## Known Issues Fixed

1. **novanet_batch calling DataForSEO** - Restructured to use separate for_each
2. **location_name vs location_code** - Fixed Neo4j query to return `location_name`
3. **Key prefix entity-native:** - Changed to `entity:` per schema
4. **Arc direction reversal** - Confirmed EntityNative -> SEOKeyword is correct

## Next Steps

1. Run workflows through Nika with live DataForSEO credentials
2. Verify generated SEOKeywords in Neo4j Browser
3. Check TARGETS arcs have correct properties
4. Scale to 200 locales with workflow 05
