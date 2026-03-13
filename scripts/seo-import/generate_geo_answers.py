#!/usr/bin/env python3
"""
Generate GEO answers for GEOQuery nodes using template-based placeholders.

Supports ALL locales dynamically from geo_queries.json.

Usage:
  python3 scripts/seo-import/generate_geo_answers.py
"""

import json
from pathlib import Path

OUTPUT_DIR = Path(__file__).parent / 'output'
GEO_QUERIES_PATH = OUTPUT_DIR / 'geo_queries.json'
GEO_ANSWERS_PATH = OUTPUT_DIR / 'geo_answers.json'


def generate_answer_locally(query):
    """Generate a placeholder answer for a GEO query.

    In production, these would come from Perplexity/GPT/Gemini.
    For seed purposes, we generate analytical placeholders.
    """
    value = query['value']
    qt = query['query_type']
    locale = query['locale_key']
    locale_label = locale

    domain = 'QR code technology'

    if qt == 'how_to':
        answer = (
            f"To address this query about {domain}, users typically need step-by-step guidance. "
            f"The most common approach involves using a QR code generator tool. "
            f"Key considerations include choosing between static and dynamic QR codes, "
            f"selecting the right encoding format, and ensuring the output meets minimum "
            f"size requirements for reliable scanning."
        )
    elif qt == 'comparison':
        answer = (
            f"When comparing options in {domain}, the main factors to consider are: "
            f"functionality (static vs dynamic), customization options (logos, colors), "
            f"analytics capabilities (scan tracking, geographic data), pricing models "
            f"(freemium vs subscription), and integration options (API, bulk generation). "
            f"The best choice depends on use case and scale requirements."
        )
    elif qt == 'recommendation':
        answer = (
            f"For {domain} tools, the most recommended options include specialized "
            f"generators that offer customization, analytics, and dynamic capabilities. "
            f"Key features to look for: custom design options, scan analytics, "
            f"dynamic link updating, bulk generation, and API access. "
            f"Free tiers are available from most providers for basic usage."
        )
    elif qt == 'factual':
        answer = (
            f"Regarding {domain}: QR codes (Quick Response codes) are two-dimensional "
            f"barcodes that can store URLs, text, contact info, and other data types. "
            f"They were invented in 1994 by Denso Wave for automotive tracking. "
            f"Modern QR codes can store up to 7,089 numeric characters or 4,296 "
            f"alphanumeric characters with built-in error correction."
        )
    else:  # informational
        answer = (
            f"This {locale_label} query relates to {domain}. "
            f"QR codes are widely used for contactless payments, marketing campaigns, "
            f"restaurant menus, product authentication, and digital business cards. "
            f"The technology continues to evolve with AI-enhanced designs, "
            f"dynamic linking capabilities, and advanced analytics."
        )

    return {
        'engine': 'seed-template',
        'engine_version': 'v1.0-deterministic',
        'answer_text': answer,
        'cited_domains': ['qrcode-ai.com'],
        'brand_mentions': ['QR Code AI'],
        'relevance_score': 75.0,
        'ai_visibility_score': 50.0,
        'share_of_voice': 0.15,
        'position_in_answer': 1,
        'word_count': len(answer.split()),
    }


def main():
    with open(GEO_QUERIES_PATH) as f:
        queries = json.load(f)

    print(f'Loaded {len(queries)} GEO queries')

    # Generate local answers as seed data
    answers = []
    for q in queries:
        answer_data = generate_answer_locally(q)
        answer_key = f'geo-answer:{q["key"].replace("geo:", "")}:seed-template'

        answers.append({
            'key': answer_key,
            'query_key': q['key'],
            'query_value': q['value'],
            'locale_key': q['locale_key'],
            **answer_data,
        })

    # Write answers
    GEO_ANSWERS_PATH.write_text(
        json.dumps(answers, indent=2, ensure_ascii=False),
        encoding='utf-8'
    )

    print(f'Generated {len(answers)} GEO answers')
    print(f'Output: {GEO_ANSWERS_PATH}')

    # Stats
    from collections import Counter
    locales = Counter(a['locale_key'] for a in answers)
    print(f'\nAnswers per locale:')
    for locale, count in sorted(locales.items()):
        print(f'  {locale}: {count}')

    print(f'\nSample answers:')
    for a in answers[:3]:
        print(f'  {a["query_value"]}')
        print(f'    -> {a["answer_text"][:100]}...')
        print()


if __name__ == '__main__':
    main()
