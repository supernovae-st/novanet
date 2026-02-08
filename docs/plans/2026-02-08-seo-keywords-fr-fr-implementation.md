# SEO Keywords fr-FR Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Import ~40K SEO keywords fr-FR and link them to 281 EntityL10n nodes for QR Code AI.

**Architecture:**
- Create EntityL10n fr-FR nodes from existing Entity definitions (281 entities across 14 phases)
- Filter ~75K keywords to ~40K (volume >= 50, French only, no spam)
- Create SEOKeyword nodes and link via `[:TARGETS]` to EntityL10n
- All data stored as Cypher seed files for reproducibility

**Tech Stack:** Python scripts for CSV processing, Cypher for Neo4j import, YAML for entity sources

---

## Phase 1: Create EntityL10n fr-FR Nodes

### Task 1.1: Create Python script to extract Entity keys

**Files:**
- Create: `scripts/seo-import/extract_entities.py`
- Read: `packages/core/data/entities/qrcode-ai/phase-*.yaml`
- Output: `scripts/seo-import/output/entities.json`

**Step 1: Create directory structure**

```bash
mkdir -p scripts/seo-import/output
```

**Step 2: Write the extraction script**

```python
#!/usr/bin/env python3
"""Extract all Entity keys from phase YAML files."""

import yaml
import json
from pathlib import Path

def extract_entities():
    """Extract all entities from phase YAML files."""
    entities_dir = Path("packages/core/data/entities/qrcode-ai")
    entities = []

    for phase_file in sorted(entities_dir.glob("phase-*.yaml")):
        with open(phase_file, "r") as f:
            data = yaml.safe_load(f)

        phase_name = data.get("name", "Unknown")
        for entity in data.get("entities", []):
            entities.append({
                "key": entity["key"],
                "type": entity.get("type", "THING"),
                "display_name": entity.get("display_name", ""),
                "description": entity.get("description", ""),
                "llm_context": entity.get("llm_context", ""),
                "phase": phase_name
            })

    return entities

if __name__ == "__main__":
    entities = extract_entities()

    output_path = Path("scripts/seo-import/output/entities.json")
    output_path.parent.mkdir(parents=True, exist_ok=True)

    with open(output_path, "w") as f:
        json.dump(entities, f, indent=2, ensure_ascii=False)

    print(f"Extracted {len(entities)} entities to {output_path}")
```

**Step 3: Run extraction**

```bash
cd /Users/thibaut/supernovae-st/novanet-hq
python scripts/seo-import/extract_entities.py
```

Expected: `Extracted 281 entities to scripts/seo-import/output/entities.json`

**Step 4: Commit**

```bash
git add scripts/seo-import/
git commit -m "feat(seo): add entity extraction script"
```

---

### Task 1.2: Create French translations for EntityL10n

**Files:**
- Create: `scripts/seo-import/generate_entityl10n_fr.py`
- Read: `scripts/seo-import/output/entities.json`
- Output: `scripts/seo-import/output/entityl10n_fr.json`

**Step 1: Write the translation generator**

```python
#!/usr/bin/env python3
"""Generate EntityL10n fr-FR content from Entity definitions.

This script creates French localized content for each Entity.
For production, this would use an LLM API. For now, uses rule-based translation.
"""

import json
import re
from pathlib import Path

# French translations for common terms
TRANSLATIONS = {
    "QR Code": "QR Code",
    "QR code": "QR Code",
    "qr code": "QR Code",
    "Create": "Créer",
    "create": "créer",
    "Scan": "Scanner",
    "scan": "scanner",
    "Generator": "Générateur",
    "generator": "générateur",
    "Scanner": "Scanner",
    "Dynamic": "Dynamique",
    "dynamic": "dynamique",
    "Static": "Statique",
    "static": "statique",
    "Custom": "Personnalisé",
    "custom": "personnalisé",
    "with Logo": "avec Logo",
    "with Text": "avec Texte",
    "with Image": "avec Image",
    "Background": "Arrière-plan",
    "Color": "Couleur",
    "Colors": "Couleurs",
    "Shapes": "Formes",
    "Transparent": "Transparent",
    "Business Card": "Carte de Visite",
    "Landing Page": "Page d'Atterrissage",
    "Smart Link": "Lien Intelligent",
    "Short Link": "Lien Court",
    "Barcode": "Code-barres",
    "WiFi": "WiFi",
    "vCard": "vCard",
    "URL": "URL",
    "SMS": "SMS",
    "Email": "Email",
    "PDF": "PDF",
    "Menu": "Menu",
    "Restaurant": "Restaurant",
    "Analytics": "Analytiques",
    "Tracking": "Suivi",
    "Download": "Télécharger",
    "Print": "Imprimer",
    "Share": "Partager",
    "Design": "Design",
    "Guide": "Guide",
    "How to": "Comment",
    "vs": "vs",
    "Free": "Gratuit",
    "Paid": "Payant",
}

def translate_display_name(en_name: str) -> str:
    """Translate display name to French."""
    result = en_name
    for en, fr in sorted(TRANSLATIONS.items(), key=lambda x: -len(x[0])):
        result = result.replace(en, fr)
    return result

def key_to_slug(key: str) -> str:
    """Convert entity key to French slug."""
    # Replace common English terms with French
    slug = key
    slug_replacements = {
        "create-": "creer-",
        "scan-": "scanner-",
        "download-": "telecharger-",
        "print-": "imprimer-",
        "share-": "partager-",
        "how-to-": "comment-",
        "-guide": "-guide",
        "-vs-": "-vs-",
        "dynamic-": "dynamique-",
        "static-": "statique-",
        "custom-": "personnalise-",
        "business-card": "carte-visite",
        "landing-page": "page-atterrissage",
        "smart-link": "lien-intelligent",
        "short-link": "lien-court",
    }
    for en, fr in slug_replacements.items():
        slug = slug.replace(en, fr)
    return slug

def generate_french_description(en_desc: str, entity_type: str) -> str:
    """Generate French description."""
    # For now, keep English with a note - in production would use LLM
    return en_desc  # TODO: LLM translation

def generate_french_llm_context(en_context: str) -> str:
    """Generate French llm_context."""
    # Keep structure, translate keywords
    result = en_context
    result = result.replace("USE:", "UTILISER:")
    result = result.replace("TRIGGERS:", "DECLENCHEURS:")
    result = result.replace("NOT:", "PAS:")
    return result

def generate_entityl10n(entities: list) -> list:
    """Generate EntityL10n fr-FR for all entities."""
    entityl10n_list = []

    for entity in entities:
        entityl10n = {
            "entity_key": entity["key"],
            "locale_key": "fr-FR",
            "slug": key_to_slug(entity["key"]),
            "display_name": translate_display_name(entity["display_name"]),
            "description": generate_french_description(entity["description"], entity["type"]),
            "llm_context": generate_french_llm_context(entity.get("llm_context", "")),
            "version": 1
        }
        entityl10n_list.append(entityl10n)

    return entityl10n_list

if __name__ == "__main__":
    # Load entities
    with open("scripts/seo-import/output/entities.json", "r") as f:
        entities = json.load(f)

    # Generate French versions
    entityl10n_list = generate_entityl10n(entities)

    # Save output
    output_path = Path("scripts/seo-import/output/entityl10n_fr.json")
    with open(output_path, "w") as f:
        json.dump(entityl10n_list, f, indent=2, ensure_ascii=False)

    print(f"Generated {len(entityl10n_list)} EntityL10n fr-FR to {output_path}")
```

**Step 2: Run generation**

```bash
python scripts/seo-import/generate_entityl10n_fr.py
```

Expected: `Generated 281 EntityL10n fr-FR to scripts/seo-import/output/entityl10n_fr.json`

**Step 3: Commit**

```bash
git add scripts/seo-import/
git commit -m "feat(seo): add EntityL10n fr-FR generator"
```

---

### Task 1.3: Generate Cypher seed file for EntityL10n

**Files:**
- Create: `scripts/seo-import/generate_cypher_entityl10n.py`
- Read: `scripts/seo-import/output/entityl10n_fr.json`
- Output: `packages/db/seed/40-entityl10n-fr-fr.cypher`

**Step 1: Write the Cypher generator**

```python
#!/usr/bin/env python3
"""Generate Cypher seed file for EntityL10n fr-FR."""

import json
from pathlib import Path
from datetime import datetime

def escape_cypher(s: str) -> str:
    """Escape string for Cypher."""
    if not s:
        return ""
    return s.replace("\\", "\\\\").replace('"', '\\"').replace("'", "\\'")

def generate_cypher(entityl10n_list: list) -> str:
    """Generate Cypher statements for EntityL10n nodes and relationships."""
    lines = [
        "// ═══════════════════════════════════════════════════════════════════════════════",
        "// EntityL10n fr-FR — 281 nodes",
        "// Generated: " + datetime.now().isoformat(),
        "// ═══════════════════════════════════════════════════════════════════════════════",
        "",
        "// Create constraint for uniqueness",
        "CREATE CONSTRAINT entityl10n_unique IF NOT EXISTS",
        "FOR (el:EntityL10n)",
        "REQUIRE (el.entity_key, el.locale_key) IS UNIQUE;",
        "",
        "// ───────────────────────────────────────────────────────────────────────────────",
        "// EntityL10n Nodes",
        "// ───────────────────────────────────────────────────────────────────────────────",
        ""
    ]

    for el in entityl10n_list:
        lines.append(f'MERGE (el:EntityL10n {{entity_key: "{el["entity_key"]}", locale_key: "{el["locale_key"]}"}})')
        lines.append("ON CREATE SET")
        lines.append(f'  el.slug = "{escape_cypher(el["slug"])}",')
        lines.append(f'  el.display_name = "{escape_cypher(el["display_name"])}",')
        lines.append(f'  el.description = "{escape_cypher(el["description"])}",')
        lines.append(f'  el.llm_context = "{escape_cypher(el["llm_context"])}",')
        lines.append(f'  el.version = {el["version"]},')
        lines.append("  el.created_at = datetime(),")
        lines.append("  el.updated_at = datetime();")
        lines.append("")

    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("// Link EntityL10n to Entity via HAS_L10N")
    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("")

    for el in entityl10n_list:
        lines.append(f'MATCH (e:Entity {{key: "{el["entity_key"]}"}})')
        lines.append(f'MATCH (el:EntityL10n {{entity_key: "{el["entity_key"]}", locale_key: "{el["locale_key"]}"}})')
        lines.append("MERGE (e)-[:HAS_L10N]->(el);")
        lines.append("")

    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("// Link EntityL10n to Locale via FOR_LOCALE")
    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("")

    for el in entityl10n_list:
        lines.append(f'MATCH (el:EntityL10n {{entity_key: "{el["entity_key"]}", locale_key: "{el["locale_key"]}"}})')
        lines.append(f'MATCH (l:Locale {{key: "{el["locale_key"]}"}})')
        lines.append("MERGE (el)-[:FOR_LOCALE]->(l);")
        lines.append("")

    return "\n".join(lines)

if __name__ == "__main__":
    # Load EntityL10n data
    with open("scripts/seo-import/output/entityl10n_fr.json", "r") as f:
        entityl10n_list = json.load(f)

    # Generate Cypher
    cypher = generate_cypher(entityl10n_list)

    # Save to seed file
    output_path = Path("packages/db/seed/40-entityl10n-fr-fr.cypher")
    with open(output_path, "w") as f:
        f.write(cypher)

    print(f"Generated Cypher for {len(entityl10n_list)} EntityL10n to {output_path}")
```

**Step 2: Run Cypher generation**

```bash
python scripts/seo-import/generate_cypher_entityl10n.py
```

Expected: `Generated Cypher for 281 EntityL10n to packages/db/seed/40-entityl10n-fr-fr.cypher`

**Step 3: Commit**

```bash
git add scripts/seo-import/ packages/db/seed/40-entityl10n-fr-fr.cypher
git commit -m "feat(seo): add EntityL10n fr-FR seed file (281 nodes)"
```

---

## Phase 2: Filter and Import SEO Keywords

### Task 2.1: Create keyword filtering script

**Files:**
- Create: `scripts/seo-import/filter_keywords.py`
- Read: `docs/assets/keywods/fr-fr_qr/seo/google_fr_qr_matching-terms_2026-02-06_16-39-15.csv`
- Output: `scripts/seo-import/output/keywords_filtered.json`

**Step 1: Write the filtering script**

```python
#!/usr/bin/env python3
"""Filter SEO keywords from Ahrefs CSV export.

Filters:
- Volume >= 50
- French language keywords only
- Remove gaming keywords (separate category)
- Remove spam/nonsense
- Remove temporary events
"""

import csv
import json
import re
from pathlib import Path

# Keywords to exclude (gaming, events, spam)
EXCLUDE_PATTERNS = [
    r"brawl\s*stars",
    r"pokemon",
    r"yo-?kai",
    r"clash\s*royale",
    r"fortnite",
    r"minecraft",
    r"roblox",
    r"pass\s*sanitaire",
    r"jo\s*paris",
    r"covid",
    r"vaccination",
]

# Non-French language patterns
NON_FRENCH_PATTERNS = [
    r"^qr\s*コード",  # Japanese
    r"^código\s*qr",  # Spanish
    r"^二维码",       # Chinese
    r"^qr\s*코드",    # Korean
]

def is_excluded(keyword: str) -> bool:
    """Check if keyword should be excluded."""
    keyword_lower = keyword.lower()
    for pattern in EXCLUDE_PATTERNS + NON_FRENCH_PATTERNS:
        if re.search(pattern, keyword_lower):
            return True
    return False

def parse_volume(vol_str: str) -> int:
    """Parse volume string to int."""
    if not vol_str:
        return 0
    try:
        return int(vol_str.replace(",", "").replace(" ", ""))
    except ValueError:
        return 0

def filter_keywords(csv_path: Path, min_volume: int = 50) -> list:
    """Filter keywords from CSV."""
    keywords = []

    # Read UTF-16LE encoded file
    with open(csv_path, "r", encoding="utf-16-le") as f:
        # Skip BOM and read as TSV
        content = f.read()
        if content.startswith('\ufeff'):
            content = content[1:]

        reader = csv.DictReader(content.splitlines(), delimiter="\t")

        for row in reader:
            keyword = row.get("Keyword", "").strip()
            volume = parse_volume(row.get("Volume", "0"))

            # Apply filters
            if volume < min_volume:
                continue
            if is_excluded(keyword):
                continue
            if not keyword:
                continue

            keywords.append({
                "value": keyword,
                "volume": volume,
                "difficulty": int(row.get("Difficulty", 0) or 0),
                "cpc": float(row.get("CPC", 0) or 0),
                "intent": row.get("Intents", ""),
                "parent_keyword": row.get("Parent Keyword", ""),
                "traffic_potential": int(row.get("Traffic potential", 0) or 0),
            })

    return keywords

if __name__ == "__main__":
    csv_path = Path("docs/assets/keywods/fr-fr_qr/seo/google_fr_qr_matching-terms_2026-02-06_16-39-15.csv")

    keywords = filter_keywords(csv_path, min_volume=50)

    # Sort by volume descending
    keywords.sort(key=lambda x: x["volume"], reverse=True)

    # Save output
    output_path = Path("scripts/seo-import/output/keywords_filtered.json")
    with open(output_path, "w") as f:
        json.dump(keywords, f, indent=2, ensure_ascii=False)

    print(f"Filtered to {len(keywords)} keywords (from ~75K)")
    print(f"Top 10 by volume:")
    for kw in keywords[:10]:
        print(f"  {kw['volume']:>6}  {kw['value']}")
```

**Step 2: Run filtering**

```bash
python scripts/seo-import/filter_keywords.py
```

Expected: `Filtered to ~40000 keywords (from ~75K)`

**Step 3: Commit**

```bash
git add scripts/seo-import/
git commit -m "feat(seo): add keyword filtering script"
```

---

### Task 2.2: Create keyword-to-entity mapping script

**Files:**
- Create: `scripts/seo-import/map_keywords_to_entities.py`
- Read: `scripts/seo-import/output/keywords_filtered.json`
- Read: `scripts/seo-import/output/entityl10n_fr.json`
- Output: `scripts/seo-import/output/keyword_mappings.json`

**Step 1: Write the mapping script**

```python
#!/usr/bin/env python3
"""Map SEO keywords to EntityL10n based on pattern matching.

Mapping rules by category:
1. Creation keywords -> create-qr-code, qr-code-generator
2. Scan keywords -> scan-qr-code, qr-code-scanner
3. Content type keywords -> qr-code-{type}
4. Brand keywords -> {brand}
5. Industry keywords -> {industry}
6. Feature keywords -> {feature}
"""

import json
import re
from pathlib import Path
from typing import Optional

# Mapping patterns: regex -> entity_key
MAPPING_RULES = [
    # Creation (high priority)
    (r"(créer|creer|générer|generer|create|generator|maker|fabriquer)", "create-qr-code"),

    # Scanning
    (r"(scanner|scan|lire|lecteur|reader|flasher|lector)", "scan-qr-code"),

    # Content types
    (r"\bwifi\b", "qr-code-wifi"),
    (r"\bvcard\b", "qr-code-vcard"),
    (r"\burl\b", "qr-code-url"),
    (r"\bsms\b", "qr-code-sms"),
    (r"\bemail\b", "qr-code-email"),
    (r"\bpdf\b", "qr-code-pdf"),
    (r"\bmenu\b", "qr-code-menu"),
    (r"\bvideo\b", "qr-code-video"),
    (r"\baudio\b", "qr-code-audio"),
    (r"\blocation\b", "qr-code-location"),
    (r"\bcalendar\b", "qr-code-calendar"),
    (r"\bpayment|paiement\b", "qr-code-payment"),

    # Social/Brands
    (r"\binstagram\b", "instagram"),
    (r"\bfacebook\b", "facebook"),
    (r"\bwhatsapp\b", "whatsapp"),
    (r"\blinkedin\b", "linkedin"),
    (r"\btiktok\b", "tiktok"),
    (r"\byoutube\b", "youtube"),
    (r"\btwitter\b", "twitter"),
    (r"\bspotify\b", "spotify"),
    (r"\bsnapchat\b", "snapchat"),
    (r"\bpinterest\b", "pinterest"),
    (r"\btelegram\b", "telegram"),

    # Design
    (r"(logo|avec logo)", "qr-code-with-logo"),
    (r"(couleur|color)", "qr-code-color"),
    (r"(personnalis|custom)", "custom-qr-code"),
    (r"(transparent)", "qr-code-transparent-background"),
    (r"(art|artistique)", "qr-code-art"),

    # Industries
    (r"\brestaurant\b", "restaurants"),
    (r"\bretail\b", "retail"),
    (r"\bsanté|health\b", "healthcare"),
    (r"\béducation|education\b", "education"),
    (r"\bhôtel|hotel\b", "hospitality"),
    (r"\bimmobilier|real estate\b", "real-estate"),

    # Concepts
    (r"\bdynamique|dynamic\b", "dynamic-qr-code"),
    (r"\bstatique|static\b", "static-qr-code"),
    (r"\bgratuit|free\b", "qr-code"),  # Maps to main entity

    # Barcode types
    (r"\bcode.?barre|barcode\b", "barcode"),
    (r"\bean.?13\b", "ean-13"),
    (r"\bupc\b", "upc-a"),
    (r"\bdata.?matrix\b", "data-matrix"),

    # Actions
    (r"\btélécharger|download\b", "download-qr-code"),
    (r"\bimprimer|print\b", "print-qr-code"),

    # Default fallback
    (r"\bqr\s*code\b", "qr-code"),
]

def find_entity_for_keyword(keyword: str, entity_keys: set) -> Optional[str]:
    """Find the best matching entity for a keyword."""
    keyword_lower = keyword.lower()

    for pattern, entity_key in MAPPING_RULES:
        if re.search(pattern, keyword_lower, re.IGNORECASE):
            if entity_key in entity_keys:
                return entity_key

    return None

def map_keywords_to_entities(keywords: list, entityl10n_list: list) -> list:
    """Map each keyword to an EntityL10n."""
    entity_keys = {el["entity_key"] for el in entityl10n_list}

    mappings = []
    unmapped_count = 0

    for kw in keywords:
        entity_key = find_entity_for_keyword(kw["value"], entity_keys)

        if entity_key:
            mappings.append({
                "keyword": kw["value"],
                "volume": kw["volume"],
                "difficulty": kw["difficulty"],
                "cpc": kw["cpc"],
                "intent": kw["intent"],
                "entity_key": entity_key,
                "locale_key": "fr-FR"
            })
        else:
            unmapped_count += 1

    return mappings, unmapped_count

if __name__ == "__main__":
    # Load data
    with open("scripts/seo-import/output/keywords_filtered.json", "r") as f:
        keywords = json.load(f)

    with open("scripts/seo-import/output/entityl10n_fr.json", "r") as f:
        entityl10n_list = json.load(f)

    # Map keywords
    mappings, unmapped = map_keywords_to_entities(keywords, entityl10n_list)

    # Save output
    output_path = Path("scripts/seo-import/output/keyword_mappings.json")
    with open(output_path, "w") as f:
        json.dump(mappings, f, indent=2, ensure_ascii=False)

    print(f"Mapped {len(mappings)} keywords to entities")
    print(f"Unmapped: {unmapped} keywords")

    # Stats by entity
    from collections import Counter
    entity_counts = Counter(m["entity_key"] for m in mappings)
    print(f"\nTop 10 entities by keyword count:")
    for entity, count in entity_counts.most_common(10):
        print(f"  {count:>5}  {entity}")
```

**Step 2: Run mapping**

```bash
python scripts/seo-import/map_keywords_to_entities.py
```

Expected: Stats showing keyword distribution across entities

**Step 3: Commit**

```bash
git add scripts/seo-import/
git commit -m "feat(seo): add keyword-to-entity mapping"
```

---

### Task 2.3: Generate Cypher seed file for SEOKeyword and TARGETS

**Files:**
- Create: `scripts/seo-import/generate_cypher_seokeyword.py`
- Read: `scripts/seo-import/output/keyword_mappings.json`
- Output: `packages/db/seed/41-seokeywords-fr-fr.cypher`

**Step 1: Write the Cypher generator**

```python
#!/usr/bin/env python3
"""Generate Cypher seed file for SEOKeyword nodes and TARGETS relationships."""

import json
import hashlib
from pathlib import Path
from datetime import datetime

def escape_cypher(s: str) -> str:
    """Escape string for Cypher."""
    if not s:
        return ""
    return s.replace("\\", "\\\\").replace('"', '\\"').replace("'", "\\'")

def keyword_to_key(keyword: str, locale: str) -> str:
    """Generate unique key for SEOKeyword."""
    # Create a slug from the keyword
    slug = keyword.lower()
    slug = slug.replace(" ", "-").replace("'", "")
    slug = "".join(c for c in slug if c.isalnum() or c == "-")
    slug = slug[:50]  # Truncate

    # Add hash suffix for uniqueness
    hash_suffix = hashlib.md5(keyword.encode()).hexdigest()[:6]

    return f"seo-{slug}-{locale.lower()}-{hash_suffix}"

def generate_cypher(mappings: list, batch_size: int = 500) -> str:
    """Generate Cypher statements for SEOKeyword nodes and TARGETS relationships."""
    lines = [
        "// ═══════════════════════════════════════════════════════════════════════════════",
        f"// SEOKeyword fr-FR — {len(mappings)} keywords",
        "// Generated: " + datetime.now().isoformat(),
        "// ═══════════════════════════════════════════════════════════════════════════════",
        "",
        "// Create index for fast lookup",
        "CREATE INDEX seokeyword_value IF NOT EXISTS FOR (kw:SEOKeyword) ON (kw.value);",
        "CREATE INDEX seokeyword_volume IF NOT EXISTS FOR (kw:SEOKeyword) ON (kw.volume);",
        "",
    ]

    # Group by entity for batching
    by_entity = {}
    for m in mappings:
        key = m["entity_key"]
        if key not in by_entity:
            by_entity[key] = []
        by_entity[key].append(m)

    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("// SEOKeyword Nodes + TARGETS relationships (grouped by Entity)")
    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("")

    for entity_key, entity_keywords in sorted(by_entity.items()):
        lines.append(f"// --- {entity_key} ({len(entity_keywords)} keywords) ---")
        lines.append("")

        for m in entity_keywords:
            kw_key = keyword_to_key(m["keyword"], m["locale_key"])

            # Create SEOKeyword node
            lines.append(f'MERGE (kw:SEOKeyword {{key: "{kw_key}"}})')
            lines.append("ON CREATE SET")
            lines.append(f'  kw.value = "{escape_cypher(m["keyword"])}",')
            lines.append(f'  kw.volume = {m["volume"]},')
            lines.append(f'  kw.difficulty = {m["difficulty"]},')
            lines.append(f'  kw.cpc = {m["cpc"]},')
            lines.append(f'  kw.intent = "{escape_cypher(m["intent"])}",')
            lines.append("  kw.created_at = datetime(),")
            lines.append("  kw.updated_at = datetime();")
            lines.append("")

            # Create TARGETS relationship
            lines.append(f'MATCH (el:EntityL10n {{entity_key: "{m["entity_key"]}", locale_key: "{m["locale_key"]}"}})')
            lines.append(f'MATCH (kw:SEOKeyword {{key: "{kw_key}"}})')
            lines.append("MERGE (el)-[:TARGETS]->(kw);")
            lines.append("")

    return "\n".join(lines)

if __name__ == "__main__":
    # Load mappings
    with open("scripts/seo-import/output/keyword_mappings.json", "r") as f:
        mappings = json.load(f)

    # Generate Cypher
    cypher = generate_cypher(mappings)

    # Save to seed file
    output_path = Path("packages/db/seed/41-seokeywords-fr-fr.cypher")
    with open(output_path, "w") as f:
        f.write(cypher)

    print(f"Generated Cypher for {len(mappings)} SEOKeywords to {output_path}")
    print(f"File size: {len(cypher) / 1024:.1f} KB")
```

**Step 2: Run Cypher generation**

```bash
python scripts/seo-import/generate_cypher_seokeyword.py
```

Expected: `Generated Cypher for ~40000 SEOKeywords`

**Step 3: Commit**

```bash
git add scripts/seo-import/ packages/db/seed/41-seokeywords-fr-fr.cypher
git commit -m "feat(seo): add SEOKeyword fr-FR seed file (~40K keywords)"
```

---

## Phase 3: Load Data into Neo4j

### Task 3.1: Run seed files

**Step 1: Start Neo4j (if not running)**

```bash
cd /Users/thibaut/supernovae-st/novanet-hq
pnpm infra:up
```

**Step 2: Load EntityL10n seed**

```bash
cd /Users/thibaut/supernovae-st/novanet-hq/tools/novanet
cargo run -- db seed --file=../../packages/db/seed/40-entityl10n-fr-fr.cypher
```

Expected: `Executed 281 statements`

**Step 3: Load SEOKeyword seed**

```bash
cargo run -- db seed --file=../../packages/db/seed/41-seokeywords-fr-fr.cypher
```

Expected: `Executed ~80000 statements` (nodes + relationships)

**Step 4: Verify in Neo4j Browser**

Open http://localhost:7474 and run:

```cypher
// Count EntityL10n fr-FR
MATCH (el:EntityL10n {locale_key: "fr-FR"})
RETURN count(el) AS entityl10n_count;
// Expected: 281

// Count SEOKeywords
MATCH (kw:SEOKeyword)
RETURN count(kw) AS seokeyword_count;
// Expected: ~40000

// Count TARGETS relationships
MATCH (:EntityL10n)-[r:TARGETS]->(:SEOKeyword)
RETURN count(r) AS targets_count;
// Expected: ~40000

// Top keywords by volume
MATCH (el:EntityL10n {locale_key: "fr-FR"})-[:TARGETS]->(kw:SEOKeyword)
RETURN el.entity_key, kw.value, kw.volume
ORDER BY kw.volume DESC
LIMIT 20;
```

**Step 5: Commit verification**

```bash
git add .
git commit -m "feat(seo): verify SEO keyword import complete"
```

---

## Summary

| Phase | Task | Output |
|-------|------|--------|
| 1.1 | Extract entities | `entities.json` (281) |
| 1.2 | Generate EntityL10n fr-FR | `entityl10n_fr.json` (281) |
| 1.3 | Generate Cypher | `40-entityl10n-fr-fr.cypher` |
| 2.1 | Filter keywords | `keywords_filtered.json` (~40K) |
| 2.2 | Map to entities | `keyword_mappings.json` (~40K) |
| 2.3 | Generate Cypher | `41-seokeywords-fr-fr.cypher` |
| 3.1 | Load into Neo4j | 281 EntityL10n + ~40K SEOKeyword |

**Total:** 281 EntityL10n fr-FR + ~40,000 SEOKeyword nodes + ~40,000 TARGETS relationships
