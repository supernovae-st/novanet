#!/usr/bin/env python3
"""Filter SEO keywords from Ahrefs CSV export.

Filters:
- Volume >= 50 (removes low-volume noise and keywords without data)
- French language keywords only (removes Japanese, Chinese, Korean, Spanish)
- Remove gaming keywords (brawl stars, pokemon, clash royale, etc.)
- Remove spam/nonsense
- Remove temporary events (jo paris, pass sanitaire, covid, vaccination)

Input:  docs/assets/keywods/fr-fr_qr/seo/google_fr_qr_matching-terms_2026-02-06_16-39-15.csv
Output: scripts/seo-import/output/keywords_filtered.json
"""

import json
import re
from pathlib import Path

import pandas as pd


# Keywords to exclude (gaming)
GAMING_PATTERNS = [
    r"brawl\s*stars",
    r"pokemon",
    r"yo[\-\s]?kai",
    r"clash\s*royale",
    r"fortnite",
    r"minecraft",
    r"roblox",
    r"animal\s*crossing",
    r"nintendo",
    r"zelda",
    r"mario",
    r"genshin",
    r"league\s*of\s*legends",
    r"lol\s+qr",
]

# Temporary events
EVENT_PATTERNS = [
    r"pass\s*sanitaire",
    r"jo\s*paris",
    r"covid",
    r"vaccination",
    r"vaccin",
    r"confinement",
]

# Non-French language patterns (Japanese, Chinese, Korean, Spanish)
NON_FRENCH_PATTERNS = [
    r"[\u3040-\u30ff]",  # Japanese Hiragana and Katakana
    r"[\u4e00-\u9fff]",  # Chinese characters
    r"[\uac00-\ud7af]",  # Korean Hangul
    r"código\s*qr",      # Spanish
    r"crear\s*qr",       # Spanish
    r"generar\s*código", # Spanish
]

# Spam/nonsense patterns
SPAM_PATTERNS = [
    r"^[a-z0-9]{50,}$",           # Very long gibberish strings
    r"http[s]?://[^\s]{100,}",    # Very long URLs
    r"\.com\.br\b",               # Portuguese domains
    r"\.com\.mx\b",               # Mexican domains
    r"\btest\s*test\b",           # Test strings
]


def is_excluded(keyword: str) -> tuple[bool, str]:
    """Check if keyword should be excluded.

    Returns: (should_exclude, reason)
    """
    keyword_lower = keyword.lower()

    # Check gaming
    for pattern in GAMING_PATTERNS:
        if re.search(pattern, keyword_lower, re.IGNORECASE):
            return True, "gaming"

    # Check events
    for pattern in EVENT_PATTERNS:
        if re.search(pattern, keyword_lower, re.IGNORECASE):
            return True, "event"

    # Check non-French
    for pattern in NON_FRENCH_PATTERNS:
        if re.search(pattern, keyword, re.IGNORECASE):  # Case-sensitive for Unicode
            return True, "non_french"

    # Check spam
    for pattern in SPAM_PATTERNS:
        if re.search(pattern, keyword, re.IGNORECASE):
            return True, "spam"

    return False, ""


def filter_keywords(csv_path: Path, min_volume: int = 50) -> tuple[list, dict]:
    """Filter keywords from CSV.

    Returns: (filtered_keywords, stats)
    """
    # Read UTF-16 LE encoded file
    df = pd.read_csv(csv_path, encoding='utf-16-le', sep='\t', low_memory=False)

    total_count = len(df)

    # Fill NaN volumes with 0
    df['Volume'] = df['Volume'].fillna(0).astype(int)
    df['Difficulty'] = df['Difficulty'].fillna(0).astype(int)
    df['CPC'] = df['CPC'].fillna(0.0)
    df['Traffic potential'] = df['Traffic potential'].fillna(0).astype(int)
    df['Intents'] = df['Intents'].fillna('')
    df['Parent Keyword'] = df['Parent Keyword'].fillna('')

    # Stats tracking
    stats = {
        "total": total_count,
        "volume_filtered": 0,
        "gaming_filtered": 0,
        "event_filtered": 0,
        "non_french_filtered": 0,
        "spam_filtered": 0,
        "kept": 0,
    }

    keywords = []

    for _, row in df.iterrows():
        keyword = str(row['Keyword']).strip()
        volume = int(row['Volume'])

        # Filter by volume
        if volume < min_volume:
            stats["volume_filtered"] += 1
            continue

        # Check exclusion patterns
        excluded, reason = is_excluded(keyword)
        if excluded:
            stats[f"{reason}_filtered"] += 1
            continue

        # Empty keyword check
        if not keyword:
            continue

        keywords.append({
            "keyword": keyword,
            "volume": volume,
            "difficulty": int(row['Difficulty']),
            "cpc": float(row['CPC']),
            "intent": str(row['Intents']),
            "parent_keyword": str(row['Parent Keyword']),
            "traffic_potential": int(row['Traffic potential']),
        })

    stats["kept"] = len(keywords)

    return keywords, stats


def main():
    csv_path = Path("docs/assets/keywods/fr-fr_qr/seo/google_fr_qr_matching-terms_2026-02-06_16-39-15.csv")

    print(f"Reading: {csv_path}")

    keywords, stats = filter_keywords(csv_path, min_volume=50)

    # Sort by volume descending
    keywords.sort(key=lambda x: x["volume"], reverse=True)

    # Save output
    output_path = Path("scripts/seo-import/output/keywords_filtered.json")
    output_path.parent.mkdir(parents=True, exist_ok=True)

    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(keywords, f, indent=2, ensure_ascii=False)

    # Print statistics
    print(f"\n{'='*60}")
    print(f"FILTERING STATISTICS")
    print(f"{'='*60}")
    print(f"Total keywords in CSV:     {stats['total']:>7,}")
    print(f"{'─'*60}")
    print(f"Filtered by volume (<50):  {stats['volume_filtered']:>7,}")
    print(f"Filtered by gaming:        {stats['gaming_filtered']:>7,}")
    print(f"Filtered by events:        {stats['event_filtered']:>7,}")
    print(f"Filtered by non-French:    {stats['non_french_filtered']:>7,}")
    print(f"Filtered by spam:          {stats['spam_filtered']:>7,}")
    print(f"{'─'*60}")
    print(f"KEPT:                      {stats['kept']:>7,}")
    print(f"{'='*60}")

    print(f"\nOutput: {output_path}")
    print(f"\nTop 10 keywords by volume:")
    for i, kw in enumerate(keywords[:10], 1):
        print(f"  {i:>2}. {kw['volume']:>6,}  {kw['keyword']}")


if __name__ == "__main__":
    main()
