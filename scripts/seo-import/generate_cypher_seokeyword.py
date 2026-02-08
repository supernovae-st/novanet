#!/usr/bin/env python3
"""Generate Cypher seed file for SEOKeyword nodes and TARGETS relationships.

Task 2.3 from implementation plan:
- Input: scripts/seo-import/output/keywords_mapped.json (1,519 keywords)
- Output: packages/db/seed/12-seokeyword-fr-fr.cypher

Key generation:
- Sanitize keyword to lowercase ASCII
- Replace spaces with hyphens
- Add 'seo-' prefix and '-fr' suffix
- Example: "créer qr code gratuit" → "seo-creer-qr-code-gratuit-fr"
"""

import json
import unicodedata
import re
from pathlib import Path
from datetime import datetime


def remove_accents(text: str) -> str:
    """Remove accents from text (créer → creer)."""
    # Normalize to decomposed form (é → e + combining accent)
    normalized = unicodedata.normalize('NFD', text)
    # Remove combining diacritical marks
    return ''.join(c for c in normalized if unicodedata.category(c) != 'Mn')


def keyword_to_key(keyword: str) -> str:
    """Convert keyword to SEOKeyword key.

    Example: "créer qr code gratuit" → "seo-creer-qr-code-gratuit-fr"
    """
    # Remove accents
    key = remove_accents(keyword.lower())

    # Replace spaces and special chars with hyphens
    key = re.sub(r'[^a-z0-9]+', '-', key)

    # Remove leading/trailing hyphens
    key = key.strip('-')

    # Collapse multiple hyphens
    key = re.sub(r'-+', '-', key)

    # Truncate to reasonable length (max 60 chars for key body)
    key = key[:60]

    # Add prefix and suffix
    return f"seo-{key}-fr"


def escape_cypher(s: str) -> str:
    """Escape string for Cypher single-quoted strings."""
    if not s:
        return ""
    # Escape backslashes first, then single quotes
    return s.replace("\\", "\\\\").replace("'", "\\'")


def generate_cypher(keywords: list) -> str:
    """Generate Cypher for SEOKeyword nodes and TARGETS relationships."""

    # Count stats
    total = len(keywords)
    with_entity = sum(1 for k in keywords if k.get('entity_key'))

    lines = [
        f"// SEOKeyword fr-FR for QR Code AI ({total} keywords)",
        f"// Generated: {datetime.now().strftime('%Y-%m-%d')}",
        "//",
        f"// Keywords with entity mapping: {with_entity}",
        f"// Keywords without entity mapping: {total - with_entity}",
        "",
        "// ═══════════════════════════════════════════════════════════════════════════════",
        "// INDEXES",
        "// ═══════════════════════════════════════════════════════════════════════════════",
        "",
        "CREATE INDEX seokeyword_key IF NOT EXISTS FOR (kw:SEOKeyword) ON (kw.key);",
        "CREATE INDEX seokeyword_value IF NOT EXISTS FOR (kw:SEOKeyword) ON (kw.value);",
        "CREATE INDEX seokeyword_volume IF NOT EXISTS FOR (kw:SEOKeyword) ON (kw.volume);",
        "",
        "// ═══════════════════════════════════════════════════════════════════════════════",
        "// SECTION 1: CREATE SEOKeyword NODES",
        "// ═══════════════════════════════════════════════════════════════════════════════",
        "",
    ]

    # Track unique keys to avoid duplicates
    seen_keys = set()

    for kw in keywords:
        key = keyword_to_key(kw['keyword'])

        # Handle duplicate keys by appending a counter
        original_key = key
        counter = 1
        while key in seen_keys:
            key = f"{original_key}-{counter}"
            counter += 1
        seen_keys.add(key)

        value = escape_cypher(kw['keyword'])
        volume = kw.get('volume', 0)
        difficulty = kw.get('difficulty', 0)
        cpc = kw.get('cpc', 0.0)
        intent = escape_cypher(kw.get('intent', ''))
        traffic_potential = kw.get('traffic_potential', 0)

        lines.append(f"MERGE (kw:SEOKeyword {{key: '{key}'}})")
        lines.append(f"SET kw.value = '{value}',")
        lines.append(f"    kw.volume = {volume},")
        lines.append(f"    kw.difficulty = {difficulty},")
        lines.append(f"    kw.cpc = {cpc},")
        lines.append(f"    kw.intent = '{intent}',")
        lines.append(f"    kw.traffic_potential = {traffic_potential},")
        lines.append(f"    kw.source = 'ahrefs',")
        lines.append(f"    kw.created_at = datetime(),")
        lines.append(f"    kw.updated_at = datetime();")
        lines.append("")

    lines.append("// ═══════════════════════════════════════════════════════════════════════════════")
    lines.append("// SECTION 2: CREATE TARGETS RELATIONS (EntityL10n -> SEOKeyword)")
    lines.append("// ═══════════════════════════════════════════════════════════════════════════════")
    lines.append("")

    # Reset for second pass
    seen_keys = set()
    relation_count = 0

    for kw in keywords:
        entity_key = kw.get('entity_key')
        if not entity_key:
            continue

        key = keyword_to_key(kw['keyword'])

        # Handle duplicate keys consistently
        original_key = key
        counter = 1
        while key in seen_keys:
            key = f"{original_key}-{counter}"
            counter += 1
        seen_keys.add(key)

        lines.append(f"MATCH (el:EntityL10n {{entity_key: '{entity_key}', locale_key: 'fr-FR'}})")
        lines.append(f"MATCH (kw:SEOKeyword {{key: '{key}'}})")
        lines.append("MERGE (el)-[:TARGETS]->(kw);")
        lines.append("")
        relation_count += 1

    return "\n".join(lines), total, relation_count


def main():
    # Paths
    input_path = Path("scripts/seo-import/output/keywords_mapped.json")
    output_path = Path("packages/db/seed/12-seokeyword-fr-fr.cypher")

    # Load keywords
    with open(input_path, "r", encoding="utf-8") as f:
        keywords = json.load(f)

    print(f"Loaded {len(keywords)} keywords from {input_path}")

    # Generate Cypher
    cypher, keyword_count, relation_count = generate_cypher(keywords)

    # Write output
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(cypher)

    # Stats
    line_count = cypher.count('\n') + 1
    file_size = len(cypher) / 1024

    print(f"\nOutput: {output_path}")
    print(f"  Lines: {line_count:,}")
    print(f"  Keywords: {keyword_count:,}")
    print(f"  TARGETS relations: {relation_count:,}")
    print(f"  File size: {file_size:.1f} KB")

    # Sample output
    print("\n--- Sample output (first 30 lines) ---")
    for line in cypher.split('\n')[:30]:
        print(line)


if __name__ == "__main__":
    main()
