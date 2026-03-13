#!/usr/bin/env python3
"""
Generate Cypher seed files for GEO data (GEOQuerySet, GEOQuery, GEOAnswer,
ProjectGEOScope, and all arcs).

Architecture:
  Locale → HAS_GEO_QUERIES → GEOQuerySet → CONTAINS_GEO_QUERY → GEOQuery → HAS_GEO_ANSWERS → GEOAnswer
  Project → HAS_GEO_SCOPE → ProjectGEOScope → MONITORS_QUERY → GEOQuery

Usage:
  python3 scripts/seo-import/generate_geo_seed.py
"""

import json
from datetime import datetime, timezone
from pathlib import Path

OUTPUT_DIR = Path(__file__).parent / 'output'
GEO_QUERIES_PATH = OUTPUT_DIR / 'geo_queries.json'
GEO_ANSWERS_PATH = OUTPUT_DIR / 'geo_answers.json'
SEED_DIR = Path(__file__).parent.parent.parent / 'packages' / 'db' / 'seed'

NOW = datetime.now(tz=timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
PROJECT_KEY = 'qrcode-ai'


def escape_cypher(s):
    """Escape single quotes for Cypher strings."""
    if s is None:
        return ''
    return str(s).replace("\\", "\\\\").replace("'", "\\'")


def to_cypher_value(val):
    """Convert Python value to Cypher literal."""
    if val is None:
        return 'null'
    if isinstance(val, bool):
        return 'true' if val else 'false'
    if isinstance(val, (int, float)):
        return str(val)
    if isinstance(val, list):
        items = ', '.join(f"'{escape_cypher(v)}'" for v in val)
        return f'[{items}]'
    return f"'{escape_cypher(val)}'"


def generate_geo_seed(locale_key, queries, answers):
    """Generate complete Cypher seed for one locale's GEO data."""
    lines = []
    set_key = f'geo-queries@{locale_key}'
    scope_key = f'geo-scope:{PROJECT_KEY}@{locale_key}'
    locale_label = locale_key

    lines.append(f'// ═══════════════════════════════════════════════════════════════')
    lines.append(f'// GEO Seed: {locale_key} ({len(queries)} queries, {len(answers)} answers)')
    lines.append(f'// Generated: {NOW}')
    lines.append(f'// ═══════════════════════════════════════════════════════════════')
    lines.append('')

    # --- GEOQuerySet ---
    lines.append(f'// --- GEOQuerySet ({locale_key}) ---')
    lines.append(f"MERGE (qs:GEOQuerySet {{key: '{set_key}'}})")
    lines.append(f"SET qs += {{")
    lines.append(f"  display_name: 'GEO Queries ({locale_key})',")
    lines.append(f"  node_class: 'GEOQuerySet',")
    lines.append(f"  content: 'Container for {len(queries)} GEO queries tracked across AI engines for the {locale_label} market.',")
    lines.append(f"  triggers: ['geo', 'queries', 'ai visibility', '{locale_key}'],")
    lines.append(f"  provenance: '{{\"source\": \"seed:geo-import\", \"version\": \"1.0\"}}',")
    lines.append(f"  total_queries: {len(queries)},")
    lines.append(f"  last_sync: datetime('{NOW}'),")
    lines.append(f"  created_at: datetime('{NOW}'),")
    lines.append(f"  updated_at: datetime('{NOW}')")
    lines.append(f"}};")
    lines.append('')

    # --- HAS_GEO_QUERIES (Locale → GEOQuerySet) ---
    lines.append(f'// --- HAS_GEO_QUERIES (Locale → GEOQuerySet) ---')
    lines.append(f"MATCH (l:Locale {{key: '{locale_key}'}})")
    lines.append(f"MATCH (qs:GEOQuerySet {{key: '{set_key}'}})")
    lines.append(f"MERGE (l)-[:HAS_GEO_QUERIES]->(qs);")
    lines.append('')

    # --- ProjectGEOScope ---
    lines.append(f'// --- ProjectGEOScope ({locale_key}) ---')
    lines.append(f"MERGE (scope:ProjectGEOScope {{key: '{scope_key}'}})")
    lines.append(f"SET scope += {{")
    lines.append(f"  display_name: 'GEO Scope ({PROJECT_KEY}, {locale_key})',")
    lines.append(f"  node_class: 'ProjectGEOScope',")
    lines.append(f"  content: '{locale_label} market AI visibility monitoring for {PROJECT_KEY} — track QR code queries across Gemini, GPT, Perplexity, and Claude.',")
    lines.append(f"  triggers: ['geo', 'scope', 'monitoring', '{PROJECT_KEY}', '{locale_key}'],")
    lines.append(f"  provenance: '{{\"source\": \"seed:geo-import\", \"version\": \"1.0\"}}',")
    lines.append(f"  project_key: '{PROJECT_KEY}',")
    lines.append(f"  locale_key: '{locale_key}',")
    lines.append(f"  strategy: 'balanced',")
    lines.append(f"  platforms: ['gemini', 'gpt', 'perplexity', 'claude'],")
    lines.append(f"  total_monitored: {len(queries)},")
    lines.append(f"  check_frequency: 'weekly',")
    lines.append(f"  curator: 'seed:geo-import',")
    lines.append(f"  created_at: datetime('{NOW}'),")
    lines.append(f"  updated_at: datetime('{NOW}')")
    lines.append(f"}};")
    lines.append('')

    # --- HAS_GEO_SCOPE (Project → ProjectGEOScope) ---
    lines.append(f'// --- HAS_GEO_SCOPE (Project → ProjectGEOScope) ---')
    lines.append(f"MATCH (p:Project {{key: '{PROJECT_KEY}'}})")
    lines.append(f"MATCH (scope:ProjectGEOScope {{key: '{scope_key}'}})")
    lines.append(f"MERGE (p)-[:HAS_GEO_SCOPE]->(scope);")
    lines.append('')

    # --- GEOQuery nodes ---
    lines.append(f'// --- GEOQuery nodes ({len(queries)}) ---')
    for q in queries:
        lines.append(f"MERGE (q:GEOQuery {{key: {to_cypher_value(q['key'])}}})")
        lines.append(f"SET q += {{")

        props = []
        props.append(f"  display_name: {to_cypher_value(q['value'])}")
        props.append(f"  node_class: 'GEOQuery'")
        props.append(f"  value: {to_cypher_value(q['value'])}")
        props.append(f"  query_type: {to_cypher_value(q['query_type'])}")
        props.append(f"  language_hint: {to_cypher_value(q['language_hint'])}")
        props.append(f"  platforms: {to_cypher_value(q['platforms'])}")
        props.append(f"  triggers: ['geo', 'query', {to_cypher_value(q['query_type'])}, {to_cypher_value(q['locale_key'])}]")

        # Content field
        src = f" Derived from SEO keyword [{q['source_keyword']}]." if q.get('source_keyword') else ''
        content = (
            f"GEO query [{q['value']}] tracks AI engine responses for {locale_label} market.{src} "
            f"Query type: {q['query_type']}. Monitored across: {', '.join(q['platforms'])}."
        )
        props.append(f"  content: {to_cypher_value(content)}")
        props.append(f"  provenance: '{{\"source\": \"seed:geo-import\", \"version\": \"1.0\"}}'")
        props.append(f"  created_at: datetime('{NOW}')")
        props.append(f"  updated_at: datetime('{NOW}')")

        lines.append(',\n'.join(props))
        lines.append(f"}};")
        lines.append('')

    # --- CONTAINS_GEO_QUERY arcs (GEOQuerySet → GEOQuery) ---
    lines.append(f'// --- CONTAINS_GEO_QUERY arcs ---')
    lines.append(f"MATCH (qs:GEOQuerySet {{key: '{set_key}'}})")
    lines.append(f"MATCH (q:GEOQuery) WHERE q.key ENDS WITH '@{locale_key}'")
    lines.append(f"MERGE (qs)-[:CONTAINS_GEO_QUERY]->(q);")
    lines.append('')

    # --- MONITORS_QUERY arcs (ProjectGEOScope → GEOQuery) ---
    lines.append(f'// --- MONITORS_QUERY arcs ---')
    for i, q in enumerate(queries):
        priority = 'high' if i < 15 else ('medium' if i < 35 else 'low')
        lines.append(f"MATCH (scope:ProjectGEOScope {{key: '{scope_key}'}})")
        lines.append(f"MATCH (q:GEOQuery {{key: {to_cypher_value(q['key'])}}})")
        lines.append(f"MERGE (scope)-[:MONITORS_QUERY {{")
        lines.append(f"  priority: '{priority}',")
        lines.append(f"  curator: 'auto-imported',")
        lines.append(f"  monitored_at: datetime('{NOW}'),")
        lines.append(f"  monitor_frequency: 'weekly'")
        lines.append(f"}}]->(q);")
        lines.append('')

    # --- GEOAnswer nodes + HAS_GEO_ANSWERS arcs ---
    locale_answers = [a for a in answers if a['locale_key'] == locale_key]
    if locale_answers:
        lines.append(f'// --- GEOAnswer nodes ({len(locale_answers)}) ---')
        for a in locale_answers:
            lines.append(f"MERGE (a:GEOAnswer {{key: {to_cypher_value(a['key'])}}})")
            lines.append(f"SET a += {{")

            props = []
            qv_short = a['query_value'][:50]
            props.append(f"  display_name: {to_cypher_value(f'Answer: {qv_short}...')}")
            props.append(f"  node_class: 'GEOAnswer'")
            props.append(f"  engine: {to_cypher_value(a['engine'])}")
            props.append(f"  engine_version: {to_cypher_value(a['engine_version'])}")
            props.append(f"  observed_at: datetime('{NOW}')")
            props.append(f"  answer_text: {to_cypher_value(a['answer_text'])}")
            props.append(f"  cited_domains: {to_cypher_value(a['cited_domains'])}")
            props.append(f"  brand_mentions: {to_cypher_value(a['brand_mentions'])}")
            props.append(f"  relevance_score: {a['relevance_score']}")
            props.append(f"  ai_visibility_score: {a['ai_visibility_score']}")
            props.append(f"  share_of_voice: {a['share_of_voice']}")
            props.append(f"  position_in_answer: {a['position_in_answer']}")
            props.append(f"  word_count: {a['word_count']}")
            props.append(f"  triggers: ['geo', 'answer', {to_cypher_value(a['engine'])}, {to_cypher_value(locale_key)}]")
            engine = a['engine']
            qv = a['query_value']
            rel = a['relevance_score']
            vis = a['ai_visibility_score']
            content_str = f'GEO answer from {engine} for query [{qv}]. Relevance: {rel}/100, visibility: {vis}/100.'
            props.append(f"  content: {to_cypher_value(content_str)}")
            props.append(f"  provenance: '{{\"source\": \"seed:geo-import\", \"engine\": \"{a['engine']}\", \"version\": \"1.0\"}}'")
            props.append(f"  created_at: datetime('{NOW}')")
            props.append(f"  updated_at: datetime('{NOW}')")

            lines.append(',\n'.join(props))
            lines.append(f"}};")
            lines.append('')

        # --- HAS_GEO_ANSWERS arcs (GEOQuery → GEOAnswer) ---
        lines.append(f'// --- HAS_GEO_ANSWERS arcs ---')
        for a in locale_answers:
            lines.append(f"MATCH (q:GEOQuery {{key: {to_cypher_value(a['query_key'])}}})")
            lines.append(f"MATCH (a:GEOAnswer {{key: {to_cypher_value(a['key'])}}})")
            lines.append(f"MERGE (q)-[:HAS_GEO_ANSWERS]->(a);")
            lines.append('')

    return '\n'.join(lines)


def main():
    # Load data
    with open(GEO_QUERIES_PATH) as f:
        all_queries = json.load(f)

    with open(GEO_ANSWERS_PATH) as f:
        all_answers = json.load(f)

    print(f'Loaded {len(all_queries)} queries, {len(all_answers)} answers')

    # Discover all locales dynamically from queries
    all_locales = sorted(set(q['locale_key'] for q in all_queries))
    print(f'Found {len(all_locales)} locales')

    # Generate per-locale seed files
    total_queries = 0
    total_answers = 0
    for locale_key in all_locales:
        queries = [q for q in all_queries if q['locale_key'] == locale_key]
        answers = [a for a in all_answers if a['locale_key'] == locale_key]

        if not queries:
            print(f'No queries for {locale_key}, skipping')
            continue

        cypher = generate_geo_seed(locale_key, queries, answers)
        filename = f'51-geoqueries-{locale_key.lower()}.cypher'
        filepath = SEED_DIR / filename

        filepath.write_text(cypher, encoding='utf-8')
        print(f'  {locale_key}: {len(queries)} queries, {len(answers)} answers')
        total_queries += len(queries)
        total_answers += len(answers)

    print(f'\nDone! {len(all_locales)} seed files in {SEED_DIR}')
    print(f'Total: {total_queries} queries, {total_answers} answers')


if __name__ == '__main__':
    main()
