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
            lines.append(f'MATCH (el:EntityContent {{entity_key: "{m["entity_key"]}", locale_key: "{m["locale_key"]}"}})')
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
