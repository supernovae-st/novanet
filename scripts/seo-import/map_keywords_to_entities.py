#!/usr/bin/env python3
"""Map SEO keywords to EntityNative based on pattern matching.

Mapping rules by category:
1. Creation keywords -> create-qr-code, qr-code-generator
2. Scan keywords -> scan-qr-code, qr-code-scanner
3. Content type keywords -> qr-code-{type}
4. Brand keywords -> {brand}
5. Industry keywords -> {industry}
6. Feature keywords -> {feature}
"""

import json
import re
from pathlib import Path
from typing import Optional

# Mapping patterns: regex -> entity_key
MAPPING_RULES = [
    # Creation (high priority)
    (r"(créer|creer|générer|generer|create|generator|maker|fabriquer)", "create-qr-code"),

    # Scanning
    (r"(scanner|scan|lire|lecteur|reader|flasher|lector)", "scan-qr-code"),

    # Content types
    (r"\bwifi\b", "qr-code-wifi"),
    (r"\bvcard\b", "qr-code-vcard"),
    (r"\burl\b", "qr-code-url"),
    (r"\bsms\b", "qr-code-sms"),
    (r"\bemail\b", "qr-code-email"),
    (r"\bpdf\b", "qr-code-pdf"),
    (r"\bmenu\b", "qr-code-menu"),
    (r"\bvideo\b", "qr-code-video"),
    (r"\baudio\b", "qr-code-audio"),
    (r"\blocation\b", "qr-code-location"),
    (r"\bcalendar\b", "qr-code-calendar"),
    (r"\bpayment|paiement\b", "qr-code-payment"),

    # Social/Brands
    (r"\binstagram\b", "instagram"),
    (r"\bfacebook\b", "facebook"),
    (r"\bwhatsapp\b", "whatsapp"),
    (r"\blinkedin\b", "linkedin"),
    (r"\btiktok\b", "tiktok"),
    (r"\byoutube\b", "youtube"),
    (r"\btwitter\b", "twitter"),
    (r"\bspotify\b", "spotify"),
    (r"\bsnapchat\b", "snapchat"),
    (r"\bpinterest\b", "pinterest"),
    (r"\btelegram\b", "telegram"),

    # Design
    (r"(logo|avec logo)", "qr-code-with-logo"),
    (r"(couleur|color)", "qr-code-color"),
    (r"(personnalis|custom)", "custom-qr-code"),
    (r"(transparent)", "qr-code-transparent-background"),
    (r"(art|artistique)", "qr-code-art"),

    # Industries
    (r"\brestaurant\b", "restaurants"),
    (r"\bretail\b", "retail"),
    (r"\bsanté|health\b", "healthcare"),
    (r"\béducation|education\b", "education"),
    (r"\bhôtel|hotel\b", "hospitality"),
    (r"\bimmobilier|real estate\b", "real-estate"),

    # Concepts
    (r"\bdynamique|dynamic\b", "dynamic-qr-code"),
    (r"\bstatique|static\b", "static-qr-code"),
    (r"\bgratuit|free\b", "qr-code"),  # Maps to main entity

    # Barcode types
    (r"\bcode.?barre|barcode\b", "barcode"),
    (r"\bean.?13\b", "ean-13"),
    (r"\bupc\b", "upc-a"),
    (r"\bdata.?matrix\b", "data-matrix"),

    # Actions
    (r"\btélécharger|download\b", "download-qr-code"),
    (r"\bimprimer|print\b", "print-qr-code"),

    # Default fallback
    (r"\bqr\s*code\b", "qr-code"),
]

def find_entity_for_keyword(keyword: str, entity_keys: set) -> Optional[str]:
    """Find the best matching entity for a keyword."""
    keyword_lower = keyword.lower()

    for pattern, entity_key in MAPPING_RULES:
        if re.search(pattern, keyword_lower, re.IGNORECASE):
            if entity_key in entity_keys:
                return entity_key

    return None

def map_keywords_to_entities(keywords: list, entitycontent_list: list) -> tuple:
    """Map each keyword to an EntityNative."""
    entity_keys = {el["entity_key"] for el in entitycontent_list}

    mappings = []
    unmapped_count = 0

    for kw in keywords:
        entity_key = find_entity_for_keyword(kw["value"], entity_keys)

        if entity_key:
            mappings.append({
                "keyword": kw["value"],
                "volume": kw["volume"],
                "difficulty": kw["difficulty"],
                "cpc": kw["cpc"],
                "intent": kw["intent"],
                "entity_key": entity_key,
                "locale_key": "fr-FR"
            })
        else:
            unmapped_count += 1

    return mappings, unmapped_count

if __name__ == "__main__":
    # Load data
    with open("scripts/seo-import/output/keywords_filtered.json", "r") as f:
        keywords = json.load(f)

    with open("scripts/seo-import/output/entitycontent_fr.json", "r") as f:
        entitycontent_list = json.load(f)

    # Map keywords
    mappings, unmapped = map_keywords_to_entities(keywords, entitycontent_list)

    # Save output
    output_path = Path("scripts/seo-import/output/keyword_mappings.json")
    with open(output_path, "w") as f:
        json.dump(mappings, f, indent=2, ensure_ascii=False)

    print(f"Mapped {len(mappings)} keywords to entities")
    print(f"Unmapped: {unmapped} keywords")

    # Stats by entity
    from collections import Counter
    entity_counts = Counter(m["entity_key"] for m in mappings)
    print(f"\nTop 10 entities by keyword count:")
    for entity, count in entity_counts.most_common(10):
        print(f"  {count:>5}  {entity}")
