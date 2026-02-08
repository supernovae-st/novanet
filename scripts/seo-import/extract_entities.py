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
