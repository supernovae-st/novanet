#!/usr/bin/env python3
"""Generate Cypher seed file for EntityContent fr-FR."""

import json
from pathlib import Path
from datetime import datetime

def escape_cypher(s: str) -> str:
    """Escape string for Cypher."""
    if not s:
        return ""
    return s.replace("\\", "\\\\").replace('"', '\\"').replace("'", "\\'")

def generate_cypher(entitycontent_list: list) -> str:
    """Generate Cypher statements for EntityContent nodes and relationships."""
    lines = [
        "// ═══════════════════════════════════════════════════════════════════════════════",
        "// EntityContent fr-FR — 281 nodes",
        "// Generated: " + datetime.now().isoformat(),
        "// ═══════════════════════════════════════════════════════════════════════════════",
        "",
        "// Create constraint for uniqueness",
        "CREATE CONSTRAINT entitycontent_unique IF NOT EXISTS",
        "FOR (ec:EntityContent)",
        "REQUIRE (ec.entity_key, ec.locale_key) IS UNIQUE;",
        "",
        "// ───────────────────────────────────────────────────────────────────────────────",
        "// EntityContent Nodes",
        "// ───────────────────────────────────────────────────────────────────────────────",
        ""
    ]

    for ec in entitycontent_list:
        lines.append(f'MERGE (ec:EntityContent {{entity_key: "{ec["entity_key"]}", locale_key: "{ec["locale_key"]}"}})')
        lines.append("ON CREATE SET")
        lines.append(f'  ec.slug = "{escape_cypher(ec["slug"])}",')
        lines.append(f'  ec.display_name = "{escape_cypher(ec["display_name"])}",')
        lines.append(f'  ec.description = "{escape_cypher(ec["description"])}",')
        lines.append(f'  ec.llm_context = "{escape_cypher(ec["llm_context"])}",')
        lines.append(f'  ec.version = {ec["version"]},')
        lines.append("  ec.created_at = datetime(),")
        lines.append("  ec.updated_at = datetime();")
        lines.append("")

    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("// Link EntityContent to Entity via HAS_CONTENT")
    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("")

    for ec in entitycontent_list:
        lines.append(f'MATCH (e:Entity {{key: "{ec["entity_key"]}"}})')
        lines.append(f'MATCH (ec:EntityContent {{entity_key: "{ec["entity_key"]}", locale_key: "{ec["locale_key"]}"}})')
        lines.append("MERGE (e)-[:HAS_CONTENT]->(ec);")
        lines.append("")

    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("// Link EntityContent to Locale via FOR_LOCALE")
    lines.append("// ───────────────────────────────────────────────────────────────────────────────")
    lines.append("")

    for ec in entitycontent_list:
        lines.append(f'MATCH (ec:EntityContent {{entity_key: "{ec["entity_key"]}", locale_key: "{ec["locale_key"]}"}})')
        lines.append(f'MATCH (l:Locale {{key: "{ec["locale_key"]}"}})')
        lines.append("MERGE (ec)-[:FOR_LOCALE]->(l);")
        lines.append("")

    return "\n".join(lines)

if __name__ == "__main__":
    # Load EntityContent data
    with open("scripts/seo-import/output/entitycontent_fr.json", "r") as f:
        entitycontent_list = json.load(f)

    # Generate Cypher
    cypher = generate_cypher(entitycontent_list)

    # Save to seed file
    output_path = Path("packages/db/seed/40-entitycontent-fr-fr.cypher")
    with open(output_path, "w") as f:
        f.write(cypher)

    print(f"Generated Cypher for {len(entitycontent_list)} EntityContent to {output_path}")
