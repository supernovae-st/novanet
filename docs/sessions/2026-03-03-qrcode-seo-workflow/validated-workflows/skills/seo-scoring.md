# SEO Keyword Scoring Skill

You are an expert SEO analyst. Use this skill to score and validate keywords.

## Scoring Formula

```
score = volume * sem_coef * intent_boost * trend_factor
```

### Semantic Coefficients (sem_coef)
| Relation | Coefficient | Description |
|----------|-------------|-------------|
| same_as | 1.00 | Exact synonym or equivalent term |
| action_for | 0.95 | Action verb + entity ("create qr code") |
| produces | 0.85 | Output of action ("qr code image") |
| subtopic_of | 0.70 | Narrower concept ("wifi qr code") |
| related_to | 0.50 | Associated but not direct ("barcode") |
| attribute_of | 0.30 | Property or feature ("dynamic qr code") |

### Intent Boost
| Intent | Boost | Example |
|--------|-------|---------|
| transactional | 1.20 | "buy", "download", "create" |
| commercial | 1.10 | "best", "review", "compare" |
| informational | 1.00 | "what is", "how to" |
| navigational | 0.80 | "brand name", "login" |

### Trend Factor
| Trend | Factor |
|-------|--------|
| rising | 1.20 |
| stable | 1.00 |
| declining | 0.70 |

## Classification Rules

### Primary Keyword (is_primary: true)
- score > 1000
- High search volume (>500 monthly)
- Direct relevance to entity

### Slug Source (is_slug_source: true)
- Best candidate for URL
- Short, descriptive
- No special characters
- Locale-appropriate

## Output Format

Return scored keywords as JSON array:
```json
[
  {
    "value": "keyword text",
    "slug_form": "keyword-slug",
    "volume": 14000,
    "difficulty": 31,
    "intent": "transactional",
    "semantic_relation": "action_for",
    "score": 15960,
    "is_primary": true,
    "is_slug_source": true
  }
]
```

## Validation Checklist
- [ ] Volume > 0
- [ ] Difficulty between 0-100
- [ ] Intent is valid enum
- [ ] Semantic relation is valid
- [ ] Slug form is URL-safe
- [ ] No duplicate keywords
