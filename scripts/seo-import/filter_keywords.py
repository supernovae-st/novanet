#!/usr/bin/env python3
"""Filter SEO keywords from Ahrefs CSV export.

Filters:
- Volume >= 10 (adjusted from 50 for better coverage)
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

    keywords = filter_keywords(csv_path, min_volume=10)

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
