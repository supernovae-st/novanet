#!/usr/bin/env python3
"""
NovaNet v0.20.0 YAML Migration Script

Replaces llm_context with triggers[] across ALL YAML model files.

Changes:
  1. Node-class files: Replace llm_context property definition → triggers definition
  2. Node-class files: Replace llm_context example values → triggers arrays
  3. Node-class files: Remove llm_hints block (EntityNative only)
  4. Arc-class files: Replace llm_context multi-line block → triggers array
  5. Realm/Layer/Arc-family files: Replace llm_context block → triggers array
  6. Template files: Update _standard-properties-template.yaml

Usage:
  python3 scripts/migrate-yaml-v020.py [--dry-run]

DELETE THIS SCRIPT AFTER MIGRATION.
"""

import json
import os
import re
import sys
from pathlib import Path

MODELS_DIR = Path(__file__).parent.parent / "packages" / "core" / "models"
DRY_RUN = "--dry-run" in sys.argv

stats = {
    "files_scanned": 0,
    "files_modified": 0,
    "llm_context_removed": 0,
    "triggers_added": 0,
    "llm_hints_removed": 0,
    "errors": [],
}


# ═══════════════════════════════════════════════════════════════════════════════
# TRIGGER EXTRACTION
# ═══════════════════════════════════════════════════════════════════════════════

def extract_triggers_from_json(value: str) -> list[str]:
    """Extract triggers from JSON format: '{"use": "...", "triggers": [...]}'"""
    try:
        data = json.loads(value)
        return [t.lower().strip() for t in data.get("triggers", []) if t.strip()]
    except (json.JSONDecodeError, TypeError):
        return []


def extract_triggers_from_adr027(text: str) -> list[str]:
    """Extract triggers from ADR-027 format: TRIGGERS: "kw1", "kw2"."""
    match = re.search(
        r'TRIGGERS?:\s*(.+?)(?:\.\s*$|\n|$|(?=\s*NOT:)|(?=\s*RELATES:))',
        text, re.IGNORECASE | re.DOTALL
    )
    if match:
        raw = match.group(1).strip().rstrip(".")
        raw = raw.replace('"', '').replace("'", "")
        return [t.strip().lower() for t in raw.split(",") if t.strip()]
    return []


def extract_triggers(value: str) -> list[str]:
    """Auto-detect format and extract triggers."""
    if not value:
        return []
    text = value.strip()
    if text.startswith("{"):
        triggers = extract_triggers_from_json(text)
        if triggers:
            return triggers[:10]
    if re.search(r'\bTRIGGERS?:', text, re.IGNORECASE):
        return extract_triggers_from_adr027(text)[:10]
    return []


def format_triggers_yaml(triggers: list[str], indent: str) -> str:
    """Format triggers as YAML array on one line."""
    quoted = ", ".join(f'"{t}"' for t in triggers)
    return f"{indent}triggers: [{quoted}]"


# ═══════════════════════════════════════════════════════════════════════════════
# LINE-BASED BLOCK OPERATIONS
# ═══════════════════════════════════════════════════════════════════════════════

def find_block(lines: list[str], key: str, start_from: int = 0) -> tuple[int, int, int]:
    """
    Find a YAML block by key name. Returns (start_line, end_line, indent_level).
    end_line is exclusive. Returns (-1, -1, -1) if not found.
    """
    for i in range(start_from, len(lines)):
        stripped = lines[i].lstrip()
        if stripped.startswith(f"{key}:"):
            # Found the key
            line = lines[i]
            indent = len(line) - len(line.lstrip())
            # Find end of block: next line at same or lower indent level
            end = i + 1
            while end < len(lines):
                next_line = lines[end]
                # Skip empty lines and comments
                if not next_line.strip() or next_line.strip().startswith('#'):
                    end += 1
                    continue
                next_indent = len(next_line) - len(next_line.lstrip())
                if next_indent <= indent:
                    break
                end += 1
            return (i, end, indent)
    return (-1, -1, -1)


def extract_block_value(lines: list[str], start: int, end: int) -> str:
    """Extract the value from a YAML block as a single string."""
    first_line = lines[start]
    # Check for inline value: "key: value" or "key: 'value'" or "key: |"
    match = re.match(r'\s*\w+:\s*(.*)', first_line)
    if match:
        value = match.group(1).strip()
        if value == '|':
            # Multi-line block scalar
            content_lines = []
            for i in range(start + 1, end):
                line = lines[i]
                if line.strip():
                    content_lines.append(line.strip())
            return '\n'.join(content_lines)
        elif value.startswith("'") and value.endswith("'"):
            return value[1:-1]
        elif value.startswith('"') and value.endswith('"'):
            return value[1:-1]
        else:
            return value
    return ""


# ═══════════════════════════════════════════════════════════════════════════════
# FILE PROCESSORS
# ═══════════════════════════════════════════════════════════════════════════════

def process_node_class_file(filepath: Path) -> bool:
    """Process a node-class YAML file.

    Node-class files have llm_context in two places:
    1. Property definition in standard_properties: (type, required, description, example)
    2. Example value in example.data:
    Plus optionally llm_hints: in EntityNative
    """
    content = filepath.read_text(encoding="utf-8")
    lines = content.splitlines(keepends=True)
    modified = False
    removals = []  # (start, end, replacement_lines)

    # --- 1. Replace llm_context property definition in standard_properties ---
    # Find standard_properties section first
    sp_start, sp_end, sp_indent = find_block(lines, "standard_properties")
    if sp_start >= 0:
        # Find llm_context within standard_properties
        lc_start, lc_end, lc_indent = find_block(lines, "llm_context", sp_start + 1)
        if sp_start < lc_start < sp_end:
            # This is the property definition — replace with triggers
            indent = " " * lc_indent
            replacement = (
                f"{indent}triggers:\n"
                f"{indent}  type: string[]\n"
                f"{indent}  required: true\n"
                f"{indent}  indexed: true\n"
                f"{indent}  description: Machine-readable routing keywords (max 10, lowercase, English)\n"
            )
            removals.append((lc_start, lc_end, replacement))
            modified = True

    # --- 2. Replace llm_context value in example.data ---
    ex_start, ex_end, ex_indent = find_block(lines, "example")
    if ex_start >= 0:
        # Find data: within example
        data_start, data_end, data_indent = find_block(lines, "data", ex_start + 1)
        if data_start >= 0:
            # Find llm_context within example.data
            for i in range(data_start + 1, min(data_end, len(lines))):
                line = lines[i]
                stripped = line.lstrip()
                if stripped.startswith("llm_context:"):
                    indent = " " * (len(line) - len(stripped))
                    # Extract the value
                    value_match = re.match(r"\s*llm_context:\s*'(.+?)'", line)
                    if not value_match:
                        value_match = re.match(r'\s*llm_context:\s*"(.+?)"', line)
                    if value_match:
                        value = value_match.group(1)
                        triggers = extract_triggers(value)
                        if triggers:
                            replacement = format_triggers_yaml(triggers, indent) + "\n"
                            removals.append((i, i + 1, replacement))
                            modified = True
                    else:
                        # Multi-line or block value
                        block_end = i + 1
                        while block_end < data_end:
                            next_line = lines[block_end]
                            if not next_line.strip():
                                block_end += 1
                                continue
                            next_indent = len(next_line) - len(next_line.lstrip())
                            if next_indent <= len(indent):
                                break
                            block_end += 1
                        value = extract_block_value(lines, i, block_end)
                        triggers = extract_triggers(value)
                        if triggers:
                            replacement = format_triggers_yaml(triggers, indent) + "\n"
                            removals.append((i, block_end, replacement))
                            modified = True
                    break

    # --- 3. Remove llm_hints from EntityNative ---
    if "entity-native" in str(filepath).lower():
        lh_start, lh_end, lh_indent = find_block(lines, "llm_hints")
        if lh_start >= 0:
            removals.append((lh_start, lh_end, ""))
            stats["llm_hints_removed"] += 1
            modified = True

    # Apply removals in reverse order (to preserve line numbers)
    if removals and modified:
        removals.sort(key=lambda x: x[0], reverse=True)
        for start, end, replacement in removals:
            if replacement:
                lines[start:end] = [replacement]
            else:
                lines[start:end] = []

        if not DRY_RUN:
            filepath.write_text("".join(lines), encoding="utf-8")
        stats["files_modified"] += 1
        stats["llm_context_removed"] += 1
        stats["triggers_added"] += 1

    return modified


def process_arc_class_file(filepath: Path) -> bool:
    """Process an arc-class YAML file.

    Arc-class files have llm_context as a direct property under arc:
    with multi-line (|) or inline format.
    """
    content = filepath.read_text(encoding="utf-8")
    lines = content.splitlines(keepends=True)

    # Find llm_context block
    lc_start, lc_end, lc_indent = find_block(lines, "llm_context")
    if lc_start < 0:
        return False

    # Extract value and get triggers
    value = extract_block_value(lines, lc_start, lc_end)
    triggers = extract_triggers(value)

    if not triggers:
        # Fallback: no triggers extractable — keep as-is with warning
        stats["errors"].append(f"{filepath.name}: no triggers extractable from llm_context")
        return False

    # Replace block with triggers array
    indent = " " * lc_indent
    replacement = format_triggers_yaml(triggers, indent) + "\n"

    # Handle trailing empty lines within the block
    # If the line after the block is empty, preserve one blank line
    lines[lc_start:lc_end] = [replacement]

    if not DRY_RUN:
        filepath.write_text("".join(lines), encoding="utf-8")
    stats["files_modified"] += 1
    stats["llm_context_removed"] += 1
    stats["triggers_added"] += 1
    return True


def process_taxonomy_file(filepath: Path) -> bool:
    """Process realm, layer, or arc-family YAML files.

    These have llm_context as a direct property under realm:/layer:/arc_family:
    """
    content = filepath.read_text(encoding="utf-8")
    lines = content.splitlines(keepends=True)

    # Find llm_context block
    lc_start, lc_end, lc_indent = find_block(lines, "llm_context")
    if lc_start < 0:
        return False

    value = extract_block_value(lines, lc_start, lc_end)
    triggers = extract_triggers(value)

    if not triggers:
        stats["errors"].append(f"{filepath.name}: no triggers extractable")
        return False

    indent = " " * lc_indent
    replacement = format_triggers_yaml(triggers, indent) + "\n"
    lines[lc_start:lc_end] = [replacement]

    if not DRY_RUN:
        filepath.write_text("".join(lines), encoding="utf-8")
    stats["files_modified"] += 1
    stats["llm_context_removed"] += 1
    stats["triggers_added"] += 1
    return True


def process_standard_template(filepath: Path) -> bool:
    """Process _standard-properties-template.yaml (DATA nodes template).

    Replace llm_context definition block with triggers definition.
    Update header comments.
    """
    content = filepath.read_text(encoding="utf-8")
    lines = content.splitlines(keepends=True)
    modified = False

    # Update header comment: position 5
    for i, line in enumerate(lines):
        if "5. llm_context" in line:
            lines[i] = line.replace("5. llm_context", "5. triggers")
            modified = True
        if "5. LLM_CONTEXT" in line:
            lines[i] = line.replace("5. LLM_CONTEXT", "5. TRIGGERS")
            modified = True

    # Find and replace llm_context block
    lc_start, lc_end, lc_indent = find_block(lines, "llm_context")
    if lc_start >= 0:
        indent = " " * lc_indent
        replacement = (
            f"{indent}# ─────────────────────────────────────────────────────────────────────────────\n"
            f"{indent}# 5. TRIGGERS - Machine-readable routing keywords\n"
            f"{indent}# ─────────────────────────────────────────────────────────────────────────────\n"
            f"{indent}triggers:\n"
            f"{indent}  type: string[]\n"
            f"{indent}  required: true\n"
            f"{indent}  indexed: true\n"
            f"{indent}  description: |\n"
            f"{indent}    Machine-readable routing keywords for graph search and spreading activation.\n"
            f"{indent}    Max 10 items. Lowercase. Always English (even on *Native nodes).\n"
            f"{indent}    Used by novanet_search(mode=\"triggers\") and trigger-boosted spreading.\n"
            f"{indent}  examples:\n"
            f'{indent}    - ["qr", "qr code", "scan", "barcode"]\n'
            f'{indent}    - ["pricing", "tarifs", "plans", "subscription"]\n'
        )
        # Find the comment block above (# 5. LLM_CONTEXT header)
        comment_start = lc_start
        while comment_start > 0 and lines[comment_start - 1].strip().startswith('#'):
            if "5." in lines[comment_start - 1] or "LLM_CONTEXT" in lines[comment_start - 1] or "────" in lines[comment_start - 1]:
                comment_start -= 1
            else:
                break

        lines[comment_start:lc_end] = [replacement]
        modified = True

    # Update version references
    result = "".join(lines)
    result = result.replace("v0.19.0", "v0.20.0")
    lines = result.splitlines(keepends=True)

    if modified:
        if not DRY_RUN:
            filepath.write_text("".join(lines), encoding="utf-8")
        stats["files_modified"] += 1
        stats["llm_context_removed"] += 1
        stats["triggers_added"] += 1

    return modified


# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════

def main():
    resolved = MODELS_DIR.resolve()
    if not resolved.exists():
        print(f"ERROR: Models directory not found at {MODELS_DIR} (resolved: {resolved})")
        sys.exit(1)

    print(f"{'[DRY RUN] ' if DRY_RUN else ''}NovaNet v0.20.0 YAML Migration")
    print(f"Models directory: {resolved}")
    print("=" * 70)

    # 0. Process _standard-properties-template.yaml (DATA template)
    std_template = MODELS_DIR / "node-classes" / "_standard-properties-template.yaml"
    if std_template.exists():
        print("\n--- Standard Properties Template ---")
        stats["files_scanned"] += 1
        try:
            result = process_standard_template(std_template)
            print(f"  {'✅' if result else '⏭ '} {std_template.relative_to(MODELS_DIR)}")
        except Exception as e:
            stats["errors"].append(f"{std_template.name}: {e}")
            print(f"  ❌ {std_template.name}: {e}")

    # 1. Process node-class files
    node_classes_dir = MODELS_DIR / "node-classes"
    if node_classes_dir.exists():
        print("\n--- Node Classes ---")
        for yaml_file in sorted(node_classes_dir.rglob("*.yaml")):
            if yaml_file.name.startswith("_"):
                continue
            stats["files_scanned"] += 1
            try:
                result = process_node_class_file(yaml_file)
                rel = yaml_file.relative_to(MODELS_DIR)
                print(f"  {'✅' if result else '⏭ '} {rel}")
            except Exception as e:
                stats["errors"].append(f"{yaml_file.relative_to(MODELS_DIR)}: {e}")
                print(f"  ❌ {yaml_file.relative_to(MODELS_DIR)}: {e}")

    # 2. Process arc-class files
    arc_classes_dir = MODELS_DIR / "arc-classes"
    if arc_classes_dir.exists():
        print("\n--- Arc Classes ---")
        for yaml_file in sorted(arc_classes_dir.rglob("*.yaml")):
            stats["files_scanned"] += 1
            try:
                result = process_arc_class_file(yaml_file)
                rel = yaml_file.relative_to(MODELS_DIR)
                print(f"  {'✅' if result else '⏭ '} {rel}")
            except Exception as e:
                stats["errors"].append(f"{yaml_file.relative_to(MODELS_DIR)}: {e}")
                print(f"  ❌ {yaml_file.relative_to(MODELS_DIR)}: {e}")

    # 3. Process realm files
    for subdir, label in [("realms", "Realms"), ("layers", "Layers"), ("arc-families", "Arc Families")]:
        target_dir = MODELS_DIR / subdir
        if target_dir.exists():
            print(f"\n--- {label} ---")
            for yaml_file in sorted(target_dir.glob("*.yaml")):
                if yaml_file.name.startswith("_"):
                    continue
                stats["files_scanned"] += 1
                try:
                    result = process_taxonomy_file(yaml_file)
                    rel = yaml_file.relative_to(MODELS_DIR)
                    print(f"  {'✅' if result else '⏭ '} {rel}")
                except Exception as e:
                    stats["errors"].append(f"{yaml_file.relative_to(MODELS_DIR)}: {e}")
                    print(f"  ❌ {yaml_file.relative_to(MODELS_DIR)}: {e}")

    # Summary
    print("\n" + "=" * 70)
    print("MIGRATION SUMMARY")
    print("=" * 70)
    print(f"  Files scanned:       {stats['files_scanned']}")
    print(f"  Files modified:      {stats['files_modified']}")
    print(f"  llm_context removed: {stats['llm_context_removed']}")
    print(f"  triggers added:      {stats['triggers_added']}")
    print(f"  llm_hints removed:   {stats['llm_hints_removed']}")
    if stats["errors"]:
        print(f"\n  ⚠️  WARNINGS ({len(stats['errors'])}):")
        for err in stats["errors"]:
            print(f"    - {err}")
    else:
        print(f"\n  ✅ Zero errors")

    # Post-migration verification
    print("\n--- Post-migration verification ---")
    remaining = []
    for yaml_file in sorted(resolved.rglob("*.yaml")):
        if yaml_file.name.startswith("_"):
            continue
        text = yaml_file.read_text(encoding="utf-8")
        for lineno, line in enumerate(text.splitlines(), 1):
            stripped = line.strip()
            if stripped.startswith("#"):
                continue
            if "llm_context" in stripped and "llm_context" not in "# llm_context":
                remaining.append((yaml_file.relative_to(resolved), lineno, stripped[:80]))
                break

    if not remaining:
        print("  ✅ No remaining llm_context references (excluding templates and comments)")
    else:
        print(f"  ⚠️  {len(remaining)} files still have llm_context references:")
        for rel, lineno, text in remaining[:20]:
            print(f"    L{lineno}: {rel}")
        if len(remaining) > 20:
            print(f"    ... and {len(remaining) - 20} more")

    # Check for remaining llm_hints
    hints_remaining = []
    for yaml_file in sorted(resolved.rglob("*.yaml")):
        text = yaml_file.read_text(encoding="utf-8")
        for lineno, line in enumerate(text.splitlines(), 1):
            stripped = line.strip()
            if stripped.startswith("#"):
                continue
            if "llm_hints:" in stripped:
                hints_remaining.append((yaml_file.relative_to(resolved), lineno))
                break

    if hints_remaining:
        print(f"\n  ⚠️  {len(hints_remaining)} files still have llm_hints:")
        for rel, lineno in hints_remaining:
            print(f"    L{lineno}: {rel}")

    if DRY_RUN:
        print("\n[DRY RUN] No files were modified. Run without --dry-run to apply.")


if __name__ == "__main__":
    main()
