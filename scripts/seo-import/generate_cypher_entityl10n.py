#!/usr/bin/env python3
"""Generate Cypher seed file for EntityL10n fr-FR.

Task 1.3: Creates Neo4j Cypher statements to:
1. MERGE EntityL10n nodes with all properties
2. Link EntityL10n to parent Entity via HAS_L10N
3. Link EntityL10n to Locale via FOR_LOCALE
"""

import json
from pathlib import Path
from datetime import datetime


def escape_cypher(s: str) -> str:
    """Escape string for Cypher single-quoted literals.

    Handles: backslashes, single quotes, and newlines.
    """
    if not s:
        return ""
    # Order matters: escape backslashes first
    s = s.replace("\\", "\\\\")
    s = s.replace("'", "\\'")
    s = s.replace("\n", "\\n")
    s = s.replace("\r", "")
    return s


def generate_cypher(entityl10n_list: list) -> str:
    """Generate Cypher statements for EntityL10n nodes and relationships."""
    now = datetime.now().strftime("%Y-%m-%d")
    count = len(entityl10n_list)

    lines = [
        "// ===================================================================",
        f"// EntityL10n fr-FR for QR Code AI ({count} entities)",
        f"// Generated: {now}",
        "// ===================================================================",
        "",
        "// -------------------------------------------------------------------",
        "// Create EntityL10n nodes",
        "// -------------------------------------------------------------------",
        "",
    ]

    # Part 1: Create all EntityL10n nodes
    for el in entityl10n_list:
        entity_key = escape_cypher(el["entity_key"])
        locale_key = escape_cypher(el["locale_key"])
        slug = escape_cypher(el["slug"])
        display_name = escape_cypher(el["display_name"])
        description = escape_cypher(el["description"])
        definition = escape_cypher(el.get("definition", ""))
        purpose = escape_cypher(el.get("purpose", ""))
        llm_context = escape_cypher(el.get("llm_context", ""))
        version = el.get("version", 1)

        lines.append(f"MERGE (el:EntityL10n {{entity_key: '{entity_key}', locale_key: '{locale_key}'}})")
        lines.append(f"SET el.slug = '{slug}',")
        lines.append(f"    el.display_name = '{display_name}',")
        lines.append(f"    el.description = '{description}',")
        lines.append(f"    el.definition = '{definition}',")
        lines.append(f"    el.purpose = '{purpose}',")
        lines.append(f"    el.llm_context = '{llm_context}',")
        lines.append(f"    el.version = {version},")
        lines.append("    el.created_at = datetime(),")
        lines.append("    el.updated_at = datetime();")
        lines.append("")

    # Part 2: Create relations to parent Entity (HAS_L10N)
    lines.append("// -------------------------------------------------------------------")
    lines.append("// Create relations to parent Entity (HAS_L10N)")
    lines.append("// -------------------------------------------------------------------")
    lines.append("")

    for el in entityl10n_list:
        entity_key = escape_cypher(el["entity_key"])
        locale_key = escape_cypher(el["locale_key"])

        lines.append(f"MATCH (e:Entity {{key: '{entity_key}'}})")
        lines.append(f"MATCH (el:EntityL10n {{entity_key: '{entity_key}', locale_key: '{locale_key}'}})")
        lines.append("MERGE (e)-[:HAS_L10N]->(el);")
        lines.append("")

    # Part 3: Create relations to Locale (FOR_LOCALE) - single batch
    lines.append("// -------------------------------------------------------------------")
    lines.append("// Create relations to Locale (FOR_LOCALE)")
    lines.append("// -------------------------------------------------------------------")
    lines.append("")
    lines.append("MATCH (l:Locale {key: 'fr-FR'})")
    lines.append("MATCH (el:EntityL10n {locale_key: 'fr-FR'})")
    lines.append("MERGE (el)-[:FOR_LOCALE]->(l);")
    lines.append("")

    return "\n".join(lines)


if __name__ == "__main__":
    # Paths relative to monorepo root
    script_dir = Path(__file__).parent
    input_path = script_dir / "output" / "entityl10n_fr.json"
    output_path = script_dir.parent.parent / "packages" / "db" / "seed" / "11-entityl10n-fr-fr.cypher"

    # Load EntityL10n data
    print(f"Reading: {input_path}")
    with open(input_path, "r", encoding="utf-8") as f:
        entityl10n_list = json.load(f)

    print(f"Loaded {len(entityl10n_list)} EntityL10n records")

    # Generate Cypher
    cypher = generate_cypher(entityl10n_list)

    # Write output
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(cypher)

    # Report
    line_count = cypher.count("\n") + 1
    print(f"Generated Cypher for {len(entityl10n_list)} EntityL10n to {output_path}")
    print(f"Output: {line_count} lines, {len(cypher):,} bytes")
