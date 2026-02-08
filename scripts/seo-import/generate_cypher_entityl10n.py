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
