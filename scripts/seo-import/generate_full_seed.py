#!/usr/bin/env python3
"""
Generate full SEOKeyword Cypher seed files from Ahrefs CSV exports.

Auto-discovers ALL locale directories from the source data directory and generates:
1. SEOKeywordSet containers (1 per locale)
2. SEOKeyword nodes with ALL available Ahrefs properties
3. Ownership arcs (HAS_SEO_KEYWORDS, CONTAINS_SEO_KEYWORD)
4. ProjectSEOScope + CURATES_KEYWORD (for qrcode-ai)
5. TARGETS_KEYWORD for ~10 keywords linked to Entity qr-code (original 5 locales)

Usage:
  python3 scripts/seo-import/generate_full_seed.py
"""

import csv
import io
import json
import re
import unicodedata
from datetime import datetime, timezone
from pathlib import Path

# === CONFIG ===

# Source: Ahrefs CSV exports organized by locale directory
SOURCE_DIR = Path('/Users/thibaut/Projects/traduction_ai/ath-know-qrcai/_docs/sitemap-structure/_sources/seo-keyword-qr-code')
OUTPUT_DIR = Path('/Users/thibaut/dev/supernovae/novanet/packages/db/seed')

NOW = datetime.now(tz=timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')

# Keywords to link to Entity qr-code via TARGETS_KEYWORD (original 5 locales only)
TARGET_KEYWORDS_BY_LOCALE = {
    'fr-FR': [
        'qr code', 'code qr', 'qr code gratuit', 'generateur qr code',
        'créer qr code', 'qr code personnalisé', 'flasher un qr code',
        'lire qr code', 'scanner qr code', 'qr code en ligne',
    ],
    'en-US': [
        'qr code', 'qr code generator', 'free qr code generator',
        'qr code scanner', 'create qr code', 'custom qr code',
        'qr code maker', 'how to make a qr code', 'scan qr code',
        'qr code reader',
    ],
    'de-DE': [
        'qr code', 'qr code generator', 'qr code erstellen',
        'qr code scannen', 'qr code scanner', 'qr code erstellen kostenlos',
        'qr code generator kostenlos', 'qr code generieren',
        'qr code lesen', 'qr code erstellen online',
    ],
    'ja-JP': [
        'qrコード作成', 'qrコード', 'qrコード読み取り',
        'qrコード作成 無料', 'qrコード生成', 'qrコードアプリ',
        'qrコード作成 無料 安全', 'qrコードリーダー',
        'qrコード読み取り アプリ', 'qrコード 作り方',
    ],
    'es-MX': [
        'codigo qr', 'qr code', 'escanear codigo qr',
        'qr code generator', 'generador de qr', 'generar codigo qr',
        'crear codigo qr', 'lector qr', 'código qr', 'generador qr',
    ],
}


def slugify(text):
    """Create URL-safe slug from keyword text.

    Handles Latin, CJK, Arabic, Cyrillic, Devanagari and other Unicode scripts.
    """
    s = text.lower().strip()
    # Replace spaces and separators with hyphens
    s = re.sub(r'[\s_/\\]+', '-', s)
    # Remove characters that are not word chars (Unicode-aware) or hyphens
    s = re.sub(r'[^\w\-]', '', s, flags=re.UNICODE)
    # Collapse multiple hyphens
    s = re.sub(r'-{2,}', '-', s)
    s = s.strip('-')
    # Truncate very long slugs (some keywords are phrases)
    if len(s) > 120:
        s = s[:120].rstrip('-')
    return s


def parse_float(val):
    """Parse float, return None if empty."""
    if not val or val == '':
        return None
    try:
        return float(val)
    except (ValueError, TypeError):
        return None


def parse_int(val):
    """Parse int, return None if empty."""
    if not val or val == '':
        return None
    try:
        return int(val)
    except (ValueError, TypeError):
        return None


def parse_intent(intents_str):
    """Extract primary intent from Ahrefs intent string."""
    if not intents_str:
        return None
    parts = [p.strip() for p in intents_str.split(',')]
    for part in parts:
        lower = part.lower()
        if lower == 'transactional':
            return 'transactional'
        elif lower == 'informational':
            return 'informational'
        elif lower == 'navigational':
            return 'navigational'
        elif lower in ('commercial', 'commercial investigation'):
            return 'commercial'
    return 'informational'


def parse_serp_features(features_str):
    """Parse SERP features string into array."""
    if not features_str:
        return []
    mapping = {
        'ai overview': 'ai_overview',
        'people also ask': 'people_also_ask',
        'featured snippet': 'featured_snippet',
        'video preview': 'videos',
        'video': 'videos',
        'image': 'images',
        'image pack': 'images',
        'thumbnail': 'images',
        'top stories': 'top_stories',
        'knowledge panel': 'knowledge_panel',
        'local pack': 'local_pack',
        'shopping': 'shopping',
        'sitelinks': 'sitelinks',
    }
    features = []
    for f in features_str.split(','):
        f_clean = f.strip().lower()
        if f_clean in mapping:
            mapped = mapping[f_clean]
            if mapped not in features:
                features.append(mapped)
    return features


def derive_trend(growth_12m):
    """Derive trend from 12-month growth."""
    if growth_12m is None:
        return None
    if growth_12m > 5:
        return 'rising'
    elif growth_12m < -5:
        return 'declining'
    else:
        return 'stable'


def derive_seasonality(sv_trend_str):
    """Derive 12-month seasonality index from monthly values.

    SV trend columns have varying date ranges in the header name,
    but always contain comma-separated monthly volume numbers.
    We take the last 12 values and normalize to 100 = average.
    """
    if not sv_trend_str:
        return None
    try:
        values = [int(v.strip()) for v in sv_trend_str.split(',') if v.strip()]
        if len(values) < 12:
            return None
        last_12 = values[-12:]
        avg = sum(last_12) / len(last_12)
        if avg == 0:
            return None
        return [round(v / avg * 100) for v in last_12]
    except (ValueError, TypeError):
        return None


def find_sv_trend_column(headers, keyword):
    """Find the SV trend column by pattern match.

    Ahrefs uses varying date ranges like:
    - 'SV trend (02-2024 - 01-2026)'
    - 'SV trend (03-2024 - 02-2026)'
    """
    for h in headers:
        if keyword in h.lower():
            return h
    return None


def parse_ahrefs_csv(filepath, locale_key):
    """Parse an Ahrefs CSV export and return structured keyword data."""
    with open(filepath, 'r', encoding='utf-16-le') as f:
        content = f.read()
    if content.startswith('\ufeff'):
        content = content[1:]

    reader = csv.reader(io.StringIO(content), delimiter='\t')
    raw_headers = next(reader)
    headers = [h.strip('"') for h in raw_headers]

    # Find SV trend columns dynamically (date range varies per export)
    sv_trend_col = find_sv_trend_column(headers, 'sv trend (')
    sv_forecast_col = find_sv_trend_column(headers, 'sv forecasting trend')

    keywords = []
    for row in reader:
        if len(row) < 5:
            continue

        data = {}
        for h, v in zip(headers, row):
            data[h] = v.strip('"')

        keyword_value = data.get('Keyword', '').strip()
        if not keyword_value:
            continue

        volume = parse_int(data.get('Volume'))
        difficulty = parse_int(data.get('Difficulty'))
        cpc = parse_float(data.get('CPC'))
        clicks_per_search = parse_float(data.get('CPS'))
        traffic_potential = parse_int(data.get('Traffic potential'))
        global_volume = parse_int(data.get('Global volume'))
        global_traffic_potential = parse_int(data.get('Global traffic potential'))
        growth_3m = parse_float(data.get('Growth (3mo)'))
        growth_6m = parse_float(data.get('Growth (6mo)'))
        growth_12m = parse_float(data.get('Growth (12mo)'))
        growth_forecast = parse_float(data.get('Growth forecast (12mo)'))
        mobile = parse_float(data.get('Mobile'))
        desktop = parse_float(data.get('Desktop'))
        parent_keyword = data.get('Parent Keyword', '').strip() or None

        slug = slugify(keyword_value)
        if not slug:
            continue
        key = f'seo:{slug}@{locale_key}'

        kw = {
            'key': key,
            'display_name': f'{keyword_value} ({locale_key})',
            'node_class': 'SEOKeyword',
            'value': keyword_value,
            'slug_form': slug,
            'locale_key': locale_key,
            'volume': volume,
            'difficulty': difficulty,
            'cpc': cpc,
            'clicks_per_search': clicks_per_search,
            'intent': parse_intent(data.get('Intents')),
            'traffic_potential': traffic_potential,
            'global_volume': global_volume,
            'global_traffic_potential': global_traffic_potential,
            'parent_keyword': parent_keyword,
            'source': 'ahrefs',
            'platform': 'google',
            'source_date': data.get('Last Update', '')[:10] if data.get('Last Update') else None,
            'first_seen': data.get('First seen') if data.get('First seen') else None,
            'serp_features': parse_serp_features(data.get('SERP Features', '')),
            'mobile_pct': round(mobile * 100, 1) if mobile is not None else None,
            'desktop_pct': round(desktop * 100, 1) if desktop is not None else None,
            'growth_3m': growth_3m,
            'growth_6m': growth_6m,
            'growth_12m': growth_12m,
            'growth_forecast_12m': growth_forecast,
            'trend': derive_trend(growth_12m),
        }

        # SV trend columns (matched dynamically)
        if sv_trend_col and data.get(sv_trend_col):
            kw['seasonality'] = derive_seasonality(data[sv_trend_col])
        if sv_forecast_col and data.get(sv_forecast_col):
            kw['seasonality_forecast'] = derive_seasonality(data[sv_forecast_col])

        # Derive opportunity score
        if traffic_potential and difficulty is not None:
            opp = (100 - difficulty) * (traffic_potential / max(volume, 1)) if volume else 0
            kw['opportunity'] = round(min(opp, 100), 1)

        keywords.append(kw)

    return keywords


def escape_cypher(s):
    """Escape string for Cypher."""
    if s is None:
        return ''
    return str(s).replace('\\', '\\\\').replace("'", "\\'").replace('"', '\\"').replace('\n', ' ').replace('\r', '')


def to_cypher_value(val):
    """Convert Python value to Cypher literal."""
    if val is None:
        return 'null'
    if isinstance(val, bool):
        return 'true' if val else 'false'
    if isinstance(val, int):
        return str(val)
    if isinstance(val, float):
        return str(val)
    if isinstance(val, list):
        if not val:
            return '[]'
        if all(isinstance(v, (int, float)) for v in val):
            return f'[{", ".join(str(v) for v in val)}]'
        return f'[{", ".join(to_cypher_value(str(v)) for v in val)}]'
    return f"'{escape_cypher(str(val))}'"


def generate_keyword_cypher(kw):
    """Generate MERGE statement for a single SEOKeyword."""
    props = []
    for field, val in kw.items():
        if val is None:
            continue
        if field in ('seasonality', 'seasonality_forecast', 'serp_features') and isinstance(val, list) and not val:
            continue
        props.append(f'  {field}: {to_cypher_value(val)}')

    # Add standard properties
    props.append(f"  triggers: ['search', 'keyword', 'seo', 'ranking', 'optimization']")
    props.append(f"  provenance: '{{\"source\": \"ahrefs\", \"imported_by\": \"seed\", \"version\": \"0.19.0\"}}'")
    props.append(f"  created_at: datetime('{NOW}')")
    props.append(f"  updated_at: datetime('{NOW}')")

    props_str = ',\n'.join(props)
    return f"MERGE (kw:SEOKeyword {{key: {to_cypher_value(kw['key'])}}})\nSET kw += {{\n{props_str}\n}};"


def generate_seed_file(locale_key, keywords, target_keywords_list):
    """Generate complete Cypher seed file for a locale."""
    lines = []
    lines.append(f'// =============================================================================')
    lines.append(f'// SEO Keywords Seed \u2014 {locale_key}')
    lines.append(f'// Generated: {NOW}')
    lines.append(f'// Source: Ahrefs export (auto-discovered)')
    lines.append(f'// Keywords: {len(keywords)}')
    lines.append(f'// =============================================================================')
    lines.append('')

    # --- Container ---
    set_key = f'seo-keywords@{locale_key}'
    lines.append(f'// --- SEOKeywordSet container ---')
    lines.append(f"MERGE (ks:SEOKeywordSet {{key: '{set_key}'}})")
    lines.append(f"SET ks += {{")
    lines.append(f"  display_name: 'SEO Keywords ({locale_key})',")
    lines.append(f"  node_class: 'SEOKeywordSet',")
    lines.append(f"  content: 'SEO keywords for {locale_key} market from Ahrefs',")
    lines.append(f"  triggers: ['seo', 'keywords', 'container', '{locale_key.lower()}'],")
    lines.append(f"  data_source: 'ahrefs',")
    lines.append(f"  total_keywords: {len(keywords)},")
    lines.append(f"  last_sync: datetime('{NOW}'),")
    lines.append(f"  provenance: '{{\"source\": \"ahrefs\", \"imported_by\": \"seed\", \"version\": \"0.19.0\"}}',")
    lines.append(f"  created_at: datetime('{NOW}'),")
    lines.append(f"  updated_at: datetime('{NOW}')")
    lines.append(f"}};")
    lines.append('')

    # --- HAS_SEO_KEYWORDS arc (Locale -> SEOKeywordSet) ---
    lines.append(f'// --- Locale -> SEOKeywordSet ownership arc ---')
    lines.append(f"MATCH (l:Locale {{key: '{locale_key}'}})")
    lines.append(f"MATCH (ks:SEOKeywordSet {{key: '{set_key}'}})")
    lines.append(f"MERGE (l)-[:HAS_SEO_KEYWORDS]->(ks);")
    lines.append('')

    # --- Keywords (in batches of 50 for readability) ---
    batch_size = 50
    for i in range(0, len(keywords), batch_size):
        batch = keywords[i:i + batch_size]
        batch_num = i // batch_size + 1
        lines.append(f'// --- Batch {batch_num}: keywords {i+1}-{min(i+batch_size, len(keywords))} ---')
        lines.append('')
        for kw in batch:
            lines.append(generate_keyword_cypher(kw))
            lines.append('')

    # --- CONTAINS_SEO_KEYWORD arcs (SEOKeywordSet -> SEOKeyword) ---
    lines.append(f'// --- CONTAINS_SEO_KEYWORD arcs ---')
    lines.append(f"MATCH (ks:SEOKeywordSet {{key: '{set_key}'}})")
    lines.append(f"MATCH (kw:SEOKeyword) WHERE kw.locale_key = '{locale_key}'")
    lines.append(f"MERGE (ks)-[:CONTAINS_SEO_KEYWORD]->(kw);")
    lines.append('')

    # --- ProjectSEOScope ---
    scope_key = f'seo-scope:qrcode-ai@{locale_key}'
    lines.append(f'// --- ProjectSEOScope for qrcode-ai ---')
    lines.append(f"MERGE (scope:ProjectSEOScope {{key: '{scope_key}'}})")
    lines.append(f"SET scope += {{")
    lines.append(f"  display_name: 'SEO Scope (qrcode-ai, {locale_key})',")
    lines.append(f"  node_class: 'ProjectSEOScope',")
    lines.append(f"  content: 'SEO keyword curation for QR Code AI \u2014 {locale_key} market',")
    lines.append(f"  triggers: ['seo', 'scope', 'curation', 'qrcode-ai', '{locale_key.lower()}'],")
    lines.append(f"  project_key: 'qrcode-ai',")
    lines.append(f"  locale_key: '{locale_key}',")
    lines.append(f"  strategy: 'balanced',")
    lines.append(f"  total_curated: {len(keywords)},")
    lines.append(f"  curator: 'seed:ahrefs-import',")
    lines.append(f"  last_curation_at: datetime('{NOW}'),")
    lines.append(f"  provenance: '{{\"source\": \"ahrefs\", \"curator\": \"seed\", \"version\": \"0.19.0\"}}',")
    lines.append(f"  created_at: datetime('{NOW}'),")
    lines.append(f"  updated_at: datetime('{NOW}')")
    lines.append(f"}};")
    lines.append('')

    # --- HAS_SEO_SCOPE arc (Project -> ProjectSEOScope) ---
    lines.append(f'// --- Project -> ProjectSEOScope arc ---')
    lines.append(f"MATCH (p:Project {{key: 'qrcode-ai'}})")
    lines.append(f"MATCH (scope:ProjectSEOScope {{key: '{scope_key}'}})")
    lines.append(f"MERGE (p)-[:HAS_SEO_SCOPE]->(scope);")
    lines.append('')

    # --- CURATES_KEYWORD arcs (ProjectSEOScope -> SEOKeyword) ---
    lines.append(f'// --- CURATES_KEYWORD arcs (all keywords curated for qrcode-ai) ---')
    lines.append(f"MATCH (scope:ProjectSEOScope {{key: '{scope_key}'}})")
    lines.append(f"MATCH (kw:SEOKeyword) WHERE kw.locale_key = '{locale_key}'")
    lines.append(f"MERGE (scope)-[:CURATES_KEYWORD {{")
    lines.append(f"  priority: 'medium',")
    lines.append(f"  curator: 'auto-imported',")
    lines.append(f"  curated_at: datetime('{NOW}'),")
    lines.append(f"  strategy: 'cluster'")
    lines.append(f"}}]->(kw);")
    lines.append('')

    # --- TARGETS_KEYWORD arcs (Entity qr-code -> top keywords, original 5 locales only) ---
    if target_keywords_list:
        lines.append(f'// --- TARGETS_KEYWORD arcs (Entity qr-code -> top keywords) ---')
        for i, kw_value in enumerate(target_keywords_list):
            slug = slugify(kw_value)
            kw_key = f'seo:{slug}@{locale_key}'
            priority = 'primary' if i < 3 else ('secondary' if i < 6 else 'tertiary')
            weight = round(1.0 - (i * 0.08), 2)
            lines.append(f"MATCH (en:EntityNative) WHERE en.key STARTS WITH 'entity:qr-code@{locale_key}'")
            lines.append(f"MATCH (kw:SEOKeyword {{key: '{kw_key}'}})")
            lines.append(f"MERGE (en)-[:TARGETS_KEYWORD {{")
            lines.append(f"  priority: '{priority}',")
            lines.append(f"  weight: {weight},")
            lines.append(f"  targeted_at: datetime('{NOW}'),")
            lines.append(f"  curator: 'seed:ahrefs-import'")
            lines.append(f"}}]->(kw);")
            lines.append('')

    return '\n'.join(lines)


def discover_locale_csvs(source_dir):
    """Auto-discover all locale directories with CSV files.

    Returns dict: locale_key -> [csv_paths]
    """
    locale_pattern = re.compile(r'^[a-z]{2,3}-[A-Z]{2}$')
    result = {}

    for d in sorted(source_dir.iterdir()):
        if not d.is_dir():
            continue
        locale_key = d.name
        if not locale_pattern.match(locale_key):
            continue

        csv_files = sorted(d.glob('*.csv'))
        if csv_files:
            result[locale_key] = csv_files

    return result


def main():
    print(f'=== SEO Keyword Seed Generator (Auto-Discovery) ===')
    print(f'Source: {SOURCE_DIR}')
    print(f'Output: {OUTPUT_DIR}')
    print()

    # Auto-discover all locale directories
    locale_csvs = discover_locale_csvs(SOURCE_DIR)
    print(f'Discovered {len(locale_csvs)} locales with CSV data')
    print()

    all_keywords = {}

    for locale_key, csv_files in locale_csvs.items():
        combined = []
        seen_keys = set()

        for filepath in csv_files:
            try:
                keywords = parse_ahrefs_csv(filepath, locale_key)
            except Exception as e:
                print(f'  WARNING: Failed to parse {filepath.name}: {e}')
                continue

            print(f'  {locale_key}: {filepath.name} -> {len(keywords)} keywords')

            # Dedup by key (keep the one with higher volume)
            for kw in keywords:
                if kw['key'] not in seen_keys:
                    seen_keys.add(kw['key'])
                    combined.append(kw)
                else:
                    existing = next(k for k in combined if k['key'] == kw['key'])
                    if (kw.get('volume') or 0) > (existing.get('volume') or 0):
                        combined.remove(existing)
                        combined.append(kw)

        if not combined:
            print(f'  {locale_key}: No keywords found, skipping')
            continue

        all_keywords[locale_key] = combined

    print()

    # Load generated content if available
    content_gen_path = Path(__file__).parent / 'output' / 'content_generated.json'
    content_map = {}
    if content_gen_path.exists():
        try:
            content_data = json.loads(content_gen_path.read_text(encoding='utf-8'))
            content_map = {item['key']: item['content'] for item in content_data if item.get('content')}
            print(f'Loaded {len(content_map)} content entries')
        except Exception as e:
            print(f'WARNING: Could not load content: {e}')

    # Inject content into keywords
    if content_map:
        for locale_key, keywords in all_keywords.items():
            injected = 0
            for kw in keywords:
                if kw['key'] in content_map:
                    kw['content'] = content_map[kw['key']]
                    injected += 1
            if injected:
                print(f'  {locale_key}: injected content into {injected}/{len(keywords)} keywords')

    # Generate seed files
    print()
    print('=== Generating Cypher seed files ===')
    for locale_key, keywords in sorted(all_keywords.items()):
        target_list = TARGET_KEYWORDS_BY_LOCALE.get(locale_key, [])
        cypher = generate_seed_file(locale_key, keywords, target_list)

        locale_slug = locale_key.lower()
        output_path = OUTPUT_DIR / f'50-seokeywords-{locale_slug}.cypher'
        output_path.write_text(cypher, encoding='utf-8')
        print(f'  {output_path.name}: {len(keywords)} keywords')

    # Write JSON for LLM content generation
    content_batch = []
    for locale_key, keywords in sorted(all_keywords.items()):
        for kw in keywords:
            content_batch.append({
                'key': kw['key'],
                'value': kw['value'],
                'locale_key': locale_key,
                'volume': kw.get('volume'),
                'difficulty': kw.get('difficulty'),
                'intent': kw.get('intent'),
                'traffic_potential': kw.get('traffic_potential'),
                'trend': kw.get('trend'),
            })

    batch_path = Path(__file__).parent / 'output' / 'content_batch.json'
    batch_path.parent.mkdir(parents=True, exist_ok=True)
    batch_path.write_text(json.dumps(content_batch, indent=2, ensure_ascii=False), encoding='utf-8')

    # Summary
    print()
    print('=== SUMMARY ===')
    total = sum(len(kws) for kws in all_keywords.values())
    print(f'Total locales: {len(all_keywords)}')
    print(f'Total keywords: {total}')
    print()

    # Group by language family for readability
    families = {}
    for locale in sorted(all_keywords.keys()):
        lang = locale.split('-')[0]
        if lang not in families:
            families[lang] = []
        families[lang].append((locale, len(all_keywords[locale])))

    for lang in sorted(families.keys()):
        locales = families[lang]
        family_total = sum(c for _, c in locales)
        print(f'  {lang}: {family_total} keywords across {len(locales)} locales')
        for locale, count in locales:
            print(f'    {locale}: {count}')

    print()
    print(f'Seed files: {OUTPUT_DIR}/50-seokeywords-*.cypher')
    print(f'Content batch: {batch_path} ({len(content_batch)} entries)')


if __name__ == '__main__':
    main()
