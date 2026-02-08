#!/usr/bin/env python3
"""Map SEO keywords to EntityL10n based on pattern matching.

Task 2.2 from the SEO Keywords fr-FR Implementation Plan.

Mapping taxonomy (priority order):
1. Creation keywords -> create-qr-code (pattern: créer|générer|generator|maker)
2. Scan keywords -> scan-qr-code (pattern: scanner|scan|lire|lecteur|reader)
3. Brand keywords -> qr-code-{brand} (pattern: instagram|facebook|linkedin|whatsapp)
4. Content types -> qr-code-{type} (pattern: wifi|vcard|url|sms|email|pdf|menu)
5. Industry -> {industry} (pattern: restaurant|retail|healthcare)
6. Medium -> {medium} (pattern: carte de visite|flyer|poster)
7. Barcode -> barcode types (pattern: code barre|ean|upc|data matrix)
8. Developer -> api, developers (pattern: api|python|zxing)
9. Competitor -> competitor comparisons (pattern: qr code monkey|qr tiger)
10. How-to -> guides (pattern: comment|how to|tutorial)
"""

import json
import re
from pathlib import Path
from typing import Optional, Tuple
from collections import Counter

# Mapping rules: (pattern, entity_key, category)
# Rules are checked in order - first match wins
MAPPING_RULES: list[tuple[str, str, str]] = [
    # ══════════════════════════════════════════════════════════════════════════
    # 1. CREATION (highest priority - core product action)
    # ══════════════════════════════════════════════════════════════════════════
    (r"(créer|creer|générer|generer|generator|générateur|maker|fabriquer|faire)", "create-qr-code", "creation"),

    # ══════════════════════════════════════════════════════════════════════════
    # 2. SCANNING
    # ══════════════════════════════════════════════════════════════════════════
    (r"(scanner|scan|lire|lecteur|reader|flasher|lector|déchiffrer|décoder)", "scan-qr-code", "scanning"),

    # ══════════════════════════════════════════════════════════════════════════
    # 3. BRANDS / SOCIAL PLATFORMS (before content types)
    # ══════════════════════════════════════════════════════════════════════════
    (r"\binstagram\b", "qr-code-instagram", "brand"),
    (r"\bfacebook\b", "qr-code-facebook", "brand"),
    (r"\bwhatsapp\b", "qr-code-whatsapp", "brand"),
    (r"\blinkedin\b", "qr-code-linkedin", "brand"),
    (r"\btiktok\b", "qr-code-tiktok", "brand"),
    (r"\byoutube\b", "qr-code-youtube", "brand"),
    (r"\btwitter\b", "qr-code-twitter", "brand"),
    (r"\bspotify\b", "qr-code-spotify", "brand"),
    (r"\bsnapchat\b", "qr-code-snapchat", "brand"),
    (r"\bpinterest\b", "qr-code-pinterest", "brand"),
    (r"\btelegram\b", "qr-code-telegram", "brand"),
    (r"\bsoundcloud\b", "qr-code-soundcloud", "brand"),
    (r"\bapple\s*music\b", "qr-code-apple-music", "brand"),
    (r"\bgoogle\s*maps\b", "qr-code-google-maps", "brand"),
    (r"\bapple\s*maps\b", "qr-code-apple-maps", "brand"),
    (r"\bwaze\b", "qr-code-waze", "brand"),
    (r"\bpaypal\b", "qr-code-paypal", "brand"),
    (r"\bvenmo\b", "qr-code-venmo", "brand"),
    (r"\bbitcoin\b", "qr-code-bitcoin", "brand"),
    (r"\bethereum\b", "qr-code-ethereum", "brand"),

    # ══════════════════════════════════════════════════════════════════════════
    # 4. CONTENT TYPES (core feature categories)
    # ══════════════════════════════════════════════════════════════════════════
    (r"\bwifi\b", "qr-code-wifi", "content-type"),
    (r"\bvcard\b", "qr-code-vcard", "content-type"),
    (r"\bmecard\b", "qr-code-mecard", "content-type"),
    (r"\burl\b", "qr-code-url", "content-type"),
    (r"\bsms\b", "qr-code-sms", "content-type"),
    (r"\bemail\b", "qr-code-email", "content-type"),
    (r"\bpdf\b", "qr-code-pdf", "content-type"),
    (r"\bmenu\b", "qr-code-menu", "content-type"),
    (r"\bvideo\b", "qr-code-video", "content-type"),
    (r"\baudio\b", "qr-code-audio", "content-type"),
    (r"\blocation|localisation|géolocalisation\b", "qr-code-location", "content-type"),
    (r"\bcalendar|calendrier|agenda\b", "qr-code-calendar", "content-type"),
    (r"\bpaiement|payment\b", "qr-code-payment", "content-type"),
    (r"\btexte|text\b", "qr-code-text", "content-type"),
    (r"\bteleph|phone|appel\b", "qr-code-phone", "content-type"),
    (r"\bticket|billet\b", "qr-code-ticket", "content-type"),
    (r"\bcoupon|bon de réduction|réduction\b", "qr-code-coupon", "content-type"),
    (r"\bfichier|file\b", "qr-code-file", "content-type"),
    (r"\bavis|review\b", "qr-code-review", "content-type"),
    (r"\bapp\s*store\b", "qr-code-app-store", "content-type"),
    (r"\bplay\s*store\b", "qr-code-play-store", "content-type"),

    # ══════════════════════════════════════════════════════════════════════════
    # 5. INDUSTRIES
    # ══════════════════════════════════════════════════════════════════════════
    (r"\brestaurant\b", "restaurants", "industry"),
    (r"\bretail|commerce de détail|magasin\b", "retail", "industry"),
    (r"\bsanté|health|médical|medical\b", "healthcare", "industry"),
    (r"\béducation|education|école|university|université\b", "education", "industry"),
    (r"\bhôtel|hotel|hospitality\b", "hospitality", "industry"),
    (r"\bimmobilier|real estate\b", "real-estate", "industry"),
    (r"\bévénement|event\b", "event-management", "industry"),
    (r"\bfinance|banque|bank\b", "finance", "industry"),
    (r"\bgovernment|gouvernement|administration\b", "government", "industry"),
    (r"\blogistique|logistics|transport\b", "logistics", "industry"),
    (r"\bmanufacturing|industrie\b", "manufacturing", "industry"),
    (r"\bbeauté|beauty|cosmétique\b", "beauty", "industry"),
    (r"\bfitness|gym|sport\b", "fitness", "industry"),
    (r"\bentertainment|divertissement\b", "entertainment", "industry"),
    (r"\bconstruction|bâtiment\b", "construction", "industry"),

    # ══════════════════════════════════════════════════════════════════════════
    # 6. MEDIUMS (physical formats)
    # ══════════════════════════════════════════════════════════════════════════
    (r"\bcarte de visite|business card\b", "qr-code-business-card", "medium"),
    (r"\bflyer|tract|prospectus\b", "qr-code-flyer", "medium"),
    (r"\bposter|affiche\b", "qr-code-poster", "medium"),
    (r"\btable tent|chevalet de table\b", "qr-code-table-tent", "medium"),
    (r"\bemballage|packaging|étiquette\b", "qr-code-packaging-label", "medium"),
    (r"\bemail signature|signature email\b", "qr-code-email-signature", "medium"),
    (r"\bsticker|autocollant\b", "stickers-labels", "medium"),
    (r"\bbrochure\b", "brochures", "medium"),
    (r"\bmagazine\b", "magazines", "medium"),
    (r"\bjournal|newspaper\b", "newspapers", "medium"),
    (r"\bpresentation|présentation\b", "presentations", "medium"),
    (r"\bdocument\b", "documents", "medium"),

    # ══════════════════════════════════════════════════════════════════════════
    # 7. BARCODE TYPES
    # ══════════════════════════════════════════════════════════════════════════
    (r"\bcode.?barre|barcode\b", "barcode", "barcode"),
    (r"\bean.?13\b", "ean-13", "barcode"),
    (r"\bean.?8\b", "ean-8", "barcode"),
    (r"\bupc.?a\b", "upc-a", "barcode"),
    (r"\bupc.?e\b", "upc-e", "barcode"),
    (r"\bdata.?matrix\b", "data-matrix", "barcode"),
    (r"\bcode.?128\b", "code-128", "barcode"),
    (r"\bcode.?39\b", "code-39", "barcode"),
    (r"\baztec\b", "aztec-code", "barcode"),
    (r"\bpdf.?417\b", "pdf417", "barcode"),
    (r"\bmaxicode\b", "maxicode", "barcode"),
    (r"\bcodabar\b", "codabar", "barcode"),
    (r"\bitf.?14\b", "itf-14", "barcode"),
    (r"\bgs1\b", "gs1-128", "barcode"),

    # ══════════════════════════════════════════════════════════════════════════
    # 8. DEVELOPERS / API
    # ══════════════════════════════════════════════════════════════════════════
    (r"\bapi\b", "api", "developer"),
    (r"\bpython|javascript|java\b", "developers", "developer"),
    (r"\bzxing\b", "developers", "developer"),
    (r"\blibrairie|library|sdk\b", "developers", "developer"),
    (r"\bintégration|integration\b", "api", "developer"),
    (r"\bwebhook\b", "webhooks", "developer"),

    # ══════════════════════════════════════════════════════════════════════════
    # 9. COMPETITORS / COMPARISONS
    # ══════════════════════════════════════════════════════════════════════════
    (r"\bqr\s*monkey|monkey\s*qr\b", "qr-code-ai-vs-competitors", "competitor"),
    (r"\bqr\s*code\s*monkey\b", "qr-code-ai-vs-competitors", "competitor"),
    (r"\bqr\s*tiger\b", "qr-code-ai-vs-competitors", "competitor"),
    (r"\bqr\s*code\s*generator\s*pro\b", "qr-code-ai-vs-competitors", "competitor"),
    (r"\bbeaconstac\b", "qr-code-ai-vs-competitors", "competitor"),
    (r"\bflowcode\b", "qr-code-ai-vs-competitors", "competitor"),
    (r"\bbitly\b", "qr-code-ai-vs-competitors", "competitor"),
    (r"\bme[\-\s]*qr\b", "qr-code-ai-vs-competitors", "competitor"),
    (r"\bqr[\-\s]*io\b", "qr-code-ai-vs-competitors", "competitor"),
    (r"\bvs\b.*\b(nfc|barcode|data matrix)\b", "qr-code-vs-nfc", "comparison"),
    (r"\bqr\s*code\s*vs\b", "qr-code-vs-barcode", "comparison"),

    # ══════════════════════════════════════════════════════════════════════════
    # 10. HOW-TO / GUIDES
    # ══════════════════════════════════════════════════════════════════════════
    (r"\bcomment\b", "how-to-create-qr-code", "guide"),
    (r"\bhow to\b", "how-to-create-qr-code", "guide"),
    (r"\btutoriel|tutorial\b", "how-to-create-qr-code", "guide"),
    (r"\bguide\b", "qr-code-design-guide", "guide"),

    # ══════════════════════════════════════════════════════════════════════════
    # 11. DESIGN / CUSTOMIZATION
    # ══════════════════════════════════════════════════════════════════════════
    (r"\blogo\b", "qr-code-with-logo", "design"),
    (r"\bcouleur|color\b", "qr-code-color", "design"),
    (r"\bpersonnalis|custom\b", "custom-qr-code", "design"),
    (r"\btransparent\b", "qr-code-transparent-background", "design"),
    (r"\bart|artistique\b", "qr-code-art", "design"),
    (r"\bforme|shape\b", "qr-code-shapes", "design"),
    (r"\bfond|background\b", "qr-code-background", "design"),
    (r"\bdark\s*mode\b", "qr-code-dark-mode", "design"),
    (r"\blight\s*mode\b", "qr-code-light-mode", "design"),

    # ══════════════════════════════════════════════════════════════════════════
    # 12. FEATURES
    # ══════════════════════════════════════════════════════════════════════════
    (r"\bdynamique|dynamic\b", "dynamic-qr-code", "feature"),
    (r"\bstatique|static\b", "static-qr-code", "feature"),
    (r"\banalytics|statistiques|suivi\b", "analytics", "feature"),
    (r"\btracking|pistage\b", "track-scans", "feature"),
    (r"\btélécharger|download\b", "download-qr-code", "feature"),
    (r"\bimprimer|print\b", "print-qr-code", "feature"),
    (r"\bpartager|share\b", "share-qr-code", "feature"),
    (r"\bbulk|masse|lot\b", "bulk-creation", "feature"),
    (r"\bexpiration|expirer\b", "expiration", "feature"),
    (r"\bpassword|mot de passe\b", "password-protection", "feature"),
    (r"\bwhite.?label|marque blanche\b", "white-label", "feature"),

    # ══════════════════════════════════════════════════════════════════════════
    # 13. INTEGRATIONS
    # ══════════════════════════════════════════════════════════════════════════
    (r"\bshopify\b", "shopify-integration", "integration"),
    (r"\bwoocommerce\b", "woocommerce-integration", "integration"),
    (r"\bwordpress\b", "wordpress-integration", "integration"),
    (r"\bhubspot\b", "hubspot-integration", "integration"),
    (r"\bmailchimp\b", "mailchimp-integration", "integration"),
    (r"\bzapier\b", "zapier-integration", "integration"),
    (r"\bgoogle\s*sheets\b", "google-sheets-integration", "integration"),
    (r"\bsalesforce\b", "salesforce-integration", "integration"),
    (r"\bn8n\b", "n8n-integration", "integration"),
    (r"\bmake\b", "make-integration", "integration"),
    (r"\bslack\b", "slack-integration", "integration"),
    (r"\bnotion\b", "notion-integration", "integration"),

    # ══════════════════════════════════════════════════════════════════════════
    # 14. PRODUCTS (landing pages, etc.)
    # ══════════════════════════════════════════════════════════════════════════
    (r"\blanding\s*page|page de destination\b", "landing-page", "product"),
    (r"\blink\s*in\s*bio|lien\s*en\s*bio\b", "link-in-bio", "product"),
    (r"\bsmart\s*link|lien\s*intelligent\b", "smart-link", "product"),
    (r"\bshort\s*link|lien\s*court\b", "short-link", "product"),
    (r"\burl\s*shortener|raccourcisseur\b", "url-shortener", "product"),

    # ══════════════════════════════════════════════════════════════════════════
    # 15. GENERIC QR CODE (fallback)
    # ══════════════════════════════════════════════════════════════════════════
    (r"\bqr\s*code\s*gratuit|free qr\b", "qr-code", "generic"),
    (r"\bcode\s*qr\b", "qr-code", "generic"),  # French word order
    (r"\bqr\s*code\b", "qr-code", "generic"),
    (r"\bqr\s*codes\b", "qr-code", "generic"),  # Plural
    (r"\bqr\s*gen\b", "qr-code-generator", "creation"),  # Short for generator
    (r"\bqr\b", "qr-code", "generic"),  # Bare "qr" as last fallback
]


def find_entity_for_keyword(keyword: str, entity_keys: set) -> Tuple[Optional[str], Optional[str]]:
    """Find the best matching entity and category for a keyword.

    Returns:
        Tuple of (entity_key, category) or (None, None) if no match.
    """
    keyword_lower = keyword.lower()

    for pattern, entity_key, category in MAPPING_RULES:
        if re.search(pattern, keyword_lower, re.IGNORECASE):
            if entity_key in entity_keys:
                return entity_key, category

    return None, None


def map_keywords_to_entities(keywords: list, entityl10n_list: list) -> Tuple[list, int]:
    """Map each keyword to an EntityL10n.

    Returns:
        Tuple of (mapped_keywords_list, unmapped_count)
    """
    entity_keys = {el["entity_key"] for el in entityl10n_list}

    mapped_keywords = []

    for kw in keywords:
        entity_key, category = find_entity_for_keyword(kw["keyword"], entity_keys)

        mapped_keywords.append({
            "keyword": kw["keyword"],
            "volume": kw["volume"],
            "difficulty": kw["difficulty"],
            "cpc": kw["cpc"],
            "intent": kw["intent"],
            "parent_keyword": kw["parent_keyword"],
            "traffic_potential": kw["traffic_potential"],
            "entity_key": entity_key,
            "category": category if category else "unclassified"
        })

    unmapped_count = sum(1 for kw in mapped_keywords if kw["entity_key"] is None)
    return mapped_keywords, unmapped_count


def print_stats(mapped_keywords: list):
    """Print mapping statistics."""
    # Count by entity
    entity_counts = Counter(kw["entity_key"] for kw in mapped_keywords if kw["entity_key"])
    category_counts = Counter(kw["category"] for kw in mapped_keywords)

    print("\n" + "=" * 70)
    print("MAPPING STATISTICS")
    print("=" * 70)

    print(f"\nTotal keywords: {len(mapped_keywords)}")
    print(f"Mapped: {len(mapped_keywords) - category_counts.get('unclassified', 0)}")
    print(f"Unclassified: {category_counts.get('unclassified', 0)}")

    print("\n" + "-" * 70)
    print("BY CATEGORY")
    print("-" * 70)
    for category, count in sorted(category_counts.items(), key=lambda x: -x[1]):
        pct = count / len(mapped_keywords) * 100
        print(f"  {category:20} {count:5}  ({pct:5.1f}%)")

    print("\n" + "-" * 70)
    print("TOP 20 ENTITIES BY KEYWORD COUNT")
    print("-" * 70)
    for entity, count in entity_counts.most_common(20):
        print(f"  {entity:40} {count:5}")

    print("\n" + "-" * 70)
    print("TOP 10 UNCLASSIFIED KEYWORDS (by volume)")
    print("-" * 70)
    unclassified = sorted(
        [kw for kw in mapped_keywords if kw["entity_key"] is None],
        key=lambda x: -x["volume"]
    )[:10]
    for kw in unclassified:
        print(f"  {kw['volume']:>6}  {kw['keyword']}")


def print_samples(mapped_keywords: list):
    """Print sample mappings."""
    print("\n" + "=" * 70)
    print("SAMPLE MAPPINGS (first 5)")
    print("=" * 70)
    for kw in mapped_keywords[:5]:
        print(f"\n  Keyword: {kw['keyword']}")
        print(f"  Volume: {kw['volume']}")
        print(f"  Entity: {kw['entity_key']}")
        print(f"  Category: {kw['category']}")


if __name__ == "__main__":
    # Load data
    input_keywords_path = Path("scripts/seo-import/output/keywords_filtered.json")
    input_entities_path = Path("scripts/seo-import/output/entityl10n_fr.json")

    with open(input_keywords_path, "r") as f:
        keywords = json.load(f)

    with open(input_entities_path, "r") as f:
        entityl10n_list = json.load(f)

    print(f"Loaded {len(keywords)} keywords")
    print(f"Loaded {len(entityl10n_list)} EntityL10n")

    # Map keywords
    mapped_keywords, unmapped_count = map_keywords_to_entities(keywords, entityl10n_list)

    # Save output
    output_path = Path("scripts/seo-import/output/keywords_mapped.json")
    with open(output_path, "w") as f:
        json.dump(mapped_keywords, f, indent=2, ensure_ascii=False)

    print(f"\nSaved mappings to {output_path}")

    # Print statistics
    print_stats(mapped_keywords)
    print_samples(mapped_keywords)
