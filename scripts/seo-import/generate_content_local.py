#!/usr/bin/env python3
"""
Generate LLM `content` field for SEOKeyword nodes using deterministic templates.

No API key needed. Builds analytical descriptions from available metadata
(volume, difficulty, intent, traffic_potential, trend).

Usage:
  python3 scripts/seo-import/generate_content_local.py
"""

import json
from pathlib import Path

INPUT_PATH = Path(__file__).parent / 'output' / 'content_batch.json'
OUTPUT_PATH = Path(__file__).parent / 'output' / 'content_generated.json'


def volume_tier(vol):
    if vol >= 100000:
        return 'extremely high'
    elif vol >= 50000:
        return 'very high'
    elif vol >= 10000:
        return 'high'
    elif vol >= 1000:
        return 'moderate'
    elif vol >= 100:
        return 'low'
    else:
        return 'very low'


def difficulty_tier(d):
    if d is None:
        return 'unknown competition'
    if d >= 80:
        return 'extremely competitive'
    elif d >= 60:
        return 'highly competitive'
    elif d >= 40:
        return 'moderately competitive'
    elif d >= 20:
        return 'low competition'
    else:
        return 'very low competition'


def opportunity_assessment(vol, diff, trend):
    """Assess strategic opportunity from volume/difficulty/trend."""
    if diff is None:
        diff = 50  # assume moderate for assessment
    if vol >= 10000 and diff <= 30:
        return 'high-opportunity gap'
    elif vol >= 10000 and diff <= 50 and trend == 'rising':
        return 'strong growth opportunity'
    elif vol >= 50000 and diff >= 70:
        return 'high-volume but requires significant authority'
    elif trend == 'rising' and vol >= 1000:
        return 'emerging opportunity worth targeting early'
    elif trend == 'declining':
        return 'declining interest, lower priority'
    elif vol < 500:
        return 'niche long-tail opportunity'
    else:
        return 'steady-state opportunity'


def content_type_suggestion(intent, vol, diff):
    """Suggest best content type based on intent and metrics."""
    suggestions = {
        'informational': {
            'high_vol': 'comprehensive guide or pillar page',
            'mid_vol': 'detailed how-to article or tutorial',
            'low_vol': 'FAQ entry or knowledge base article',
        },
        'transactional': {
            'high_vol': 'optimized landing page with clear CTA',
            'mid_vol': 'product comparison or feature page',
            'low_vol': 'targeted landing page',
        },
        'navigational': {
            'high_vol': 'branded landing page with tool access',
            'mid_vol': 'branded page or tool entry point',
            'low_vol': 'redirect or branded page',
        },
        'commercial': {
            'high_vol': 'comparison guide or buying guide',
            'mid_vol': 'product review or comparison table',
            'low_vol': 'focused comparison article',
        },
    }

    intent_map = suggestions.get(intent, suggestions['informational'])
    if vol >= 10000:
        return intent_map['high_vol']
    elif vol >= 1000:
        return intent_map['mid_vol']
    else:
        return intent_map['low_vol']


def generate_content(kw):
    """Generate a 40-80 word content description for a keyword."""
    value = kw['value']
    locale = kw['locale_key']
    vol = kw.get('volume') or 0
    diff = kw.get('difficulty')
    intent = kw.get('intent') or 'informational'
    tp = kw.get('traffic_potential') or 0
    trend = kw.get('trend') or 'stable'

    vol_desc = volume_tier(vol)
    diff_desc = difficulty_tier(diff)
    opp = opportunity_assessment(vol, diff, trend)
    content_type = content_type_suggestion(intent, vol, diff)

    # Build the description
    parts = []

    # Sentence 1: Intent and volume context
    locale_label = 'French' if locale == 'fr-FR' else 'US English' if locale == 'en-US' else locale
    diff_str = f'difficulty {diff}/100' if diff is not None else 'unknown difficulty'
    # Avoid double quotes — use single quotes for keyword value in content
    safe_value = value.replace("'", "")
    parts.append(
        f'The keyword [{safe_value}] is a {vol_desc}-volume {intent} query in {locale_label} '
        f'({vol:,}/mo, {diff_str}, {trend} trend).'
    )

    # Sentence 2: Strategic opportunity and content recommendation
    tp_note = ''
    if tp and tp > vol * 1.5:
        tp_note = f' with traffic potential of {tp:,}'

    parts.append(
        f'This represents a {opp}{tp_note}; '
        f'best served by a {content_type}.'
    )

    return ' '.join(parts)


def main():
    with open(INPUT_PATH) as f:
        keywords = json.load(f)

    results = []
    for kw in keywords:
        content = generate_content(kw)
        results.append({'key': kw['key'], 'content': content})

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(
        json.dumps(results, indent=2, ensure_ascii=False),
        encoding='utf-8'
    )

    # Stats
    lengths = [len(r['content'].split()) for r in results]
    avg_len = sum(lengths) / len(lengths) if lengths else 0
    print(f'Generated {len(results)} content entries')
    print(f'Word count: min={min(lengths)}, max={max(lengths)}, avg={avg_len:.0f}')
    print(f'Output: {OUTPUT_PATH}')

    # Sample
    print(f'\nSample entries:')
    for r in results[:3]:
        print(f'  {r["key"]}:')
        print(f'    {r["content"]}')
        print()


if __name__ == '__main__':
    main()
