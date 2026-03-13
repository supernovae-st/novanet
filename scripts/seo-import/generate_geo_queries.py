#!/usr/bin/env python3
"""
Generate GEO queries from top SEO keywords for ALL locales.

Transforms SEO keywords into natural-language AI queries that users might
ask LLMs (Gemini, GPT, Perplexity, Claude) about QR codes.

Supports all 73 locales via parent-language template mapping:
  fr-BE, fr-CA → TEMPLATES_FR
  en-GB, en-IN → TEMPLATES_EN
  es-AR, es-CO → TEMPLATES_ES
  etc.

Usage:
  python3 scripts/seo-import/generate_geo_queries.py
"""

import json
import re
import unicodedata
from datetime import datetime, timezone
from pathlib import Path

OUTPUT_DIR = Path(__file__).parent / 'output'
SEO_BATCH_PATH = OUTPUT_DIR / 'content_batch.json'
GEO_QUERIES_PATH = OUTPUT_DIR / 'geo_queries.json'

NOW = datetime.now(tz=timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')


def slugify(text):
    """Create URL-safe slug with full Unicode support."""
    s = unicodedata.normalize('NFKC', text).lower().strip()
    s = re.sub(r'[\s_/\\]+', '-', s)
    s = re.sub(r'[^\w\-]', '', s, flags=re.UNICODE)
    s = re.sub(r'-{2,}', '-', s)
    s = s.strip('-')
    return s


# ---------------------------------------------------------------------------
# Query Templates by Language
# ---------------------------------------------------------------------------
# For each intent type, templates to convert keywords into AI-style queries.
# {kw} is replaced by the keyword value.

TEMPLATES_FR = {
    'informational': [
        "Qu'est-ce qu'un {kw} ?",
        "Comment fonctionne un {kw} ?",
        "À quoi sert un {kw} ?",
        "{kw} : guide complet",
    ],
    'transactional': [
        "Quel est le meilleur outil pour {kw} ?",
        "Comment {kw} gratuitement ?",
        "Quelle application recommander pour {kw} ?",
    ],
    'navigational': [
        "Où trouver un {kw} en ligne ?",
        "Quel site utiliser pour {kw} ?",
    ],
    'commercial': [
        "Comparatif des meilleurs outils de {kw}",
        "Quel {kw} choisir en 2026 ?",
        "Avis sur les solutions de {kw}",
    ],
}

TEMPLATES_EN = {
    'informational': [
        "What is a {kw}?",
        "How does a {kw} work?",
        "What are {kw} used for?",
        "{kw}: complete guide",
    ],
    'transactional': [
        "What is the best tool for {kw}?",
        "How to {kw} for free?",
        "What app should I use to {kw}?",
    ],
    'navigational': [
        "Where can I find a {kw} online?",
        "Best website for {kw}",
    ],
    'commercial': [
        "Best {kw} tools compared",
        "Which {kw} should I choose in 2026?",
        "Top {kw} solutions reviewed",
    ],
}

TEMPLATES_DE = {
    'informational': [
        "Was ist ein {kw}?",
        "Wie funktioniert ein {kw}?",
        "Wofür wird ein {kw} verwendet?",
        "{kw}: vollständiger Leitfaden",
    ],
    'transactional': [
        "Was ist das beste Tool für {kw}?",
        "Wie kann man {kw} kostenlos nutzen?",
        "Welche App eignet sich für {kw}?",
    ],
    'navigational': [
        "Wo finde ich einen {kw} online?",
        "Beste Webseite für {kw}",
    ],
    'commercial': [
        "Die besten {kw}-Tools im Vergleich",
        "Welchen {kw} sollte man 2026 wählen?",
        "Top {kw}-Lösungen im Test",
    ],
}

TEMPLATES_JP = {
    'informational': [
        "{kw}とは何ですか？",
        "{kw}の仕組みは？",
        "{kw}の使い方は？",
        "{kw}：完全ガイド",
    ],
    'transactional': [
        "{kw}に最適なツールは？",
        "{kw}を無料で使う方法は？",
        "{kw}におすすめのアプリは？",
    ],
    'navigational': [
        "{kw}をオンラインで見つけるには？",
        "{kw}に最適なサイトは？",
    ],
    'commercial': [
        "{kw}ツールの比較",
        "2026年に選ぶべき{kw}は？",
        "{kw}ソリューションのレビュー",
    ],
}

TEMPLATES_ES = {
    'informational': [
        "¿Qué es un {kw}?",
        "¿Cómo funciona un {kw}?",
        "¿Para qué sirve un {kw}?",
        "{kw}: guía completa",
    ],
    'transactional': [
        "¿Cuál es la mejor herramienta para {kw}?",
        "¿Cómo {kw} gratis?",
        "¿Qué aplicación recomiendas para {kw}?",
    ],
    'navigational': [
        "¿Dónde encontrar un {kw} en línea?",
        "Mejor sitio web para {kw}",
    ],
    'commercial': [
        "Comparativa de las mejores herramientas de {kw}",
        "¿Qué {kw} elegir en 2026?",
        "Las mejores soluciones de {kw}",
    ],
}

TEMPLATES_AR = {
    'informational': [
        "ما هو {kw}؟",
        "كيف يعمل {kw}؟",
        "ما هي استخدامات {kw}؟",
        "{kw}: دليل شامل",
    ],
    'transactional': [
        "ما هي أفضل أداة لـ {kw}؟",
        "كيف يمكن {kw} مجاناً؟",
        "ما التطبيق الأفضل لـ {kw}؟",
    ],
    'navigational': [
        "أين أجد {kw} على الإنترنت؟",
        "أفضل موقع لـ {kw}",
    ],
    'commercial': [
        "مقارنة أفضل أدوات {kw}",
        "أي {kw} تختار في 2026؟",
        "أفضل حلول {kw}",
    ],
}

TEMPLATES_PT = {
    'informational': [
        "O que é um {kw}?",
        "Como funciona um {kw}?",
        "Para que serve um {kw}?",
        "{kw}: guia completo",
    ],
    'transactional': [
        "Qual a melhor ferramenta para {kw}?",
        "Como {kw} de graça?",
        "Qual aplicativo usar para {kw}?",
    ],
    'navigational': [
        "Onde encontrar um {kw} online?",
        "Melhor site para {kw}",
    ],
    'commercial': [
        "Comparativo das melhores ferramentas de {kw}",
        "Qual {kw} escolher em 2026?",
        "As melhores soluções de {kw}",
    ],
}

TEMPLATES_IT = {
    'informational': [
        "Cos'è un {kw}?",
        "Come funziona un {kw}?",
        "A cosa serve un {kw}?",
        "{kw}: guida completa",
    ],
    'transactional': [
        "Qual è il miglior strumento per {kw}?",
        "Come {kw} gratis?",
        "Quale app usare per {kw}?",
    ],
    'navigational': [
        "Dove trovare un {kw} online?",
        "Miglior sito per {kw}",
    ],
    'commercial': [
        "Confronto dei migliori strumenti per {kw}",
        "Quale {kw} scegliere nel 2026?",
        "Le migliori soluzioni di {kw}",
    ],
}

TEMPLATES_AF = {
    'informational': [
        "Wat is 'n {kw}?",
        "Hoe werk 'n {kw}?",
        "Waarvoor word {kw} gebruik?",
        "{kw}: volledige gids",
    ],
    'transactional': [
        "Wat is die beste hulpmiddel vir {kw}?",
        "Hoe kan ek {kw} gratis gebruik?",
        "Watter app is die beste vir {kw}?",
    ],
    'navigational': [
        "Waar vind ek 'n {kw} aanlyn?",
        "Beste webwerf vir {kw}",
    ],
    'commercial': [
        "Vergelyking van die beste {kw}-gereedskap",
        "Watter {kw} om te kies in 2026?",
        "Top {kw}-oplossings beoordeel",
    ],
}

TEMPLATES_ET = {
    'informational': [
        "Mis on {kw}?",
        "Kuidas {kw} töötab?",
        "Milleks {kw} kasutatakse?",
        "{kw}: täielik juhend",
    ],
    'transactional': [
        "Mis on parim tööriist {kw} jaoks?",
        "Kuidas {kw} tasuta kasutada?",
        "Millist rakendust {kw} jaoks kasutada?",
    ],
    'navigational': [
        "Kust leida {kw} veebist?",
        "Parim veebileht {kw} jaoks",
    ],
    'commercial': [
        "Parimate {kw}-tööriistade võrdlus",
        "Milline {kw} valida 2026. aastal?",
        "Parimad {kw}-lahendused",
    ],
}

# ---------------------------------------------------------------------------
# Language → Templates mapping (all locale variants map to parent language)
# ---------------------------------------------------------------------------

LANG_TO_TEMPLATES = {
    'fr': TEMPLATES_FR,
    'en': TEMPLATES_EN,
    'de': TEMPLATES_DE,
    'ja': TEMPLATES_JP,
    'es': TEMPLATES_ES,
    'ar': TEMPLATES_AR,
    'pt': TEMPLATES_PT,
    'it': TEMPLATES_IT,
    'af': TEMPLATES_AF,
    'et': TEMPLATES_ET,
}


def get_templates_for_locale(locale_key):
    """Map any locale to its parent language templates.

    e.g. fr-BE → TEMPLATES_FR, en-GB → TEMPLATES_EN, es-AR → TEMPLATES_ES
    Falls back to TEMPLATES_EN if language is unknown.
    """
    lang = locale_key.split('-')[0]
    return LANG_TO_TEMPLATES.get(lang, TEMPLATES_EN)


# ---------------------------------------------------------------------------
# Manual high-priority queries (only for original 5 locales)
# ---------------------------------------------------------------------------

MANUAL_QUERIES_FR = [
    {"value": "Comment créer un QR code gratuitement ?", "query_type": "how_to"},
    {"value": "Quel est le meilleur générateur de QR code ?", "query_type": "recommendation"},
    {"value": "QR code vs lien court : lequel utiliser ?", "query_type": "comparison"},
    {"value": "Les QR codes sont-ils sécurisés ?", "query_type": "informational"},
    {"value": "Comment personnaliser un QR code avec un logo ?", "query_type": "how_to"},
    {"value": "Quelle est la différence entre QR code statique et dynamique ?", "query_type": "comparison"},
    {"value": "Comment scanner un QR code sans application ?", "query_type": "how_to"},
    {"value": "Quels sont les usages professionnels du QR code ?", "query_type": "informational"},
    {"value": "Comment créer un QR code pour un menu de restaurant ?", "query_type": "how_to"},
    {"value": "Les QR codes fonctionnent-ils hors ligne ?", "query_type": "factual"},
    {"value": "Comment suivre les scans d'un QR code ?", "query_type": "how_to"},
    {"value": "Quelle taille minimum pour un QR code imprimé ?", "query_type": "factual"},
    {"value": "Comment créer un QR code pour WhatsApp ?", "query_type": "how_to"},
    {"value": "QR code AI : les nouvelles tendances 2026", "query_type": "informational"},
    {"value": "Comment intégrer un QR code dans une campagne marketing ?", "query_type": "how_to"},
]

MANUAL_QUERIES_EN = [
    {"value": "How to create a QR code for free?", "query_type": "how_to"},
    {"value": "What is the best QR code generator?", "query_type": "recommendation"},
    {"value": "QR code vs short link: which should I use?", "query_type": "comparison"},
    {"value": "Are QR codes safe to scan?", "query_type": "informational"},
    {"value": "How to customize a QR code with a logo?", "query_type": "how_to"},
    {"value": "What is the difference between static and dynamic QR codes?", "query_type": "comparison"},
    {"value": "How to scan a QR code without an app?", "query_type": "how_to"},
    {"value": "What are the business uses of QR codes?", "query_type": "informational"},
    {"value": "How to create a QR code for a restaurant menu?", "query_type": "how_to"},
    {"value": "Do QR codes work offline?", "query_type": "factual"},
    {"value": "How to track QR code scans?", "query_type": "how_to"},
    {"value": "What is the minimum size for a printed QR code?", "query_type": "factual"},
    {"value": "How to create a QR code for WhatsApp?", "query_type": "how_to"},
    {"value": "AI QR codes: trends for 2026", "query_type": "informational"},
    {"value": "How to use QR codes in marketing campaigns?", "query_type": "how_to"},
]

MANUAL_QUERIES_DE = [
    {"value": "Wie erstelle ich einen QR-Code kostenlos?", "query_type": "how_to"},
    {"value": "Was ist der beste QR-Code-Generator?", "query_type": "recommendation"},
    {"value": "QR-Code vs. Kurzlink: Was ist besser?", "query_type": "comparison"},
    {"value": "Sind QR-Codes sicher?", "query_type": "informational"},
    {"value": "Wie kann man einen QR-Code mit Logo erstellen?", "query_type": "how_to"},
    {"value": "Was ist der Unterschied zwischen statischen und dynamischen QR-Codes?", "query_type": "comparison"},
    {"value": "Wie scanne ich einen QR-Code ohne App?", "query_type": "how_to"},
    {"value": "Welche geschäftlichen Einsatzmöglichkeiten gibt es für QR-Codes?", "query_type": "informational"},
    {"value": "Wie erstelle ich einen QR-Code für eine Speisekarte?", "query_type": "how_to"},
    {"value": "Funktionieren QR-Codes offline?", "query_type": "factual"},
    {"value": "Wie kann man QR-Code-Scans verfolgen?", "query_type": "how_to"},
    {"value": "Welche Mindestgröße braucht ein gedruckter QR-Code?", "query_type": "factual"},
    {"value": "Wie erstelle ich einen QR-Code für WhatsApp?", "query_type": "how_to"},
    {"value": "KI-QR-Codes: Trends für 2026", "query_type": "informational"},
    {"value": "Wie nutzt man QR-Codes in Marketingkampagnen?", "query_type": "how_to"},
]

MANUAL_QUERIES_JP = [
    {"value": "QRコードを無料で作成するには？", "query_type": "how_to"},
    {"value": "最高のQRコード生成ツールは？", "query_type": "recommendation"},
    {"value": "QRコードと短縮URL、どちらを使うべき？", "query_type": "comparison"},
    {"value": "QRコードは安全ですか？", "query_type": "informational"},
    {"value": "QRコードにロゴを入れる方法は？", "query_type": "how_to"},
    {"value": "静的QRコードと動的QRコードの違いは？", "query_type": "comparison"},
    {"value": "アプリなしでQRコードを読み取る方法は？", "query_type": "how_to"},
    {"value": "QRコードのビジネス活用法は？", "query_type": "informational"},
    {"value": "レストランメニュー用のQRコードを作成するには？", "query_type": "how_to"},
    {"value": "QRコードはオフラインで使えますか？", "query_type": "factual"},
    {"value": "QRコードのスキャン数を追跡する方法は？", "query_type": "how_to"},
    {"value": "印刷するQRコードの最小サイズは？", "query_type": "factual"},
    {"value": "WhatsApp用のQRコードを作成するには？", "query_type": "how_to"},
    {"value": "AI QRコード：2026年のトレンド", "query_type": "informational"},
    {"value": "マーケティングキャンペーンでQRコードを活用する方法は？", "query_type": "how_to"},
]

MANUAL_QUERIES_ES = [
    {"value": "¿Cómo crear un código QR gratis?", "query_type": "how_to"},
    {"value": "¿Cuál es el mejor generador de códigos QR?", "query_type": "recommendation"},
    {"value": "Código QR vs enlace corto: ¿cuál usar?", "query_type": "comparison"},
    {"value": "¿Los códigos QR son seguros?", "query_type": "informational"},
    {"value": "¿Cómo personalizar un código QR con logo?", "query_type": "how_to"},
    {"value": "¿Cuál es la diferencia entre código QR estático y dinámico?", "query_type": "comparison"},
    {"value": "¿Cómo escanear un código QR sin aplicación?", "query_type": "how_to"},
    {"value": "¿Cuáles son los usos empresariales del código QR?", "query_type": "informational"},
    {"value": "¿Cómo crear un código QR para el menú de un restaurante?", "query_type": "how_to"},
    {"value": "¿Los códigos QR funcionan sin internet?", "query_type": "factual"},
    {"value": "¿Cómo rastrear los escaneos de un código QR?", "query_type": "how_to"},
    {"value": "¿Cuál es el tamaño mínimo para un código QR impreso?", "query_type": "factual"},
    {"value": "¿Cómo crear un código QR para WhatsApp?", "query_type": "how_to"},
    {"value": "Códigos QR con IA: tendencias 2026", "query_type": "informational"},
    {"value": "¿Cómo usar códigos QR en campañas de marketing?", "query_type": "how_to"},
]

MANUAL_QUERIES_AR = [
    {"value": "كيف أنشئ رمز QR مجاناً؟", "query_type": "how_to"},
    {"value": "ما هو أفضل مولد رمز QR؟", "query_type": "recommendation"},
    {"value": "رمز QR مقابل الرابط القصير: أيهما أفضل؟", "query_type": "comparison"},
    {"value": "هل رموز QR آمنة للمسح؟", "query_type": "informational"},
    {"value": "كيف أخصص رمز QR بشعار؟", "query_type": "how_to"},
    {"value": "ما الفرق بين رمز QR الثابت والديناميكي؟", "query_type": "comparison"},
    {"value": "كيف أمسح رمز QR بدون تطبيق؟", "query_type": "how_to"},
    {"value": "ما هي استخدامات رمز QR في الأعمال؟", "query_type": "informational"},
    {"value": "كيف أنشئ رمز QR لقائمة مطعم؟", "query_type": "how_to"},
    {"value": "هل تعمل رموز QR بدون إنترنت؟", "query_type": "factual"},
]

MANUAL_QUERIES_PT = [
    {"value": "Como criar um QR code de graça?", "query_type": "how_to"},
    {"value": "Qual o melhor gerador de QR code?", "query_type": "recommendation"},
    {"value": "QR code vs link curto: qual usar?", "query_type": "comparison"},
    {"value": "QR codes são seguros?", "query_type": "informational"},
    {"value": "Como personalizar um QR code com logo?", "query_type": "how_to"},
    {"value": "Qual a diferença entre QR code estático e dinâmico?", "query_type": "comparison"},
    {"value": "Como escanear um QR code sem aplicativo?", "query_type": "how_to"},
    {"value": "Quais os usos empresariais do QR code?", "query_type": "informational"},
    {"value": "Como criar um QR code para cardápio de restaurante?", "query_type": "how_to"},
    {"value": "QR codes funcionam offline?", "query_type": "factual"},
]

MANUAL_QUERIES_IT = [
    {"value": "Come creare un QR code gratis?", "query_type": "how_to"},
    {"value": "Qual è il miglior generatore di QR code?", "query_type": "recommendation"},
    {"value": "QR code vs link corto: quale usare?", "query_type": "comparison"},
    {"value": "I QR code sono sicuri?", "query_type": "informational"},
    {"value": "Come personalizzare un QR code con logo?", "query_type": "how_to"},
    {"value": "Qual è la differenza tra QR code statico e dinamico?", "query_type": "comparison"},
    {"value": "Come scansionare un QR code senza app?", "query_type": "how_to"},
    {"value": "Quali sono gli usi aziendali del QR code?", "query_type": "informational"},
    {"value": "Come creare un QR code per il menu di un ristorante?", "query_type": "how_to"},
    {"value": "I QR code funzionano offline?", "query_type": "factual"},
]

# Manual queries mapped by language (applied to primary locale per language)
MANUAL_QUERIES_BY_LANG = {
    'fr': ('fr-FR', MANUAL_QUERIES_FR),
    'en': ('en-US', MANUAL_QUERIES_EN),
    'de': ('de-DE', MANUAL_QUERIES_DE),
    'ja': ('ja-JP', MANUAL_QUERIES_JP),
    'es': ('es-MX', MANUAL_QUERIES_ES),
    'ar': ('ar-SA', MANUAL_QUERIES_AR),
    'pt': ('pt-BR', MANUAL_QUERIES_PT),
    'it': ('it-IT', MANUAL_QUERIES_IT),
}


# ---------------------------------------------------------------------------
# Query classification + generation
# ---------------------------------------------------------------------------

def classify_query_type(intent, template):
    """Map SEO intent + template pattern to GEO query_type."""
    tpl_lower = template.lower()
    # how_to patterns (multi-language)
    how_to_markers = [
        'comment', 'how', 'wie', 'cómo', '方法', '作成', '使い方',
        'como', 'come', 'كيف', 'hoe', 'kuidas',
    ]
    if any(m in tpl_lower for m in how_to_markers):
        return 'how_to'
    # comparison patterns
    comparison_markers = [
        'comparatif', 'compared', 'vergleich', 'comparativ', '比較', 'vs',
        'comparativo', 'confronto', 'مقارنة', 'vergelyking', 'võrdlus',
    ]
    if any(m in tpl_lower for m in comparison_markers):
        return 'comparison'
    # recommendation patterns
    rec_markers = [
        'meilleur', 'best', 'beste', 'mejor', '最適', '最高', 'recommand', 'おすすめ',
        'melhor', 'miglior', 'أفضل', 'parim',
    ]
    if any(m in tpl_lower for m in rec_markers):
        return 'recommendation'
    if intent == 'informational':
        return 'informational'
    if intent == 'transactional':
        return 'transactional'
    if intent == 'navigational':
        return 'navigational'
    return 'informational'


def is_question(text):
    """Check if keyword already forms a question."""
    lower = text.lower().strip()
    fr_starters = ['comment', "qu'est", 'quel', 'quelle', 'pourquoi', 'où', 'quand',
                   'combien', 'est-ce', 'peut-on', 'faut-il']
    en_starters = ['how', 'what', 'why', 'where', 'when', 'which', 'can', 'do', 'does',
                   'is', 'are', 'should']
    de_starters = ['wie', 'was', 'warum', 'wo', 'wann', 'welch', 'kann', 'ist', 'sind',
                   'wofür', 'funktionier']
    es_starters = ['cómo', 'qué', 'cuál', 'por qué', 'dónde', 'cuándo', 'para qué',
                   '¿cómo', '¿qué', '¿cuál', '¿por', '¿dónde', '¿cuándo', '¿para']
    pt_starters = ['como', 'o que', 'qual', 'por que', 'onde', 'quando', 'para que']
    it_starters = ["cos'è", 'come', 'quale', 'perché', 'dove', 'quando', 'a cosa']
    ar_starters = ['كيف', 'ما', 'هل', 'أين', 'متى', 'لماذا']
    af_starters = ['hoe', 'wat', 'waarom', 'waar', 'wanneer', 'watter']
    et_starters = ['kuidas', 'mis', 'miks', 'kus', 'millal', 'milline']

    starters = (fr_starters + en_starters + de_starters + es_starters +
                pt_starters + it_starters + ar_starters + af_starters + et_starters)

    if any(lower.startswith(s) for s in starters):
        return True
    # Japanese suffix-based questions
    jp_suffixes = ['ですか', 'ますか', 'には？', 'とは', 'は？', 'の？']
    if any(lower.endswith(s) or lower.endswith(s.rstrip('？') + '?') for s in jp_suffixes):
        return True
    # Arabic question marker at end
    if lower.endswith('؟'):
        return True
    return False


def keyword_to_query(kw_value, intent, locale_key, templates, rank):
    """Convert a keyword into a natural AI query."""
    if is_question(kw_value):
        q = kw_value[0].upper() + kw_value[1:]
        if not q.endswith('?') and not q.endswith('？') and not q.endswith('؟'):
            q += ' ?'
        lower = q.lower()
        how_markers = ['comment', 'how', 'wie', 'cómo', '方法', '作成', '使い方',
                       'como', 'come', 'كيف', 'hoe', 'kuidas']
        rec_markers = ['quel', 'which', 'best', 'welch', 'beste', 'mejor', 'cuál',
                       '最適', '最高', 'おすすめ', 'melhor', 'miglior', 'أفضل', 'parim']
        cmp_markers = ['vs', 'difference', 'comparati', 'unterschied', 'vergleich',
                       'diferencia', '違い', 'مقابل', 'مقارنة']
        if any(m in lower for m in how_markers):
            qt = 'how_to'
        elif any(m in lower for m in rec_markers):
            qt = 'recommendation'
        elif any(m in lower for m in cmp_markers):
            qt = 'comparison'
        else:
            qt = 'informational'
        return q, qt

    intent_templates = templates.get(intent, templates['informational'])
    template_idx = rank % len(intent_templates)
    template = intent_templates[template_idx]
    query_value = template.format(kw=kw_value)
    query_type = classify_query_type(intent, template)

    return query_value, query_type


def generate_queries_from_keywords(keywords, locale_key, templates):
    """Generate GEO queries by applying templates to top keywords."""
    sorted_kw = sorted(keywords, key=lambda k: k.get('volume') or 0, reverse=True)
    top_keywords = sorted_kw[:40]

    queries = []
    seen_slugs = set()

    for rank, kw in enumerate(top_keywords):
        intent = kw.get('intent', 'informational')
        query_value, query_type = keyword_to_query(
            kw['value'], intent, locale_key, templates, rank
        )

        slug = slugify(query_value)
        if slug in seen_slugs:
            continue
        seen_slugs.add(slug)

        query_key = f'geo:{slug}@{locale_key}'

        queries.append({
            'key': query_key,
            'value': query_value,
            'locale_key': locale_key,
            'query_type': query_type,
            'language_hint': locale_key.split('-')[0],
            'source_keyword': kw['value'],
            'source_keyword_key': kw['key'],
            'platforms': ['gemini', 'gpt', 'perplexity', 'claude'],
        })

    return queries


def main():
    with open(SEO_BATCH_PATH) as f:
        all_keywords = json.load(f)

    # Discover all unique locales from content_batch.json
    all_locales = sorted(set(kw['locale_key'] for kw in all_keywords))
    print(f'Discovered {len(all_locales)} locales in content_batch.json')

    # Split by locale
    keywords_by_locale = {}
    for locale_key in all_locales:
        keywords_by_locale[locale_key] = [
            kw for kw in all_keywords if kw['locale_key'] == locale_key
        ]

    print('\nSEO keywords loaded:')
    for locale_key in all_locales:
        count = len(keywords_by_locale[locale_key])
        print(f'  {locale_key}: {count}')

    # Build set of primary locales per language for manual queries
    primary_locales = {locale for _, (locale, _) in MANUAL_QUERIES_BY_LANG.items()}

    # Generate from templates + manual queries per locale
    all_queries = []
    queries_by_locale = {}

    for locale_key in all_locales:
        templates = get_templates_for_locale(locale_key)
        kws = keywords_by_locale.get(locale_key, [])
        lang = locale_key.split('-')[0]

        # Generate from top SEO keywords
        locale_queries = generate_queries_from_keywords(kws, locale_key, templates)

        # Add manual high-priority queries only for primary locales
        if lang in MANUAL_QUERIES_BY_LANG:
            primary_locale, manual_queries = MANUAL_QUERIES_BY_LANG[lang]
            if locale_key == primary_locale:
                for mq in manual_queries:
                    slug = slugify(mq['value'])
                    key = f'geo:{slug}@{locale_key}'
                    if not any(q['key'] == key for q in locale_queries):
                        locale_queries.append({
                            'key': key,
                            'value': mq['value'],
                            'locale_key': locale_key,
                            'query_type': mq['query_type'],
                            'language_hint': lang,
                            'source_keyword': None,
                            'source_keyword_key': None,
                            'platforms': ['gemini', 'gpt', 'perplexity', 'claude'],
                        })

        queries_by_locale[locale_key] = locale_queries
        all_queries.extend(locale_queries)

    # Write output
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    GEO_QUERIES_PATH.write_text(
        json.dumps(all_queries, indent=2, ensure_ascii=False),
        encoding='utf-8'
    )

    print(f'\nGenerated {len(all_queries)} GEO queries across {len(all_locales)} locales:')
    for locale_key in all_locales:
        queries = queries_by_locale.get(locale_key, [])
        if queries:
            print(f'  {locale_key}: {len(queries)} queries')

    # Show samples per language (one locale per language)
    seen_langs = set()
    for locale_key in all_locales:
        lang = locale_key.split('-')[0]
        if lang in seen_langs:
            continue
        seen_langs.add(lang)
        queries = queries_by_locale.get(locale_key, [])
        if queries:
            print(f'\nSample {locale_key} ({lang}) queries:')
            for q in queries[:3]:
                src = f' (from: {q["source_keyword"]})' if q['source_keyword'] else ' (manual)'
                print(f'  [{q["query_type"]:15s}] {q["value"]}{src}')

    # Query type distribution
    from collections import Counter
    types = Counter(q['query_type'] for q in all_queries)
    print(f'\nQuery type distribution:')
    for qt, count in types.most_common():
        print(f'  {qt:20s}: {count}')


if __name__ == '__main__':
    main()
