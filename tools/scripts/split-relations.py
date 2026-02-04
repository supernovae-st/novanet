#!/usr/bin/env python3
"""
Split relations.yaml into individual arc-kinds/{family}/{name}.yaml files.

Usage:
    python tools/scripts/split-relations.py

This script:
1. Reads packages/core/models/relations.yaml
2. Creates one YAML file per relation in arc-kinds/{family}/{kebab-name}.yaml
3. Skips relations that already have arc-kind files
"""

import os
import re
import yaml
from pathlib import Path

# Find monorepo root
def find_root() -> Path:
    current = Path(__file__).resolve()
    while current != current.parent:
        if (current / "pnpm-workspace.yaml").exists():
            return current
        current = current.parent
    raise RuntimeError("Could not find monorepo root")

ROOT = find_root()
RELATIONS_PATH = ROOT / "packages/core/models/relations.yaml"
ARC_KINDS_DIR = ROOT / "packages/core/models/arc-kinds"


def to_kebab_case(name: str) -> str:
    """Convert SCREAMING_SNAKE to kebab-case."""
    return name.lower().replace("_", "-")


def format_source_target(value) -> str:
    """Format source/target for YAML output."""
    if isinstance(value, list):
        return value  # Keep as list
    return value  # Keep as string


def infer_scope(source, target) -> str:
    """Infer scope based on source/target patterns."""
    # Self-referential = intra_realm
    if source == target:
        return "intra_realm"
    # Cross-realm if spanning global/project/shared
    return "intra_realm"  # Default for most


def build_cypher_pattern(rel_type: str, source, target, properties: list | None) -> str:
    """Build Cypher pattern string."""
    src_str = source if isinstance(source, str) else source[0]
    tgt_str = target if isinstance(target, str) else target[0]

    if properties:
        prop_names = ", ".join(p if isinstance(p, str) else p.get("name", p) for p in properties)
        return f"({src_str})-[:{rel_type} {{{prop_names}}}]->({tgt_str})"
    return f"({src_str})-[:{rel_type}]->({tgt_str})"


def write_arc_kind(rel: dict, output_path: Path):
    """Write a single arc-kind YAML file."""
    rel_type = rel["type"]
    family = rel["family"]
    source = rel.get("source")
    target = rel.get("target")
    cardinality = rel.get("cardinality", "many_to_many")
    llm_context = rel.get("llm_context", "")
    properties = rel.get("properties")
    inverse_of = rel.get("inverse_of")
    is_self_ref = rel.get("is_self_referential", False)

    # Build arc document
    arc = {
        "name": rel_type,
        "family": family,
        "scope": infer_scope(source, target),
        "source": format_source_target(source),
        "target": format_source_target(target),
        "cardinality": cardinality,
    }

    # Add optional fields
    if is_self_ref:
        arc["is_self_referential"] = True

    if properties:
        # Normalize properties to full format
        arc["properties"] = []
        for prop in properties:
            if isinstance(prop, str):
                arc["properties"].append({
                    "name": prop,
                    "type": "string",
                    "required": False,
                })
            else:
                arc["properties"].append(prop)

    if inverse_of:
        arc["inverse_of"] = inverse_of

    arc["llm_context"] = llm_context
    arc["cypher_pattern"] = build_cypher_pattern(rel_type, source, target, properties)

    # Build full document
    doc = {"arc": arc}

    # Create header comment
    short_desc = llm_context.split(".")[0] if llm_context else f"{rel_type} relationship"
    header = f"""\
# packages/core/models/arc-kinds/{family}/{to_kebab_case(rel_type)}.yaml
# ArcKind: {rel_type} — {short_desc}

"""

    # Write file
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "w") as f:
        f.write(header)
        yaml.dump(doc, f, default_flow_style=False, allow_unicode=True, sort_keys=False)

    return output_path


def main():
    # Load relations.yaml
    with open(RELATIONS_PATH) as f:
        data = yaml.safe_load(f)

    relations = data.get("relations", [])
    print(f"Found {len(relations)} relations in relations.yaml")

    created = 0
    skipped = 0
    errors = []

    for rel in relations:
        if "type" not in rel:
            continue

        rel_type = rel["type"]
        family = rel.get("family")

        if not family:
            errors.append(f"{rel_type}: missing family")
            continue

        # Determine output path
        filename = f"{to_kebab_case(rel_type)}.yaml"
        output_path = ARC_KINDS_DIR / family / filename

        # Skip if already exists
        if output_path.exists():
            print(f"  SKIP {family}/{filename} (already exists)")
            skipped += 1
            continue

        try:
            write_arc_kind(rel, output_path)
            print(f"  CREATE {family}/{filename}")
            created += 1
        except Exception as e:
            errors.append(f"{rel_type}: {e}")

    print(f"\nSummary:")
    print(f"  Created: {created}")
    print(f"  Skipped: {skipped}")
    print(f"  Errors: {len(errors)}")

    if errors:
        print("\nErrors:")
        for e in errors:
            print(f"  - {e}")


if __name__ == "__main__":
    main()
